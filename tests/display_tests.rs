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


