# Design Document

## Overview

The SeriesSelectWindow component extracts series selection and creation functionality from the `draw_series_window()` function in `display.rs` into a modular, composable component architecture. This component follows the established pattern used by DetailPanel, providing a container component that switches between specialized sub-components based on application mode.

The component consists of:
- **SeriesSelectWindow**: Container component that manages mode switching and window layout
- **SeriesSelector**: Sub-component for displaying and navigating existing series with scrolling
- **SeriesCreator**: Sub-component for text input when creating new series

## Architecture

The SeriesSelectWindow follows the established component architecture pattern:

```
SeriesSelectWindow (Container)
├── SeriesSelector (SeriesSelect mode)
│   ├── Series list rendering with numbering
│   ├── Scrollbar integration for long lists
│   └── Selection highlighting
└── SeriesCreator (SeriesCreate mode)
    ├── Text input field rendering
    ├── Cursor positioning
    └── Input prompt display
```

The component integrates with existing systems:
- **Theme System**: Uses theme colors for selection highlighting and borders
- **Scrollbar Component**: Reuses existing scrollbar for series list navigation
- **Mode System**: Responds to SeriesSelect and SeriesCreate modes
- **Database Layer**: Works with Series DTOs from database queries

## Components and Interfaces

### SeriesSelectWindow

Container component that manages the overall window layout and delegates rendering to appropriate sub-components.

```rust
pub struct SeriesSelectWindow {
    mode: Mode,
    series_list: Vec<Series>,
    series_selection: Option<usize>,
    new_series_text: String,
    edit_cursor_pos: usize,
    first_visible_series: usize,
    window_width: usize,
    window_height: usize,
}

impl SeriesSelectWindow {
    pub fn new(
        mode: Mode,
        series_list: Vec<Series>,
        series_selection: Option<usize>,
        new_series_text: String,
        edit_cursor_pos: usize,
        first_visible_series: usize,
        window_width: usize,
        window_height: usize,
    ) -> Self;
}
```

### SeriesSelector

Sub-component for displaying the series selection interface with scrolling support.

```rust
pub struct SeriesSelector {
    series_list: Vec<Series>,
    selected_index: Option<usize>,
    first_visible_index: usize,
    visible_items: usize,
    effective_width: usize,
}

impl SeriesSelector {
    pub fn new(
        series_list: Vec<Series>,
        selected_index: Option<usize>,
        first_visible_index: usize,
        visible_items: usize,
        effective_width: usize,
    ) -> Self;
    
    fn needs_scrollbar(&self) -> bool;
    fn format_series_item(&self, index: usize, series: &Series) -> String;
}
```

### SeriesCreator

Sub-component for handling new series name input with text editing capabilities.

```rust
pub struct SeriesCreator {
    text: String,
    cursor_position: usize,
    field_width: usize,
}

impl SeriesCreator {
    pub fn new(text: String, cursor_position: usize, field_width: usize) -> Self;
    
    fn render_prompt(&self) -> String;
    fn render_input_field(&self) -> String;
}
```

## Data Models

The component works with existing data structures:

### Series (from dto.rs)
```rust
#[derive(Clone)]
pub struct Series {
    pub id: usize,
    pub name: String,
}
```

### Mode (from util.rs)
The component responds to these mode variants:
- `Mode::SeriesSelect`: Display series selection interface
- `Mode::SeriesCreate`: Display series creation interface

### Window Dimensions
Window sizing follows the existing pattern from `draw_series_window()`:
- Width: `SERIES_WIDTH + 2` (42 characters including borders)
- Height: Dynamic based on series count, minimum 4, maximum available terminal height
- Positioning: Centered horizontally within sidebar area

## Error Handling

The component handles several edge cases:

1. **Empty Series List**: Displays appropriate message when no series exist
2. **Terminal Size Constraints**: Adjusts window dimensions to fit within available space
3. **Text Overflow**: Truncates long series names to fit within window width
4. **Invalid Selection**: Handles out-of-bounds selection indices gracefully
5. **Scrolling Bounds**: Prevents scrolling beyond list boundaries

Error handling follows the existing pattern of graceful degradation rather than panicking.

## Testing Strategy

The testing approach follows the workspace guidelines with minimal, focused tests:

### Unit Testing
- Component rendering with various series list sizes
- Window dimension calculations under different terminal sizes
- Text truncation for long series names
- Scrollbar visibility logic
- Cursor positioning in text input mode

### Integration Testing
- Component integration with existing display system
- Theme application and color rendering
- Mode switching between SeriesSelect and SeriesCreate
- Scrollbar component integration

The testing strategy emphasizes essential functionality testing without excessive edge case coverage, following the established workspace patterns.
#
# Test Cases

*A test case is a specific scenario that verifies expected behavior of the system under particular conditions.*

### Test Case 1: Series item formatting

When SeriesSelector renders series items, the system should format each item as "[N] Series Name" where N is the 1-based index.
**Validates: Requirements 2.1**

### Test Case 2: Scrollbar visibility logic

When the series list length exceeds the available window height, the system should include a scrollbar in the rendered output.
**Validates: Requirements 2.2**

### Test Case 3: Selection highlighting

When a series is selected, the system should apply theme colors to highlight that specific series item in the rendered output.
**Validates: Requirements 2.3**

### Test Case 4: Viewport scrolling

When the selected series index is outside the current visible range, the system should adjust the first_visible_index to keep the selection visible.
**Validates: Requirements 2.4**

### Test Case 5: Text truncation

When series names exceed the available width, the system should truncate them to fit within the window boundaries.
**Validates: Requirements 2.5**

### Test Case 6: Cursor positioning

When SeriesCreator renders with different cursor positions, the system should visually indicate the cursor location within the text input field.
**Validates: Requirements 3.5**

### Test Case 7: Window centering

When calculating window position, the system should center the window horizontally within the available sidebar space.
**Validates: Requirements 5.1**

### Test Case 8: Dynamic height calculation

When rendering with different series counts, the system should adjust window height based on the list size while respecting terminal boundaries.
**Validates: Requirements 5.2**

### Test Case 9: Border styling by mode

When rendering in SeriesCreate mode, the system should use thick border characters, and when in SeriesSelect mode, it should use thin border characters.
**Validates: Requirements 5.3**

### Test Case 10: Content alignment

When positioning content within the window, the system should maintain consistent padding and alignment relative to the window borders.
**Validates: Requirements 5.4**

### Test Case 11: Theme integration

When rendering with different theme configurations, the system should apply the appropriate colors and styling from the theme parameters.
**Validates: Requirements 6.4**