# Implementation Plan

- [x] 1. Create DetailPanel component structure
  - Create `src/components/detail_panel.rs` with DetailPanel, MetadataDisplay, and EpisodeEditor structs
  - Implement Component trait for all three components
  - Add module declaration to `src/components/mod.rs`
  - _Requirements: 1.1, 1.2, 1.4_

- [x] 2. Implement MetadataDisplay component
  - Create MetadataDisplay struct with episode details and location parameters
  - Implement field rendering logic for read-only display
  - Add text truncation and layout formatting
  - Handle path/filename extraction from location string
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

- [x] 3. Implement EpisodeEditor component
  - Create EpisodeEditor struct with editing state parameters
  - Implement field highlighting for current edit field
  - Add dirty field visual indication using theme colors
  - Handle cursor positioning within field boundaries
  - Prevent editing of non-editable fields (path, filename, etc.)
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_

- [x] 4. Implement DetailPanel container component
  - Create DetailPanel struct that switches between sub-components based on mode
  - Implement mode-based rendering logic (Browse -> MetadataDisplay, Edit -> EpisodeEditor)
  - Handle window borders and layout positioning consistently
  - Ensure episode data is preserved across mode transitions
  - _Requirements: 1.1, 1.2, 1.3, 1.5_

- [x] 5. Integrate DetailPanel into display.rs
  - Replace `draw_detail_window()` function call with DetailPanel component usage
  - Update `draw_screen()` function to create and render DetailPanel component
  - Ensure all existing parameters are passed correctly to new components
  - Maintain identical visual appearance and behavior
  - _Requirements: 4.2, 4.3, 4.4, 4.5_

- [x] 6. Add component tests
  - Create `tests/detail_panel_tests.rs` for component testing
  - Test component creation with various parameter combinations
  - Test mode switching between MetadataDisplay and EpisodeEditor
  - Test basic rendering output for typical use cases
  - _Requirements: 4.2, 5.3_

- [x] 7. Final integration and cleanup
  - Remove unused `draw_detail_window()` function from display.rs
  - Update component exports in `src/components/mod.rs`
  - Verify cursor positioning works correctly in Edit mode
  - Ensure all tests pass and application functions identically
  - _Requirements: 4.1, 4.5_