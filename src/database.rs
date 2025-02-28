use rusqlite::{params, Connection, Result};
use crate::util::Entry;
use crate::dto::{EpisodeDetail, Series, Season};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DB_CONN: Mutex<Option<Connection>> = Mutex::new(None);
}

fn initialize_db_connection(db_path: &str) {
    let mut db_conn = DB_CONN.lock().unwrap();
    if db_conn.is_some() {
        // Close the existing connection if it exists
        db_conn.take();
    }
    let conn = Connection::open(db_path).expect("Failed to initialize database");
    *db_conn = Some(conn);
}

pub fn initialize_database(db_path: &str) -> Result<()> {
    initialize_db_connection(db_path);

    let conn = DB_CONN.lock().unwrap();
    let conn = conn.as_ref().expect("Database connection is not initialized");

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
    Ok(())
}

pub fn episode_exists(location: &str) -> Result<bool> {
    let conn = DB_CONN.lock().unwrap();
    let conn = conn.as_ref().expect("Database connection is not initialized");

    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM episode WHERE location = ?1)")?;
    let exists: bool = stmt.query_row(params![location], |row| row.get(0))?;
    Ok(exists)
}

pub fn import_episode(
    location: &str,
    name: &str
) -> Result<()> {
    if episode_exists(location)? {
        return Ok(());
    }

    let conn = DB_CONN.lock().unwrap();
    let conn = conn.as_ref().expect("Database connection is not initialized");

    conn.execute(
        "INSERT INTO episode (location, name, watched, length, series_id, season_id, episode_number, year)
         VALUES (?1, ?2, false, 0, null, null, null, null)",
        params![location, name],
    )?;
    Ok(())
}

pub fn get_entries() -> Result<Vec<Entry>> {
    let conn = DB_CONN.lock().unwrap();
    let conn = conn.as_ref().expect("Database connection is not initialized");

    let mut entries = Vec::new();

    // Retrieve series
    let mut stmt = conn.prepare("SELECT id, name FROM series ORDER BY name")?;
    let series_iter = stmt.query_map([], |row| {
        Ok(Entry::Series {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    for series in series_iter {
        entries.push(series?);
    }

    // Retrieve episodes that are not part of a series
    let mut stmt = conn.prepare(
        "SELECT id, name, location, 
              COALESCE(CAST(episode.episode_number AS TEXT), '') as episode_number 
         FROM episode WHERE series_id IS NULL ORDER BY episode_number, name")?;
    let episode_iter = stmt.query_map([], |row| {
        Ok(Entry::Episode {
            id: row.get(0)?,
            name: row.get(1)?,
            location: row.get(2)?,
            episode_number: row.get(3)?,
        })
    })?;

    for episode in episode_iter {
        entries.push(episode?);
    }

    Ok(entries)
}

pub fn get_entries_for_series(series_id: i32) -> Result<Vec<Entry>> {
    let conn = DB_CONN.lock().unwrap();
    let conn = conn.as_ref().expect("Database connection is not initialized");

    let mut entries = Vec::new();

    // Retrieve episodes that are part of the series
    let mut stmt = conn.prepare(
        "SELECT id, name, location, 
              COALESCE(CAST(episode.episode_number AS TEXT), '') as episode_number 
         FROM episode WHERE series_id = ?1 ORDER BY episode_number, name")?;
    let episode_iter = stmt.query_map(params![series_id], |row| {
        Ok(Entry::Episode {
            id: row.get(0)?,
            name: row.get(1)?,
            location: row.get(2)?,
            episode_number: row.get(3)?,
        })
    })?;

    for episode in episode_iter {
        entries.push(episode?);
    }

    Ok(entries)
}

pub fn get_episode_detail(id: i32) -> Result<EpisodeDetail, Box<dyn std::error::Error>> {
    // Fetch details from the database for episode
    let conn = DB_CONN.lock().unwrap();
    let conn = conn.as_ref().expect("Database connection is not initialized");

    let mut stmt = conn.prepare(
        "SELECT 
                episode.name as title, 
                COALESCE(CAST(episode.year AS TEXT), '') as year, 
                CASE WHEN episode.watched THEN 'true' ELSE 'false' END as watched, 
                COALESCE(CAST(episode.length AS TEXT), '') as length, 
                series.id as series_id,
                COALESCE(series.name, '') as series_name, 
                season.id as season_id,
                COALESCE(CAST(season.number AS TEXT), '') as season_number, 
                COALESCE(CAST(episode.episode_number AS TEXT), '') as episode_number
            FROM episode
            LEFT JOIN season ON season.id = episode.season_id AND season.series_id = episode.series_id
            LEFT JOIN series ON series.id = episode.series_id
            WHERE episode.id = ?1"
    )?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        let series = if let Some(series_id) = row.get::<_, Option<i32>>(4)? {
            Some(Series {
                id: series_id,
                name: row.get(5)?,
            })
        } else {
            None
        };

        let season = if let Some(season_id) = row.get::<_, Option<i32>>(6)? {
            Some(Season {
                id: season_id,
                series: series.clone().expect("Season must have a series"),
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

pub fn update_episode_detail(id: i32, details: &EpisodeDetail) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_CONN.lock().unwrap();
    let conn = conn.as_ref().expect("Database connection is not initialized");

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

pub fn toggle_watched_status(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_CONN.lock().unwrap();
    let conn = conn.as_ref().expect("Database connection is not initialized");

    conn.execute(
        "UPDATE episode SET watched = NOT watched WHERE id = ?1",
        params![id],
    )?;

    Ok(())
}

pub fn get_all_series() -> Result<Vec<Series>> {
    let conn = DB_CONN.lock().unwrap();
    let conn = conn.as_ref().expect("Database connection is not initialized");

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

pub fn create_series_and_assign(name: &str, episode_id: i32) -> Result<EpisodeDetail> {
    { // Create a new scope to release the lock after the transaction
        let conn = DB_CONN.lock().unwrap();
        let conn = conn.as_ref().expect("Database connection is not initialized");
        conn.execute(
            "INSERT INTO series (name) VALUES (?1)",
            params![name],
        )?;
        let series_id = conn.last_insert_rowid() as i32;
        conn.execute(
            "UPDATE episode SET series_id = ?1 WHERE id = ?2",
            params![series_id, episode_id],
        )?;
    }
    Ok(get_episode_detail(episode_id).expect("Failed to get episode details"))
}

pub fn assign_series(series_id: i32, episode_id: i32) -> Result<EpisodeDetail> {
    { // Create a new scope to release the lock after the transaction
        let conn = DB_CONN.lock().unwrap();
        let conn = conn.as_ref().expect("Database connection is not initialized");
        conn.execute(
            "UPDATE episode SET series_id = ?1 WHERE id = ?2",
            params![series_id, episode_id],
        )?;
    }
    Ok(get_episode_detail(episode_id).expect("Failed to get episode details"))
}