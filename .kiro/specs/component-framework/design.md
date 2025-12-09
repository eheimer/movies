# Design Document

## Overview

This design establishes a component-based rendering architecture for the movies terminal application. The core abstraction is a `Component` trait that produces 2D arrays of `Cell` objects, where each `Cell` represents a single terminal character with its styling information. This separates rendering logic from terminal I/O, making components testable and reusable.

The `Episode` component serves as the reference implementation, demonstrating how to encapsulate complex rendering logic (watched indicators, color states, truncation) into a self-contained unit. The design intentionally keeps the scope narrow—refactoring only the browse mode episode list—to validate the architecture before expanding to other components.

## Architecture

### Module Structure

```
src/
├── components/
│   ├── mod.rs          # Exports Cell, Component trait
│   └── episode.rs      # Episode component implementation
├── display.rs          # Updated to use Episode component
└── ... (existing modules)
```

The `components` module will be added to `src/lib.rs` to make it accessible throughout the application.

### Component Rendering Flow

```
1. Create component with data (name, watched status, etc.)
2. Call component.render(width, theme, is_selected)
3. Component returns Vec<Vec<Cell>>
4. Display code converts Cell arrays to terminal output
```

This flow separates concerns:
- Components handle **what** to render (logic, layout, styling)
- Display code handles **how** to output (terminal I/O, cursor positioning)

## Components and Interfaces

### Cell Struct

```rust
pub struct Cell {
    pub character: char,
    pub fg_color: Color,
    pub bg_color: Color,
    pub style: TextStyle,
}
```

**Fields:**
- `character`: The character to display (supports Unicode)
- `fg_color`: Foreground color (using crossterm::style::Color)
- `bg_color`: Background color (using crossterm::style::Color)
- `style`: Text styling attributes (bold, italic, etc.)

**Methods:**
- `new(character, fg_color, bg_color, style)`: Constructor
- `to_styled_content()`: Converts to crossterm's StyledContent for terminal output

### TextStyle Enum

```rust
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub dim: bool,
    pub crossed_out: bool,
}
```

This struct represents the combination of text attributes that can be applied. It mirrors the capabilities of crossterm's styling system.

### Component Trait

```rust
pub trait Component {
    fn render(&self, width: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>>;
}
```

**Parameters:**
- `width`: Maximum width in characters (for truncation)
- `theme`: Theme object containing colors and style preferences
- `is_selected`: Whether this component represents the currently selected item

**Returns:**
- `Vec<Vec<Cell>>`: 2D array where outer Vec is rows, inner Vec is columns

**Design Rationale:**
- Single-line components (like Episode) return a Vec with one row
- Multi-line components (future: detail windows) return multiple rows
- Components are responsible for respecting width and height (if provided) constraints
- Components don't perform terminal I/O—they just produce data

### Episode Component

```rust
pub struct Episode {
    pub name: String,
    pub is_watched: bool,
    pub file_exists: bool,
    pub is_new: bool,
}

impl Component for Episode {
    fn render(&self, width: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>> {
        // Implementation details below
    }
}
```

**Rendering Logic:**

The Episode component replicates the current display.rs logic:

1. **Determine base state** (priority order):
   - Invalid (file doesn't exist) → use invalid colors
   - New + Watched → use new colors with watched indicator
   - New + Unwatched → use new colors with unwatched indicator
   - Watched → use episode colors with watched indicator
   - Unwatched → use episode colors with unwatched indicator

2. **Apply indicator:**
   - Prepend watched_indicator or unwatched_indicator from theme
   - Apply watched_style or unwatched_style to the name

3. **Apply selection override:**
   - If `is_selected` is true, override colors with current_fg/current_bg
   - Keep the indicator and text styling

4. **Truncate to width:**
   - Use the existing `truncate_string` logic
   - Account for multi-byte UTF-8 characters (indicators like ●, ○)

5. **Convert to Cell array:**
   - Create one Cell per character
   - Apply appropriate colors and styles to each Cell

## Data Models

### Episode State

The Episode component needs four pieces of state information:

- `name`: The episode title/filename
- `is_watched`: Boolean indicating watched status
- `file_exists`: Boolean indicating if the video file exists on disk
- `is_new`: Boolean indicating if title matches filename (unedited)

These are derived from the current `Entry::Episode` and `EpisodeDetail` data structures.

### Color Resolution

Colors are resolved from theme strings to crossterm::style::Color using the existing `string_to_fg_color_or_default` and `string_to_bg_color_or_default` functions. These will be moved or made accessible to the components module.

### Style Resolution

Text styles are resolved from theme strings using the existing `apply_text_style` logic. This will be refactored into a function that returns a `TextStyle` struct instead of applying styles directly.

## Test Cases

*A test case is a characteristic or behavior that should hold true across valid executions of a system—essentially, a formal statement about what the system should do. Test cases serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Test Case 1: Cell creation and conversion

When a Cell is created with specific character and styling, converting it to terminal output should produce the correct ANSI escape sequences.
**Validates: Requirements 1.1, 1.2, 1.3, 1.4, 1.5**

### Test Case 2: Component trait implementation

When the Episode component implements the Component trait, calling render should return a 2D Cell array with dimensions matching the expected output.
**Validates: Requirements 2.1, 2.2, 2.6**

### Test Case 3: Episode rendering with watched indicator

When an Episode component renders with is_watched=true, the output should contain the watched indicator character from the theme.
**Validates: Requirements 3.6**

### Test Case 4: Episode rendering with unwatched indicator

When an Episode component renders with is_watched=false, the output should contain the unwatched indicator character from the theme.
**Validates: Requirements 3.7**

### Test Case 5: Episode rendering with new state

When an Episode component renders with is_new=true, the output cells should use the new_fg and new_bg colors from the theme.
**Validates: Requirements 3.8**

### Test Case 6: Episode rendering with invalid state

When an Episode component renders with file_exists=false, the output cells should use the invalid_fg and invalid_bg colors from the theme.
**Validates: Requirements 3.9**

### Test Case 7: Episode rendering with selection

When an Episode component renders with is_selected=true, the output cells should use current_fg and current_bg colors from the theme, overriding state-based colors.
**Validates: Requirements 3.11**

### Test Case 8: Episode rendering without selection

When an Episode component renders with is_selected=false, the output cells should use colors based on the episode's state (watched, new, invalid, or normal).
**Validates: Requirements 3.12**

### Test Case 9: Episode text truncation

When an Episode component renders with a width smaller than the formatted name length, the output should be truncated to fit within the specified width.
**Validates: Requirements 3.10**

### Test Case 10: Browse mode visual consistency

When the application displays episodes in browse mode using the Episode component, the visual output should be identical to the previous implementation.
**Validates: Requirements 5.2**

### Test Case 11: Component isolation from terminal I/O

When a component's render method is called, it should not perform any terminal I/O operations (no print statements, cursor movements, or screen clears).
**Validates: Requirements 6.1**

### Test Case 12: Cell array verification

When a component produces a Cell array, the contents should be verifiable without requiring terminal interaction.
**Validates: Requirements 6.3**

## Error Handling

### Invalid Width

If `width` is 0, components should return an empty Cell array rather than panicking. This gracefully handles edge cases where terminal size calculations might produce invalid values.

### Missing Theme Values

If theme values are missing or invalid, components should fall back to default colors using the existing `string_to_fg_color_or_default` pattern. This ensures robustness against configuration errors.

### Unicode Handling

Components must correctly handle multi-byte UTF-8 characters (like ● and ○) when calculating visual width for truncation. Use `.chars().count()` instead of `.len()` to get character count rather than byte count.

## Testing Strategy

### Unit Testing

1. **Cell Tests**
   - Test Cell creation with various character and style combinations
   - Test Cell to_styled_content conversion produces correct output
   - Test TextStyle combinations (bold+italic, etc.)

2. **Episode Component Tests**
   - Test rendering with each state combination (watched, new, invalid)
   - Test indicator inclusion based on watched status
   - Test color application based on state
   - Test selection override behavior
   - Test truncation with various widths
   - Test Unicode character handling in indicators

3. **Integration Tests**
   - Test Episode component produces identical output to current implementation
   - Test browse mode rendering with Episode component
   - Test that no terminal I/O occurs during component rendering

### Test Organization

Following the project's testing conventions:
- All tests in separate files in `tests/` directory
- Use naming pattern: `components_tests.rs` for component module tests
- Import from library crate: `use movies::components::*;`

### Test Data

Create test fixtures with:
- Sample Theme objects with known color values
- Sample episode names (short, long, with Unicode)
- Various state combinations (watched/unwatched, new/old, valid/invalid)

## Implementation Notes

### Refactoring Strategy

1. Create the components module structure
2. Implement Cell and TextStyle
3. Define the Component trait
4. Implement the Episode component
5. Add helper functions for color/style conversion
6. Update display.rs to use Episode component in browse mode episode list
7. Verify visual output matches previous implementation
8. Add tests

### Backward Compatibility

The refactoring maintains complete backward compatibility:
- No changes to database schema
- No changes to configuration format
- No changes to user-visible behavior
- Only internal rendering logic is restructured

### Future Extensions

This design enables future work:
- **Category component**: Render series/season entries with counts
- **Scrollbar component**: Encapsulate scrollbar rendering logic
- **Browser component**: Compose multiple components into a list view
- **Screen composer** (#48): Manage component layout and positioning
- **Double-buffer rendering** (#45): Optimize terminal updates

The Component trait's simple interface (render returns Cell arrays) makes these extensions straightforward.
