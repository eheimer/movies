use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub path: String,
    pub current_fg: String,
    pub current_bg: String,
    pub video_extensions: Vec<String>,
    pub video_player: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: ".".to_string(),
            current_fg: "Black".to_string(),
            current_bg: "White".to_string(),
            video_extensions: vec!["mp4".to_string(), "mkv".to_string(), "avi".to_string(), "mov".to_string(), "flv".to_string(), "wmv".to_string(), "webm".to_string()],
            video_player: "/usr/bin/vlc".to_string(),
        }
    }
}

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