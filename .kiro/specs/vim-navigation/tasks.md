# Implementation Plan

- [x] 1. Add vim key navigation handlers in Browse mode
  - Add `KeyCode::Char('k')` match arm in `handle_browse_mode()` function in `src/handlers.rs`
  - Add `KeyCode::Char('j')` match arm in `handle_browse_mode()` function in `src/handlers.rs`
  - Both match arms must include the `if !*filter_mode` guard to prevent navigation when filtering
  - Implement upward navigation logic for `k` key (decrement `current_item`, update `first_entry` for scrolling)
  - Implement downward navigation logic for `j` key (increment `current_item`, handle list bounds)
  - Set `*redraw = true` after navigation to trigger UI update
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 3.3, 3.4, 4.1, 4.2, 4.3_

- [ ]* 2. Manual testing and verification
  - Test basic navigation with `k` and `j` keys in Browse mode
  - Verify boundary conditions (top and bottom of list)
  - Test that vim keys are ignored in filter mode (typing `k` and `j` should add to filter text)
  - Test that vim keys are ignored in other modes (Edit, Menu, SeriesSelect, SeriesCreate)
  - Verify scrolling behavior matches arrow key behavior
  - Test mixed usage of arrow keys and vim keys
  - Test in all view contexts (TopLevel, Series, Season)
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 3.3, 3.4_
