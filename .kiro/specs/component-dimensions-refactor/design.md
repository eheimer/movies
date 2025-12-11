# Design Document

## Overview

This design refactors the Component trait to accept dimensions (width, height) instead of just width. This change simplifies component implementations by removing internal height management from components like Browser and prepares the foundation for horizontal scrollbar support. The refactoring maintains backward compatibility for all existing functionality while providing a cleaner separation of concerns between layout calculation and component rendering.

The key insight is that components should receive complete layout information from their callers rather than managing dimensions internally. This makes components more predictable, testable, and composable.

## Architecture

### Current Component Trait

```rust
pub trait Component {
    fn render(&self, width: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>>;
}
```

### Updated Component Trait

```rust
pub trait Component {
    fn render(&self, width: usize, height: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>>;
}
```

### Component Categories

**Single-line Components (Episode, Category):**
- Ignore the height parameter
- Always return exactly one row
- Use width for truncation as before

**Multi-line Components (Browser, Scrollbar):**
- Use both width and height parameters
- Remove internal height management
- Rely on caller to provide correct dimensions

## Components and Interfaces

### Component Trait Update

```rust
pub trait Component {
    /// Renders the component to a 2D array of Cells
    ///
    /// # Parameters
    ///
    /// * `width` - Maximum width in characters for the rendered output
    /// * `height` - Maximum height in rows for the rendered output
    /// * `theme` - Theme object containing color and style preferences
    /// * `is_selected` - Whether this component represents the currently selected item
    ///
    /// # Returns
    ///
    /// A 2D array of Cells where:
    /// * The outer Vec represents rows (vertical dimension)
    /// * Each inner Vec represents columns (horizontal dimension)
    /// * Output must not exceed width × height dimensions
    fn render(&self, width: usize, height: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>>;
}
```

### Episode Component Changes

The Episode component implementation remains largely unchanged:

```rust
impl Component for Episode {
    fn render(&self, width: usize, _height: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>> {
        // Ignore height parameter (single-line component)
        // Existing logic remains the same
        // ...
    }
}
```

**Key Changes:**
- Add `_height` parameter (prefixed with underscore to indicate intentional non-use)
- No other changes to implementation logic

### Category Component Changes

Similar to Episode, the Category component ignores height:

```rust
impl Component for Category {
    fn render(&self, width: usize, _height: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>> {
        // Ignore height parameter (single-line component)
        // Existing logic remains the same
        // ...
    }
}
```

### Browser Component Simplification

The Browser component benefits most from this change:

**Current Implementation Issues:**
- Manages internal height through `self.height`
- Calculates viewport dimensions internally
- Mixes layout logic with rendering logic

**Updated Implementation:**
```rust
impl Component for Browser {
    fn render(&self, width: usize, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
        // Use provided height instead of self.height
        // Remove internal height management
        // Simplify viewport calculations
        
        let total_items = self.total_items();
        if total_items == 0 || width == 0 || height == 0 {
            return vec![vec![]; height];
        }

        let needs_scrollbar = total_items > height;
        let content_width = if needs_scrollbar { width.saturating_sub(1) } else { width };
        
        // Use provided height for all calculations
        // ...
    }
}
```

**Removed Fields:**
- `self.height` field can be removed from Browser struct
- Height is now provided at render time

**Simplified Logic:**
- No need to track height changes
- No need to recalculate viewport on height changes
- Cleaner separation between state and rendering

### Scrollbar Component Changes

The Scrollbar component currently uses a confusing pattern where `width` is treated as `height`:

**Current Implementation:**
```rust
fn render(&self, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
    // height parameter is actually passed as width due to trait signature
}
```

**Updated Implementation:**
```rust
fn render(&self, _width: usize, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
    // Now height parameter is actually height
    // width is ignored (scrollbars are single-column)
}
```

**Benefits:**
- Eliminates confusing parameter naming
- Prepares for horizontal scrollbar support (would use width parameter)
- Makes the interface more intuitive

## Data Models

### Browser Struct Simplification

**Current Browser Struct:**
```rust
pub struct Browser {
    pub top_left: (usize, usize),
    pub width: usize,
    pub height: usize,  // Remove this field
    pub categories: Vec<Category>,
    pub episodes: Vec<Episode>,
    pub selected_item: usize,
    pub first_visible_item: usize,
}
```

**Updated Browser Struct:**
```rust
pub struct Browser {
    pub top_left: (usize, usize),
    pub width: usize,
    // height field removed - provided at render time
    pub categories: Vec<Category>,
    pub episodes: Vec<Episode>,
    pub selected_item: usize,
    pub first_visible_item: usize,
}
```

### Display Code Changes

The display code that calls component render methods needs to be updated:

**Current Calls:**
```rust
component.render(width, theme, is_selected)
```

**Updated Calls:**
```rust
component.render(width, height, theme, is_selected)
```

**Height Calculation:**
Display code becomes responsible for calculating appropriate heights for each component based on terminal size and layout requirements.

## Test Cases

*A test case is a characteristic or behavior that should hold true across valid executions of a system—essentially, a formal statement about what the system should do. Test cases serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*
### Test Case 1: Component trait signature consistency

When all components implement the updated Component trait, calling render with width, height, theme, and is_selected parameters should work without compilation errors.
**Validates: Requirements 1.1, 1.4**

### Test Case 2: Theme and selection parameter compatibility

When components render with different theme configurations and selection states, the system should produce the same color and styling behavior as the previous implementation.
**Validates: Requirements 1.2**

### Test Case 3: Dimension validation handling

When render methods are called with various dimension values including zero and large values, components should handle edge cases gracefully without panicking.
**Validates: Requirements 1.5**

### Test Case 4: Single-line component height independence

When Episode and Category components render with different height values, the output should be identical regardless of the height parameter.
**Validates: Requirements 2.1, 2.2**

### Test Case 5: Single-line component row count

When single-line components render, the output should always contain exactly one row regardless of the height parameter value.
**Validates: Requirements 2.3**

### Test Case 6: Single-line component width handling

When single-line components render with different width values, the output should be truncated appropriately to fit within the width constraint.
**Validates: Requirements 2.4**

### Test Case 7: Single-line component visual consistency

When single-line components render, the visual output should match the previous implementation's appearance.
**Validates: Requirements 2.5**

### Test Case 8: Browser component height parameter usage

When Browser components render with different height values, the output should vary appropriately based on the provided height.
**Validates: Requirements 3.2**

### Test Case 9: Browser component dimension constraints

When Browser components render, the output dimensions should never exceed the provided width × height bounds.
**Validates: Requirements 3.3, 3.5**

### Test Case 10: Browser component scrolling behavior

When Browser components render with different scroll positions and heights, the scrolling behavior should remain consistent with the previous implementation.
**Validates: Requirements 3.4**

### Test Case 11: Scrollbar component height parameter usage

When Scrollbar components render with different height values, the scrollbar track and indicator should scale appropriately to the provided height.
**Validates: Requirements 4.1**

### Test Case 12: Scrollbar component dimension constraints

When Scrollbar components render, the output should fit within the provided width × height bounds.
**Validates: Requirements 4.2, 4.5**

### Test Case 13: Scrollbar component functionality preservation

When Scrollbar components render with different scroll states, the indicator position should correctly reflect the scroll position as in the previous implementation.
**Validates: Requirements 4.3**

### Test Case 14: Application display integration

When the application runs with the updated components, all visual elements should display correctly without visual regressions.
**Validates: Requirements 5.1**

### Test Case 15: Navigation behavior consistency

When users perform navigation actions, the interface should respond with the same behavior as the previous implementation.
**Validates: Requirements 5.2**

### Test Case 16: Scrolling interaction consistency

When users interact with scrollable content, the scrolling should work correctly and consistently with previous behavior.
**Validates: Requirements 5.3**

### Test Case 17: Responsive layout behavior

When the application renders at different terminal sizes, components should adapt their layout appropriately using the provided dimensions.
**Validates: Requirements 5.4**

### Test Case 18: Visual regression prevention

When components render, the visual output should maintain consistency with previous versions to prevent regressions.
**Validates: Requirements 5.5**

### Test Case 19: Layout calculation separation

When layout changes occur, dimension calculations should happen at the display level and be passed to components rather than calculated internally.
**Validates: Requirements 6.3**

## Error Handling

### Invalid Dimensions

Components should handle edge cases gracefully:

**Zero Width or Height:**
- Single-line components should return empty arrays for zero width
- Multi-line components should return appropriate empty structures
- No component should panic on zero dimensions

**Extremely Large Dimensions:**
- Components should not allocate excessive memory
- Output should be reasonable and bounded
- Performance should remain acceptable

### Theme Parameter Handling

Components should maintain robust theme handling:
- Missing theme values should use defaults
- Invalid color strings should fall back to safe defaults
- Theme changes should be reflected immediately in render output

### Selection State Handling

Components should handle selection state consistently:
- Selection highlighting should work regardless of dimension changes
- Selection state should not affect dimension calculations
- Selection colors should override base colors appropriately

## Testing Strategy

### Unit Testing

1. **Component Trait Implementation Tests**
   - Test that all components implement the new trait signature
   - Test that render methods accept width, height, theme, and is_selected parameters
   - Test that components handle edge cases (zero dimensions, large dimensions)

2. **Single-line Component Tests**
   - Test Episode and Category components ignore height parameter
   - Test that output is always exactly one row
   - Test that width truncation still works correctly
   - Test visual consistency with previous implementation

3. **Multi-line Component Tests**
   - Test Browser component uses provided height instead of internal height
   - Test Scrollbar component uses height parameter correctly
   - Test that output respects dimension constraints
   - Test scrolling behavior consistency

4. **Integration Tests**
   - Test that display code provides both width and height to components
   - Test that layout changes trigger appropriate dimension recalculation
   - Test that the application displays correctly after refactoring
   - Test that user interactions work as before

### Test Organization

Following the project's testing conventions:
- All tests in separate files in `tests/` directory
- Use naming pattern: `component_dimensions_tests.rs` for this refactoring
- Import from library crate: `use movies::components::*;`

### Test Data

Create test fixtures with:
- Various dimension combinations (small, large, zero, typical)
- Sample Theme objects with different configurations
- Sample component data (episodes, categories, browser states)
- Different selection states and scroll positions

## Implementation Notes

### Migration Strategy

1. **Update Component Trait**
   - Add height parameter to trait definition
   - Update trait documentation

2. **Update Single-line Components**
   - Add `_height` parameter to Episode::render
   - Add `_height` parameter to Category::render
   - Verify no behavioral changes

3. **Update Multi-line Components**
   - Update Browser::render to use provided height
   - Remove height field from Browser struct
   - Update Browser constructor and methods
   - Update Scrollbar::render to use height parameter correctly

4. **Update Display Code**
   - Update all component.render() calls to include height
   - Add height calculation logic to display code
   - Test that layout works correctly

5. **Update Tests**
   - Update existing tests to use new signature
   - Add new tests for dimension handling
   - Verify no regressions

### Backward Compatibility

The refactoring maintains functional backward compatibility:
- No changes to user-visible behavior
- No changes to configuration or data formats
- Only internal API changes (Component trait)

### Performance Considerations

The refactoring should have minimal performance impact:
- No additional memory allocations
- Simplified logic in Browser component
- Cleaner separation of concerns

### Future Extensions

This refactoring enables future enhancements:
- **Horizontal Scrollbars**: Scrollbar component can now use width parameter
- **Complex Layouts**: Components can be composed with precise dimension control
- **Responsive Design**: Layout can adapt more precisely to terminal size changes
- **Component Testing**: Easier to test components with specific dimensions