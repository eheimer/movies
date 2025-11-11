# Implementation Plan

- [x] 1. Update handle_series_select_mode to clear series title field

  - Modify the `KeyCode::Char('+')` handler in `handle_series_select_mode` function
  - Clear the `new_series` string to an empty string
  - Reset `edit_cursor_pos` to 0
  - Ensure the existing `*redraw = true` flag is set to trigger screen update
  - _Requirements: 1.1, 1.2, 1.3_

- [ ]\* 2. Verify the fix with manual testing
  - Build and run the application
  - Test the first series creation scenario
  - Test the second series creation scenario (the original bug case)
  - Test the cancel and retry scenario
  - Verify no regressions in other mode transitions
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5_
