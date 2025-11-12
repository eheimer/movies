# Implementation Plan

- [x] 1. Add database functions for bulk unwatch operations

  - Create three new functions in `src/database.rs`: `unwatch_all_in_season()`, `unwatch_all_in_series()`, and `unwatch_all_standalone()`
  - Each function executes a SQL UPDATE statement to set watched = false for episodes in the specified scope
  - Return `Result<(), Box<dyn std::error::Error>>` for error handling
  - _Requirements: 1.1, 1.4, 2.1, 2.4, 3.1, 3.4_

- [x] 2. Add UnwatchAll menu action and item definition

  - Add `UnwatchAll` variant to the `MenuAction` enum in `src/menu.rs`
  - Add menu item definition in `define_all_menu_items()` with label "Unwatch All", hotkey F7, and ContextMenu location
  - Implement availability logic in `is_item_available()` to make the action available in all contexts
  - _Requirements: 4.2, 4.3_

- [x] 3. Implement UnwatchAll action handler
  - Add match arm for `MenuAction::UnwatchAll` in `execute_menu_action()` function in `src/handlers.rs`
  - Use ViewContext to determine which database function to call (season, series, or standalone)
  - Reload entries based on current view context after updating database
  - Set mode to Browse and trigger redraw
  - _Requirements: 1.1, 1.2, 1.3, 2.1, 2.2, 2.3, 3.1, 3.2, 3.3, 4.1_
