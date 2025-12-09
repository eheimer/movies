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
fn generate_theme_yaml_with_comments(theme: &Theme) -> String {
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



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Test loading a valid theme file with all fields
    #[test]
    fn test_load_valid_theme() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let theme_path = temp_dir.path().join("THEME-test.yaml");

        // Create a theme file with custom values
        let theme_yaml = r#"
current_fg: Red
current_bg: Blue
dirty_fg: Yellow
dirty_bg: Green
watched_indicator: "★"
watched_fg: Cyan
watched_style: bold
unwatched_indicator: "☆"
unwatched_fg: Magenta
unwatched_style: dim
new_fg: White
new_bg: Black
invalid_fg: Red
invalid_bg: Yellow
series_fg: Green
series_bg: Reset
season_fg: Blue
season_bg: Reset
episode_fg: White
episode_bg: Reset
status_fg: Black
status_bg: White
scrollbar_track_char: "|"
scrollbar_indicator_char: "="
scrollbar_fg: Cyan
scrollbar_bg: Reset
count_fg: Gray
count_style: bold
header_fg: White
header_style: underline
help_fg: Yellow
help_style: italic
"#;
        fs::write(&theme_path, theme_yaml).expect("Failed to write theme file");

        // Load the theme
        let theme = load_theme(&theme_path);

        // Verify all fields are loaded correctly
        assert_eq!(theme.current_fg, "Red");
        assert_eq!(theme.current_bg, "Blue");
        assert_eq!(theme.dirty_fg, "Yellow");
        assert_eq!(theme.dirty_bg, "Green");
        assert_eq!(theme.watched_indicator, "★");
        assert_eq!(theme.watched_fg, "Cyan");
        assert_eq!(theme.watched_style, "bold");
        assert_eq!(theme.unwatched_indicator, "☆");
        assert_eq!(theme.unwatched_fg, "Magenta");
        assert_eq!(theme.unwatched_style, "dim");
        assert_eq!(theme.new_fg, "White");
        assert_eq!(theme.new_bg, "Black");
        assert_eq!(theme.invalid_fg, "Red");
        assert_eq!(theme.invalid_bg, "Yellow");
        assert_eq!(theme.series_fg, "Green");
        assert_eq!(theme.season_fg, "Blue");
        assert_eq!(theme.episode_fg, "White");
        assert_eq!(theme.status_fg, "Black");
        assert_eq!(theme.status_bg, "White");
        assert_eq!(theme.scrollbar_track_char, "|");
        assert_eq!(theme.scrollbar_indicator_char, "=");
        assert_eq!(theme.scrollbar_fg, "Cyan");
        assert_eq!(theme.count_fg, "Gray");
        assert_eq!(theme.count_style, "bold");
        assert_eq!(theme.header_fg, "White");
        assert_eq!(theme.header_style, "underline");
        assert_eq!(theme.help_fg, "Yellow");
        assert_eq!(theme.help_style, "italic");
    }

    /// Test loading a theme file with missing fields (should use defaults)
    #[test]
    fn test_load_theme_with_missing_fields() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let theme_path = temp_dir.path().join("THEME-partial.yaml");

        // Create a theme file with only some fields
        let theme_yaml = r#"
current_fg: Red
current_bg: Blue
watched_indicator: "★"
"#;
        fs::write(&theme_path, theme_yaml).expect("Failed to write theme file");

        // Load the theme
        let theme = load_theme(&theme_path);

        // Verify specified fields are loaded
        assert_eq!(theme.current_fg, "Red");
        assert_eq!(theme.current_bg, "Blue");
        assert_eq!(theme.watched_indicator, "★");

        // Verify missing fields use defaults
        let default_theme = Theme::default();
        assert_eq!(theme.dirty_fg, default_theme.dirty_fg);
        assert_eq!(theme.dirty_bg, default_theme.dirty_bg);
        assert_eq!(theme.unwatched_indicator, default_theme.unwatched_indicator);
        assert_eq!(theme.scrollbar_track_char, default_theme.scrollbar_track_char);
    }

    /// Test loading a non-existent theme file (should create default)
    #[test]
    fn test_load_nonexistent_theme() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let theme_path = temp_dir.path().join("THEME-nonexistent.yaml");

        // Ensure the file doesn't exist
        assert!(!theme_path.exists());

        // Load the theme (should create default)
        let theme = load_theme(&theme_path);

        // Verify the file was created
        assert!(theme_path.exists());

        // Verify the theme has default values
        let default_theme = Theme::default();
        assert_eq!(theme.current_fg, default_theme.current_fg);
        assert_eq!(theme.current_bg, default_theme.current_bg);
        assert_eq!(theme.watched_indicator, default_theme.watched_indicator);
        assert_eq!(theme.scrollbar_track_char, default_theme.scrollbar_track_char);
    }

    /// Test loading invalid YAML (should fall back to defaults)
    #[test]
    fn test_load_invalid_yaml() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let theme_path = temp_dir.path().join("THEME-invalid.yaml");

        // Create a file with invalid YAML
        let invalid_yaml = r#"
current_fg: Red
current_bg: [this is not valid yaml structure
watched_indicator: "★
"#;
        fs::write(&theme_path, invalid_yaml).expect("Failed to write theme file");

        // Load the theme (should fall back to defaults)
        let theme = load_theme(&theme_path);

        // Verify the theme has default values
        let default_theme = Theme::default();
        assert_eq!(theme.current_fg, default_theme.current_fg);
        assert_eq!(theme.current_bg, default_theme.current_bg);
        assert_eq!(theme.watched_indicator, default_theme.watched_indicator);
    }

    /// Test saving a theme with all fields
    #[test]
    fn test_save_theme() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let theme_path = temp_dir.path().join("THEME-save-test.yaml");

        // Create a custom theme
        let mut theme = Theme::default();
        theme.current_fg = "Red".to_string();
        theme.current_bg = "Blue".to_string();
        theme.watched_indicator = "★".to_string();

        // Save the theme
        save_theme(&theme, &theme_path);

        // Verify the file was created
        assert!(theme_path.exists());

        // Load the theme back and verify
        let loaded_theme = load_theme(&theme_path);
        assert_eq!(loaded_theme.current_fg, "Red");
        assert_eq!(loaded_theme.current_bg, "Blue");
        assert_eq!(loaded_theme.watched_indicator, "★");
    }

    /// Test that YAML generation includes comments
    #[test]
    fn test_yaml_generation_includes_comments() {
        let theme = Theme::default();
        let yaml_content = generate_theme_yaml_with_comments(&theme);

        // Verify comments are present
        assert!(yaml_content.contains("# === Color Configuration ==="));
        assert!(yaml_content.contains("# Valid colors:"));
        assert!(yaml_content.contains("# Current selection colors"));
        assert!(yaml_content.contains("# Watched episode indicator"));
        assert!(yaml_content.contains("# Unicode character displayed"));
        assert!(yaml_content.contains("# Scroll bar configuration"));
        assert!(yaml_content.contains("# Count display styling"));
        assert!(yaml_content.contains("# Header text styling"));
        assert!(yaml_content.contains("# Help text styling"));

        // Verify actual values are present
        assert!(yaml_content.contains("current_fg: Black"));
        assert!(yaml_content.contains("current_bg: White"));
        assert!(yaml_content.contains("watched_indicator: \"●\""));
        assert!(yaml_content.contains("scrollbar_track_char: \"│\""));
    }
}
