# Implementation Plan

- [x] 1. Add status message state to main loop
  - Add `status_message: String` variable to `main_loop` function in `src/main.rs`
  - Initialize to empty string
  - Pass status message to `draw_screen()` function
  - _Requirements: 6.4_

- [x] 2. Update display module to draw status line
  - [x] 2.1 Add status_message parameter to draw_screen function
    - Update `draw_screen()` signature in `src/display.rs` to accept `status_message: &str`
    - Update all calls to `draw_screen()` in `src/main.rs` to pass the status message
    - _Requirements: 1.1, 1.3_
  
  - [x] 2.2 Implement draw_status_line function
    - Create `draw_status_line(message: &str) -> io::Result<()>` function in `src/display.rs`
    - Get terminal size and calculate status row (last row)
    - Clear the status line using `clear_line()`
    - Truncate message if longer than terminal width
    - Display message using `print_at()` if not empty
    - _Requirements: 1.1, 1.2, 1.4_
  
  - [x] 2.3 Call draw_status_line at end of draw_screen
    - Add call to `draw_status_line(status_message)?` at the end of `draw_screen()` function
    - Ensure it's called after all other drawing operations
    - _Requirements: 1.3_
  
  - [x] 2.4 Remove or deprecate load_videos function
    - Remove the `load_videos()` function from `src/display.rs` as it's no longer needed
    - Remove all calls to `load_videos()` from `src/handlers.rs` and `src/main.rs`
    - _Requirements: 6.1_

- [x] 3. Update handlers to accept and update status message
  - [x] 3.1 Update handle_entry_mode signature and implementation
    - Add `status_message: &mut String` parameter to `handle_entry_mode()` in `src/handlers.rs`
    - Update call site in `main_loop()` to pass `&mut status_message`
    - Set status to "Connected to existing database at <path>" when connecting to existing DB
    - Set status to "Creating new database..." when creating new DB
    - Set status to "Scanning <path>..." when starting scan
    - Set status to "Connected to existing database. Found N new videos" after scan with existing DB
    - Set status to "Created new database and imported N videos" after scan with new DB
    - Set `*redraw = true` after each status update
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 5.1, 5.2, 5.3_
  
  - [x] 3.2 Update handle_browse_mode signature and implementation
    - Add `status_message: &mut String` parameter to `handle_browse_mode()` in `src/handlers.rs`
    - Update call site in `main_loop()` to pass `&mut status_message`
    - Set status to "Playing video: <name>" when launching video player
    - Set `*redraw = true` after status update
    - _Requirements: 3.1, 3.2_
  
  - [x] 3.3 Update execute_menu_action for rescan
    - Add `status_message: &mut String` parameter to `execute_menu_action()` in `src/handlers.rs`
    - Update call site in `handle_menu_mode()` to pass status_message
    - In `MenuAction::Rescan` handler, set status to "Rescanning <path>..." before scan
    - After scan, set status to "Rescan complete. Found N new videos" if videos found
    - After scan, set status to "Rescan complete. No new videos found" if no videos found
    - Set `*redraw = true` after each status update
    - _Requirements: 2.1, 2.2, 2.3, 2.4_
  
  - [x] 3.4 Update handle_menu_mode signature
    - Add `status_message: &mut String` parameter to `handle_menu_mode()` in `src/handlers.rs`
    - Update call site in `main_loop()` to pass `&mut status_message`
    - Pass status_message to `execute_menu_action()`
    - _Requirements: 6.1_

- [x] 4. Update first_run_flow to return initial status
  - [x] 4.1 Update first_run_flow signature
    - Change return type from `io::Result<(Vec<Entry>, PathResolver)>` to `io::Result<(Vec<Entry>, PathResolver, String)>`
    - Calculate appropriate status message based on whether DB existed and how many videos were imported
    - Return status message as third element of tuple
    - _Requirements: 5.1, 5.2, 5.3_
  
  - [x] 4.2 Update main function to handle initial status
    - Update call to `first_run_flow()` to destructure three values including status message
    - Pass initial status message to `main_loop()`
    - For non-first-run path, create empty initial status: `let initial_status = String::new();`
    - Pass initial status to `main_loop()`
    - _Requirements: 6.4_

- [x] 5. Update main_loop signature
  - [x] 5.1 Add status_message parameter to main_loop
    - Add `mut status_message: String` parameter to `main_loop()` function signature
    - Update both call sites in `main()` to pass the status message
    - _Requirements: 6.4_

- [x] 6. Test and verify status line functionality
  - [x] 6.1 Test status line display and persistence
    - Start application and verify status line appears at bottom
    - Perform operations and verify status messages appear
    - Navigate through entries and verify status persists
    - Change modes and verify status persists
    - Resize terminal and verify status line adjusts
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 4.1, 4.2, 4.3, 4.4_
  
  - [x] 6.2 Test scan operation status messages
    - Perform first-run scan and verify "Scanning..." appears
    - Verify completion message shows correct count
    - Perform rescan and verify messages update correctly
    - Test scan with no new videos and verify "No new videos found" message
    - _Requirements: 2.1, 2.2, 2.3, 2.4_
  
  - [x] 6.3 Test video playback status message
    - Launch a video and verify "Playing video: <name>" appears
    - Test with videos with long names and verify truncation
    - Test with videos with special characters in names
    - _Requirements: 3.1, 3.2_
  
  - [x] 6.4 Test database operation status messages
    - Test first-run with new database creation
    - Test first-run with existing database
    - Verify appropriate messages appear for each scenario
    - _Requirements: 5.1, 5.2, 5.3_
