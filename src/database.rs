use crate::dto::{EpisodeDetail, Season, Series};
use crate::path_resolver::PathResolver;
use crate::util::Entry;
use rusqlite::{params, Connection, Result};
use std::path::Path;
use std::sync::{Mutex, OnceLock};

static DB_CONN: OnceLock<Mutex<Connection>> = OnceLock::new();

/// Initialize the database connection and schema
/// 
/// # Arguments
/// * `db_path` - Path to the database file
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, error otherwise
/// 
/// # Errors
/// * Returns error if database is already initialized
/// * Returns error if parent directory cannot be created
/// * Returns error if database cannot be opened
/// * Returns error if schema creation fails
pub fn initialize_database(db_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Open or create database connection
    let conn = Connection::open(db_path)?;
    
    // Initialize schema
    conn.execute(
        "CREATE TABLE IF NOT EXISTS series (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS season (
            id INTEGER PRIMARY KEY,
            series_id INTEGER NOT NULL,
            number INTEGER NOT NULL,
            FOREIGN KEY(series_id) REFERENCES series(id)
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS episode (
            id INTEGER PRIMARY KEY,
            location TEXT NOT NULL,
            name TEXT NOT NULL,
            watched BOOLEAN NOT NULL,
            length INTEGER NOT NULL,
            series_id INTEGER,
            season_id INTEGER,
            episode_number INTEGER,
            year INTEGER,
            FOREIGN KEY(series_id) REFERENCES series(id),
            FOREIGN KEY(season_id) REFERENCES season(id)
        )",
        [],
    )?;
    
    // Data cleanup operations
    conn.execute(
        "UPDATE episode SET season_id = NULL WHERE series_id IS NULL",
        [],
    )?;
    
    conn.execute(
        "UPDATE episode 
         SET season_id = NULL 
         WHERE season_id IS NOT NULL 
         AND (SELECT series_id FROM season WHERE id = episode.season_id) != episode.series_id",
        [],
    )?;
    
    conn.execute(
        "UPDATE episode SET episode_number = NULL WHERE season_id IS NULL",
        [],
    )?;
    
    conn.execute(
        "DELETE FROM season 
         WHERE series_id IN (
             SELECT id FROM series 
             WHERE id NOT IN (SELECT DISTINCT series_id FROM episode WHERE series_id IS NOT NULL)
         )",
        [],
    )?;
    
    conn.execute(
        "DELETE FROM series 
         WHERE id NOT IN (SELECT DISTINCT series_id FROM episode WHERE series_id IS NOT NULL)",
        [],
    )?;
    
    conn.execute(
        "DELETE FROM season 
         WHERE id NOT IN (SELECT DISTINCT season_id FROM episode WHERE season_id IS NOT NULL)",
        [],
    )?;
    
    // Store connection in OnceLock
    DB_CONN.set(Mutex::new(conn))
        .map_err(|_| "Database already initialized")?;
    
    Ok(())
}

/// Get a reference to the database connection
/// 
/// # Returns
/// * `&'static Mutex<Connection>` - Reference to the database connection
/// 
/// # Panics
/// * Panics if database has not been initialized
pub fn get_connection() -> &'static Mutex<Connection> {
    DB_CONN.get().expect("Database not initialized")
}

pub fn episode_exists(location: &str) -> Result<bool> {
    let conn = get_connection().lock().unwrap();

    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM episode WHERE location = ?1)")?;
    let exists: bool = stmt.query_row(params![location], |row| row.get(0))?;
    Ok(exists)
}

/// Import an episode with relative path storage
/// 
/// # Arguments
/// * `absolute_location` - The absolute path to the video file
/// * `name` - The name of the episode
/// * `resolver` - PathResolver for converting to relative paths and validation
/// 
/// # Returns
/// * `Result<bool, Box<dyn std::error::Error>>` - Ok(true) if inserted, Ok(false) if already exists, Err on error
pub fn import_episode_relative(
    absolute_location: &str,
    name: &str,
    resolver: &PathResolver,
) -> Result<bool, Box<dyn std::error::Error>> {
    let absolute_path = Path::new(absolute_location);
    
    // Validate that the path is under the configured root directory
    resolver.validate_path_under_root(absolute_path)?;
    
    // Convert absolute path to relative path
    let relative_path = resolver.to_relative(absolute_path)?;
    let relative_location = relative_path.to_str()
        .ok_or("Failed to convert path to string")?;
    
    // Check if episode already exists with this relative path
    if episode_exists(relative_location)? {
        return Ok(false); // Already exists, not inserted
    }

    let conn = get_connection().lock().unwrap();

    conn.execute(
        "INSERT INTO episode (location, name, watched, length, series_id, season_id, episode_number, year)
         VALUES (?1, ?2, false, 0, null, null, null, null)",
        params![relative_location, name],
    )?;
    Ok(true) // Successfully inserted
}

pub fn get_entries() -> Result<Vec<Entry>> {
    let conn = get_connection().lock().unwrap();

    let mut entries = Vec::new();

    // Retrieve series
    let mut stmt = conn.prepare("SELECT id, name FROM series ORDER BY name")?;
    let series_iter = stmt.query_map([], |row| {
        Ok(Entry::Series {
            series_id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    for series in series_iter {
        entries.push(series?);
    }

    // Retrieve episodes that are not part of a series
    let mut stmt = conn.prepare(
        "SELECT id, name, location 
         FROM episode WHERE series_id IS NULL 
         ORDER BY 
           CASE WHEN episode_number IS NULL OR episode_number = '' THEN 1 ELSE 0 END,
           CAST(episode_number AS INTEGER),
           name",
    )?;
    let episode_iter = stmt.query_map([], |row| {
        Ok(Entry::Episode {
            episode_id: row.get(0)?,
            name: row.get(1)?,
            location: row.get(2)?,
        })
    })?;

    for episode in episode_iter {
        entries.push(episode?);
    }

    Ok(entries)
}

pub fn get_entries_for_series(series_id: usize) -> Result<Vec<Entry>> {
    let conn = get_connection().lock().unwrap();

    let mut entries = Vec::new();

    // Retrieve seasons for the selected series
    let mut stmt =
        conn.prepare("SELECT id, number FROM season WHERE series_id = ?1 ORDER BY number")?;
    let season_iter = stmt.query_map(params![series_id], |row| {
        Ok(Entry::Season {
            season_id: row.get(0)?,
            number: row.get(1)?,
        })
    })?;

    for season in season_iter {
        entries.push(season?);
    }

    // Retrieve episodes that are part of the series but not part of a season
    let mut stmt = conn.prepare(
        "SELECT id, name, location 
         FROM episode WHERE series_id = ?1 AND season_id IS NULL ORDER BY year, name",
    )?;
    let episode_iter = stmt.query_map(params![series_id], |row| {
        Ok(Entry::Episode {
            episode_id: row.get(0)?,
            name: row.get(1)?,
            location: row.get(2)?,
        })
    })?;

    for episode in episode_iter {
        entries.push(episode?);
    }

    Ok(entries)
}

pub fn get_entries_for_season(season_id: usize) -> Result<Vec<Entry>> {
    let conn = get_connection().lock().unwrap();

    let mut entries = Vec::new();

    // Retrieve episodes that are part of the season
    let mut stmt = conn.prepare(
        "SELECT id, name, location 
         FROM episode WHERE season_id = ?1 
         ORDER BY 
           CASE WHEN episode_number IS NULL OR episode_number = '' THEN 1 ELSE 0 END,
           CAST(episode_number AS INTEGER),
           name",
    )?;
    let episode_iter = stmt.query_map(params![season_id], |row| {
        Ok(Entry::Episode {
            episode_id: row.get(0)?,
            name: row.get(1)?,
            location: row.get(2)?,
        })
    })?;

    for episode in episode_iter {
        entries.push(episode?);
    }

    Ok(entries)
}

/// Get the absolute location of an episode by resolving its relative path
/// 
/// # Arguments
/// * `episode_id` - The ID of the episode
/// * `resolver` - PathResolver for converting relative paths to absolute
/// 
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Absolute path as string or error
pub fn get_episode_absolute_location(
    episode_id: usize,
    resolver: &PathResolver,
) -> Result<String, Box<dyn std::error::Error>> {
    let conn = get_connection().lock().unwrap();

    let mut stmt = conn.prepare("SELECT location FROM episode WHERE id = ?1")?;
    let relative_location: String = stmt.query_row(params![episode_id], |row| row.get(0))?;
    
    // Convert relative path to absolute path
    let relative_path = Path::new(&relative_location);
    let absolute_path = resolver.to_absolute(relative_path);
    
    absolute_path.to_str()
        .ok_or("Failed to convert path to string".into())
        .map(|s| s.to_string())
}

pub fn get_episode_detail(id: usize) -> Result<EpisodeDetail, Box<dyn std::error::Error>> {
    // Fetch details from the database for episode
    let conn = get_connection().lock().unwrap();

    let mut stmt = conn.prepare(
        "SELECT 
                episode.name as title, 
                COALESCE(CAST(episode.year AS TEXT), '') as year, 
                CASE WHEN episode.watched THEN 'true' ELSE 'false' END as watched, 
                COALESCE(CAST(episode.length AS TEXT), '') as length, 
                series.id as series_id,
                COALESCE(series.name, '') as series_name, 
                season.id as season_id,
                COALESCE(season.number, '') as season_number, 
                COALESCE(CAST(episode.episode_number AS TEXT), '') as episode_number
            FROM episode
            LEFT JOIN season ON season.id = episode.season_id AND season.series_id = episode.series_id
            LEFT JOIN series ON series.id = episode.series_id
            WHERE episode.id = ?1"
    )?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        let series = if let Some(series_id) = row.get::<_, Option<usize>>(4)? {
            Some(Series {
                id: series_id,
                name: row.get(5)?,
            })
        } else {
            None
        };

        let season = if let Some(season_id) = row.get::<_, Option<usize>>(6)? {
            Some(Season {
                id: season_id,
                number: row.get(7)?,
            })
        } else {
            None
        };

        Ok(EpisodeDetail {
            title: row.get(0)?,
            year: row.get(1)?,
            watched: row.get(2)?,
            length: row.get(3)?,
            series,
            season,
            episode_number: row.get(8)?,
        })
    } else {
        Err("Episode not found".into())
    }
}

pub fn update_episode_detail(
    id: usize,
    details: &EpisodeDetail,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection().lock().unwrap();

    conn.execute(
        "UPDATE episode SET name = ?1, year = ?2, watched = ?3, length = ?4, series_id = ?5, season_id = ?6, episode_number = ?7 WHERE id = ?8",
        params![
            details.title,
            details.year,
            details.watched == "true",
            details.length,
            details.series.as_ref().map(|s| &s.id),
            details.season.as_ref().map(|s| &s.id),
            details.episode_number,
            id
        ],
    )?;
    Ok(())
}

pub fn toggle_watched_status(id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection().lock().unwrap();

    conn.execute(
        "UPDATE episode SET watched = NOT watched WHERE id = ?1",
        params![id],
    )?;

    Ok(())
}

pub fn unwatch_all_in_season(season_id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection().lock().unwrap();

    conn.execute(
        "UPDATE episode SET watched = false WHERE season_id = ?1",
        params![season_id],
    )?;

    Ok(())
}

pub fn unwatch_all_in_series(series_id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection().lock().unwrap();

    conn.execute(
        "UPDATE episode SET watched = false WHERE series_id = ?1",
        params![series_id],
    )?;

    Ok(())
}

pub fn unwatch_all_standalone() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection().lock().unwrap();

    conn.execute(
        "UPDATE episode SET watched = false WHERE series_id IS NULL",
        [],
    )?;

    Ok(())
}

pub fn clear_series_data(episode_id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection().lock().unwrap();

    conn.execute(
        "UPDATE episode SET series_id = NULL, season_id = NULL, episode_number = NULL WHERE id = ?1",
        params![episode_id],
    )?;

    Ok(())
}

pub fn delete_episode(episode_id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection().lock().unwrap();

    conn.execute(
        "DELETE FROM episode WHERE id = ?1",
        params![episode_id],
    )?;

    Ok(())
}

pub fn get_all_series() -> Result<Vec<Series>> {
    let conn = get_connection().lock().unwrap();

    let mut series = Vec::new();

    let mut stmt = conn.prepare("SELECT id, name FROM series")?;
    let series_iter = stmt.query_map([], |row| {
        Ok(Series {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    for s in series_iter {
        series.push(s?);
    }

    Ok(series)
}

pub fn get_series_by_id(series_id: usize) -> Result<Series> {
    let conn = get_connection().lock().unwrap();
    
    let mut stmt = conn.prepare("SELECT id, name FROM series WHERE id = ?1")?;
    let series = stmt.query_row(params![series_id], |row| {
        Ok(Series {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    
    Ok(series)
}

pub fn get_season_by_id(season_id: usize) -> Result<(Season, usize)> {
    let conn = get_connection().lock().unwrap();
    
    let mut stmt = conn.prepare("SELECT id, number, series_id FROM season WHERE id = ?1")?;
    let (season, series_id) = stmt.query_row(params![season_id], |row| {
        Ok((
            Season {
                id: row.get(0)?,
                number: row.get(1)?,
            },
            row.get(2)?
        ))
    })?;
    
    Ok((season, series_id))
}

pub fn create_series_and_assign(name: &str, episode_id: usize) -> Result<EpisodeDetail> {
    {
        // Create a new scope to release the lock after the transaction
        let conn = get_connection().lock().unwrap();
        conn.execute("INSERT INTO series (name) VALUES (?1)", params![name])?;
        let series_id = conn.last_insert_rowid() as i32;
        conn.execute(
            "UPDATE episode SET series_id = ?1 WHERE id = ?2",
            params![series_id, episode_id],
        )?;
    }
    Ok(get_episode_detail(episode_id).expect("Failed to get episode details"))
}

pub fn assign_series(series_id: usize, episode_id: usize) -> Result<EpisodeDetail> {
    {
        // Create a new scope to release the lock after the transaction
        let conn = get_connection().lock().unwrap();
        conn.execute(
            "UPDATE episode SET series_id = ?1 WHERE id = ?2",
            params![series_id, episode_id],
        )?;
    }
    Ok(get_episode_detail(episode_id).expect("Failed to get episode details"))
}

// can_create_season is a helper function to determine if a season can be created for the series
// it takes a series_id and a season_number (int) and returns a boolean
// special rules:
//  if the series does not exist, return false
//  if the season_number is 1 or 0, return true
//  if the season_number is greater than 1, check if the previous season exists
//  if the previous season exists, return true
//  otherwise, return false
pub fn can_create_season(series_id: Option<usize>, season_number: usize) -> Result<bool> {
    let conn = get_connection().lock().unwrap();

    if series_id.is_none() {
        return Ok(false);
    }

    if season_number <= 1 {
        return Ok(true);
    }

    let mut stmt = conn.prepare("SELECT 1 FROM season WHERE series_id = ?1 AND number = ?2")?;
    let exists: bool = stmt.query_row(params![series_id, season_number - 1], |row| row.get(0))?;
    Ok(exists)
}

// create_season_and_assign is a function to create a new season for a series and assign an episode to it
// it takes a series_id, season_number, and episode_id
// it returns the season_id of the newly created season
// it is private and only called from the update_episode_detail function
pub fn create_season_and_assign(
    series_id: usize,
    season_number: usize,
    episode_id: usize,
) -> Result<usize> {
    let conn = get_connection().lock().unwrap();

    //first, try to retrieve an existing season based on the series_id and season_number
    let mut stmt = conn.prepare("SELECT id FROM season WHERE series_id = ?1 AND number = ?2")?;
    let season_id: Option<usize> = stmt
        .query_row(params![series_id, season_number], |row| row.get(0))
        .ok();

    if let Some(season_id) = season_id {
        conn.execute(
            "UPDATE episode SET season_id = ?1 WHERE id = ?2",
            params![season_id, episode_id],
        )?;
        Ok(season_id)
    } else {
        //if the season does not exist, create it
        conn.execute(
            "INSERT INTO season (series_id, number) VALUES (?1, ?2)",
            params![series_id, season_number],
        )?;
        let season_id = conn.last_insert_rowid() as usize;
        conn.execute(
            "UPDATE episode SET season_id = ?1 WHERE id = ?2",
            params![season_id, episode_id],
        )?;
        Ok(season_id)
    }
}

/// Get episode counts for a series
/// 
/// Counts all episodes across all seasons and standalone episodes within the series.
/// Treats NULL watched status as unwatched (false).
/// 
/// # Arguments
/// * `series_id` - The series ID
/// 
/// # Returns
/// * `Result<(usize, usize), Box<dyn std::error::Error>>` - (total_episodes, unwatched_episodes) or error
pub fn get_series_episode_counts(series_id: usize) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let conn = get_connection().lock().unwrap();
    
    let mut stmt = conn.prepare(
        "SELECT 
            COUNT(*) as total,
            SUM(CASE WHEN watched = 0 OR watched IS NULL THEN 1 ELSE 0 END) as unwatched
         FROM episode
         WHERE series_id = ?1"
    )?;
    
    let (total, unwatched) = stmt.query_row(params![series_id], |row| {
        Ok((
            row.get::<_, i64>(0)? as usize,
            row.get::<_, i64>(1)? as usize,
        ))
    })?;
    
    Ok((total, unwatched))
}

/// Get episode counts for a season
/// 
/// Counts only episodes that belong to the specific season.
/// Treats NULL watched status as unwatched (false).
/// 
/// # Arguments
/// * `season_id` - The season ID
/// 
/// # Returns
/// * `Result<(usize, usize), Box<dyn std::error::Error>>` - (total_episodes, unwatched_episodes) or error
pub fn get_season_episode_counts(season_id: usize) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let conn = get_connection().lock().unwrap();
    
    let mut stmt = conn.prepare(
        "SELECT 
            COUNT(*) as total,
            SUM(CASE WHEN watched = 0 OR watched IS NULL THEN 1 ELSE 0 END) as unwatched
         FROM episode
         WHERE season_id = ?1"
    )?;
    
    let (total, unwatched) = stmt.query_row(params![season_id], |row| {
        Ok((
            row.get::<_, i64>(0)? as usize,
            row.get::<_, i64>(1)? as usize,
        ))
    })?;
    
    Ok((total, unwatched))
}

/// Calculate the next available episode number for a series and optional season
/// 
/// This function finds the lowest positive integer starting from 1 that is not
/// currently assigned to any episode within the same series and season combination.
/// If there are gaps in the sequence (e.g., 1, 2, 4, 5), it returns the first gap (3).
/// If all sequential numbers are taken, it returns max + 1.
/// If no episodes exist, it returns 1.
/// 
/// # Arguments
/// * `series_id` - The ID of the series
/// * `season_number` - Optional season number to scope the query
/// 
/// # Returns
/// * `Result<usize>` - The next available episode number or error
pub fn get_next_available_episode_number(
    series_id: usize,
    season_number: Option<usize>,
) -> Result<usize> {
    let conn = get_connection().lock().unwrap();

    // Query episode numbers based on whether season_number is provided
    let episode_numbers: Vec<usize> = if let Some(season_num) = season_number {
        // Query episodes for specific season
        let mut stmt = conn.prepare(
            "SELECT CAST(episode_number AS INTEGER) as ep_num
             FROM episode
             WHERE series_id = ?1
               AND season_id = (SELECT id FROM season WHERE series_id = ?1 AND number = ?2)
               AND episode_number IS NOT NULL
               AND episode_number != ''
             ORDER BY ep_num"
        )?;
        
        let rows = stmt.query_map(params![series_id, season_num], |row| {
            row.get::<_, i64>(0).map(|n| n as usize)
        })?;
        
        rows.filter_map(|r| r.ok()).collect()
    } else {
        // Query episodes without season (season_id IS NULL)
        let mut stmt = conn.prepare(
            "SELECT CAST(episode_number AS INTEGER) as ep_num
             FROM episode
             WHERE series_id = ?1
               AND season_id IS NULL
               AND episode_number IS NOT NULL
               AND episode_number != ''
             ORDER BY ep_num"
        )?;
        
        let rows = stmt.query_map(params![series_id], |row| {
            row.get::<_, i64>(0).map(|n| n as usize)
        })?;
        
        rows.filter_map(|r| r.ok()).collect()
    };

    // Find the first gap in the sequence starting from 1
    let mut expected = 1;
    for &num in &episode_numbers {
        if num == expected {
            expected += 1;
        } else if num > expected {
            // Found a gap, return the first missing number
            return Ok(expected);
        }
        // If num < expected, skip it (duplicate or out of order, shouldn't happen with ORDER BY)
    }

    // No gaps found, return the next sequential number
    Ok(expected)
}
