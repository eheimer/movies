use crate::config::Config;
use crate::dto::EpisodeDetail;
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
    },
    Episode {
        episode_id: usize,
        name: String,
        location: String,
    },
}

#[derive(Debug, Clone)]
pub enum LastAction {
    SeriesAssignment {
        series_id: usize,
        series_name: String,
    },
    SeasonAssignment {
        series_id: usize,
        series_name: String,
        season_id: usize,
        season_number: usize,
    },
}

impl LastAction {
    pub fn format_display(&self) -> String {
        match self {
            LastAction::SeriesAssignment { series_name, .. } => {
                format!("Last action: {}", series_name)
            }
            LastAction::SeasonAssignment {
                series_name,
                season_number,
                ..
            } => {
                format!("Last action: {}, Season {}", series_name, season_number)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ViewContext {
    TopLevel,
    Series { series_id: usize, series_name: String },
    Season { season_id: usize, series_name: String, season_number: usize },
}

#[derive(Clone, PartialEq)]
pub enum Mode {
    Browse,       // video browse
    Edit,         // video details edit
    Entry,        // initial load from disk
    SeriesSelect, // series selection
    SeriesCreate, // create a new series
    Menu,         // context menu
}

pub fn truncate_string(s: &str, max_length: usize) -> String {
    // Handle edge case where max_length is too small for ellipsis
    if max_length < 3 {
        // If max_length is 0, return empty string
        if max_length == 0 {
            return String::new();
        }
        // If max_length is 1 or 2, just truncate without ellipsis
        return s.chars().take(max_length).collect();
    }
    
    if s.len() > max_length {
        // Use saturating_sub to prevent underflow
        let truncate_at = max_length.saturating_sub(3);
        format!("{}...", &s[..truncate_at])
    } else {
        s.to_string()
    }
}

pub fn run_video_player(config: &Config, file_path: &Path) -> io::Result<Child> {
    run_video_player_with_resume(config, file_path, None)
}

/// Run video player with optional resume position
pub fn run_video_player_with_resume(
    config: &Config, 
    file_path: &Path, 
    start_time: Option<u64>
) -> io::Result<Child> {
    let mut command = Command::new(&config.video_player);
    command.arg(file_path);
    
    // Add resume position if provided
    if let Some(seconds) = start_time {
        add_resume_parameters(&mut command, &config.video_player, seconds);
    }
    
    command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

/// Add resume parameters based on video player type
fn add_resume_parameters(command: &mut Command, player_path: &str, start_seconds: u64) {
    // Extract player name from path for identification
    let player_name = std::path::Path::new(player_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // Format time for different players
    if player_name.contains("vlc") {
        // VLC: --start-time=seconds
        command.arg(format!("--start-time={}", start_seconds));
    } else if player_name.contains("mpv") {
        // mpv: --start=seconds
        command.arg(format!("--start={}", start_seconds));
    } else if player_name.contains("mplayer") {
        // MPlayer: -ss seconds
        command.arg("-ss").arg(start_seconds.to_string());
    } else if player_name.contains("ffplay") {
        // ffplay: -ss seconds
        command.arg("-ss").arg(start_seconds.to_string());
    } else {
        // Fallback: try common -ss format used by many players
        // This works with: ffmpeg-based players, some others
        command.arg("-ss").arg(start_seconds.to_string());
    }
}

pub fn can_repeat_action(
    last_action: &Option<LastAction>,
    selected_entry: &Entry,
    episode_detail: &EpisodeDetail,
) -> bool {
    // Must have a last action
    let action = match last_action {
        Some(action) => action,
        None => return false,
    };

    // Selected entry must be an Episode
    match selected_entry {
        Entry::Episode { .. } => {}
        _ => return false,
    }

    // Validate based on action type
    match action {
        LastAction::SeriesAssignment { series_id, .. } => {
            // Episode must not already be assigned to this series
            match &episode_detail.series {
                Some(series) => series.id != *series_id,
                None => true, // Unassigned episodes can be assigned
            }
        }
        LastAction::SeasonAssignment {
            series_id,
            season_id,
            ..
        } => {
            // Episode must not already be assigned to this season
            match &episode_detail.season {
                Some(season) if season.id == *season_id => false,
                _ => {
                    // If episode belongs to a different series, allow reassignment
                    // If episode has no series, allow assignment
                    // If episode belongs to same series but different season, allow assignment
                    match &episode_detail.series {
                        Some(series) if series.id != *series_id => true, // Different series - allow reassignment
                        Some(_) => true, // Same series, different season - allow assignment
                        None => true,    // No series - allow assignment
                    }
                }
            }
        }
    }
}


