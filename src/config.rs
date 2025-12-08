use serde::{Deserialize, Serialize};
use serde_yaml;
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
    
    // Scroll bar configuration
    #[serde(default = "default_scrollbar_track_char")]
    pub scrollbar_track_char: String,
    #[serde(default = "default_scrollbar_indicator_char")]
    pub scrollbar_indicator_char: String,
    #[serde(default = "default_scrollbar_fg")]
    pub scrollbar_fg: String,
    #[serde(default = "default_scrollbar_bg")]
    pub scrollbar_bg: String,
    
    // Logging configuration
    #[serde(default = "default_log_file")]
    pub log_file: Option<String>,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    
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

fn default_scrollbar_track_char() -> String {
    "│".to_string()
}

fn default_scrollbar_indicator_char() -> String {
    "█".to_string()
}

fn default_scrollbar_fg() -> String {
    "White".to_string()
}

fn default_scrollbar_bg() -> String {
    "Reset".to_string()
}

fn default_log_file() -> Option<String> {
    None
}

fn default_log_level() -> String {
    "info".to_string()
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
            scrollbar_track_char: "│".to_string(),
            scrollbar_indicator_char: "█".to_string(),
            scrollbar_fg: "White".to_string(),
            scrollbar_bg: "Reset".to_string(),
            log_file: None,
            log_level: "info".to_string(),
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
/// If the config file has missing optional fields, they will be filled with defaults.
/// 
/// # Arguments
/// * `config_path` - Path to the config.yaml file
/// 
/// # Returns
/// * `Config` - Loaded configuration or default if file doesn't exist
/// 
/// # Behavior
/// 1. If config.yaml exists, parse with serde_yaml
/// 2. If config.yaml missing, create default config.yaml
/// 3. Handle YAML parsing errors: display error, log warning, fall back to defaults
/// 4. Unknown fields are ignored (serde default behavior)
/// 5. Missing optional fields use default values (serde defaults)
pub fn read_config(config_path: &PathBuf) -> Config {
    if config_path.exists() {
        match fs::read_to_string(config_path) {
            Ok(content) => {
                match serde_yaml::from_str::<Config>(&content) {
                    Ok(config) => {
                        config
                    }
                    Err(e) => {
                        eprintln!("Error: Could not parse config.yaml: {}. Using default values.", e);
                        crate::logger::log_warn(&format!("Could not parse config.yaml: {}. Using default values.", e));
                        let default_config = Config::default();
                        // Try to write the default config
                        save_config(&default_config, config_path);
                        default_config
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: Could not read the config.yaml file. Using default values.");
                crate::logger::log_warn(&format!("Could not read config.yaml file: {}. Using default values.", e));
                Config::default()
            }
        }
    } else {
        // config.yaml doesn't exist, create default config.yaml
        let default_config = Config::default();
        save_config(&default_config, config_path);
        default_config
    }
}

/// Generate YAML configuration string with inline documentation
/// 
/// # Arguments
/// * `config` - Configuration to serialize
/// 
/// # Returns
/// * `String` - YAML string with inline comments documenting each setting
fn generate_yaml_with_comments(config: &Config) -> String {
    let mut yaml = String::new();
    
    // Database configuration
    yaml.push_str("# === Database Configuration ===\n");
    yaml.push_str("# Path to the SQLite database file\n");
    yaml.push_str("# Set to null to use default location in executable directory\n");
    if let Some(ref db_loc) = config.db_location {
        yaml.push_str(&format!("db_location: \"{}\"\n", db_loc));
    } else {
        yaml.push_str("db_location: null\n");
    }
    yaml.push_str("\n");
    
    // Color documentation header
    yaml.push_str("# === Color Configuration ===\n");
    yaml.push_str("# Valid colors: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, DarkGray, Reset\n");
    yaml.push_str("# Reset means use the terminal's default color\n");
    yaml.push_str("\n");
    
    // Current selection colors
    yaml.push_str("# Current selection colors (highlighted item in browse mode)\n");
    yaml.push_str(&format!("current_fg: {}\n", config.current_fg));
    yaml.push_str(&format!("current_bg: {}\n", config.current_bg));
    yaml.push_str("\n");
    
    // Dirty state colors
    yaml.push_str("# Dirty state colors (items with unsaved changes)\n");
    yaml.push_str(&format!("dirty_fg: {}\n", config.dirty_fg));
    yaml.push_str(&format!("dirty_bg: {}\n", config.dirty_bg));
    yaml.push_str("\n");
    
    // Watched indicator
    yaml.push_str("# Watched episode indicator\n");
    yaml.push_str("# Unicode character displayed for watched episodes\n");
    yaml.push_str(&format!("watched_indicator: \"{}\"\n", config.watched_indicator));
    yaml.push_str("# Foreground color for watched indicator\n");
    yaml.push_str(&format!("watched_fg: {}\n", config.watched_fg));
    yaml.push_str("# Style for watched indicator (none, bold, dim, italic, underline)\n");
    yaml.push_str(&format!("watched_style: {}\n", config.watched_style));
    yaml.push_str("\n");
    
    // Unwatched indicator
    yaml.push_str("# Unwatched episode indicator\n");
    yaml.push_str("# Unicode character displayed for unwatched episodes\n");
    yaml.push_str(&format!("unwatched_indicator: \"{}\"\n", config.unwatched_indicator));
    yaml.push_str("# Foreground color for unwatched indicator\n");
    yaml.push_str(&format!("unwatched_fg: {}\n", config.unwatched_fg));
    yaml.push_str("# Style for unwatched indicator (none, bold, dim, italic, underline)\n");
    yaml.push_str(&format!("unwatched_style: {}\n", config.unwatched_style));
    yaml.push_str("\n");
    
    // New episode colors
    yaml.push_str("# New episode colors (when title matches filename)\n");
    yaml.push_str(&format!("new_fg: {}\n", config.new_fg));
    yaml.push_str(&format!("new_bg: {}\n", config.new_bg));
    yaml.push_str("\n");
    
    // Invalid episode colors
    yaml.push_str("# Invalid episode colors (when video file doesn't exist)\n");
    yaml.push_str(&format!("invalid_fg: {}\n", config.invalid_fg));
    yaml.push_str(&format!("invalid_bg: {}\n", config.invalid_bg));
    yaml.push_str("\n");
    
    // Series entry colors
    yaml.push_str("# Series entry colors (for series items in browse mode)\n");
    yaml.push_str(&format!("series_fg: {}\n", config.series_fg));
    yaml.push_str(&format!("series_bg: {}\n", config.series_bg));
    yaml.push_str("\n");
    
    // Season entry colors
    yaml.push_str("# Season entry colors (for season items in browse mode)\n");
    yaml.push_str(&format!("season_fg: {}\n", config.season_fg));
    yaml.push_str(&format!("season_bg: {}\n", config.season_bg));
    yaml.push_str("\n");
    
    // Episode entry colors
    yaml.push_str("# Episode entry colors (for episode items in normal state)\n");
    yaml.push_str(&format!("episode_fg: {}\n", config.episode_fg));
    yaml.push_str(&format!("episode_bg: {}\n", config.episode_bg));
    yaml.push_str("\n");
    
    // Status line colors
    yaml.push_str("# Status line colors (bottom status bar)\n");
    yaml.push_str(&format!("status_fg: {}\n", config.status_fg));
    yaml.push_str(&format!("status_bg: {}\n", config.status_bg));
    yaml.push_str("\n");
    
    // Scroll bar configuration
    yaml.push_str("# Scroll bar configuration\n");
    yaml.push_str("# Character used for the scroll bar track\n");
    yaml.push_str(&format!("scrollbar_track_char: \"{}\"\n", config.scrollbar_track_char));
    yaml.push_str("# Character used for the scroll bar indicator\n");
    yaml.push_str(&format!("scrollbar_indicator_char: \"{}\"\n", config.scrollbar_indicator_char));
    yaml.push_str("# Foreground color for scroll bar\n");
    yaml.push_str(&format!("scrollbar_fg: {}\n", config.scrollbar_fg));
    yaml.push_str("# Background color for scroll bar\n");
    yaml.push_str(&format!("scrollbar_bg: {}\n", config.scrollbar_bg));
    yaml.push_str("\n");
    
    // Logging configuration
    yaml.push_str("# === Logging Configuration ===\n");
    yaml.push_str("# Log file location\n");
    yaml.push_str("# Set to null to use default location (app.log in executable directory)\n");
    if let Some(ref log_file) = config.log_file {
        yaml.push_str(&format!("log_file: \"{}\"\n", log_file));
    } else {
        yaml.push_str("log_file: null\n");
    }
    yaml.push_str("\n");
    
    yaml.push_str("# Log level - controls verbosity of logging\n");
    yaml.push_str("# Valid levels:\n");
    yaml.push_str("#   error - Only log errors\n");
    yaml.push_str("#   warn  - Log warnings and errors\n");
    yaml.push_str("#   info  - Log informational messages, warnings, and errors (default)\n");
    yaml.push_str("#   debug - Log all messages including detailed debugging information\n");
    yaml.push_str("# Invalid values will default to info\n");
    yaml.push_str(&format!("log_level: {}\n", config.log_level));
    yaml.push_str("\n");
    
    // Video configuration
    yaml.push_str("# === Video Configuration ===\n");
    yaml.push_str("# File extensions recognized as video files\n");
    yaml.push_str("video_extensions:\n");
    for ext in &config.video_extensions {
        yaml.push_str(&format!("  - {}\n", ext));
    }
    yaml.push_str("\n");
    
    yaml.push_str("# Path to external video player executable\n");
    yaml.push_str(&format!("video_player: {}\n", config.video_player));
    
    yaml
}



/// Save configuration to file
/// 
/// # Arguments
/// * `config` - Configuration to save
/// * `config_path` - Path to the config.yaml file
pub fn save_config(config: &Config, config_path: &PathBuf) {
    let yaml_content = generate_yaml_with_comments(config);
    if let Err(e) = fs::write(config_path, yaml_content) {
        eprintln!("Warning: Could not write config.yaml file at: {}", config_path.display());
        eprintln!("Error: {}", e);
        crate::logger::log_warn(&format!("Could not write config.yaml file at {}: {}", config_path.display(), e));
    }
}

/// Parse log level string into LogLevel enum
/// 
/// # Arguments
/// * `level_str` - Log level string (error, warn, info, debug)
/// 
/// # Returns
/// * `crate::logger::LogLevel` - Parsed log level, defaults to Info for invalid values
pub fn parse_log_level(level_str: &str) -> crate::logger::LogLevel {
    match level_str.to_lowercase().as_str() {
        "error" => crate::logger::LogLevel::Error,
        "warn" => crate::logger::LogLevel::Warn,
        "info" => crate::logger::LogLevel::Info,
        "debug" => crate::logger::LogLevel::Debug,
        _ => crate::logger::LogLevel::Info, // Default to Info for invalid values
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
        let yaml_path = temp_dir.path().join("config.yaml");

        // Create a minimal YAML config file with only required fields (missing all new color fields)
        let minimal_config = r#"current_fg: Black
current_bg: Yellow
video_extensions:
  - mp4
  - mkv
video_player: /usr/bin/vlc
"#;
        fs::write(&yaml_path, minimal_config).expect("Failed to write test config");

        // Load the config
        let config = read_config(&yaml_path);

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
        
        // Verify scrollbar fields have their default values
        assert_eq!(config.scrollbar_track_char, "│");
        assert_eq!(config.scrollbar_indicator_char, "█");
        assert_eq!(config.scrollbar_fg, "White");
        assert_eq!(config.scrollbar_bg, "Reset");
        
        // Verify logging fields have their default values
        assert_eq!(config.log_file, None);
        assert_eq!(config.log_level, "info");

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
        assert_eq!(config.scrollbar_track_char, "│");
        assert_eq!(config.scrollbar_indicator_char, "█");
        assert_eq!(config.scrollbar_fg, "White");
        assert_eq!(config.scrollbar_bg, "Reset");
        assert_eq!(config.log_file, None);
        assert_eq!(config.log_level, "info");
    }

    /// Test Case 8: Config color loading
    /// When the config file contains valid values for color configuration fields,
    /// the loaded Config should contain those values.
    /// Validates: Requirements 3.1, 4.5, 6.4
    #[test]
    fn test_config_with_all_fields_loads_correctly() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let yaml_path = temp_dir.path().join("config.yaml");

        // Create a complete YAML config file with custom values for all fields
        let complete_config = r#"current_fg: Red
current_bg: Blue
dirty_fg: Yellow
dirty_bg: Magenta
watched_indicator: "●"
watched_fg: Green
watched_style: none
unwatched_indicator: "○"
unwatched_fg: Reset
unwatched_style: none
new_fg: Yellow
new_bg: Black
invalid_fg: Magenta
invalid_bg: White
series_fg: Green
series_bg: Black
season_fg: Red
season_bg: White
episode_fg: Blue
episode_bg: Yellow
status_fg: Black
status_bg: Cyan
scrollbar_track_char: "┃"
scrollbar_indicator_char: "▓"
scrollbar_fg: Cyan
scrollbar_bg: Black
log_file: "/custom/path/app.log"
log_level: debug
video_extensions:
  - mp4
video_player: /usr/bin/mpv
"#;
        fs::write(&yaml_path, complete_config).expect("Failed to write test config");

        // Load the config
        let config = read_config(&yaml_path);

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
        assert_eq!(config.scrollbar_track_char, "┃");
        assert_eq!(config.scrollbar_indicator_char, "▓");
        assert_eq!(config.scrollbar_fg, "Cyan");
        assert_eq!(config.scrollbar_bg, "Black");
        assert_eq!(config.log_file, Some("/custom/path/app.log".to_string()));
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.video_extensions, vec!["mp4"]);
        assert_eq!(config.video_player, "/usr/bin/mpv");
    }



    #[test]
    fn test_save_config_includes_all_fields() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("config.yaml");

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
        
        // Check that all new color fields are in the saved file (YAML format)
        assert!(saved_content.contains("watched_indicator:"));
        assert!(saved_content.contains("watched_fg:"));
        assert!(saved_content.contains("unwatched_indicator:"));
        assert!(saved_content.contains("unwatched_fg:"));
        assert!(saved_content.contains("new_fg:"));
        assert!(saved_content.contains("new_bg:"));
        assert!(saved_content.contains("invalid_fg:"));
        assert!(saved_content.contains("invalid_bg:"));
        assert!(saved_content.contains("series_fg:"));
        assert!(saved_content.contains("series_bg:"));
        assert!(saved_content.contains("season_fg:"));
        assert!(saved_content.contains("season_bg:"));
        assert!(saved_content.contains("episode_fg:"));
        assert!(saved_content.contains("episode_bg:"));
        assert!(saved_content.contains("status_fg:"));
        assert!(saved_content.contains("status_bg:"));
        assert!(saved_content.contains("scrollbar_track_char:"));
        assert!(saved_content.contains("scrollbar_indicator_char:"));
        assert!(saved_content.contains("scrollbar_fg:"));
        assert!(saved_content.contains("scrollbar_bg:"));
        assert!(saved_content.contains("log_file:"));
        assert!(saved_content.contains("log_level:"));

        // Verify custom values are saved (YAML format without JSON quotes for simple values)
        assert!(saved_content.contains("current_fg: Magenta"));
        assert!(saved_content.contains("watched_indicator: \"●\""));
        assert!(saved_content.contains("unwatched_indicator: \"○\""));
        assert!(saved_content.contains("series_fg: Cyan"));
        assert!(saved_content.contains("scrollbar_track_char: \"│\""));
        assert!(saved_content.contains("scrollbar_indicator_char: \"█\""));
        
        // Verify inline documentation is present
        assert!(saved_content.contains("=== Color Configuration ==="));
        assert!(saved_content.contains("=== Logging Configuration ==="));
        assert!(saved_content.contains("Scroll bar configuration"));
    }

    /// Test Case: Default log file location when not configured
    /// When no log_file is configured in config.yaml, the Config should have log_file as None.
    /// Validates: Requirements 2.1, 2.3
    #[test]
    fn test_default_log_file_location() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let yaml_path = temp_dir.path().join("config.yaml");

        // Create a YAML config file without log_file
        let config_without_log = r#"current_fg: Black
current_bg: White
video_extensions:
  - mp4
video_player: /usr/bin/vlc
"#;
        fs::write(&yaml_path, config_without_log).expect("Failed to write test config");

        // Load the config
        let config = read_config(&yaml_path);

        // Verify log_file is None (will use standard location)
        assert_eq!(config.log_file, None);
    }

    /// Test Case: Custom log file location when configured
    /// When a log_file path is specified in config.yaml, the Config should contain that path.
    /// Validates: Requirements 2.1
    #[test]
    fn test_custom_log_file_location() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let yaml_path = temp_dir.path().join("config.yaml");

        // Create a YAML config file with custom log_file
        let config_with_log = r#"current_fg: Black
current_bg: White
log_file: "/custom/logs/myapp.log"
video_extensions:
  - mp4
video_player: /usr/bin/vlc
"#;
        fs::write(&yaml_path, config_with_log).expect("Failed to write test config");

        // Load the config
        let config = read_config(&yaml_path);

        // Verify log_file contains the custom path
        assert_eq!(config.log_file, Some("/custom/logs/myapp.log".to_string()));
    }

    /// Test Case: Default log level when not configured
    /// When no log_level is configured in config.yaml, the Config should default to "info".
    /// Validates: Requirements 2.2, 2.4
    #[test]
    fn test_default_log_level() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let yaml_path = temp_dir.path().join("config.yaml");

        // Create a YAML config file without log_level
        let config_without_level = r#"current_fg: Black
current_bg: White
video_extensions:
  - mp4
video_player: /usr/bin/vlc
"#;
        fs::write(&yaml_path, config_without_level).expect("Failed to write test config");

        // Load the config
        let config = read_config(&yaml_path);

        // Verify log_level defaults to "info"
        assert_eq!(config.log_level, "info");
    }

    /// Test Case: Valid log level parsing
    /// When valid log level values (error, warn, info, debug) are provided,
    /// parse_log_level should return the correct LogLevel enum variant.
    /// Validates: Requirements 2.2
    #[test]
    fn test_valid_log_level_parsing() {
        use crate::logger::LogLevel;

        // Test all valid log levels
        assert_eq!(parse_log_level("error"), LogLevel::Error);
        assert_eq!(parse_log_level("warn"), LogLevel::Warn);
        assert_eq!(parse_log_level("info"), LogLevel::Info);
        assert_eq!(parse_log_level("debug"), LogLevel::Debug);

        // Test case insensitivity
        assert_eq!(parse_log_level("ERROR"), LogLevel::Error);
        assert_eq!(parse_log_level("Warn"), LogLevel::Warn);
        assert_eq!(parse_log_level("INFO"), LogLevel::Info);
        assert_eq!(parse_log_level("DeBuG"), LogLevel::Debug);
    }

    /// Test Case: Invalid log level defaults to info
    /// When an invalid log_level value is provided, parse_log_level should default to Info.
    /// Validates: Requirements 2.5
    #[test]
    fn test_invalid_log_level_defaults_to_info() {
        use crate::logger::LogLevel;

        // Test various invalid values
        assert_eq!(parse_log_level("invalid"), LogLevel::Info);
        assert_eq!(parse_log_level("trace"), LogLevel::Info);
        assert_eq!(parse_log_level(""), LogLevel::Info);
        assert_eq!(parse_log_level("123"), LogLevel::Info);
        assert_eq!(parse_log_level("warning"), LogLevel::Info);
    }

    /// Test Case: YAML generation includes all required comments
    /// When generate_yaml_with_comments is called, the output should include
    /// comprehensive inline documentation for all settings.
    /// Validates: Requirements 1.5, 3.1, 4.1, 7.1, 7.2, 7.3
    #[test]
    fn test_yaml_generation_includes_comments() {
        let config = Config::default();
        let yaml = generate_yaml_with_comments(&config);

        // Verify section headers are present
        assert!(yaml.contains("=== Database Configuration ==="));
        assert!(yaml.contains("=== Color Configuration ==="));
        assert!(yaml.contains("=== Logging Configuration ==="));
        assert!(yaml.contains("=== Video Configuration ==="));

        // Verify color documentation
        assert!(yaml.contains("Valid colors: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, DarkGray, Reset"));

        // Verify log level documentation
        assert!(yaml.contains("error - Only log errors"));
        assert!(yaml.contains("warn  - Log warnings and errors"));
        assert!(yaml.contains("info  - Log informational messages, warnings, and errors"));
        assert!(yaml.contains("debug - Log all messages including detailed debugging information"));

        // Verify specific setting comments
        assert!(yaml.contains("Path to the SQLite database file"));
        assert!(yaml.contains("Current selection colors"));
        assert!(yaml.contains("Watched episode indicator"));
        assert!(yaml.contains("Unwatched episode indicator"));
        assert!(yaml.contains("New episode colors"));
        assert!(yaml.contains("Invalid episode colors"));
        assert!(yaml.contains("Series entry colors"));
        assert!(yaml.contains("Season entry colors"));
        assert!(yaml.contains("Episode entry colors"));
        assert!(yaml.contains("Status line colors"));
        assert!(yaml.contains("Log file location"));
        assert!(yaml.contains("File extensions recognized as video files"));
        assert!(yaml.contains("Path to external video player executable"));
    }

    /// Test Case: YAML generation has proper formatting
    /// When generate_yaml_with_comments is called, the output should use
    /// consistent 2-space indentation and group related settings.
    /// Validates: Requirements 7.1, 7.2, 7.3, 7.4
    #[test]
    fn test_yaml_generation_formatting() {
        let config = Config::default();
        let yaml = generate_yaml_with_comments(&config);

        // Verify 2-space indentation for list items
        assert!(yaml.contains("  - mp4"));
        assert!(yaml.contains("  - mkv"));

        // Verify blank lines between groups (check for double newlines)
        assert!(yaml.contains("\n\n"));

        // Verify comments are placed above settings (# followed by setting name)
        assert!(yaml.contains("# Current selection colors"));
        assert!(yaml.contains("current_fg:"));

        // Verify null values are properly formatted
        assert!(yaml.contains("db_location: null") || yaml.contains("db_location: \""));
        assert!(yaml.contains("log_file: null") || yaml.contains("log_file: \""));
    }

    /// Test Case: YAML generation preserves all config values
    /// When generate_yaml_with_comments is called with custom config values,
    /// the output should include all those values correctly.
    /// Validates: Requirements 1.5, 7.4, 7.5
    #[test]
    fn test_yaml_generation_preserves_values() {
        let mut config = Config::default();
        config.current_fg = "Magenta".to_string();
        config.current_bg = "Cyan".to_string();
        config.watched_indicator = "✓".to_string();
        config.unwatched_indicator = "✗".to_string();
        config.log_level = "debug".to_string();
        config.video_player = "/usr/bin/mpv".to_string();
        config.db_location = Some("/custom/path/db.sqlite".to_string());
        config.log_file = Some("/var/log/app.log".to_string());

        let yaml = generate_yaml_with_comments(&config);

        // Verify custom values are in the output
        assert!(yaml.contains("current_fg: Magenta"));
        assert!(yaml.contains("current_bg: Cyan"));
        assert!(yaml.contains("watched_indicator: \"✓\""));
        assert!(yaml.contains("unwatched_indicator: \"✗\""));
        assert!(yaml.contains("log_level: debug"));
        assert!(yaml.contains("video_player: /usr/bin/mpv"));
        assert!(yaml.contains("db_location: \"/custom/path/db.sqlite\""));
        assert!(yaml.contains("log_file: \"/var/log/app.log\""));
    }


}
