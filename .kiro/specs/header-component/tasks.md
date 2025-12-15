# Implementation Plan

- [x] 1. Create header component module structure
  - Create `src/components/header.rs` with module structure
  - Add header module to `src/components/mod.rs`
  - Define HeaderContext struct for passing data to components
  - _Requirements: 1.1, 2.1-2.4_

- [x] 2. Implement HotkeyHelper component
  - Create HotkeyHelper struct with constructor taking all needed parameters
  - Implement hardcoded helper text generation based on mode
  - Add first-line preferred menu items integration with menu.rs
  - Handle terminal width constraints and overflow
  - _Requirements: 3.1, 3.2, 3.3_

- [ ]* 2.1 Write unit tests for HotkeyHelper component
  - Test primary menu hotkey display
  - Test context-specific helper display
  - Test overflow handling
  - _Requirements: 3.1, 3.2, 3.3_

- [x] 3. Implement LastActionLine component
  - Create LastActionLine struct with constructor
  - Implement last action formatting logic
  - Handle empty state when no repeatable action available
  - _Requirements: 4.1, 4.2, 4.3_

- [ ]* 3.1 Write unit tests for LastActionLine component
  - Test last action display formatting
  - Test empty display when no action available
  - _Requirements: 4.1, 4.2, 4.3_

- [x] 4. Implement Breadcrumbs component
  - Create Breadcrumbs struct with constructor
  - Implement breadcrumb formatting for different view contexts
  - Handle top-level, series, and season navigation contexts
  - _Requirements: 5.1, 5.2, 5.3_

- [ ]* 4.1 Write unit tests for Breadcrumbs component
  - Test series breadcrumb formatting
  - Test season breadcrumb formatting
  - Test top-level context display
  - _Requirements: 5.1, 5.2, 5.3_

- [x] 5. Implement FilterLine component
  - Create FilterLine struct with constructor
  - Implement filter display with highlighting for active state
  - Handle focus indication for filter input
  - Handle empty state when no filter is active
  - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [ ]* 5.1 Write unit tests for FilterLine component
  - Test active filter display with highlighting
  - Test empty display when no filter active
  - Test focus indication
  - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [x] 6. Implement main Header component
  - Create Header struct that composes all four sub-components
  - Implement dynamic height calculation based on active sub-components
  - Implement Component trait with Cell-based rendering
  - Handle conditional rendering of sub-components
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 7.1, 7.2, 7.3_

- [ ]* 6.1 Write unit tests for Header component
  - Test header component composition
  - Test dynamic height calculation
  - Test empty sub-component handling
  - _Requirements: 1.1, 1.2, 7.1, 7.2_

- [x] 7. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 8. Integrate Header component with display.rs
  - Update display.rs to use new Header component instead of draw_header()
  - Pass HeaderContext with all required data to Header component
  - Update terminal rendering to use Header component output
  - _Requirements: 1.5_

- [x] 9. Remove old header implementation
  - Remove draw_header() function from display.rs
  - Clean up any unused helper functions
  - Update any remaining references to old header system
  - _Requirements: 1.5_

- [ ]* 9.1 Write integration tests for header system
  - Test header integration with display system
  - Test header with different application modes
  - Test header with various data states
  - _Requirements: 1.5, 7.4_

- [x] 10. Final Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.