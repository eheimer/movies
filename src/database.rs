use rusqlite::{params, Connection, Result};
use std::sync::Mutex;
use lazy_static::lazy_static;

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
    name: &str,
    watched: bool
) -> Result<()> {
    if episode_exists(location)? {
        return Ok(());
    }

    let conn = DB_CONN.lock().unwrap();
    let conn = conn.as_ref().expect("Database connection is not initialized");

    conn.execute(
        "INSERT INTO episode (location, name, watched, length, series_id, season_id, episode_number, year)
         VALUES (?1, ?2, ?3, 0, null, null, null, null)",
        params![location, name, watched],
    )?;
    Ok(())
}