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
    pub video_extensions: Vec<String>,
    pub video_player: String,
}

fn default_dirty_fg() -> String {
    "Black".to_string()
}

fn default_dirty_bg() -> String {
    "White".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            db_location: None,
            current_fg: "Black".to_string(),
            current_bg: "White".to_string(),
            dirty_fg: "Black".to_string(),
            dirty_bg: "White".to_string(),
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


