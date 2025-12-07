# Implementation Plan

- [x] 1. Research and add video parsing dependencies
  - Research available Rust crates for MKV, MP4, and AVI parsing
  - Add selected crates to Cargo.toml
  - Verify crates are pure Rust without external dependencies
  - _Requirements: 5.1, 5.2, 5.3_

- [x] 2. Create video metadata extraction module
- [x] 2.1 Create video_metadata.rs module with core extraction function
  - Create `src/video_metadata.rs` file
  - Implement `extract_duration_seconds(file_path: &Path) -> Result<u64, Box<dyn std::error::Error>>`
  - Implement format detection based on file extension
  - Add error handling for unsupported formats
  - _Requirements: 1.1, 1.2, 1.3, 3.3_

- [x] 2.2 Implement MKV format handler
  - Implement `extract_mkv_duration(file_path: &Path) -> Result<u64, Box<dyn std::error::Error>>`
  - Parse MKV container metadata to extract duration
  - Handle parse errors gracefully
  - _Requirements: 1.1_

- [x] 2.3 Implement MP4 format handler
  - Implement `extract_mp4_duration(file_path: &Path) -> Result<u64, Box<dyn std::error::Error>>`
  - Parse MP4 container metadata to extract duration
  - Handle parse errors gracefully
  - _Requirements: 1.3_

- [x] 2.4 Implement AVI format handler
  - Implement `extract_avi_duration(file_path: &Path) -> Result<u64, Box<dyn std::error::Error>>`
  - Parse AVI container metadata to extract duration
  - Handle parse errors gracefully
  - _Requirements: 1.2_

- [x] 2.5 Implement database update helper function
  - Implement `extract_and_update_episode_length(episode_id: usize, file_path: &Path) -> Result<(), Box<dyn std::error::Error>>`
  - Call `extract_duration_seconds` to get duration
  - Update episode.length field in database with seconds value
  - Handle extraction failures without crashing
  - _Requirements: 1.4, 1.5_

- [x] 2.6 Implement duration formatting function
  - Implement `format_duration_hms(seconds: u64) -> String`
  - Format as "hh:mm:ss"
  - Handle durations less than one hour (e.g., "00:45:30")
  - Handle very long durations (> 24 hours)
  - _Requirements: 7.1, 7.2, 7.3, 7.4_

- [ ]* 2.7 Write unit tests for video metadata module
  - Test format detection logic
  - Test duration formatting with various inputs
  - Test error handling for invalid files
  - Test each format handler with sample files
  - _Requirements: 1.1, 1.2, 1.3, 7.1, 7.2_

- [x] 3. Integrate extraction into rescan operation
- [x] 3.1 Add database query for episodes with missing length
  - Add function to query episodes where length IS NULL OR length = 0
  - Return list of (episode_id, file_path) tuples
  - _Requirements: 2.1, 2.2_

- [x] 3.2 Update rescan_directory to extract missing lengths
  - After scanning for new files, query episodes with missing length
  - For each episode, call `extract_and_update_episode_length`
  - Log errors but continue processing remaining episodes
  - _Requirements: 2.3, 2.4, 2.5_

- [ ]* 3.3 Write integration tests for rescan
  - Create test database with episodes having NULL/0 lengths
  - Run rescan and verify lengths are updated
  - Verify rescan continues after extraction failures
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

- [-] 4. Integrate extraction into playback workflow
- [x] 4.1 Add extraction check before video playback
  - In `handle_browse_mode` when Enter key is pressed
  - Check if current episode has length = 0 or NULL
  - If so, call `extract_and_update_episode_length` for selected episode only
  - Proceed with playback regardless of extraction result
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [ ]* 4.2 Write integration tests for playback extraction
  - Select episode with NULL/0 length
  - Trigger playback
  - Verify length is extracted before player launches
  - Verify only selected episode is processed
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [-] 5. Integrate extraction into edit mode workflow
- [x] 5.1 Add extraction check when entering edit mode
  - In `handle_browse_mode` when transitioning to Edit mode
  - Check if current episode has length = 0 or NULL
  - If so, call `extract_and_update_episode_length` for selected episode only
  - Reload episode detail to show updated length
  - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

- [ ]* 5.2 Write integration tests for edit mode extraction
  - Select episode with NULL/0 length
  - Enter edit mode
  - Verify length is extracted and displayed
  - Verify only selected episode is processed
  - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

- [x] 6. Update display formatting to show hh:mm:ss
- [x] 6.1 Update display.rs to format length as hh:mm:ss
  - Locate where episode length is displayed in `src/display.rs`
  - Replace integer display with `format_duration_hms` call
  - Handle NULL/0 length values appropriately (show empty or "00:00:00")
  - Update both browse mode and edit mode displays
  - _Requirements: 7.1, 7.2, 7.3, 7.4_

- [ ]* 6.2 Write tests for display formatting
  - Test display with various duration values
  - Test display with NULL/0 length
  - Test display in browse and edit modes
  - _Requirements: 7.1, 7.2, 7.3, 7.4_

- [x] 7. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise
