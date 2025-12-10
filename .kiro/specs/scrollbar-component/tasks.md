# Implementation Plan

- [x] 1. Create Scrollbar component structure
  - Create `src/components/scrollbar.rs` file
  - Define Scrollbar struct with total_items, visible_items, first_visible_index fields
  - Export Scrollbar from `src/components/mod.rs`
  - _Requirements: 1.1, 2.1, 2.2, 2.3_

- [x] 2. Implement Component trait for Scrollbar
  - Implement render method signature accepting height, theme, is_selected parameters
  - Add visibility logic (return empty Vec when scrollbar not needed)
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 3.1, 3.2, 3.3_

- [x] 3. Implement indicator dimension calculations
  - Add indicator height calculation with minimum of 1
  - Add indicator position calculation based on scroll position
  - Add bounds clamping to ensure indicator stays within track
  - _Requirements: 2.4, 2.5, 4.5, 5.1, 5.2, 5.4_

- [x] 4. Implement Cell array generation
  - Create Vec<Vec<Cell>> with height rows and 1 column each
  - Populate track positions with scrollbar_track_char from theme
  - Populate indicator positions with scrollbar_indicator_char from theme
  - Apply scrollbar_fg and scrollbar_bg colors to all cells
  - _Requirements: 1.2, 3.4, 4.1, 4.2, 4.3, 4.4_

- [x] 5. Write unit tests for Scrollbar component
  - Test visibility logic (hidden when items fit, visible when needed)
  - Test indicator position calculation at top, middle, bottom positions
  - Test indicator height calculation with various viewport ratios
  - Test indicator bounds constraint
  - Test minimum indicator height of 1
  - Test track and indicator character usage
  - Test color application from theme
  - Test edge cases (zero height, zero items, invalid positions)
  - Test that render doesn't perform terminal I/O
  - _Requirements: 1.5, 2.4, 2.5, 3.1, 3.2, 3.3, 3.4, 4.1, 4.2, 4.3, 4.4, 4.5, 5.1, 5.2, 5.4, 7.1, 7.2, 7.3, 7.4, 7.5_

- [x] 6. Add helper function for Cell array to terminal output conversion
  - Create function to convert Scrollbar Cell array to terminal output at specified column
  - Handle positioning of single-column multi-row output
  - _Requirements: 1.2_

- [x] 7. Update display.rs to use Scrollbar component
  - Replace render_scrollbar calls with Scrollbar component usage
  - Convert Cell arrays to terminal output at appropriate column positions
  - Verify visual output matches previous implementation
  - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5_

- [x] 8. Write integration tests for browse mode
  - Test scrollbar rendering in browse mode with Scrollbar component
  - Test scrollbar updates when scrolling through lists
  - Test scrollbar visibility based on list size
  - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5_

- [x] 9. Write equivalence tests
  - Test that Scrollbar component produces same output as legacy render_scrollbar
  - Compare Cell arrays with terminal output from legacy function
  - Use specific examples from current codebase
  - _Requirements: 8.2_

- [x] 10. Verify backward compatibility
  - Ensure calculate_scrollbar_state function still works
  - Ensure ScrollBarState structure is unchanged
  - Test that existing code using legacy functions continues to work
  - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [x] 11. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
