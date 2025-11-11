# Implementation Plan

- [x] 1. Add LastAction enum and validation function to util.rs

  - Create LastAction enum with SeriesAssignment and SeasonAssignment variants
  - Implement format_display() method for LastAction
  - Create can_repeat_action() validation function that checks if selected entry is an Episode and not already assigned to the target series/season
  - _Requirements: 1.1, 1.2, 2.1, 2.2, 4.1, 4.2_

- [x] 2. Update main.rs to add last_action state

  - Add `let mut last_action: Option<LastAction> = None;` state variable in main_loop
  - Pass last_action reference to draw_screen function
  - Pass last_action mutable reference to handle_browse_mode function
  - Pass last_action mutable reference to handle_series_select_mode function
  - Pass last_action mutable reference to handle_edit_mode function
  - _Requirements: 1.2, 2.2_

- [x] 3. Modify display.rs to show last action and F5 menu option

  - Update HEADER_SIZE constant from 5 to 6
  - Add last_action parameter to draw_screen function
  - Calculate last_action_display string using can_repeat_action and format_display
  - Add last_action_display parameter to draw_header function
  - Print last action display at row 2 (between menu and filter)
  - Adjust filter line to row 4 (was row 3)
  - Conditionally add "[F5] Repeat action" to menu instructions when last_action_display is not empty
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 5.1, 5.2, 5.3_

- [x] 4. Add F5 key handler to handle_browse_mode in handlers.rs

  - Add KeyCode::F(5) match arm in handle_browse_mode
  - Check if can_repeat_action returns true
  - For SeriesAssignment: call database::assign_series with stored series_id
  - For SeasonAssignment: call database::create_season_and_assign with stored series_id, season_number, and episode_id
  - Reload entries based on current view_context
  - Update filtered_entries and trigger redraw
  - Do not modify last_action (preserve it for next repeat)
  - _Requirements: 1.1, 1.3, 2.1, 2.2, 4.3, 4.4_

- [x] 5. Update handle_series_select_mode to set last_action on series assignment

  - In KeyCode::Enter handler after successful database::assign_series call
  - Set last_action to LastAction::SeriesAssignment with series_id and series_name from the series vector
  - _Requirements: 1.2_

- [x] 6. Update handle_edit_mode to set last_action on season assignment
  - In KeyCode::F(2) handler after successful database::create_season_and_assign call
  - Check if both series and season_number are set in edit_details
  - Set last_action to LastAction::SeasonAssignment with series_id, series_name, season_id, and season_number
  - _Requirements: 2.2_
