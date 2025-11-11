# Implementation Plan

- [x] 1. Add dirty color configuration to Config struct

  - Add `dirty_fg` and `dirty_bg` fields to the `Config` struct in `src/config.rs`
  - Update the `Default` implementation to set `dirty_fg: "Black"` and `dirty_bg: "White"`
  - _Requirements: 2.2, 2.3, 2.4_

- [x] 2. Add dirty state tracking to main application

  - Add `use std::collections::HashSet;` import to `src/main.rs`
  - Add `original_edit_details: Option<EpisodeDetail>` state variable in main loop
  - Add `dirty_fields: HashSet<EpisodeField>` state variable in main loop
  - Initialize both variables appropriately at application start
  - _Requirements: 1.1, 1.2, 4.3, 4.6_

- [x] 3. Update draw_header to conditionally display save changes menu

  - Modify `draw_header` function signature in `src/display.rs` to accept `is_dirty: bool` parameter
  - Update EDIT mode instructions to conditionally append "[F2] save changes" only when `is_dirty` is true
  - Update all call sites of `draw_header` to pass the `is_dirty` parameter
  - _Requirements: 4.1, 4.2_

- [x] 4. Update draw_detail_window to highlight dirty field names

  - Modify `draw_detail_window` function signature in `src/display.rs` to accept `dirty_fields: &HashSet<EpisodeField>` and `config: &Config` parameters
  - Update field name rendering logic to check if field is in `dirty_fields`
  - Apply dirty colors to field names when in EDIT mode and field is dirty
  - Update all call sites of `draw_detail_window` to pass the new parameters
  - _Requirements: 1.1, 1.2, 3.1, 3.4_

- [x] 5. Update draw_screen to pass dirty state to child functions

  - Modify `draw_screen` function signature in `src/display.rs` to accept `dirty_fields: &HashSet<EpisodeField>` parameter
  - Calculate `is_dirty` as `!dirty_fields.is_empty()`
  - Pass `is_dirty` to `draw_header` call
  - Pass `dirty_fields` and `config` to `draw_detail_window` call
  - Update the call site in `src/main.rs` to pass `&dirty_fields`
  - _Requirements: 1.1, 4.1_

- [x] 6. Implement dirty field detection in handle_edit_mode

  - Modify `handle_edit_mode` function signature in `src/handlers.rs` to accept `original_edit_details: &EpisodeDetail` and `dirty_fields: &mut HashSet<EpisodeField>` parameters
  - After each field modification (Char, Backspace, Delete, +, -), compare current value with original value
  - Insert field into `dirty_fields` if values differ, remove if they match
  - Handle special case for Season field comparison using `season_number` vs `original_edit_details.season`
  - Clear `dirty_fields` when saving (F2) or canceling (Esc)
  - Update the call site in `src/main.rs` to pass the new parameters
  - _Requirements: 1.1, 1.3, 3.2, 3.3, 4.3, 4.4, 4.5_

- [x] 7. Initialize dirty state when entering EDIT mode

  - In `handle_browse_mode` function in `src/handlers.rs`, when F2 is pressed
  - Set `original_edit_details = Some(edit_details.clone())` before entering EDIT mode
  - Clear `dirty_fields` before entering EDIT mode
  - Pass `original_edit_details` and `dirty_fields` as mutable parameters to `handle_browse_mode`
  - Update the call site in `src/main.rs` to pass these parameters
  - _Requirements: 4.6_

- [x] 8. Clear dirty state when exiting EDIT mode
  - In `src/main.rs`, after `handle_edit_mode` returns and mode changes from Edit to Browse
  - Set `original_edit_details = None`
  - Clear `dirty_fields` with `dirty_fields.clear()`
  - _Requirements: 4.4, 4.5_
