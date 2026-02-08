# Implementation Plan: Online Torrent Search

## Overview

This implementation adds torrent search functionality to the video library manager by integrating the `magneto` crate, adding two new modes to the state machine, and implementing UI components for search input and results display.

## Tasks

- [x] 1. Add dependencies and create torrent search module
  - Add `magneto = "0.2"` and `tokio = { version = "1", features = ["full"] }` to Cargo.toml
  - Create `src/torrent_search.rs` with `TorrentResult` struct
  - Implement `search_torrents()` function using magneto with PirateBay provider
  - Implement `open_magnet_link()` function with OS-specific commands
  - Add logging for all operations
  - _Requirements: 3.1, 3.2, 3.3, 6.1, 6.2_

- [x] 2. Extend mode system and add state variables
  - Add `TorrentSearchInput` and `TorrentSearchResults` variants to `Mode` enum in `src/util.rs`
  - Add state variables to `src/main.rs`: `search_query`, `torrent_results`, `selected_torrent_result`
  - _Requirements: 1.3, 2.1_

- [x] 3. Add menu item and action
  - Add `SearchOnline` variant to `MenuAction` enum in `src/menu.rs`
  - Add "Search Online" menu item with F8 hotkey in `get_menu_items()`
  - Implement availability check in `is_item_available()` to show only in Browse mode
  - _Requirements: 1.1, 1.2_

- [x] 4. Implement menu action handler
  - Add `MenuAction::SearchOnline` case in `execute_menu_action()` in `src/handlers.rs`
  - Transition to `TorrentSearchInput` mode and initialize search query
  - _Requirements: 1.3_

- [x] 5. Implement TorrentSearchInput mode handler
  - Create `handle_torrent_search_input()` function in `src/handlers.rs`
  - Handle character input, backspace, Enter (execute search), and ESC (cancel)
  - Execute async search and transition to results mode on Enter
  - Add logging for search initiation
  - _Requirements: 2.2, 2.3, 2.4, 2.5, 3.1, 3.2, 3.3_

- [x] 6. Implement TorrentSearchResults mode handler
  - Create `handle_torrent_search_results()` function in `src/handlers.rs`
  - Handle up/down arrow navigation, Enter (open magnet link), and ESC (cancel)
  - Call `open_magnet_link()` on Enter and update status message
  - Add logging for result selection and magnet link opening
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5, 6.1, 6.2, 6.3, 6.4, 6.5_

- [x] 7. Implement TorrentSearchInput display
  - Create `draw_torrent_search_input()` function in `src/display.rs`
  - Display header "Search Online - The Pirate Bay"
  - Display input field with current query
  - Display instructions: "Enter: Search | ESC: Cancel"
  - _Requirements: 2.1, 2.2_

- [x] 8. Implement TorrentSearchResults display
  - Create `draw_torrent_search_results()` function in `src/display.rs`
  - Display header "Search Results (Top 5)"
  - Display table with columns: Title, Uploaded, Size, Seeds, Leeches
  - Highlight selected row
  - Display instructions: "↑↓: Navigate | Enter: Download | ESC: Cancel"
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 5.1_

- [x] 9. Integrate handlers and display into main loop
  - Add mode handler calls in event loop for `TorrentSearchInput` and `TorrentSearchResults`
  - Add display calls in render section for both new modes
  - _Requirements: All_

- [x] 10. Test and verify functionality
  - Test search with various queries
  - Test navigation in results
  - Test magnet link opening
  - Test cancellation at each stage
  - Verify logging output
  - Test error handling (no results, network errors, magnet link errors)
  - _Requirements: All_

## Notes

- The `magneto` crate requires tokio runtime, which needs to be integrated into the main event loop
- Async search execution may require spawning a task or using `block_on()` depending on the main loop structure
- OS-specific magnet link opening commands: `xdg-open` (Linux), `open` (macOS), `start` (Windows)
- All operations should be logged using the existing logger module
- Error messages should be displayed in the status line and logged
