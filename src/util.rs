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

pub enum Mode {
    Browse,       // video browse
    Edit,         // video details edit
    Entry,        // initial load from disk
    SeriesSelect, // series selection
    SeriesCreate, // create a new series
    Menu,         // context menu
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


