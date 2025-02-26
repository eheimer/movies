use std::process::{Command, Child, Stdio};
use std::path::Path;
use std::io;
use crate::config::Config;

#[derive(Debug, Clone)]
pub enum Entry {
  Series { id: i32, name: String},
  Episode { id: i32, name: String, location: String, episode_number: String},
}

pub enum Mode {
    Browse, // video browse
    Edit, // video details edit
    Entry, // initial load from disk
    SeriesSelect, // series selection
    SeriesCreate, // create a new series
}

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