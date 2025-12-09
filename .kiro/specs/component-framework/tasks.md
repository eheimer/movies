# Implementation Plan

- [x] 1. Create component module structure
  - Create `src/components/mod.rs` with Cell, TextStyle structs and Component trait
  - Create `src/components/episode.rs` with Episode struct stub
  - Add `pub mod components;` to `src/lib.rs`
  - _Requirements: 4.1, 4.2, 4.3_

- [x] 1.1 Write unit tests for Cell struct
  - Test Cell creation with various character and style combinations
  - Test TextStyle combinations (bold, italic, underlined, dim, crossed_out)
  - _Requirements: 1.1, 1.2, 1.3, 1.4_

- [x] 2. Implement Cell and TextStyle
  - Implement Cell struct with character, fg_color, bg_color, style fields
  - Implement TextStyle struct with boolean flags for each style attribute
  - Implement Cell::new() constructor
  - Implement Cell::to_styled_content() method to convert to crossterm StyledContent
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5_

- [x] 2.1 Write unit tests for Cell conversion
  - Test Cell::to_styled_content() produces correct ANSI escape sequences
  - Test various color and style combinations
  - _Requirements: 1.5_

- [x] 3. Define Component trait
  - Define Component trait with render method signature
  - Document trait parameters (width, theme, is_selected)
  - Document return type (Vec<Vec<Cell>>)
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6_

- [x] 4. Implement Episode component structure
  - Create Episode struct with name, is_watched, file_exists, is_new fields
  - Implement Component trait for Episode
  - Add helper functions for color resolution (move from display.rs if needed)
  - Add helper functions for style resolution (refactor from display.rs)
  - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [x] 5. Implement Episode rendering logic
  - Implement state priority logic (invalid > new+watched > new > watched > normal)
  - Implement indicator prepending (watched_indicator or unwatched_indicator)
  - Implement style application (watched_style or unwatched_style)
  - Implement selection override (current_fg/current_bg when is_selected=true)
  - Implement text truncation with Unicode awareness
  - Convert formatted string to Vec<Vec<Cell>>
  - _Requirements: 3.5, 3.6, 3.7, 3.8, 3.9, 3.10, 3.11, 3.12_

- [x] 5.1 Write unit tests for Episode rendering states
  - Test rendering with watched indicator
  - Test rendering with unwatched indicator
  - Test rendering with new state colors
  - Test rendering with invalid state colors
  - Test rendering with selection override
  - Test rendering without selection
  - _Requirements: 3.6, 3.7, 3.8, 3.9, 3.11, 3.12_

- [x] 5.2 Write unit tests for Episode text truncation
  - Test truncation with width smaller than name length
  - Test truncation with Unicode characters in indicators
  - Test truncation with various width values
  - _Requirements: 3.10_

- [x] 6. Refactor display.rs to use Episode component
  - Identify the browse mode episode rendering code in draw_screen()
  - Create Episode component instances for each episode entry
  - Call component.render() with appropriate parameters
  - Convert Cell arrays to terminal output using existing print_at() and color functions
  - Remove old inline episode rendering logic
  - _Requirements: 5.1, 5.2, 6.2, 6.4_

- [x] 6.1 Write integration tests for browse mode rendering
  - Test that episodes display correctly in browse mode
  - Test that selection highlighting works
  - Test that filtering works with component rendering
  - Test that different episode states display correctly
  - _Requirements: 5.2, 5.3, 5.4, 5.5_

- [x] 6.2 Write unit tests for component isolation
  - Test that Episode::render() does not perform terminal I/O
  - Test that Cell arrays can be verified without terminal interaction
  - _Requirements: 6.1, 6.3_

- [x] 7. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
