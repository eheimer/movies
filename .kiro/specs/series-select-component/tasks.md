# Implementation Plan

- [x] 1. Create SeriesCreator sub-component
  - Implement SeriesCreator struct with text input rendering capabilities
  - Add text field rendering with cursor positioning support
  - Add prompt display functionality for series creation mode
  - _Requirements: 3.1, 3.5_

- [ ]* 1.1 Write unit tests for SeriesCreator component
  - Test cursor positioning at different text positions
  - Test text input field rendering with various text lengths
  - Test prompt display formatting
  - _Requirements: 3.1, 3.5_

- [x] 2. Create SeriesSelector sub-component
  - Implement SeriesSelector struct with series list rendering
  - Add series item formatting with numbered labels "[N] Series Name"
  - Add selection highlighting using theme colors
  - Add text truncation for long series names
  - _Requirements: 2.1, 2.3, 2.5_

- [ ]* 2.1 Write unit tests for SeriesSelector component
  - Test series item formatting with various series names
  - Test selection highlighting with different theme configurations
  - Test text truncation with long series names
  - _Requirements: 2.1, 2.3, 2.5_

- [x] 3. Implement scrolling support in SeriesSelector
  - Add scrollbar integration for long series lists
  - Implement viewport management for series navigation
  - Add logic to determine when scrollbar is needed
  - Calculate effective width when scrollbar is visible
  - _Requirements: 2.2, 2.4_

- [ ]* 3.1 Write unit tests for scrolling functionality
  - Test scrollbar visibility with different list sizes
  - Test viewport adjustment when selection changes
  - Test effective width calculation with and without scrollbar
  - _Requirements: 2.2, 2.4_

- [x] 4. Create SeriesSelectWindow container component
  - Implement SeriesSelectWindow struct with mode-based sub-component switching
  - Add constructor accepting all required parameters (mode, series list, selection state, dimensions)
  - Add Component trait implementation that delegates to appropriate sub-component
  - Handle window border rendering with mode-specific border styles
  - _Requirements: 1.1, 1.2, 1.4, 5.3_

- [ ]* 4.1 Write unit tests for SeriesSelectWindow container
  - Test mode-based sub-component switching
  - Test parameter handling in constructor
  - Test border style selection based on mode
  - Test component delegation to sub-components
  - _Requirements: 1.1, 1.2, 1.4, 5.3_

- [x] 5. Implement window positioning and sizing logic
  - Add window dimension calculation based on series count and terminal size
  - Add horizontal centering logic for sidebar positioning
  - Add content alignment and padding within window borders
  - Handle edge cases like empty series lists and small terminal sizes
  - _Requirements: 5.1, 5.2, 5.4, 5.5_

- [ ]* 5.1 Write unit tests for window positioning
  - Test window centering with different sidebar widths
  - Test dynamic height calculation with various series counts
  - Test content alignment within borders
  - Test edge case handling with empty lists and small terminals
  - _Requirements: 5.1, 5.2, 5.4, 5.5_

- [x] 6. Add components to module system
  - Add new component modules to src/components/mod.rs
  - Export SeriesSelectWindow, SeriesSelector, and SeriesCreator
  - Update component imports and visibility
  - _Requirements: 4.1_

- [x] 7. Integrate SeriesSelectWindow into display system
  - Replace draw_series_window() calls in display.rs with SeriesSelectWindow component
  - Update display.rs to use component rendering instead of direct terminal output
  - Maintain identical visual appearance and behavior to original implementation
  - Ensure compatibility with existing state parameters
  - _Requirements: 4.1, 4.2, 4.4_

- [ ]* 7.1 Write integration tests for display system
  - Test component integration with existing display logic
  - Test visual consistency with original implementation
  - Test state parameter compatibility
  - _Requirements: 4.2, 4.4_

- [x] 8. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.