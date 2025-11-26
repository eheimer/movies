# Implementation Plan

- [x] 1. Add scroll state tracking to main application
  - Add `first_series` variable to track scroll offset in series select mode
  - Initialize `first_series` to 0 in main.rs
  - Pass `first_series` as mutable reference to `draw_screen()` function
  - _Requirements: 1.1, 2.3_

- [x] 2. Update display module function signatures
  - [x] 2.1 Update `draw_screen()` signature to accept `first_series` parameter
    - Add `first_series: &mut usize` parameter to function signature
    - Pass `first_series` to `draw_series_window()` call
    - _Requirements: 1.1, 2.3_
  
  - [x] 2.2 Update `draw_series_window()` signature to accept `first_series` parameter
    - Add `first_series: &mut usize` parameter to function signature
    - _Requirements: 1.1, 2.3_

- [x] 3. Implement scrolling logic in series window rendering
  - [x] 3.1 Calculate maximum visible series items
    - Compute `max_visible_series` by subtracting borders and title from window height
    - Ensure minimum of 1 visible item
    - _Requirements: 1.1, 1.3, 4.3_
  
  - [x] 3.2 Implement viewport adjustment logic
    - Add logic to adjust `first_series` when selected item is above viewport
    - Add logic to adjust `first_series` when selected item is below viewport
    - Keep `first_series` unchanged when selected item is within viewport
    - Mirror the scrolling pattern used in browse mode's `first_entry` logic
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 3.3, 3.4_
  
  - [x] 3.3 Update series rendering to use viewport
    - Use `.skip(first_series)` to start rendering from scroll offset
    - Use `.take(max_visible_series)` to limit rendered items to viewport
    - Adjust row calculation to account for skipped items
    - _Requirements: 1.2, 1.4, 2.4_

- [x] 4. Update handlers module for navigation
  - [x] 4.1 Update `handle_series_select_mode()` signature
    - Add `first_series: &mut usize` parameter to function signature
    - _Requirements: 2.3_
  
  - [x] 4.2 Add vim-style navigation support
    - Add `KeyCode::Char('k')` as alternative to `KeyCode::Up`
    - Add `KeyCode::Char('j')` as alternative to `KeyCode::Down`
    - Ensure both navigation styles trigger the same selection and redraw logic
    - _Requirements: 5.1, 5.2, 5.3, 5.4_

- [x] 5. Wire up the new parameter in main event loop
  - Update the call to `handle_series_select_mode()` in main.rs to pass `first_series`
  - Ensure `first_series` is reset to 0 when entering series select mode
  - _Requirements: 2.3, 2.4_

- [ ]* 6. Manual testing verification
  - Test with small series list (< viewport height)
  - Test with large series list (> viewport height)
  - Test arrow key navigation (up/down)
  - Test vim-style navigation (j/k)
  - Test boundary conditions (first/last series)
  - Test terminal resize behavior
  - Test series assignment flow end-to-end
  - Test series creation flow
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 3.3, 3.4, 4.1, 4.2, 4.3, 4.4, 5.1, 5.2, 5.3, 5.4_
