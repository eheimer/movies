# Implementation Plan

- [x] 1. Update menu system to support FirstLinePreferred location
  - Add `FirstLinePreferred` variant to `MenuLocation` enum in src/menu.rs
  - Add `get_first_line_preferred_items()` function to filter and return FirstLinePreferred items in priority order
  - Add `calculate_menu_helper_width()` helper function to calculate display width for menu items
  - _Requirements: 1.1, 2.1, 3.1, 4.1_

- [x] 2. Update rescan menu item definition
  - Change rescan menu item location from `MenuLocation::FirstLine` to `MenuLocation::ContextMenu` in `define_all_menu_items()`
  - Change rescan hotkey from `KeyCode::Char('l')` to `KeyCode::Char('s')`
  - _Requirements: 2.1, 2.2, 2.3_

- [x] 3. Remove CTRL+L rescan handler
  - Remove CTRL+L hotkey handling block from `handle_browse_mode()` in src/handlers.rs
  - Verify that [S] hotkey is automatically handled by existing menu hotkey logic
  - _Requirements: 2.5_

- [x] 4. Refactor draw_header to single line display
  - Modify `draw_header()` function in src/display.rs to build single instruction line
  - Remove all code that builds and displays second line of menu helpers
  - Remove the `instructions` vector and replace with single string building
  - _Requirements: 3.1, 3.2, 3.3_

- [x] 5. Implement header building algorithm with overflow logic
  - Start header with "[F1] Menu, "
  - Build hardcoded context-specific helpers string based on mode/context
  - Calculate remaining width for FirstLine-preferred items
  - Get FirstLine-preferred items and add those that fit within remaining width
  - Assemble final header string in correct order: [F1] Menu, hardcoded helpers, FirstLine-preferred items
  - _Requirements: 1.1, 4.1, 4.2, 4.3, 4.4, 4.5_

- [x] 6. Verify menu item availability and context awareness
  - Ensure FirstLine-preferred items respect `is_item_available()` checks
  - Test that items only appear when context supports them
  - Verify hardcoded helpers change appropriately based on mode/context
  - _Requirements: 4.1, 4.2_

- [x] 7. Test rescan functionality with new hotkey
  - Verify CTRL+L no longer triggers rescan
  - Verify [S] key triggers rescan in browse mode without opening F1 menu
  - Verify rescan appears in F1 context menu with [S] hotkey label
  - Verify rescan can be triggered from within F1 menu
  - _Requirements: 2.1, 2.2, 2.3, 2.4_

- [x] 8. Test menu display and overflow behavior
  - Verify [F1] Menu always appears first
  - Verify no second line of menu helpers is displayed
  - Test in narrow terminal windows to verify overflow behavior
  - Verify FirstLine-preferred items that overflow are accessible via F1 menu
  - Verify hardcoded helpers never overflow
  - _Requirements: 1.1, 3.1, 3.2, 4.3, 4.4_

- [x] 9. Test hotkey functionality across all contexts
  - Verify all menu item hotkeys work in browse mode without opening F1 menu
  - Verify hotkeys work regardless of whether item is displayed on first line or only in context menu
  - Test FirstLine-preferred items that overflow: verify their hotkeys still work
  - _Requirements: 2.2, 2.3, 2.4_

- [x] 10. Test across different modes and contexts
  - Test in all modes: Browse, Edit, Entry, SeriesSelect, SeriesCreate, Menu
  - Test in all view contexts: TopLevel, Series, Season
  - Verify appropriate menu helpers are displayed for each mode/context
  - Test filter mode to ensure simplified display works correctly
  - _Requirements: 1.1, 3.1, 4.1_
