# Implementation Plan

- [x] 1. Add Delete menu action to menu system
  - Add `Delete` variant to `MenuAction` enum in `src/menu.rs`
  - Add Delete menu item to `define_all_menu_items()` with no hotkey and ContextMenu location
  - Add availability logic to `is_item_available()` to show Delete only for episodes
  - _Requirements: 1.1, 2.1, 2.2, 4.1, 4.2, 4.3_

- [ ]* 1.1 Write unit tests for Delete menu availability
  - Test that Delete appears in menu for Episode entries
  - Test that Delete does not appear for Series entries
  - Test that Delete does not appear for Season entries
  - Test that Delete menu item has no hotkey assigned
  - _Requirements: 2.1, 2.2, 4.1, 4.2, 4.3_

- [x] 2. Implement database delete operation
  - Add `delete_episode(episode_id: usize)` function to `src/database.rs`
  - Use SQL DELETE statement to remove episode record
  - Return `Result<(), Box<dyn std::error::Error>>` for error handling
  - _Requirements: 1.2_

- [ ]* 2.1 Write unit tests for database deletion
  - Test that delete_episode removes the episode from database
  - Test that deleting non-existent episode handles error gracefully
  - _Requirements: 1.2_

- [x] 3. Add Delete action handler
  - Add `MenuAction::Delete` match arm to `execute_menu_action()` in `src/handlers.rs`
  - Extract episode_id from remembered item
  - Call `database::delete_episode(episode_id)`
  - Reload entries based on view_context (TopLevel, Series, or Season)
  - Update filtered_entries and return to Browse mode
  - _Requirements: 1.2, 1.3, 3.1, 3.2, 3.3_

- [ ]* 3.1 Write integration tests for delete and reload
  - Test deleting episode from top-level view reloads correctly
  - Test deleting episode from series view reloads correctly
  - Test deleting episode from season view reloads correctly
  - _Requirements: 1.3, 3.1, 3.2, 3.3_

- [x] 4. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
