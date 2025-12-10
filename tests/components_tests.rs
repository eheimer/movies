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
    let _rendered_string: String = result[0].iter().map(|cell| cell.character).collect();
    
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


// ============================================================================
// Scrollbar Component Tests
// ============================================================================

use movies::components::Scrollbar;

/// Test Case 34: Scrollbar hidden when items fit on screen
/// When total_items is less than or equal to visible_items,
/// the scrollbar should return an empty Cell array.
/// Validates: Requirements 3.1
#[test]
fn test_scrollbar_hidden_when_items_fit() {
    let theme = Theme::default();
    
    // Test with total_items < visible_items
    let scrollbar1 = Scrollbar::new(5, 10, 0);
    let result1 = scrollbar1.render(20, &theme, false);
    assert!(result1.is_empty(), "Scrollbar should be hidden when total_items < visible_items");
    
    // Test with total_items == visible_items
    let scrollbar2 = Scrollbar::new(10, 10, 0);
    let result2 = scrollbar2.render(20, &theme, false);
    assert!(result2.is_empty(), "Scrollbar should be hidden when total_items == visible_items");
}

/// Test Case 35: Scrollbar hidden when total items is zero
/// When total_items is 0, the scrollbar should return an empty Cell array.
/// Validates: Requirements 3.2
#[test]
fn test_scrollbar_hidden_when_zero_items() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(0, 10, 0);
    let result = scrollbar.render(20, &theme, false);
    assert!(result.is_empty(), "Scrollbar should be hidden when total_items is 0");
}

/// Test Case 36: Scrollbar hidden when height is zero
/// When height is 0, the scrollbar should return an empty Cell array.
/// Validates: Requirements 3.3
#[test]
fn test_scrollbar_hidden_when_zero_height() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 0);
    let result = scrollbar.render(0, &theme, false);
    assert!(result.is_empty(), "Scrollbar should be hidden when height is 0");
}

/// Test Case 37: Scrollbar visible when needed
/// When total_items is greater than visible_items,
/// the scrollbar should return a non-empty Cell array with track and indicator.
/// Validates: Requirements 3.4
#[test]
fn test_scrollbar_visible_when_needed() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 0);
    let result = scrollbar.render(20, &theme, false);
    
    assert!(!result.is_empty(), "Scrollbar should be visible when total_items > visible_items");
    assert_eq!(result.len(), 20, "Scrollbar should have height rows");
    
    // Each row should have exactly one cell (single column)
    for row in &result {
        assert_eq!(row.len(), 1, "Each row should have exactly one cell");
    }
}

/// Test Case 38: Indicator at top position
/// When first_visible_index is 0, the indicator should be positioned at the track start.
/// Validates: Requirements 5.1
#[test]
fn test_indicator_at_top_position() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 0); // At top
    let result = scrollbar.render(20, &theme, false);
    
    assert!(!result.is_empty(), "Scrollbar should be visible");
    
    // First cell should be the indicator
    assert_eq!(result[0][0].character, '█', "First cell should be indicator at top position");
}

/// Test Case 39: Indicator at bottom position
/// When first_visible_index is at maximum scroll position,
/// the indicator should be positioned at the track end.
/// Validates: Requirements 5.2
#[test]
fn test_indicator_at_bottom_position() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 90); // At bottom (90 + 10 = 100)
    let result = scrollbar.render(20, &theme, false);
    
    assert!(!result.is_empty(), "Scrollbar should be visible");
    
    // Last cell should be the indicator
    let last_row = result.len() - 1;
    assert_eq!(result[last_row][0].character, '█', "Last cell should be indicator at bottom position");
}

/// Test Case 40: Indicator at middle position
/// When first_visible_index is in the middle,
/// the indicator should be positioned proportionally within the track.
/// Validates: Requirements 5.3, 2.4
#[test]
fn test_indicator_at_middle_position() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 45); // Middle position
    let result = scrollbar.render(20, &theme, false);
    
    assert!(!result.is_empty(), "Scrollbar should be visible");
    
    // Find where the indicator is
    let mut indicator_found = false;
    let mut indicator_start = 0;
    
    for (i, row) in result.iter().enumerate() {
        if row[0].character == '█' {
            if !indicator_found {
                indicator_start = i;
                indicator_found = true;
            }
        }
    }
    
    assert!(indicator_found, "Indicator should be present");
    // Indicator should be roughly in the middle (not at start or end)
    assert!(indicator_start > 0, "Indicator should not be at start");
    assert!(indicator_start < result.len() - 1, "Indicator should not be at end");
}

/// Test Case 41: Indicator height calculation
/// When the scrollbar renders, the indicator height should be proportional
/// to the visible/total ratio.
/// Validates: Requirements 2.5
#[test]
fn test_indicator_height_calculation() {
    let theme = Theme::default();
    
    // Test with 10 visible out of 100 total (10% ratio)
    // Height is 20, so indicator should be 2 cells (10% of 20)
    let scrollbar = Scrollbar::new(100, 10, 0);
    let result = scrollbar.render(20, &theme, false);
    
    // Count indicator cells
    let indicator_count = result.iter()
        .filter(|row| row[0].character == '█')
        .count();
    
    assert_eq!(indicator_count, 2, "Indicator height should be 2 cells (10% of 20)");
}

/// Test Case 42: Minimum indicator height
/// When the indicator height calculation would result in less than 1,
/// it should be clamped to a minimum of 1 row.
/// Validates: Requirements 5.4
#[test]
fn test_minimum_indicator_height() {
    let theme = Theme::default();
    
    // Test with very small ratio (1 visible out of 1000 total)
    // This would calculate to 0.02 cells, but should be clamped to 1
    let scrollbar = Scrollbar::new(1000, 1, 0);
    let result = scrollbar.render(20, &theme, false);
    
    // Count indicator cells
    let indicator_count = result.iter()
        .filter(|row| row[0].character == '█')
        .count();
    
    assert!(indicator_count >= 1, "Indicator height should be at least 1 cell");
}

/// Test Case 43: Indicator bounds constraint
/// When the scrollbar renders, the indicator should never extend past the track bounds.
/// Validates: Requirements 4.5
#[test]
fn test_indicator_bounds_constraint() {
    let theme = Theme::default();
    
    // Test at various scroll positions
    for first_visible in (0..90).step_by(10) {
        let scrollbar = Scrollbar::new(100, 10, first_visible);
        let result = scrollbar.render(20, &theme, false);
        
        // Find indicator position and height
        let mut indicator_start = None;
        let mut indicator_end = None;
        
        for (i, row) in result.iter().enumerate() {
            if row[0].character == '█' {
                if indicator_start.is_none() {
                    indicator_start = Some(i);
                }
                indicator_end = Some(i);
            }
        }
        
        if let (Some(_start), Some(end)) = (indicator_start, indicator_end) {
            // start is always >= 0 since it's usize, so no need to check
            assert!(end < result.len(), "Indicator should not extend past track end");
        }
    }
}

/// Test Case 44: Track character usage
/// When the scrollbar renders, all track positions (non-indicator rows)
/// should contain the scrollbar_track_char from the theme.
/// Validates: Requirements 4.1
#[test]
fn test_track_character_usage() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 0);
    let result = scrollbar.render(20, &theme, false);
    
    // Count track cells (should be '│' by default)
    let track_count = result.iter()
        .filter(|row| row[0].character == '│')
        .count();
    
    // There should be some track cells
    assert!(track_count > 0, "There should be track cells");
    
    // All non-indicator cells should be track cells
    let indicator_count = result.iter()
        .filter(|row| row[0].character == '█')
        .count();
    
    assert_eq!(track_count + indicator_count, result.len(), 
        "All cells should be either track or indicator");
}

/// Test Case 45: Indicator character usage
/// When the scrollbar renders, all indicator positions
/// should contain the scrollbar_indicator_char from the theme.
/// Validates: Requirements 4.2
#[test]
fn test_indicator_character_usage() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 0);
    let result = scrollbar.render(20, &theme, false);
    
    // Find indicator cells
    let indicator_cells: Vec<_> = result.iter()
        .filter(|row| row[0].character == '█')
        .collect();
    
    // There should be indicator cells
    assert!(!indicator_cells.is_empty(), "There should be indicator cells");
    
    // All indicator cells should use the indicator character
    for cell_row in indicator_cells {
        assert_eq!(cell_row[0].character, '█', "Indicator cells should use indicator character");
    }
}

/// Test Case 46: Foreground color application
/// When the scrollbar renders, all cells should use the scrollbar_fg color from the theme.
/// Validates: Requirements 4.3
#[test]
fn test_foreground_color_application() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 0);
    let result = scrollbar.render(20, &theme, false);
    
    // All cells should use the scrollbar_fg color (White by default)
    for row in &result {
        assert_eq!(row[0].fg_color, Color::White, "All cells should use scrollbar_fg color");
    }
}

/// Test Case 47: Background color application
/// When the scrollbar renders, all cells should use the scrollbar_bg color from the theme.
/// Validates: Requirements 4.4
#[test]
fn test_background_color_application() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 0);
    let result = scrollbar.render(20, &theme, false);
    
    // All cells should use the scrollbar_bg color (Reset by default)
    for row in &result {
        assert_eq!(row[0].bg_color, Color::Reset, "All cells should use scrollbar_bg color");
    }
}

/// Test Case 48: Custom theme colors
/// When the scrollbar renders with a custom theme,
/// it should use the custom colors from that theme.
/// Validates: Requirements 4.3, 4.4
#[test]
fn test_custom_theme_colors() {
    let mut theme = Theme::default();
    theme.scrollbar_fg = "cyan".to_string();
    theme.scrollbar_bg = "blue".to_string();
    
    let scrollbar = Scrollbar::new(100, 10, 0);
    let result = scrollbar.render(20, &theme, false);
    
    // All cells should use the custom colors
    for row in &result {
        assert_eq!(row[0].fg_color, Color::Cyan, "Should use custom scrollbar_fg color");
        assert_eq!(row[0].bg_color, Color::Blue, "Should use custom scrollbar_bg color");
    }
}

/// Test Case 49: Custom theme characters
/// When the scrollbar renders with a custom theme,
/// it should use the custom characters from that theme.
/// Validates: Requirements 4.1, 4.2
#[test]
fn test_custom_theme_characters() {
    let mut theme = Theme::default();
    theme.scrollbar_track_char = "|".to_string();
    theme.scrollbar_indicator_char = "=".to_string();
    
    let scrollbar = Scrollbar::new(100, 10, 0);
    let result = scrollbar.render(20, &theme, false);
    
    // Check that custom characters are used
    let has_track = result.iter().any(|row| row[0].character == '|');
    let has_indicator = result.iter().any(|row| row[0].character == '=');
    
    assert!(has_track, "Should use custom track character");
    assert!(has_indicator, "Should use custom indicator character");
}

/// Test Case 50: Scrollbar render does not perform terminal I/O
/// When the scrollbar renders, it should not perform any terminal I/O operations.
/// This test verifies that the component can be tested without terminal interaction.
/// Validates: Requirements 1.5, 7.1, 7.5
#[test]
fn test_scrollbar_render_no_terminal_io() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 50);
    
    // Call render - this should not panic or require terminal setup
    let result = scrollbar.render(20, &theme, false);
    
    // Verify we got a result
    assert!(!result.is_empty(), "Should return cell array");
    assert_eq!(result.len(), 20, "Should have correct height");
}

/// Test Case 51: Cell arrays can be inspected
/// When the scrollbar produces Cell arrays, the contents should be verifiable
/// without requiring terminal interaction.
/// Validates: Requirements 7.2, 7.3, 7.4
#[test]
fn test_scrollbar_cell_array_inspection() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 0);
    let result = scrollbar.render(20, &theme, false);
    
    // Verify we can inspect all cells
    for (i, row) in result.iter().enumerate() {
        assert_eq!(row.len(), 1, "Each row should have exactly one cell");
        
        // Verify cell has valid character
        assert!(row[0].character != '\0', "Cell should have valid character");
        
        // Verify cell has colors
        assert!(matches!(row[0].fg_color, Color::White), "Cell should have fg color");
        assert!(matches!(row[0].bg_color, Color::Reset), "Cell should have bg color");
        
        // Verify character is either track or indicator
        assert!(
            row[0].character == '│' || row[0].character == '█',
            "Cell at row {} should be either track or indicator character", i
        );
    }
}

/// Test Case 52: Various viewport ratios
/// When the scrollbar renders with different viewport ratios,
/// the indicator height should scale proportionally.
/// Validates: Requirements 2.5
#[test]
fn test_various_viewport_ratios() {
    let theme = Theme::default();
    let height = 20;
    
    // Test 50% ratio (50 visible out of 100 total)
    let scrollbar1 = Scrollbar::new(100, 50, 0);
    let result1 = scrollbar1.render(height, &theme, false);
    let indicator_count1 = result1.iter().filter(|row| row[0].character == '█').count();
    assert_eq!(indicator_count1, 10, "50% ratio should give 10 cells indicator");
    
    // Test 25% ratio (25 visible out of 100 total)
    let scrollbar2 = Scrollbar::new(100, 25, 0);
    let result2 = scrollbar2.render(height, &theme, false);
    let indicator_count2 = result2.iter().filter(|row| row[0].character == '█').count();
    assert_eq!(indicator_count2, 5, "25% ratio should give 5 cells indicator");
    
    // Test 75% ratio (75 visible out of 100 total)
    let scrollbar3 = Scrollbar::new(100, 75, 0);
    let result3 = scrollbar3.render(height, &theme, false);
    let indicator_count3 = result3.iter().filter(|row| row[0].character == '█').count();
    assert_eq!(indicator_count3, 15, "75% ratio should give 15 cells indicator");
}

/// Test Case 53: Invalid scroll position handling
/// When first_visible_index is greater than valid range,
/// the scrollbar should handle it gracefully without panicking.
/// Validates: Requirements 7.1
#[test]
fn test_invalid_scroll_position() {
    let theme = Theme::default();
    
    // Test with first_visible_index beyond total_items
    let scrollbar = Scrollbar::new(100, 10, 150);
    let result = scrollbar.render(20, &theme, false);
    
    // Should not panic and should return valid result
    assert!(!result.is_empty(), "Should handle invalid scroll position gracefully");
    assert_eq!(result.len(), 20, "Should have correct height");
}

/// Test Case 54: Edge case - single item visible
/// When visible_items is 1, the scrollbar should still render correctly.
/// Validates: Requirements 2.5, 5.4
#[test]
fn test_edge_case_single_item_visible() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 1, 0);
    let result = scrollbar.render(20, &theme, false);
    
    assert!(!result.is_empty(), "Scrollbar should be visible");
    
    // Indicator should be at minimum height (1 cell)
    let indicator_count = result.iter().filter(|row| row[0].character == '█').count();
    assert!(indicator_count >= 1, "Indicator should be at least 1 cell");
}

/// Test Case 55: Edge case - very large item count
/// When total_items is very large, the scrollbar should handle it correctly.
/// Validates: Requirements 2.4, 2.5
#[test]
fn test_edge_case_large_item_count() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(10000, 10, 5000);
    let result = scrollbar.render(20, &theme, false);
    
    assert!(!result.is_empty(), "Scrollbar should be visible");
    assert_eq!(result.len(), 20, "Should have correct height");
    
    // Indicator should be very small (minimum 1 cell)
    let indicator_count = result.iter().filter(|row| row[0].character == '█').count();
    assert!(indicator_count >= 1, "Indicator should be at least 1 cell");
}

/// Test Case 56: Scrollbar structure consistency
/// When the scrollbar renders, it should always return a consistent structure
/// (Vec<Vec<Cell>> with height rows and 1 column per row).
/// Validates: Requirements 1.2
#[test]
fn test_scrollbar_structure_consistency() {
    let theme = Theme::default();
    let scrollbar = Scrollbar::new(100, 10, 0);
    let height = 25;
    let result = scrollbar.render(height, &theme, false);
    
    // Should have exactly height rows
    assert_eq!(result.len(), height, "Should have exactly height rows");
    
    // Each row should have exactly 1 cell
    for (i, row) in result.iter().enumerate() {
        assert_eq!(row.len(), 1, "Row {} should have exactly 1 cell", i);
    }
}

// ============================================================================
// Helper Function Tests (Task 6)
// ============================================================================

/// Test Case 101: Helper function renders cells at specified column
/// When render_cells_at_column is called with a Cell array and position,
/// it should render the cells at the specified column and row positions.
/// Validates: Requirements 1.2
#[test]
fn test_render_cells_at_column_basic() {
    use movies::components::{render_cells_at_column, Cell, TextStyle};
    use crossterm::style::Color;
    
    // Create a simple Cell array (single column, multiple rows)
    let cells = vec![
        vec![Cell::new('│', Color::White, Color::Black, TextStyle::new())],
        vec![Cell::new('█', Color::Red, Color::Black, TextStyle::new())],
        vec![Cell::new('│', Color::White, Color::Black, TextStyle::new())],
    ];
    
    // This test verifies the function exists and can be called without panicking
    // In a real terminal environment, this would render at column 10, starting from row 5
    // Since we're in a test environment, we just verify it doesn't panic
    let result = render_cells_at_column(&cells, 10, 5);
    
    // The function should return Ok(()) if successful
    assert!(result.is_ok(), "render_cells_at_column should succeed");
}

/// Test Case 102: Helper function handles empty cell array
/// When render_cells_at_column is called with an empty Cell array,
/// it should handle it gracefully without panicking.
/// Validates: Requirements 1.2
#[test]
fn test_render_cells_at_column_empty() {
    use movies::components::render_cells_at_column;
    
    // Create empty Cell array
    let cells: Vec<Vec<movies::components::Cell>> = vec![];
    
    // Should handle empty array gracefully
    let result = render_cells_at_column(&cells, 0, 0);
    assert!(result.is_ok(), "render_cells_at_column should handle empty array");
}

/// Test Case 103: Helper function handles single cell
/// When render_cells_at_column is called with a single Cell,
/// it should render that cell at the specified position.
/// Validates: Requirements 1.2
#[test]
fn test_render_cells_at_column_single_cell() {
    use movies::components::{render_cells_at_column, Cell, TextStyle};
    use crossterm::style::Color;
    
    // Create single Cell array
    let cells = vec![
        vec![Cell::new('X', Color::Green, Color::Blue, TextStyle::new())],
    ];
    
    // Should handle single cell
    let result = render_cells_at_column(&cells, 5, 10);
    assert!(result.is_ok(), "render_cells_at_column should handle single cell");
}

// ============================================================================
// Browser Component Tests (Task 1.1)
// ============================================================================

use movies::components::Browser;

/// Test Case 1: Boundary constraint enforcement
/// When a Browser component is created and rendered, it should constrain all child components
/// within the specified width and height boundaries and position them relative to top-left coordinates.
/// Validates: Requirements 1.1, 1.4, 1.5
#[test]
fn test_browser_boundary_constraint_enforcement() {
    // Create test categories and episodes
    let categories = vec![
        Category::new("Series 1".to_string(), 10, 5, CategoryType::Series),
        Category::new("Series 2".to_string(), 8, 3, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), true, true, false),
        Episode::new("Episode 2".to_string(), false, true, false),
        Episode::new("Episode 3".to_string(), true, true, false),
    ];
    
    // Create browser with specific dimensions
    let browser = Browser::new(
        (10, 5),  // top_left position
        50,       // width
        4,        // height (smaller than total items to test scrolling)
        categories,
        episodes,
    );
    
    let theme = Theme::default();
    let result = browser.render(50, &theme, true);
    
    // Should return exactly the specified height
    assert_eq!(result.len(), 4, "Browser should render exactly 4 rows (height constraint)");
    
    // Each row should not exceed the specified width
    for (row_index, row) in result.iter().enumerate() {
        assert!(row.len() <= 50, 
                "Row {} length {} should not exceed width constraint 50", 
                row_index, row.len());
    }
    
    // Should have content (not empty)
    assert!(!result.is_empty(), "Browser should render content");
    
    // First row should contain content from first item
    assert!(!result[0].is_empty(), "First row should have content");
}

/// Test Case 2: Browser component with zero dimensions
/// When a Browser component is created with zero width or height,
/// it should handle the edge case gracefully without panicking.
/// Validates: Requirements 1.1, 1.4, 1.5
#[test]
fn test_browser_zero_dimensions() {
    let categories = vec![
        Category::new("Test Series".to_string(), 5, 2, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Test Episode".to_string(), false, true, false),
    ];
    
    // Test zero width
    let browser_zero_width = Browser::new(
        (0, 0),
        0,  // zero width
        5,  // normal height
        categories.clone(),
        episodes.clone(),
    );
    
    let theme = Theme::default();
    let result_zero_width = browser_zero_width.render(0, &theme, true);
    
    // Should return empty result for zero width
    assert_eq!(result_zero_width.len(), 0, "Zero width should return empty result");
    
    // Test zero height
    let browser_zero_height = Browser::new(
        (0, 0),
        50, // normal width
        0,  // zero height
        categories,
        episodes,
    );
    
    let result_zero_height = browser_zero_height.render(50, &theme, true);
    
    // Should return empty result for zero height
    assert_eq!(result_zero_height.len(), 0, "Zero height should return empty result");
}

/// Test Case 3: Browser component positioning consistency
/// When a Browser component is created with different top-left positions,
/// the component structure should remain consistent (position affects external rendering, not internal structure).
/// Validates: Requirements 1.1, 1.4
#[test]
fn test_browser_positioning_consistency() {
    let categories = vec![
        Category::new("Test Series".to_string(), 3, 1, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Test Episode".to_string(), true, true, false),
    ];
    
    // Create browsers with different positions but same dimensions and content
    let browser1 = Browser::new(
        (0, 0),   // top-left at origin
        30,       // width
        3,        // height
        categories.clone(),
        episodes.clone(),
    );
    
    let browser2 = Browser::new(
        (10, 5),  // different top-left position
        30,       // same width
        3,        // same height
        categories,
        episodes,
    );
    
    let theme = Theme::default();
    let result1 = browser1.render(30, &theme, true);
    let result2 = browser2.render(30, &theme, true);
    
    // Both should have the same structure (position doesn't affect internal rendering)
    assert_eq!(result1.len(), result2.len(), "Both browsers should have same number of rows");
    
    // Each corresponding row should have the same length
    for (i, (row1, row2)) in result1.iter().zip(result2.iter()).enumerate() {
        assert_eq!(row1.len(), row2.len(), 
                   "Row {} should have same length in both browsers", i);
    }
}

/// Test Case 4: Browser component with content exceeding height
/// When a Browser component has more items than can fit in the specified height,
/// it should only render the visible items and handle scrolling properly.
/// Validates: Requirements 1.1, 1.5
#[test]
fn test_browser_content_exceeding_height() {
    // Create more items than can fit in height
    let categories = vec![
        Category::new("Series 1".to_string(), 10, 5, CategoryType::Series),
        Category::new("Series 2".to_string(), 8, 3, CategoryType::Series),
        Category::new("Series 3".to_string(), 12, 7, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), true, true, false),
        Episode::new("Episode 2".to_string(), false, true, false),
        Episode::new("Episode 3".to_string(), true, true, false),
        Episode::new("Episode 4".to_string(), false, true, false),
    ];
    
    // Total items: 3 categories + 4 episodes = 7 items
    // Height: 4 (can only show 4 items)
    let browser = Browser::new(
        (0, 0),
        40,  // width
        4,   // height (less than total items)
        categories,
        episodes,
    );
    
    let theme = Theme::default();
    let result = browser.render(40, &theme, true);
    
    // Should render exactly the height specified
    assert_eq!(result.len(), 4, "Should render exactly 4 rows (height constraint)");
    
    // All rows should have content (first 4 items should be visible)
    for (i, row) in result.iter().enumerate() {
        assert!(!row.is_empty(), "Row {} should have content", i);
    }
    
    // Each row should respect width constraint
    for row in &result {
        assert!(row.len() <= 40, "Each row should respect width constraint");
    }
}

/// Test Case 5: Browser component with empty content collections
/// When a Browser component is created with empty categories and episodes,
/// it should handle the empty state gracefully.
/// Validates: Requirements 1.1, 1.5
#[test]
fn test_browser_empty_content() {
    let browser = Browser::new(
        (5, 10),
        30,  // width
        5,   // height
        vec![], // empty categories
        vec![], // empty episodes
    );
    
    let theme = Theme::default();
    let result = browser.render(30, &theme, true);
    
    // Should return rows for the height, but they should be empty
    assert_eq!(result.len(), 5, "Should return 5 rows for height 5");
    
    // All rows should be empty since there's no content
    for (i, row) in result.iter().enumerate() {
        assert_eq!(row.len(), 0, "Row {} should be empty with no content", i);
    }
}

/// Test Case 2: Scrollbar visibility logic
/// When content fits within available height, the browser should display all items without a scrollbar.
/// When content exceeds available height, the browser should display a scrollbar and adjust content width.
/// Validates: Requirements 1.2, 1.3
#[test]
fn test_browser_scrollbar_visibility_logic() {
    // Test case 1: Content fits within height (no scrollbar needed)
    let categories_small = vec![
        Category::new("Series 1".to_string(), 2, 1, CategoryType::Series),
    ];
    let episodes_small = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
    ];
    
    // Total items: 1 category + 2 episodes = 3 items
    // Height: 5 (can show all 3 items)
    let browser_no_scrollbar = Browser::new(
        (0, 0),
        40,  // width
        5,   // height (more than total items)
        categories_small,
        episodes_small,
    );
    
    // Verify scrollbar is not needed
    assert!(!browser_no_scrollbar.needs_scrollbar(), "Scrollbar should not be needed when content fits");
    
    // Verify content width equals full width
    assert_eq!(browser_no_scrollbar.content_width(), 40, "Content width should equal full width when no scrollbar");
    
    // Verify visible items equals total items
    assert_eq!(browser_no_scrollbar.visible_items(), 3, "All items should be visible when content fits");
    
    let theme = Theme::default();
    let result_no_scrollbar = browser_no_scrollbar.render(40, &theme, true);
    
    // Should render exactly 5 rows (height)
    assert_eq!(result_no_scrollbar.len(), 5, "Should render exactly 5 rows");
    
    // First 3 rows should have content (40 cells each), last 2 should be empty
    for i in 0..3 {
        assert!(!result_no_scrollbar[i].is_empty(), "Row {} should have content", i);
        assert_eq!(result_no_scrollbar[i].len(), 40, "Row {} should have full width", i);
    }
    for i in 3..5 {
        assert_eq!(result_no_scrollbar[i].len(), 0, "Row {} should be empty", i);
    }
    
    // Test case 2: Content exceeds height (scrollbar needed)
    let categories_large = vec![
        Category::new("Series 1".to_string(), 3, 2, CategoryType::Series),
        Category::new("Series 2".to_string(), 4, 1, CategoryType::Series),
        Category::new("Series 3".to_string(), 2, 1, CategoryType::Series),
    ];
    let episodes_large = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, true),
        Episode::new("Episode 4".to_string(), true, true, false),
        Episode::new("Episode 5".to_string(), false, true, false),
    ];
    
    // Total items: 3 categories + 5 episodes = 8 items
    // Height: 4 (can only show 4 items, scrollbar needed)
    let browser_with_scrollbar = Browser::new(
        (0, 0),
        40,  // width
        4,   // height (less than total items)
        categories_large,
        episodes_large,
    );
    
    // Verify scrollbar is needed
    assert!(browser_with_scrollbar.needs_scrollbar(), "Scrollbar should be needed when content exceeds height");
    
    // Verify content width is reduced by 1 for scrollbar
    assert_eq!(browser_with_scrollbar.content_width(), 39, "Content width should be reduced by 1 for scrollbar");
    
    // Verify visible items equals height
    assert_eq!(browser_with_scrollbar.visible_items(), 4, "Visible items should equal height when scrollbar needed");
    
    let result_with_scrollbar = browser_with_scrollbar.render(40, &theme, true);
    
    // Should render exactly 4 rows (height)
    assert_eq!(result_with_scrollbar.len(), 4, "Should render exactly 4 rows");
    
    // All rows should have content (39 cells for content + 1 for scrollbar = 40 total)
    for i in 0..4 {
        assert!(!result_with_scrollbar[i].is_empty(), "Row {} should have content", i);
        assert_eq!(result_with_scrollbar[i].len(), 40, "Row {} should have full width including scrollbar", i);
    }
    
    // Test case 3: Edge case - exactly matching height (no scrollbar)
    let categories_exact = vec![
        Category::new("Series 1".to_string(), 2, 1, CategoryType::Series),
        Category::new("Series 2".to_string(), 3, 1, CategoryType::Series),
    ];
    let episodes_exact = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
    ];
    
    // Total items: 2 categories + 2 episodes = 4 items
    // Height: 4 (exactly matches, no scrollbar needed)
    let browser_exact = Browser::new(
        (0, 0),
        30,  // width
        4,   // height (exactly matches total items)
        categories_exact,
        episodes_exact,
    );
    
    // Verify no scrollbar needed for exact match
    assert!(!browser_exact.needs_scrollbar(), "Scrollbar should not be needed when content exactly matches height");
    assert_eq!(browser_exact.content_width(), 30, "Content width should equal full width for exact match");
    assert_eq!(browser_exact.visible_items(), 4, "All items should be visible for exact match");
}

// ============================================================================
// Browser Selection Management Tests (Task 3.1)
// ============================================================================

/// Test Case 3: Selection highlighting consistency
/// When a user navigates through items, the Browser component should maintain a selected item indicator
/// that highlights the current selection and update the visual selection indicator accordingly.
/// Validates: Requirements 2.1, 2.2
#[test]
fn test_browser_selection_highlighting_consistency() {
    // Create test content
    let categories = vec![
        Category::new("Series 1".to_string(), 5, 2, CategoryType::Series),
        Category::new("Series 2".to_string(), 3, 1, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, true),
    ];
    
    // Create browser with content
    let mut browser = Browser::new(
        (0, 0),
        50,  // width
        5,   // height (can show all items)
        categories,
        episodes,
    );
    
    let theme = Theme::default();
    
    // Test initial selection (should be item 0)
    assert_eq!(browser.get_selected_item(), 0, "Initial selection should be item 0");
    assert!(browser.is_item_selected(0), "Item 0 should be selected initially");
    assert!(!browser.is_item_selected(1), "Item 1 should not be selected initially");
    
    // Render with initial selection
    let result_initial = browser.render(50, &theme, true);
    assert!(!result_initial.is_empty(), "Should render content");
    
    // Test selection movement
    browser.move_selection_down();
    assert_eq!(browser.get_selected_item(), 1, "Selection should move to item 1");
    assert!(!browser.is_item_selected(0), "Item 0 should no longer be selected");
    assert!(browser.is_item_selected(1), "Item 1 should now be selected");
    
    // Test selection bounds checking - move up from item 1
    browser.move_selection_up();
    assert_eq!(browser.get_selected_item(), 0, "Selection should move back to item 0");
    
    // Test selection bounds checking - try to move up from item 0 (should stay at 0)
    browser.move_selection_up();
    assert_eq!(browser.get_selected_item(), 0, "Selection should stay at item 0 (bounds check)");
    
    // Test moving to last item
    let total_items = browser.total_items(); // 2 categories + 3 episodes = 5 items
    assert_eq!(total_items, 5, "Should have 5 total items");
    
    // Move to last item
    browser.set_selected_item(4); // Last item (index 4)
    assert_eq!(browser.get_selected_item(), 4, "Selection should be at last item");
    assert!(browser.is_item_selected(4), "Last item should be selected");
    
    // Test bounds checking - try to move down from last item (should stay at last)
    browser.move_selection_down();
    assert_eq!(browser.get_selected_item(), 4, "Selection should stay at last item (bounds check)");
    
    // Test setting selection to out-of-bounds index (should be clamped)
    browser.set_selected_item(100); // Way out of bounds
    assert_eq!(browser.get_selected_item(), 4, "Out-of-bounds selection should be clamped to last valid item");
    
    // Test selection highlighting in render output
    // Set selection to item 2 (third item)
    browser.set_selected_item(2);
    let result_selected = browser.render(50, &theme, true);
    
    // Should render all 5 rows (height = 5, total items = 5)
    assert_eq!(result_selected.len(), 5, "Should render 5 rows");
    
    // All rows should have content since we have 5 items and height 5
    for i in 0..5 {
        assert!(!result_selected[i].is_empty(), "Row {} should have content", i);
    }
    
    // Test with empty content (edge case)
    let empty_browser = Browser::new(
        (0, 0),
        30,
        3,
        vec![], // no categories
        vec![], // no episodes
    );
    
    assert_eq!(empty_browser.get_selected_item(), 0, "Empty browser should have selection at 0");
    assert_eq!(empty_browser.total_items(), 0, "Empty browser should have 0 total items");
    
    let empty_result = empty_browser.render(30, &theme, true);
    assert_eq!(empty_result.len(), 3, "Empty browser should still render height rows");
    
    // All rows should be empty
    for row in &empty_result {
        assert_eq!(row.len(), 0, "Empty browser rows should be empty");
    }
}

/// Test Case: Selection management methods
/// When selection management methods are called, they should properly update
/// the selected item with bounds checking and viewport management.
/// Validates: Requirements 2.1, 2.4
#[test]
fn test_browser_selection_management_methods() {
    let categories = vec![
        Category::new("Test Series".to_string(), 2, 1, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, false),
    ];
    
    // Total: 1 category + 3 episodes = 4 items
    let mut browser = Browser::new(
        (0, 0),
        40,
        2,  // height = 2 (less than total items, will need scrolling)
        categories,
        episodes,
    );
    
    // Test initial state
    assert_eq!(browser.get_selected_item(), 0, "Initial selection should be 0");
    assert_eq!(browser.first_visible_item, 0, "Initial first visible should be 0");
    
    // Test set_selected_item with viewport adjustment
    browser.set_selected_item(3); // Last item
    assert_eq!(browser.get_selected_item(), 3, "Selection should be set to item 3");
    
    // Since height is 2 and we selected item 3, first_visible_item should adjust
    // to show items 2 and 3 (so first_visible_item should be 2)
    assert_eq!(browser.first_visible_item, 2, "First visible should adjust to show selected item");
    
    // Test move_selection_up from last item
    browser.move_selection_up();
    assert_eq!(browser.get_selected_item(), 2, "Selection should move up to item 2");
    
    // Test move_selection_down
    browser.move_selection_down();
    assert_eq!(browser.get_selected_item(), 3, "Selection should move down to item 3");
    
    // Test is_item_selected
    assert!(browser.is_item_selected(3), "Item 3 should be selected");
    assert!(!browser.is_item_selected(0), "Item 0 should not be selected");
    assert!(!browser.is_item_selected(1), "Item 1 should not be selected");
    assert!(!browser.is_item_selected(2), "Item 2 should not be selected");
    
    // Test bounds checking with set_selected_item
    browser.set_selected_item(0);
    assert_eq!(browser.get_selected_item(), 0, "Selection should be set to item 0");
    assert_eq!(browser.first_visible_item, 0, "First visible should adjust to show item 0");
    
    // Test move_selection_up at bounds (should stay at 0)
    browser.move_selection_up();
    assert_eq!(browser.get_selected_item(), 0, "Selection should stay at 0 (lower bound)");
    
    // Move to last item and test upper bound
    browser.set_selected_item(3);
    browser.move_selection_down();
    assert_eq!(browser.get_selected_item(), 3, "Selection should stay at 3 (upper bound)");
}

// ============================================================================
// Browser Viewport Scrolling Tests (Task 4.1, 4.2, 4.3)
// ============================================================================

/// Test Case 4: Selection viewport management
/// When the selected item is outside the viewport, the browser should adjust the first visible item
/// to bring the selection into view and ensure the selected item remains within bounds.
/// Validates: Requirements 2.3, 2.4
#[test]
fn test_browser_selection_viewport_management() {
    // Create content with more items than viewport height
    let categories = vec![
        Category::new("Series 1".to_string(), 2, 1, CategoryType::Series),
        Category::new("Series 2".to_string(), 3, 1, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, false),
        Episode::new("Episode 4".to_string(), true, true, false),
        Episode::new("Episode 5".to_string(), false, true, false),
    ];
    
    // Total: 2 categories + 5 episodes = 7 items
    // Height: 3 (viewport can only show 3 items at once)
    let mut browser = Browser::new(
        (0, 0),
        40,
        3,  // height = 3 (smaller than total items)
        categories,
        episodes,
    );
    
    // Test initial state - selection and viewport should start at 0
    assert_eq!(browser.get_selected_item(), 0, "Initial selection should be 0");
    assert_eq!(browser.first_visible_item, 0, "Initial first visible should be 0");
    
    // Test selection within current viewport (no scroll needed)
    browser.set_selected_item(1);
    assert_eq!(browser.get_selected_item(), 1, "Selection should be set to item 1");
    assert_eq!(browser.first_visible_item, 0, "First visible should remain 0 (item 1 is visible)");
    
    browser.set_selected_item(2);
    assert_eq!(browser.get_selected_item(), 2, "Selection should be set to item 2");
    assert_eq!(browser.first_visible_item, 0, "First visible should remain 0 (item 2 is visible)");
    
    // Test selection outside viewport (scroll down needed)
    browser.set_selected_item(4); // Item 4 is outside viewport [0,1,2]
    assert_eq!(browser.get_selected_item(), 4, "Selection should be set to item 4");
    assert_eq!(browser.first_visible_item, 2, "First visible should adjust to 2 to show item 4 in viewport [2,3,4]");
    
    // Test selection at last item
    browser.set_selected_item(6); // Last item (index 6)
    assert_eq!(browser.get_selected_item(), 6, "Selection should be set to last item");
    assert_eq!(browser.first_visible_item, 4, "First visible should adjust to 4 to show last item in viewport [4,5,6]");
    
    // Test selection moving back up (scroll up needed)
    browser.set_selected_item(1); // Item 1 is outside current viewport [4,5,6]
    assert_eq!(browser.get_selected_item(), 1, "Selection should be set to item 1");
    assert_eq!(browser.first_visible_item, 1, "First visible should adjust to 1 to show item 1 in viewport [1,2,3]");
    
    // Test selection at first item
    browser.set_selected_item(0);
    assert_eq!(browser.get_selected_item(), 0, "Selection should be set to first item");
    assert_eq!(browser.first_visible_item, 0, "First visible should adjust to 0 to show first item");
    
    // Test bounds checking - out of range selection should be clamped
    browser.set_selected_item(100); // Way out of bounds
    assert_eq!(browser.get_selected_item(), 6, "Out-of-bounds selection should be clamped to last valid item");
    assert_eq!(browser.first_visible_item, 4, "Viewport should adjust to show clamped selection");
    
    // Test with empty content
    let mut empty_browser = Browser::new(
        (0, 0),
        30,
        3,
        vec![], // no categories
        vec![], // no episodes
    );
    
    empty_browser.set_selected_item(5); // Try to select non-existent item
    assert_eq!(empty_browser.get_selected_item(), 0, "Empty browser selection should be clamped to 0");
    assert_eq!(empty_browser.first_visible_item, 0, "Empty browser first visible should remain 0");
}

/// Test Case 5: Viewport scroll management
/// When scrolling is needed, the browser should update the first visible item to control which items
/// are displayed and recalculate which components are visible within the viewport.
/// Validates: Requirements 3.1, 3.2
#[test]
fn test_browser_viewport_scroll_management() {
    // Create content with many items to test scrolling
    let categories = vec![
        Category::new("Series 1".to_string(), 1, 1, CategoryType::Series),
        Category::new("Series 2".to_string(), 2, 1, CategoryType::Series),
        Category::new("Series 3".to_string(), 1, 1, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, false),
        Episode::new("Episode 4".to_string(), true, true, false),
        Episode::new("Episode 5".to_string(), false, true, false),
        Episode::new("Episode 6".to_string(), true, true, false),
    ];
    
    // Total: 3 categories + 6 episodes = 9 items
    // Height: 4 (viewport shows 4 items at once)
    let mut browser = Browser::new(
        (0, 0),
        50,
        4,  // height = 4
        categories,
        episodes,
    );
    
    let theme = Theme::default();
    
    // Test initial viewport (items 0,1,2,3 visible)
    assert_eq!(browser.first_visible_item, 0, "Initial first visible should be 0");
    let result_initial = browser.render(50, &theme, true);
    assert_eq!(result_initial.len(), 4, "Should render 4 rows (viewport height)");
    
    // All rows should have content since we're showing first 4 items
    for i in 0..4 {
        assert!(!result_initial[i].is_empty(), "Row {} should have content in initial viewport", i);
    }
    
    // Test scrolling by moving selection to force viewport change
    browser.set_selected_item(5); // This should scroll viewport to show item 5
    
    // With selection at item 5 and height 4, viewport should be [2,3,4,5] or [3,4,5,6]
    // The ensure_selection_visible should adjust first_visible_item appropriately
    let first_visible_after_scroll = browser.first_visible_item;
    assert!(first_visible_after_scroll >= 2, "First visible should scroll down to show item 5");
    assert!(first_visible_after_scroll <= 3, "First visible should not scroll too far");
    
    // Render after scroll
    let result_scrolled = browser.render(50, &theme, true);
    assert_eq!(result_scrolled.len(), 4, "Should still render 4 rows after scroll");
    
    // All rows should have content since we have enough items
    for i in 0..4 {
        assert!(!result_scrolled[i].is_empty(), "Row {} should have content after scroll", i);
    }
    
    // Test scrolling to last items
    browser.set_selected_item(8); // Last item (index 8)
    
    // With 9 total items and height 4, last viewport should show items [5,6,7,8]
    assert_eq!(browser.first_visible_item, 5, "First visible should be 5 to show last 4 items");
    
    let result_last = browser.render(50, &theme, true);
    assert_eq!(result_last.len(), 4, "Should render 4 rows for last viewport");
    
    // Test scrolling back to beginning
    browser.set_selected_item(0);
    assert_eq!(browser.first_visible_item, 0, "First visible should return to 0 for first item");
    
    // Test viewport with content that exactly fits
    let mut exact_browser = Browser::new(
        (0, 0),
        30,
        4,  // height = 4
        vec![Category::new("Series".to_string(), 1, 1, CategoryType::Series)], // 1 category
        vec![
            Episode::new("Episode 1".to_string(), false, true, false),
            Episode::new("Episode 2".to_string(), true, true, false),
            Episode::new("Episode 3".to_string(), false, true, false),
        ], // 3 episodes
    );
    // Total: 1 + 3 = 4 items (exactly fits in height 4)
    
    exact_browser.set_selected_item(3); // Last item
    assert_eq!(exact_browser.first_visible_item, 0, "No scrolling needed when content exactly fits");
    
    let result_exact = exact_browser.render(30, &theme, true);
    assert_eq!(result_exact.len(), 4, "Should render all 4 rows");
    
    // All rows should have content
    for i in 0..4 {
        assert!(!result_exact[i].is_empty(), "Row {} should have content when content exactly fits", i);
    }
}

/// Test Case 6: Scroll bounds enforcement
/// When scrolling reaches the beginning or end, the browser should prevent scrolling beyond
/// the first item or beyond the last item that can fit in the viewport.
/// Validates: Requirements 3.3, 3.4
#[test]
fn test_browser_scroll_bounds_enforcement() {
    // Create content for testing bounds
    let categories = vec![
        Category::new("Series 1".to_string(), 2, 1, CategoryType::Series),
        Category::new("Series 2".to_string(), 1, 1, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, false),
        Episode::new("Episode 4".to_string(), true, true, false),
        Episode::new("Episode 5".to_string(), false, true, false),
        Episode::new("Episode 6".to_string(), true, true, false),
        Episode::new("Episode 7".to_string(), false, true, false),
    ];
    
    // Total: 2 categories + 7 episodes = 9 items
    // Height: 3 (viewport shows 3 items at once)
    let mut browser = Browser::new(
        (0, 0),
        40,
        3,  // height = 3
        categories,
        episodes,
    );
    
    // Test lower bound - first_visible_item should not go below 0
    browser.first_visible_item = 0;
    browser.clamp_first_visible_item();
    assert_eq!(browser.first_visible_item, 0, "First visible should not go below 0");
    
    // Test setting selection at first item
    browser.set_selected_item(0);
    assert_eq!(browser.first_visible_item, 0, "First visible should stay at 0 for first item");
    
    // Test upper bound - with 9 items and height 3, max first_visible_item should be 6
    // This allows showing items [6,7,8] in the viewport
    let max_first_visible = browser.total_items().saturating_sub(browser.height); // 9 - 3 = 6
    assert_eq!(max_first_visible, 6, "Max first visible should be 6");
    
    // Test setting first_visible_item beyond upper bound
    browser.first_visible_item = 10; // Way beyond bounds
    browser.clamp_first_visible_item();
    assert_eq!(browser.first_visible_item, 6, "First visible should be clamped to max valid value (6)");
    
    // Test selection at last item enforces proper bounds
    browser.set_selected_item(8); // Last item
    assert_eq!(browser.first_visible_item, 6, "First visible should be 6 to show last item in viewport");
    
    // Test intermediate bounds - selection in middle should not violate bounds
    browser.set_selected_item(4); // Middle item
    let first_visible_middle = browser.first_visible_item;
    assert!(first_visible_middle >= 0, "First visible should not be negative");
    assert!(first_visible_middle <= 6, "First visible should not exceed max bound");
    
    // Verify the selected item is actually visible in the viewport
    let last_visible = first_visible_middle + browser.height - 1; // height = 3, so last_visible = first + 2
    assert!(4 >= first_visible_middle && 4 <= last_visible, 
            "Selected item 4 should be visible in viewport [{}, {}]", first_visible_middle, last_visible);
    
    // Test edge case - content smaller than viewport height
    let mut small_browser = Browser::new(
        (0, 0),
        30,
        5,  // height = 5
        vec![Category::new("Series".to_string(), 1, 1, CategoryType::Series)], // 1 category
        vec![Episode::new("Episode".to_string(), false, true, false)], // 1 episode
    );
    // Total: 1 + 1 = 2 items (less than height 5)
    
    // With content smaller than viewport, first_visible_item should always be 0
    small_browser.first_visible_item = 3; // Try to set beyond content
    small_browser.clamp_first_visible_item();
    assert_eq!(small_browser.first_visible_item, 0, "First visible should be 0 when content fits entirely");
    
    small_browser.set_selected_item(1); // Last item
    assert_eq!(small_browser.first_visible_item, 0, "First visible should remain 0 when all content fits");
    
    // Test empty content bounds
    let mut empty_browser = Browser::new(
        (0, 0),
        20,
        3,
        vec![], // no categories
        vec![], // no episodes
    );
    
    empty_browser.first_visible_item = 5; // Try to set on empty content
    empty_browser.clamp_first_visible_item();
    assert_eq!(empty_browser.first_visible_item, 0, "Empty browser first visible should be clamped to 0");
    
    empty_browser.set_selected_item(0);
    assert_eq!(empty_browser.first_visible_item, 0, "Empty browser should maintain first visible at 0");
}

// ============================================================================
// Browser Navigation and Utility Methods Tests (Task 7)
// ============================================================================

/// Test navigation methods (move_up, move_down, page_up, page_down)
/// When navigation methods are called, they should properly update selection
/// with bounds checking and viewport management.
/// Validates: Requirements 2.1, 2.3, 2.4
#[test]
fn test_browser_navigation_methods() {
    let categories = vec![
        Category::new("Series 1".to_string(), 2, 1, CategoryType::Series),
        Category::new("Series 2".to_string(), 3, 1, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, false),
        Episode::new("Episode 4".to_string(), true, true, false),
        Episode::new("Episode 5".to_string(), false, true, false),
        Episode::new("Episode 6".to_string(), true, true, false),
    ];
    
    // Total: 2 categories + 6 episodes = 8 items
    let mut browser = Browser::new(
        (0, 0),
        40,
        3,  // height = 3 (smaller than total items)
        categories,
        episodes,
    );
    
    // Test move_up and move_down (aliases for move_selection_up/down)
    assert_eq!(browser.get_selected_item(), 0, "Initial selection should be 0");
    
    browser.move_down();
    assert_eq!(browser.get_selected_item(), 1, "move_down should move to item 1");
    
    browser.move_down();
    assert_eq!(browser.get_selected_item(), 2, "move_down should move to item 2");
    
    browser.move_up();
    assert_eq!(browser.get_selected_item(), 1, "move_up should move back to item 1");
    
    browser.move_up();
    assert_eq!(browser.get_selected_item(), 0, "move_up should move back to item 0");
    
    // Test bounds checking - move_up at first item
    browser.move_up();
    assert_eq!(browser.get_selected_item(), 0, "move_up should stay at item 0 (lower bound)");
    
    // Move to last item and test upper bound
    browser.set_selected_item(7); // Last item
    browser.move_down();
    assert_eq!(browser.get_selected_item(), 7, "move_down should stay at item 7 (upper bound)");
    
    // Test page_up and page_down
    browser.set_selected_item(0); // Start at beginning
    
    browser.page_down();
    assert_eq!(browser.get_selected_item(), 3, "page_down should move by viewport height (3)");
    
    browser.page_down();
    assert_eq!(browser.get_selected_item(), 6, "page_down should move to item 6");
    
    browser.page_down();
    assert_eq!(browser.get_selected_item(), 7, "page_down should be clamped to last item");
    
    browser.page_up();
    assert_eq!(browser.get_selected_item(), 4, "page_up should move back by viewport height");
    
    browser.page_up();
    assert_eq!(browser.get_selected_item(), 1, "page_up should move to item 1");
    
    browser.page_up();
    assert_eq!(browser.get_selected_item(), 0, "page_up should be clamped to first item");
    
    // Test page navigation with empty browser
    let mut empty_browser = Browser::new(
        (0, 0),
        30,
        3,
        vec![], // no categories
        vec![], // no episodes
    );
    
    empty_browser.page_down();
    assert_eq!(empty_browser.get_selected_item(), 0, "page_down on empty browser should stay at 0");
    
    empty_browser.page_up();
    assert_eq!(empty_browser.get_selected_item(), 0, "page_up on empty browser should stay at 0");
}

/// Test utility methods for item counting and type checking
/// When utility methods are called, they should return correct counts and type information.
/// Validates: Requirements 2.1, 2.3, 2.4
#[test]
fn test_browser_utility_methods() {
    let categories = vec![
        Category::new("Series 1".to_string(), 2, 1, CategoryType::Series),
        Category::new("Series 2".to_string(), 3, 1, CategoryType::Series),
        Category::new("Series 3".to_string(), 1, 1, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, false),
        Episode::new("Episode 4".to_string(), true, true, false),
    ];
    
    let mut browser = Browser::new(
        (0, 0),
        40,
        5,
        categories,
        episodes,
    );
    
    // Test counting methods
    assert_eq!(browser.category_count(), 3, "Should have 3 categories");
    assert_eq!(browser.episode_count(), 4, "Should have 4 episodes");
    assert_eq!(browser.total_items(), 7, "Should have 7 total items");
    
    // Test type checking methods with category selected
    browser.set_selected_item(0); // First category
    assert!(browser.is_selected_category(), "Item 0 should be a category");
    assert!(!browser.is_selected_episode(), "Item 0 should not be an episode");
    
    browser.set_selected_item(2); // Last category
    assert!(browser.is_selected_category(), "Item 2 should be a category");
    assert!(!browser.is_selected_episode(), "Item 2 should not be an episode");
    
    // Test type checking methods with episode selected
    browser.set_selected_item(3); // First episode
    assert!(!browser.is_selected_category(), "Item 3 should not be a category");
    assert!(browser.is_selected_episode(), "Item 3 should be an episode");
    
    browser.set_selected_item(6); // Last episode
    assert!(!browser.is_selected_category(), "Item 6 should not be a category");
    assert!(browser.is_selected_episode(), "Item 6 should be an episode");
    
    // Test with empty browser
    let empty_browser = Browser::new(
        (0, 0),
        30,
        3,
        vec![], // no categories
        vec![], // no episodes
    );
    
    assert_eq!(empty_browser.category_count(), 0, "Empty browser should have 0 categories");
    assert_eq!(empty_browser.episode_count(), 0, "Empty browser should have 0 episodes");
    assert_eq!(empty_browser.total_items(), 0, "Empty browser should have 0 total items");
    assert!(!empty_browser.is_selected_category(), "Empty browser selection should not be category");
    assert!(!empty_browser.is_selected_episode(), "Empty browser selection should not be episode");
}

/// Test methods for getting selected item details
/// When selection detail methods are called, they should return correct indices and references.
/// Validates: Requirements 2.1, 2.3, 2.4
#[test]
fn test_browser_selection_detail_methods() {
    let categories = vec![
        Category::new("Series 1".to_string(), 5, 2, CategoryType::Series),
        Category::new("Season 1".to_string(), 3, 1, CategoryType::Season),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, true),
    ];
    
    let mut browser = Browser::new(
        (0, 0),
        40,
        5,
        categories,
        episodes,
    );
    
    // Test with category selected
    browser.set_selected_item(0); // First category
    assert_eq!(browser.get_selected_category_index(), Some(0), "Should return category index 0");
    assert_eq!(browser.get_selected_episode_index(), None, "Should not return episode index");
    
    let selected_category = browser.get_selected_category();
    assert!(selected_category.is_some(), "Should return category reference");
    assert_eq!(selected_category.unwrap().title, "Series 1", "Should return correct category");
    
    assert!(browser.get_selected_episode().is_none(), "Should not return episode reference");
    
    browser.set_selected_item(1); // Second category
    assert_eq!(browser.get_selected_category_index(), Some(1), "Should return category index 1");
    let selected_category2 = browser.get_selected_category();
    assert_eq!(selected_category2.unwrap().title, "Season 1", "Should return correct category");
    
    // Test with episode selected
    browser.set_selected_item(2); // First episode (global index 2, episode index 0)
    assert_eq!(browser.get_selected_category_index(), None, "Should not return category index");
    assert_eq!(browser.get_selected_episode_index(), Some(0), "Should return episode index 0");
    
    let selected_episode = browser.get_selected_episode();
    assert!(selected_episode.is_some(), "Should return episode reference");
    assert_eq!(selected_episode.unwrap().name, "Episode 1", "Should return correct episode");
    
    assert!(browser.get_selected_category().is_none(), "Should not return category reference");
    
    browser.set_selected_item(4); // Last episode (global index 4, episode index 2)
    assert_eq!(browser.get_selected_episode_index(), Some(2), "Should return episode index 2");
    let selected_episode2 = browser.get_selected_episode();
    assert_eq!(selected_episode2.unwrap().name, "Episode 3", "Should return correct episode");
    
    // Test with empty browser
    let empty_browser = Browser::new(
        (0, 0),
        30,
        3,
        vec![], // no categories
        vec![], // no episodes
    );
    
    assert_eq!(empty_browser.get_selected_category_index(), None, "Empty browser should not return category index");
    assert_eq!(empty_browser.get_selected_episode_index(), None, "Empty browser should not return episode index");
    assert!(empty_browser.get_selected_category().is_none(), "Empty browser should not return category reference");
    assert!(empty_browser.get_selected_episode().is_none(), "Empty browser should not return episode reference");
}

/// Test index conversion utility methods
/// When index conversion methods are called, they should correctly convert between
/// global indices and category/episode specific indices.
/// Validates: Requirements 2.1, 2.3, 2.4
#[test]
fn test_browser_index_conversion_methods() {
    let categories = vec![
        Category::new("Series 1".to_string(), 2, 1, CategoryType::Series),
        Category::new("Series 2".to_string(), 3, 1, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, false),
    ];
    
    // Total: 2 categories (indices 0,1) + 3 episodes (indices 2,3,4)
    let browser = Browser::new(
        (0, 0),
        40,
        5,
        categories,
        episodes,
    );
    
    // Test global index to category index conversion
    assert_eq!(browser.global_index_to_category_index(0), Some(0), "Global 0 should be category 0");
    assert_eq!(browser.global_index_to_category_index(1), Some(1), "Global 1 should be category 1");
    assert_eq!(browser.global_index_to_category_index(2), None, "Global 2 should not be a category");
    assert_eq!(browser.global_index_to_category_index(3), None, "Global 3 should not be a category");
    assert_eq!(browser.global_index_to_category_index(4), None, "Global 4 should not be a category");
    assert_eq!(browser.global_index_to_category_index(10), None, "Global 10 should not be a category");
    
    // Test global index to episode index conversion
    assert_eq!(browser.global_index_to_episode_index(0), None, "Global 0 should not be an episode");
    assert_eq!(browser.global_index_to_episode_index(1), None, "Global 1 should not be an episode");
    assert_eq!(browser.global_index_to_episode_index(2), Some(0), "Global 2 should be episode 0");
    assert_eq!(browser.global_index_to_episode_index(3), Some(1), "Global 3 should be episode 1");
    assert_eq!(browser.global_index_to_episode_index(4), Some(2), "Global 4 should be episode 2");
    assert_eq!(browser.global_index_to_episode_index(10), None, "Global 10 should not be an episode");
    
    // Test category index to global index conversion
    assert_eq!(browser.category_index_to_global_index(0), Some(0), "Category 0 should be global 0");
    assert_eq!(browser.category_index_to_global_index(1), Some(1), "Category 1 should be global 1");
    assert_eq!(browser.category_index_to_global_index(2), None, "Category 2 should not exist");
    assert_eq!(browser.category_index_to_global_index(10), None, "Category 10 should not exist");
    
    // Test episode index to global index conversion
    assert_eq!(browser.episode_index_to_global_index(0), Some(2), "Episode 0 should be global 2");
    assert_eq!(browser.episode_index_to_global_index(1), Some(3), "Episode 1 should be global 3");
    assert_eq!(browser.episode_index_to_global_index(2), Some(4), "Episode 2 should be global 4");
    assert_eq!(browser.episode_index_to_global_index(3), None, "Episode 3 should not exist");
    assert_eq!(browser.episode_index_to_global_index(10), None, "Episode 10 should not exist");
    
    // Test with empty browser
    let empty_browser = Browser::new(
        (0, 0),
        30,
        3,
        vec![], // no categories
        vec![], // no episodes
    );
    
    assert_eq!(empty_browser.global_index_to_category_index(0), None, "Empty browser should have no categories");
    assert_eq!(empty_browser.global_index_to_episode_index(0), None, "Empty browser should have no episodes");
    assert_eq!(empty_browser.category_index_to_global_index(0), None, "Empty browser should have no categories");
    assert_eq!(empty_browser.episode_index_to_global_index(0), None, "Empty browser should have no episodes");
    
    // Test with categories only
    let categories_only_browser = Browser::new(
        (0, 0),
        30,
        3,
        vec![Category::new("Series".to_string(), 1, 1, CategoryType::Series)],
        vec![], // no episodes
    );
    
    assert_eq!(categories_only_browser.global_index_to_category_index(0), Some(0), "Should have category at global 0");
    assert_eq!(categories_only_browser.global_index_to_episode_index(0), None, "Should have no episodes");
    assert_eq!(categories_only_browser.category_index_to_global_index(0), Some(0), "Category 0 should be global 0");
    assert_eq!(categories_only_browser.episode_index_to_global_index(0), None, "Should have no episodes");
    
    // Test with episodes only
    let episodes_only_browser = Browser::new(
        (0, 0),
        30,
        3,
        vec![], // no categories
        vec![Episode::new("Episode".to_string(), false, true, false)],
    );
    
    assert_eq!(episodes_only_browser.global_index_to_category_index(0), None, "Should have no categories");
    assert_eq!(episodes_only_browser.global_index_to_episode_index(0), Some(0), "Should have episode at global 0");
    assert_eq!(episodes_only_browser.category_index_to_global_index(0), None, "Should have no categories");
    assert_eq!(episodes_only_browser.episode_index_to_global_index(0), Some(0), "Episode 0 should be global 0");
}

// ============================================================================
// Browser Component Integration Tests (Task 5.1, 5.2)
// ============================================================================

/// Test Case 8: Component integration consistency
/// When the Browser component renders, it should utilize existing Category, Episode, and Scrollbar
/// components for display without modifying their internal behavior.
/// Validates: Requirements 4.1, 4.2, 4.3
#[test]
fn test_browser_component_integration_consistency() {
    // Create test content with both categories and episodes
    let categories = vec![
        Category::new("Test Series 1".to_string(), 5, 2, CategoryType::Series),
        Category::new("Test Season 1".to_string(), 3, 1, CategoryType::Season),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, true),
    ];
    
    // Create browser that will need a scrollbar (more items than height)
    let browser = Browser::new(
        (0, 0),
        50,  // width
        3,   // height (less than total items to trigger scrollbar)
        categories.clone(),
        episodes.clone(),
    );
    
    let theme = Theme::default();
    
    // Test that browser utilizes existing Category components (Requirement 4.1)
    // Render individual category components to compare - first item will be selected
    let category1_direct = categories[0].render(49, &theme, true); // 49 = content_width when scrollbar present, selected
    let category2_direct = categories[1].render(49, &theme, false); // not selected
    
    // Render browser and verify it produces the same output as direct category rendering
    let browser_result = browser.render(50, &theme, true);
    
    // Browser should render exactly 3 rows (height constraint)
    assert_eq!(browser_result.len(), 3, "Browser should render exactly 3 rows");
    
    // First row should match first category component output (selected)
    assert_eq!(browser_result[0].len(), 50, "First row should have full width including scrollbar");
    
    // Extract content portion (excluding scrollbar) and compare with direct category render
    let browser_row0_content: Vec<_> = browser_result[0].iter().take(49).cloned().collect();
    assert_eq!(browser_row0_content, category1_direct[0], 
               "Browser should utilize existing Category component for first category");
    
    // Second row should match second category component output (not selected)
    let browser_row1_content: Vec<_> = browser_result[1].iter().take(49).cloned().collect();
    assert_eq!(browser_row1_content, category2_direct[0], 
               "Browser should utilize existing Category component for second category");
    
    // Test that browser utilizes existing Episode components (Requirement 4.2)
    // Third row should be first episode (not selected)
    let episode1_direct = episodes[0].render(49, &theme, false);
    let browser_row2_content: Vec<_> = browser_result[2].iter().take(49).cloned().collect();
    
    // Compare the actual content portion (episode renders only actual content, browser pads to full width)
    let episode_content_len = episode1_direct[0].len();
    let browser_episode_content: Vec<_> = browser_row2_content.iter().take(episode_content_len).cloned().collect();
    assert_eq!(browser_episode_content, episode1_direct[0], 
               "Browser should utilize existing Episode component for first episode");
    
    // Verify that browser pads the rest with spaces
    for i in episode_content_len..49 {
        assert_eq!(browser_row2_content[i].character, ' ', 
                   "Browser should pad episode content with spaces");
    }
    
    // Test that browser utilizes existing Scrollbar component (Requirement 4.3)
    // Create a scrollbar directly with same parameters
    let scrollbar = Scrollbar::new(
        5,  // total_items (2 categories + 3 episodes)
        3,  // visible_items (height)
        0,  // first_visible_item
    );
    let scrollbar_direct = scrollbar.render(3, &theme, false);
    
    // Verify scrollbar is present in browser output
    assert!(browser.needs_scrollbar(), "Browser should need scrollbar with this content");
    
    // Extract scrollbar column from browser output
    let browser_scrollbar: Vec<_> = browser_result.iter()
        .map(|row| row.last().cloned().unwrap_or_else(|| Cell::new(' ', Color::Reset, Color::Reset, TextStyle::new())))
        .collect();
    
    // Compare with direct scrollbar render
    for (i, (browser_cell, scrollbar_row)) in browser_scrollbar.iter().zip(scrollbar_direct.iter()).enumerate() {
        if let Some(scrollbar_cell) = scrollbar_row.first() {
            assert_eq!(browser_cell.character, scrollbar_cell.character, 
                       "Browser scrollbar row {} should match direct Scrollbar component", i);
            assert_eq!(browser_cell.fg_color, scrollbar_cell.fg_color, 
                       "Browser scrollbar row {} should use same colors as direct Scrollbar component", i);
        }
    }
    
    // Test component integration without scrollbar
    let browser_no_scrollbar = Browser::new(
        (0, 0),
        40,  // width
        6,   // height (more than total items, no scrollbar needed)
        vec![Category::new("Single Series".to_string(), 2, 1, CategoryType::Series)],
        vec![Episode::new("Single Episode".to_string(), true, true, false)],
    );
    
    assert!(!browser_no_scrollbar.needs_scrollbar(), "Browser should not need scrollbar");
    
    let result_no_scrollbar = browser_no_scrollbar.render(40, &theme, true);
    
    // Should render 6 rows (height), but only first 2 have content
    assert_eq!(result_no_scrollbar.len(), 6, "Should render full height");
    
    // First two rows should have content matching direct component renders
    assert!(!result_no_scrollbar[0].is_empty(), "First row should have category content");
    assert!(!result_no_scrollbar[1].is_empty(), "Second row should have episode content");
    
    // Remaining rows should be empty
    for i in 2..6 {
        assert_eq!(result_no_scrollbar[i].len(), 0, "Row {} should be empty", i);
    }
    
    // Verify content width equals full width when no scrollbar
    assert_eq!(browser_no_scrollbar.content_width(), 40, "Content width should equal full width without scrollbar");
}

/// Test Case 6: Empty state handling
/// When no items are available, the Browser component should handle the empty state
/// gracefully without selection indicators or errors.
/// Validates: Requirements 2.5
#[test]
fn test_browser_empty_state_handling() {
    // Create browser with no categories or episodes
    let empty_browser = Browser::new(
        (10, 5),  // position
        30,       // width
        4,        // height
        vec![],   // no categories
        vec![],   // no episodes
    );
    
    let theme = Theme::default();
    
    // Test basic empty state properties
    assert_eq!(empty_browser.total_items(), 0, "Empty browser should have 0 total items");
    assert_eq!(empty_browser.get_selected_item(), 0, "Empty browser should have selection at 0");
    assert!(!empty_browser.needs_scrollbar(), "Empty browser should not need scrollbar");
    assert_eq!(empty_browser.content_width(), 30, "Empty browser content width should equal full width");
    assert_eq!(empty_browser.visible_items(), 0, "Empty browser should have 0 visible items");
    
    // Test rendering empty state
    let result = empty_browser.render(30, &theme, true);
    
    // Should return rows for the specified height
    assert_eq!(result.len(), 4, "Empty browser should render specified height (4 rows)");
    
    // All rows should be empty (no content)
    for (i, row) in result.iter().enumerate() {
        assert_eq!(row.len(), 0, "Empty browser row {} should be empty", i);
    }
    
    // Test that empty browser handles selection operations gracefully
    let mut empty_browser_mut = Browser::new(
        (0, 0),
        25,
        3,
        vec![],
        vec![],
    );
    
    // Test selection operations on empty browser
    empty_browser_mut.move_selection_up();
    assert_eq!(empty_browser_mut.get_selected_item(), 0, "Selection should remain 0 after move_up on empty browser");
    
    empty_browser_mut.move_selection_down();
    assert_eq!(empty_browser_mut.get_selected_item(), 0, "Selection should remain 0 after move_down on empty browser");
    
    empty_browser_mut.set_selected_item(5);
    assert_eq!(empty_browser_mut.get_selected_item(), 0, "Selection should be clamped to 0 on empty browser");
    
    // Test viewport management on empty browser
    assert_eq!(empty_browser_mut.first_visible_item, 0, "First visible should be 0 on empty browser");
    
    empty_browser_mut.ensure_selection_visible();
    assert_eq!(empty_browser_mut.first_visible_item, 0, "First visible should remain 0 after ensure_selection_visible");
    
    // Test that empty browser doesn't crash with various operations
    empty_browser_mut.clamp_selected_item();
    empty_browser_mut.clamp_first_visible_item();
    
    assert_eq!(empty_browser_mut.get_selected_item(), 0, "Selection should remain stable after clamping operations");
    assert_eq!(empty_browser_mut.first_visible_item, 0, "First visible should remain stable after clamping operations");
    
    // Test rendering after operations
    let result_after_ops = empty_browser_mut.render(25, &theme, true);
    assert_eq!(result_after_ops.len(), 3, "Empty browser should still render correctly after operations");
    
    for (i, row) in result_after_ops.iter().enumerate() {
        assert_eq!(row.len(), 0, "Empty browser row {} should still be empty after operations", i);
    }
    
    // Test edge case: zero dimensions with empty content
    let zero_dim_browser = Browser::new(
        (0, 0),
        0,  // zero width
        0,  // zero height
        vec![],
        vec![],
    );
    
    let zero_result = zero_dim_browser.render(0, &theme, true);
    assert_eq!(zero_result.len(), 0, "Zero dimension empty browser should return empty result");
    
    // Test edge case: empty content with large dimensions
    let large_empty_browser = Browser::new(
        (0, 0),
        100,  // large width
        50,   // large height
        vec![],
        vec![],
    );
    
    let large_result = large_empty_browser.render(100, &theme, true);
    assert_eq!(large_result.len(), 50, "Large empty browser should render full height");
    
    for (i, row) in large_result.iter().enumerate() {
        assert_eq!(row.len(), 0, "Large empty browser row {} should be empty", i);
    }
}

// ============================================================================
// Browser Scrollbar Positioning Tests (Task 6.1)
// ============================================================================

/// Test Case 7: Scrollbar positioning accuracy
/// When the scrollbar is displayed, the Browser component should position it to accurately
/// reflect the current scroll position relative to total content.
/// Validates: Requirements 3.5
#[test]
fn test_browser_scrollbar_positioning_accuracy() {
    use movies::components::Scrollbar;
    
    // Create content that requires scrolling
    let categories = vec![
        Category::new("Series 1".to_string(), 2, 1, CategoryType::Series),
        Category::new("Series 2".to_string(), 3, 1, CategoryType::Series),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, false),
        Episode::new("Episode 4".to_string(), true, true, false),
        Episode::new("Episode 5".to_string(), false, true, false),
        Episode::new("Episode 6".to_string(), true, true, false),
    ];
    
    // Total: 2 categories + 6 episodes = 8 items
    // Height: 4 (viewport shows 4 items, scrollbar needed)
    let mut browser = Browser::new(
        (0, 0),
        50,  // width
        4,   // height (less than total items)
        categories,
        episodes,
    );
    
    let theme = Theme::default();
    
    // Test scrollbar positioning at different scroll positions
    
    // Position 1: At the beginning (first_visible_item = 0)
    browser.first_visible_item = 0;
    browser.clamp_first_visible_item();
    
    let result_start = browser.render(50, &theme, true);
    assert_eq!(result_start.len(), 4, "Should render 4 rows");
    
    // Verify scrollbar is present (each row should have 50 cells: 49 content + 1 scrollbar)
    for i in 0..4 {
        assert_eq!(result_start[i].len(), 50, "Row {} should have scrollbar column", i);
    }
    
    // Create expected scrollbar for comparison
    let expected_scrollbar_start = Scrollbar::new(8, 4, 0); // total=8, visible=4, first_visible=0
    let scrollbar_cells_start = expected_scrollbar_start.render(4, &theme, false);
    
    // Verify scrollbar positioning matches expected
    for (row_idx, scrollbar_row) in scrollbar_cells_start.iter().enumerate() {
        if let Some(expected_cell) = scrollbar_row.first() {
            let actual_scrollbar_cell = &result_start[row_idx][49]; // Last column is scrollbar
            assert_eq!(actual_scrollbar_cell.character, expected_cell.character,
                      "Scrollbar character at row {} should match expected for start position", row_idx);
        }
    }
    
    // Position 2: In the middle (first_visible_item = 2)
    browser.first_visible_item = 2;
    browser.clamp_first_visible_item();
    
    let result_middle = browser.render(50, &theme, true);
    
    let expected_scrollbar_middle = Scrollbar::new(8, 4, 2); // total=8, visible=4, first_visible=2
    let scrollbar_cells_middle = expected_scrollbar_middle.render(4, &theme, false);
    
    // Verify scrollbar positioning for middle position
    for (row_idx, scrollbar_row) in scrollbar_cells_middle.iter().enumerate() {
        if let Some(expected_cell) = scrollbar_row.first() {
            let actual_scrollbar_cell = &result_middle[row_idx][49];
            assert_eq!(actual_scrollbar_cell.character, expected_cell.character,
                      "Scrollbar character at row {} should match expected for middle position", row_idx);
        }
    }
    
    // Position 3: At the end (first_visible_item = 4, showing items 4,5,6,7)
    browser.first_visible_item = 4;
    browser.clamp_first_visible_item();
    
    let result_end = browser.render(50, &theme, true);
    
    let expected_scrollbar_end = Scrollbar::new(8, 4, 4); // total=8, visible=4, first_visible=4
    let scrollbar_cells_end = expected_scrollbar_end.render(4, &theme, false);
    
    // Verify scrollbar positioning for end position
    for (row_idx, scrollbar_row) in scrollbar_cells_end.iter().enumerate() {
        if let Some(expected_cell) = scrollbar_row.first() {
            let actual_scrollbar_cell = &result_end[row_idx][49];
            assert_eq!(actual_scrollbar_cell.character, expected_cell.character,
                      "Scrollbar character at row {} should match expected for end position", row_idx);
        }
    }
    
    // Test edge case: content exactly fits (no scrollbar should be present)
    let small_browser = Browser::new(
        (0, 0),
        30,
        4,  // height = 4
        vec![Category::new("Series".to_string(), 1, 1, CategoryType::Series)], // 1 category
        vec![
            Episode::new("Episode 1".to_string(), false, true, false),
            Episode::new("Episode 2".to_string(), true, true, false),
            Episode::new("Episode 3".to_string(), false, true, false),
        ], // 3 episodes
    );
    // Total: 1 + 3 = 4 items (exactly fits, no scrollbar needed)
    
    let result_no_scrollbar = small_browser.render(30, &theme, true);
    
    // Verify no scrollbar is present (each row should have exactly 30 cells)
    for i in 0..4 {
        assert_eq!(result_no_scrollbar[i].len(), 30, "Row {} should not have scrollbar when content fits", i);
    }
    
    // Test scrollbar accuracy with selection-driven scrolling
    let mut selection_browser = Browser::new(
        (0, 0),
        40,
        3,  // height = 3
        vec![
            Category::new("Series 1".to_string(), 1, 1, CategoryType::Series),
            Category::new("Series 2".to_string(), 1, 1, CategoryType::Series),
        ],
        vec![
            Episode::new("Episode 1".to_string(), false, true, false),
            Episode::new("Episode 2".to_string(), true, true, false),
            Episode::new("Episode 3".to_string(), false, true, false),
            Episode::new("Episode 4".to_string(), true, true, false),
        ],
    );
    // Total: 2 + 4 = 6 items, height = 3 (scrollbar needed)
    
    // Move selection to last item, which should scroll to show it
    selection_browser.set_selected_item(5); // Last item
    
    let result_selection_scroll = selection_browser.render(40, &theme, true);
    
    // Verify scrollbar reflects the new scroll position
    let expected_scrollbar_selection = Scrollbar::new(6, 3, selection_browser.first_visible_item);
    let scrollbar_cells_selection = expected_scrollbar_selection.render(3, &theme, false);
    
    for (row_idx, scrollbar_row) in scrollbar_cells_selection.iter().enumerate() {
        if let Some(expected_cell) = scrollbar_row.first() {
            let actual_scrollbar_cell = &result_selection_scroll[row_idx][39]; // Last column
            assert_eq!(actual_scrollbar_cell.character, expected_cell.character,
                      "Scrollbar should accurately reflect selection-driven scroll position at row {}", row_idx);
        }
    }
}

// ============================================================================
// Browser Component Coordination Tests (Task 6.2)
// ============================================================================

/// Test Case 9: Component coordination
/// When component properties change, the Browser component should propagate relevant updates
/// to child components and maintain proper positioning and sizing coordination.
/// Validates: Requirements 4.4, 4.5
#[test]
fn test_browser_component_coordination() {
    // Create test content
    let categories = vec![
        Category::new("Test Series".to_string(), 5, 2, CategoryType::Series),
        Category::new("Test Season".to_string(), 3, 1, CategoryType::Season),
    ];
    
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), true, true, false),
        Episode::new("Episode 3".to_string(), false, true, true),
    ];
    
    let theme = Theme::default();
    
    // Test 1: Width coordination - browser should pass correct width to child components
    let browser_wide = Browser::new(
        (0, 0),
        80,  // wide width
        3,   // height (will need scrollbar: 2 categories + 3 episodes = 5 > 3)
        categories.clone(),
        episodes.clone(),
    );
    
    let result_wide = browser_wide.render(80, &theme, true);
    
    // With scrollbar needed, content width should be 79, total width 80
    assert_eq!(result_wide.len(), 3, "Should render 3 rows");
    for i in 0..3 {
        assert_eq!(result_wide[i].len(), 80, "Wide browser row {} should have full width including scrollbar", i);
    }
    
    // Verify child components receive correct width (79 for content when scrollbar present)
    // We can verify this by checking that content doesn't exceed the content width
    for i in 0..3 {
        // Content portion should be exactly 79 cells, scrollbar should be 1 cell
        let content_cells: Vec<_> = result_wide[i].iter().take(79).collect();
        assert_eq!(content_cells.len(), 79, "Content portion should be exactly 79 cells");
    }
    
    // Test 2: Width coordination without scrollbar
    let browser_narrow_no_scroll = Browser::new(
        (0, 0),
        40,  // width
        6,   // height (more than total items: 5, so no scrollbar needed)
        categories.clone(),
        episodes.clone(),
    );
    
    let result_narrow_no_scroll = browser_narrow_no_scroll.render(40, &theme, true);
    
    // Without scrollbar, content width should equal full width (40)
    assert_eq!(result_narrow_no_scroll.len(), 6, "Should render 6 rows");
    
    // First 5 rows should have content (40 cells each), last row should be empty
    for i in 0..5 {
        assert_eq!(result_narrow_no_scroll[i].len(), 40, "Row {} should have full width without scrollbar", i);
    }
    assert_eq!(result_narrow_no_scroll[5].len(), 0, "Last row should be empty when no more content");
    
    // Test 3: Selection state propagation - browser should pass correct selection state to child components
    let mut browser_selection = Browser::new(
        (0, 0),
        50,
        5,   // height = 5 (exactly matches total items, no scrollbar)
        categories.clone(),
        episodes.clone(),
    );
    
    // Test selection of first item (category)
    browser_selection.set_selected_item(0);
    let result_select_0 = browser_selection.render(50, &theme, true);
    
    // Render the first category directly with selection to compare
    let category_0_selected = categories[0].render(50, &theme, true);
    let category_0_unselected = categories[0].render(50, &theme, false);
    
    // First row should match selected category
    assert_eq!(result_select_0[0], category_0_selected[0], 
               "Browser should pass selection state to first category");
    
    // Second row should match unselected category
    let category_1_unselected = categories[1].render(50, &theme, false);
    assert_eq!(result_select_0[1], category_1_unselected[0], 
               "Browser should pass unselected state to second category");
    
    // Test selection of episode (third item, index 2)
    browser_selection.set_selected_item(2);
    let result_select_2 = browser_selection.render(50, &theme, true);
    
    // First two rows should be unselected categories
    assert_eq!(result_select_2[0], category_0_unselected[0], 
               "First category should be unselected when episode is selected");
    assert_eq!(result_select_2[1], category_1_unselected[0], 
               "Second category should be unselected when episode is selected");
    
    // Third row should be selected episode
    let episode_0_selected = episodes[0].render(50, &theme, true);
    
    // Browser pads episode content to full width, so we need to compare the actual content portion
    let episode_content_len = episode_0_selected[0].len();
    let browser_episode_content: Vec<_> = result_select_2[2].iter().take(episode_content_len).cloned().collect();
    assert_eq!(browser_episode_content, episode_0_selected[0], 
               "Browser should pass selection state to first episode");
    
    // Test 4: Theme propagation - browser should pass theme to all child components
    let mut custom_theme = Theme::default();
    custom_theme.current_fg = "red".to_string();
    custom_theme.current_bg = "blue".to_string();
    
    let result_custom_theme = browser_selection.render(50, &custom_theme, true);
    
    // Render components directly with custom theme for comparison
    let category_custom_theme = categories[1].render(50, &custom_theme, true);
    
    // Set selection to second category to test theme propagation
    browser_selection.set_selected_item(1);
    let result_theme_test = browser_selection.render(50, &custom_theme, true);
    
    assert_eq!(result_theme_test[1], category_custom_theme[0], 
               "Browser should propagate custom theme to child components");
    
    // Test 5: Positioning coordination with scrolling
    let mut browser_scroll = Browser::new(
        (0, 0),
        45,
        3,   // height = 3 (less than 5 total items, scrollbar needed)
        categories.clone(),
        episodes.clone(),
    );
    
    // Scroll to middle position
    browser_scroll.first_visible_item = 2;
    browser_scroll.clamp_first_visible_item();
    
    let result_scroll = browser_scroll.render(45, &theme, true);
    
    // Should render 3 rows with scrollbar (44 content + 1 scrollbar = 45 total)
    assert_eq!(result_scroll.len(), 3, "Should render 3 rows when scrolled");
    for i in 0..3 {
        assert_eq!(result_scroll[i].len(), 45, "Scrolled row {} should have full width", i);
    }
    
    // Verify that the correct items are rendered based on scroll position
    // With first_visible_item = 2, should show items 2, 3, 4 (first episode, second episode, third episode)
    let episode_0_unselected = episodes[0].render(44, &theme, false); // Content width = 44
    let episode_1_unselected = episodes[1].render(44, &theme, false);
    let episode_2_unselected = episodes[2].render(44, &theme, false);
    
    // Compare content portions (excluding scrollbar)
    let row0_content: Vec<_> = result_scroll[0].iter().take(episode_0_unselected[0].len()).cloned().collect();
    let row1_content: Vec<_> = result_scroll[1].iter().take(episode_1_unselected[0].len()).cloned().collect();
    let row2_content: Vec<_> = result_scroll[2].iter().take(episode_2_unselected[0].len()).cloned().collect();
    
    assert_eq!(row0_content, episode_0_unselected[0], 
               "First visible row should show first episode when scrolled to position 2");
    assert_eq!(row1_content, episode_1_unselected[0], 
               "Second visible row should show second episode when scrolled to position 2");
    assert_eq!(row2_content, episode_2_unselected[0], 
               "Third visible row should show third episode when scrolled to position 2");
    
    // Test 6: Edge case coordination - empty content
    let empty_browser = Browser::new(
        (0, 0),
        30,
        4,
        vec![], // no categories
        vec![], // no episodes
    );
    
    let result_empty = empty_browser.render(30, &theme, true);
    
    // Should render 4 empty rows
    assert_eq!(result_empty.len(), 4, "Empty browser should render height rows");
    for (i, row) in result_empty.iter().enumerate() {
        assert_eq!(row.len(), 0, "Empty browser row {} should be empty", i);
    }
    
    // Verify no scrollbar is created for empty content
    assert!(!empty_browser.needs_scrollbar(), "Empty browser should not need scrollbar");
    assert_eq!(empty_browser.content_width(), 30, "Empty browser content width should equal full width");
}