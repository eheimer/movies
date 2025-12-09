use movies::components::{Cell, TextStyle};
use crossterm::style::Color;

/// Test Case 1: Cell creation with various character and style combinations
/// When a Cell is created with specific character and styling, it should store
/// all the provided values correctly.
/// Validates: Requirements 1.1, 1.2, 1.3, 1.4
#[test]
fn test_cell_creation_with_character_and_style() {
    let style = TextStyle {
        bold: true,
        italic: false,
        underlined: false,
        dim: false,
        crossed_out: false,
    };
    
    let cell = Cell::new('A', Color::Red, Color::Blue, style);
    
    assert_eq!(cell.character, 'A');
    assert_eq!(cell.fg_color, Color::Red);
    assert_eq!(cell.bg_color, Color::Blue);
    assert_eq!(cell.style.bold, true);
    assert_eq!(cell.style.italic, false);
}

/// Test Case 2: Cell creation with Unicode character
/// When a Cell is created with a Unicode character, it should store
/// the character correctly.
/// Validates: Requirements 1.1
#[test]
fn test_cell_creation_with_unicode() {
    let style = TextStyle::new();
    let cell = Cell::new('●', Color::White, Color::Black, style);
    
    assert_eq!(cell.character, '●');
}

/// Test Case 3: TextStyle combinations
/// When TextStyle is created with various combinations of attributes,
/// it should store all the boolean flags correctly.
/// Validates: Requirements 1.4
#[test]
fn test_text_style_combinations() {
    // Test all flags true
    let style_all = TextStyle {
        bold: true,
        italic: true,
        underlined: true,
        dim: true,
        crossed_out: true,
    };
    
    assert_eq!(style_all.bold, true);
    assert_eq!(style_all.italic, true);
    assert_eq!(style_all.underlined, true);
    assert_eq!(style_all.dim, true);
    assert_eq!(style_all.crossed_out, true);
    
    // Test all flags false
    let style_none = TextStyle::new();
    
    assert_eq!(style_none.bold, false);
    assert_eq!(style_none.italic, false);
    assert_eq!(style_none.underlined, false);
    assert_eq!(style_none.dim, false);
    assert_eq!(style_none.crossed_out, false);
    
    // Test mixed flags
    let style_mixed = TextStyle {
        bold: true,
        italic: false,
        underlined: true,
        dim: false,
        crossed_out: true,
    };
    
    assert_eq!(style_mixed.bold, true);
    assert_eq!(style_mixed.italic, false);
    assert_eq!(style_mixed.underlined, true);
    assert_eq!(style_mixed.dim, false);
    assert_eq!(style_mixed.crossed_out, true);
}

/// Test Case 4: TextStyle default
/// When TextStyle::default() is called, all flags should be false.
/// Validates: Requirements 1.4
#[test]
fn test_text_style_default() {
    let style = TextStyle::default();
    
    assert_eq!(style.bold, false);
    assert_eq!(style.italic, false);
    assert_eq!(style.underlined, false);
    assert_eq!(style.dim, false);
    assert_eq!(style.crossed_out, false);
}

/// Test Case 5: Cell with different color combinations
/// When Cells are created with various color combinations,
/// they should store the colors correctly.
/// Validates: Requirements 1.2, 1.3
#[test]
fn test_cell_with_different_colors() {
    let style = TextStyle::new();
    
    // Test with basic colors
    let cell1 = Cell::new('X', Color::Red, Color::Blue, style);
    assert_eq!(cell1.fg_color, Color::Red);
    assert_eq!(cell1.bg_color, Color::Blue);
    
    // Test with RGB colors
    let cell2 = Cell::new('Y', Color::Rgb { r: 255, g: 0, b: 0 }, Color::Rgb { r: 0, g: 255, b: 0 }, style);
    assert_eq!(cell2.fg_color, Color::Rgb { r: 255, g: 0, b: 0 });
    assert_eq!(cell2.bg_color, Color::Rgb { r: 0, g: 255, b: 0 });
    
    // Test with Reset color
    let cell3 = Cell::new('Z', Color::Reset, Color::Reset, style);
    assert_eq!(cell3.fg_color, Color::Reset);
    assert_eq!(cell3.bg_color, Color::Reset);
}

/// Test Case 6: Cell to_styled_content conversion
/// When a Cell is converted to StyledContent, it should produce
/// the correct styling attributes.
/// Validates: Requirements 1.5
#[test]
fn test_cell_to_styled_content_basic() {
    let style = TextStyle::new();
    let cell = Cell::new('A', Color::Red, Color::Blue, style);
    
    let styled = cell.to_styled_content();
    
    // Verify the character is correct
    assert_eq!(*styled.content(), 'A');
    
    // Verify colors are applied
    assert_eq!(styled.style().foreground_color, Some(Color::Red));
    assert_eq!(styled.style().background_color, Some(Color::Blue));
}

/// Test Case 7: Cell to_styled_content with bold style
/// When a Cell with bold style is converted to StyledContent,
/// it should include the bold attribute.
/// Validates: Requirements 1.5
#[test]
fn test_cell_to_styled_content_with_bold() {
    use crossterm::style::Attribute;
    
    let style = TextStyle {
        bold: true,
        italic: false,
        underlined: false,
        dim: false,
        crossed_out: false,
    };
    let cell = Cell::new('B', Color::White, Color::Black, style);
    
    let styled = cell.to_styled_content();
    
    assert_eq!(*styled.content(), 'B');
    assert!(styled.style().attributes.has(Attribute::Bold));
}

/// Test Case 8: Cell to_styled_content with multiple styles
/// When a Cell with multiple style attributes is converted to StyledContent,
/// it should include all the specified attributes.
/// Validates: Requirements 1.5
#[test]
fn test_cell_to_styled_content_with_multiple_styles() {
    use crossterm::style::Attribute;
    
    let style = TextStyle {
        bold: true,
        italic: true,
        underlined: true,
        dim: false,
        crossed_out: false,
    };
    let cell = Cell::new('C', Color::Green, Color::Yellow, style);
    
    let styled = cell.to_styled_content();
    
    assert_eq!(*styled.content(), 'C');
    assert!(styled.style().attributes.has(Attribute::Bold));
    assert!(styled.style().attributes.has(Attribute::Italic));
    assert!(styled.style().attributes.has(Attribute::Underlined));
    assert!(!styled.style().attributes.has(Attribute::Dim));
    assert!(!styled.style().attributes.has(Attribute::CrossedOut));
}

/// Test Case 9: Cell to_styled_content with all styles
/// When a Cell with all style attributes enabled is converted to StyledContent,
/// it should include all attributes.
/// Validates: Requirements 1.5
#[test]
fn test_cell_to_styled_content_with_all_styles() {
    use crossterm::style::Attribute;
    
    let style = TextStyle {
        bold: true,
        italic: true,
        underlined: true,
        dim: true,
        crossed_out: true,
    };
    let cell = Cell::new('D', Color::Cyan, Color::Magenta, style);
    
    let styled = cell.to_styled_content();
    
    assert_eq!(*styled.content(), 'D');
    assert!(styled.style().attributes.has(Attribute::Bold));
    assert!(styled.style().attributes.has(Attribute::Italic));
    assert!(styled.style().attributes.has(Attribute::Underlined));
    assert!(styled.style().attributes.has(Attribute::Dim));
    assert!(styled.style().attributes.has(Attribute::CrossedOut));
}

/// Test Case 10: Cell to_styled_content with Unicode character
/// When a Cell with a Unicode character is converted to StyledContent,
/// it should preserve the character correctly.
/// Validates: Requirements 1.5
#[test]
fn test_cell_to_styled_content_with_unicode() {
    let style = TextStyle::new();
    let cell = Cell::new('●', Color::White, Color::Black, style);
    
    let styled = cell.to_styled_content();
    
    assert_eq!(*styled.content(), '●');
}

/// Test Case 11: Cell to_styled_content with RGB colors
/// When a Cell with RGB colors is converted to StyledContent,
/// it should preserve the RGB values correctly.
/// Validates: Requirements 1.5
#[test]
fn test_cell_to_styled_content_with_rgb_colors() {
    let style = TextStyle::new();
    let cell = Cell::new('E', Color::Rgb { r: 128, g: 64, b: 32 }, Color::Rgb { r: 200, g: 150, b: 100 }, style);
    
    let styled = cell.to_styled_content();
    
    assert_eq!(*styled.content(), 'E');
    assert_eq!(styled.style().foreground_color, Some(Color::Rgb { r: 128, g: 64, b: 32 }));
    assert_eq!(styled.style().background_color, Some(Color::Rgb { r: 200, g: 150, b: 100 }));
}

// ============================================================================
// Episode Component Tests
// ============================================================================

use movies::components::{Component, episode::Episode};
use movies::theme::Theme;

/// Test Case 12: Episode component creation
/// When an Episode component is created with specific fields,
/// it should store all the provided values correctly.
/// Validates: Requirements 3.1, 3.2, 3.3, 3.4
#[test]
fn test_episode_creation() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        true,
        true,
        false,
    );
    
    assert_eq!(episode.name, "Test Episode");
    assert_eq!(episode.is_watched, true);
    assert_eq!(episode.file_exists, true);
    assert_eq!(episode.is_new, false);
}

/// Test Case 13: Episode component implements Component trait
/// When an Episode component calls render, it should return a 2D Cell array.
/// Validates: Requirements 2.1, 2.2
#[test]
fn test_episode_implements_component_trait() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        false,
        true,
        false,
    );
    
    let theme = Theme::default();
    let result = episode.render(50, &theme, false);
    
    // Should return a 2D array (Vec<Vec<Cell>>)
    assert!(!result.is_empty(), "Result should not be empty");
    
    // Should be a single row for Episode component
    assert_eq!(result.len(), 1, "Episode should render as single row");
    
    // Should have cells in the row
    assert!(!result[0].is_empty(), "Row should contain cells");
}

/// Test Case 14: Episode component respects width constraint
/// When an Episode component renders with a width constraint,
/// the output should not exceed that width.
/// Validates: Requirements 2.6
#[test]
fn test_episode_respects_width_constraint() {
    let episode = Episode::new(
        "This is a very long episode name that should be truncated".to_string(),
        false,
        true,
        false,
    );
    
    let theme = Theme::default();
    let width = 20;
    let result = episode.render(width, &theme, false);
    
    // Check that the output doesn't exceed the width
    assert!(result[0].len() <= width, "Output should not exceed width constraint");
}

/// Test Case 15: Episode component handles zero width
/// When an Episode component renders with zero width,
/// it should return an empty cell array without panicking.
/// Validates: Requirements 2.6
#[test]
fn test_episode_handles_zero_width() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        false,
        true,
        false,
    );
    
    let theme = Theme::default();
    let result = episode.render(0, &theme, false);
    
    // Should return a single row with no cells
    assert_eq!(result.len(), 1, "Should return single row");
    assert_eq!(result[0].len(), 0, "Row should be empty for zero width");
}

// ============================================================================
// Episode Rendering States Tests (Task 5.1)
// ============================================================================

/// Test Case 16: Episode rendering with watched indicator
/// When an Episode component renders with is_watched=true,
/// the output should contain the watched indicator character from the theme.
/// Validates: Requirements 3.6
#[test]
fn test_episode_rendering_with_watched_indicator() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        true,  // is_watched
        true,  // file_exists
        false, // is_new
    );
    
    let theme = Theme::default();
    let result = episode.render(50, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // First character should be the watched indicator
    assert_eq!(result[0][0].character, '●', "First character should be watched indicator");
}

/// Test Case 17: Episode rendering with unwatched indicator
/// When an Episode component renders with is_watched=false,
/// the output should contain the unwatched indicator character from the theme.
/// Validates: Requirements 3.7
#[test]
fn test_episode_rendering_with_unwatched_indicator() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        false, // is_watched
        true,  // file_exists
        false, // is_new
    );
    
    let theme = Theme::default();
    let result = episode.render(50, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // First character should be the unwatched indicator
    assert_eq!(result[0][0].character, '○', "First character should be unwatched indicator");
}

/// Test Case 18: Episode rendering with new state colors
/// When an Episode component renders with is_new=true,
/// the output cells should use the new_fg and new_bg colors from the theme.
/// Validates: Requirements 3.8
#[test]
fn test_episode_rendering_with_new_state() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        false, // is_watched
        true,  // file_exists
        true,  // is_new
    );
    
    let theme = Theme::default();
    let result = episode.render(50, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // All cells should use new colors (green foreground by default)
    for cell in &result[0] {
        assert_eq!(cell.fg_color, Color::Green, "Should use new_fg color");
        assert_eq!(cell.bg_color, Color::Reset, "Should use new_bg color");
    }
}

/// Test Case 19: Episode rendering with invalid state colors
/// When an Episode component renders with file_exists=false,
/// the output cells should use the invalid_fg and invalid_bg colors from the theme.
/// Validates: Requirements 3.9
#[test]
fn test_episode_rendering_with_invalid_state() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        false, // is_watched
        false, // file_exists (invalid)
        false, // is_new
    );
    
    let theme = Theme::default();
    let result = episode.render(50, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // All cells should use invalid colors (red foreground by default)
    for cell in &result[0] {
        assert_eq!(cell.fg_color, Color::Red, "Should use invalid_fg color");
        assert_eq!(cell.bg_color, Color::Reset, "Should use invalid_bg color");
    }
}

/// Test Case 20: Episode rendering with selection override
/// When an Episode component renders with is_selected=true,
/// the output cells should use current_fg and current_bg colors from the theme,
/// overriding state-based colors.
/// Validates: Requirements 3.11
#[test]
fn test_episode_rendering_with_selection_override() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        false, // is_watched
        true,  // file_exists
        false, // is_new
    );
    
    let theme = Theme::default();
    let result = episode.render(50, &theme, true); // is_selected=true
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // All cells should use selection colors (black on white by default)
    for cell in &result[0] {
        assert_eq!(cell.fg_color, Color::Black, "Should use current_fg color");
        assert_eq!(cell.bg_color, Color::White, "Should use current_bg color");
    }
}

/// Test Case 21: Episode rendering without selection
/// When an Episode component renders with is_selected=false,
/// the output cells should use colors based on the episode's state.
/// Validates: Requirements 3.12
#[test]
fn test_episode_rendering_without_selection() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        true,  // is_watched
        true,  // file_exists
        false, // is_new
    );
    
    let theme = Theme::default();
    let result = episode.render(50, &theme, false); // is_selected=false
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // Cells should use episode colors (Reset by default), not selection colors
    for cell in &result[0] {
        assert_ne!(cell.fg_color, Color::Black, "Should not use selection fg color");
        assert_ne!(cell.bg_color, Color::White, "Should not use selection bg color");
    }
}

/// Test Case 22: Episode rendering with new and watched state
/// When an Episode component renders with both is_new=true and is_watched=true,
/// it should use new colors with the watched indicator.
/// Validates: Requirements 3.6, 3.8
#[test]
fn test_episode_rendering_new_and_watched() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        true,  // is_watched
        true,  // file_exists
        true,  // is_new
    );
    
    let theme = Theme::default();
    let result = episode.render(50, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // First character should be the watched indicator
    assert_eq!(result[0][0].character, '●', "First character should be watched indicator");
    
    // All cells should use new colors (green foreground by default)
    for cell in &result[0] {
        assert_eq!(cell.fg_color, Color::Green, "Should use new_fg color");
        assert_eq!(cell.bg_color, Color::Reset, "Should use new_bg color");
    }
}

/// Test Case 23: Episode rendering priority - invalid overrides new
/// When an Episode component has file_exists=false and is_new=true,
/// it should use invalid colors (invalid has higher priority).
/// Validates: Requirements 3.9
#[test]
fn test_episode_rendering_priority_invalid_over_new() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        false, // is_watched
        false, // file_exists (invalid - highest priority)
        true,  // is_new
    );
    
    let theme = Theme::default();
    let result = episode.render(50, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // All cells should use invalid colors, not new colors
    for cell in &result[0] {
        assert_eq!(cell.fg_color, Color::Red, "Should use invalid_fg color");
        assert_eq!(cell.bg_color, Color::Reset, "Should use invalid_bg color");
    }
}

// ============================================================================
// Episode Text Truncation Tests (Task 5.2)
// ============================================================================

/// Test Case 24: Episode text truncation with width smaller than name length
/// When an Episode component renders with a width smaller than the formatted name length,
/// the output should be truncated to fit within the specified width.
/// Validates: Requirements 3.10
#[test]
fn test_episode_truncation_smaller_width() {
    let episode = Episode::new(
        "This is a very long episode name that should be truncated".to_string(),
        false, // is_watched
        true,  // file_exists
        false, // is_new
    );
    
    let theme = Theme::default();
    let width = 20;
    let result = episode.render(width, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // Output should not exceed the width
    assert!(result[0].len() <= width, "Output length {} should not exceed width {}", result[0].len(), width);
}

/// Test Case 25: Episode truncation with Unicode characters in indicators
/// When an Episode component renders with Unicode indicators and limited width,
/// it should correctly handle multi-byte UTF-8 characters during truncation.
/// Validates: Requirements 3.10
#[test]
fn test_episode_truncation_with_unicode_indicators() {
    let episode = Episode::new(
        "Episode Name".to_string(),
        true,  // is_watched (will add ● indicator)
        true,  // file_exists
        false, // is_new
    );
    
    let theme = Theme::default();
    // Width that includes indicator + space + partial name
    let width = 10;
    let result = episode.render(width, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // Output should not exceed the width
    assert!(result[0].len() <= width, "Output length {} should not exceed width {}", result[0].len(), width);
    
    // First character should still be the watched indicator
    assert_eq!(result[0][0].character, '●', "First character should be watched indicator");
}

/// Test Case 26: Episode truncation with various width values
/// When an Episode component renders with different width values,
/// it should correctly truncate to each specified width.
/// Validates: Requirements 3.10
#[test]
fn test_episode_truncation_various_widths() {
    let episode = Episode::new(
        "Test Episode Name".to_string(),
        false, // is_watched
        true,  // file_exists
        false, // is_new
    );
    
    let theme = Theme::default();
    
    // Test with width 5
    let result_5 = episode.render(5, &theme, false);
    assert!(result_5[0].len() <= 5, "Width 5: output should not exceed width");
    
    // Test with width 10
    let result_10 = episode.render(10, &theme, false);
    assert!(result_10[0].len() <= 10, "Width 10: output should not exceed width");
    
    // Test with width 30
    let result_30 = episode.render(30, &theme, false);
    assert!(result_30[0].len() <= 30, "Width 30: output should not exceed width");
    
    // Test with width 100 (larger than content)
    let result_100 = episode.render(100, &theme, false);
    assert!(result_100[0].len() <= 100, "Width 100: output should not exceed width");
}

/// Test Case 27: Episode truncation preserves indicator
/// When an Episode component is truncated, the indicator should be preserved
/// even if the name is cut off.
/// Validates: Requirements 3.6, 3.7, 3.10
#[test]
fn test_episode_truncation_preserves_indicator() {
    let episode_watched = Episode::new(
        "Very Long Episode Name".to_string(),
        true,  // is_watched
        true,  // file_exists
        false, // is_new
    );
    
    let episode_unwatched = Episode::new(
        "Very Long Episode Name".to_string(),
        false, // is_watched
        true,  // file_exists
        false, // is_new
    );
    
    let theme = Theme::default();
    let width = 5; // Very small width
    
    // Test watched episode
    let result_watched = episode_watched.render(width, &theme, false);
    assert!(!result_watched[0].is_empty(), "Should have cells");
    assert_eq!(result_watched[0][0].character, '●', "Watched indicator should be preserved");
    
    // Test unwatched episode
    let result_unwatched = episode_unwatched.render(width, &theme, false);
    assert!(!result_unwatched[0].is_empty(), "Should have cells");
    assert_eq!(result_unwatched[0][0].character, '○', "Unwatched indicator should be preserved");
}

/// Test Case 28: Episode truncation with exact width match
/// When an Episode component renders with width exactly matching the content length,
/// no truncation should occur.
/// Validates: Requirements 3.10
#[test]
fn test_episode_truncation_exact_width() {
    let episode = Episode::new(
        "Test".to_string(),
        false, // is_watched
        true,  // file_exists
        false, // is_new
    );
    
    let theme = Theme::default();
    // "○ Test" = 6 characters (indicator + space + 4 chars)
    let width = 6;
    let result = episode.render(width, &theme, false);
    
    // Should have exactly 6 cells
    assert_eq!(result[0].len(), 6, "Should have exactly 6 cells");
    
    // Verify content
    assert_eq!(result[0][0].character, '○', "First char should be unwatched indicator");
    assert_eq!(result[0][1].character, ' ', "Second char should be space");
    assert_eq!(result[0][2].character, 'T', "Third char should be 'T'");
    assert_eq!(result[0][3].character, 'e', "Fourth char should be 'e'");
    assert_eq!(result[0][4].character, 's', "Fifth char should be 's'");
    assert_eq!(result[0][5].character, 't', "Sixth char should be 't'");
}

// ============================================================================
// Component Isolation Tests (Task 6.2)
// ============================================================================

/// Test Case 29: Episode::render() does not perform terminal I/O
/// When Episode::render() is called, it should not perform any terminal I/O operations.
/// This test verifies that the component can be tested without terminal interaction.
/// Validates: Requirements 6.1
#[test]
fn test_episode_render_no_terminal_io() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        true,
        true,
        false,
    );
    
    let theme = Theme::default();
    
    // Call render - this should not panic or require terminal setup
    // If this test passes, it means render() doesn't perform terminal I/O
    let result = episode.render(50, &theme, false);
    
    // Verify we got a result
    assert!(!result.is_empty(), "Should return cell array");
    assert!(!result[0].is_empty(), "Should have cells in the row");
}

/// Test Case 30: Cell arrays can be verified without terminal interaction
/// When a component produces Cell arrays, the contents should be verifiable
/// without requiring terminal interaction.
/// Validates: Requirements 6.3
#[test]
fn test_cell_array_verification_without_terminal() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        false, // unwatched
        true,  // file exists
        false, // not new
    );
    
    let theme = Theme::default();
    let result = episode.render(50, &theme, false);
    
    // Verify cell contents without terminal interaction
    assert_eq!(result.len(), 1, "Should have one row");
    
    // Verify first character is unwatched indicator
    assert_eq!(result[0][0].character, '○', "First character should be unwatched indicator");
    
    // Verify colors
    assert_eq!(result[0][0].fg_color, Color::Reset, "Should use episode_fg color");
    assert_eq!(result[0][0].bg_color, Color::Reset, "Should use episode_bg color");
    
    // Verify we can inspect all cells
    for cell in &result[0] {
        // Each cell should have valid character, colors, and style
        assert!(cell.character != '\0', "Cell should have valid character");
    }
}

/// Test Case 31: Component rendering is deterministic
/// When Episode::render() is called multiple times with the same parameters,
/// it should produce identical results.
/// Validates: Requirements 6.1, 6.3
#[test]
fn test_component_rendering_is_deterministic() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        true,
        true,
        false,
    );
    
    let theme = Theme::default();
    
    // Render multiple times
    let result1 = episode.render(50, &theme, false);
    let result2 = episode.render(50, &theme, false);
    let result3 = episode.render(50, &theme, false);
    
    // All results should be identical
    assert_eq!(result1, result2, "First and second render should be identical");
    assert_eq!(result2, result3, "Second and third render should be identical");
}

/// Test Case 32: Component can be tested with different themes
/// When Episode::render() is called with different themes,
/// it should produce different cell arrays without requiring terminal interaction.
/// Validates: Requirements 6.3
#[test]
fn test_component_with_different_themes() {
    let episode = Episode::new(
        "Test Episode".to_string(),
        false,
        true,
        true, // new episode
    );
    
    // Create two different themes
    let theme1 = Theme::default();
    let mut theme2 = Theme::default();
    theme2.new_fg = "blue".to_string();
    
    // Render with both themes
    let result1 = episode.render(50, &theme1, false);
    let result2 = episode.render(50, &theme2, false);
    
    // Results should be different (different colors)
    assert_ne!(result1[0][0].fg_color, result2[0][0].fg_color, "Different themes should produce different colors");
}

/// Test Case 33: Component state can be verified through Cell inspection
/// When a component renders, all state information should be verifiable
/// by inspecting the Cell array without terminal interaction.
/// Validates: Requirements 6.3
#[test]
fn test_component_state_verification_through_cells() {
    // Test watched state
    let watched_episode = Episode::new(
        "Watched Episode".to_string(),
        true,  // watched
        true,
        false,
    );
    
    let theme = Theme::default();
    let watched_result = watched_episode.render(50, &theme, false);
    
    // Verify watched indicator is present
    assert_eq!(watched_result[0][0].character, '●', "Watched episode should have watched indicator");
    
    // Test unwatched state
    let unwatched_episode = Episode::new(
        "Unwatched Episode".to_string(),
        false, // unwatched
        true,
        false,
    );
    
    let unwatched_result = unwatched_episode.render(50, &theme, false);
    
    // Verify unwatched indicator is present
    assert_eq!(unwatched_result[0][0].character, '○', "Unwatched episode should have unwatched indicator");
    
    // Test invalid state
    let invalid_episode = Episode::new(
        "Invalid Episode".to_string(),
        false,
        false, // file doesn't exist
        false,
    );
    
    let invalid_result = invalid_episode.render(50, &theme, false);
    
    // Verify invalid colors are used
    assert_eq!(invalid_result[0][0].fg_color, Color::Red, "Invalid episode should use invalid_fg color");
}
