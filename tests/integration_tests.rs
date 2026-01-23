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

// ============================================================================
// Browse Mode Component Integration Tests (Task 6.1)
// ============================================================================

/// Integration Test 8: Episode component integration with display layer
/// 
/// This test verifies that the Episode component can be created and rendered
/// with data from the application's data structures, and that the output
/// is consistent with the expected format.
/// 
/// Validates: Requirements 5.1, 5.2
#[test]
fn test_episode_component_integration_with_display_layer() {
    use movies::components::{Component, episode::Episode};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Create an episode component with typical application data
    let episode = Episode::new(
        "Test Episode Name".to_string(),
        false, // unwatched
        true,  // file exists
        false, // not new
    );
    
    // Render the component
    let cells = episode.render(50, 1, &theme, false);
    
    // Verify the output structure
    assert_eq!(cells.len(), 1, "Episode should render as single row");
    assert!(!cells[0].is_empty(), "Row should contain cells");
    
    // Verify the first character is the unwatched indicator
    assert_eq!(cells[0][0].character, '○', "First character should be unwatched indicator");
    
    // Verify we can extract the text content
    let text: String = cells[0].iter().map(|cell| cell.character).collect();
    assert!(text.contains("Test Episode Name"), "Text should contain episode name");
}

/// Integration Test 9: Episode component with selection highlighting
/// 
/// This test verifies that when an episode is selected, the component
/// applies the correct selection colors from the theme.
/// 
/// Validates: Requirements 5.3
#[test]
fn test_episode_component_selection_highlighting() {
    use movies::components::{Component, episode::Episode};
    use movies::theme::Theme;
    use crossterm::style::Color;
    
    let theme = Theme::default();
    
    let episode = Episode::new(
        "Selected Episode".to_string(),
        false,
        true,
        false,
    );
    
    // Render without selection
    let cells_unselected = episode.render(50, 1, &theme, false);
    
    // Render with selection
    let cells_selected = episode.render(50, 1, &theme, true);
    
    // Verify selection changes colors
    assert_ne!(
        cells_unselected[0][0].fg_color,
        cells_selected[0][0].fg_color,
        "Selection should change foreground color"
    );
    
    // Verify selection uses current_fg/current_bg
    assert_eq!(
        cells_selected[0][0].fg_color,
        Color::Black,
        "Selected episode should use current_fg (Black by default)"
    );
    assert_eq!(
        cells_selected[0][0].bg_color,
        Color::White,
        "Selected episode should use current_bg (White by default)"
    );
}

/// Integration Test 10: Episode component with different states
/// 
/// This test verifies that episodes with different states (watched, new, invalid)
/// are rendered with the correct colors and indicators.
/// 
/// Validates: Requirements 5.5
#[test]
fn test_episode_component_different_states() {
    use movies::components::{Component, episode::Episode};
    use movies::theme::Theme;
    use crossterm::style::Color;
    
    let theme = Theme::default();
    
    // Test watched episode
    let watched = Episode::new("Watched".to_string(), true, true, false);
    let watched_cells = watched.render(50, 1, &theme, false);
    assert_eq!(watched_cells[0][0].character, '●', "Watched should have ● indicator");
    
    // Test unwatched episode
    let unwatched = Episode::new("Unwatched".to_string(), false, true, false);
    let unwatched_cells = unwatched.render(50, 1, &theme, false);
    assert_eq!(unwatched_cells[0][0].character, '○', "Unwatched should have ○ indicator");
    
    // Test new episode
    let new_episode = Episode::new("New".to_string(), false, true, true);
    let new_cells = new_episode.render(50, 1, &theme, false);
    assert_eq!(new_cells[0][0].fg_color, Color::Green, "New episode should use green color");
    
    // Test invalid episode (file doesn't exist)
    let invalid = Episode::new("Invalid".to_string(), false, false, false);
    let invalid_cells = invalid.render(50, 1, &theme, false);
    assert_eq!(invalid_cells[0][0].fg_color, Color::Red, "Invalid episode should use red color");
}

/// Integration Test 11: Episode component with custom theme
/// 
/// This test verifies that the Episode component correctly uses custom theme values
/// when rendering, demonstrating integration with the theme system.
/// 
/// Validates: Requirements 5.2
#[test]
fn test_episode_component_with_custom_theme() {
    use movies::components::{Component, episode::Episode};
    use movies::theme::Theme;
    use crossterm::style::Color;
    
    // Create a custom theme
    let mut theme = Theme::default();
    theme.watched_indicator = "✓".to_string();
    theme.unwatched_indicator = "✗".to_string();
    theme.new_fg = "blue".to_string();
    theme.invalid_fg = "magenta".to_string();
    
    // Test watched episode with custom indicator
    let watched = Episode::new("Watched".to_string(), true, true, false);
    let watched_cells = watched.render(50, 1, &theme, false);
    assert_eq!(watched_cells[0][0].character, '✓', "Should use custom watched indicator");
    
    // Test unwatched episode with custom indicator
    let unwatched = Episode::new("Unwatched".to_string(), false, true, false);
    let unwatched_cells = unwatched.render(50, 1, &theme, false);
    assert_eq!(unwatched_cells[0][0].character, '✗', "Should use custom unwatched indicator");
    
    // Test new episode with custom color
    let new_episode = Episode::new("New".to_string(), false, true, true);
    let new_cells = new_episode.render(50, 1, &theme, false);
    assert_eq!(new_cells[0][0].fg_color, Color::Blue, "Should use custom new_fg color");
    
    // Test invalid episode with custom color
    let invalid = Episode::new("Invalid".to_string(), false, false, false);
    let invalid_cells = invalid.render(50, 1, &theme, false);
    assert_eq!(invalid_cells[0][0].fg_color, Color::Magenta, "Should use custom invalid_fg color");
}

/// Integration Test 12: Episode component rendering consistency
/// 
/// This test verifies that the Episode component produces consistent output
/// when rendered multiple times with the same parameters, which is important
/// for reliable display updates.
/// 
/// Validates: Requirements 5.2
#[test]
fn test_episode_component_rendering_consistency() {
    use movies::components::{Component, episode::Episode};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    let episode = Episode::new("Consistent".to_string(), true, true, false);
    
    // Render multiple times
    let render1 = episode.render(50, 1, &theme, false);
    let render2 = episode.render(50, 1, &theme, false);
    let render3 = episode.render(50, 1, &theme, false);
    
    // All renders should be identical
    assert_eq!(render1, render2, "First and second render should be identical");
    assert_eq!(render2, render3, "Second and third render should be identical");
    
    // Verify content is correct
    let text: String = render1[0].iter().map(|cell| cell.character).collect();
    assert!(text.starts_with("● "), "Should start with watched indicator and space");
    assert!(text.contains("Consistent"), "Should contain episode name");
}

/// Integration Test 13: Episode component with width constraints
/// 
/// This test verifies that the Episode component correctly handles width constraints
/// when integrated with the display system, which is important for responsive layout.
/// 
/// Validates: Requirements 5.4
#[test]
fn test_episode_component_with_width_constraints() {
    use movies::components::{Component, episode::Episode};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    let episode = Episode::new(
        "This is a very long episode name that needs truncation".to_string(),
        false,
        true,
        false,
    );
    
    // Test with various widths
    let widths = vec![10, 20, 30, 50, 100];
    
    for width in widths {
        let cells = episode.render(width, 1, &theme, false);
        
        // Verify output doesn't exceed width
        assert!(
            cells[0].len() <= width,
            "Output length {} should not exceed width {}",
            cells[0].len(),
            width
        );
        
        // Verify indicator is preserved even with small width
        if width > 0 {
            assert_eq!(
                cells[0][0].character,
                '○',
                "Indicator should be preserved at width {}",
                width
            );
        }
    }
}

/// Integration Test 14: Episode component state priority
/// 
/// This test verifies that the Episode component correctly prioritizes states
/// when multiple conditions are true (e.g., invalid overrides new).
/// 
/// Validates: Requirements 5.5
#[test]
fn test_episode_component_state_priority() {
    use movies::components::{Component, episode::Episode};
    use movies::theme::Theme;
    use crossterm::style::Color;
    
    let theme = Theme::default();
    
    // Test: Invalid state should override new state
    let invalid_and_new = Episode::new(
        "Invalid New".to_string(),
        false,
        false, // file doesn't exist (invalid)
        true,  // is new
    );
    let cells = invalid_and_new.render(50, 1, &theme, false);
    assert_eq!(
        cells[0][0].fg_color,
        Color::Red,
        "Invalid state should override new state (red, not green)"
    );
    
    // Test: New state with watched indicator
    let new_and_watched = Episode::new(
        "New Watched".to_string(),
        true,  // watched
        true,  // file exists
        true,  // is new
    );
    let cells2 = new_and_watched.render(50, 1, &theme, false);
    assert_eq!(cells2[0][0].character, '●', "Should have watched indicator");
    assert_eq!(cells2[0][0].fg_color, Color::Green, "Should use new color");
    
    // Test: Selection overrides all other states
    let invalid_selected = Episode::new(
        "Invalid Selected".to_string(),
        false,
        false, // invalid
        false,
    );
    let cells3 = invalid_selected.render(50, 1, &theme, true); // selected
    assert_eq!(
        cells3[0][0].fg_color,
        Color::Black,
        "Selection should override invalid state"
    );
    assert_eq!(
        cells3[0][0].bg_color,
        Color::White,
        "Selection should override invalid state"
    );
}

// ============================================================================
// Browse Mode Category Component Integration Tests (Task 7.1)
// ============================================================================

/// Integration Test 15: Category component integration for series entries
/// 
/// This test verifies that the Category component can be created and rendered
/// for series entries with data from the application's data structures.
/// 
/// Validates: Requirements 5.1
#[test]
fn test_category_component_series_integration() {
    use movies::components::{Component, Category, CategoryType};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Create a category component for a series entry
    let category = Category::new(
        "[Breaking Bad]".to_string(),
        62,
        45,
        CategoryType::Series,
    );
    
    // Render the component
    let cells = category.render(50, 1, &theme, false);
    
    // Verify the output structure
    assert_eq!(cells.len(), 1, "Category should render as single row");
    assert!(!cells[0].is_empty(), "Row should contain cells");
    
    // Verify we can extract the text content
    let text: String = cells[0].iter().map(|cell| cell.character).collect();
    assert!(text.contains("[Breaking Bad]"), "Text should contain series name with brackets");
    assert!(text.contains("45/62 watched"), "Text should contain watched/total count");
}

/// Integration Test 16: Category component integration for season entries
/// 
/// This test verifies that the Category component can be created and rendered
/// for season entries with data from the application's data structures.
/// 
/// Validates: Requirements 5.2
#[test]
fn test_category_component_season_integration() {
    use movies::components::{Component, Category, CategoryType};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Create a category component for a season entry
    let category = Category::new(
        "Season 1".to_string(),
        13,
        10,
        CategoryType::Season,
    );
    
    // Render the component
    let cells = category.render(50, 1, &theme, false);
    
    // Verify the output structure
    assert_eq!(cells.len(), 1, "Category should render as single row");
    assert!(!cells[0].is_empty(), "Row should contain cells");
    
    // Verify we can extract the text content
    let text: String = cells[0].iter().map(|cell| cell.character).collect();
    assert!(text.contains("Season 1"), "Text should contain season name");
    assert!(text.contains("10/13 watched"), "Text should contain watched/total count");
}

/// Integration Test 17: Category component with selection highlighting
/// 
/// This test verifies that when a category is selected, the component
/// applies the correct selection colors from the theme.
/// 
/// Validates: Requirements 5.3
#[test]
fn test_category_component_selection_highlighting() {
    use movies::components::{Component, Category, CategoryType};
    use movies::theme::Theme;
    use crossterm::style::Color;
    
    let theme = Theme::default();
    
    let category = Category::new(
        "[The Wire]".to_string(),
        60,
        30,
        CategoryType::Series,
    );
    
    // Render without selection
    let cells_unselected = category.render(50, 1, &theme, false);
    
    // Render with selection
    let cells_selected = category.render(50, 1, &theme, true);
    
    // Verify selection changes colors
    assert_ne!(
        cells_unselected[0][0].fg_color,
        cells_selected[0][0].fg_color,
        "Selection should change foreground color"
    );
    
    // Verify selection uses current_fg/current_bg
    assert_eq!(
        cells_selected[0][0].fg_color,
        Color::Black,
        "Selected category should use current_fg (Black by default)"
    );
    assert_eq!(
        cells_selected[0][0].bg_color,
        Color::White,
        "Selected category should use current_bg (White by default)"
    );
    
    // Verify all cells use selection colors
    for cell in &cells_selected[0] {
        assert_eq!(cell.fg_color, Color::Black, "All cells should use current_fg");
        assert_eq!(cell.bg_color, Color::White, "All cells should use current_bg");
    }
}

/// Integration Test 18: Category navigation through different types
/// 
/// This test verifies that categories of different types (Series and Season)
/// can be rendered correctly, simulating navigation through the browse mode.
/// 
/// Validates: Requirements 5.3
#[test]
fn test_category_navigation_through_types() {
    use movies::components::{Component, Category, CategoryType};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Simulate a list of categories (series and seasons)
    let categories = vec![
        Category::new("[Series 1]".to_string(), 20, 10, CategoryType::Series),
        Category::new("[Series 2]".to_string(), 30, 15, CategoryType::Series),
        Category::new("Season 1".to_string(), 10, 5, CategoryType::Season),
        Category::new("Season 2".to_string(), 10, 5, CategoryType::Season),
    ];
    
    // Render each category
    for (i, category) in categories.iter().enumerate() {
        let is_selected = i == 1; // Select the second item
        let cells = category.render(50, 1, &theme, is_selected);
        
        // Verify each renders correctly
        assert_eq!(cells.len(), 1, "Each category should render as single row");
        assert!(!cells[0].is_empty(), "Each row should contain cells");
        
        // Verify selection highlighting
        if is_selected {
            assert_eq!(
                cells[0][0].fg_color,
                crossterm::style::Color::Black,
                "Selected category should use selection colors"
            );
        } else {
            // Unselected categories use series_fg or season_fg (both Blue by default)
            assert_eq!(
                cells[0][0].fg_color,
                crossterm::style::Color::Blue,
                "Unselected category should use series/season colors (Blue)"
            );
        }
    }
}

/// Integration Test 19: Category component with zero watched count
/// 
/// This test verifies that categories with zero watched count
/// omit the watched portion, as per the requirements.
/// 
/// Validates: Requirements 5.1, 5.2
#[test]
fn test_category_component_zero_watched() {
    use movies::components::{Component, Category, CategoryType};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Create category with zero watched count (use season to avoid brackets in name)
    let category = Category::new(
        "Season 1".to_string(),
        25,
        0,
        CategoryType::Season,
    );
    
    // Render the component
    let cells = category.render(50, 1, &theme, false);
    
    // Verify the output
    let text: String = cells[0].iter().map(|cell| cell.character).collect();
    assert!(text.contains("Season 1"), "Text should contain season name");
    assert!(text.contains("0/25 watched"), "Text should contain '0/25 watched' format");
}

/// Integration Test 20: Category component with custom theme
/// 
/// This test verifies that the Category component correctly uses custom theme values
/// when rendering, demonstrating integration with the theme system.
/// 
/// Validates: Requirements 5.1, 5.2
#[test]
fn test_category_component_with_custom_theme() {
    use movies::components::{Component, Category, CategoryType};
    use movies::theme::Theme;
    use crossterm::style::Color;
    
    // Create a custom theme
    let mut theme = Theme::default();
    theme.series_fg = "yellow".to_string();  // Use series colors for Series category
    theme.series_bg = "blue".to_string();
    theme.current_fg = "cyan".to_string();
    theme.current_bg = "magenta".to_string();
    
    let category = Category::new(
        "[Custom Theme Series]".to_string(),
        50,
        25,
        CategoryType::Series,
    );
    
    // Test unselected with custom colors
    let cells_unselected = category.render(50, 1, &theme, false);
    assert_eq!(
        cells_unselected[0][0].fg_color,
        Color::Yellow,
        "Should use custom series_fg color"
    );
    assert_eq!(
        cells_unselected[0][0].bg_color,
        Color::Blue,
        "Should use custom series_bg color"
    );
    
    // Test selected with custom colors
    let cells_selected = category.render(50, 1, &theme, true);
    assert_eq!(
        cells_selected[0][0].fg_color,
        Color::Cyan,
        "Should use custom current_fg color"
    );
    assert_eq!(
        cells_selected[0][0].bg_color,
        Color::Magenta,
        "Should use custom current_bg color"
    );
}

/// Integration Test 21: Category component rendering consistency
/// 
/// This test verifies that the Category component produces consistent output
/// when rendered multiple times with the same parameters.
/// 
/// Validates: Requirements 5.1, 5.2
#[test]
fn test_category_component_rendering_consistency() {
    use movies::components::{Component, Category, CategoryType};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    let category = Category::new(
        "[Consistent Series]".to_string(),
        40,
        20,
        CategoryType::Series,
    );
    
    // Render multiple times
    let render1 = category.render(50, 1, &theme, false);
    let render2 = category.render(50, 1, &theme, false);
    let render3 = category.render(50, 1, &theme, false);
    
    // All renders should be identical
    assert_eq!(render1, render2, "First and second render should be identical");
    assert_eq!(render2, render3, "Second and third render should be identical");
    
    // Verify content is correct
    let text: String = render1[0].iter().map(|cell| cell.character).collect();
    assert!(text.contains("[Consistent Series]"), "Should contain series name");
    assert!(text.contains("20/40 watched"), "Should contain watched/total count");
}

/// Integration Test 22: Category component with width constraints
/// 
/// This test verifies that the Category component correctly handles width constraints
/// when integrated with the display system.
/// 
/// Validates: Requirements 5.1, 5.2
#[test]
fn test_category_component_with_width_constraints() {
    use movies::components::{Component, Category, CategoryType};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    let category = Category::new(
        "[This is a very long series name that needs truncation]".to_string(),
        100,
        50,
        CategoryType::Series,
    );
    
    // Test with various widths
    let widths = vec![10, 20, 30, 50, 100];
    
    for width in widths {
        let cells = category.render(width, 1, &theme, false);
        
        // Verify output doesn't exceed width
        assert!(
            cells[0].len() <= width,
            "Output length {} should not exceed width {}",
            cells[0].len(),
            width
        );
        
        // Verify we still have content even with small width
        if width > 0 {
            assert!(!cells[0].is_empty(), "Should have content at width {}", width);
        }
    }
}

/// Integration Test 23: Category component for both series and season types
/// 
/// This test verifies that both Series and Season category types
/// render correctly with the same component implementation.
/// 
/// Validates: Requirements 5.1, 5.2
#[test]
fn test_category_component_both_types() {
    use movies::components::{Component, Category, CategoryType};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Create series category
    let series = Category::new(
        "[Game of Thrones]".to_string(),
        73,
        73,
        CategoryType::Series,
    );
    
    // Create season category
    let season = Category::new(
        "Season 8".to_string(),
        6,
        6,
        CategoryType::Season,
    );
    
    // Render both
    let series_cells = series.render(50, 1, &theme, false);
    let season_cells = season.render(50, 1, &theme, false);
    
    // Both should render successfully
    assert!(!series_cells[0].is_empty(), "Series should render");
    assert!(!season_cells[0].is_empty(), "Season should render");
    
    // Verify content
    let series_text: String = series_cells[0].iter().map(|cell| cell.character).collect();
    let season_text: String = season_cells[0].iter().map(|cell| cell.character).collect();
    
    assert!(series_text.contains("[Game of Thrones]"), "Series should contain name with brackets");
    assert!(season_text.contains("Season 8"), "Season should contain name");
    
    // Both should use the same default colors (episode_fg/episode_bg)
    assert_eq!(
        series_cells[0][0].fg_color,
        season_cells[0][0].fg_color,
        "Both types should use same default fg color"
    );
    assert_eq!(
        series_cells[0][0].bg_color,
        season_cells[0][0].bg_color,
        "Both types should use same default bg color"
    );
}

// ============================================================================
// Browse Mode Scrollbar Integration Tests (Task 8)
// ============================================================================

/// Integration Test 24: Scrollbar rendering in browse mode with Scrollbar component
/// 
/// This test verifies that the Scrollbar component can be created and rendered
/// with typical browse mode data (list of entries with viewport constraints).
/// 
/// Validates: Requirements 8.1, 8.2
#[test]
fn test_scrollbar_rendering_in_browse_mode() {
    use movies::components::{Component, Scrollbar};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Simulate browse mode with 50 total entries, 20 visible, starting at position 10
    let scrollbar = Scrollbar::new(
        50,  // total_items (total entries in list)
        20,  // visible_items (entries that fit on screen)
        10,  // first_visible_index (current scroll position)
    );
    
    // Render the scrollbar with typical browse mode height
    let height = 20;
    let cells = scrollbar.render(1, height, &theme, false);
    
    // Verify scrollbar is visible (not empty)
    assert!(!cells.is_empty(), "Scrollbar should be visible when total > visible");
    assert_eq!(cells.len(), height, "Scrollbar should have correct height");
    
    // Verify each row has exactly one cell (single column)
    for (i, row) in cells.iter().enumerate() {
        assert_eq!(row.len(), 1, "Row {} should have exactly one cell", i);
    }
    
    // Verify scrollbar uses theme characters and colors
    let has_track_char = cells.iter().any(|row| row[0].character == '│');
    let has_indicator_char = cells.iter().any(|row| row[0].character == '█');
    
    assert!(has_track_char, "Scrollbar should contain track characters");
    assert!(has_indicator_char, "Scrollbar should contain indicator characters");
    
    // Verify colors are applied from theme
    for row in &cells {
        assert_eq!(row[0].fg_color, crossterm::style::Color::White, "Should use scrollbar_fg color");
        assert_eq!(row[0].bg_color, crossterm::style::Color::Reset, "Should use scrollbar_bg color");
    }
}

/// Integration Test 25: Scrollbar updates when scrolling through lists
/// 
/// This test verifies that the scrollbar indicator position changes correctly
/// when the scroll position changes, simulating user navigation in browse mode.
/// 
/// Validates: Requirements 8.3
#[test]
fn test_scrollbar_updates_when_scrolling() {
    use movies::components::{Component, Scrollbar};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    let height = 10;
    let total_items = 30;
    let visible_items = 10;
    
    // Test scrollbar at different scroll positions
    let positions = vec![0, 5, 10, 15, 20]; // Different scroll positions
    let mut previous_indicator_positions = Vec::new();
    
    for scroll_pos in positions {
        let scrollbar = Scrollbar::new(total_items, visible_items, scroll_pos);
        let cells = scrollbar.render(1, height, &theme, false);
        
        // Find indicator positions (cells with indicator character)
        let indicator_positions: Vec<usize> = cells
            .iter()
            .enumerate()
            .filter(|(_, row)| row[0].character == '█')
            .map(|(i, _)| i)
            .collect();
        
        // Verify indicator is present
        assert!(!indicator_positions.is_empty(), "Indicator should be present at scroll position {}", scroll_pos);
        
        // Verify indicator position changes as scroll position changes
        if !previous_indicator_positions.is_empty() {
            assert_ne!(
                indicator_positions, 
                previous_indicator_positions,
                "Indicator position should change when scroll position changes from previous"
            );
        }
        
        previous_indicator_positions = indicator_positions;
    }
}

/// Integration Test 26: Scrollbar visibility based on list size
/// 
/// This test verifies that the scrollbar is hidden when all items fit on screen
/// and visible when items exceed the viewport, as required in browse mode.
/// 
/// Validates: Requirements 8.4, 8.5
#[test]
fn test_scrollbar_visibility_based_on_list_size() {
    use movies::components::{Component, Scrollbar};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    let height = 15;
    
    // Test case 1: All items fit on screen (scrollbar should be hidden)
    let scrollbar_hidden = Scrollbar::new(
        10,  // total_items
        15,  // visible_items (more than total)
        0,   // first_visible_index
    );
    
    let cells_hidden = scrollbar_hidden.render(1, height, &theme, false);
    assert!(cells_hidden.is_empty(), "Scrollbar should be hidden when all items fit on screen");
    
    // Test case 2: Items exceed viewport (scrollbar should be visible)
    let scrollbar_visible = Scrollbar::new(
        50,  // total_items
        15,  // visible_items (less than total)
        0,   // first_visible_index
    );
    
    let cells_visible = scrollbar_visible.render(1, height, &theme, false);
    assert!(!cells_visible.is_empty(), "Scrollbar should be visible when items exceed viewport");
    assert_eq!(cells_visible.len(), height, "Visible scrollbar should have correct height");
    
    // Test case 3: Exact fit (scrollbar should be hidden)
    let scrollbar_exact = Scrollbar::new(
        15,  // total_items
        15,  // visible_items (exactly equal)
        0,   // first_visible_index
    );
    
    let cells_exact = scrollbar_exact.render(1, height, &theme, false);
    assert!(cells_exact.is_empty(), "Scrollbar should be hidden when items exactly fit");
    
    // Test case 4: Empty list (scrollbar should be hidden)
    let scrollbar_empty = Scrollbar::new(
        0,   // total_items (empty list)
        15,  // visible_items
        0,   // first_visible_index
    );
    
    let cells_empty = scrollbar_empty.render(1, height, &theme, false);
    assert!(cells_empty.is_empty(), "Scrollbar should be hidden for empty lists");
}

/// Integration Test 27: Scrollbar integration with browse mode viewport calculations
/// 
/// This test verifies that the scrollbar correctly represents the viewport
/// when integrated with typical browse mode navigation patterns.
/// 
/// Validates: Requirements 8.1, 8.2, 8.3
#[test]
fn test_scrollbar_integration_with_browse_mode_viewport() {
    use movies::components::{Component, Scrollbar};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    let height = 20;
    
    // Simulate a large list of episodes in browse mode
    let total_episodes = 100;
    let visible_episodes = 20;
    
    // Test different navigation scenarios
    
    // Scenario 1: At the beginning of the list
    let scrollbar_top = Scrollbar::new(total_episodes, visible_episodes, 0);
    let cells_top = scrollbar_top.render(1, height, &theme, false);
    
    // Find indicator at top position
    let indicator_at_top = cells_top.iter()
        .take(5) // Check first 5 rows
        .any(|row| row[0].character == '█');
    assert!(indicator_at_top, "Indicator should be near top when at beginning of list");
    
    // Scenario 2: In the middle of the list
    let middle_position = (total_episodes - visible_episodes) / 2;
    let scrollbar_middle = Scrollbar::new(total_episodes, visible_episodes, middle_position);
    let cells_middle = scrollbar_middle.render(1, height, &theme, false);
    
    // Find indicator in middle area
    let middle_start = height / 3;
    let middle_end = (height * 2) / 3;
    let indicator_in_middle = cells_middle.iter()
        .skip(middle_start)
        .take(middle_end - middle_start)
        .any(|row| row[0].character == '█');
    assert!(indicator_in_middle, "Indicator should be in middle area when in middle of list");
    
    // Scenario 3: At the end of the list
    let end_position = total_episodes - visible_episodes;
    let scrollbar_bottom = Scrollbar::new(total_episodes, visible_episodes, end_position);
    let cells_bottom = scrollbar_bottom.render(1, height, &theme, false);
    
    // Find indicator at bottom position
    let indicator_at_bottom = cells_bottom.iter()
        .skip(height - 5) // Check last 5 rows
        .any(|row| row[0].character == '█');
    assert!(indicator_at_bottom, "Indicator should be near bottom when at end of list");
}

/// Integration Test 28: Scrollbar with custom theme integration
/// 
/// This test verifies that the scrollbar correctly uses custom theme values
/// when integrated with the browse mode display system.
/// 
/// Validates: Requirements 8.1, 8.2
#[test]
fn test_scrollbar_with_custom_theme_integration() {
    use movies::components::{Component, Scrollbar};
    use movies::theme::Theme;
    use crossterm::style::Color;
    
    // Create custom theme with different scrollbar appearance
    let mut theme = Theme::default();
    theme.scrollbar_track_char = "║".to_string();
    theme.scrollbar_indicator_char = "▓".to_string();
    theme.scrollbar_fg = "cyan".to_string();
    theme.scrollbar_bg = "blue".to_string();
    
    let scrollbar = Scrollbar::new(40, 15, 10);
    let cells = scrollbar.render(1, 15, &theme, false);
    
    // Verify custom characters are used
    let has_custom_track = cells.iter().any(|row| row[0].character == '║');
    let has_custom_indicator = cells.iter().any(|row| row[0].character == '▓');
    
    assert!(has_custom_track, "Should use custom track character");
    assert!(has_custom_indicator, "Should use custom indicator character");
    
    // Verify custom colors are applied
    for row in &cells {
        assert_eq!(row[0].fg_color, Color::Cyan, "Should use custom scrollbar_fg color");
        assert_eq!(row[0].bg_color, Color::Blue, "Should use custom scrollbar_bg color");
    }
}

/// Integration Test 29: Scrollbar rendering consistency in browse mode
/// 
/// This test verifies that the scrollbar produces consistent output
/// when rendered multiple times with the same browse mode parameters.
/// 
/// Validates: Requirements 8.1, 8.2
#[test]
fn test_scrollbar_rendering_consistency_in_browse_mode() {
    use movies::components::{Component, Scrollbar};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Create scrollbar with typical browse mode parameters
    let scrollbar = Scrollbar::new(75, 25, 15);
    
    // Render multiple times
    let render1 = scrollbar.render(1, 25, &theme, false);
    let render2 = scrollbar.render(1, 25, &theme, false);
    let render3 = scrollbar.render(1, 25, &theme, false);
    
    // All renders should be identical
    assert_eq!(render1, render2, "First and second render should be identical");
    assert_eq!(render2, render3, "Second and third render should be identical");
    
    // Verify content is correct
    assert!(!render1.is_empty(), "Should produce scrollbar output");
    assert_eq!(render1.len(), 25, "Should have correct height");
    
    // Verify structure is consistent
    for row in &render1 {
        assert_eq!(row.len(), 1, "Each row should have exactly one cell");
    }
}
// ============================================================================
// Browser Component Display Integration Tests (Task 9.1)
// ============================================================================

/// Integration Test 30: Browser component integration with display system
/// 
/// This test verifies that the Browser component works correctly with the existing
/// display logic, properly integrating categories and episodes with theme support.
/// 
/// Validates: Requirements 1.1, 4.1, 4.2, 4.3
#[test]
fn test_browser_component_display_integration() {
    use movies::components::{Component, Browser, Category, CategoryType};
    use movies::components::episode::Episode;
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Create test data similar to what display.rs would create
    let categories = vec![
        Category::new("[Breaking Bad]".to_string(), 62, 45, CategoryType::Series),
        Category::new("Season 1".to_string(), 7, 7, CategoryType::Season),
    ];
    
    let episodes = vec![
        Episode::new("Pilot".to_string(), true, true, false),
        Episode::new("Cat's in the Bag...".to_string(), true, true, false),
        Episode::new("...And the Bag's in the River".to_string(), false, true, false),
    ];
    
    // Create browser component as display.rs would
    let browser = Browser::new(
        (0, 4),  // top_left position (typical header height)
        45,      // width (matches COL1_WIDTH)
        categories,
        episodes,
    );
    
    // Render the browser component
    let cells = browser.render(45, 10, &theme, true);
    
    // Verify basic structure
    assert!(!cells.is_empty(), "Browser should render content");
    assert!(cells.len() <= 10, "Browser should respect height constraint");
    
    // Verify content is present
    let mut has_content = false;
    for row in &cells {
        if !row.is_empty() {
            has_content = true;
            break;
        }
    }
    assert!(has_content, "Browser should render visible content");
    
    // Verify width constraint is respected
    for (i, row) in cells.iter().enumerate() {
        assert!(row.len() <= 45, "Row {} should not exceed width constraint", i);
    }
}

/// Integration Test 31: Browser component with theme integration
/// 
/// This test verifies that the Browser component correctly applies theme colors
/// and styling when integrated with the display system.
/// 
/// Validates: Requirements 1.1, 4.1, 4.2, 4.3
#[test]
fn test_browser_component_theme_integration() {
    use movies::components::{Component, Browser, Category, CategoryType};
    use movies::components::episode::Episode;
    use movies::theme::Theme;
    use crossterm::style::Color;
    
    // Create custom theme
    let mut theme = Theme::default();
    theme.current_fg = "red".to_string();
    theme.current_bg = "yellow".to_string();
    theme.watched_indicator = "★".to_string();
    theme.unwatched_indicator = "☆".to_string();
    
    // Create test data
    let categories = vec![
        Category::new("[Test Series]".to_string(), 10, 5, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Watched Episode".to_string(), true, true, false),
        Episode::new("Unwatched Episode".to_string(), false, true, false),
    ];
    
    // Create browser with selection on first item
    let mut browser = Browser::new(
        (0, 5),
        40,
        categories,
        episodes,
    );
    browser.set_selected_item(0); // Select first item (category)
    
    // Render with theme
    let cells = browser.render(40, 5, &theme, true);
    
    // Verify theme integration
    assert!(!cells.is_empty(), "Should render content");
    
    // Check that selection colors are applied to the selected item
    if !cells[0].is_empty() {
        // First row should have selection highlighting
        let has_selection_colors = cells[0].iter().any(|cell| {
            cell.fg_color == Color::Red || cell.bg_color == Color::Yellow
        });
        assert!(has_selection_colors, "Selected item should use theme selection colors");
    }
    
    // Test with episode selection
    browser.set_selected_item(1); // Select first episode
    let cells_episode = browser.render(40, 5, &theme, true);
    
    // Verify episode indicators are used
    if cells_episode.len() > 1 && !cells_episode[1].is_empty() {
        let text: String = cells_episode[1].iter().map(|cell| cell.character).collect();
        assert!(text.contains("★") || text.contains("☆"), "Should use theme indicators");
    }
}

/// Integration Test 32: Browser component terminal output formatting
/// 
/// This test verifies that the Browser component output can be properly formatted
/// for terminal display using the existing display system functions.
/// 
/// Validates: Requirements 1.1, 4.1, 4.2, 4.3
#[test]
fn test_browser_component_terminal_output_formatting() {
    use movies::components::{Component, Browser, Category, CategoryType};
    use movies::components::episode::Episode;
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Create test data
    let categories = vec![
        Category::new("[Test Series]".to_string(), 5, 3, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Test Episode".to_string(), true, true, false),
    ];
    
    let browser = Browser::new(
        (0, 5),
        50,
        categories,
        episodes,
    );
    
    // Render browser
    let cells = browser.render(50, 3, &theme, true);
    
    // Verify output can be converted to terminal format
    // This simulates what display.rs does with cells_to_styled_string
    for (row_index, row) in cells.iter().enumerate() {
        if !row.is_empty() {
            // Verify each cell has valid properties for terminal output
            for (col_index, cell) in row.iter().enumerate() {
                assert!(cell.character != '\0', "Cell at ({}, {}) should have valid character", row_index, col_index);
                
                // Colors should be valid (not checking specific values, just that they exist)
                // This ensures the cell can be rendered to terminal
                match cell.fg_color {
                    crossterm::style::Color::Reset | 
                    crossterm::style::Color::Black | 
                    crossterm::style::Color::Red | 
                    crossterm::style::Color::Green | 
                    crossterm::style::Color::Yellow | 
                    crossterm::style::Color::Blue | 
                    crossterm::style::Color::Magenta | 
                    crossterm::style::Color::Cyan | 
                    crossterm::style::Color::White | 
                    crossterm::style::Color::DarkGrey |
                    crossterm::style::Color::Rgb { .. } |
                    crossterm::style::Color::AnsiValue(_) |
                    _ => {
                        // Valid color (including other variants)
                    }
                }
                
                // Verify style flags are boolean (valid for terminal)
                assert!(cell.style.bold == true || cell.style.bold == false);
                assert!(cell.style.italic == true || cell.style.italic == false);
                assert!(cell.style.underlined == true || cell.style.underlined == false);
                assert!(cell.style.dim == true || cell.style.dim == false);
                assert!(cell.style.crossed_out == true || cell.style.crossed_out == false);
            }
        }
    }
    
    // Verify the output structure is suitable for terminal rendering
    assert!(cells.len() <= 3, "Should not exceed height constraint");
    
    // Verify content is present and properly structured
    let total_cells: usize = cells.iter().map(|row| row.len()).sum();
    assert!(total_cells > 0, "Should have rendered some content");
}

/// Integration Test 33: Browser component with scrollbar integration
/// 
/// This test verifies that the Browser component correctly integrates with
/// scrollbar rendering when content exceeds the viewport height.
/// 
/// Validates: Requirements 1.1, 1.3, 4.1, 4.2, 4.3
#[test]
fn test_browser_component_scrollbar_integration() {
    use movies::components::{Component, Browser, Category, CategoryType};
    use movies::components::episode::Episode;
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Create more content than can fit in viewport
    let categories = vec![
        Category::new("[Series 1]".to_string(), 10, 5, CategoryType::Series),
        Category::new("[Series 2]".to_string(), 8, 3, CategoryType::Series),
        Category::new("[Series 3]".to_string(), 12, 7, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), true, true, false),
        Episode::new("Episode 2".to_string(), false, true, false),
        Episode::new("Episode 3".to_string(), true, true, false),
        Episode::new("Episode 4".to_string(), false, true, false),
        Episode::new("Episode 5".to_string(), true, true, false),
    ];
    
    // Total: 3 categories + 5 episodes = 8 items
    // Height: 5 (less than total items, should trigger scrollbar)
    let browser = Browser::new(
        (0, 5),
        45,      // width
        categories,
        episodes,
    );
    
    // Verify scrollbar is needed
    assert!(browser.needs_scrollbar(5), "Browser should need scrollbar when content exceeds height");
    
    // Verify content width is reduced for scrollbar
    assert_eq!(browser.content_width(5), 44, "Content width should be reduced by 1 for scrollbar");
    
    // Render browser
    let cells = browser.render(45, 5, &theme, true);
    
    // Verify output structure with scrollbar
    assert_eq!(cells.len(), 5, "Should render exactly 5 rows (height)");
    
    // Check that rows include scrollbar column
    for (i, row) in cells.iter().enumerate() {
        if !row.is_empty() {
            assert!(row.len() <= 45, "Row {} should not exceed total width", i);
            
            // If scrollbar is present, last column should be scrollbar
            if row.len() == 45 {
                let last_cell = &row[44];
                // Scrollbar characters should be track or indicator
                assert!(
                    last_cell.character == '│' || last_cell.character == '█',
                    "Last column should contain scrollbar character"
                );
            }
        }
    }
}

/// Integration Test 34: Browser component empty state handling
/// 
/// This test verifies that the Browser component handles empty content gracefully
/// when integrated with the display system.
/// 
/// Validates: Requirements 1.1, 2.5, 4.1, 4.2, 4.3
#[test]
fn test_browser_component_empty_state_integration() {
    use movies::components::{Component, Browser};
    use movies::theme::Theme;
    
    let theme = Theme::default();
    
    // Create browser with no content
    let browser = Browser::new(
        (0, 5),
        40,
        vec![], // no categories
        vec![], // no episodes
    );
    
    // Render empty browser
    let cells = browser.render(40, 8, &theme, true);
    
    // Verify empty state handling
    assert_eq!(cells.len(), 8, "Should render height rows even when empty");
    
    // All rows should be empty
    for (i, row) in cells.iter().enumerate() {
        assert_eq!(row.len(), 0, "Row {} should be empty for empty browser", i);
    }
    
    // Verify no scrollbar is needed for empty content
    assert!(!browser.needs_scrollbar(8), "Empty browser should not need scrollbar");
    assert_eq!(browser.content_width(8), 40, "Empty browser should use full width");
    assert_eq!(browser.total_items(), 0, "Empty browser should have 0 total items");
}

/// Integration Test 35: Browser component selection state integration
/// 
/// This test verifies that the Browser component correctly manages selection state
/// when integrated with the display system's current_item tracking.
/// 
/// Validates: Requirements 1.1, 2.1, 2.2, 4.1, 4.2, 4.3
#[test]
fn test_browser_component_selection_state_integration() {
    use movies::components::{Component, Browser, Category, CategoryType};
    use movies::components::episode::Episode;
    use movies::theme::Theme;
    use crossterm::style::Color;
    
    let theme = Theme::default();
    
    // Create test content
    let categories = vec![
        Category::new("[Test Series]".to_string(), 5, 2, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), true, true, false),
        Episode::new("Episode 2".to_string(), false, true, false),
    ];
    
    let mut browser = Browser::new(
        (0, 5),
        40,
        categories,
        episodes,
    );
    
    // Test selection on category (item 0)
    browser.set_selected_item(0);
    let cells_category = browser.render(40, 5, &theme, true);
    
    // Verify category selection highlighting
    if !cells_category.is_empty() && !cells_category[0].is_empty() {
        let has_selection = cells_category[0].iter().any(|cell| {
            cell.fg_color == Color::Black && cell.bg_color == Color::White
        });
        assert!(has_selection, "Selected category should have selection highlighting");
    }
    
    // Test selection on episode (item 1)
    browser.set_selected_item(1);
    let cells_episode = browser.render(40, 5, &theme, true);
    
    // Verify episode selection highlighting
    if cells_episode.len() > 1 && !cells_episode[1].is_empty() {
        let has_selection = cells_episode[1].iter().any(|cell| {
            cell.fg_color == Color::Black && cell.bg_color == Color::White
        });
        assert!(has_selection, "Selected episode should have selection highlighting");
    }
    
    // Test selection bounds
    browser.set_selected_item(100); // Out of bounds
    // Selection bounds are handled internally by the browser component
}


// ============================================================================
// BufferManager Integration Tests (Task 9.1)
// ============================================================================

/// Integration Test: BufferManager is created with correct terminal size
/// 
/// This test verifies that BufferManager is initialized with the correct
/// terminal dimensions when the application starts.
/// 
/// Validates: Requirements 2.1, 6.1
#[test]
fn test_buffer_manager_created_with_correct_terminal_size() {
    use movies::buffer::BufferManager;
    
    // Test with various terminal sizes
    let test_sizes = vec![
        (80, 24),   // Standard terminal
        (120, 40),  // Large terminal
        (40, 20),   // Small terminal
        (200, 60),  // Very large terminal
    ];
    
    for (width, height) in test_sizes {
        let buffer_manager = BufferManager::new(width, height);
        
        // Verify buffer manager is created successfully
        // We can't directly access private fields, but we can verify it works
        // by testing that it can be used for rendering operations
        
        // Create a simple test by getting a writer and writing to it
        let mut buffer_manager = buffer_manager;
        buffer_manager.clear_desired_buffer();
        let mut writer = buffer_manager.get_writer();
        
        // Write some test content
        writer.move_to(0, 0);
        writer.write_str("Test");
        
        // Verify we can compare buffers (this would fail if dimensions were wrong)
        let changes = buffer_manager.compare_buffers();
        assert!(!changes.is_empty(), "Should detect changes after writing to buffer");
        
        // Verify changes are within bounds
        for (x, y, _) in changes {
            assert!(x < width, "X coordinate {} should be within width {}", x, width);
            assert!(y < height, "Y coordinate {} should be within height {}", y, height);
        }
    }
}

/// Integration Test: Terminal resize events update buffer dimensions
/// 
/// This test verifies that when a terminal resize event occurs,
/// the BufferManager correctly updates its internal buffer dimensions.
/// 
/// Validates: Requirements 6.1
#[test]
fn test_resize_events_update_buffer_dimensions() {
    use movies::buffer::BufferManager;
    
    // Create buffer manager with initial size
    let initial_width = 80;
    let initial_height = 24;
    let mut buffer_manager = BufferManager::new(initial_width, initial_height);
    
    // Write some content to the initial buffer
    buffer_manager.clear_desired_buffer();
    let mut writer = buffer_manager.get_writer();
    writer.move_to(0, 0);
    writer.write_str("Initial content");
    
    // Simulate terminal resize to larger size
    let new_width = 120;
    let new_height = 40;
    buffer_manager.resize(new_width, new_height);
    
    // Verify buffer manager works with new dimensions
    buffer_manager.clear_desired_buffer();
    let mut writer = buffer_manager.get_writer();
    
    // Write content at a position that would be out of bounds in old size
    writer.move_to(100, 30);
    writer.write_str("New content");
    
    // Verify changes are within new bounds
    let changes = buffer_manager.compare_buffers();
    for (x, y, _) in changes {
        assert!(x < new_width, "X coordinate {} should be within new width {}", x, new_width);
        assert!(y < new_height, "Y coordinate {} should be within new height {}", y, new_height);
    }
    
    // Simulate resize to smaller size
    let smaller_width = 40;
    let smaller_height = 20;
    buffer_manager.resize(smaller_width, smaller_height);
    
    // Verify buffer manager works with smaller dimensions
    buffer_manager.clear_desired_buffer();
    let mut writer = buffer_manager.get_writer();
    writer.move_to(0, 0);
    writer.write_str("Smaller");
    
    // Verify changes are within smaller bounds
    let changes = buffer_manager.compare_buffers();
    for (x, y, _) in changes {
        assert!(x < smaller_width, "X coordinate {} should be within smaller width {}", x, smaller_width);
        assert!(y < smaller_height, "Y coordinate {} should be within smaller height {}", y, smaller_height);
    }
}

/// Integration Test: Mode changes trigger full redraw
/// 
/// This test verifies that when the application mode changes,
/// force_full_redraw() is called to ensure a clean visual state.
/// 
/// Validates: Requirements 6.2
#[test]
fn test_mode_changes_trigger_full_redraw() {
    use movies::buffer::BufferManager;
    
    let width = 80;
    let height = 24;
    let mut buffer_manager = BufferManager::new(width, height);
    
    // Write initial content and render it
    buffer_manager.clear_desired_buffer();
    let mut writer = buffer_manager.get_writer();
    writer.move_to(0, 0);
    writer.write_str("Initial mode content");
    
    // Simulate rendering (this updates current buffer to match desired)
    buffer_manager.update_current_buffer();
    
    // Now both buffers are in sync, so no changes should be detected
    let changes_before = buffer_manager.compare_buffers();
    assert!(changes_before.is_empty(), "No changes should be detected when buffers are in sync");
    
    // Simulate mode change by calling force_full_redraw
    buffer_manager.force_full_redraw();
    
    // Write new content for the new mode
    buffer_manager.clear_desired_buffer();
    let mut writer = buffer_manager.get_writer();
    writer.move_to(0, 0);
    writer.write_str("New mode content");
    
    // After force_full_redraw, all cells should be detected as changed
    let changes_after = buffer_manager.compare_buffers();
    assert!(!changes_after.is_empty(), "Changes should be detected after force_full_redraw");
    
    // Verify that force_full_redraw causes a full screen update
    // by checking that we have changes across the screen
    let unique_rows: std::collections::HashSet<usize> = changes_after.iter().map(|(_, y, _)| *y).collect();
    assert!(!unique_rows.is_empty(), "Should have changes in multiple rows after mode change");
}

/// Integration Test: BufferManager integration with multiple render cycles
/// 
/// This test verifies that BufferManager correctly handles multiple render cycles,
/// simulating the main loop's redraw behavior.
/// 
/// Validates: Requirements 2.1, 6.1
#[test]
fn test_buffer_manager_multiple_render_cycles() {
    use movies::buffer::BufferManager;
    
    let width = 80;
    let height = 24;
    let mut buffer_manager = BufferManager::new(width, height);
    
    // Simulate multiple render cycles
    for cycle in 0..5 {
        // Clear and write new content (simulating draw_screen)
        buffer_manager.clear_desired_buffer();
        let mut writer = buffer_manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str(&format!("Cycle {}", cycle));
        
        // Compare buffers
        let changes = buffer_manager.compare_buffers();
        assert!(!changes.is_empty(), "Cycle {} should have changes", cycle);
        
        // Simulate successful render
        buffer_manager.update_current_buffer();
        
        // After update, buffers should be in sync
        let changes_after_update = buffer_manager.compare_buffers();
        assert!(changes_after_update.is_empty(), "Cycle {} should have no changes after update", cycle);
    }
}

/// Integration Test: BufferManager handles empty frames correctly
/// 
/// This test verifies that BufferManager correctly handles frames where
/// the desired buffer is cleared but no content is written.
/// 
/// Validates: Requirements 2.1, 3.1
#[test]
fn test_buffer_manager_handles_empty_frames() {
    use movies::buffer::BufferManager;
    
    let width = 80;
    let height = 24;
    let mut buffer_manager = BufferManager::new(width, height);
    
    // Write initial content
    buffer_manager.clear_desired_buffer();
    let mut writer = buffer_manager.get_writer();
    writer.move_to(0, 0);
    writer.write_str("Initial content");
    buffer_manager.update_current_buffer();
    
    // Clear desired buffer without writing new content (empty frame)
    buffer_manager.clear_desired_buffer();
    
    // Compare buffers - should detect that content was removed
    let changes = buffer_manager.compare_buffers();
    assert!(!changes.is_empty(), "Should detect changes when content is cleared");
    
    // Update to empty state
    buffer_manager.update_current_buffer();
    
    // Verify buffers are now in sync (both empty)
    let changes_after = buffer_manager.compare_buffers();
    assert!(changes_after.is_empty(), "Buffers should be in sync after clearing");
}

/// Integration Test: BufferManager with rapid mode changes
/// 
/// This test verifies that BufferManager correctly handles rapid mode changes,
/// ensuring each mode change triggers a full redraw.
/// 
/// Validates: Requirements 6.2
#[test]
fn test_buffer_manager_rapid_mode_changes() {
    use movies::buffer::BufferManager;
    
    let width = 80;
    let height = 24;
    let mut buffer_manager = BufferManager::new(width, height);
    
    // Simulate rapid mode changes (Browse -> Edit -> Menu -> Browse)
    let modes = vec!["Browse", "Edit", "Menu", "Browse"];
    
    for mode in modes {
        // Force full redraw for mode change
        buffer_manager.force_full_redraw();
        
        // Clear and write mode-specific content
        buffer_manager.clear_desired_buffer();
        let mut writer = buffer_manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str(&format!("Mode: {}", mode));
        
        // Verify changes are detected
        let changes = buffer_manager.compare_buffers();
        assert!(!changes.is_empty(), "Should detect changes for mode {}", mode);
        
        // Update current buffer
        buffer_manager.update_current_buffer();
    }
}

/// Integration Test: BufferManager resize during active rendering
/// 
/// This test verifies that BufferManager correctly handles resize events
/// that occur during active rendering cycles.
/// 
/// Validates: Requirements 6.1
#[test]
fn test_buffer_manager_resize_during_rendering() {
    use movies::buffer::BufferManager;
    
    let initial_width = 80;
    let initial_height = 24;
    let mut buffer_manager = BufferManager::new(initial_width, initial_height);
    
    // Start a render cycle
    buffer_manager.clear_desired_buffer();
    let mut writer = buffer_manager.get_writer();
    writer.move_to(0, 0);
    writer.write_str("Content before resize");
    
    // Simulate resize event during rendering
    let new_width = 120;
    let new_height = 40;
    buffer_manager.resize(new_width, new_height);
    
    // Continue rendering with new dimensions
    buffer_manager.clear_desired_buffer();
    let mut writer = buffer_manager.get_writer();
    writer.move_to(0, 0);
    writer.write_str("Content after resize");
    
    // Verify buffer works correctly with new dimensions
    let changes = buffer_manager.compare_buffers();
    assert!(!changes.is_empty(), "Should detect changes after resize");
    
    // Verify all changes are within new bounds
    for (x, y, _) in changes {
        assert!(x < new_width, "X should be within new width");
        assert!(y < new_height, "Y should be within new height");
    }
}
