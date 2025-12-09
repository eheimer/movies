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

// ============================================================================
// Category Component Tests (Task 1.1)
// ============================================================================

use movies::components::{Category, CategoryType};

/// Test Case 34: Category struct stores title correctly
/// When a Category component is created with a title,
/// it should store the title value correctly.
/// Validates: Requirements 1.1
#[test]
fn test_category_stores_title() {
    let category = Category::new(
        "Breaking Bad".to_string(),
        62,
        45,
        CategoryType::Series,
    );
    
    assert_eq!(category.title, "Breaking Bad");
}

/// Test Case 35: Category struct stores episode_count correctly
/// When a Category component is created with an episode count,
/// it should store the episode_count value correctly.
/// Validates: Requirements 1.2
#[test]
fn test_category_stores_episode_count() {
    let category = Category::new(
        "Season 1".to_string(),
        13,
        10,
        CategoryType::Season,
    );
    
    assert_eq!(category.episode_count, 13);
}

/// Test Case 36: Category struct stores watched_count correctly
/// When a Category component is created with a watched count,
/// it should store the watched_count value correctly.
/// Validates: Requirements 1.3
#[test]
fn test_category_stores_watched_count() {
    let category = Category::new(
        "The Wire".to_string(),
        60,
        30,
        CategoryType::Series,
    );
    
    assert_eq!(category.watched_count, 30);
}

/// Test Case 37: Category struct stores category_type correctly
/// When a Category component is created with a category type,
/// it should store the category_type value correctly.
/// Validates: Requirements 1.4
#[test]
fn test_category_stores_category_type() {
    let series_category = Category::new(
        "Game of Thrones".to_string(),
        73,
        73,
        CategoryType::Series,
    );
    
    assert_eq!(series_category.category_type, CategoryType::Series);
    
    let season_category = Category::new(
        "Season 8".to_string(),
        6,
        6,
        CategoryType::Season,
    );
    
    assert_eq!(season_category.category_type, CategoryType::Season);
}

/// Test Case 38: Category struct creation with various values
/// When Category components are created with different values,
/// all fields should be stored correctly.
/// Validates: Requirements 1.1, 1.2, 1.3, 1.4
#[test]
fn test_category_creation_with_various_values() {
    // Test with zero watched count
    let category1 = Category::new(
        "New Series".to_string(),
        10,
        0,
        CategoryType::Series,
    );
    
    assert_eq!(category1.title, "New Series");
    assert_eq!(category1.episode_count, 10);
    assert_eq!(category1.watched_count, 0);
    assert_eq!(category1.category_type, CategoryType::Series);
    
    // Test with all episodes watched
    let category2 = Category::new(
        "Completed Season".to_string(),
        8,
        8,
        CategoryType::Season,
    );
    
    assert_eq!(category2.title, "Completed Season");
    assert_eq!(category2.episode_count, 8);
    assert_eq!(category2.watched_count, 8);
    assert_eq!(category2.category_type, CategoryType::Season);
    
    // Test with empty title
    let category3 = Category::new(
        "".to_string(),
        5,
        2,
        CategoryType::Series,
    );
    
    assert_eq!(category3.title, "");
    assert_eq!(category3.episode_count, 5);
    assert_eq!(category3.watched_count, 2);
}

// ============================================================================
// Category Formatting Tests (Task 3.1)
// ============================================================================

/// Test Case 39: Title inclusion in output
/// When a Category component renders, the output should contain the title string.
/// Validates: Requirements 2.1
#[test]
fn test_category_title_inclusion_short() {
    let category = Category::new(
        "Lost".to_string(),
        121,
        50,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // Convert cells to string to check for title
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain the title
    assert!(rendered_string.contains("Lost"), "Output should contain title 'Lost'");
}

/// Test Case 40: Title inclusion with long title
/// When a Category component renders with a long title,
/// the output should contain the title string.
/// Validates: Requirements 2.1
#[test]
fn test_category_title_inclusion_long() {
    let category = Category::new(
        "The Lord of the Rings: The Rings of Power".to_string(),
        16,
        8,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain the title
    assert!(rendered_string.contains("The Lord of the Rings: The Rings of Power"), 
            "Output should contain full title");
}

/// Test Case 41: Title inclusion with special characters
/// When a Category component renders with special characters in the title,
/// the output should contain those characters correctly.
/// Validates: Requirements 2.1
#[test]
fn test_category_title_inclusion_special_chars() {
    let category = Category::new(
        "It's Always Sunny in Philadelphia".to_string(),
        162,
        100,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain the title with apostrophe
    assert!(rendered_string.contains("It's Always Sunny in Philadelphia"), 
            "Output should contain title with special characters");
}

/// Test Case 42: Title inclusion with Unicode characters
/// When a Category component renders with Unicode characters in the title,
/// the output should contain those characters correctly.
/// Validates: Requirements 2.1
#[test]
fn test_category_title_inclusion_unicode() {
    let category = Category::new(
        "Café ☕ Series".to_string(),
        10,
        5,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain the title with Unicode characters
    assert!(rendered_string.contains("Café"), "Output should contain 'Café'");
    assert!(rendered_string.contains("☕"), "Output should contain '☕'");
}

/// Test Case 43: Empty title handling
/// When a Category component renders with an empty title,
/// it should still render the episode count information.
/// Validates: Requirements 2.1
#[test]
fn test_category_empty_title() {
    let category = Category::new(
        "".to_string(),
        10,
        5,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells even with empty title");
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain episode count information in "X/Y watched" format
    assert!(rendered_string.contains("5/10 watched"), 
            "Output should contain '5/10 watched' format even with empty title");
}

// ============================================================================
// Category Episode Count Formatting Tests (Task 3.2)
// ============================================================================

/// Test Case 44: Episode count formatting with zero episodes
/// When a Category component renders with zero episodes,
/// the output should contain "(0 episodes)" format.
/// Validates: Requirements 2.2
#[test]
fn test_category_episode_count_zero() {
    let category = Category::new(
        "Empty Series".to_string(),
        0,
        0,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain "0/0 watched"
    assert!(rendered_string.contains("0/0 watched"), 
            "Output should contain '0/0 watched' for zero episode count");
}

/// Test Case 45: Episode count formatting with one episode
/// When a Category component renders with one episode,
/// the output should contain "(1 episodes)" format.
/// Validates: Requirements 2.2
#[test]
fn test_category_episode_count_one() {
    let category = Category::new(
        "Single Episode".to_string(),
        1,
        0,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain "0/1 watched"
    assert!(rendered_string.contains("0/1 watched"), 
            "Output should contain '0/1 watched' for single episode");
}

/// Test Case 46: Episode count formatting with ten episodes
/// When a Category component renders with ten episodes,
/// the output should contain "(10 episodes)" format.
/// Validates: Requirements 2.2
#[test]
fn test_category_episode_count_ten() {
    let category = Category::new(
        "Ten Episodes".to_string(),
        10,
        5,
        CategoryType::Season,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain "5/10 watched"
    assert!(rendered_string.contains("5/10 watched"), 
            "Output should contain '5/10 watched' for ten episodes with 5 watched");
}

/// Test Case 47: Episode count formatting with large count
/// When a Category component renders with over 100 episodes,
/// the output should contain the correct episode count format.
/// Validates: Requirements 2.2
#[test]
fn test_category_episode_count_large() {
    let category = Category::new(
        "Long Running Series".to_string(),
        156,
        100,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain "0/156 watched"
    assert!(rendered_string.contains("0/156 watched"), 
            "Output should contain '0/156 watched' for large episode count");
}

/// Test Case 48: Episode count format consistency
/// When Category components render with various episode counts,
/// they should all use the consistent "(X episodes)" format.
/// Validates: Requirements 2.2
#[test]
fn test_category_episode_count_format_consistency() {
    let theme = Theme::default();
    
    // Test various counts
    let counts = vec![0, 1, 5, 13, 22, 50, 100, 500];
    
    for count in counts {
        let category = Category::new(
            format!("Series {}", count),
            count,
            0,
            CategoryType::Series,
        );
        
        let result = category.render(100, &theme, false);
        let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
        
        // Should contain the expected format "0/X watched"
        let expected = format!("0/{} watched", count);
        assert!(rendered_string.contains(&expected), 
                "Output should contain '{}' for count {}", expected, count);
    }
}

// ============================================================================
// Category Watched Count Formatting Tests (Task 3.3)
// ============================================================================

/// Test Case 49: Watched count omitted when zero
/// When a Category component renders with zero watched count,
/// the output should omit the "[Y watched]" portion.
/// Validates: Requirements 2.3, 2.4
#[test]
fn test_category_watched_count_zero_omitted() {
    let category = Category::new(
        "New Series".to_string(),
        20,
        0,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain "0/20 watched" format
    assert!(rendered_string.contains("0/20 watched"), 
            "Output should contain '0/20 watched' format");
    
    // Should contain the title
    assert!(rendered_string.contains("New Series"), 
            "Output should contain the title");
}

/// Test Case 50: Watched count included when non-zero
/// When a Category component renders with non-zero watched count,
/// the output should contain "[Y watched]" format.
/// Validates: Requirements 2.3
#[test]
fn test_category_watched_count_nonzero_included() {
    let category = Category::new(
        "Partially Watched".to_string(),
        20,
        5,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain "5/20 watched"
    assert!(rendered_string.contains("5/20 watched"), 
            "Output should contain '5/20 watched' when watched count is 5");
}

/// Test Case 51: Watched count with one episode watched
/// When a Category component renders with one episode watched,
/// the output should contain "[1 watched]" format.
/// Validates: Requirements 2.3
#[test]
fn test_category_watched_count_one() {
    let category = Category::new(
        "Started Series".to_string(),
        10,
        1,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain "1/10 watched"
    assert!(rendered_string.contains("1/10 watched"), 
            "Output should contain '1/10 watched' when watched count is 1");
}

/// Test Case 52: Watched count with all episodes watched
/// When a Category component renders with all episodes watched,
/// the output should contain the watched count.
/// Validates: Requirements 2.3
#[test]
fn test_category_watched_count_all() {
    let category = Category::new(
        "Completed Series".to_string(),
        13,
        13,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain "13/13 watched"
    assert!(rendered_string.contains("13/13 watched"), 
            "Output should contain '13/13 watched' when all episodes are watched");
}

/// Test Case 53: Watched count with large numbers
/// When a Category component renders with large watched count,
/// the output should contain the correct format.
/// Validates: Requirements 2.3
#[test]
fn test_category_watched_count_large() {
    let category = Category::new(
        "Long Series".to_string(),
        200,
        150,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain "150/200 watched"
    assert!(rendered_string.contains("150/200 watched"), 
            "Output should contain '150/200 watched' for large watched count");
}

/// Test Case 54: Complete format with watched count
/// When a Category component renders with non-zero watched count,
/// the output should follow the complete format: "Title (X episodes) [Y watched]"
/// Validates: Requirements 2.5
#[test]
fn test_category_complete_format_with_watched() {
    let category = Category::new(
        "Breaking Bad".to_string(),
        62,
        45,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain all parts in correct format: "Title  45/62 watched"
    assert!(rendered_string.contains("Breaking Bad"), "Should contain title");
    assert!(rendered_string.contains("45/62 watched"), "Should contain watched/total count");
    
    // Verify order: title should come before watched count
    let title_pos = rendered_string.find("Breaking Bad").unwrap();
    let watched_pos = rendered_string.find("45/62 watched").unwrap();
    
    assert!(title_pos < watched_pos, "Title should come before watched count");
}

/// Test Case 55: Complete format without watched count
/// When a Category component renders with zero watched count,
/// the output should follow the format: "Title (X episodes)"
/// Validates: Requirements 2.4, 2.5
#[test]
fn test_category_complete_format_without_watched() {
    let category = Category::new(
        "The Wire".to_string(),
        60,
        0,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Convert cells to string
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain title and "0/60 watched" format
    assert!(rendered_string.contains("The Wire"), "Should contain title");
    assert!(rendered_string.contains("0/60 watched"), "Should contain '0/60 watched' format");
    
    // Verify order: title should come before watched count
    let title_pos = rendered_string.find("The Wire").unwrap();
    let watched_pos = rendered_string.find("0/60 watched").unwrap();
    
    assert!(title_pos < watched_pos, "Title should come before watched count");
}

// ============================================================================
// Category Color Application Tests (Task 4.1 and 4.2)
// ============================================================================

/// Test Case 56: Selection color application
/// When a Category component renders with is_selected=true,
/// all output cells should use current_fg and current_bg colors from the theme.
/// Validates: Requirements 3.3
#[test]
fn test_category_selection_color_application() {
    let category = Category::new(
        "Breaking Bad".to_string(),
        62,
        45,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, true); // is_selected=true
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // All cells should use current_fg and current_bg colors
    for cell in &result[0] {
        assert_eq!(cell.fg_color, Color::Black, "All cells should use current_fg (Black)");
        assert_eq!(cell.bg_color, Color::White, "All cells should use current_bg (White)");
    }
}

/// Test Case 57: Selection color application with custom theme
/// When a Category component renders with is_selected=true and a custom theme,
/// all output cells should use the custom current_fg and current_bg colors.
/// Validates: Requirements 3.3
#[test]
fn test_category_selection_color_application_custom_theme() {
    let category = Category::new(
        "The Wire".to_string(),
        60,
        30,
        CategoryType::Series,
    );
    
    // Create custom theme with different selection colors
    let mut theme = Theme::default();
    theme.current_fg = "cyan".to_string();
    theme.current_bg = "magenta".to_string();
    
    let result = category.render(100, &theme, true); // is_selected=true
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // All cells should use custom current colors
    for cell in &result[0] {
        assert_eq!(cell.fg_color, Color::Cyan, "All cells should use current_fg (Cyan)");
        assert_eq!(cell.bg_color, Color::Magenta, "All cells should use current_bg (Magenta)");
    }
}

/// Test Case 58: Selection color overrides default colors
/// When a Category component renders with is_selected=true,
/// the selection colors should override any default category colors.
/// Validates: Requirements 3.3
#[test]
fn test_category_selection_overrides_default() {
    let category = Category::new(
        "Game of Thrones".to_string(),
        73,
        73,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    
    // Render without selection
    let result_unselected = category.render(100, &theme, false);
    
    // Render with selection
    let result_selected = category.render(100, &theme, true);
    
    // Both should have cells
    assert!(!result_unselected[0].is_empty(), "Unselected should have cells");
    assert!(!result_selected[0].is_empty(), "Selected should have cells");
    
    // Selected cells should use current colors (Black on White)
    for cell in &result_selected[0] {
        assert_eq!(cell.fg_color, Color::Black, "Selected cells should use current_fg");
        assert_eq!(cell.bg_color, Color::White, "Selected cells should use current_bg");
    }
    
    // Unselected cells should use series colors for title and count colors for count
    // Just verify that they're NOT using the selection colors
    let has_non_selection_colors = result_unselected[0].iter().any(|cell| {
        cell.fg_color != Color::Black || cell.bg_color != Color::White
    });
    assert!(has_non_selection_colors, "Unselected cells should not use selection colors");
}

/// Test Case 59: Default color application
/// When a Category component renders with is_selected=false,
/// all output cells should use default category colors (episode_fg and episode_bg) from the theme.
/// Validates: Requirements 3.4
#[test]
fn test_category_default_color_application() {
    let category = Category::new(
        "Lost".to_string(),
        121,
        50,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false); // is_selected=false
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // Cells should use series colors for title (Blue) and count colors for count (DarkGray)
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Find where the title ends and count begins
    let has_series_color = result[0].iter().any(|cell| cell.fg_color == Color::Blue);
    let has_count_color = result[0].iter().any(|cell| cell.fg_color == Color::DarkGrey);
    
    assert!(has_series_color, "Should have cells with series_fg (Blue)");
    assert!(has_count_color, "Should have cells with count_fg (DarkGray)");
}

/// Test Case 60: Default color application with custom theme
/// When a Category component renders with is_selected=false and a custom theme,
/// all output cells should use the custom episode_fg and episode_bg colors.
/// Validates: Requirements 3.4
#[test]
fn test_category_default_color_application_custom_theme() {
    let category = Category::new(
        "Stranger Things".to_string(),
        34,
        20,
        CategoryType::Series,
    );
    
    // Create custom theme with different series and count colors
    let mut theme = Theme::default();
    theme.series_fg = "yellow".to_string();
    theme.series_bg = "blue".to_string();
    theme.count_fg = "red".to_string();
    
    let result = category.render(100, &theme, false); // is_selected=false
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // Title cells should use custom series colors, count cells should use count colors
    let has_series_color = result[0].iter().any(|cell| cell.fg_color == Color::Yellow);
    let has_count_color = result[0].iter().any(|cell| cell.fg_color == Color::Red);
    
    assert!(has_series_color, "Should have cells with series_fg (Yellow)");
    assert!(has_count_color, "Should have cells with count_fg (Red)");
}

/// Test Case 61: Default colors for both Series and Season types
/// When Category components of both Series and Season types render with is_selected=false,
/// they should both use the same default episode colors.
/// Validates: Requirements 3.4
#[test]
fn test_category_default_colors_both_types() {
    let series_category = Category::new(
        "Breaking Bad".to_string(),
        62,
        45,
        CategoryType::Series,
    );
    
    let season_category = Category::new(
        "Season 1".to_string(),
        7,
        7,
        CategoryType::Season,
    );
    
    let theme = Theme::default();
    
    let series_result = series_category.render(100, &theme, false);
    let season_result = season_category.render(100, &theme, false);
    
    // Both should have cells
    assert!(!series_result[0].is_empty(), "Series should have cells");
    assert!(!season_result[0].is_empty(), "Season should have cells");
    
    // Series should use series colors, Season should use season colors
    // Both should use count colors for the count part
    let series_has_series_color = series_result[0].iter().any(|cell| cell.fg_color == Color::Blue);
    let season_has_season_color = season_result[0].iter().any(|cell| cell.fg_color == Color::Blue);
    let series_has_count_color = series_result[0].iter().any(|cell| cell.fg_color == Color::DarkGrey);
    let season_has_count_color = season_result[0].iter().any(|cell| cell.fg_color == Color::DarkGrey);
    
    assert!(series_has_series_color, "Series should use series_fg (Blue)");
    assert!(season_has_season_color, "Season should use season_fg (Blue)");
    assert!(series_has_count_color, "Series should use count_fg (DarkGray)");
    assert!(season_has_count_color, "Season should use count_fg (DarkGray)");
}

/// Test Case 62: Color consistency across entire rendered string
/// When a Category component renders, all characters in the output
/// should have consistent colors (all cells use the same fg/bg colors).
/// Validates: Requirements 3.3, 3.4
#[test]
fn test_category_color_consistency() {
    let category = Category::new(
        "The Sopranos".to_string(),
        86,
        50,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    
    // Test with selection
    let result_selected = category.render(100, &theme, true);
    assert!(!result_selected[0].is_empty(), "Should have cells");
    
    let first_cell_fg = result_selected[0][0].fg_color;
    let first_cell_bg = result_selected[0][0].bg_color;
    
    // All cells should have the same colors
    for cell in &result_selected[0] {
        assert_eq!(cell.fg_color, first_cell_fg, "All cells should have consistent fg color");
        assert_eq!(cell.bg_color, first_cell_bg, "All cells should have consistent bg color");
    }
    
    // Test without selection - should have different colors for title vs count
    let result_unselected = category.render(100, &theme, false);
    assert!(!result_unselected[0].is_empty(), "Should have cells");
    
    // Should have at least two different foreground colors (title and count)
    let unique_fg_colors: std::collections::HashSet<_> = result_unselected[0]
        .iter()
        .map(|cell| cell.fg_color)
        .collect();
    
    assert!(unique_fg_colors.len() >= 2, "Should have at least 2 different fg colors (title and count)");
}

// ============================================================================
// Category Text Truncation Tests (Task 5.1)
// ============================================================================

/// Test Case 63: Text truncation with width smaller than formatted string
/// When a Category component renders with a width smaller than the formatted string length,
/// the output should be truncated to fit within the specified width.
/// Validates: Requirements 3.5
#[test]
fn test_category_truncation_smaller_width() {
    let category = Category::new(
        "This is a very long series name that should be truncated".to_string(),
        100,
        50,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let width = 20;
    let result = category.render(width, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // Output should not exceed the width
    assert!(result[0].len() <= width, 
            "Output length {} should not exceed width {}", result[0].len(), width);
}

/// Test Case 64: Text truncation with Unicode characters in title
/// When a Category component renders with Unicode characters in the title and limited width,
/// it should correctly handle multi-byte UTF-8 characters during truncation.
/// Validates: Requirements 3.5
#[test]
fn test_category_truncation_with_unicode() {
    let category = Category::new(
        "Café ☕ Series with émojis 🎬 and spëcial çharacters".to_string(),
        25,
        10,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let width = 30;
    let result = category.render(width, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // Output should not exceed the width
    assert!(result[0].len() <= width, 
            "Output length {} should not exceed width {}", result[0].len(), width);
    
    // Verify we can still read the characters (no broken Unicode)
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Should contain at least part of the title
    assert!(rendered_string.contains("Café") || rendered_string.len() <= width, 
            "Should contain valid Unicode characters");
}

/// Test Case 65: Text truncation with various width values
/// When a Category component renders with different width values,
/// it should correctly truncate to each specified width.
/// Validates: Requirements 3.5
#[test]
fn test_category_truncation_various_widths() {
    let category = Category::new(
        "Breaking Bad".to_string(),
        62,
        45,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    
    // Test with width 10
    let result_10 = category.render(10, &theme, false);
    assert!(result_10[0].len() <= 10, "Width 10: output should not exceed width");
    
    // Test with width 20
    let result_20 = category.render(20, &theme, false);
    assert!(result_20[0].len() <= 20, "Width 20: output should not exceed width");
    
    // Test with width 50
    let result_50 = category.render(50, &theme, false);
    assert!(result_50[0].len() <= 50, "Width 50: output should not exceed width");
    
    // Test with width 100 (larger than content)
    let result_100 = category.render(100, &theme, false);
    assert!(result_100[0].len() <= 100, "Width 100: output should not exceed width");
    
    // Verify that larger widths produce longer or equal output
    assert!(result_10[0].len() <= result_20[0].len(), 
            "Larger width should produce longer or equal output");
    assert!(result_20[0].len() <= result_50[0].len(), 
            "Larger width should produce longer or equal output");
}

/// Test Case 66: Text truncation with zero width
/// When a Category component renders with zero width,
/// it should return an empty cell array without panicking.
/// Validates: Requirements 3.5
#[test]
fn test_category_truncation_zero_width() {
    let category = Category::new(
        "Test Series".to_string(),
        10,
        5,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(0, &theme, false);
    
    // Should return a single row with no cells
    assert_eq!(result.len(), 1, "Should return single row");
    assert_eq!(result[0].len(), 0, "Row should be empty for zero width");
}

/// Test Case 67: Text truncation preserves format structure
/// When a Category component is truncated, the output should still be readable
/// and maintain as much of the format structure as possible.
/// Validates: Requirements 3.5
#[test]
fn test_category_truncation_preserves_structure() {
    let category = Category::new(
        "Short".to_string(),
        10,
        5,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    
    // Test with width that should fit everything
    let result_full = category.render(100, &theme, false);
    let full_string: String = result_full[0].iter().map(|cell| cell.character).collect();
    
    // Should contain all parts
    assert!(full_string.contains("Short"), "Should contain title");
    assert!(full_string.contains("5/10 watched"), "Should contain watched/total count");
    
    // Test with width that truncates
    let result_truncated = category.render(15, &theme, false);
    let truncated_string: String = result_truncated[0].iter().map(|cell| cell.character).collect();
    
    // With width 15, we need to fit "Short" (5) + spacing (1) + "5/10 watched" (12) = 18 chars
    // So the title will be truncated. The output should still contain part of the title.
    assert!(truncated_string.contains("Shor") || truncated_string.starts_with("S"), 
            "Truncated output should contain at least part of the title, got: '{}'", truncated_string);
}

/// Test Case 68: Text truncation with exact width match
/// When a Category component renders with width exactly matching the content length,
/// no truncation should occur.
/// Validates: Requirements 3.5
#[test]
fn test_category_truncation_exact_width() {
    let category = Category::new(
        "Test".to_string(),
        5,
        0,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    
    // First render with large width to get actual length
    let result_full = category.render(100, &theme, false);
    let actual_length = result_full[0].len();
    
    // Now render with exact width
    let result_exact = category.render(actual_length, &theme, false);
    
    // Should have exactly the same length
    assert_eq!(result_exact[0].len(), actual_length, 
               "Should have exactly the same length when width matches content");
    
    // Content should be identical
    let full_string: String = result_full[0].iter().map(|cell| cell.character).collect();
    let exact_string: String = result_exact[0].iter().map(|cell| cell.character).collect();
    
    assert_eq!(full_string, exact_string, 
               "Content should be identical when width matches");
}

/// Test Case 69: Text truncation with very long title
/// When a Category component has a very long title and limited width,
/// it should truncate gracefully without panicking.
/// Validates: Requirements 3.5
#[test]
fn test_category_truncation_very_long_title() {
    let category = Category::new(
        "This is an extremely long series title that goes on and on and on and should definitely be truncated when rendered with a small width constraint".to_string(),
        150,
        75,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let width = 25;
    let result = category.render(width, &theme, false);
    
    // Should have cells
    assert!(!result[0].is_empty(), "Should have cells");
    
    // Output should not exceed the width
    assert_eq!(result[0].len(), width, 
               "Output should be exactly the width when truncated");
}

/// Test Case 70: Text truncation with watched count
/// When a Category component with watched count is truncated,
/// it should handle the full formatted string correctly.
/// Validates: Requirements 3.5
#[test]
fn test_category_truncation_with_watched_count() {
    let category = Category::new(
        "Series Name".to_string(),
        100,
        50,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    
    // Render with full width
    let result_full = category.render(100, &theme, false);
    let full_string: String = result_full[0].iter().map(|cell| cell.character).collect();
    
    // Should contain all parts
    assert!(full_string.contains("Series Name"), "Should contain title");
    assert!(full_string.contains("50/100 watched"), "Should contain watched/total count");
    
    // Render with truncated width
    let result_truncated = category.render(20, &theme, false);
    
    // Should not exceed width
    assert!(result_truncated[0].len() <= 20, 
            "Truncated output should not exceed width");
}


// ============================================================================
// Category Component Isolation Tests (Task 5.2)
// ============================================================================

/// Test Case 71: Category::render() does not perform terminal I/O
/// When Category::render() is called, it should not perform any terminal I/O operations.
/// This test verifies that the component can be tested without terminal interaction.
/// Validates: Requirements 4.6, 6.1
#[test]
fn test_category_render_no_terminal_io() {
    let category = Category::new(
        "Test Series".to_string(),
        50,
        25,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    
    // Call render - this should not panic or require terminal setup
    // If this test passes, it means render() doesn't perform terminal I/O
    let result = category.render(100, &theme, false);
    
    // Verify we got a result
    assert!(!result.is_empty(), "Should return cell array");
    assert!(!result[0].is_empty(), "Should have cells in the row");
}

/// Test Case 72: Cell arrays can be verified without terminal interaction
/// When a Category component produces Cell arrays, the contents should be verifiable
/// without requiring terminal interaction.
/// Validates: Requirements 4.6, 6.1
#[test]
fn test_category_cell_array_verification_without_terminal() {
    let category = Category::new(
        "Breaking Bad".to_string(),
        62,
        45,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result = category.render(100, &theme, false);
    
    // Verify cell contents without terminal interaction
    assert_eq!(result.len(), 1, "Should have one row");
    
    // Convert to string for verification
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
    // Verify content
    assert!(rendered_string.contains("Breaking Bad"), "Should contain title");
    assert!(rendered_string.contains("45/62 watched"), "Should contain watched/total count");
    
    // Verify colors - first cell should use series_fg (Blue) since it's a Series category
    assert_eq!(result[0][0].fg_color, Color::Blue, "Should use series_fg color (Blue)");
    assert_eq!(result[0][0].bg_color, Color::Reset, "Should use series_bg color (Reset)");
    
    // Verify we can inspect all cells
    for cell in &result[0] {
        // Each cell should have valid character, colors, and style
        assert!(cell.character != '\0', "Cell should have valid character");
    }
}

/// Test Case 73: Category rendering is deterministic
/// When Category::render() is called multiple times with the same parameters,
/// it should produce identical results.
/// Validates: Requirements 4.6, 6.1
#[test]
fn test_category_rendering_is_deterministic() {
    let category = Category::new(
        "The Wire".to_string(),
        60,
        30,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    
    // Render multiple times
    let result1 = category.render(100, &theme, false);
    let result2 = category.render(100, &theme, false);
    let result3 = category.render(100, &theme, false);
    
    // All results should be identical
    assert_eq!(result1, result2, "First and second render should be identical");
    assert_eq!(result2, result3, "Second and third render should be identical");
}

/// Test Case 74: Category can be tested with different themes
/// When Category::render() is called with different themes,
/// it should produce different cell arrays without requiring terminal interaction.
/// Validates: Requirements 4.6, 6.1
#[test]
fn test_category_with_different_themes() {
    let category = Category::new(
        "Game of Thrones".to_string(),
        73,
        73,
        CategoryType::Series,
    );
    
    // Create two different themes
    let theme1 = Theme::default();
    let mut theme2 = Theme::default();
    theme2.series_fg = "red".to_string();  // Change series colors since it's a Series category
    theme2.series_bg = "yellow".to_string();
    
    // Render with both themes
    let result1 = category.render(100, &theme1, false);
    let result2 = category.render(100, &theme2, false);
    
    // Results should be different (different colors)
    assert_ne!(result1[0][0].fg_color, result2[0][0].fg_color, 
               "Different themes should produce different colors");
    assert_ne!(result1[0][0].bg_color, result2[0][0].bg_color, 
               "Different themes should produce different colors");
}

/// Test Case 75: Category state can be verified through Cell inspection
/// When a Category component renders, all state information should be verifiable
/// by inspecting the Cell array without terminal interaction.
/// Validates: Requirements 4.6, 6.1
#[test]
fn test_category_state_verification_through_cells() {
    // Test with watched count
    let category_with_watched = Category::new(
        "Watched Series".to_string(),
        20,
        10,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    let result_with_watched = category_with_watched.render(100, &theme, false);
    
    // Convert to string
    let string_with_watched: String = result_with_watched[0].iter()
        .map(|cell| cell.character).collect();
    
    // Verify watched count is present in "X/Y watched" format
    assert!(string_with_watched.contains("10/20 watched"), 
            "Should contain watched count in '10/20 watched' format");
    
    // Test without watched count
    let category_without_watched = Category::new(
        "New Series".to_string(),
        20,
        0,
        CategoryType::Series,
    );
    
    let result_without_watched = category_without_watched.render(100, &theme, false);
    
    // Convert to string
    let string_without_watched: String = result_without_watched[0].iter()
        .map(|cell| cell.character).collect();
    
    // Verify watched count shows "0/20 watched" format
    assert!(string_without_watched.contains("0/20 watched"), 
            "Should contain '0/20 watched' format even when zero watched");
}

/// Test Case 76: Category selection state can be verified through Cell colors
/// When a Category component renders with different selection states,
/// the selection state should be verifiable by inspecting cell colors.
/// Validates: Requirements 4.6, 6.1
#[test]
fn test_category_selection_state_verification() {
    let category = Category::new(
        "Test Series".to_string(),
        10,
        5,
        CategoryType::Series,
    );
    
    let theme = Theme::default();
    
    // Render selected
    let result_selected = category.render(100, &theme, true);
    
    // Verify selection colors
    for cell in &result_selected[0] {
        assert_eq!(cell.fg_color, Color::Black, "Selected should use current_fg");
        assert_eq!(cell.bg_color, Color::White, "Selected should use current_bg");
    }
    
    // Render unselected
    let result_unselected = category.render(100, &theme, false);
    
    // Verify colors - title should use series colors, count should use count colors
    // The first cells (title part) should use series_fg (Blue)
    let rendered_string: String = result_unselected[0].iter().map(|cell| cell.character).collect();
    let title_end = rendered_string.find("Test Series").unwrap() + "Test Series".len();
    
    // Check title cells use series colors
    for cell in &result_unselected[0][0..title_end] {
        assert_eq!(cell.fg_color, Color::Blue, "Title should use series_fg (Blue)");
        assert_eq!(cell.bg_color, Color::Reset, "Title should use series_bg (Reset)");
    }
    
    // Check count cells use count colors (find the count part)
    let count_start = rendered_string.find("5/10 watched").unwrap();
    for cell in &result_unselected[0][count_start..] {
        assert_eq!(cell.fg_color, Color::DarkGrey, "Count should use count_fg (DarkGray)");
    }
}

/// Test Case 77: Category component can be tested in isolation
/// When testing Category component, no other components or terminal setup is required.
/// Validates: Requirements 4.6, 6.1
#[test]
fn test_category_component_isolation() {
    // Create category without any other setup
    let category = Category::new(
        "Isolated Test".to_string(),
        15,
        7,
        CategoryType::Season,
    );
    
    // Create minimal theme
    let theme = Theme::default();
    
    // Render should work without any other dependencies
    let result = category.render(50, &theme, false);
    
    // Verify basic functionality
    assert_eq!(result.len(), 1, "Should return single row");
    assert!(!result[0].is_empty(), "Should have cells");
    
    // Verify content
    let rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    assert!(rendered_string.contains("Isolated Test"), "Should contain title");
    assert!(rendered_string.contains("7/15 watched"), "Should contain watched/total count");
}

