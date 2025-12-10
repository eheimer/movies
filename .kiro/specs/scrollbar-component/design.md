# Design Document

## Overview

This design specifies the refactoring of the existing scrollbar functionality to conform to the component framework established in issue #51. The current scrollbar implementation in `src/scrollbar.rs` predates the component framework and performs direct terminal I/O operations through the `render_scrollbar` function. This refactoring transforms it into a proper Scrollbar component that implements the Component trait and produces Cell arrays.

The design maintains all existing scrollbar functionality—vertical scrollbar with configurable height, item count tracking, and proportional position indication—while adopting the component architecture pattern. The existing `calculate_scrollbar_state` function and `ScrollBarState` struct will be preserved for backward compatibility during migration, allowing gradual adoption of the new component-based approach.

## Architecture

### Module Structure

```
src/
├── components/
│   ├── mod.rs          # Exports Cell, Component trait, Episode, Category, Scrollbar
│   ├── episode.rs      # Episode component (existing)
│   ├── category.rs     # Category component (existing)
│   └── scrollbar.rs    # Scrollbar component (new)
├── scrollbar.rs        # Legacy functions (calculate_scrollbar_state, render_scrollbar)
├── display.rs          # Updated to use Scrollbar component
└── ... (existing modules)
```

The Scrollbar component will be added to the `components` module as `src/components/scrollbar.rs` and exported from `mod.rs`. The existing `src/scrollbar.rs` will be maintained temporarily for backward compatibility, with the legacy `render_scrollbar` function eventually deprecated in favor of the component-based approach.

### Component Rendering Flow

```
1. Create Scrollbar component with data (total_items, visible_items, first_visible_index)
2. Call component.render(height, theme, is_selected=false)
3. Component calculates indicator position and height
4. Component returns Vec<Vec<Cell>> (one column, multiple rows)
5. Display code converts Cell arrays to terminal output at specified column position
```

This follows the same pattern as Episode and Category components, maintaining architectural consistency. Note that scrollbars don't use the `is_selected` parameter but accept it for trait compliance.

## Components and Interfaces

### Scrollbar Component

```rust
pub struct Scrollbar {
    pub total_items: usize,
    pub visible_items: usize,
    pub first_visible_index: usize,
}

impl Component for Scrollbar {
    fn render(&self, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
        // Implementation details below
    }
}
```

**Fields:**
- `total_items`: Total number of items in the scrollable list
- `visible_items`: Number of items that fit in the current viewport
- `first_visible_index`: Index of the first visible item (0-based scroll position)

**Rendering Logic:**

1. **Determine visibility:**
   - If `total_items <= visible_items`: return empty Vec (no scrollbar needed)
   - If `total_items == 0`: return empty Vec
   - If `height == 0`: return empty Vec
   - Otherwise: proceed with rendering

2. **Calculate indicator dimensions:**
   - `indicator_height = max(1, (visible_items * height) / total_items)`
   - Ensures minimum height of 1 for visibility

3. **Calculate indicator position:**
   - `indicator_travel_range = height - indicator_height`
   - `scrollable_items = total_items - visible_items`
   - `indicator_offset = (first_visible_index * indicator_travel_range) / scrollable_items`
   - `indicator_start = indicator_offset`

4. **Clamp indicator to bounds:**
   - If `indicator_start + indicator_height > height`: adjust indicator_start
   - Ensures indicator never extends past track end

5. **Build Cell array:**
   - Create Vec with `height` rows, each containing 1 Cell
   - For rows in track but not in indicator: use `scrollbar_track_char` from theme
   - For rows in indicator: use `scrollbar_indicator_char` from theme
   - Apply `scrollbar_fg` and `scrollbar_bg` colors to all cells

6. **Return result:**
   - Return Vec<Vec<Cell>> where outer Vec has `height` elements (rows)
   - Each inner Vec has 1 element (single column)

### Integration with Existing Code

The Scrollbar component will be used in `display.rs` where scrollbars are currently rendered. The current code uses:

```rust
let state = calculate_scrollbar_state(total, visible, first_index, start_row, height, column);
render_scrollbar(&state, theme)?;
```

This will be replaced with:

```rust
let scrollbar = Scrollbar {
    total_items: total,
    visible_items: visible,
    first_visible_index: first_index,
};
let cells = scrollbar.render(height, theme, false);
// Convert cells to terminal output at specified column position
```

### Backward Compatibility Layer

The existing `calculate_scrollbar_state` and `render_scrollbar` functions will remain in `src/scrollbar.rs` for backward compatibility. They can be gradually phased out as display code migrates to the component-based approach.

```rust
// Legacy function - maintained for compatibility
pub fn calculate_scrollbar_state(...) -> ScrollBarState {
    // Existing implementation unchanged
}

// Legacy function - maintained for compatibility
pub fn render_scrollbar(state: &ScrollBarState, theme: &Theme) -> io::Result<()> {
    // Existing implementation unchanged
}
```

## Data Models

### Scrollbar State

The Scrollbar component requires three pieces of information:

- `total_items`: usize - Complete count of items in the list
- `visible_items`: usize - Number of items visible in the viewport
- `first_visible_index`: usize - Current scroll position (0-based)

These values are sufficient to calculate all rendering parameters (indicator position, indicator height, visibility).

### Calculation Formulas

The component uses the same calculation logic as the existing `calculate_scrollbar_state` function:

**Indicator Height:**
```
indicator_height = max(1, (visible_items * available_height) / total_items)
```

**Indicator Position:**
```
indicator_travel_range = available_height - indicator_height
scrollable_items = total_items - visible_items
indicator_offset = (first_visible_index * indicator_travel_range) / scrollable_items
```

**Bounds Clamping:**
```
if indicator_start + indicator_height > track_end:
    indicator_start = track_end - indicator_height
```

### Color Resolution

Colors are resolved from the theme using the existing color resolution functions:
- `string_to_fg_color_or_default` for `scrollbar_fg`
- `string_to_bg_color_or_default` for `scrollbar_bg`

Characters are taken directly from theme:
- `scrollbar_track_char` for track positions
- `scrollbar_indicator_char` for indicator positions

## Test Cases

*A test case is a characteristic or behavior that should hold true across valid executions of a system—essentially, a formal statement about what the system should do. Test cases serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*


### Test Case 1: Render returns Cell array structure

When the Scrollbar component's render method is called, it should return a Vec<Vec<Cell>> structure.
**Validates: Requirements 1.2**

### Test Case 2: No terminal I/O during rendering

When the Scrollbar component's render method is called, it should not perform any terminal I/O operations.
**Validates: Requirements 1.5**

### Test Case 3: Indicator position calculation

When the Scrollbar component renders with any valid configuration, the indicator position should be calculated proportionally based on the scroll position using the formula: indicator_offset = (first_visible_index * indicator_travel_range) / scrollable_items.
**Validates: Requirements 2.4**

### Test Case 4: Indicator height proportional to viewport ratio

When the Scrollbar component renders with any valid configuration, the indicator height should be proportional to the visible/total ratio using the formula: indicator_height = max(1, (visible_items * height) / total_items).
**Validates: Requirements 2.5**

### Test Case 5: Hidden when items fit on screen

When the Scrollbar component renders with total_items less than or equal to visible_items, it should return an empty Cell array.
**Validates: Requirements 3.1**

### Test Case 6: Visible scrollbar produces output

When the Scrollbar component renders with total_items greater than visible_items, it should return a non-empty Cell array containing track and indicator characters.
**Validates: Requirements 3.4**

### Test Case 7: Track character usage

When the Scrollbar component renders a visible scrollbar, all track positions (non-indicator rows) should contain the scrollbar_track_char from the theme.
**Validates: Requirements 4.1**

### Test Case 8: Indicator character usage

When the Scrollbar component renders a visible scrollbar, all indicator positions should contain the scrollbar_indicator_char from the theme.
**Validates: Requirements 4.2**

### Test Case 9: Foreground color application

When the Scrollbar component renders a visible scrollbar, all cells should use the scrollbar_fg color from the theme.
**Validates: Requirements 4.3**

### Test Case 10: Background color application

When the Scrollbar component renders a visible scrollbar, all cells should use the scrollbar_bg color from the theme.
**Validates: Requirements 4.4**

### Test Case 11: Indicator bounds constraint

When the Scrollbar component renders with any valid configuration, the indicator should never extend past the track bounds (indicator_start + indicator_height <= track_height).
**Validates: Requirements 4.5**

### Test Case 12: Indicator at top position

When the Scrollbar component renders with first_visible_index = 0, the indicator should be positioned at the track start (indicator_start = 0).
**Validates: Requirements 5.1**

### Test Case 13: Indicator at bottom position

When the Scrollbar component renders with first_visible_index at maximum scroll position, the indicator should be positioned at the track end.
**Validates: Requirements 5.2**

### Test Case 14: Minimum indicator height

When the Scrollbar component renders with any valid configuration, the indicator height should never be less than 1 row.
**Validates: Requirements 5.4**

### Test Case 15: Legacy function compatibility

When the calculate_scrollbar_state function is called with the same parameters as before refactoring, it should return a ScrollBarState with the same structure and values.
**Validates: Requirements 6.2**

### Test Case 16: Component and legacy equivalence

When the Scrollbar component renders with the same parameters as the legacy render_scrollbar function, it should produce visually identical output.
**Validates: Requirements 8.2**

## Error Handling

### Zero Height

If `height` is 0, the component should return an empty Cell array rather than panicking. This gracefully handles edge cases where terminal size calculations might produce invalid values.

### Zero Total Items

If `total_items` is 0, the component should return an empty Cell array. There's nothing to scroll through, so no scrollbar is needed.

### Invalid Scroll Position

If `first_visible_index` is greater than or equal to `total_items`, the component should clamp it to a valid range rather than panicking. This handles edge cases where scroll position might be out of sync with list size.

### Division by Zero

The component must handle cases where `scrollable_items` is 0 (when `total_items == visible_items`). In this case, the scrollbar should be hidden (empty Cell array).

### Theme Character Handling

If theme scrollbar characters are empty strings or contain multi-byte UTF-8 characters, the component should handle them gracefully. Use the first character of the string, or a default character if the string is empty.

## Testing Strategy

### Unit Testing

1. **Scrollbar Component Tests**
   - Test rendering with various item counts (total > visible, total <= visible, total = 0)
   - Test rendering with various heights (0, 1, 10, 100)
   - Test rendering with various scroll positions (top, middle, bottom)
   - Test indicator position calculation with different configurations
   - Test indicator height calculation with different viewport ratios
   - Test that hidden scrollbars return empty arrays
   - Test that visible scrollbars contain correct characters
   - Test color application from theme
   - Test indicator bounds constraint
   - Test minimum indicator height of 1
   - Test that render method doesn't perform terminal I/O

2. **Legacy Function Tests**
   - Test that calculate_scrollbar_state still works as before
   - Test that ScrollBarState structure is unchanged
   - Test conversion between ScrollBarState and Scrollbar component

3. **Equivalence Tests**
   - Test that Scrollbar component produces same output as legacy render_scrollbar
   - Use specific examples from current codebase
   - Compare Cell arrays with terminal output from legacy function

### Integration Testing

1. **Browse Mode Integration**
   - Test that scrollbars display correctly in browse mode using Scrollbar component
   - Test that scrollbar updates correctly when scrolling through lists
   - Test that scrollbar hides when all items fit on screen
   - Test that scrollbar appears when items exceed viewport

### Edge Case Testing

1. **Boundary Conditions**
   - Test with total_items = 0
   - Test with height = 0
   - Test with visible_items = 0
   - Test with first_visible_index at maximum
   - Test with very large item counts (1000+)
   - Test with very small heights (1, 2)

2. **Calculation Edge Cases**
   - Test when indicator_height would be 0 (should be clamped to 1)
   - Test when indicator would extend past track end (should be clamped)
   - Test when scrollable_items = 0 (should hide scrollbar)

### Test Organization

Following the project's testing conventions:
- All tests in separate files in `tests/` directory
- Update `tests/components_tests.rs` to include Scrollbar component tests
- Create `tests/scrollbar_tests.rs` for legacy function tests if needed
- Import from library crate: `use movies::components::*;`

### Test Data

Create test fixtures with:
- Sample Theme objects with known scrollbar characters and colors
- Various item count combinations (total, visible, position)
- Various height values (small, medium, large)
- Edge case configurations (zeros, maximums, boundaries)

## Implementation Notes

### Refactoring Strategy

1. Create the Scrollbar component in `src/components/scrollbar.rs`
2. Implement the Component trait for Scrollbar
3. Add calculation logic (reuse formulas from existing code)
4. Add helper functions for bounds checking and clamping
5. Export Scrollbar from `src/components/mod.rs`
6. Update display.rs to use Scrollbar component
7. Verify visual output matches previous implementation
8. Add tests
9. Gradually deprecate legacy render_scrollbar function

### Backward Compatibility

The refactoring maintains complete backward compatibility:
- No changes to database schema
- No changes to configuration format
- No changes to user-visible behavior
- Legacy functions remain available during migration
- Only internal rendering logic is restructured

### Code Reuse

The Scrollbar component will reuse existing utilities:
- Color resolution functions from display.rs or theme.rs
- Cell and Component trait from the components module
- Calculation formulas from existing calculate_scrollbar_state function

### Migration Path

The migration from legacy to component-based scrollbar can happen gradually:

1. **Phase 1**: Create Scrollbar component alongside legacy functions
2. **Phase 2**: Update display.rs to use Scrollbar component
3. **Phase 3**: Verify all scrollbar rendering uses component approach
4. **Phase 4**: Deprecate legacy render_scrollbar function
5. **Phase 5**: Eventually remove legacy function (optional)

This allows for safe, incremental adoption without breaking existing functionality.

### Differences from Other Components

The Scrollbar component differs from Episode and Category components in several ways:

1. **Multi-row output**: Returns multiple rows (one per height unit) vs single row
2. **Single column**: Returns one column vs variable width
3. **No selection state**: Doesn't use is_selected parameter (always false)
4. **Height parameter**: Uses height instead of width for sizing
5. **Visibility logic**: Can return empty array when not needed

These differences are accommodated by the flexible Component trait interface.

## Performance Considerations

### Calculation Overhead

The component performs integer arithmetic for position and height calculations. This is negligible for typical scrollbar sizes (10-100 rows). The calculations are O(1) and don't depend on item count.

### Cell Array Creation

Each render call creates a Vec<Vec<Cell>> with `height` rows and 1 column per row. For typical heights (20-50 rows), this is a small allocation. The double-buffer rendering system (issue #45) will handle minimizing unnecessary re-renders.

### Memory Usage

A scrollbar with height 50 creates 50 Cell objects. Each Cell contains:
- 1 char (4 bytes)
- 2 Color enums (~8 bytes each)
- 1 TextStyle struct (~5 bytes)

Total per Cell: ~25 bytes. For 50 cells: ~1.25 KB. This is negligible.

## Dependencies

This feature depends on:
- Component framework (issue #51) - COMPLETED
- Cell struct and Component trait
- Existing theme system (scrollbar_fg, scrollbar_bg, scrollbar_track_char, scrollbar_indicator_char)
- Existing scrollbar calculation logic

This feature enables:
- Browser component (future) - can compose scrollbars with other components
- Screen composer (issue #48) - can layout scrollbars with other components
- Double-buffer rendering (issue #45) - can optimize scrollbar rendering

## Future Extensions

The Scrollbar component design enables future work:

1. **Horizontal scrollbars**: Add orientation parameter (vertical/horizontal)
2. **Custom styling**: Support different track/indicator styles per theme
3. **Smooth scrolling**: Add fractional position support for smoother animation
4. **Interactive scrollbars**: Add click/drag support for direct position control
5. **Multi-column scrollbars**: Support wider scrollbar designs

The component architecture makes these extensions straightforward without breaking existing code.
