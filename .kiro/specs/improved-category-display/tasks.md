# Implementation Plan

- [x] 1. Implement database count functions
  - Add `get_series_episode_counts` function to `src/database.rs`
  - Add `get_season_episode_counts` function to `src/database.rs`
  - Use SQL COUNT and SUM aggregation for efficiency
  - Handle NULL watched status as unwatched
  - _Requirements: 1.2, 1.3, 2.2, 2.3_

- [ ]* 1.1 Write unit tests for database count functions
  - Test `get_series_episode_counts` with various series configurations
  - Test `get_season_episode_counts` with various season configurations
  - Test with empty series/seasons (0 episodes)
  - Test with all watched episodes
  - Test with all unwatched episodes
  - Test with mixed watched status
  - _Requirements: 1.2, 1.3, 1.4, 2.2, 2.3, 2.4_

- [x] 2. Implement display formatting functions
  - Add `format_series_display` function to `src/display.rs`
  - Add `format_season_display` function to `src/display.rs`
  - Format series as "[<series title>] <x> episodes (<y> unwatched)"
  - Format season as "<season title> - <x> episodes (<y> unwatched)"
  - Handle database query errors with fallback display
  - _Requirements: 1.1, 2.1, 3.1, 3.2, 3.3_

- [ ]* 2.1 Write unit tests for display formatting functions
  - Test `format_series_display` with different count combinations
  - Test `format_season_display` with different count combinations
  - Test with zero unwatched episodes
  - Test with special characters in titles
  - Test error handling with fallback display
  - _Requirements: 1.1, 1.4, 1.5, 2.1, 2.4, 2.5, 3.1, 3.2, 3.3_

- [x] 3. Update draw_screen to use new formatting
  - Modify the Series entry rendering in `draw_screen` function
  - Modify the Season entry rendering in `draw_screen` function
  - Call `format_series_display` for Series entries
  - Call `format_season_display` for Season entries
  - Ensure truncation is applied after formatting
  - Ensure color application works with new format
  - _Requirements: 1.1, 2.1, 3.1, 3.2, 3.3, 3.4_

- [ ]* 3.1 Write integration tests for display rendering
  - Create test database with series, seasons, and episodes
  - Verify display strings match expected format
  - Test with various episode counts and watched statuses
  - Test that filtering maintains correct counts
  - _Requirements: 1.1, 1.2, 1.3, 2.1, 2.2, 2.3, 3.4, 3.5_

- [x] 4. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
