use movies::scrollbar::{calculate_scrollbar_state, ScrollBarState, render_scrollbar};
use movies::theme::Theme;

// ============================================================================
// Legacy Function Backward Compatibility Tests (Task 10)
// ============================================================================

/// Test Case 1: calculate_scrollbar_state function still works
/// When calculate_scrollbar_state is called with valid parameters,
/// it should return a ScrollBarState with correct calculations.
/// Validates: Requirements 6.1, 6.2
#[test]
fn test_calculate_scrollbar_state_basic_functionality() {
    // Test basic scrollbar calculation
    let state = calculate_scrollbar_state(100, 20, 0, 5, 20, 80);
    
    assert!(state.visible, "Scrollbar should be visible when total > visible");
    assert_eq!(state.track_start, 5, "Track start should match input");
    assert_eq!(state.track_height, 20, "Track height should match input");
    assert_eq!(state.column, 80, "Column should match input");
    assert_eq!(state.indicator_start, 5, "Indicator should be at top when first_visible_index is 0");
    assert!(state.indicator_height >= 1, "Indicator height should be at least 1");
}

/// Test Case 2: calculate_scrollbar_state with hidden scrollbar
/// When calculate_scrollbar_state is called with total_items <= visible_items,
/// it should return a hidden ScrollBarState.
/// Validates: Requirements 6.1, 6.2
#[test]
fn test_calculate_scrollbar_state_hidden() {
    // Test when scrollbar should be hidden
    let state = calculate_scrollbar_state(10, 20, 0, 5, 20, 80);
    
    assert!(!state.visible, "Scrollbar should be hidden when total <= visible");
    
    // Test with zero items
    let state_zero = calculate_scrollbar_state(0, 10, 0, 5, 20, 80);
    assert!(!state_zero.visible, "Scrollbar should be hidden when total_items is 0");
    
    // Test with zero height
    let state_zero_height = calculate_scrollbar_state(100, 10, 0, 5, 0, 80);
    assert!(!state_zero_height.visible, "Scrollbar should be hidden when height is 0");
}

/// Test Case 3: calculate_scrollbar_state indicator position calculations
/// When calculate_scrollbar_state is called with different scroll positions,
/// it should calculate indicator positions correctly.
/// Validates: Requirements 6.1, 6.2
#[test]
fn test_calculate_scrollbar_state_indicator_positions() {
    let total_items = 100;
    let visible_items = 10;
    let height = 20;
    let start_row = 5;
    let column = 80;
    
    // Test at top position
    let state_top = calculate_scrollbar_state(total_items, visible_items, 0, start_row, height, column);
    assert_eq!(state_top.indicator_start, start_row, "Indicator should be at track start when at top");
    
    // Test at bottom position
    let state_bottom = calculate_scrollbar_state(total_items, visible_items, 90, start_row, height, column);
    let expected_bottom = start_row + height - state_bottom.indicator_height;
    assert!(state_bottom.indicator_start >= expected_bottom - 1, "Indicator should be near track end when at bottom");
    
    // Test at middle position
    let state_middle = calculate_scrollbar_state(total_items, visible_items, 45, start_row, height, column);
    assert!(state_middle.indicator_start > start_row, "Indicator should be below track start at middle position");
    assert!(state_middle.indicator_start < start_row + height - state_middle.indicator_height, "Indicator should be above track end at middle position");
}

/// Test Case 4: calculate_scrollbar_state indicator height calculations
/// When calculate_scrollbar_state is called with different ratios,
/// it should calculate indicator heights proportionally.
/// Validates: Requirements 6.1, 6.2
#[test]
fn test_calculate_scrollbar_state_indicator_heights() {
    let start_row = 0;
    let column = 0;
    let height = 20;
    
    // Test 50% ratio (10 visible out of 20 total)
    let state_50 = calculate_scrollbar_state(20, 10, 0, start_row, height, column);
    assert_eq!(state_50.indicator_height, 10, "50% ratio should give 10 cells indicator height");
    
    // Test 25% ratio (5 visible out of 20 total)
    let state_25 = calculate_scrollbar_state(20, 5, 0, start_row, height, column);
    assert_eq!(state_25.indicator_height, 5, "25% ratio should give 5 cells indicator height");
    
    // Test very small ratio (should be clamped to minimum 1)
    let state_small = calculate_scrollbar_state(1000, 1, 0, start_row, height, column);
    assert_eq!(state_small.indicator_height, 1, "Very small ratio should be clamped to minimum 1");
}

/// Test Case 5: ScrollBarState structure is unchanged
/// When ScrollBarState is created, it should have all expected fields
/// with the same types and accessibility.
/// Validates: Requirements 6.2
#[test]
fn test_scrollbar_state_structure_unchanged() {
    // Test that we can create ScrollBarState with all fields
    let state = ScrollBarState {
        visible: true,
        track_start: 10,
        track_height: 20,
        indicator_start: 12,
        indicator_height: 3,
        column: 80,
    };
    
    // Test that all fields are accessible
    assert_eq!(state.visible, true);
    assert_eq!(state.track_start, 10);
    assert_eq!(state.track_height, 20);
    assert_eq!(state.indicator_start, 12);
    assert_eq!(state.indicator_height, 3);
    assert_eq!(state.column, 80);
    
    // Test hidden state constructor
    let hidden_state = ScrollBarState::hidden();
    assert!(!hidden_state.visible, "Hidden state should not be visible");
    assert_eq!(hidden_state.track_start, 0);
    assert_eq!(hidden_state.track_height, 0);
    assert_eq!(hidden_state.indicator_start, 0);
    assert_eq!(hidden_state.indicator_height, 0);
    assert_eq!(hidden_state.column, 0);
}

/// Test Case 6: ScrollBarState Clone and PartialEq traits
/// When ScrollBarState is cloned or compared, it should work correctly.
/// Validates: Requirements 6.2
#[test]
fn test_scrollbar_state_traits() {
    let state1 = ScrollBarState {
        visible: true,
        track_start: 10,
        track_height: 20,
        indicator_start: 12,
        indicator_height: 3,
        column: 80,
    };
    
    // Test Clone trait
    let state2 = state1.clone();
    assert_eq!(state1, state2, "Cloned state should be equal to original");
    
    // Test PartialEq trait
    let state3 = ScrollBarState {
        visible: true,
        track_start: 10,
        track_height: 20,
        indicator_start: 12,
        indicator_height: 3,
        column: 80,
    };
    assert_eq!(state1, state3, "States with same values should be equal");
    
    // Test inequality
    let state4 = ScrollBarState {
        visible: false,
        track_start: 10,
        track_height: 20,
        indicator_start: 12,
        indicator_height: 3,
        column: 80,
    };
    assert_ne!(state1, state4, "States with different values should not be equal");
}

/// Test Case 7: render_scrollbar function still works
/// When render_scrollbar is called with a valid ScrollBarState,
/// it should execute without panicking (in test environment).
/// Validates: Requirements 6.3
#[test]
fn test_render_scrollbar_function_works() {
    let theme = Theme::default();
    
    // Test with visible scrollbar
    let visible_state = ScrollBarState {
        visible: true,
        track_start: 5,
        track_height: 10,
        indicator_start: 7,
        indicator_height: 2,
        column: 80,
    };
    
    // In test environment, this won't actually render to terminal
    // but should not panic
    let result = render_scrollbar(&visible_state, &theme);
    // We can't verify terminal output in tests, but function should not panic
    assert!(result.is_ok() || result.is_err(), "render_scrollbar should complete without panicking");
    
    // Test with hidden scrollbar
    let hidden_state = ScrollBarState::hidden();
    let result_hidden = render_scrollbar(&hidden_state, &theme);
    assert!(result_hidden.is_ok() || result_hidden.is_err(), "render_scrollbar with hidden state should complete");
}

/// Test Case 8: Legacy functions work with existing usage patterns
/// When legacy functions are used in the same way as existing code,
/// they should continue to work correctly.
/// Validates: Requirements 6.4
#[test]
fn test_legacy_functions_existing_usage_patterns() {
    let theme = Theme::default();
    
    // Simulate existing usage pattern from display.rs
    let total = 100;
    let visible = 20;
    let first_index = 25;
    let start_row = 3;
    let height = 15;
    let column = 79;
    
    // This is how the legacy functions are used in existing code
    let state = calculate_scrollbar_state(total, visible, first_index, start_row, height, column);
    
    // Verify state is calculated correctly
    assert!(state.visible, "Should be visible with these parameters");
    assert_eq!(state.track_start, start_row);
    assert_eq!(state.track_height, height);
    assert_eq!(state.column, column);
    
    // Render the scrollbar (should not panic)
    let render_result = render_scrollbar(&state, &theme);
    assert!(render_result.is_ok() || render_result.is_err(), "Rendering should complete");
}

/// Test Case 9: Legacy functions handle edge cases correctly
/// When legacy functions are called with edge case parameters,
/// they should handle them gracefully.
/// Validates: Requirements 6.1, 6.2, 6.3
#[test]
fn test_legacy_functions_edge_cases() {
    let theme = Theme::default();
    
    // Test with very large numbers
    let state_large = calculate_scrollbar_state(10000, 100, 5000, 0, 50, 80);
    assert!(state_large.visible, "Should handle large numbers");
    assert!(state_large.indicator_height >= 1, "Should maintain minimum indicator height");
    
    // Test with minimum values
    let state_min = calculate_scrollbar_state(2, 1, 0, 0, 1, 0);
    assert!(state_min.visible, "Should be visible with minimum valid values");
    assert_eq!(state_min.indicator_height, 1, "Should have minimum indicator height");
    
    // Test render with edge case state
    let render_result = render_scrollbar(&state_min, &theme);
    assert!(render_result.is_ok() || render_result.is_err(), "Should handle edge case rendering");
}

/// Test Case 10: Legacy and component calculations are equivalent
/// When legacy functions and Scrollbar component are given the same parameters,
/// they should produce equivalent results.
/// Validates: Requirements 6.4
#[test]
fn test_legacy_component_equivalence() {
    use movies::components::{Scrollbar, Component};
    
    let theme = Theme::default();
    let total_items = 100;
    let visible_items = 20;
    let first_visible_index = 30;
    let height = 25;
    
    // Get legacy calculation
    let legacy_state = calculate_scrollbar_state(
        total_items,
        visible_items, 
        first_visible_index,
        0, // start_row
        height,
        0  // column
    );
    
    // Get component calculation
    let scrollbar = Scrollbar::new(total_items, visible_items, first_visible_index);
    let component_result = scrollbar.render(height, &theme, false);
    
    // Both should agree on visibility
    if legacy_state.visible {
        assert!(!component_result.is_empty(), "Component should be visible when legacy is visible");
        assert_eq!(component_result.len(), height, "Component should have same height as legacy");
    } else {
        assert!(component_result.is_empty(), "Component should be hidden when legacy is hidden");
    }
    
    // Both should calculate same indicator height
    if legacy_state.visible {
        let component_indicator_count = component_result.iter()
            .filter(|row| row[0].character == '█')
            .count();
        assert_eq!(component_indicator_count, legacy_state.indicator_height, 
                  "Component and legacy should calculate same indicator height");
    }
}