# Implementation Plan

- [x] 1. Create splash screen module with ASCII art and core rendering logic
  - Create `src/splash.rs` file
  - Define ASCII art as a static string constant for "movies" in script font style
  - Define tagline as a static string constant
  - Implement `get_ascii_art()` and `get_tagline()` helper functions
  - Add module declaration to `src/lib.rs`
  - _Requirements: 2.1, 3.1_

- [x] 2. Implement text centering and rendering functions
  - Implement `center_text()` function to horizontally center text based on terminal width
  - Implement `render_splash()` function to display ASCII art and tagline with proper positioning
  - Use crossterm to query terminal dimensions
  - Position ASCII art at 40% from top of screen
  - Position tagline 2 lines below ASCII art, right-aligned with the artwork
  - Apply horizontal centering to both ASCII art lines and tagline
  - _Requirements: 2.3, 2.4, 3.2, 3.3, 3.4_

- [ ]* 2.1 Write unit tests for text centering logic
  - Test `center_text()` with various terminal widths (narrow, wide, exact match)
  - Test with empty text and text wider than terminal
  - Test with odd and even terminal widths
  - _Requirements: 2.4, 3.3_

- [x] 3. Implement main splash screen display function
  - Create `show_splash_screen()` public function
  - Clear terminal and hide cursor at start
  - Call `render_splash()` with terminal dimensions
  - Use `std::thread::sleep` for 2.5 second display duration
  - Handle terminal size query errors with default fallback (80x24)
  - Return Result type for error propagation
  - _Requirements: 1.1, 1.2, 1.4, 4.4_

- [ ]* 3.1 Write unit tests for splash screen components
  - Test ASCII art content contains "movies"
  - Test tagline content matches specification
  - Test error handling for terminal operations
  - _Requirements: 2.1, 3.1_

- [x] 4. Integrate splash screen into application startup
  - Add `mod splash;` to `src/lib.rs` if not already present
  - Import splash module in `src/main.rs`
  - Call `splash::show_splash_screen()?` after `terminal::init()` and before main event loop
  - Clear terminal after splash screen completes (before first browse screen render)
  - _Requirements: 1.1, 1.3, 4.1, 4.2, 4.3_

- [ ]* 4.1 Write integration test for startup sequence
  - Verify splash screen is called during application initialization
  - Verify terminal is properly cleared after splash completes
  - _Requirements: 1.1, 1.3, 4.3_

- [x] 5. Manual testing and edge case verification
  - Test on various terminal sizes (narrow, wide, tall, short)
  - Verify visual appearance and centering
  - Verify 2.5 second timing
  - Test transition to main browse screen
  - Document behavior for very narrow terminals (< 60 chars)
  - _Requirements: 1.2, 2.3, 2.4, 3.3_
