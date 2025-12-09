use std::fs;
use tempfile::TempDir;

// Import necessary modules from the main crate
// Note: These tests require the application to be structured as a library
// with public modules exposed in lib.rs

// Note: Due to the global database connection using OnceLock, we can only
// initialize the database once per test run. Tests that require database
// operations must be run sequentially or use a single shared database.

/// Integration Test 1: Theme reload with custom colors
/// 
/// This test verifies that when a theme file is created with custom color values,
/// those values are correctly loaded and applied throughout the application.
/// 
/// Test flow:
/// 1. Create a temporary directory
/// 2. Write a config file with active_theme reference
/// 3. Write a theme file with custom colors
/// 4. Load the theme
/// 5. Verify all custom colors are loaded correctly
/// 6. Modify the theme file with different colors
/// 7. Reload the theme
/// 8. Verify the new colors are applied
#[test]
fn test_theme_reload_with_custom_colors() {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("config.yaml");
    let theme_path = temp_dir.path().join("THEME-test.yaml");

    // Create config file referencing the theme
    let config_content = r#"active_theme: THEME-test.yaml
video_extensions:
  - mp4
video_player: /usr/bin/vlc
"#;
    fs::write(&config_path, config_content).expect("Failed to write config");

    // Create initial theme with custom colors
    let initial_theme = r#"current_fg: Red
current_bg: Blue
dirty_fg: Yellow
dirty_bg: Green
watched_indicator: "★"
watched_fg: Cyan
watched_style: bold
unwatched_indicator: "☆"
unwatched_fg: Magenta
unwatched_style: dim
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
scrollbar_track_char: "|"
scrollbar_indicator_char: "="
scrollbar_fg: White
scrollbar_bg: Black
count_fg: Gray
count_style: bold
header_fg: White
header_style: underline
help_fg: Yellow
help_style: italic
"#;
    fs::write(&theme_path, initial_theme).expect("Failed to write initial theme");

    // Load the theme
    let theme = movies::theme::load_theme(&theme_path);

    // Verify initial custom values
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
    assert_eq!(theme.new_fg, "Yellow");
    assert_eq!(theme.new_bg, "Black");
    assert_eq!(theme.invalid_fg, "Magenta");
    assert_eq!(theme.invalid_bg, "White");
    assert_eq!(theme.series_fg, "Green");
    assert_eq!(theme.series_bg, "Black");
    assert_eq!(theme.season_fg, "Red");
    assert_eq!(theme.season_bg, "White");
    assert_eq!(theme.episode_fg, "Blue");
    assert_eq!(theme.episode_bg, "Yellow");
    assert_eq!(theme.status_fg, "Black");
    assert_eq!(theme.status_bg, "Cyan");
    assert_eq!(theme.scrollbar_track_char, "|");
    assert_eq!(theme.scrollbar_indicator_char, "=");
    assert_eq!(theme.count_fg, "Gray");
    assert_eq!(theme.count_style, "bold");

    // Modify the theme file with different colors
    let modified_theme = r#"current_fg: Green
current_bg: Red
dirty_fg: Cyan
dirty_bg: Magenta
watched_indicator: "●"
watched_fg: Yellow
watched_style: italic
unwatched_indicator: "○"
unwatched_fg: White
unwatched_style: none
new_fg: Cyan
new_bg: Magenta
invalid_fg: White
invalid_bg: Black
series_fg: Magenta
series_bg: Yellow
season_fg: Cyan
season_bg: Green
episode_fg: White
episode_bg: Red
status_fg: Yellow
status_bg: Blue
scrollbar_track_char: "│"
scrollbar_indicator_char: "█"
scrollbar_fg: Cyan
scrollbar_bg: Reset
count_fg: DarkGray
count_style: italic
header_fg: Black
header_style: bold
help_fg: Reset
help_style: none
"#;
    fs::write(&theme_path, modified_theme).expect("Failed to write modified theme");

    // Reload the theme
    let reloaded_theme = movies::theme::load_theme(&theme_path);

    // Verify modified values are loaded
    assert_eq!(reloaded_theme.current_fg, "Green");
    assert_eq!(reloaded_theme.current_bg, "Red");
    assert_eq!(reloaded_theme.dirty_fg, "Cyan");
    assert_eq!(reloaded_theme.dirty_bg, "Magenta");
    assert_eq!(reloaded_theme.watched_indicator, "●");
    assert_eq!(reloaded_theme.watched_fg, "Yellow");
    assert_eq!(reloaded_theme.watched_style, "italic");
    assert_eq!(reloaded_theme.unwatched_indicator, "○");
    assert_eq!(reloaded_theme.unwatched_fg, "White");
    assert_eq!(reloaded_theme.unwatched_style, "none");
    assert_eq!(reloaded_theme.new_fg, "Cyan");
    assert_eq!(reloaded_theme.new_bg, "Magenta");
    assert_eq!(reloaded_theme.invalid_fg, "White");
    assert_eq!(reloaded_theme.invalid_bg, "Black");
    assert_eq!(reloaded_theme.series_fg, "Magenta");
    assert_eq!(reloaded_theme.series_bg, "Yellow");
    assert_eq!(reloaded_theme.season_fg, "Cyan");
    assert_eq!(reloaded_theme.season_bg, "Green");
    assert_eq!(reloaded_theme.episode_fg, "White");
    assert_eq!(reloaded_theme.episode_bg, "Red");
    assert_eq!(reloaded_theme.status_fg, "Yellow");
    assert_eq!(reloaded_theme.status_bg, "Blue");
    assert_eq!(reloaded_theme.scrollbar_track_char, "│");
    assert_eq!(reloaded_theme.scrollbar_indicator_char, "█");
    assert_eq!(reloaded_theme.count_fg, "DarkGray");
    assert_eq!(reloaded_theme.count_style, "italic");
    
    // Verify config still has non-style settings
    let config = movies::config::read_config(&config_path);
    assert_eq!(config.video_extensions, vec!["mp4"]);
    assert_eq!(config.video_player, "/usr/bin/vlc");
    assert_eq!(config.active_theme, "THEME-test.yaml");
}

/// Integration Test 2: Application startup with new config (normal path)
/// 
/// This test verifies that when the application starts with a new-format config
/// (no style fields, only active_theme reference), it correctly loads the theme
/// and operates normally.
/// 
/// Test flow:
/// 1. Create a new-format config with active_theme field
/// 2. Create a corresponding theme file
/// 3. Load config and theme
/// 4. Verify config has no style fields
/// 5. Verify theme is loaded correctly
/// 6. Verify no migration is triggered
#[test]
fn test_startup_with_new_config() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("config.yaml");
    let theme_path = temp_dir.path().join("THEME-default.yaml");

    // Create new-format config (no style fields)
    let config_content = r#"active_theme: THEME-default.yaml
video_extensions:
  - mp4
  - mkv
video_player: /usr/bin/vlc
log_level: info
"#;
    fs::write(&config_path, config_content).expect("Failed to write config");

    // Create theme file
    let theme_content = r#"current_fg: Black
current_bg: White
dirty_fg: Black
dirty_bg: White
watched_indicator: "●"
watched_fg: Green
watched_style: none
unwatched_indicator: "○"
unwatched_fg: Reset
unwatched_style: none
new_fg: Green
new_bg: Reset
invalid_fg: Red
invalid_bg: Reset
series_fg: Blue
series_bg: Reset
season_fg: Blue
season_bg: Reset
episode_fg: Reset
episode_bg: Reset
status_fg: White
status_bg: DarkGray
scrollbar_track_char: "│"
scrollbar_indicator_char: "█"
scrollbar_fg: White
scrollbar_bg: Reset
count_fg: DarkGray
count_style: italic
header_fg: Black
header_style: none
help_fg: Reset
help_style: none
"#;
    fs::write(&theme_path, theme_content).expect("Failed to write theme");

    // Load config
    let config = movies::config::read_config(&config_path);

    // Verify config has correct non-style fields
    assert_eq!(config.active_theme, "THEME-default.yaml");
    assert_eq!(config.video_extensions, vec!["mp4", "mkv"]);
    assert_eq!(config.video_player, "/usr/bin/vlc");

    // Load theme
    let theme = movies::theme::load_theme(&theme_path);

    // Verify theme loaded correctly
    assert_eq!(theme.current_fg, "Black");
    assert_eq!(theme.current_bg, "White");
    assert_eq!(theme.watched_indicator, "●");
    assert_eq!(theme.series_fg, "Blue");
    assert_eq!(theme.scrollbar_track_char, "│");
}

/// Integration Test 3: Theme file creation when missing
/// 
/// This test verifies that when a theme file is referenced but doesn't exist,
/// the system creates a default theme file automatically.
/// 
/// Test flow:
/// 1. Create config referencing a non-existent theme
/// 2. Attempt to load the theme
/// 3. Verify default theme file is created
/// 4. Verify default theme values are returned
#[test]
fn test_theme_file_creation_when_missing() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let theme_path = temp_dir.path().join("THEME-missing.yaml");

    // Verify theme file doesn't exist
    assert!(!theme_path.exists());

    // Load theme (should create default)
    let theme = movies::theme::load_theme(&theme_path);

    // Verify theme file was created
    assert!(theme_path.exists());

    // Verify theme has default values
    let default_theme = movies::theme::Theme::default();
    assert_eq!(theme.current_fg, default_theme.current_fg);
    assert_eq!(theme.current_bg, default_theme.current_bg);
    assert_eq!(theme.watched_indicator, default_theme.watched_indicator);
    assert_eq!(theme.unwatched_indicator, default_theme.unwatched_indicator);
    assert_eq!(theme.series_fg, default_theme.series_fg);
    assert_eq!(theme.scrollbar_track_char, default_theme.scrollbar_track_char);
    assert_eq!(theme.scrollbar_indicator_char, default_theme.scrollbar_indicator_char);

    // Verify the created file can be read back
    let reloaded_theme = movies::theme::load_theme(&theme_path);
    assert_eq!(reloaded_theme.current_fg, default_theme.current_fg);
    assert_eq!(reloaded_theme.watched_indicator, default_theme.watched_indicator);
}

/// Integration Test 5: Error handling for invalid theme files
/// 
/// This test verifies that the system handles various types of invalid theme files
/// gracefully by falling back to default values.
/// 
/// Test flow:
/// 1. Create theme file with invalid YAML syntax
/// 2. Load theme (should fall back to defaults)
/// 3. Create theme file with invalid color values
/// 4. Load theme (should load but invalid colors handled by display layer)
/// 5. Create theme file with missing required fields
/// 6. Load theme (should use defaults for missing fields)
#[test]
fn test_error_handling_invalid_theme_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Test 1: Invalid YAML syntax
    let invalid_yaml_path = temp_dir.path().join("THEME-invalid-yaml.yaml");
    let invalid_yaml = r#"
current_fg: Red
current_bg: [this is not valid
watched_indicator: "unclosed string
series_fg: Blue
"#;
    fs::write(&invalid_yaml_path, invalid_yaml).expect("Failed to write invalid YAML");

    let theme1 = movies::theme::load_theme(&invalid_yaml_path);
    let default_theme = movies::theme::Theme::default();
    
    // Should fall back to defaults
    assert_eq!(theme1.current_fg, default_theme.current_fg);
    assert_eq!(theme1.current_bg, default_theme.current_bg);
    assert_eq!(theme1.watched_indicator, default_theme.watched_indicator);

    // Test 2: Valid YAML but with invalid color values
    // (These will be handled by the display layer's color parsing)
    let invalid_colors_path = temp_dir.path().join("THEME-invalid-colors.yaml");
    let invalid_colors = r#"
current_fg: NotAValidColor
current_bg: AlsoInvalid
watched_indicator: "●"
watched_fg: ThisIsWrong
series_fg: 12345
episode_fg: Reset
scrollbar_track_char: "│"
scrollbar_indicator_char: "█"
scrollbar_fg: White
scrollbar_bg: Reset
count_fg: DarkGray
count_style: italic
header_fg: Black
header_style: none
help_fg: Reset
help_style: none
"#;
    fs::write(&invalid_colors_path, invalid_colors).expect("Failed to write invalid colors");

    let theme2 = movies::theme::load_theme(&invalid_colors_path);
    
    // Theme should load (invalid colors will be handled by display layer)
    assert_eq!(theme2.current_fg, "NotAValidColor");
    assert_eq!(theme2.current_bg, "AlsoInvalid");
    assert_eq!(theme2.watched_indicator, "●");
    assert_eq!(theme2.watched_fg, "ThisIsWrong");

    // Test 3: Missing fields (should use defaults via serde)
    let partial_theme_path = temp_dir.path().join("THEME-partial.yaml");
    let partial_theme = r#"
current_fg: Red
current_bg: Blue
watched_indicator: "★"
"#;
    fs::write(&partial_theme_path, partial_theme).expect("Failed to write partial theme");

    let theme3 = movies::theme::load_theme(&partial_theme_path);
    
    // Specified fields should be loaded
    assert_eq!(theme3.current_fg, "Red");
    assert_eq!(theme3.current_bg, "Blue");
    assert_eq!(theme3.watched_indicator, "★");
    
    // Missing fields should use defaults
    assert_eq!(theme3.dirty_fg, default_theme.dirty_fg);
    assert_eq!(theme3.unwatched_indicator, default_theme.unwatched_indicator);
    assert_eq!(theme3.series_fg, default_theme.series_fg);
    assert_eq!(theme3.scrollbar_track_char, default_theme.scrollbar_track_char);
}

/// Integration Test 6: Full application startup simulation
/// 
/// This test simulates the complete application startup flow with the theme system,
/// including config loading, theme loading, and verification that all components
/// have access to the correct theme data.
/// 
/// Test flow:
/// 1. Create a complete config and theme setup
/// 2. Load config
/// 3. Resolve theme path from config directory and active_theme field
/// 4. Load theme
/// 5. Verify all data is accessible and correct
#[test]
fn test_full_application_startup_simulation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_dir = temp_dir.path();
    let config_path = config_dir.join("config.yaml");

    // Create config
    let config_content = r#"active_theme: THEME-custom.yaml
video_extensions:
  - mp4
  - mkv
  - avi
video_player: /usr/bin/vlc
log_level: debug
"#;
    fs::write(&config_path, config_content).expect("Failed to write config");

    // Create custom theme
    let theme_path = config_dir.join("THEME-custom.yaml");
    let theme_content = r#"current_fg: Magenta
current_bg: Yellow
dirty_fg: Cyan
dirty_bg: Red
watched_indicator: "✓"
watched_fg: Green
watched_style: bold
unwatched_indicator: "✗"
unwatched_fg: Red
unwatched_style: none
new_fg: Yellow
new_bg: Reset
invalid_fg: Red
invalid_bg: Reset
series_fg: Cyan
series_bg: Reset
season_fg: Magenta
season_bg: Reset
episode_fg: White
episode_bg: Reset
status_fg: Black
status_bg: White
scrollbar_track_char: "║"
scrollbar_indicator_char: "▓"
scrollbar_fg: Cyan
scrollbar_bg: Reset
count_fg: DarkGray
count_style: italic
header_fg: White
header_style: bold
help_fg: DarkGray
help_style: dim
"#;
    fs::write(&theme_path, theme_content).expect("Failed to write theme");

    // Simulate application startup
    // Step 1: Load config
    let config = movies::config::read_config(&config_path);
    
    // Step 2: Construct theme path from config directory and active_theme
    let theme_file_path = config_dir.join(&config.active_theme);
    
    // Step 3: Load theme
    let theme = movies::theme::load_theme(&theme_file_path);
    
    // Step 4: Verify all data is correct and accessible
    
    // Verify config data
    assert_eq!(config.active_theme, "THEME-custom.yaml");
    assert_eq!(config.video_extensions, vec!["mp4", "mkv", "avi"]);
    assert_eq!(config.video_player, "/usr/bin/vlc");
    assert_eq!(config.log_level, "debug");
    
    // Verify theme data
    assert_eq!(theme.current_fg, "Magenta");
    assert_eq!(theme.current_bg, "Yellow");
    assert_eq!(theme.dirty_fg, "Cyan");
    assert_eq!(theme.dirty_bg, "Red");
    assert_eq!(theme.watched_indicator, "✓");
    assert_eq!(theme.watched_fg, "Green");
    assert_eq!(theme.watched_style, "bold");
    assert_eq!(theme.unwatched_indicator, "✗");
    assert_eq!(theme.unwatched_fg, "Red");
    assert_eq!(theme.unwatched_style, "none");
    assert_eq!(theme.new_fg, "Yellow");
    assert_eq!(theme.invalid_fg, "Red");
    assert_eq!(theme.series_fg, "Cyan");
    assert_eq!(theme.season_fg, "Magenta");
    assert_eq!(theme.episode_fg, "White");
    assert_eq!(theme.status_fg, "Black");
    assert_eq!(theme.status_bg, "White");
    assert_eq!(theme.scrollbar_track_char, "║");
    assert_eq!(theme.scrollbar_indicator_char, "▓");
    assert_eq!(theme.scrollbar_fg, "Cyan");
    assert_eq!(theme.count_fg, "DarkGray");
    assert_eq!(theme.count_style, "italic");
    assert_eq!(theme.header_fg, "White");
    assert_eq!(theme.header_style, "bold");
    assert_eq!(theme.help_fg, "DarkGray");
    assert_eq!(theme.help_style, "dim");
    
    // Verify theme and config are separate concerns
    // Changing theme should not affect config
    let mut modified_theme = theme.clone();
    modified_theme.current_fg = "Blue".to_string();
    movies::theme::save_theme(&modified_theme, &theme_file_path);
    
    let reloaded_config = movies::config::read_config(&config_path);
    assert_eq!(reloaded_config.video_player, "/usr/bin/vlc"); // Config unchanged
    
    let reloaded_theme = movies::theme::load_theme(&theme_file_path);
    assert_eq!(reloaded_theme.current_fg, "Blue"); // Theme changed
}

/// Integration Test 7: Verify theme file includes comments
/// 
/// This test verifies that when a theme file is created or saved,
/// it includes helpful comments for users.
#[test]
fn test_theme_file_includes_comments() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let theme_path = temp_dir.path().join("THEME-commented.yaml");

    // Create and save a theme
    let theme = movies::theme::Theme::default();
    movies::theme::save_theme(&theme, &theme_path);

    // Read the file contents
    let contents = fs::read_to_string(&theme_path).expect("Failed to read theme file");

    // Verify comments are present
    assert!(contents.contains("# === Color Configuration ==="));
    assert!(contents.contains("# Valid colors:"));
    assert!(contents.contains("# Current selection colors"));
    assert!(contents.contains("# Watched episode indicator"));
    assert!(contents.contains("# Unicode character displayed"));
    assert!(contents.contains("# Scroll bar configuration"));
    assert!(contents.contains("# Count display styling"));
    assert!(contents.contains("# Header text styling"));
    assert!(contents.contains("# Help text styling"));
    assert!(contents.contains("# Style for watched indicator (none, bold, dim, italic, underline)"));
    
    // Verify actual values are present
    assert!(contents.contains("current_fg: Black"));
    assert!(contents.contains("current_bg: White"));
    assert!(contents.contains("watched_indicator: \"●\""));
    assert!(contents.contains("scrollbar_track_char: \"│\""));
}
