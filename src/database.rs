use rusqlite::{params, Connection, Result};
use crate::util::Entry;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct EntryDetails{
    pub title: String,
    pub year: String,
    pub watched: String,
    pub length: String,
    pub series: String,
    pub season: String,
    pub episode_number: String,
}

lazy_static! {
    pub static ref DB_CONN: Mutex<Option<Connection>> = Mutex::new(None);
}

fn initialize_db_connection(db_path: &str) {
    let conn = Connection::open(db_path).expect("Failed to initialize database");
    let mut db_conn = DB_CONN.lock().unwrap();
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
    let mut stmt = conn.prepare("SELECT id, name FROM series")?;
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
         FROM episode WHERE series_id IS NULL")?;
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

pub fn get_entry_details(entry: &Entry) -> Result<EntryDetails, Box<dyn std::error::Error>> {
     match entry {
        Entry::Series { name, .. } => {
            // Return sample data for series
            Ok(EntryDetails {
                title: name.clone(),
                year: "2025".to_string(),
                watched: "No".to_string(),
                length: "N/A".to_string(),
                series: name.clone(),
                season: "N/A".to_string(),
                episode_number: "N/A".to_string(),
            })
        }
        Entry::Episode { id, .. } => {
            // Fetch details from the database for episode
            let conn = Connection::open("videos.db")?;
            let mut stmt = conn.prepare(
                "SELECT episode.name as title, 
                        COALESCE(CAST(episode.year AS TEXT), '') as year, 
                        CASE WHEN episode.watched THEN 'true' ELSE 'false' END as watched, 
                        COALESCE(CAST(episode.length AS TEXT), '') as length, 
                        COALESCE(series.name, '') as series, 
                        COALESCE(CAST(season.number AS TEXT), '') as season, 
                        COALESCE(CAST(episode.episode_number AS TEXT), '') as episode_number
                 FROM episode
                 LEFT JOIN season ON season.id = episode.season_id
                 LEFT JOIN series ON series.id = episode.series_id
                 WHERE episode.id = ?1"
            )?;
            let mut rows = stmt.query(params![id])?;

            if let Some(row) = rows.next()? {
                Ok(EntryDetails {
                    title: row.get(0)?,
                    year: row.get(1)?,
                    watched: row.get(2)?,
                    length: row.get(3)?,
                    series: row.get(4)?,
                    season: row.get(5)?,
                    episode_number: row.get(6)?,
                })
            } else {
                Err("Episode not found".into())
            }
        }
    }
}

pub fn update_entry_details(entry: &Entry, details: &EntryDetails) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("videos.db")?;
    match entry {
        Entry::Series { id, .. } => {
            conn.execute(
                "UPDATE series SET name = ?1 WHERE id = ?2",
                params![details.title, id],
            )?;
        }
        Entry::Episode { id, .. } => {
            conn.execute(
                "UPDATE episode SET name = ?1, year = ?2, watched = ?3, length = ?4, series_id = (SELECT id FROM series WHERE name = ?5), season_id = (SELECT id FROM season WHERE number = ?6), episode_number = ?7 WHERE id = ?8",
                params![details.title, details.year, details.watched == "true", details.length, details.series, details.season, details.episode_number, id],
            )?;
        }
    }
    Ok(())
}

pub fn toggle_watched_status(entry: &Entry) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("videos.db")?;
    match entry {
        Entry::Episode { id, .. } => {
            conn.execute(
                "UPDATE episode SET watched = NOT watched WHERE id = ?1",
                params![id],
            )?;
        }
        _ => {}
    }
    Ok(())
}