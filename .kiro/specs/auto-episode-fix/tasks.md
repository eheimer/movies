# Implementation Plan

- [x] 1. Fix auto-episode entry logic in execute_menu_action

  - Locate the `execute_menu_action` function in `src/handlers.rs` within the `MenuAction::Edit` branch
  - Modify the conditional check for auto-filling episode numbers to include `&& season_number.is_some()` after the `edit_details.series.is_some()` check
  - Add `dirty_fields.insert(EpisodeField::EpisodeNumber);` immediately after the line that sets `edit_details.episode_number = next_episode.to_string();`
  - Ensure the changes maintain proper code formatting and alignment
  - _Requirements: 1.1, 1.2, 1.3, 2.1, 2.2_

- [x]\* 2. Verify the fix with manual testing
  - Build and run the application with `cargo run`
  - Test scenario: Episode with series but no season (verify no auto-fill)
  - Test scenario: Episode with series and season, empty episode number (verify auto-fill and dirty indicator)
  - Test scenario: Episode with series and season, existing episode number (verify no modification)
  - Test scenario: Save auto-filled episode number and verify persistence
  - _Requirements: 1.1, 1.2, 1.3, 2.1, 2.2, 2.3_
