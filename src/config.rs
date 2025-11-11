use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::path_resolver::PathResolver;

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_dir: Option<String>,
    pub path: String,
    pub current_fg: String,
    pub current_bg: String,
    pub dirty_fg: String,
    pub dirty_bg: String,
    pub video_extensions: Vec<String>,
    pub video_player: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            root_dir: None,
            path: ".".to_string(),
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
    /// Get the resolved absolute path for the video directory
    /// Uses the PathResolver to resolve the path relative to the configured root directory
    /// 
    /// # Arguments
    /// * `resolver` - PathResolver instance to use for path resolution
    /// 
    /// # Returns
    /// * `PathBuf` - Resolved absolute path to the video directory
    pub fn get_resolved_path(&self, resolver: &PathResolver) -> PathBuf {
        resolver.resolve_config_path(&self.path)
    }
}

/// Read configuration from file
/// This function reads the config without requiring a PathResolver,
/// allowing the root_dir to be extracted and used to create the PathResolver
/// 
/// # Arguments
/// * `config_path` - Path to the config.json file
/// 
/// # Returns
/// * `Config` - Loaded configuration or default if file doesn't exist
pub fn read_config(config_path: &str) -> Config {
    if Path::new(config_path).exists() {
        match fs::read_to_string(config_path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => {
                eprintln!("Error: Could not read the config.json file. Using default values.");
                Config::default()
            }
        }
    } else {
        let default_config = Config::default();
        let default_config_json = serde_json::to_string_pretty(&default_config).unwrap();
        fs::write(config_path, default_config_json).unwrap();
        default_config
    }
}
