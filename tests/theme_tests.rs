use movies::theme::*;
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
