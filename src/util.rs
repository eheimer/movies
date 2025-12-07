use crate::config::Config;
use crate::dto::EpisodeDetail;
use crate::path_resolver::PathResolver;
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

#[derive(Debug, Clone, PartialEq)]
pub enum EpisodeState {
    Normal,
    Watched,
    New,     // title == filename
    Invalid, // file doesn't exist
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
    Season { season_id: usize, series_id: usize, series_name: String, season_number: usize },
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

/// Determine the state of an episode based on its properties and file existence
/// 
/// Priority order: Invalid > New > Watched > Normal
/// 
/// # Arguments
/// * `entry` - The entry to check (must be an Episode)
/// * `episode_detail` - The episode details including title and watched status
/// * `resolver` - PathResolver for checking file existence
/// 
/// # Returns
/// * `EpisodeState` - The determined state of the episode
pub fn determine_episode_state(
    entry: &Entry,
    episode_detail: &EpisodeDetail,
    resolver: &PathResolver,
) -> EpisodeState {
    // Extract location from entry
    let location = match entry {
        Entry::Episode { location, .. } => location,
        _ => return EpisodeState::Normal,
    };
    
    // Priority 1: Check if file exists
    let absolute_path = resolver.to_absolute(Path::new(location));
    if !absolute_path.exists() {
        return EpisodeState::Invalid;
    }
    
    // Priority 2: Check if title equals filename
    let filename = location.rsplit('/').next().unwrap_or("");
    if episode_detail.title == filename {
        return EpisodeState::New;
    }
    
    // Priority 3: Check watched status
    if episode_detail.watched == "true" {
        return EpisodeState::Watched;
    }
    
    // Default: Normal state
    EpisodeState::Normal
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::EpisodeDetail;
    use crate::path_resolver::PathResolver;
    use std::fs;
    use tempfile::TempDir;

    /// Test Case 4: New episode color application
    /// When an episode's title field equals its filename (extracted from location),
    /// the display should apply the configured new_fg and new_bg colors.
    /// Validates: Requirements 2.1
    #[test]
    fn test_new_episode_state_detection() {
        // Create a temporary directory and database
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        // Create a test video file
        let video_file = temp_path.join("test_video.mp4");
        fs::write(&video_file, "test content").unwrap();
        
        // Create PathResolver
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
        // Create an episode entry where title equals filename
        let entry = Entry::Episode {
            episode_id: 1,
            name: "test_video.mp4".to_string(),
            location: "test_video.mp4".to_string(),
        };
        
        let episode_detail = EpisodeDetail {
            title: "test_video.mp4".to_string(), // Title equals filename
            year: "2024".to_string(),
            watched: "false".to_string(),
            length: "120".to_string(),
            series: None,
            season: None,
            episode_number: "".to_string(),
        };
        
        // Determine state
        let state = determine_episode_state(&entry, &episode_detail, &resolver);
        
        // Should be New state because title equals filename
        assert_eq!(state, EpisodeState::New);
    }

    /// Test Case 5: Invalid episode color application
    /// When an episode's file path does not exist on disk,
    /// the display should apply the configured invalid_fg and invalid_bg colors.
    /// Validates: Requirements 2.2
    #[test]
    fn test_invalid_episode_state_detection() {
        // Create a temporary directory and database
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        // Create PathResolver (but don't create the video file)
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
        // Create an episode entry pointing to non-existent file
        let entry = Entry::Episode {
            episode_id: 1,
            name: "Missing Video".to_string(),
            location: "nonexistent_video.mp4".to_string(),
        };
        
        let episode_detail = EpisodeDetail {
            title: "Missing Video".to_string(),
            year: "2024".to_string(),
            watched: "false".to_string(),
            length: "120".to_string(),
            series: None,
            season: None,
            episode_number: "".to_string(),
        };
        
        // Determine state
        let state = determine_episode_state(&entry, &episode_detail, &resolver);
        
        // Should be Invalid state because file doesn't exist
        assert_eq!(state, EpisodeState::Invalid);
    }

    #[test]
    fn test_watched_episode_state_detection() {
        // Create a temporary directory and database
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        // Create a test video file
        let video_file = temp_path.join("watched_video.mp4");
        fs::write(&video_file, "test content").unwrap();
        
        // Create PathResolver
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
        // Create an episode entry with watched status
        let entry = Entry::Episode {
            episode_id: 1,
            name: "Watched Video".to_string(),
            location: "watched_video.mp4".to_string(),
        };
        
        let episode_detail = EpisodeDetail {
            title: "Watched Video".to_string(), // Title differs from filename
            year: "2024".to_string(),
            watched: "true".to_string(), // Watched
            length: "120".to_string(),
            series: None,
            season: None,
            episode_number: "".to_string(),
        };
        
        // Determine state
        let state = determine_episode_state(&entry, &episode_detail, &resolver);
        
        // Should be Watched state
        assert_eq!(state, EpisodeState::Watched);
    }

    #[test]
    fn test_normal_episode_state_detection() {
        // Create a temporary directory and database
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        // Create a test video file
        let video_file = temp_path.join("normal_video.mp4");
        fs::write(&video_file, "test content").unwrap();
        
        // Create PathResolver
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
        // Create an episode entry in normal state
        let entry = Entry::Episode {
            episode_id: 1,
            name: "Normal Video".to_string(),
            location: "normal_video.mp4".to_string(),
        };
        
        let episode_detail = EpisodeDetail {
            title: "Normal Video".to_string(), // Title differs from filename
            year: "2024".to_string(),
            watched: "false".to_string(), // Not watched
            length: "120".to_string(),
            series: None,
            season: None,
            episode_number: "".to_string(),
        };
        
        // Determine state
        let state = determine_episode_state(&entry, &episode_detail, &resolver);
        
        // Should be Normal state
        assert_eq!(state, EpisodeState::Normal);
    }

    #[test]
    fn test_state_priority_invalid_over_new() {
        // Create a temporary directory and database
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        // Create PathResolver (but don't create the video file)
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
        // Create an episode where title equals filename BUT file doesn't exist
        let entry = Entry::Episode {
            episode_id: 1,
            name: "missing.mp4".to_string(),
            location: "missing.mp4".to_string(),
        };
        
        let episode_detail = EpisodeDetail {
            title: "missing.mp4".to_string(), // Title equals filename (would be New)
            year: "2024".to_string(),
            watched: "false".to_string(),
            length: "120".to_string(),
            series: None,
            season: None,
            episode_number: "".to_string(),
        };
        
        // Determine state
        let state = determine_episode_state(&entry, &episode_detail, &resolver);
        
        // Should be Invalid (higher priority than New)
        assert_eq!(state, EpisodeState::Invalid);
    }

    #[test]
    fn test_state_priority_new_over_watched() {
        // Create a temporary directory and database
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        // Create a test video file
        let video_file = temp_path.join("new_watched.mp4");
        fs::write(&video_file, "test content").unwrap();
        
        // Create PathResolver
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
        // Create an episode where title equals filename AND it's watched
        let entry = Entry::Episode {
            episode_id: 1,
            name: "new_watched.mp4".to_string(),
            location: "new_watched.mp4".to_string(),
        };
        
        let episode_detail = EpisodeDetail {
            title: "new_watched.mp4".to_string(), // Title equals filename (New)
            year: "2024".to_string(),
            watched: "true".to_string(), // Also watched
            length: "120".to_string(),
            series: None,
            season: None,
            episode_number: "".to_string(),
        };
        
        // Determine state
        let state = determine_episode_state(&entry, &episode_detail, &resolver);
        
        // Should be New (higher priority than Watched)
        assert_eq!(state, EpisodeState::New);
    }

    #[test]
    fn test_non_episode_entry_returns_normal() {
        // Create a temporary directory and database
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        // Create PathResolver
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
        // Create a Series entry (not an Episode)
        let entry = Entry::Series {
            series_id: 1,
            name: "Test Series".to_string(),
        };
        
        let episode_detail = EpisodeDetail {
            title: "".to_string(),
            year: "".to_string(),
            watched: "false".to_string(),
            length: "".to_string(),
            series: None,
            season: None,
            episode_number: "".to_string(),
        };
        
        // Determine state
        let state = determine_episode_state(&entry, &episode_detail, &resolver);
        
        // Should be Normal for non-Episode entries
        assert_eq!(state, EpisodeState::Normal);
    }

    /// Test Case 6: State transition from new to normal
    /// When an episode that was previously new (title == filename) has its title changed
    /// to differ from the filename, the next render should not apply new episode colors.
    /// Validates: Requirements 2.5
    #[test]
    fn test_state_transition_from_new_to_normal() {
        // Create a temporary directory and database
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        // Create a test video file
        let video_file = temp_path.join("original.mp4");
        fs::write(&video_file, "test content").unwrap();
        
        // Create PathResolver
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
        // Create an episode entry
        let entry = Entry::Episode {
            episode_id: 1,
            name: "original.mp4".to_string(),
            location: "original.mp4".to_string(),
        };
        
        // Initially, title equals filename (New state)
        let episode_detail_new = EpisodeDetail {
            title: "original.mp4".to_string(),
            year: "2024".to_string(),
            watched: "false".to_string(),
            length: "120".to_string(),
            series: None,
            season: None,
            episode_number: "".to_string(),
        };
        
        let state_before = determine_episode_state(&entry, &episode_detail_new, &resolver);
        assert_eq!(state_before, EpisodeState::New);
        
        // After editing, title differs from filename (Normal state)
        let episode_detail_edited = EpisodeDetail {
            title: "Edited Title".to_string(), // Title now differs from filename
            year: "2024".to_string(),
            watched: "false".to_string(),
            length: "120".to_string(),
            series: None,
            season: None,
            episode_number: "".to_string(),
        };
        
        let state_after = determine_episode_state(&entry, &episode_detail_edited, &resolver);
        assert_eq!(state_after, EpisodeState::Normal);
    }

    /// Test Case 7: State transition from invalid to normal
    /// When an episode that was previously invalid (file doesn't exist) has its file created
    /// at the expected path, the next render should not apply invalid episode colors.
    /// Validates: Requirements 2.6
    #[test]
    fn test_state_transition_from_invalid_to_normal() {
        // Create a temporary directory and database
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        // Create PathResolver
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
        // Create an episode entry pointing to a file that doesn't exist yet
        let entry = Entry::Episode {
            episode_id: 1,
            name: "Restored Video".to_string(),
            location: "restored.mp4".to_string(),
        };
        
        let episode_detail = EpisodeDetail {
            title: "Restored Video".to_string(),
            year: "2024".to_string(),
            watched: "false".to_string(),
            length: "120".to_string(),
            series: None,
            season: None,
            episode_number: "".to_string(),
        };
        
        // Initially, file doesn't exist (Invalid state)
        let state_before = determine_episode_state(&entry, &episode_detail, &resolver);
        assert_eq!(state_before, EpisodeState::Invalid);
        
        // Create the file
        let video_file = temp_path.join("restored.mp4");
        fs::write(&video_file, "restored content").unwrap();
        
        // After file is created, state should be Normal
        let state_after = determine_episode_state(&entry, &episode_detail, &resolver);
        assert_eq!(state_after, EpisodeState::Normal);
    }
}
