# Implementation Plan

- [x] 1. Add ClearSeriesData menu action variant

  - Add `ClearSeriesData` variant to the `MenuAction` enum in `src/menu.rs`
  - _Requirements: 1.1, 1.3, 1.4_

- [x] 2. Define Clear Series Data menu item

  - Add new menu item to `define_all_menu_items()` function in `src/menu.rs`
  - Set label to "Clear Series Data"
  - Set hotkey to `KeyCode::F(6)`
  - Set action to `MenuAction::ClearSeriesData`
  - Set location to `MenuLocation::ContextMenu`
  - _Requirements: 1.1, 1.3, 1.4_

- [x] 3. Implement conditional visibility logic

  - Add new case for `MenuAction::ClearSeriesData` in `is_item_available()` function in `src/menu.rs`
  - Check if selected entry is an Episode
  - Check if episode has series OR season OR non-empty/non-zero episode number
  - Return true only if at least one field is populated
  - _Requirements: 1.1, 1.2_

- [x] 4. Create database function to clear series data

  - Add `clear_series_data()` function to `src/database.rs`
  - Function signature: `pub fn clear_series_data(episode_id: usize) -> Result<(), Box<dyn std::error::Error>>`
  - Execute SQL UPDATE to set series_id, season_id, and episode_number to NULL
  - _Requirements: 2.1, 2.2, 2.3_

- [x] 5. Implement action handler in execute_menu_action

  - Add new match case for `MenuAction::ClearSeriesData` in `execute_menu_action()` function in `src/handlers.rs`
  - Extract episode_id from filtered_entries at remembered_item position
  - Call `database::clear_series_data(episode_id)`
  - Reload entries based on current view_context (TopLevel, Series, or Season)
  - Update filtered_entries with reloaded entries
  - Set mode to Browse
  - Set redraw to true
  - _Requirements: 1.3, 1.4, 2.4, 2.5, 2.6_

- [ ]\* 6. Manual testing
  - Test clearing episode with all series data (series, season, episode number)
  - Test clearing episode with only series assigned
  - Test clearing episode with only season assigned
  - Test clearing episode with only episode number
  - Verify menu item is NOT visible for episodes without series data
  - Verify menu item is NOT visible for episode_number = "0"
  - Test F6 hotkey from Browse mode
  - Test clearing from TopLevel, Series, and Season view contexts
  - Verify entries reload correctly after clearing
  - _Requirements: All_
