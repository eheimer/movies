use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_location: Option<String>,
    pub current_fg: String,
    pub current_bg: String,
    #[serde(default = "default_dirty_fg")]
    pub dirty_fg: String,
    #[serde(default = "default_dirty_bg")]
    pub dirty_bg: String,
    
    // Watched episode indicator
    #[serde(default = "default_watched_indicator")]
    pub watched_indicator: String,
    #[serde(default = "default_watched_fg")]
    pub watched_fg: String,
    #[serde(default = "default_watched_style")]
    pub watched_style: String,
    
    // Unwatched episode indicator
    #[serde(default = "default_unwatched_indicator")]
    pub unwatched_indicator: String,
    #[serde(default = "default_unwatched_fg")]
    pub unwatched_fg: String,
    #[serde(default = "default_unwatched_style")]
    pub unwatched_style: String,
    
    // New episode colors (title == filename)
    #[serde(default = "default_new_fg")]
    pub new_fg: String,
    #[serde(default = "default_new_bg")]
    pub new_bg: String,
    
    // Invalid episode colors (file doesn't exist)
    #[serde(default = "default_invalid_fg")]
    pub invalid_fg: String,
    #[serde(default = "default_invalid_bg")]
    pub invalid_bg: String,
    
    // Series entry colors
    #[serde(default = "default_series_fg")]
    pub series_fg: String,
    #[serde(default = "default_series_bg")]
    pub series_bg: String,
    
    // Season entry colors
    #[serde(default = "default_season_fg")]
    pub season_fg: String,
    #[serde(default = "default_season_bg")]
    pub season_bg: String,
    
    // Episode entry colors (normal state)
    #[serde(default = "default_episode_fg")]
    pub episode_fg: String,
    #[serde(default = "default_episode_bg")]
    pub episode_bg: String,
    
    // Status line colors
    #[serde(default = "default_status_fg")]
    pub status_fg: String,
    #[serde(default = "default_status_bg")]
    pub status_bg: String,
    
    pub video_extensions: Vec<String>,
    pub video_player: String,
}

fn default_dirty_fg() -> String {
    "Black".to_string()
}

fn default_dirty_bg() -> String {
    "White".to_string()
}

fn default_watched_indicator() -> String {
    "●".to_string()
}

fn default_watched_fg() -> String {
    "Green".to_string()
}

fn default_watched_style() -> String {
    "none".to_string()
}

fn default_unwatched_indicator() -> String {
    "○".to_string()
}

fn default_unwatched_fg() -> String {
    "Reset".to_string()
}

fn default_unwatched_style() -> String {
    "none".to_string()
}

fn default_new_fg() -> String {
    "Green".to_string()
}

fn default_new_bg() -> String {
    "Reset".to_string()
}

fn default_invalid_fg() -> String {
    "Red".to_string()
}

fn default_invalid_bg() -> String {
    "Reset".to_string()
}

fn default_series_fg() -> String {
    "Blue".to_string()
}

fn default_series_bg() -> String {
    "Reset".to_string()
}

fn default_season_fg() -> String {
    "Blue".to_string()
}

fn default_season_bg() -> String {
    "Reset".to_string()
}

fn default_episode_fg() -> String {
    "Reset".to_string()
}

fn default_episode_bg() -> String {
    "Reset".to_string()
}

fn default_status_fg() -> String {
    "White".to_string()
}

fn default_status_bg() -> String {
    "DarkGray".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            db_location: None,
            current_fg: "Black".to_string(),
            current_bg: "White".to_string(),
            dirty_fg: "Black".to_string(),
            dirty_bg: "White".to_string(),
            watched_indicator: "●".to_string(),
            watched_fg: "Green".to_string(),
            watched_style: "none".to_string(),
            unwatched_indicator: "○".to_string(),
            unwatched_fg: "Reset".to_string(),
            unwatched_style: "none".to_string(),
            new_fg: "Green".to_string(),
            new_bg: "Reset".to_string(),
            invalid_fg: "Red".to_string(),
            invalid_bg: "Reset".to_string(),
            series_fg: "Blue".to_string(),
            series_bg: "Reset".to_string(),
            season_fg: "Blue".to_string(),
            season_bg: "Reset".to_string(),
            episode_fg: "Reset".to_string(),
            episode_bg: "Reset".to_string(),
            status_fg: "White".to_string(),
            status_bg: "DarkGray".to_string(),
            video_extensions: vec![
                "mp4".to_string(),
                "mkv".to_string(),
                "avi".to_string(),
                "mov".to_string(),
                "flv".to_string(),
                "wmv".to_string(),
                "webm".to_string(),
            ],
            video_player: "/usr/bin/vlc".to_string(),
        }
    }
}

impl Config {
    /// Get the database path as a PathBuf
    /// 
    /// # Returns
    /// * `Option<PathBuf>` - Database path if configured, None otherwise
    pub fn get_database_path(&self) -> Option<PathBuf> {
        self.db_location.as_ref().map(PathBuf::from)
    }
    
    /// Set the database path and save the config
    /// 
    /// # Arguments
    /// * `path` - Path to the database file
    pub fn set_database_path(&mut self, path: PathBuf) {
        self.db_location = Some(path.to_string_lossy().to_string());
    }
    
    /// Check if this is a first run (no database location configured)
    /// 
    /// # Returns
    /// * `bool` - True if db_location is None
    pub fn is_first_run(&self) -> bool {
        self.db_location.is_none()
    }
}

/// Read configuration from file
/// If the config file has missing optional fields, they will be filled with defaults
/// and the file will be updated.
/// 
/// # Arguments
/// * `config_path` - Path to the config.json file
/// 
/// # Returns
/// * `Config` - Loaded configuration or default if file doesn't exist
pub fn read_config(config_path: &PathBuf) -> Config {
    if config_path.exists() {
        match fs::read_to_string(config_path) {
            Ok(content) => {
                match serde_json::from_str::<Config>(&content) {
                    Ok(config) => {
                        // Write back the config to ensure any missing optional fields are added
                        if let Ok(updated_json) = serde_json::to_string_pretty(&config) {
                            // Only write if the content has changed
                            if updated_json != content {
                                if let Err(e) = fs::write(config_path, updated_json) {
                                    eprintln!("Warning: Could not update config.json with default values: {}", e);
                                }
                            }
                        }
                        config
                    }
                    Err(e) => {
                        eprintln!("Error: Could not parse config.json: {}. Using default values.", e);
                        let default_config = Config::default();
                        // Try to write the default config
                        save_config(&default_config, config_path);
                        default_config
                    }
                }
            }
            Err(_) => {
                eprintln!("Error: Could not read the config.json file. Using default values.");
                Config::default()
            }
        }
    } else {
        let default_config = Config::default();
        save_config(&default_config, config_path);
        default_config
    }
}

/// Save configuration to file
/// 
/// # Arguments
/// * `config` - Configuration to save
/// * `config_path` - Path to the config.json file
pub fn save_config(config: &Config, config_path: &PathBuf) {
    if let Ok(config_json) = serde_json::to_string_pretty(config) {
        if let Err(e) = fs::write(config_path, config_json) {
            eprintln!("Warning: Could not write config file at: {}", config_path.display());
            eprintln!("Error: {}", e);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Test Case 9: Missing config field defaults
    /// When color configuration fields are missing from the config file,
    /// the loaded Config should contain the default values for those fields.
    /// Validates: Requirements 3.2, 7.2
    #[test]
    fn test_missing_config_field_defaults() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("config.json");

        // Create a minimal config file with only required fields (missing all new color fields)
        let minimal_config = r#"{
  "current_fg": "Black",
  "current_bg": "Yellow",
  "video_extensions": ["mp4", "mkv"],
  "video_player": "/usr/bin/vlc"
}"#;
        fs::write(&config_path, minimal_config).expect("Failed to write test config");

        // Load the config
        let config = read_config(&config_path);

        // Verify all new color fields have their default values
        assert_eq!(config.watched_indicator, "●");
        assert_eq!(config.watched_fg, "Green");
        assert_eq!(config.watched_style, "none");
        assert_eq!(config.unwatched_indicator, "○");
        assert_eq!(config.unwatched_fg, "Reset");
        assert_eq!(config.unwatched_style, "none");
        assert_eq!(config.new_fg, "Green");
        assert_eq!(config.new_bg, "Reset");
        assert_eq!(config.invalid_fg, "Red");
        assert_eq!(config.invalid_bg, "Reset");
        assert_eq!(config.series_fg, "Blue");
        assert_eq!(config.series_bg, "Reset");
        assert_eq!(config.season_fg, "Blue");
        assert_eq!(config.season_bg, "Reset");
        assert_eq!(config.episode_fg, "Reset");
        assert_eq!(config.episode_bg, "Reset");
        assert_eq!(config.status_fg, "White");
        assert_eq!(config.status_bg, "DarkGray");

        // Verify existing fields are preserved
        assert_eq!(config.current_fg, "Black");
        assert_eq!(config.current_bg, "Yellow");
        assert_eq!(config.video_extensions, vec!["mp4", "mkv"]);
        assert_eq!(config.video_player, "/usr/bin/vlc");
    }

    #[test]
    fn test_config_default_includes_all_color_fields() {
        // Create a default config
        let config = Config::default();

        // Verify all color fields are present with expected defaults
        assert_eq!(config.current_fg, "Black");
        assert_eq!(config.current_bg, "White");
        assert_eq!(config.dirty_fg, "Black");
        assert_eq!(config.dirty_bg, "White");
        assert_eq!(config.watched_indicator, "●");
        assert_eq!(config.watched_fg, "Green");
        assert_eq!(config.watched_style, "none");
        assert_eq!(config.unwatched_indicator, "○");
        assert_eq!(config.unwatched_fg, "Reset");
        assert_eq!(config.unwatched_style, "none");
        assert_eq!(config.new_fg, "Green");
        assert_eq!(config.new_bg, "Reset");
        assert_eq!(config.invalid_fg, "Red");
        assert_eq!(config.invalid_bg, "Reset");
        assert_eq!(config.series_fg, "Blue");
        assert_eq!(config.series_bg, "Reset");
        assert_eq!(config.season_fg, "Blue");
        assert_eq!(config.season_bg, "Reset");
        assert_eq!(config.episode_fg, "Reset");
        assert_eq!(config.episode_bg, "Reset");
        assert_eq!(config.status_fg, "White");
        assert_eq!(config.status_bg, "DarkGray");
    }

    /// Test Case 8: Config color loading
    /// When the config file contains valid values for color configuration fields,
    /// the loaded Config should contain those values.
    /// Validates: Requirements 3.1, 4.5, 6.4
    #[test]
    fn test_config_with_all_fields_loads_correctly() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("config.json");

        // Create a complete config file with custom values for all fields
        let complete_config = r#"{
  "current_fg": "Red",
  "current_bg": "Blue",
  "dirty_fg": "Yellow",
  "dirty_bg": "Magenta",
  "watched_indicator": "●",
  "watched_fg": "Green",
  "watched_style": "none",
  "unwatched_indicator": "○",
  "unwatched_fg": "Reset",
  "unwatched_style": "none",
  "new_fg": "Yellow",
  "new_bg": "Black",
  "invalid_fg": "Magenta",
  "invalid_bg": "White",
  "series_fg": "Green",
  "series_bg": "Black",
  "season_fg": "Red",
  "season_bg": "White",
  "episode_fg": "Blue",
  "episode_bg": "Yellow",
  "status_fg": "Black",
  "status_bg": "Cyan",
  "video_extensions": ["mp4"],
  "video_player": "/usr/bin/mpv"
}"#;
        fs::write(&config_path, complete_config).expect("Failed to write test config");

        // Load the config
        let config = read_config(&config_path);

        // Verify all custom values are loaded correctly
        assert_eq!(config.current_fg, "Red");
        assert_eq!(config.current_bg, "Blue");
        assert_eq!(config.dirty_fg, "Yellow");
        assert_eq!(config.dirty_bg, "Magenta");
        assert_eq!(config.watched_indicator, "●");
        assert_eq!(config.watched_fg, "Green");
        assert_eq!(config.watched_style, "none");
        assert_eq!(config.unwatched_indicator, "○");
        assert_eq!(config.unwatched_fg, "Reset");
        assert_eq!(config.unwatched_style, "none");
        assert_eq!(config.new_fg, "Yellow");
        assert_eq!(config.new_bg, "Black");
        assert_eq!(config.invalid_fg, "Magenta");
        assert_eq!(config.invalid_bg, "White");
        assert_eq!(config.series_fg, "Green");
        assert_eq!(config.series_bg, "Black");
        assert_eq!(config.season_fg, "Red");
        assert_eq!(config.season_bg, "White");
        assert_eq!(config.episode_fg, "Blue");
        assert_eq!(config.episode_bg, "Yellow");
        assert_eq!(config.status_fg, "Black");
        assert_eq!(config.status_bg, "Cyan");
        assert_eq!(config.video_extensions, vec!["mp4"]);
        assert_eq!(config.video_player, "/usr/bin/mpv");
    }

    #[test]
    fn test_backward_compatibility_with_old_config() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("config.json");

        // Create an old-style config file (before color enhancements were added)
        // This simulates a user upgrading from an older version
        let old_config = r#"{
  "current_fg": "Black",
  "current_bg": "Yellow",
  "video_extensions": ["mp4", "mkv", "avi"],
  "video_player": "/usr/bin/vlc"
}"#;
        fs::write(&config_path, old_config).expect("Failed to write test config");

        // Load the config
        let config = read_config(&config_path);

        // Verify old fields are preserved
        assert_eq!(config.current_fg, "Black");
        assert_eq!(config.current_bg, "Yellow");
        assert_eq!(config.video_extensions, vec!["mp4", "mkv", "avi"]);
        assert_eq!(config.video_player, "/usr/bin/vlc");

        // Verify new fields have defaults
        assert_eq!(config.watched_indicator, "●");
        assert_eq!(config.watched_fg, "Green");
        assert_eq!(config.unwatched_indicator, "○");
        assert_eq!(config.unwatched_fg, "Reset");
        assert_eq!(config.new_fg, "Green");
        assert_eq!(config.new_bg, "Reset");
        assert_eq!(config.invalid_fg, "Red");
        assert_eq!(config.invalid_bg, "Reset");
        assert_eq!(config.series_fg, "Blue");
        assert_eq!(config.series_bg, "Reset");
        assert_eq!(config.season_fg, "Blue");
        assert_eq!(config.season_bg, "Reset");
        assert_eq!(config.episode_fg, "Reset");
        assert_eq!(config.episode_bg, "Reset");
        assert_eq!(config.status_fg, "White");
        assert_eq!(config.status_bg, "DarkGray");

        // Verify the config file was updated with the new fields
        let updated_content = fs::read_to_string(&config_path)
            .expect("Failed to read updated config");
        assert!(updated_content.contains("watched_indicator"));
        assert!(updated_content.contains("unwatched_indicator"));
        assert!(updated_content.contains("series_fg"));
        assert!(updated_content.contains("status_bg"));
    }

    #[test]
    fn test_save_config_includes_all_fields() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("config.json");

        // Create a config with custom values
        let mut config = Config::default();
        config.current_fg = "Magenta".to_string();
        config.watched_indicator = "●".to_string();
        config.series_fg = "Cyan".to_string();

        // Save the config
        save_config(&config, &config_path);

        // Read the file and verify all fields are present
        let saved_content = fs::read_to_string(&config_path)
            .expect("Failed to read saved config");
        
        // Check that all new color fields are in the saved file
        assert!(saved_content.contains("watched_indicator"));
        assert!(saved_content.contains("watched_fg"));
        assert!(saved_content.contains("unwatched_indicator"));
        assert!(saved_content.contains("unwatched_fg"));
        assert!(saved_content.contains("new_fg"));
        assert!(saved_content.contains("new_bg"));
        assert!(saved_content.contains("invalid_fg"));
        assert!(saved_content.contains("invalid_bg"));
        assert!(saved_content.contains("series_fg"));
        assert!(saved_content.contains("series_bg"));
        assert!(saved_content.contains("season_fg"));
        assert!(saved_content.contains("season_bg"));
        assert!(saved_content.contains("episode_fg"));
        assert!(saved_content.contains("episode_bg"));
        assert!(saved_content.contains("status_fg"));
        assert!(saved_content.contains("status_bg"));

        // Verify custom values are saved
        assert!(saved_content.contains("\"Magenta\""));
        assert!(saved_content.contains("\"●\""));
        assert!(saved_content.contains("\"○\""));
        assert!(saved_content.contains("\"Cyan\""));
    }
}
