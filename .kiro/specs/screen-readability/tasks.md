# Implementation Plan

- [x] 1. Extend Config structure with new color fields
  - Add color fields for watched indicator, new episodes, invalid episodes, series, seasons, episodes, and status line
  - Implement default values for all new fields
  - Update Config::default() to include all new color options
  - _Requirements: 3.1, 3.2, 3.5, 7.1, 7.2_

- [x] 1.1 Write unit tests for Config defaults
  - **Test Case 9: Missing config field defaults**
  - **Validates: Requirements 3.2, 7.2**

- [x] 2. Enhance color parsing system
  - Extend string_to_color to support "Reset" as terminal default
  - Add helper functions for consistent color application
  - Implement fallback behavior for invalid color names
  - _Requirements: 7.3, 7.4, 7.5_

- [x] 2.1 Write unit tests for color parsing
  - **Test Case 10: Invalid color fallback**
  - **Validates: Requirements 3.4, 7.4**

- [x] 3. Implement episode state detection
  - Create EpisodeState enum (Normal, Watched, New, Invalid)
  - Implement determine_episode_state function with priority logic
  - Add file existence checking using PathResolver
  - Add title-equals-filename detection
  - _Requirements: 2.1, 2.2_

- [x] 3.1 Write unit tests for state detection
  - **Test Case 4: New episode color application**
  - **Test Case 5: Invalid episode color application**
  - **Validates: Requirements 2.1, 2.2**

- [x] 4. Implement watched indicator formatting
  - Create format_episode_with_indicator function
  - Add watched indicator from config with proper spacing
  - Handle both watched and unwatched states
  - _Requirements: 1.1, 1.2, 1.4_

- [x] 4.1 Write unit tests for indicator formatting
  - **Test Case 1: Watched indicator presence**
  - **Test Case 2: Unwatched indicator absence**
  - **Test Case 3: Watched indicator distinctness**
  - **Validates: Requirements 1.1, 1.2, 1.4**

- [x] 5. Update entry display logic for type-based coloring
  - Modify draw_screen to apply series colors to series entries
  - Apply season colors to season entries
  - Apply episode colors to normal episode entries
  - Ensure selection highlighting overrides type colors
  - _Requirements: 4.1, 4.2, 4.3, 4.4_

- [x] 5.1 Write unit tests for entry coloring
  - **Test Case 11: Series entry coloring**
  - **Test Case 12: Season entry coloring**
  - **Test Case 13: Episode entry coloring**
  - **Test Case 14: Selection highlight override**
  - **Validates: Requirements 4.1, 4.2, 4.3, 4.4**

- [x] 6. Integrate state-based coloring for episodes
  - Call determine_episode_state during episode rendering
  - Apply new episode colors when state is New
  - Apply invalid episode colors when state is Invalid
  - Apply watched indicator when state is Watched
  - Handle state transitions correctly
  - _Requirements: 2.1, 2.2, 2.5, 2.6_

- [x] 6.1 Write unit tests for state-based coloring
  - **Test Case 6: State transition from new to normal**
  - **Test Case 7: State transition from invalid to normal**
  - **Validates: Requirements 2.5, 2.6**

- [x] 7. Enhance status line display
  - Apply status line colors from config
  - Ensure status line uses configured foreground and background
  - Handle empty status line with neutral appearance
  - _Requirements: 6.1, 6.2, 6.4_

- [x] 8. Update menu display coloring
  - Ensure menu selection uses current_fg/current_bg
  - Verify filter mode highlighting uses configured colors
  - _Requirements: 5.1, 5.5_

- [x] 8.1 Write unit tests for menu highlighting
  - **Test Case 15: Menu selection highlighting**
  - **Validates: Requirements 5.5**

- [x] 9. Update config file handling
  - Ensure read_config populates missing color fields with defaults
  - Update save_config to include all new color fields
  - Test with existing config files to ensure backward compatibility
  - _Requirements: 3.1, 3.2, 3.5_

- [x] 9.1 Write unit tests for config loading
  - **Test Case 8: Config color loading**
  - **Validates: Requirements 3.1, 4.5, 6.4**

- [x] 10. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 11. Write integration tests
  - Test config reload with custom colors
  - Test state transitions (new to normal, invalid to normal)
  - Test file system integration (file creation/deletion)
  - Test watched status toggling with visual updates
