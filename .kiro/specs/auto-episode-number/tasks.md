# Implementation Plan

- [x] 1. Implement database function to calculate next available episode number

  - Create `get_next_available_episode_number` function in `src/database.rs`
  - Function takes `series_id: usize` and `season_number: Option<usize>` as parameters
  - Query database for all episode numbers matching the series and season criteria
  - Implement gap-finding algorithm: find first missing number in sequence starting from 1
  - Return next available episode number as `Result<usize>`
  - Handle edge cases: empty result set returns 1, no gaps returns max + 1
  - _Requirements: 2.1, 2.2, 2.3, 4.1, 4.2, 4.3_

- [x] 2. Enhance browse mode F2 handler to support auto-fill behavior

  - Modify the `KeyCode::F(2)` handler in `handle_browse_mode` function in `src/handlers.rs`
  - After loading `edit_details`, check if series is assigned AND episode number is empty/zero
  - If conditions met, call `get_next_available_episode_number` with series_id and season_number
  - Pre-fill `edit_details.episode_number` with the calculated value converted to string
  - Set `edit_field` to `EpisodeField::EpisodeNumber` instead of `EpisodeField::Title`
  - Set `edit_cursor_pos` to the length of the pre-filled episode number string
  - If conditions not met, use existing behavior (cursor on Title field)
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.4, 2.5, 3.1, 3.2, 3.3_

- [x] 3. Add edit_field parameter to handle_browse_mode function signature

  - Update `handle_browse_mode` function signature to accept `edit_field: &mut EpisodeField`
  - Update the function call in `src/main.rs` to pass the `edit_field` mutable reference
  - This allows the browse mode handler to set the initial cursor field when entering edit mode
  - _Requirements: 1.1, 2.4_

- [x] 4. Manual testing and validation
  - Test basic auto-fill scenario: episode with series, no episode number
  - Test sequential numbering: verify next number after existing sequence
  - Test gap filling: verify first gap is filled when sequence has gaps
  - Test episode with existing number: verify normal edit mode behavior
  - Test episode without series: verify normal edit mode behavior
  - Test season-specific numbering: verify episode numbers are scoped to season
  - Test user modification: verify pre-filled value can be changed
  - Test empty series: verify first episode gets number 1
  - _Requirements: All requirements_
