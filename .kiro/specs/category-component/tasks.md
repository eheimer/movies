# Implementation Plan

- [x] 1. Create Category component structure
  - Create `src/components/category.rs` file
  - Define CategoryType enum (Series, Season)
  - Define Category struct with title, episode_count, watched_count, category_type fields
  - Add `pub mod category;` and `pub use category::*;` to `src/components/mod.rs`
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 4.1_

- [x] 1.1 Write unit tests for Category struct creation
  - Test Category struct stores title correctly
  - Test Category struct stores episode_count correctly
  - Test Category struct stores watched_count correctly
  - Test Category struct stores category_type correctly
  - _Requirements: 1.1, 1.2, 1.3, 1.4_

- [x] 2. Implement Component trait for Category
  - Implement Component trait with render method signature
  - Add stub implementation that returns empty Cell array
  - Verify trait implementation compiles
  - _Requirements: 1.5, 4.2, 4.3, 4.4, 4.5_

- [x] 3. Implement Category formatting logic
  - Implement format_display_string helper that creates "Title (X episodes) [Y watched]" format
  - Handle zero watched count case (omit "[Y watched]" portion)
  - Handle edge cases (empty title, zero episodes)
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

- [x] 3.1 Write unit tests for formatting logic
  - **Test Case 1: Title inclusion in output**
  - **Validates: Requirements 2.1**
  - Test that rendered output contains the title string
  - Test with various title lengths and characters

- [x] 3.2 Write unit tests for episode count formatting
  - **Test Case 2: Episode count formatting**
  - **Validates: Requirements 2.2**
  - Test that output contains "(X episodes)" format
  - Test with various episode counts (0, 1, 10, 100+)

- [x] 3.3 Write unit tests for watched count formatting
  - **Test Case 3: Watched count formatting**
  - **Validates: Requirements 2.3**
  - Test that output contains "[Y watched]" format when watched_count > 0
  - Test that output omits watched portion when watched_count = 0
  - Test with various watched counts

- [x] 4. Implement Category rendering with colors
  - Implement color resolution based on is_selected parameter
  - Apply current_fg/current_bg when is_selected=true
  - Apply default category colors when is_selected=false
  - Convert formatted string to Cell array with appropriate colors
  - _Requirements: 3.3, 3.4_

- [x] 4.1 Write unit tests for color application
  - **Test Case 4: Selection color application**
  - **Validates: Requirements 3.3**
  - Test that is_selected=true applies current selection colors
  - Verify all cells use current_fg and current_bg

- [x] 4.2 Write unit tests for default color application
  - **Test Case 5: Default color application**
  - **Validates: Requirements 3.4**
  - Test that is_selected=false applies default colors
  - Verify all cells use default category colors

- [x] 5. Implement text truncation
  - Add truncation logic using existing truncate_string utility
  - Handle Unicode characters correctly in width calculation
  - Ensure output fits within specified width parameter
  - _Requirements: 3.5_

- [x] 5.1 Write unit tests for truncation
  - **Test Case 6: Text truncation**
  - **Validates: Requirements 3.5**
  - Test truncation with width smaller than formatted string
  - Test truncation with Unicode characters in title
  - Test truncation with various width values

- [x] 5.2 Write unit tests for component isolation
  - **Test Case 7: Component isolation from terminal I/O**
  - **Validates: Requirements 4.6, 6.1**
  - Test that render method doesn't perform terminal I/O
  - Verify Cell arrays can be inspected programmatically

- [x] 6. Integrate Category component into display.rs for series entries
  - Locate series entry rendering code in draw_screen()
  - Create Category component instances for series entries
  - Call component.render() with appropriate parameters
  - Convert Cell arrays to terminal output
  - Remove old inline series rendering logic
  - _Requirements: 5.1, 3.1_

- [x] 7. Integrate Category component into display.rs for season entries
  - Locate season entry rendering code in draw_screen()
  - Create Category component instances for season entries
  - Call component.render() with appropriate parameters
  - Convert Cell arrays to terminal output
  - Remove old inline season rendering logic
  - _Requirements: 5.2, 3.2_

- [x] 7.1 Write integration tests for browse mode
  - Test that series entries display correctly with Category component
  - Test that season entries display correctly with Category component
  - Test that selection highlighting works correctly
  - Test navigation through categories
  - _Requirements: 5.3_

- [x] 8. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
