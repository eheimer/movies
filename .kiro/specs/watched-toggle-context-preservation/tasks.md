# Implementation Plan

- [x] 1. Add ViewContext enum to util.rs

  - Create a new enum `ViewContext` with three variants: `TopLevel`, `Series { series_id: usize }`, and `Season { season_id: usize }`
  - Add appropriate derive macros (Debug, Clone)
  - _Requirements: 4.1, 4.2_

- [x] 2. Update main loop to track and maintain view context

  - [x] 2.1 Add view_context variable to main_loop function

    - Initialize `view_context` to `ViewContext::TopLevel` at the start of main_loop
    - _Requirements: 4.1_

  - [x] 2.2 Pass view_context to handle_browse_mode
    - Add `view_context: &mut ViewContext` parameter to handle_browse_mode function signature
    - Update the call to handle_browse_mode in main_loop to pass `&mut view_context`
    - _Requirements: 4.2_

- [x] 3. Update browse mode handler to use view context

  - [x] 3.1 Update Enter key handler to set view context

    - When Enter is pressed on a Series entry, set `*view_context = ViewContext::Series { series_id }`
    - When Enter is pressed on a Season entry, set `*view_context = ViewContext::Season { season_id }`
    - _Requirements: 4.3, 1.2, 2.2_

  - [x] 3.2 Update Esc key handlers to restore previous view context

    - When navigating back from Season view, set view context to Series view using the series_id from edit_details
    - When navigating back from Series view, set view context to TopLevel
    - _Requirements: 4.3_

  - [x] 3.3 Update F3 handler to reload entries based on view context
    - Replace the hardcoded `database::get_entries()` call with a match statement on view_context
    - For `ViewContext::TopLevel`, call `database::get_entries()`
    - For `ViewContext::Series { series_id }`, call `database::get_entries_for_series(series_id)`
    - For `ViewContext::Season { season_id }`, call `database::get_entries_for_season(season_id)`
    - _Requirements: 1.1, 2.1, 3.1, 4.2_

- [x] 4. Verify the fix with manual testing
  - Test toggling watched status in series view maintains series context
  - Test toggling watched status in season view maintains season context
  - Test toggling watched status at top level maintains top-level context
  - Test that navigation (Enter/Esc) correctly updates view context
  - _Requirements: 1.1, 1.2, 1.3, 2.1, 2.2, 2.3, 3.1, 3.2, 3.3_
