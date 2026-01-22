# Implementation Plan: Progress Tracking

## Overview

This implementation plan breaks down the progress tracking feature into discrete coding tasks that build incrementally. Each task focuses on a specific component while ensuring integration with existing functionality.

## Tasks

- [x] 1. Database schema migration and core operations
  - Add new columns to episode table with proper migration
  - Implement progress update and retrieval functions
  - Add watched status management with timestamps
  - _Requirements: 2.1, 2.2, 2.3, 2.4_

- [x] 2. Configuration enhancement for watched threshold
  - Add watched_threshold field to Config struct with default value
  - Implement validation logic for threshold values (1-100%)
  - Update configuration file generation with inline documentation
  - _Requirements: 1.1, 1.2, 1.3, 1.4_

- [x] 3. Enhanced episode details with progress information
  - Extend EpisodeDetail struct to include progress fields
  - Update get_episode_detail function to retrieve progress data
  - Implement progress formatting for display (HH:MM:SS format)
  - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

- [x] 4. Video player integration with resume functionality
  - Modify run_video_player function to accept start time parameter
  - Implement resume position parameter passing to video player
  - Add fallback handling for players that don't support resume
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 8.1, 8.2_

- [x] 5. Player plugin trait and Celluloid implementation
  - Define PlayerPlugin trait with launch_command and get_final_position methods
  - Implement CelluloidPlugin with mpv watch-later file support
  - Add MD5 hash calculation for watch-later file lookup
  - Implement watch-later file parsing to extract playback position
  - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5_

- [x] 6. Integration with existing video playback workflow
  - Update video launch handler to use player plugin system
  - Query database for resume position before launch
  - Wait for player process to complete (blocking)
  - Retrieve final position from plugin after playback
  - Update database with new progress and check watched threshold
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 4.1, 4.2, 4.3, 4.4, 5.1, 5.2, 5.3, 5.4_

- [x] 7. UI integration for progress display
  - Update episode detail display to show progress information
  - Implement read-only display of last watched time and progress
  - Add proper formatting and conditional display logic
  - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6_

- [x] 8. Manual watched status integration
  - Update toggle_watched_status to handle progress reset
  - Ensure manual watched marking updates timestamp
  - Preserve existing watched status functionality
  - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [-] 9. Checkpoint - Manual testing and validation
  - Manually test database operations work correctly
  - Manually test video playback with resume functionality
  - Manually validate progress tracking and auto-watched behavior
  - Manually verify UI displays progress information correctly
  - Ask the user if questions arise during testing

## Notes

- Each task builds on previous functionality
- Database operations are implemented first to support all other features
- Player plugin architecture allows easy addition of new players in future
- Celluloid/mpv plugin uses watch-later files instead of active monitoring
- Video player integration blocks until playback completes
- UI changes are minimal and preserve existing behavior
- Progress tracking happens after playback completes, not during
- Testing focuses on manual validation rather than complex integration tests
- Unit tests are only written for easily testable, isolated functionality