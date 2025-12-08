# Implementation Plan

- [x] 1. Create scroll bar module with core data structures and calculation logic
  - Create `src/scrollbar.rs` module file
  - Implement `ScrollBarState` struct with visibility, position, and dimension fields
  - Implement `ScrollBarState::hidden()` constructor for non-visible state
  - Implement `calculate_scrollbar_state()` function with proportional positioning logic
  - Add module declaration to `src/lib.rs`
  - _Requirements: 1.1, 1.2, 2.1, 2.2, 2.3, 2.5_

- [ ]* 1.1 Write unit tests for scroll bar calculation logic
  - Test visibility threshold (items <= visible should hide scroll bar)
  - Test top position (first_index = 0 should position at top)
  - Test bottom position (last item visible should position at bottom)
  - Test proportional middle position
  - Test minimum indicator height of 1
  - Test edge cases (empty list, single item, exact fit)
  - _Requirements: 1.1, 1.2, 2.1, 2.2, 2.3, 2.5_

- [x] 2. Add scroll bar configuration to Config struct
  - Add `scrollbar_track_char`, `scrollbar_indicator_char`, `scrollbar_fg`, `scrollbar_bg` fields to `Config` in `src/config.rs`
  - Set default values: track="│", indicator="█", fg="White", bg="Reset"
  - Update `Config::default()` implementation
  - _Requirements: 2.4, 3.4_

- [ ]* 2.1 Write unit tests for scroll bar configuration
  - Test default configuration values
  - Test configuration loading from YAML
  - _Requirements: 2.4, 3.4_

- [x] 3. Implement scroll bar rendering function
  - Implement `render_scrollbar()` function in `src/scrollbar.rs`
  - Draw track characters for full track height using `print_at()`
  - Draw indicator characters at calculated position
  - Apply colors from config using existing color helper functions
  - Handle hidden state by returning early
  - _Requirements: 1.1, 2.4, 3.4_

- [ ]* 3.1 Write unit tests for scroll bar rendering
  - Test that hidden state renders nothing
  - Test track rendering at correct positions
  - Test indicator rendering at correct positions
  - Test color application
  - _Requirements: 1.1, 2.4_

- [x] 4. Integrate scroll bar into episode browser (main list view)
  - Import scrollbar module in `src/display.rs`
  - In `draw_screen()`, calculate scroll bar state after determining `first_entry` and `max_lines`
  - Calculate effective column width: `COL1_WIDTH - 1` when scroll bar is visible
  - Adjust `truncate_string()` calls to use effective width
  - Call `render_scrollbar()` after rendering list items but before detail window
  - Pass `first_entry`, `entries.len()`, `max_lines`, and column position to calculation
  - _Requirements: 1.1, 1.3, 1.4, 3.1_

- [ ]* 4.1 Write integration tests for episode browser scroll bar
  - Test scroll bar appears when list exceeds screen height
  - Test scroll bar hidden when list fits on screen
  - Test content width reduction
  - Test scroll bar updates when scrolling
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 3.1_

- [x] 5. Integrate scroll bar into series select window
  - In `draw_series_window()`, calculate scroll bar state using `first_series` and visible series count
  - Calculate available height: `series_window_height - 3` (subtract borders and title)
  - Adjust series item truncation width to accommodate scroll bar when visible
  - Call `render_scrollbar()` after rendering series items
  - Position scroll bar at `series_window_start_col + series_window_width - 1`
  - _Requirements: 3.2, 3.3_

- [ ]* 5.1 Write integration tests for series select scroll bar
  - Test scroll bar in series selection dialog
  - Test scroll bar with viewport scrolling
  - Test width adjustment for series items
  - _Requirements: 3.2, 3.3_

- [x] 6. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 7. Handle terminal resize scenarios
  - Verify scroll bar recalculates on terminal resize (existing redraw logic should handle this)
  - Test with various terminal sizes
  - Ensure scroll bar stays within bounds
  - _Requirements: 3.5_

- [ ]* 7.1 Write tests for terminal resize handling
  - Test scroll bar recalculation on resize
  - Test bounds checking
  - _Requirements: 3.5_

- [x] 8. Add documentation and examples
  - Add doc comments to all public functions in `src/scrollbar.rs`
  - Document scroll bar configuration options in README or config documentation
  - Add inline comments explaining calculation logic
  - _Requirements: 4.1, 4.2, 4.3, 4.4_
