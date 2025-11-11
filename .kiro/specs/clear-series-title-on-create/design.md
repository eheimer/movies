# Design Document

## Overview

This design addresses a bug where the series title input field retains previous text when entering series creation mode. The fix involves clearing the `new_series` string and resetting the `edit_cursor_pos` when transitioning to `SeriesCreate` mode from `SeriesSelect` mode.

## Architecture

The application uses a mode-based state machine architecture where different modes handle user input differently. The relevant modes for this fix are:

- **SeriesSelect Mode**: Allows users to select an existing series or press `+` to create a new one
- **SeriesCreate Mode**: Allows users to enter a new series name

State is managed in the main event loop (`main.rs`) and passed to mode-specific handlers in `handlers.rs`.

## Components and Interfaces

### Affected Components

1. **handlers.rs::handle_series_select_mode**

   - Currently handles the transition to `SeriesCreate` mode when `+` is pressed
   - Needs to clear `new_series` and reset `edit_cursor_pos` during this transition

2. **State Variables** (managed in main.rs)
   - `new_series: String` - Stores the text being entered for a new series
   - `edit_cursor_pos: usize` - Tracks the cursor position within the text field
   - `mode: Mode` - Current application mode

### Current Behavior

When the user presses `+` in `SeriesSelect` mode:

```rust
KeyCode::Char('+') => {
    *series_selection = None;
    *mode = Mode::SeriesCreate;
    *redraw = true;
}
```

The `new_series` string and `edit_cursor_pos` are not modified, causing the bug.

### Desired Behavior

When transitioning to `SeriesCreate` mode, both `new_series` and `edit_cursor_pos` should be reset to their initial states (empty string and 0, respectively).

## Data Models

No changes to data models are required. The fix only involves state management during mode transitions.

## Error Handling

No new error handling is required. The fix is a simple state reset operation that cannot fail.

## Testing Strategy

### Manual Testing

1. **Test Case 1: First Series Creation**

   - Start application
   - Select an episode
   - Press F4 to enter SeriesSelect mode
   - Press `+` to enter SeriesCreate mode
   - Verify the series title field is empty
   - Enter a series name and press Enter
   - Verify series is created successfully

2. **Test Case 2: Second Series Creation (Bug Scenario)**

   - After creating a series, select another episode
   - Press F4 to enter SeriesSelect mode
   - Press `+` to enter SeriesCreate mode
   - Verify the series title field is empty (not showing previous series name)
   - Verify cursor is at position 0

3. **Test Case 3: Cancel and Retry**
   - Enter SeriesCreate mode
   - Type some text
   - Press Esc to cancel
   - Press `+` again to re-enter SeriesCreate mode
   - Verify the field is empty

### Implementation Verification

After implementing the fix, verify that:

- The `new_series` string is cleared when entering `SeriesCreate` mode
- The `edit_cursor_pos` is reset to 0 when entering `SeriesCreate` mode
- The screen redraws to show the empty field
- No regression in other mode transitions

## Implementation Notes

The fix should be applied in the `handle_series_select_mode` function in `handlers.rs`. The function signature already includes mutable references to both `new_series` and `edit_cursor_pos`, so no signature changes are needed.

The fix is minimal and localized to a single location in the codebase, reducing the risk of introducing new bugs.
