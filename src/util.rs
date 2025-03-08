use crate::config::Config;
use std::io;
use std::path::Path;
use std::process::{Child, Command, Stdio};

#[derive(Debug, Clone)]
pub enum Entry {
    Series {
        series_id: usize,
        name: String,
    },
    Season {
        season_id: usize,
        number: usize,
        series_id: usize,
    },
    Episode {
        episode_id: usize,
        name: String,
        location: String,
        episode_number: String,
    },
}

pub enum Mode {
    Browse,       // video browse
    Edit,         // video details edit
    Entry,        // initial load from disk
    SeriesSelect, // series selection
    SeriesCreate, // create a new series
}

//TODO: this might need to be moved to database.rs or at least
// utilized by the get_all_entries function for sorting the episodes
pub fn pad_string_as_number(s: &str, width: usize) -> String {
    let mut padded = String::new();
    for _ in 0..width.saturating_sub(s.len()) {
        padded.push('0');
    }
    padded.push_str(s);
    padded
}

pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() > max_length {
        format!("{}...", &s[..max_length - 3])
    } else {
        s.to_string()
    }
}

pub fn run_video_player(config: &Config, file_path: &Path) -> io::Result<Child> {
    Command::new(&config.video_player)
        .arg(file_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}
