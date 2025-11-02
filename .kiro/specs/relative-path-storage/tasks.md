# Implementation Plan

- [ ] 1. Create path resolution module with configurable root directory

  - Create new `src/path_resolver.rs` module with PathResolver struct
  - Implement executable directory detection using `std::env::current_exe()`
  - Implement configurable root directory support (from config.json)
  - Implement database path method that always uses executable directory
  - Implement strict path validation to ensure all paths are under configured root directory
  - Implement relative/absolute path conversion methods using configured root
  - _Requirements: 1.4, 2.1, 4.1, 4.2, 4.3, 4.4, 4.5_

- [ ] 2. Update configuration handling for configurable root directory

  - Add `root_dir` field to Config struct in `src/config.rs`
  - Modify config loading to support optional root_dir configuration
  - Update Config struct to resolve relative video directory path using configured root
  - Ensure config.json supports both root_dir and relative path format
  - _Requirements: 2.5, 2.6, 4.1, 4.2, 4.4_

- [ ] 3. Update database operations for relative path storage

  - Modify `src/database.rs` import_episode function to store relative paths
  - Add path validation before storing to ensure files are under configured root directory
  - Update database retrieval functions to work with relative paths
  - Ensure database file is always created in executable directory
  - _Requirements: 1.6, 2.1, 2.2, 3.6, 4.2_

- [ ] 4. Update handlers for relative path processing

  - Modify `scan_directory_for_videos` in `src/handlers.rs` to convert absolute paths to relative
  - Update `update_database_with_videos` to use relative path storage
  - Ensure video playback resolves relative paths to absolute before launching player
  - Add error handling for files outside configured root directory during directory scanning
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 4.2_

- [ ] 5. Integrate PathResolver throughout application

  - Update `src/main.rs` to initialize PathResolver with configurable root directory at startup
  - Pass PathResolver instance to modules that need path resolution
  - Update video player launch to resolve relative paths to absolute
  - Ensure database operations use executable directory for database file location
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.6, 4.3_

- [ ] 6. Update configuration file format

  - Add `root_dir` field to existing config.json for configurable root directory
  - Modify existing config.json to use relative path format for video directory
  - Ensure video_player path remains absolute (external tool)
  - _Requirements: 3.5, 4.1, 4.2_

- [ ] 7. Add core functionality tests
  - Write tests for PathResolver path conversion and validation with configurable root
  - Write tests for database file location in executable directory
  - Write integration test for directory scan → store → playback workflow
  - _Requirements: 1.1, 1.2, 1.3, 1.6, 2.1, 2.2, 4.1, 4.2_
