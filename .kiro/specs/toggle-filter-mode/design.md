# Design Document

## Overview

This design implements a toggle filter mode that requires users to explicitly press `/` to enter filter mode before typing characters will be added to the filter string. This prevents accidental filtering during navigation and provides clearer visual feedback about the current state.

The implementation will add a new state variable to track whether filter mode is active, modify the keyboard event handling in Browse mode, update the header display logic to show appropriate menu helpers, and conditionally display the filter line based on the filter mode state and filter string content.

## Architecture

### State Management

The filter mode state will be managed in the main event loop alongside other application state. A new boolean variable `in_filter_mode` will be added to track whether the user has explicitly activated filtering.

**State Transitions:**
- Browse Mode (not in filter) → Filter Mode: User presses `/`
- Filter Mode → Browse Mode (filter accepted): User presses `ENTER`
- Filter Mode → Browse Mode (filter canceled): User presses `ESC`

### Component Changes

The following components will be modified:

1. **main.rs**: Add `in_filter_mode` state variable and pass it to handlers and display functions
2. **handlers.rs**: Modify `handle_browse_mode` to check filter mode state before adding characters to search string
3. **display.rs**: Update `draw_header` to conditionally display menu helpers and filter line based on filter mode state
4. **util.rs**: No changes required (Mode enum remains unchanged)

## Components and Interfaces

### Main Loop State (main.rs)

Add a new state variable to the main loop:

```rust
let mut in_filter_mode: bool = false;
```

This variable will be passed to:
- `handle_browse_mode()` - to control character input behavior
- `draw_screen()` - to control display of menu helpers and filter line

### Browse Mode Handler (handlers.rs)

The `handle_browse_mode` function will be modified to:

1. Check for `/` key press to enter filter mode
2. Only add typed characters to search string when `in_filter_mode` is true
3. Handle `ENTER` to accept filter and exit filter mode
4. Handle `ESC` to cancel filter (clear search string) and exit filter mode

**Key Logic Changes:**

```rust
// New handler for / key
KeyCode::Char('/') if !in_filter_mode => {
    *in_filter_mode = true;
    search.push('/');
    *redraw = true;
}

// Modified handler for ENTER in filter mode
KeyCode::Enter if *in_filter_mode => {
    *in_filter_mode = false;
    *redraw = true;
}

// Modified handler for ESC in filter mode
KeyCode::Esc if *in_filter_mode => {
    search.clear();
    *in_filter_mode = false;
    *redraw = true;
}

// Modified character input - only add to search when in filter mode
KeyCode::Char(c) if *in_filter_mode => {
    search.push(c);
    *redraw = true;
}
```

**Existing ESC Handlers:**

The current ESC handlers for navigating back through view contexts (Season → Series → TopLevel) will take precedence over the filter mode ESC handler. The filter mode ESC will only trigger when:
- In filter mode AND
- Not in a context where ESC would navigate back

This maintains existing navigation behavior while adding filter cancellation.

### Display Module (display.rs)

The `draw_header` function will be modified to:

1. Accept `in_filter_mode` parameter
2. Conditionally display menu helpers based on filter mode state
3. Conditionally display the filter line based on filter mode and filter string length
4. Show cursor at end of filter string when in filter mode
5. Hide cursor when not in filter mode (handled by existing `hide_cursor()` at start of `draw_screen()`)

**Menu Helper Logic:**

When NOT in filter mode:
- Display normal menu helpers including `[/] filter`
- Do not show `type to filter`

When in filter mode:
- Change `[ENTER] play` to `[ENTER] accept`
- Change `[ESC] back` to `[ESC] cancel`
- Remove all other menu helpers except `[ENTER]` and `[ESC]`

**Filter Line Display Logic:**

```rust
// Show filter line if:
// 1. In filter mode (always show), OR
// 2. Filter string has content (filter was accepted)
if in_filter_mode || !filter.is_empty() {
    // Apply highlight to "filter:" label when in filter mode
    let filter_label = if in_filter_mode {
        format!("filter:", )
            .with(config.current_fg)
            .on(config.current_bg)
            .to_string()
    } else {
        "filter:".to_string()
    };
    
    print_at(0, 4, &format!("{} {}", filter_label, filter))?;
    
    // Show cursor at end of filter string when in filter mode
    if in_filter_mode {
        show_cursor()?;
        move_cursor(8 + filter.len(), 4)?; // "filter: " is 8 chars
    }
}
```

**Cursor Management:**

When entering filter mode:
- Show cursor at the end of the filter string
- Position cursor after "filter: " prefix plus filter string length

When exiting filter mode:
- Hide cursor
- This is handled by the existing `hide_cursor()` call at the start of `draw_screen()`

**Episode List Highlighting Logic:**

The episode list rendering in `draw_screen()` needs to be modified to conditionally apply highlighting based on filter mode state:

```rust
// When rendering each entry in the episode list
for (i, entry) in entries.iter().enumerate().skip(*first_entry).take(max_lines as usize) {
    let display_text = match entry {
        // ... existing entry formatting ...
    };

    let mut formatted_text = format!("{}", display_text);
    
    // Only apply highlight to current item when NOT in filter mode
    if i == current_item && !filter_mode {
        formatted_text = format!(
            "{}",
            display_text
                .with(string_to_fg_color_or_default(&config.current_fg))
                .on(string_to_bg_color_or_default(&config.current_bg))
        );
    }
    
    print_at(0, i - *first_entry + HEADER_SIZE, &formatted_text)?;
}
```

This ensures that:
- When in filter mode: No episode list item is highlighted (focus is on filter input)
- When not in filter mode: Current episode list item is highlighted (focus is on list navigation)

### Function Signatures

The following function signatures will be updated:

**handlers.rs:**
```rust
pub fn handle_browse_mode(
    // ... existing parameters ...
    in_filter_mode: &mut bool,  // NEW
) -> io::Result<bool>
```

**display.rs:**
```rust
pub fn draw_screen(
    // ... existing parameters ...
    in_filter_mode: bool,  // NEW
) -> io::Result<()>

fn draw_header(
    // ... existing parameters ...
    in_filter_mode: bool,  // NEW
) -> io::Result<()>
```

## Data Models

No new data structures are required. The implementation uses a simple boolean flag to track filter mode state.

## Error Handling

No new error conditions are introduced. Existing error handling patterns will be maintained:
- Terminal I/O errors propagated via `io::Result`
- Keyboard event handling errors handled in main event loop

## Testing Strategy

### Manual Testing Scenarios

1. **Enter Filter Mode:**
   - Start in Browse mode
   - Press `/`
   - Verify filter line appears with `/` character
   - Verify menu helpers change to show `[ENTER] accept` and `[ESC] cancel`

2. **Type in Filter Mode:**
   - Enter filter mode with `/`
   - Type additional characters
   - Verify characters are added to filter string
   - Verify entries are filtered in real-time

3. **Accept Filter:**
   - Enter filter mode and type a filter
   - Press `ENTER`
   - Verify filter mode exits
   - Verify filter string is maintained
   - Verify normal menu helpers are restored
   - Verify filter line remains visible

4. **Cancel Filter:**
   - Enter filter mode and type a filter
   - Press `ESC`
   - Verify filter mode exits
   - Verify filter string is cleared
   - Verify normal menu helpers are restored
   - Verify filter line is hidden

5. **Type Without Filter Mode:**
   - Start in Browse mode (not in filter mode)
   - Type characters (not `/`)
   - Verify characters are NOT added to filter string
   - Verify entries are NOT filtered

6. **Navigation Keys in Filter Mode:**
   - Enter filter mode
   - Press arrow keys, PageUp, PageDown
   - Verify navigation still works
   - Verify filter mode remains active

7. **Backspace in Filter Mode:**
   - Enter filter mode and type characters
   - Press Backspace
   - Verify characters are removed from filter string
   - Verify filter mode remains active
   - Verify cursor position updates correctly

7a. **Cursor Display:**
   - Enter filter mode
   - Verify cursor is visible at end of filter string
   - Type characters and verify cursor moves
   - Exit filter mode (ENTER or ESC)
   - Verify cursor is hidden

7b. **Visual Focus Indicators:**
   - Start in Browse mode with an episode selected
   - Verify the selected episode has highlight styling
   - Verify the "filter:" label has no highlight styling
   - Press `/` to enter filter mode
   - Verify the selected episode NO LONGER has highlight styling
   - Verify the "filter:" label NOW has highlight styling
   - Press ENTER to accept filter
   - Verify the selected episode has highlight styling again
   - Verify the "filter:" label no longer has highlight styling

8. **Empty Filter Acceptance:**
   - Enter filter mode (press `/`)
   - Immediately press `ENTER` without typing
   - Verify filter mode exits
   - Verify filter line is hidden (empty filter)

9. **Context Navigation with Filter:**
   - Apply a filter in Series view
   - Press ESC to navigate back to TopLevel
   - Verify navigation works correctly
   - Verify filter behavior is appropriate for context

10. **Menu Helpers in Different Contexts:**
    - Test filter mode in TopLevel, Series, and Season views
    - Verify menu helpers update correctly in each context
    - Verify `[ESC] back` vs `[ESC] exit` vs `[ESC] cancel` display correctly

### Edge Cases

1. **Rapid Mode Switching:**
   - Quickly enter and exit filter mode multiple times
   - Verify state remains consistent

2. **Filter Mode with Empty Entries:**
   - Enter filter mode when no entries are displayed
   - Verify no crashes or unexpected behavior

3. **Long Filter Strings:**
   - Enter filter mode and type a very long filter
   - Verify display truncation works correctly

4. **Special Characters in Filter:**
   - Enter filter mode and type special characters
   - Verify filtering works correctly with special characters

## Implementation Notes

### Order of Operations

1. Add `in_filter_mode` state variable to main.rs
2. Update function signatures in handlers.rs and display.rs
3. Implement filter mode logic in `handle_browse_mode`
4. Update menu helper display logic in `draw_header`
5. Update filter line display logic in `draw_header`
6. Test all scenarios

### Backward Compatibility

This change modifies the user interaction model but does not affect:
- Database schema or data
- Configuration files
- Video playback functionality
- Series/season management

Users will need to adapt to the new `/` key requirement for filtering, but all other functionality remains unchanged.

### Performance Considerations

The addition of a single boolean flag has negligible performance impact. The filtering logic itself remains unchanged - only the trigger mechanism is modified.
