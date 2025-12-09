use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::logger;

/// Theme struct containing all color and style configuration
/// 
/// This struct holds all visual styling options that were previously
/// stored in the Config struct. It allows for easy theme switching
/// and separation of visual concerns from application configuration.
#[derive(Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Theme {
    // Current selection colors
    pub current_fg: String,
    pub current_bg: String,
    
    // Dirty state colors
    pub dirty_fg: String,
    pub dirty_bg: String,
    
    // Watched episode indicator
    pub watched_indicator: String,
    pub watched_fg: String,
    pub watched_style: String,
    
    // Unwatched episode indicator
    pub unwatched_indicator: String,
    pub unwatched_fg: String,
    pub unwatched_style: String,
    
    // New episode colors
    pub new_fg: String,
    pub new_bg: String,
    
    // Invalid episode colors
    pub invalid_fg: String,
    pub invalid_bg: String,
    
    // Series entry colors
    pub series_fg: String,
    pub series_bg: String,
    
    // Season entry colors
    pub season_fg: String,
    pub season_bg: String,
    
    // Episode entry colors
    pub episode_fg: String,
    pub episode_bg: String,
    
    // Status line colors
    pub status_fg: String,
    pub status_bg: String,
    
    // Scroll bar configuration
    pub scrollbar_track_char: String,
    pub scrollbar_indicator_char: String,
    pub scrollbar_fg: String,
    pub scrollbar_bg: String,
    
    // Count display styling
    pub count_fg: String,
    pub count_style: String,
    
    // Header text styling
    pub header_fg: String,
    pub header_style: String,
    
    // Help text styling
    pub help_fg: String,
    pub help_style: String,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
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
            count_fg: "DarkGray".to_string(),
            count_style: "italic".to_string(),
            header_fg: "Black".to_string(),
            header_style: "none".to_string(),
            help_fg: "Reset".to_string(),
            help_style: "none".to_string(),
        }
    }
}

/// Load a theme from a YAML file
/// 
/// If the file doesn't exist, creates a default theme file and returns the default theme.
/// If the file exists but contains invalid YAML, logs a warning and returns the default theme.
/// If the file exists but is missing fields, serde will use default values for those fields.
pub fn load_theme(theme_path: &PathBuf) -> Theme {
    if !theme_path.exists() {
        logger::log_warn(&format!("Theme file not found at {:?}, creating default theme", theme_path));
        let default_theme = Theme::default();
        save_theme(&default_theme, theme_path);
        return default_theme;
    }

    match fs::read_to_string(theme_path) {
        Ok(contents) => {
            match serde_yaml::from_str::<Theme>(&contents) {
                Ok(theme) => theme,
                Err(e) => {
                    logger::log_warn(&format!("Failed to parse theme file at {:?}: {}. Using default theme.", theme_path, e));
                    Theme::default()
                }
            }
        }
        Err(e) => {
            logger::log_warn(&format!("Failed to read theme file at {:?}: {}. Using default theme.", theme_path, e));
            Theme::default()
        }
    }
}

/// Save a theme to a YAML file with comments
/// 
/// Generates a YAML file with inline documentation comments to help users
/// understand each configuration option.
pub fn save_theme(theme: &Theme, theme_path: &PathBuf) {
    let yaml_content = generate_theme_yaml_with_comments(theme);
    
    if let Err(e) = fs::write(theme_path, yaml_content) {
        logger::log_error(&format!("Failed to save theme file to {:?}: {}", theme_path, e));
    } else {
        logger::log_info(&format!("Theme saved to {:?}", theme_path));
    }
}

/// Generate YAML content with inline documentation comments
/// 
/// Creates a human-readable YAML file with comments explaining each field,
/// valid values, and usage examples.
pub fn generate_theme_yaml_with_comments(theme: &Theme) -> String {
    format!(
        r##"# === Color Configuration ===
# Valid colors: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, DarkGray, Reset
# Reset means use the terminal's default color

# Current selection colors (highlighted item in browse mode)
current_fg: {}
current_bg: {}

# Dirty state colors (items with unsaved changes)
dirty_fg: {}
dirty_bg: {}

# Watched episode indicator
# Unicode character displayed for watched episodes
watched_indicator: "{}"
# Foreground color for watched indicator
watched_fg: {}
# Style for watched indicator (none, bold, dim, italic, underline)
watched_style: {}

# Unwatched episode indicator
# Unicode character displayed for unwatched episodes
unwatched_indicator: "{}"
# Foreground color for unwatched indicator
unwatched_fg: {}
# Style for unwatched indicator (none, bold, dim, italic, underline)
unwatched_style: {}

# New episode colors (when title matches filename)
new_fg: {}
new_bg: {}

# Invalid episode colors (when video file doesn't exist)
invalid_fg: {}
invalid_bg: {}

# Series entry colors (for series items in browse mode)
series_fg: {}
series_bg: {}

# Season entry colors (for season items in browse mode)
season_fg: {}
season_bg: {}

# Episode entry colors (for episode items in normal state)
episode_fg: {}
episode_bg: {}

# Status line colors (bottom status bar)
status_fg: {}
status_bg: {}

# Scroll bar configuration
# Character used for the scroll bar track
scrollbar_track_char: "{}"
# Character used for the scroll bar indicator
scrollbar_indicator_char: "{}"
# Foreground color for scroll bar
scrollbar_fg: {}
# Background color for scroll bar
scrollbar_bg: {}

# Count display styling (watched/unwatched counts for series and seasons)
# Foreground color for count text
count_fg: {}
# Style for count text (none, bold, dim, italic, underline)
count_style: {}

# Header text styling
# Foreground color for header text
header_fg: {}
# Style for header text (none, bold, dim, italic, underline)
header_style: {}

# Help text styling
# Foreground color for help text
help_fg: {}
# Style for help text (none, bold, dim, italic, underline)
help_style: {}
"##,
        theme.current_fg,
        theme.current_bg,
        theme.dirty_fg,
        theme.dirty_bg,
        theme.watched_indicator,
        theme.watched_fg,
        theme.watched_style,
        theme.unwatched_indicator,
        theme.unwatched_fg,
        theme.unwatched_style,
        theme.new_fg,
        theme.new_bg,
        theme.invalid_fg,
        theme.invalid_bg,
        theme.series_fg,
        theme.series_bg,
        theme.season_fg,
        theme.season_bg,
        theme.episode_fg,
        theme.episode_bg,
        theme.status_fg,
        theme.status_bg,
        theme.scrollbar_track_char,
        theme.scrollbar_indicator_char,
        theme.scrollbar_fg,
        theme.scrollbar_bg,
        theme.count_fg,
        theme.count_style,
        theme.header_fg,
        theme.header_style,
        theme.help_fg,
        theme.help_style,
    )
}

