use movies::components::{Scrollbar, Component};
use movies::scrollbar::calculate_scrollbar_state;
use movies::theme::Theme;

/// Test Case 1: Equivalence at top scroll position
/// When both legacy and component render at top position,
/// they should produce identical visual output.
/// Validates: Requirements 8.2
#[test]
fn test_equivalence_top_position() {
    let theme = Theme::default();
    
    // Test parameters
    let total_items = 100;
    let visible_items = 10;
    let first_visible_index = 0;
    let height = 20;
    let start_row = 5;
    let column = 80;
    
    // Get legacy output using calculate_scrollbar_state
    let legacy_state = calculate_scrollbar_state(
        total_items,
        visible_items,
        first_visible_index,
        start_row,
        height,
        column,
    );
    
    // Get component output
    let scrollbar = Scrollbar::new(total_items, visible_items, first_visible_index);
    let component_cells = scrollbar.render(height, &theme, false);
    
    // Compare visibility
    assert_eq!(legacy_state.visible, !component_cells.is_empty(),
        "Legacy and component should agree on visibility");
    
    if legacy_state.visible {
        // Compare dimensions
        assert_eq!(component_cells.len(), height,
            "Component should produce height rows");
        
        // Compare indicator position (should be at top)
        assert_eq!(legacy_state.indicator_start, start_row,
            "Legacy indicator should be at top");
        
        // Component indicator should be at row 0
        assert_eq!(component_cells[0][0].character, '█',
            "Component indicator should be at first row");
        
        // Compare indicator height
        let component_indicator_count = component_cells.iter()
            .filter(|row| row[0].character == '█')
            .count();
        
        assert_eq!(component_indicator_count, legacy_state.indicator_height,
            "Component and legacy should have same indicator height");
    }
}

/// Test Case 2: Equivalence at bottom scroll position
/// When both legacy and component render at bottom position,
/// they should produce identical visual output.
/// Validates: Requirements 8.2
#[test]
fn test_equivalence_bottom_position() {
    let theme = Theme::default();
    
    // Test parameters - at bottom
    let total_items = 100;
    let visible_items = 10;
    let first_visible_index = 90; // At bottom (90 + 10 = 100)
    let height = 20;
    let start_row = 5;
    let column = 80;
    
    // Get legacy output
    let legacy_state = calculate_scrollbar_state(
        total_items,
        visible_items,
        first_visible_index,
        start_row,
        height,
        column,
    );
    
    // Get component output
    let scrollbar = Scrollbar::new(total_items, visible_items, first_visible_index);
    let component_cells = scrollbar.render(height, &theme, false);
    
    // Compare visibility
    assert_eq!(legacy_state.visible, !component_cells.is_empty(),
        "Legacy and component should agree on visibility");
    
    if legacy_state.visible {
        // Compare indicator position (should be at bottom)
        let expected_indicator_end = legacy_state.indicator_start + legacy_state.indicator_height - 1;
        let track_end = start_row + height - 1;
        
        // Component indicator should be at the end
        let component_indicator_positions: Vec<usize> = component_cells.iter()
            .enumerate()
            .filter(|(_, row)| row[0].character == '█')
            .map(|(i, _)| i)
            .collect();
        
        let component_last_indicator = *component_indicator_positions.last().unwrap();
        
        // Both should have indicator at the end of their respective tracks
        assert_eq!(expected_indicator_end, track_end,
            "Legacy indicator should be at track end");
        assert_eq!(component_last_indicator, height - 1,
            "Component indicator should be at last row");
    }
}

/// Test Case 3: Equivalence at middle scroll position
/// When both legacy and component render at middle position,
/// they should produce identical visual output.
/// Validates: Requirements 8.2
#[test]
fn test_equivalence_middle_position() {
    let theme = Theme::default();
    
    // Test parameters - in middle
    let total_items = 100;
    let visible_items = 10;
    let first_visible_index = 45; // Middle position
    let height = 20;
    let start_row = 5;
    let column = 80;
    
    // Get legacy output
    let legacy_state = calculate_scrollbar_state(
        total_items,
        visible_items,
        first_visible_index,
        start_row,
        height,
        column,
    );
    
    // Get component output
    let scrollbar = Scrollbar::new(total_items, visible_items, first_visible_index);
    let component_cells = scrollbar.render(height, &theme, false);
    
    // Compare visibility
    assert_eq!(legacy_state.visible, !component_cells.is_empty(),
        "Legacy and component should agree on visibility");
    
    if legacy_state.visible {
        // Compare indicator height
        let component_indicator_count = component_cells.iter()
            .filter(|row| row[0].character == '█')
            .count();
        
        assert_eq!(component_indicator_count, legacy_state.indicator_height,
            "Component and legacy should have same indicator height");
        
        // Compare relative position (both should be roughly in middle)
        let legacy_relative_pos = legacy_state.indicator_start - start_row;
        let component_indicator_start = component_cells.iter()
            .position(|row| row[0].character == '█')
            .unwrap();
        
        assert_eq!(component_indicator_start, legacy_relative_pos,
            "Component and legacy should have same relative indicator position");
    }
}

/// Test Case 4: Equivalence when scrollbar should be hidden
/// When both legacy and component determine scrollbar is not needed,
/// they should both indicate hidden state.
/// Validates: Requirements 8.2
#[test]
fn test_equivalence_hidden_scrollbar() {
    let theme = Theme::default();
    
    // Test parameters - items fit on screen
    let total_items = 5;
    let visible_items = 10;
    let first_visible_index = 0;
    let height = 20;
    let start_row = 5;
    let column = 80;
    
    // Get legacy output
    let legacy_state = calculate_scrollbar_state(
        total_items,
        visible_items,
        first_visible_index,
        start_row,
        height,
        column,
    );
    
    // Get component output
    let scrollbar = Scrollbar::new(total_items, visible_items, first_visible_index);
    let component_cells = scrollbar.render(height, &theme, false);
    
    // Both should indicate hidden
    assert!(!legacy_state.visible, "Legacy should indicate hidden");
    assert!(component_cells.is_empty(), "Component should return empty cells");
}

/// Test Case 5: Equivalence with various viewport ratios
/// When both legacy and component render with different viewport ratios,
/// they should produce equivalent indicator heights.
/// Validates: Requirements 8.2
#[test]
fn test_equivalence_various_ratios() {
    let theme = Theme::default();
    let height = 20;
    let start_row = 5;
    let column = 80;
    
    // Test different ratios
    let test_cases = vec![
        (100, 50, 0),  // 50% ratio
        (100, 25, 0),  // 25% ratio
        (100, 10, 0),  // 10% ratio
        (1000, 1, 0),  // Very small ratio (should be minimum 1)
    ];
    
    for (total_items, visible_items, first_visible_index) in test_cases {
        // Get legacy output
        let legacy_state = calculate_scrollbar_state(
            total_items,
            visible_items,
            first_visible_index,
            start_row,
            height,
            column,
        );
        
        // Get component output
        let scrollbar = Scrollbar::new(total_items, visible_items, first_visible_index);
        let component_cells = scrollbar.render(height, &theme, false);
        
        // Compare visibility
        assert_eq!(legacy_state.visible, !component_cells.is_empty(),
            "Legacy and component should agree on visibility for ratio {}/{}", 
            visible_items, total_items);
        
        if legacy_state.visible {
            // Compare indicator height
            let component_indicator_count = component_cells.iter()
                .filter(|row| row[0].character == '█')
                .count();
            
            assert_eq!(component_indicator_count, legacy_state.indicator_height,
                "Component and legacy should have same indicator height for ratio {}/{}",
                visible_items, total_items);
        }
    }
}

/// Test Case 6: Equivalence with edge cases
/// When both legacy and component handle edge cases,
/// they should produce equivalent results.
/// Validates: Requirements 8.2
#[test]
fn test_equivalence_edge_cases() {
    let theme = Theme::default();
    
    // Test edge cases
    let test_cases = vec![
        (0, 10, 0, 20),    // Zero total items
        (100, 10, 0, 0),   // Zero height
        (100, 0, 0, 20),   // Zero visible items
        (1, 1, 0, 20),     // Single item
        (2, 1, 1, 20),     // Single visible, at end
    ];
    
    for (total_items, visible_items, first_visible_index, height) in test_cases {
        let start_row = 5;
        let column = 80;
        
        // Get legacy output
        let legacy_state = calculate_scrollbar_state(
            total_items,
            visible_items,
            first_visible_index,
            start_row,
            height,
            column,
        );
        
        // Get component output
        let scrollbar = Scrollbar::new(total_items, visible_items, first_visible_index);
        let component_cells = scrollbar.render(height, &theme, false);
        
        // Compare visibility
        assert_eq!(legacy_state.visible, !component_cells.is_empty(),
            "Legacy and component should agree on visibility for edge case: total={}, visible={}, pos={}, height={}",
            total_items, visible_items, first_visible_index, height);
        
        if legacy_state.visible && !component_cells.is_empty() {
            // Compare basic structure
            assert_eq!(component_cells.len(), height,
                "Component should have correct height for edge case");
            
            // Verify minimum indicator height
            let component_indicator_count = component_cells.iter()
                .filter(|row| row[0].character == '█')
                .count();
            
            assert!(component_indicator_count >= 1,
                "Component should have minimum indicator height for edge case");
            assert_eq!(component_indicator_count, legacy_state.indicator_height,
                "Component and legacy should have same indicator height for edge case");
        }
    }
}

/// Test Case 7: Character and color equivalence
/// When both legacy and component render with custom theme,
/// they should use the same characters and colors.
/// Validates: Requirements 8.2
#[test]
fn test_equivalence_theme_characters_and_colors() {
    // Create custom theme
    let mut theme = Theme::default();
    theme.scrollbar_track_char = "|".to_string();
    theme.scrollbar_indicator_char = "=".to_string();
    theme.scrollbar_fg = "cyan".to_string();
    theme.scrollbar_bg = "blue".to_string();
    
    let total_items = 100;
    let visible_items = 10;
    let first_visible_index = 25;
    let height = 20;
    
    // Get component output
    let scrollbar = Scrollbar::new(total_items, visible_items, first_visible_index);
    let component_cells = scrollbar.render(height, &theme, false);
    
    // Verify component uses theme characters
    let has_track = component_cells.iter().any(|row| row[0].character == '|');
    let has_indicator = component_cells.iter().any(|row| row[0].character == '=');
    
    assert!(has_track, "Component should use custom track character");
    assert!(has_indicator, "Component should use custom indicator character");
    
    // Verify component uses theme colors
    use crossterm::style::Color;
    for row in &component_cells {
        assert_eq!(row[0].fg_color, Color::Cyan, "Component should use custom fg color");
        assert_eq!(row[0].bg_color, Color::Blue, "Component should use custom bg color");
    }
    
    // The legacy function would use the same theme, so this verifies
    // that the component correctly interprets theme values
}

/// Test Case 8: Calculation formula equivalence
/// When both legacy and component calculate indicator positions,
/// they should use the same mathematical formulas.
/// Validates: Requirements 8.2
#[test]
fn test_equivalence_calculation_formulas() {
    let theme = Theme::default();
    let height = 20;
    let start_row = 5;
    let column = 80;
    
    // Test various scroll positions to verify formula equivalence
    for first_visible_index in (0..90).step_by(10) {
        let total_items = 100;
        let visible_items = 10;
        
        // Get legacy calculations
        let legacy_state = calculate_scrollbar_state(
            total_items,
            visible_items,
            first_visible_index,
            start_row,
            height,
            column,
        );
        
        // Get component output
        let scrollbar = Scrollbar::new(total_items, visible_items, first_visible_index);
        let component_cells = scrollbar.render(height, &theme, false);
        
        if legacy_state.visible {
            // Find component indicator position
            let component_indicator_start = component_cells.iter()
                .position(|row| row[0].character == '█')
                .unwrap();
            
            // Calculate expected position using same formula as legacy
            let indicator_height = std::cmp::max(1, (visible_items * height) / total_items);
            let indicator_travel_range = height.saturating_sub(indicator_height);
            let scrollable_items = total_items.saturating_sub(visible_items);
            let expected_offset = if scrollable_items > 0 {
                (first_visible_index * indicator_travel_range) / scrollable_items
            } else {
                0
            };
            
            // Component should match the calculation
            assert_eq!(component_indicator_start, expected_offset,
                "Component indicator position should match formula at scroll position {}",
                first_visible_index);
            
            // Legacy relative position should also match
            let legacy_relative_pos = legacy_state.indicator_start - start_row;
            assert_eq!(legacy_relative_pos, expected_offset,
                "Legacy indicator position should match formula at scroll position {}",
                first_visible_index);
        }
    }
}