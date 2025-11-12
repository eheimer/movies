# Implementation Plan

- [x] 1. Create menu module with core data structures

  - Create `src/menu.rs` file with MenuItem, MenuLocation, MenuAction, and MenuContext structures
  - Implement `define_all_menu_items()` function with all current menu items (Edit, Toggle Watched, Assign to Series, Repeat Action, Rescan)
  - Implement `is_item_available()` function with context-based filtering logic
  - Implement `get_available_menu_items()`, `get_first_line_items()`, and `get_context_menu_items()` functions
  - Add `mod menu;` declaration to `src/main.rs`
  - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

- [x] 2. Add Menu mode to application state

  - Add `Menu` variant to `Mode` enum in `src/util.rs`
  - Add `menu_selection: usize` and `remembered_item: usize` state variables to main loop in `src/main.rs`
  - Initialize new state variables with appropriate default values
  - _Requirements: 2.3_

- [x] 3. Update header display to show [F1] Menu

  - Modify `draw_header()` function in `src/display.rs` to display `[F1] Menu` on second line in Browse mode
  - Remove context-sensitive menu items (F2, F3, F4, F5) from second line
  - Keep first line unchanged
  - Update ESC text to `[ESC] close menu` when in Menu mode
  - _Requirements: 1.1, 1.2, 1.3, 5.1_

- [x] 4. Implement menu rendering

  - Add `draw_context_menu()` function to `src/display.rs`
  - Implement double-line border drawing using `╔═╗║╚╝` characters
  - Display each menu item with hotkey label on separate lines
  - Implement selection cursor highlighting using config colors
  - Calculate menu position (centered, below header)
  - Update `draw_screen()` to call `draw_context_menu()` when in Menu mode
  - _Requirements: 2.4, 2.5, 3.3_

- [x] 5. Implement menu event handling

  - Create `handle_menu_mode()` function in `src/handlers.rs`
  - Implement up/down arrow navigation with wrapping
  - Implement Enter key to execute selected menu item
  - Implement ESC key to close menu and return to Browse mode
  - Implement hotkey detection to directly execute menu items
  - Add action execution logic for each MenuAction variant (Edit, ToggleWatched, AssignToSeries, RepeatAction, Rescan)
  - _Requirements: 2.2, 3.1, 3.2, 3.4, 3.5, 4.1, 4.2, 4.3, 5.2, 5.3, 5.4_

- [x] 6. Update browse mode handler for F1 key

  - Modify `handle_browse_mode()` in `src/handlers.rs` to handle F1 key press
  - Set `mode = Mode::Menu`, store current item in `remembered_item`, reset `menu_selection` to 0
  - Remove F2, F3, F4, F5 key handling from browse mode (these will be handled in menu mode)
  - Keep CTRL+L handling in browse mode for rescan
  - _Requirements: 2.1, 2.2_

- [x] 7. Wire menu mode into main event loop

  - Add Menu mode case to event handling match statement in `src/main.rs`
  - Build MenuContext from current state
  - Get context menu items using `menu::get_context_menu_items()`
  - Call `handle_menu_mode()` with appropriate state parameters
  - Ensure mode transitions work correctly (Browse ↔ Menu)
  - _Requirements: 2.1, 2.2, 5.3_

- [ ]\* 8. Test menu functionality
  - Verify F1 opens menu with correct items based on context
  - Test arrow key navigation and wrapping
  - Test Enter key execution of menu items
  - Test hotkey execution of menu items
  - Test ESC closes menu and restores focus
  - Verify menu displays with double-line border
  - Test context-sensitive item availability (e.g., "assign to series" only for episodes without series)
  - Verify all menu actions execute correctly
  - _Requirements: All_
