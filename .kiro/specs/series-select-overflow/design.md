# Design Document: Series Select Window Scrolling

## Overview

This design implements scrolling functionality for the series selection window to prevent UI overflow when the number of series exceeds available screen space. The solution mirrors the existing scrolling mechanism used in browse mode for episode lists, ensuring consistency across the application.

## Architecture

### Current State Analysis

The application currently has a working scrolling implementation in browse mode that uses:
- `first_entry`: Tracks the index of the first visible item in the viewport
- `current_item`: Tracks the currently selected item
- Automatic viewport adjustment logic in `draw_screen()` that keeps the selected item visible

The series select window currently lacks this scrolling infrastructure, causing overflow when many series exist.

### Proposed Changes

The fix requires modifications to three main areas:

1. **State Management**: Add a `first_series` variable to track the scroll position in series select mode
2. **Display Logic**: Modify `draw_series_window()` to implement viewport-based rendering with scrolling
3. **Navigation Logic**: Update `handle_series_select_mode()` to support 'j' and 'k' vim-style navigation (arrow keys already work)

## Components and Interfaces

### 1. Main Application State (src/main.rs)

**New State Variable:**
```rust
let mut first_series: usize = 0;
```

This variable tracks the scroll offset for the series selection window, analogous to `first_entry` for browse mode.

**Function Signature Updates:**

The `draw_screen()` function call must pass the new `first_series` parameter:
```rust
display::draw_screen(
    // ... existing parameters ...
    &mut first_series,  // New parameter
)?;
```

### 2. Display Module (src/display.rs)

**Function Signature Changes:**

```rust
pub fn draw_screen(
    // ... existing parameters ...
    first_series: &mut usize,  // New parameter
) -> io::Result<()>
```

```rust
fn draw_series_window(
    mode: &Mode,
    series: &Vec<Series>,
    new_series: &String,
    series_selection: &mut Option<usize>,
    config: &Config,
    first_series: &mut usize,  // New parameter
) -> io::Result<()>
```

**Scrolling Logic Implementation:**

The `draw_series_window()` function will implement scrolling using the same pattern as browse mode:

1. Calculate the maximum number of series items that fit in the window:
   ```rust
   let max_visible_series = series_window_height.saturating_sub(3); // Subtract borders and title
   ```

2. Adjust `first_series` to keep the selected series visible:
   ```rust
   if let Some(selection) = series_selection {
       if *selection < *first_series {
           *first_series = *selection;
       } else if *selection >= *first_series + max_visible_series {
           *first_series = *selection - max_visible_series + 1;
       }
   }
   ```

3. Render only the visible series items:
   ```rust
   for (i, series) in series.iter()
       .enumerate()
       .skip(*first_series)
       .take(max_visible_series)
   {
       // Render series item
   }
   ```

**Height Calculation:**

The series window height calculation must account for:
- Window borders (top and bottom): 2 rows
- Title row: 1 row
- Available space for series items: `series_window_height - 3`

The window height itself is constrained by:
```rust
let max_height = terminal_height.saturating_sub(start_row + 2);
let series_window_height = (series.len() + 3).min(max_height).max(4);
```

### 3. Handlers Module (src/handlers.rs)

**Function Signature Update:**

```rust
pub fn handle_series_select_mode(
    code: KeyCode,
    // ... existing parameters ...
    first_series: &mut usize,  // New parameter
)
```

**Navigation Enhancement:**

Add support for 'j' and 'k' vim-style navigation:

```rust
match code {
    KeyCode::Up | KeyCode::Char('k') => {
        *series_selection = series_selection.map(|s| s.saturating_sub(1)).or(Some(0));
        *redraw = true;
    }
    KeyCode::Down | KeyCode::Char('j') => {
        *series_selection = series_selection.map(|s| s.saturating_add(1)).or(Some(0));
        *redraw = true;
    }
    // ... rest of the match arms ...
}
```

**Bounds Checking:**

The existing bounds checking logic in `draw_series_window()` ensures `series_selection` stays within valid range:
```rust
if let Some(selection) = series_selection {
    if *selection >= series.len() {
        *series_selection = series.len().checked_sub(1);
    }
}
```

This logic will continue to work with the scrolling implementation.

## Data Models

No changes to data models are required. The implementation only adds state tracking for UI scrolling.

## Error Handling

The implementation follows existing error handling patterns:

1. **Terminal Size Queries**: Use `get_terminal_size()` which returns `io::Result`
2. **Arithmetic Operations**: Use saturating arithmetic (`saturating_sub`, `saturating_add`) to prevent overflow/underflow
3. **Bounds Checking**: Validate array indices before access using `checked_sub()` and range checks

No new error conditions are introduced by this change.

## Testing Strategy

### Manual Testing Scenarios

1. **Small Series List (< viewport height)**
   - Verify no scrolling occurs
   - Verify all series are visible
   - Verify navigation works correctly

2. **Large Series List (> viewport height)**
   - Verify only viewport-sized subset is visible
   - Verify scrolling occurs when navigating beyond viewport
   - Verify selected item remains visible during navigation

3. **Navigation Testing**
   - Test arrow key navigation (up/down)
   - Test vim-style navigation (j/k)
   - Test boundary conditions (first/last series)
   - Test rapid navigation in both directions

4. **Terminal Resize**
   - Resize terminal while in series select mode
   - Verify viewport adjusts correctly
   - Verify selected item remains visible

5. **Edge Cases**
   - Empty series list
   - Single series
   - Exactly viewport-sized list
   - Very small terminal height

### Integration Testing

Test the series select scrolling in context:
1. Navigate to series select from different view contexts (TopLevel, Series, Season)
2. Assign series and verify return to browse mode works correctly
3. Create new series and verify list updates with scrolling intact
4. Cancel series selection and verify state is properly reset

### Regression Testing

Verify existing functionality remains intact:
1. Browse mode scrolling still works correctly
2. Other modes (Edit, Entry, SeriesCreate, Menu) are unaffected
3. Series assignment functionality works as before
4. Series creation functionality works as before

## Implementation Notes

### Code Reuse

The scrolling logic directly mirrors the browse mode implementation in `draw_screen()`:

**Browse Mode Pattern:**
```rust
if current_item < *first_entry {
    *first_entry = current_item;
} else if current_item >= *first_entry + max_lines as usize {
    *first_entry = current_item - max_lines as usize + 1;
}
```

**Series Select Pattern:**
```rust
if *selection < *first_series {
    *first_series = *selection;
} else if *selection >= *first_series + max_visible_series {
    *first_series = *selection - max_visible_series + 1;
}
```

This consistency ensures maintainability and predictable behavior.

### Performance Considerations

The implementation has minimal performance impact:
- Scrolling calculations are O(1)
- Iterator operations (`skip`, `take`) are lazy and efficient
- No additional memory allocations beyond a single `usize` variable

### Backward Compatibility

This is a bug fix that improves existing functionality without breaking changes:
- No API changes to public interfaces
- No database schema changes
- No configuration changes
- Existing keyboard shortcuts remain unchanged
- New vim-style navigation is additive (doesn't replace arrow keys)

## Visual Design

The series select window will maintain its current appearance with scrolling behavior:

```
┌────────────────────────────────────────┐
│ Choose a series or [+] to create      │  <- Title (always visible)
│ [1] Series Name 1                      │  <- First visible item
│ [2] Series Name 2                      │
│ [3] Series Name 3                      │  <- Selected (highlighted)
│ [4] Series Name 4                      │
│ [5] Series Name 5                      │  <- Last visible item
└────────────────────────────────────────┘
     (more items below, not shown)
```

When scrolling down:
```
┌────────────────────────────────────────┐
│ Choose a series or [+] to create      │
│ [3] Series Name 3                      │  <- First visible item (scrolled)
│ [4] Series Name 4                      │
│ [5] Series Name 5                      │  <- Selected (highlighted)
│ [6] Series Name 6                      │
│ [7] Series Name 7                      │  <- Last visible item
└────────────────────────────────────────┘
```

The user will not see any visual indicator of scrolling (no scrollbar), but the selected item will always remain visible, matching the browse mode behavior.
