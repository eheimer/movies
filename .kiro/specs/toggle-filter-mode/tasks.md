# Implementation Plan

- [x] 1. Add filter mode state variable to main loop
  - Add `filter_mode: bool = false` state variable in main.rs main_loop function
  - Pass `filter_mode` to `handle_browse_mode()` as `&mut bool`
  - Pass `filter_mode` to `draw_screen()` as `bool`
  - _Requirements: 1.1, 1.2, 1.3_

- [x] 2. Update function signatures
  - [x] 2.1 Update `handle_browse_mode` signature in handlers.rs
    - Add `filter_mode: &mut bool` parameter
    - _Requirements: 1.1, 1.2, 1.3_
  
  - [x] 2.2 Update `draw_screen` signature in display.rs
    - Add `filter_mode: bool` parameter
    - Pass `filter_mode` to `draw_header()` function call
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_
  
  - [x] 2.3 Update `draw_header` signature in display.rs
    - Add `filter_mode: bool` parameter
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

- [x] 3. Implement filter mode toggle logic in browse mode handler
  - [x] 3.1 Add handler for `/` key to enter filter mode
    - Check `!filter_mode` condition
    - Set `*filter_mode = true`
    - Set `*redraw = true`
    - _Requirements: 1.1, 1.3_
  
  - [x] 3.2 Modify existing character input handler
    - Add condition `if *filter_mode` before adding characters to search string
    - Ensure characters are only added when in filter mode
    - _Requirements: 1.2, 1.3_
  
  - [x] 3.3 Add ENTER handler for filter mode
    - Check `if *filter_mode` condition
    - Set `*filter_mode = false`
    - Set `*redraw = true`
    - Ensure this handler runs before the existing ENTER handler for playing videos
    - _Requirements: 3.1, 3.3_
  
  - [x] 3.4 Modify ESC handler for filter mode
    - Add condition to check `if *filter_mode` before existing ESC handlers
    - Clear search string with `search.clear()`
    - Set `*filter_mode = false`
    - Set `*redraw = true`
    - Ensure existing ESC navigation handlers still work when not in filter mode
    - _Requirements: 3.2, 3.4_

- [x] 4. Update header display logic
  - [x] 4.1 Modify menu helper text based on filter mode
    - When NOT in filter mode: change instruction to include `[/] filter` instead of `type to filter`
    - When in filter mode: change `[ENTER] play` to `[ENTER] accept`
    - When in filter mode: change `[ESC] back` or `[ESC] exit` to `[ESC] cancel`
    - When in filter mode: remove all menu helpers except ENTER and ESC from the first instruction line
    - Keep `[F1] Menu, [CTRL][L] rescan` line unchanged
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_
  
  - [x] 4.2 Implement conditional filter line display
    - Show filter line when `filter_mode` is true OR filter string is not empty
    - Hide filter line when `filter_mode` is false AND filter string is empty
    - _Requirements: 4.1, 4.2, 4.3, 4.4_
  
  - [x] 4.3 Implement cursor display for filter mode
    - Show cursor at end of filter string when `filter_mode` is true
    - Position cursor at column `8 + filter.len()` (after "filter: " prefix)
    - Position cursor at row 4 (filter line row)
    - Cursor will be hidden by existing `hide_cursor()` call when not in filter mode
    - _Requirements: 4.2_

- [x] 5. Implement visual focus indicators for filter mode
  - [x] 5.1 Modify episode list rendering to conditionally apply highlighting
    - In the episode list rendering loop in `draw_screen()`, add condition `&& !filter_mode` to the highlight check
    - Only apply current item highlighting when `i == current_item && !filter_mode`
    - This removes highlight from episode list when in filter mode
    - _Requirements: 5.1, 5.3_
  
  - [x] 5.2 Add highlight styling to filter label when in filter mode
    - In `draw_header()` where the filter line is displayed, conditionally style the "filter:" label
    - When `filter_mode` is true, apply highlight colors to "filter:" label using `with()` and `on()` methods
    - When `filter_mode` is false, display "filter:" label without styling
    - Only style the label text "filter:", not the entire line or the filter string value
    - _Requirements: 5.2, 5.4_
