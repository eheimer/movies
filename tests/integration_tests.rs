use std::fs;
use tempfile::TempDir;

// Import necessary modules from the main crate
// Note: These tests require the application to be structured as a library
// with public modules exposed in lib.rs

// Note: Due to the global database connection using OnceLock, we can only
// initialize the database once per test run. Tests that require database
// operations must be run sequentially or use a single shared database.

/// Integration Test 1: Config reload with custom colors
/// 
/// This test verifies that when a config file is created with custom color values,
/// those values are correctly loaded and applied throughout the application.
/// 
/// Test flow:
/// 1. Create a temporary directory
/// 2. Write a config file with custom colors
/// 3. Load the config
/// 4. Verify all custom colors are loaded correctly
/// 5. Modify the config file with different colors
/// 6. Reload the config
/// 7. Verify the new colors are applied
#[test]
fn test_config_reload_with_custom_colors() {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("config.yaml");

    // Create initial config with custom colors
    let initial_config = r#"current_fg: Red
current_bg: Blue
watched_indicator: "★"
watched_fg: Cyan
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
video_extensions:
  - mp4
video_player: /usr/bin/vlc
"#;
    fs::write(&config_path, initial_config).expect("Failed to write initial config");

    // Load the config
    let config = movies::config::read_config(&config_path);

    // Verify initial custom values
    assert_eq!(config.current_fg, "Red");
    assert_eq!(config.current_bg, "Blue");
    assert_eq!(config.watched_indicator, "★");
    assert_eq!(config.watched_fg, "Cyan");
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

    // Modify the config file with different colors
    let modified_config = r#"current_fg: Green
current_bg: Red
watched_indicator: "●"
watched_fg: Yellow
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
video_extensions:
  - mkv
video_player: /usr/bin/mpv
"#;
    fs::write(&config_path, modified_config).expect("Failed to write modified config");

    // Reload the config
    let reloaded_config = movies::config::read_config(&config_path);

    // Verify modified values are loaded
    assert_eq!(reloaded_config.current_fg, "Green");
    assert_eq!(reloaded_config.current_bg, "Red");
    assert_eq!(reloaded_config.watched_indicator, "●");
    assert_eq!(reloaded_config.watched_fg, "Yellow");
    assert_eq!(reloaded_config.new_fg, "Cyan");
    assert_eq!(reloaded_config.new_bg, "Magenta");
    assert_eq!(reloaded_config.invalid_fg, "White");
    assert_eq!(reloaded_config.invalid_bg, "Black");
    assert_eq!(reloaded_config.series_fg, "Magenta");
    assert_eq!(reloaded_config.series_bg, "Yellow");
    assert_eq!(reloaded_config.season_fg, "Cyan");
    assert_eq!(reloaded_config.season_bg, "Green");
    assert_eq!(reloaded_config.episode_fg, "White");
    assert_eq!(reloaded_config.episode_bg, "Red");
    assert_eq!(reloaded_config.status_fg, "Yellow");
    assert_eq!(reloaded_config.status_bg, "Blue");
    assert_eq!(reloaded_config.video_extensions, vec!["mkv"]);
    assert_eq!(reloaded_config.video_player, "/usr/bin/mpv");
}
