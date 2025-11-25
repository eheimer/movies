# Implementation Plan

- [x] 1. Add filter clearing to Series navigation
  - Locate the ENTER key handler for Series entries in `handle_browse_mode` function
  - Add `search.clear();` immediately after the match arm begins, before loading series entries
  - Ensure the clear happens before `*entries = database::get_entries_for_series(*series_id)`
  - _Requirements: 1.1, 1.2_

- [x] 2. Add filter clearing to Season navigation
  - Locate the ENTER key handler for Season entries in `handle_browse_mode` function
  - Add `search.clear();` immediately after the match arm begins, before loading season entries
  - Ensure the clear happens before `*entries = database::get_entries_for_season(*season_id)`
  - _Requirements: 2.1, 2.2_

- [x] 3. Verify and update ESC navigation handlers
  - Review all three ESC key handlers in `handle_browse_mode` function
  - Ensure each ESC handler that changes view context includes `search.clear();`
  - Verify ESC handler for Season → Series navigation clears filter
  - Verify ESC handler for Series → TopLevel navigation clears filter
  - Verify ESC handler for TopLevel → Exit does not break with filter clearing
  - _Requirements: 3.1, 3.2, 3.3_

- [x] 4. Verify Episode navigation does not clear filter
  - Review the ENTER key handler for Episode entries in `handle_browse_mode` function
  - Confirm that no `search.clear();` call is present in the Episode entry handler
  - Ensure filter text persists when playing videos
  - _Requirements: 4.3_

- [ ]* 5. Manual testing of filter clearing behavior
  - Test filter + navigate into Series scenario
  - Test filter + navigate into Season scenario
  - Test filter + ESC from Season to Series scenario
  - Test filter + ESC from Series to TopLevel scenario
  - Test filter + ESC from TopLevel (exit) scenario
  - Test filter + navigate into Episode (filter should persist)
  - Test edge cases: empty filter, rapid navigation, no entries
  - _Requirements: 1.1, 1.2, 2.1, 2.2, 3.1, 3.2, 3.3, 4.3_
