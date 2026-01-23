# Implementation Plan: Auto-select First Unwatched Episode

## Overview

This implementation adds auto-selection of the first unwatched episode when entering new browsing contexts. The work is organized into three main tasks: creating the utility function, integrating it into context entry points, and testing the complete behavior.

## Tasks

- [x] 1. Create utility function to find first unwatched entry
  - Add `find_first_unwatched_index` function to `src/handlers.rs`
  - Function takes a slice of Entry items and returns Option<usize>
  - For Episode entries: check watched status directly from database
  - For Series entries: use `get_series_episode_counts` to check for unwatched episodes
  - For Season entries: use `get_season_episode_counts` to check for unwatched episodes
  - Return the index of the first unwatched entry, or None if all are watched
  - _Requirements: 1.1, 1.2, 2.1, 2.2, 3.1, 3.2, 5.1, 5.2, 5.3_

- [x] 2. Integrate auto-selection into series entry
  - Locate the series entry handler in `handle_browse_mode` (KeyCode::Enter, Entry::Series branch)
  - After loading entries with `database::get_entries_for_series()`
  - Call `find_first_unwatched_index(&entries)` to get the optimal cursor position
  - Set `*current_item` to the returned index, or 0 if None
  - Ensure this happens before setting `*redraw = true`
  - _Requirements: 1.1, 1.2, 1.3_

- [x] 3. Integrate auto-selection into season entry
  - Locate the season entry handler in `handle_browse_mode` (KeyCode::Enter, Entry::Season branch)
  - After loading entries with `database::get_entries_for_season()`
  - Call `find_first_unwatched_index(&entries)` to get the optimal cursor position
  - Set `*current_item` to the returned index, or 0 if None
  - Ensure this happens before setting `*redraw = true`
  - _Requirements: 2.1, 2.2, 2.3_

- [x] 4. Integrate auto-selection into top level entry
  - Locate the top level entry handlers in `handle_browse_mode` (KeyCode::Esc branches that return to top level)
  - After loading entries with `database::get_entries()`
  - Call `find_first_unwatched_index(&entries)` to get the optimal cursor position
  - Set `*current_item` to the returned index, or 0 if None
  - Apply to both Esc handlers that return to top level
  - _Requirements: 3.1, 3.2, 3.3_

- [x] 5. Checkpoint - Manual testing
  - Build and run the application
  - Test entering a series with unwatched episodes - verify cursor positions correctly
  - Test entering a season with unwatched episodes - verify cursor positions correctly
  - Test returning to top level - verify cursor positions on first series with unwatched content
  - Test edge cases: all watched, empty contexts, filtered entries
  - Ensure manual cursor movements are preserved within contexts
  - Ask the user if any issues arise

- [ ] 6. Write unit tests for find_first_unwatched_index
  - Create test file `tests/auto_select_tests.rs`
  - Test empty list returns None
  - Test all watched entries returns None
  - Test first entry unwatched returns index 0
  - Test middle entry unwatched returns correct index
  - Test mixed entry types (Series, Season, Episode)
  - Test Series with unwatched episodes
  - Test Season with unwatched episodes
  - _Requirements: 1.1, 1.2, 2.1, 2.2, 3.1, 3.2, 5.1, 5.2, 5.3_

- [ ] 7. Write integration tests for auto-selection behavior
  - Add tests to `tests/integration_tests.rs` or create new test file
  - Test entering series with unwatched episodes positions cursor correctly
  - Test entering season with unwatched episodes positions cursor correctly
  - Test returning to top level positions cursor correctly
  - Test filtered entries are respected
  - Test edge cases: empty contexts, all watched, single entry
  - _Requirements: 1.1, 1.2, 1.3, 2.1, 2.2, 2.3, 3.1, 3.2, 3.3, 4.1, 4.2, 4.3, 5.1, 5.2, 5.3_

## Notes

- The implementation preserves existing navigation behavior
- Auto-selection only triggers on context entry, not during manual navigation
- The feature respects filtered entry lists
- No database schema changes required
- No configuration changes required
