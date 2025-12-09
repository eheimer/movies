use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_location: Option<String>,
    
    // Theme configuration
    #[serde(default = "default_active_theme")]
    pub active_theme: String,
    
    // Logging configuration
    #[serde(default = "default_log_file")]
    pub log_file: Option<String>,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    
    pub video_extensions: Vec<String>,
    pub video_player: String,
}

fn default_active_theme() -> String {
    "THEME-default.yaml".to_string()
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
            active_theme: "THEME-default.yaml".to_string(),
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
    
    // Theme configuration
    yaml.push_str("# === Theme Configuration ===\n");
    yaml.push_str("# Name of the active theme file (without path)\n");
    yaml.push_str("# Theme files are stored in the same directory as this config file\n");
    yaml.push_str("# Default: THEME-default.yaml\n");
    yaml.push_str(&format!("active_theme: {}\n", config.active_theme));
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
    /// When optional configuration fields are missing from the config file,
    /// the loaded Config should contain the default values for those fields.
    /// Validates: Requirements 3.2, 7.2
    #[test]
    fn test_missing_config_field_defaults() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let yaml_path = temp_dir.path().join("config.yaml");

        // Create a minimal YAML config file with only required fields
        let minimal_config = r#"video_extensions:
  - mp4
  - mkv
video_player: /usr/bin/vlc
"#;
        fs::write(&yaml_path, minimal_config).expect("Failed to write test config");

        // Load the config
        let config = read_config(&yaml_path);

        // Verify logging fields have their default values
        assert_eq!(config.log_file, None);
        assert_eq!(config.log_level, "info");
        
        // Verify active_theme has default value
        assert_eq!(config.active_theme, "THEME-default.yaml");

        // Verify existing fields are preserved
        assert_eq!(config.video_extensions, vec!["mp4", "mkv"]);
        assert_eq!(config.video_player, "/usr/bin/vlc");
    }

    #[test]
    fn test_config_default_has_no_style_fields() {
        // Create a default config
        let config = Config::default();

        // Verify config only has non-visual settings
        assert_eq!(config.log_file, None);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.active_theme, "THEME-default.yaml");
        assert_eq!(config.db_location, None);
        assert!(!config.video_extensions.is_empty());
        assert_eq!(config.video_player, "/usr/bin/vlc");
    }

    /// Test Case 8: Config loading with all fields
    /// When the config file contains valid values for all configuration fields,
    /// the loaded Config should contain those values.
    /// Validates: Requirements 3.1, 4.5, 6.4
    #[test]
    fn test_config_with_all_fields_loads_correctly() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let yaml_path = temp_dir.path().join("config.yaml");

        // Create a complete YAML config file with custom values for all fields
        let complete_config = r#"active_theme: THEME-custom.yaml
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
        assert_eq!(config.active_theme, "THEME-custom.yaml");
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
        config.active_theme = "THEME-custom.yaml".to_string();

        // Save the config
        save_config(&config, &config_path);

        // Read the file and verify fields are present
        let saved_content = fs::read_to_string(&config_path)
            .expect("Failed to read saved config");
        
        // Check that style fields are NOT in the saved file (they're None)
        assert!(!saved_content.contains("current_fg:"));
        assert!(!saved_content.contains("watched_indicator:"));
        assert!(!saved_content.contains("series_fg:"));
        assert!(!saved_content.contains("scrollbar_track_char:"));
        
        // Verify active_theme is in the output
        assert!(saved_content.contains("active_theme:"));
        assert!(saved_content.contains("THEME-custom.yaml"));
        
        // Verify log fields are present
        assert!(saved_content.contains("log_file:"));
        assert!(saved_content.contains("log_level:"));
        
        // Verify inline documentation is present
        assert!(saved_content.contains("=== Theme Configuration ==="));
        assert!(saved_content.contains("=== Logging Configuration ==="));
        
        // Verify color configuration section is NOT present
        assert!(!saved_content.contains("=== Color Configuration ==="));
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
        assert!(yaml.contains("=== Theme Configuration ==="));
        assert!(yaml.contains("=== Logging Configuration ==="));
        assert!(yaml.contains("=== Video Configuration ==="));

        // Verify color configuration section is NOT present
        assert!(!yaml.contains("=== Color Configuration ==="));

        // Verify log level documentation
        assert!(yaml.contains("error - Only log errors"));
        assert!(yaml.contains("warn  - Log warnings and errors"));
        assert!(yaml.contains("info  - Log informational messages, warnings, and errors"));
        assert!(yaml.contains("debug - Log all messages including detailed debugging information"));

        // Verify specific setting comments
        assert!(yaml.contains("Path to the SQLite database file"));
        assert!(yaml.contains("Name of the active theme file"));
        assert!(yaml.contains("Log file location"));
        assert!(yaml.contains("File extensions recognized as video files"));
        assert!(yaml.contains("Path to external video player executable"));
        
        // Verify style-related comments are NOT present
        assert!(!yaml.contains("Current selection colors"));
        assert!(!yaml.contains("Watched episode indicator"));
        assert!(!yaml.contains("Series entry colors"));
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

        // Verify comments are placed above settings
        assert!(yaml.contains("# Name of the active theme file"));
        assert!(yaml.contains("active_theme:"));

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
        config.active_theme = "THEME-custom.yaml".to_string();
        config.log_level = "debug".to_string();
        config.video_player = "/usr/bin/mpv".to_string();
        config.db_location = Some("/custom/path/db.sqlite".to_string());
        config.log_file = Some("/var/log/app.log".to_string());

        let yaml = generate_yaml_with_comments(&config);

        // Verify custom values are in the output
        assert!(yaml.contains("active_theme: THEME-custom.yaml"));
        assert!(yaml.contains("log_level: debug"));
        assert!(yaml.contains("video_player: /usr/bin/mpv"));
        assert!(yaml.contains("db_location: \"/custom/path/db.sqlite\""));
        assert!(yaml.contains("log_file: \"/var/log/app.log\""));
        
        // Verify style fields are NOT in the output
        assert!(!yaml.contains("current_fg:"));
        assert!(!yaml.contains("watched_indicator:"));
    }

    /// Test Case: Invalid YAML configuration parse error handling
    /// When the config file contains invalid YAML, read_config should
    /// log a warning and return default configuration.
    /// Validates: Requirements 2.4
    #[test]
    #[serial_test::serial]
    fn test_invalid_yaml_parse_error_handling() {
        use tempfile::TempDir;
        
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let yaml_path = temp_dir.path().join("config.yaml");
        let log_file = temp_dir.path().join("test_parse_error.log");
        
        // Initialize logger
        crate::logger::initialize_logger(log_file.clone(), crate::logger::LogLevel::Warn)
            .expect("Failed to initialize logger");

        // Create an invalid YAML config file (malformed syntax - unclosed quote)
        let invalid_yaml = r#"active_theme: "THEME-test.yaml
video_extensions:
  - mp4
video_player: /usr/bin/vlc
"#;
        fs::write(&yaml_path, invalid_yaml).expect("Failed to write invalid config");

        // Load the config - should return defaults and log warning
        let config = read_config(&yaml_path);

        // Verify default values are used (since parse failed)
        assert_eq!(config.active_theme, "THEME-default.yaml");
        assert_eq!(config.log_level, "info");
        
        // Log a final message to ensure flush
        crate::logger::log_warn("test_complete");
        
        // Give time for log to flush
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Verify warning was logged
        let log_contents = fs::read_to_string(&log_file)
            .expect("Failed to read log file");
        assert!(log_contents.contains("Could not parse config.yaml"));
    }

    /// Test Case: Config with active_theme field loads correctly
    /// When config.yaml contains an active_theme field, the loaded Config
    /// should contain that value.
    /// Validates: Requirements 1.4, 4.2
    #[test]
    fn test_config_with_active_theme_loads() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let yaml_path = temp_dir.path().join("config.yaml");

        let config_with_theme = r#"active_theme: THEME-custom.yaml
video_extensions:
  - mp4
video_player: /usr/bin/vlc
"#;
        fs::write(&yaml_path, config_with_theme).expect("Failed to write test config");

        let config = read_config(&yaml_path);

        assert_eq!(config.active_theme, "THEME-custom.yaml");
    }

    /// Test Case: Config without active_theme field uses default
    /// When config.yaml does not contain an active_theme field, the loaded Config
    /// should use the default value "THEME-default.yaml".
    /// Validates: Requirements 1.4, 4.2
    #[test]
    fn test_config_without_active_theme_uses_default() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let yaml_path = temp_dir.path().join("config.yaml");

        let config_without_theme = r#"video_extensions:
  - mp4
video_player: /usr/bin/vlc
"#;
        fs::write(&yaml_path, config_without_theme).expect("Failed to write test config");

        let config = read_config(&yaml_path);

        assert_eq!(config.active_theme, "THEME-default.yaml");
    }

    /// Test Case: Config serialization only includes non-visual settings
    /// When a Config is serialized, only non-visual settings should appear.
    /// Validates: Requirements 5.1
    #[test]
    fn test_config_serialization_only_includes_non_visual_settings() {
        let mut config = Config::default();
        config.active_theme = "THEME-test.yaml".to_string();
        
        // Serialize the config
        let yaml = serde_yaml::to_string(&config).expect("Failed to serialize config");

        // Verify active_theme is in the output
        assert!(yaml.contains("active_theme:"));
        
        // Verify video settings are in the output
        assert!(yaml.contains("video_extensions:"));
        assert!(yaml.contains("video_player:"));
        
        // Verify log settings are in the output
        assert!(yaml.contains("log_level:"));
    }

    /// Test Case: Config YAML generation includes active_theme documentation
    /// When generate_yaml_with_comments is called, the output should include
    /// documentation for the active_theme field.
    /// Validates: Requirements 1.4, 4.2
    #[test]
    fn test_yaml_generation_includes_active_theme_documentation() {
        let config = Config::default();
        let yaml = generate_yaml_with_comments(&config);

        // Verify theme configuration section is present
        assert!(yaml.contains("=== Theme Configuration ==="));
        assert!(yaml.contains("Name of the active theme file"));
        assert!(yaml.contains("Theme files are stored in the same directory"));
        assert!(yaml.contains("Default: THEME-default.yaml"));
        assert!(yaml.contains("active_theme: THEME-default.yaml"));
    }

    /// Test Case: Config YAML generation excludes style fields
    /// When generate_yaml_with_comments is called, the output should not
    /// include any style fields (they'll be None after migration).
    /// Validates: Requirements 5.1, 5.2
    #[test]
    fn test_yaml_generation_excludes_style_fields() {
        let config = Config::default();
        let yaml = generate_yaml_with_comments(&config);

        // Verify style fields are not in the output
        assert!(!yaml.contains("current_fg:"));
        assert!(!yaml.contains("current_bg:"));
        assert!(!yaml.contains("dirty_fg:"));
        assert!(!yaml.contains("watched_indicator:"));
        assert!(!yaml.contains("unwatched_indicator:"));
        assert!(!yaml.contains("new_fg:"));
        assert!(!yaml.contains("invalid_fg:"));
        assert!(!yaml.contains("series_fg:"));
        assert!(!yaml.contains("season_fg:"));
        assert!(!yaml.contains("episode_fg:"));
        assert!(!yaml.contains("status_fg:"));
        assert!(!yaml.contains("scrollbar_track_char:"));
        assert!(!yaml.contains("scrollbar_fg:"));
        assert!(!yaml.contains("count_fg:"));
        assert!(!yaml.contains("header_fg:"));
        assert!(!yaml.contains("help_fg:"));
        
        // Verify color configuration section is not present
        assert!(!yaml.contains("=== Color Configuration ==="));
    }

    /// Test Case: Config default includes active_theme field
    /// When Config::default() is called, the returned Config should have
    /// active_theme set to "THEME-default.yaml".
    /// Validates: Requirements 1.4, 4.2
    #[test]
    fn test_config_default_includes_active_theme() {
        let config = Config::default();

        assert_eq!(config.active_theme, "THEME-default.yaml");
        assert_eq!(config.log_level, "info");
        assert_eq!(config.log_file, None);
    }




}
