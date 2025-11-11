# Implementation Plan

- [x] 1. Modify F4 key handler in Browse mode to restrict series assignment

  - Update the `KeyCode::F(4)` match arm in `handle_browse_mode` function in `src/handlers.rs`
  - Add conditional check to verify selected entry is an Episode using pattern matching
  - Add conditional check to verify episode has no series assigned using `edit_details.series.is_none()`
  - Only transition to SeriesSelect mode if both conditions are met
  - _Requirements: 1.1, 1.2, 1.3_

- [x] 2. Update UI header to conditionally display F4 key hint
- [x] 2.1 Modify draw_header function signature

  - Add `selected_entry: Option<&Entry>` parameter to `draw_header` function in `src/display.rs`
  - Add `edit_details: &EpisodeDetail` parameter to `draw_header` function
  - _Requirements: 2.1, 2.2, 2.3_

- [x] 2.2 Implement conditional F4 hint logic

  - In Browse mode instruction generation within `draw_header`, add logic to determine if F4 should be shown
  - Check if selected_entry is Some(Entry::Episode { .. })
  - Check if edit_details.series.is_none()
  - Only include "[F4] assign to series" in instruction string when both conditions are true
  - Apply this logic to all Browse mode instruction variants (series_selected, season_selected, series_filter_active, default)
  - _Requirements: 2.1, 2.2, 2.3_

- [x] 2.3 Update draw_screen to pass new parameters

  - In `draw_screen` function in `src/display.rs`, extract the selected entry using `entries.get(current_item)`
  - Update the call to `draw_header` to pass `selected_entry` and `edit_details` as arguments
  - _Requirements: 2.1, 2.2, 2.3_

- [x] 3. Verify F5 repeat action compatibility

  - Review the `can_repeat_action` function in `src/util.rs` to confirm it already prevents repeating series assignments on episodes with series
  - Manually test F5 behavior with the new F4 restrictions to ensure consistency
  - _Requirements: 3.1, 3.2, 3.3_

- [ ]\* 4. Manual testing and validation
  - Test F4 on unassigned episodes (should enter SeriesSelect mode, hint should show)
  - Test F4 on assigned episodes (should remain in Browse mode, hint should not show)
  - Test F4 on series and season entries (should remain in Browse mode, hint should not show)
  - Test F5 repeat action on unassigned episodes (should work)
  - Test F5 repeat action on assigned episodes (should not work, hint should not show)
  - Test UI consistency across TopLevel, Series, and Season view contexts
  - _Requirements: 1.1, 1.2, 1.3, 2.1, 2.2, 2.3, 3.1, 3.2, 3.3_
