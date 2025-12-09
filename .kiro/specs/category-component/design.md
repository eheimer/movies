# Design Document

## Overview

This design specifies the Category component for rendering series and season entries in the movies terminal application. The Category component implements the Component trait established in the component framework (issue #51), producing 2D Cell arrays that represent formatted category information including title, episode count, and watched count.

The component encapsulates the rendering logic currently embedded in display.rs for series and season entries, making it testable, reusable, and consistent with the Episode component pattern. The design maintains complete visual compatibility with the existing implementation while enabling future extensions for other hierarchical categorization needs.

## Architecture

### Module Structure

```
src/
├── components/
│   ├── mod.rs          # Exports Cell, Component trait, Episode, Category
│   ├── episode.rs      # Episode component (existing)
│   └── category.rs     # Category component (new)
├── display.rs          # Updated to use Category component
└── ... (existing modules)
```

The Category component will be added to the existing `components` module and exported from `mod.rs`.

### Component Rendering Flow

```
1. Create Category component with data (title, episode_count, watched_count, category_type)
2. Call component.render(width, theme, is_selected)
3. Component formats: "Title (X episodes) [Y watched]"
4. Component returns Vec<Vec<Cell>> (single row)
5. Display code converts Cell arrays to terminal output
```

This follows the same pattern as the Episode component, maintaining architectural consistency.

## Components and Interfaces

### Category Component

```rust
pub struct Category {
    pub title: String,
    pub episode_count: usize,
    pub watched_count: usize,
    pub category_type: CategoryType,
}

pub enum CategoryType {
    Series,
    Season,
}

impl Component for Category {
    fn render(&self, width: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>> {
        // Implementation details below
    }
}
```

**Fields:**
- `title`: The name of the series or season
- `episode_count`: Total number of episodes in this category
- `watched_count`: Number of episodes marked as watched
- `category_type`: Distinguishes between series and season (for future extensibility)

**Rendering Logic:**

1. **Format the display string:**
   - Base format: `"{title} ({episode_count} episodes)"`
   - If watched_count > 0: append `" [{watched_count} watched]"`
   - If watched_count == 0: omit the watched portion

2. **Determine colors:**
   - If `is_selected` is true: use `current_fg` and `current_bg` from theme
   - If `is_selected` is false: use default category colors from theme (likely the same as episode colors for consistency)

3. **Truncate to width:**
   - Use the existing `truncate_string` logic from util.rs
   - Account for multi-byte UTF-8 characters if present in title

4. **Convert to Cell array:**
   - Create one Cell per character
   - Apply appropriate colors to each Cell
   - Return as Vec<Vec<Cell>> with a single row

### Integration with Existing Code

The Category component will be used in `display.rs` where series and season entries are currently rendered. The current code likely has inline formatting logic that constructs strings like:

```rust
format!("{} ({} episodes) [{} watched]", name, total, watched)
```

This will be replaced with:

```rust
let category = Category {
    title: name.clone(),
    episode_count: total,
    watched_count: watched,
    category_type: CategoryType::Series, // or Season
};
let cells = category.render(width, theme, is_selected);
// Convert cells to terminal output
```

## Data Models

### Category State

The Category component requires four pieces of information:

- `title`: String - The category name (series or season name)
- `episode_count`: usize - Total episodes in the category
- `watched_count`: usize - Number of watched episodes
- `category_type`: CategoryType enum - Series or Season

These are derived from the current `Entry::Series` and `Entry::Season` data structures, which already contain this information.

### Color Resolution

Colors are resolved from the theme using the existing color resolution functions:
- `string_to_fg_color_or_default` for foreground colors
- `string_to_bg_color_or_default` for background colors

For categories, we'll use:
- Selected: `current_fg` and `current_bg`
- Unselected: Default episode colors (or we can add specific category colors to the theme if needed)

## Test Cases

*A test case is a characteristic or behavior that should hold true across valid executions of a system—essentially, a formal statement about what the system should do. Test cases serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*


### Test Case 1: Title inclusion in output

When a Category component renders with any title string, the rendered Cell array should contain all characters from that title.
**Validates: Requirements 2.1**

### Test Case 2: Episode count formatting

When a Category component renders with any episode count value, the rendered string should contain the formatted count in the pattern "(X episodes)" where X is the episode count.
**Validates: Requirements 2.2**

### Test Case 3: Watched count formatting

When a Category component renders with any non-zero watched count, the rendered string should contain the formatted watched count in the pattern "[Y watched]" where Y is the watched count.
**Validates: Requirements 2.3**

### Test Case 4: Selection color application

When a Category component renders with is_selected=true, all output cells should use the current_fg and current_bg colors from the theme.
**Validates: Requirements 3.3**

### Test Case 5: Default color application

When a Category component renders with is_selected=false, all output cells should use the default category colors from the theme.
**Validates: Requirements 3.4**

### Test Case 6: Text truncation

When a Category component renders with a width smaller than the formatted string length, the output should be truncated to fit within the specified width.
**Validates: Requirements 3.5**

### Test Case 7: Component isolation from terminal I/O

When a Category component's render method is called, it should not perform any terminal I/O operations (no print statements, cursor movements, or screen clears).
**Validates: Requirements 4.6, 6.1**

## Error Handling

### Invalid Width

If `width` is 0, the component should return an empty Cell array rather than panicking. This gracefully handles edge cases where terminal size calculations might produce invalid values.

### Empty Title

If the title is an empty string, the component should still render the episode count information. The output would be: "(X episodes) [Y watched]" or just "(X episodes)" if watched count is zero.

### Unicode Handling

The component must correctly handle multi-byte UTF-8 characters in titles when calculating visual width for truncation. Use `.chars().count()` instead of `.len()` to get character count rather than byte count.

### Zero Episode Count

If episode_count is 0, the component should still render "(0 episodes)" to maintain format consistency. This edge case should be handled gracefully.

## Testing Strategy

### Unit Testing

1. **Category Component Tests**
   - Test rendering with various title lengths (short, long, empty)
   - Test episode count formatting with various values (0, 1, 10, 100+)
   - Test watched count formatting with various values (0, 1, partial, all)
   - Test that zero watched count omits the watched portion
   - Test selection color application (is_selected=true)
   - Test default color application (is_selected=false)
   - Test truncation with various widths
   - Test Unicode character handling in titles
   - Test that render method doesn't perform terminal I/O

2. **Backward Compatibility Tests**
   - Test that Category component output matches current series rendering
   - Test that Category component output matches current season rendering
   - Use specific examples from the current codebase

### Integration Testing

1. **Browse Mode Integration**
   - Test that series entries display correctly using Category component
   - Test that season entries display correctly using Category component
   - Test that selection highlighting works correctly
   - Test navigation through categories

### Test Organization

Following the project's testing conventions:
- All tests in separate files in `tests/` directory
- Update `tests/components_tests.rs` to include Category component tests
- Import from library crate: `use movies::components::*;`

### Test Data

Create test fixtures with:
- Sample Theme objects with known color values
- Sample category titles (short, long, with Unicode, empty)
- Various episode count and watched count combinations
- Edge cases (0 episodes, 0 watched, all watched)

## Implementation Notes

### Refactoring Strategy

1. Create the Category component in `src/components/category.rs`
2. Implement the CategoryType enum
3. Implement the Component trait for Category
4. Add helper functions for formatting (if needed)
5. Export Category from `src/components/mod.rs`
6. Update display.rs to use Category component for series entries
7. Update display.rs to use Category component for season entries
8. Verify visual output matches previous implementation
9. Add tests

### Backward Compatibility

The refactoring maintains complete backward compatibility:
- No changes to database schema
- No changes to configuration format
- No changes to user-visible behavior
- Only internal rendering logic is restructured

### Code Reuse

The Category component will reuse existing utilities:
- `truncate_string` from util.rs for text truncation
- Color resolution functions from display.rs or theme.rs
- Cell and Component trait from the components module

### Future Extensions

The Category component design enables future work:
- **Custom category types**: Add more CategoryType variants for other hierarchical structures
- **Custom formatting**: Add configuration options for count display format
- **Progress indicators**: Add visual progress bars showing watched/total ratio
- **Nested categories**: Support deeper hierarchies beyond series/season/episode

The simple interface and clear separation of concerns make these extensions straightforward.

## Performance Considerations

### String Allocation

The component creates formatted strings during rendering. For typical category names and counts, this is negligible. If performance becomes an issue, we could:
- Pre-allocate string buffers with estimated capacity
- Cache formatted strings if categories are rendered multiple times per frame

### Cell Array Creation

Each render call creates a new Vec<Vec<Cell>>. For single-line components like Category, this is a small allocation (one outer Vec, one inner Vec). The double-buffer rendering system (issue #45) will handle minimizing unnecessary re-renders.

## Dependencies

This feature depends on:
- Component framework (issue #51) - COMPLETED
- Cell struct and Component trait
- Existing theme system
- Existing utility functions (truncate_string)

This feature enables:
- Browser component (future) - can compose Episode and Category components
- Screen composer (issue #48) - can layout multiple components
- Double-buffer rendering (issue #45) - can optimize component rendering
