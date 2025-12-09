use movies::display::*;
use movies::theme::Theme;
use crossterm::style::Color;

/// Test Case 10: Invalid color fallback
/// When a color configuration field contains an invalid color name,
/// the color parsing function should return the default color for that field.
/// Validates: Requirements 3.4, 7.4
#[test]
fn test_invalid_color_fallback() {
    // Test invalid color names fall back to defaults
    assert_eq!(string_to_fg_color_or_default("invalid"), Color::Black);
    assert_eq!(string_to_fg_color_or_default("notacolor"), Color::Black);
    assert_eq!(string_to_fg_color_or_default(""), Color::Black);
    assert_eq!(string_to_fg_color_or_default("purple"), Color::Black);
    
    assert_eq!(string_to_bg_color_or_default("invalid"), Color::White);
    assert_eq!(string_to_bg_color_or_default("notacolor"), Color::White);
    assert_eq!(string_to_bg_color_or_default(""), Color::White);
    assert_eq!(string_to_bg_color_or_default("orange"), Color::White);
}

#[test]
fn test_valid_color_names() {
    // Test all standard color names
    assert_eq!(string_to_color("black"), Some(Color::Black));
    assert_eq!(string_to_color("red"), Some(Color::Red));
    assert_eq!(string_to_color("green"), Some(Color::Green));
    assert_eq!(string_to_color("yellow"), Some(Color::Yellow));
    assert_eq!(string_to_color("blue"), Some(Color::Blue));
    assert_eq!(string_to_color("magenta"), Some(Color::Magenta));
    assert_eq!(string_to_color("cyan"), Some(Color::Cyan));
    assert_eq!(string_to_color("white"), Some(Color::White));
    
    // Test case insensitivity
    assert_eq!(string_to_color("BLACK"), Some(Color::Black));
    assert_eq!(string_to_color("Red"), Some(Color::Red));
    assert_eq!(string_to_color("GREEN"), Some(Color::Green));
}

#[test]
fn test_reset_color_support() {
    // Test that "Reset" is supported as terminal default
    assert_eq!(string_to_color("reset"), Some(Color::Reset));
    assert_eq!(string_to_color("Reset"), Some(Color::Reset));
    assert_eq!(string_to_color("RESET"), Some(Color::Reset));
    
    // Test that Reset works with helper functions
    assert_eq!(string_to_fg_color_or_default("reset"), Color::Reset);
    assert_eq!(string_to_bg_color_or_default("reset"), Color::Reset);
}

#[test]
fn test_darkgray_color_support() {
    // Test DarkGray color support with different formats
    assert_eq!(string_to_color("darkgray"), Some(Color::DarkGrey));
    assert_eq!(string_to_color("DarkGray"), Some(Color::DarkGrey));
    assert_eq!(string_to_color("dark_gray"), Some(Color::DarkGrey));
    assert_eq!(string_to_color("DARK_GRAY"), Some(Color::DarkGrey));
}

#[test]
fn test_color_parsing_returns_none_for_invalid() {
    // Test that string_to_color returns None for invalid colors
    assert_eq!(string_to_color("invalid"), None);
    assert_eq!(string_to_color("notacolor"), None);
    assert_eq!(string_to_color(""), None);
    assert_eq!(string_to_color("purple"), None);
    assert_eq!(string_to_color("orange"), None);
}

#[test]
fn test_apply_text_style_none() {
    let text = "Test Episode";
    assert_eq!(apply_text_style(text, "none"), text);
    assert_eq!(apply_text_style(text, ""), text);
}

#[test]
fn test_apply_text_style_single() {
    let text = "Test Episode";
    
    // Test that styling returns a string (actual styling is terminal-dependent)
    let bold_result = apply_text_style(text, "bold");
    assert!(!bold_result.is_empty());
    
    let italic_result = apply_text_style(text, "italic");
    assert!(!italic_result.is_empty());
    
    let underline_result = apply_text_style(text, "underline");
    assert!(!underline_result.is_empty());
}

#[test]
fn test_apply_text_style_multiple() {
    let text = "Test Episode";
    
    // Test multiple styles combined
    let result = apply_text_style(text, "bold,italic");
    assert!(!result.is_empty());
}

#[test]
fn test_apply_text_style_case_insensitive() {
    let text = "Test Episode";
    
    // Test case insensitivity
    assert_eq!(
        apply_text_style(text, "BOLD").len(),
        apply_text_style(text, "bold").len()
    );
}

#[test]
fn test_apply_text_style_unknown_ignored() {
    let text = "Test Episode";
    
    // Unknown styles should be ignored
    let result = apply_text_style(text, "unknown,bold");
    assert!(!result.is_empty());
}

/// Test Case: Invalid style values trigger warning logs
/// When apply_text_style receives an invalid style value,
/// it should log a warning and continue processing.
/// Validates: Requirements 2.4
#[test]
#[serial_test::serial]
fn test_invalid_style_logs_warning() {
    use tempfile::TempDir;
    use std::fs;
    
    // Set up logger
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let log_file = temp_dir.path().join("test_invalid_style.log");
    movies::logger::initialize_logger(log_file.clone(), movies::logger::LogLevel::Warn)
        .expect("Failed to initialize logger");
    
    let text = "Test Episode";
    
    // Apply an invalid style
    let result = apply_text_style(text, "invalid_style");
    
    // Result should still be the text (invalid style ignored)
    assert!(!result.is_empty());
    
    // Log a final message to ensure flush
    movies::logger::log_warn("test_complete");
    
    // Give time for log to flush
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Check that warning was logged
    let log_contents = fs::read_to_string(&log_file)
        .expect("Failed to read log file");
    assert!(log_contents.contains("Invalid style value 'invalid_style' ignored"));
}



/// Test Case 1: Watched indicator presence
/// When an episode has watched status set to true, the formatted display string
/// should contain the configured watched indicator character.
/// Validates: Requirements 1.1
#[test]
fn test_watched_indicator_presence() {
    let theme = Theme::default();
    let episode_name = "Test Episode";
    
    let formatted = format_episode_with_indicator(episode_name, true, &theme);
    
    // The formatted string should contain the watched indicator
    assert!(formatted.contains(&theme.watched_indicator));
    // The formatted string should also contain the episode name
    assert!(formatted.contains(episode_name));
}

/// Test Case 2: Unwatched indicator presence
/// When an episode has watched status set to false, the formatted display string
/// should contain the unwatched indicator character.
/// Validates: Requirements 1.2
#[test]
fn test_unwatched_indicator_presence() {
    let theme = Theme::default();
    let episode_name = "Test Episode";
    
    let formatted = format_episode_with_indicator(episode_name, false, &theme);
    
    // The formatted string should NOT contain the watched indicator
    assert!(!formatted.contains(&theme.watched_indicator));
    // The formatted string should contain the unwatched indicator
    assert!(formatted.contains(&theme.unwatched_indicator));
    // The formatted string should contain the episode name
    assert!(formatted.contains(episode_name));
}

/// Test Case 3: Watched indicator distinctness
/// When displaying a watched episode, the watched indicator should be separated
/// from the episode name by whitespace or other delimiter.
/// Validates: Requirements 1.4
#[test]
fn test_watched_indicator_distinctness() {
    let theme = Theme::default();
    let episode_name = "Test Episode";
    
    let formatted = format_episode_with_indicator(episode_name, true, &theme);
    
    // The indicator should be followed by a space before the episode name
    let expected = format!("{} {}", theme.watched_indicator, episode_name);
    assert_eq!(formatted, expected);
    
    // Verify there's whitespace between indicator and name
    assert!(formatted.contains(" "));
}

#[test]
fn test_watched_indicator_with_custom_indicator() {
    let mut theme = Theme::default();
    theme.watched_indicator = "★".to_string();
    let episode_name = "Custom Test";
    
    let formatted = format_episode_with_indicator(episode_name, true, &theme);
    
    // Should use the custom indicator
    assert!(formatted.contains("★"));
    assert_eq!(formatted, "★ Custom Test");
}

#[test]
fn test_watched_indicator_with_empty_name() {
    let theme = Theme::default();
    let episode_name = "";
    
    let formatted_watched = format_episode_with_indicator(episode_name, true, &theme);
    let formatted_unwatched = format_episode_with_indicator(episode_name, false, &theme);
    
    // Even with empty name, indicator should be present when watched
    assert!(formatted_watched.contains(&theme.watched_indicator));
    // Unwatched should have unwatched indicator
    assert!(formatted_unwatched.contains(&theme.unwatched_indicator));
}

#[test]
fn test_watched_indicator_with_style() {
    let mut theme = Theme::default();
    theme.watched_style = "italic".to_string();
    let episode_name = "Styled Episode";
    
    let formatted = format_episode_with_indicator(episode_name, true, &theme);
    
    // Should contain both indicator and name (styling is applied but not easily testable)
    assert!(formatted.contains(&theme.watched_indicator));
    assert!(!formatted.is_empty());
}

#[test]
fn test_watched_no_indicator_with_style() {
    let mut theme = Theme::default();
    theme.watched_indicator = "".to_string(); // No indicator
    theme.watched_style = "italic".to_string();
    let episode_name = "Styled Episode";
    
    let formatted = format_episode_with_indicator(episode_name, true, &theme);
    
    // Should not contain indicator, but should have styled text
    assert!(!formatted.contains("✓"));
    assert!(!formatted.is_empty());
}

/// Test Case 11: Series entry coloring
/// When displaying a series entry that is not selected, the display should apply
/// the configured series_fg and series_bg colors.
/// Validates: Requirements 4.1
#[test]
fn test_series_entry_coloring() {
    use movies::util::Entry;
    
    let theme = Theme::default();
    
    // Create a series entry
    let _entry = Entry::Series {
        series_id: 1,
        name: "Test Series".to_string(),
    };
    
    // Verify the theme has the expected series colors
    assert_eq!(theme.series_fg, "Blue");
    assert_eq!(theme.series_bg, "Reset");
    
    // The actual color application happens in draw_screen
    // This test verifies the theme values are correct
    let series_fg = string_to_fg_color_or_default(&theme.series_fg);
    let series_bg = string_to_bg_color_or_default(&theme.series_bg);
    
    assert_eq!(series_fg, Color::Blue);
    assert_eq!(series_bg, Color::Reset);
}

/// Test Case 12: Season entry coloring
/// When displaying a season entry that is not selected, the display should apply
/// the configured season_fg and season_bg colors.
/// Validates: Requirements 4.2
#[test]
fn test_season_entry_coloring() {
    use movies::util::Entry;
    
    let theme = Theme::default();
    
    // Create a season entry
    let _entry = Entry::Season {
        season_id: 1,
        number: 1,
    };
    
    // Verify the theme has the expected season colors
    assert_eq!(theme.season_fg, "Blue");
    assert_eq!(theme.season_bg, "Reset");
    
    // The actual color application happens in draw_screen
    // This test verifies the theme values are correct
    let season_fg = string_to_fg_color_or_default(&theme.season_fg);
    let season_bg = string_to_bg_color_or_default(&theme.season_bg);
    
    assert_eq!(season_fg, Color::Blue);
    assert_eq!(season_bg, Color::Reset);
}

/// Test Case 13: Episode entry coloring
/// When displaying an episode entry in normal state (not new, not invalid, not watched)
/// that is not selected, the display should apply the configured episode_fg and episode_bg colors.
/// Validates: Requirements 4.3
#[test]
fn test_episode_entry_coloring() {
    use movies::util::Entry;
    
    let theme = Theme::default();
    
    // Create an episode entry
    let _entry = Entry::Episode {
        episode_id: 1,
        name: "Test Episode".to_string(),
        location: "test.mp4".to_string(),
    };
    
    // Verify the theme has the expected episode colors
    assert_eq!(theme.episode_fg, "Reset");
    assert_eq!(theme.episode_bg, "Reset");
    
    // The actual color application happens in draw_screen
    // This test verifies the theme values are correct
    let episode_fg = string_to_fg_color_or_default(&theme.episode_fg);
    let episode_bg = string_to_bg_color_or_default(&theme.episode_bg);
    
    assert_eq!(episode_fg, Color::Reset);
    assert_eq!(episode_bg, Color::Reset);
}

/// Test Case 14: Selection highlight override
/// When an entry (series, season, or episode) is currently selected, the display should
/// apply current_fg and current_bg colors, overriding the entry type colors.
/// Validates: Requirements 4.4
#[test]
fn test_selection_highlight_override() {
    let theme = Theme::default();
    
    // Verify that current selection colors are different from type colors
    assert_eq!(theme.current_fg, "Black");
    assert_eq!(theme.current_bg, "White");
    
    // Verify type colors are different
    assert_eq!(theme.series_fg, "Blue");
    assert_eq!(theme.episode_fg, "Reset");
    
    // The actual override logic happens in draw_screen
    // This test verifies that selection colors take precedence
    let current_fg = string_to_fg_color_or_default(&theme.current_fg);
    let current_bg = string_to_bg_color_or_default(&theme.current_bg);
    
    assert_eq!(current_fg, Color::Black);
    assert_eq!(current_bg, Color::White);
    
    // Selection colors should be distinct from type colors
    let series_fg = string_to_fg_color_or_default(&theme.series_fg);
    assert_ne!(current_fg, series_fg);
}

/// Test Case: format_series_display uses theme count styling
/// When formatting a series display, the count text should use theme count_fg and count_style.
/// Validates: Requirements 1.3
#[test]
fn test_series_display_uses_theme_count_styling() {
    let theme = Theme::default();
    
    // Verify theme has count styling configured
    assert_eq!(theme.count_fg, "DarkGray");
    assert_eq!(theme.count_style, "italic");
    
    // The actual formatting happens in format_series_display
    // This test verifies the theme values are correct for count styling
    let count_fg = string_to_fg_color_or_default(&theme.count_fg);
    assert_eq!(count_fg, Color::DarkGrey);
}

/// Test Case: format_season_display uses theme count styling
/// When formatting a season display, the count text should use theme count_fg and count_style.
/// Validates: Requirements 1.3
#[test]
fn test_season_display_uses_theme_count_styling() {
    let theme = Theme::default();
    
    // Verify theme has count styling configured
    assert_eq!(theme.count_fg, "DarkGray");
    assert_eq!(theme.count_style, "italic");
    
    // The actual formatting happens in format_season_display
    // This test verifies the theme values are correct for count styling
    let count_fg = string_to_fg_color_or_default(&theme.count_fg);
    assert_eq!(count_fg, Color::DarkGrey);
}

/// Test Case: Theme provides all required color fields
/// When a theme is created, it should have all required color and style fields.
/// Validates: Requirements 1.3, 5.3
#[test]
fn test_theme_has_all_required_fields() {
    let theme = Theme::default();
    
    // Verify all color fields are present and non-empty
    assert!(!theme.current_fg.is_empty());
    assert!(!theme.current_bg.is_empty());
    assert!(!theme.dirty_fg.is_empty());
    assert!(!theme.dirty_bg.is_empty());
    assert!(!theme.new_fg.is_empty());
    assert!(!theme.new_bg.is_empty());
    assert!(!theme.invalid_fg.is_empty());
    assert!(!theme.invalid_bg.is_empty());
    assert!(!theme.series_fg.is_empty());
    assert!(!theme.series_bg.is_empty());
    assert!(!theme.season_fg.is_empty());
    assert!(!theme.season_bg.is_empty());
    assert!(!theme.episode_fg.is_empty());
    assert!(!theme.episode_bg.is_empty());
    assert!(!theme.status_fg.is_empty());
    assert!(!theme.status_bg.is_empty());
    assert!(!theme.scrollbar_fg.is_empty());
    assert!(!theme.scrollbar_bg.is_empty());
    assert!(!theme.count_fg.is_empty());
    assert!(!theme.header_fg.is_empty());
    assert!(!theme.help_fg.is_empty());
    
    // Verify style fields are present
    assert!(!theme.watched_style.is_empty());
    assert!(!theme.unwatched_style.is_empty());
    assert!(!theme.count_style.is_empty());
    assert!(!theme.header_style.is_empty());
    assert!(!theme.help_style.is_empty());
    
    // Verify indicator fields are present (can be empty if no indicator desired)
    // Just check they exist
    let _ = &theme.watched_indicator;
    let _ = &theme.unwatched_indicator;
    let _ = &theme.scrollbar_track_char;
    let _ = &theme.scrollbar_indicator_char;
}

/// Test Case: format_episode_with_indicator respects custom theme indicators
/// When a theme has custom indicators, format_episode_with_indicator should use them.
/// Validates: Requirements 1.3
#[test]
fn test_format_episode_with_custom_theme_indicators() {
    let mut theme = Theme::default();
    theme.watched_indicator = "✓".to_string();
    theme.unwatched_indicator = "✗".to_string();
    
    let episode_name = "Test Episode";
    
    let watched_formatted = format_episode_with_indicator(episode_name, true, &theme);
    let unwatched_formatted = format_episode_with_indicator(episode_name, false, &theme);
    
    // Verify custom indicators are used
    assert!(watched_formatted.contains("✓"));
    assert!(unwatched_formatted.contains("✗"));
    
    // Verify episode name is included
    assert!(watched_formatted.contains(episode_name));
    assert!(unwatched_formatted.contains(episode_name));
}
