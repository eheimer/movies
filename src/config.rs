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
    pub fn get_database_path(&self) -> Option<PathBuf> {
        self.db_location.as_ref().map(PathBuf::from)
    }
    
    /// Set the database path and save the config
    pub fn set_database_path(&mut self, path: PathBuf) {
        self.db_location = Some(path.to_string_lossy().to_string());
    }
    
    /// Check if this is a first run (no database location configured)
    pub fn is_first_run(&self) -> bool {
        self.db_location.is_none()
    }
}

/// Read configuration from file, creating default if missing
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
pub fn generate_yaml_with_comments(config: &Config) -> String {
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
pub fn save_config(config: &Config, config_path: &PathBuf) {
    let yaml_content = generate_yaml_with_comments(config);
    if let Err(e) = fs::write(config_path, yaml_content) {
        eprintln!("Warning: Could not write config.yaml file at: {}", config_path.display());
        eprintln!("Error: {}", e);
        crate::logger::log_warn(&format!("Could not write config.yaml file at {}: {}", config_path.display(), e));
    }
}

/// Parse log level string into LogLevel enum
pub fn parse_log_level(level_str: &str) -> crate::logger::LogLevel {
    match level_str.to_lowercase().as_str() {
        "error" => crate::logger::LogLevel::Error,
        "warn" => crate::logger::LogLevel::Warn,
        "info" => crate::logger::LogLevel::Info,
        "debug" => crate::logger::LogLevel::Debug,
        _ => crate::logger::LogLevel::Info, // Default to Info for invalid values
    }
}


