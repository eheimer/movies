# Implementation Plan

- [x] 1. Add count styling configuration fields
  - Add `count_fg`, `count_style`, `header_fg`, `header_style`, `help_fg`, and `help_style` fields to Config struct
  - Add default functions for each new field
  - Update `Config::default()` implementation
  - Update `generate_yaml_with_comments()` to include new fields with documentation
  - _Requirements: 2.1, 2.2, 3.1, 3.2_

- [ ]* 1.1 Write unit tests for configuration fields
  - Test loading config with new count style fields
  - Test missing fields use defaults
  - Test config save includes new fields
  - _Requirements: 2.1, 2.2, 2.5_

- [x] 2. Update series display formatting
  - Modify `format_series_display()` signature to accept `terminal_width` and `config` parameters
  - Change count format from "X episodes (Y unwatched)" to "W/X watched" where W = X - Y
  - Implement right-justification logic with spacing calculation
  - Apply count styling using `apply_text_style()` and color from config
  - Handle edge cases (zero episodes, database errors)
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5_

- [ ]* 2.1 Write unit tests for series display formatting
  - Test count format change (3/5 watched)
  - Test right-justification with various widths
  - Test spacing preservation
  - Test edge cases (0/0, 5/5, database error)
  - _Requirements: 1.3, 1.4, 1.5, 4.1, 4.4_

- [x] 3. Update season display formatting
  - Modify `format_season_display()` signature to accept `terminal_width` and `config` parameters
  - Change count format from "X episodes (Y unwatched)" to "W/X watched"
  - Implement right-justification logic with spacing calculation
  - Apply count styling using `apply_text_style()` and color from config
  - Handle edge cases (zero episodes, database errors)
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5_

- [ ]* 3.1 Write unit tests for season display formatting
  - Test count format change
  - Test right-justification
  - Test spacing preservation
  - Test edge cases
  - _Requirements: 1.3, 1.4, 1.5, 4.1, 4.4_

- [x] 4. Update display function call sites
  - Update `draw_screen()` to pass `effective_col_width` and `config` to format functions
  - Ensure Series entry rendering uses new format
  - Ensure Season entry rendering uses new format
  - Verify truncation logic works with new format
  - _Requirements: 1.1, 1.2, 1.3, 1.4_

- [ ]* 4.1 Write integration tests for display rendering
  - Test series entries display with new format
  - Test season entries display with new format
  - Test counts are right-justified
  - Test styling is applied correctly
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 4.3_

- [x] 5. Handle terminal width constraints
  - Ensure count display doesn't cause overflow when terminal is narrow
  - Use `saturating_sub` for all width calculations
  - Test with minimum terminal widths
  - Ensure graceful degradation for very long names
  - _Requirements: 4.2_

- [ ]* 5.1 Write tests for width constraint handling
  - Test narrow terminal handling
  - Test very long entry names
  - Test width calculation edge cases
  - _Requirements: 4.2_

- [x] 6. Add error handling and logging
  - Add warning logs for invalid style values
  - Ensure database error fallback works with new format
  - Test configuration parse error handling
  - _Requirements: 2.4_

- [x] 7. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 8. Update documentation
  - Update inline code comments for modified functions
  - Ensure config.yaml comments explain new fields
  - _Requirements: 2.5, 3.1_
