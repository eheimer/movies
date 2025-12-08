# Design Document

## Overview

This design implements a visual scroll bar indicator for scrollable lists in the terminal-based video library manager. The scroll bar will appear in the rightmost column when a list exceeds the visible screen height, providing users with visual feedback about their position within the list. The implementation will be reusable across different list views (episode browser, series select window) and will integrate seamlessly with the existing rendering system.

## Architecture

The scroll bar feature will be implemented as a reusable module within the display system. The architecture follows these principles:

1. **Separation of Concerns**: Scroll bar calculation logic is separate from rendering logic
2. **Reusability**: A single scroll bar component can be used across all list views
3. **Integration**: Minimal changes to existing display code, using composition rather than modification
4. **Responsiveness**: Scroll bar updates automatically when terminal size changes or list position changes

### Component Interaction

```
┌─────────────────┐
│  draw_screen()  │
│                 │
└────────┬────────┘
         │
         ├──────────────────────┐
         │                      │
         v                      v
┌────────────────┐    ┌─────────────────────┐
│ draw_list()    │    │ draw_series_window()│
│                │    │                     │
└────────┬───────┘    └──────────┬──────────┘
         │                       │
         v                       v
┌────────────────────────────────────┐
│  calculate_scrollbar_position()    │
│  - Computes scroll bar metrics    │
└────────┬───────────────────────────┘
         │
         v
┌────────────────────────────────────┐
│  render_scrollbar()                │
│  - Draws scroll bar in column      │
└────────────────────────────────────┘
```

## Components and Interfaces

### 1. ScrollBarState Structure

A structure to hold scroll bar calculation results:

```rust
pub struct ScrollBarState {
    pub visible: bool,           // Whether scroll bar should be shown
    pub track_start: usize,      // Starting row of scroll bar track
    pub track_height: usize,     // Height of scroll bar track
    pub indicator_start: usize,  // Starting row of indicator
    pub indicator_height: usize, // Height of indicator (minimum 1)
    pub column: usize,           // Column position for scroll bar
}
```

### 2. Scroll Bar Calculation Function

```rust
pub fn calculate_scrollbar_state(
    total_items: usize,
    visible_items: usize,
    first_visible_index: usize,
    start_row: usize,
    available_height: usize,
    column: usize,
) -> ScrollBarState
```

**Parameters:**
- `total_items`: Total number of items in the list
- `visible_items`: Number of items that fit in the visible area
- `first_visible_index`: Index of the first visible item
- `start_row`: Starting row for the scroll bar track
- `available_height`: Height available for the scroll bar
- `column`: Column position for rendering

**Returns:** `ScrollBarState` with calculated positions

**Logic:**
- If `total_items <= visible_items`, return `visible: false`
- Calculate indicator height as `max(1, (visible_items * available_height) / total_items)`
- Calculate indicator position proportionally based on `first_visible_index`
- Ensure indicator stays within track bounds

### 3. Scroll Bar Rendering Function

```rust
pub fn render_scrollbar(state: &ScrollBarState, config: &Config) -> io::Result<()>
```

**Parameters:**
- `state`: The calculated scroll bar state
- `config`: Configuration for colors and characters

**Behavior:**
- If `state.visible` is false, return immediately
- Draw track characters for the full track height
- Draw indicator characters at the calculated position
- Use distinct characters for track vs indicator

**Characters:**
- Track: `│` (thin vertical line)
- Indicator: `█` (full block) or `▓` (dark shade)

### 4. Content Width Adjustment

When a scroll bar is visible, content width must be reduced by 1 character. This affects:

- `COL1_WIDTH` constant usage in list rendering
- `truncate_string()` calls for list items
- Series window width calculations

**Implementation approach:**
- Add a helper function `get_effective_col_width(has_scrollbar: bool) -> usize`
- Returns `COL1_WIDTH - 1` if scrollbar is visible, otherwise `COL1_WIDTH`

## Data Models

### ScrollBarState

```rust
pub struct ScrollBarState {
    pub visible: bool,
    pub track_start: usize,
    pub track_height: usize,
    pub indicator_start: usize,
    pub indicator_height: usize,
    pub column: usize,
}

impl ScrollBarState {
    pub fn hidden() -> Self {
        ScrollBarState {
            visible: false,
            track_start: 0,
            track_height: 0,
            indicator_start: 0,
            indicator_height: 0,
            column: 0,
        }
    }
}
```

### Configuration Extensions

Add to `Config` structure:

```rust
pub scrollbar_track_char: String,      // Default: "│"
pub scrollbar_indicator_char: String,  // Default: "█"
pub scrollbar_fg: String,              // Default: "White"
pub scrollbar_bg: String,              // Default: "Reset"
```

## Test Cases

*A test case is a specific scenario that should hold true across valid executions of the system.*

### Test Case 1: Scroll bar visibility threshold

When the total number of list items is less than or equal to the visible screen height, the scroll bar should not be rendered.
**Validates: Requirements 1.2**

### Test Case 2: Scroll bar visibility when needed

When the total number of list items exceeds the visible screen height, the scroll bar should be rendered in the rightmost column.
**Validates: Requirements 1.1**

### Test Case 3: Top position indicator

When the first visible item is at index 0, the scroll bar indicator should be positioned at the top of the scroll bar track.
**Validates: Requirements 1.5, 2.1**

### Test Case 4: Bottom position indicator

When the last visible item is the last item in the list, the scroll bar indicator should be positioned at the bottom of the scroll bar track.
**Validates: Requirements 2.2**

### Test Case 5: Proportional middle position

When the first visible item is at the middle of the list, the scroll bar indicator should be positioned approximately in the middle of the track.
**Validates: Requirements 2.3**

### Test Case 6: Content width reduction

When the scroll bar is visible, the content width available for list items should be reduced by exactly 1 character.
**Validates: Requirements 1.4**

### Test Case 7: Indicator minimum height

When calculating the indicator height, the result should always be at least 1 character tall, even for very long lists.
**Validates: Requirements 2.4**

### Test Case 8: Episode browser integration

When the episode browser displays a scrollable list, the scroll bar should render using the reusable component without errors.
**Validates: Requirements 3.1**

### Test Case 9: Series select integration

When the series select window displays a scrollable list, the scroll bar should render using the reusable component without errors.
**Validates: Requirements 3.2**

### Test Case 10: Terminal resize handling

When the terminal window is resized, the scroll bar should recalculate its position and dimensions based on the new terminal size.
**Validates: Requirements 3.5**

### Test Case 11: Scroll position updates

When the user scrolls through the list (up or down), the scroll bar indicator position should update to reflect the new position.
**Validates: Requirements 1.3**

### Test Case 12: Distinct visual characters

When the scroll bar is rendered, the indicator character should be visually distinct from the track character.
**Validates: Requirements 2.4**

## Error Handling

### Terminal Size Errors

If `get_terminal_size()` fails:
- Log the error
- Return a hidden scroll bar state
- Continue rendering without scroll bar

### Invalid Parameters

If scroll bar calculation receives invalid parameters (e.g., `total_items == 0`):
- Return a hidden scroll bar state
- Log a warning for debugging

### Rendering Errors

If `render_scrollbar()` encounters an IO error:
- Log the error
- Continue with the rest of the display rendering
- Do not crash the application

## Testing Strategy

### Unit Testing

1. **Scroll Bar Calculation Tests**
   - Test visibility logic with various item counts
   - Test indicator position at top, middle, and bottom
   - Test indicator height calculation
   - Test edge cases (empty list, single item, exact fit)

2. **Content Width Tests**
   - Test width reduction when scroll bar is visible
   - Test width remains unchanged when scroll bar is hidden

3. **Configuration Tests**
   - Test default scroll bar characters
   - Test custom scroll bar characters from config

### Integration Testing

1. **Episode Browser Tests**
   - Test scroll bar appears when list is long
   - Test scroll bar updates when scrolling
   - Test scroll bar disappears when list is short

2. **Series Select Window Tests**
   - Test scroll bar in series selection dialog
   - Test scroll bar with viewport scrolling

3. **Terminal Resize Tests**
   - Test scroll bar recalculates on resize
   - Test content width adjusts appropriately

### Edge Case Testing

1. **Boundary Conditions**
   - List with exactly visible_items count
   - List with visible_items + 1 count
   - Very long lists (1000+ items)
   - Single item lists

2. **Visual Verification**
   - Scroll bar appears in correct column
   - Indicator is visually distinct from track
   - No overlap with list content

## Implementation Notes

### Integration Points

1. **Episode Browser** (`draw_screen` function)
   - Calculate scroll bar state after determining `first_entry` and `max_lines`
   - Adjust `COL1_WIDTH` when rendering list items
   - Call `render_scrollbar()` after rendering list items

2. **Series Select Window** (`draw_series_window` function)
   - Calculate scroll bar state using `first_series` and visible series count
   - Adjust series window width to accommodate scroll bar
   - Render scroll bar within the series window

### Performance Considerations

- Scroll bar calculations are O(1) operations
- No additional memory allocation per frame
- Minimal impact on rendering performance

### Accessibility

- Use Unicode box-drawing characters for better visual clarity
- Ensure scroll bar colors have sufficient contrast
- Provide configuration options for character customization

## Future Enhancements

1. **Mouse Support**: Allow clicking on scroll bar to jump to position
2. **Smooth Scrolling**: Animate indicator movement
3. **Page Indicators**: Show page numbers alongside scroll bar
4. **Horizontal Scroll Bars**: Support for wide content
