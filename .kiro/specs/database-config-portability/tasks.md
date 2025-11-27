# Implementation Plan

- [x] 1. Update Config module structure and methods
  - [x] 1.1 Modify Config struct to replace root_dir and path with db_location
    - Change the `Config` struct in `src/config.rs` to have `db_location: Option<String>` field
    - Remove `root_dir` and `path` fields
    - Keep all other fields (colors, video_extensions, video_player) unchanged
    - Update the `Default` implementation to set `db_location: None`
    - _Requirements: 2.3, 2.4_
  
  - [x] 1.2 Add config helper methods for database path management
    - Implement `get_database_path() -> Option<PathBuf>` method that converts `db_location` string to PathBuf
    - Implement `set_database_path(&mut self, path: PathBuf)` method that updates `db_location`
    - Implement `is_first_run() -> bool` method that returns true when `db_location` is None
    - _Requirements: 3.1, 3.2, 7.1_
  
  - [x] 1.3 Implement config migration logic from old format
    - Update `read_config()` function to detect old config format (has `root_dir` field)
    - When old format detected, attempt to find `videos.sqlite` in the old data directory
    - If found, set `db_location` to that path and save updated config
    - If not found, set `db_location` to None (first-run state)
    - _Requirements: 3.4_
  
  - [x] 1.4 Update config file writing to use new format
    - Ensure `read_config()` writes config with new format when creating default config
    - Remove `get_resolved_path()` method as it's no longer needed
    - _Requirements: 2.1, 2.2, 7.1_

- [x] 2. Refactor PathResolver to derive root from database location
  - [x] 2.1 Add new constructor from_database_path
    - Implement `from_database_path(db_path: &Path) -> Result<Self, PathResolverError>` constructor
    - Extract parent directory from database path as the root directory
    - Validate that database file exists
    - Canonicalize the root directory path
    - _Requirements: 6.1, 6.2_
  
  - [x] 2.2 Add new error types for database path validation
    - Add `DatabaseNotFound(PathBuf)` variant to `PathResolverError` enum
    - Add `InvalidDatabasePath(PathBuf)` variant for database without valid parent
    - Update `Display` implementation for new error types
    - _Requirements: 6.1_
  
  - [x] 2.3 Add method to expose root directory
    - Implement `get_root_dir(&self) -> &Path` method to return reference to root_dir
    - This will be used by rescan logic to determine scan directory
    - _Requirements: 5.2, 6.2_
  
  - [x] 2.4 Remove old constructor and unused methods
    - Remove or deprecate the `new(config_root_dir: Option<&str>)` constructor
    - Remove `resolve_config_path()` method as config no longer has a path field
    - Keep all other methods unchanged (to_relative, to_absolute, validate_path_under_root)
    - _Requirements: 6.4_

- [x] 3. Refactor database initialization to support delayed creation
  - [x] 3.1 Replace lazy_static with OnceLock for database connection
    - Replace `lazy_static! { pub static ref DB_CONN: Mutex<Connection> }` with `static DB_CONN: OnceLock<Mutex<Connection>>`
    - Remove the immediate initialization code from lazy_static block
    - _Requirements: 7.3_
  
  - [x] 3.2 Implement explicit database initialization function
    - Create `initialize_database(db_path: &Path) -> Result<(), Box<dyn std::error::Error>>` function
    - Create parent directory if it doesn't exist using `std::fs::create_dir_all`
    - Open or create database connection with `Connection::open(db_path)`
    - Execute all schema creation SQL (CREATE TABLE IF NOT EXISTS for series, season, episode)
    - Execute all data cleanup SQL operations
    - Store connection in `DB_CONN` using `OnceLock::set()`
    - Return error if database already initialized
    - _Requirements: 1.1, 4.4, 7.3_
  
  - [x] 3.3 Add database connection accessor function
    - Create `get_connection() -> &'static Mutex<Connection>` function
    - Return reference from `DB_CONN.get().expect("Database not initialized")`
    - _Requirements: 1.1_
  
  - [x] 3.4 Update all database functions to use new accessor
    - Replace all instances of `DB_CONN.lock()` with `get_connection().lock()` throughout `src/database.rs`
    - Ensure all functions work correctly with new initialization approach
    - Remove `set_database_path()` function as it's no longer needed
    - _Requirements: 1.1, 1.3_

- [x] 4. Update AppPaths module to remove database path management
  - [x] 4.1 Simplify AppPaths struct to only manage config file
    - Remove `database_file` field from `AppPaths` struct in `src/paths.rs`
    - Keep only `config_file` field
    - Update `new()` method to only create config directory (remove data directory creation)
    - _Requirements: 2.1_

- [x] 5. Implement first-run flow in main module
  - [x] 5.1 Create first_run_flow function for initial setup
    - Create new function `first_run_flow(config: &mut Config, config_path: &Path, resolver_option: Option<PathResolver>) -> io::Result<()>`
    - Display welcome message explaining first-run setup
    - Enter Entry mode to prompt user for video collection directory
    - Handle user input for directory path
    - Check if `videos.sqlite` exists in provided directory
    - If exists: call `initialize_database()` with existing database path
    - If not exists: call `initialize_database()` to create new database
    - Update config with `db_location` using `config.set_database_path()`
    - Save updated config to file
    - Create PathResolver from database path
    - Perform initial scan of the directory
    - Return to allow main loop to start
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 7.1, 7.2, 8.1, 8.2, 8.3_
  
  - [x] 5.2 Refactor main function to handle first-run scenario
    - Update `main()` function to read config first
    - Check if `config.is_first_run()` returns true
    - If first-run: initialize terminal, call `first_run_flow()`, then proceed to main loop
    - If not first-run: initialize database from `config.get_database_path()`, create PathResolver, load entries, start main loop
    - Handle errors gracefully with appropriate error messages
    - _Requirements: 7.1, 7.2, 7.4_
  
  - [x] 5.3 Update main_loop function signature and initialization
    - Remove automatic Entry mode when entries are empty (first-run flow handles this now)
    - Ensure PathResolver is properly passed through to handlers
    - Update any references to old config fields (root_dir, path)
    - _Requirements: 7.2_

- [x] 6. Update handlers for new scan and rescan behavior
  - [x] 6.1 Refactor Entry mode handler for first-run setup
    - Update `handle_entry_mode()` in `src/handlers.rs` to accept config and config_path parameters
    - When user presses Enter with a directory path:
      - Validate the directory exists
      - Check if `videos.sqlite` exists in that directory
      - If exists: display message "Found existing database, connecting..."
      - If not exists: display message "Creating new database..."
      - Call `database::initialize_database()` with appropriate path
      - Update config with `db_location` and save to file
      - Create PathResolver from database path
      - Perform scan of the directory
      - Load entries and switch to Browse mode
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 8.2, 8.3, 8.4_
  
  - [x] 6.2 Update rescan logic to use database parent directory
    - Update rescan handling in `handle_browse_mode()` for CTRL+L
    - Check if `config.db_location` is None (shouldn't happen but handle gracefully)
    - If None: enter Entry mode for first-run setup
    - If set: get root directory from PathResolver using `resolver.get_root_dir()`
    - Scan that directory automatically without prompting user
    - Display message showing scan directory
    - _Requirements: 5.1, 5.2, 5.3, 8.4, 8.5_
  
  - [x] 6.3 Update scan implementation to work with new architecture
    - Ensure scan functions use PathResolver correctly for relative path conversion
    - Update any hardcoded references to old config fields
    - Ensure scan properly imports videos using `database::import_episode_relative()`
    - _Requirements: 1.3, 6.3_

- [x] 7. Add user-facing messages and error handling
  - [x] 7.1 Implement informative messages for different scenarios
    - Add welcome message for first-run: "Welcome! Please enter the path to your video collection directory to get started."
    - Add message when existing database found: "Connected to existing database at {path}"
    - Add message when new database created: "Created new database and imported {count} videos"
    - Add message for rescan: "Rescanning {directory}..."
    - _Requirements: 7.2, 8.1_
  
  - [x] 7.2 Implement error handling for database and path issues
    - Handle database not found error with message: "Error: Database not found at {path}"
    - Handle invalid database path error with message: "Error: Invalid database path: {path}"
    - Handle permission errors with appropriate messages
    - Handle database initialization failures with specific error details
    - Ensure application exits gracefully on fatal errors
    - _Requirements: 7.4_

- [x] 8. Update display module for first-run and scan messages
  - [x] 8.1 Add display support for first-run state
    - Update `draw_screen()` in `src/display.rs` to handle first-run messaging
    - Display appropriate instructions when in Entry mode during first-run
    - Show database status (creating new vs. connecting to existing)
    - _Requirements: 7.2_
  
  - [x] 8.2 Update Entry mode display for new behavior
    - Update Entry mode display to show clearer instructions
    - Display whether database exists in entered directory
    - Show scan progress and results
    - _Requirements: 4.1, 4.2_

- [x] 9. Integration and end-to-end testing
  - [x] 9.1 Test first-run scenario with new database creation
    - Start application with no config file
    - Verify config is created with `db_location: null`
    - Enter directory path without existing database
    - Verify database is created in that directory
    - Verify config is updated with correct `db_location`
    - Verify videos are scanned and imported
    - Verify application enters Browse mode successfully
    - _Requirements: 1.1, 2.2, 4.4, 7.1, 7.2, 7.3_
  
  - [x] 9.2 Test first-run scenario with existing database
    - Start application with no config file
    - Enter directory path with existing `videos.sqlite`
    - Verify existing database is detected and used
    - Verify config is updated with correct `db_location`
    - Verify existing data is loaded
    - Verify rescan finds and imports new videos
    - _Requirements: 4.2, 4.3, 8.2, 8.3, 8.4, 8.5_
  
  - [x] 9.3 Test rescan behavior after initial setup
    - Start application with valid config and database
    - Trigger rescan with CTRL+L
    - Verify no directory prompt appears
    - Verify database's parent directory is scanned automatically
    - Verify new videos are imported
    - _Requirements: 5.1, 5.2, 5.3_
  
  - [x] 9.4 Test multi-computer portability scenario
    - Create database and import videos on one system
    - Copy video collection to different location
    - Start application on different system (no config)
    - Enter new directory path
    - Verify existing database is detected
    - Verify config is created with new path
    - Verify all existing data is accessible
    - Verify new videos are imported
    - _Requirements: 3.3, 3.4, 8.1, 8.2, 8.3, 8.4, 8.5_
  
  - [x] 9.5 Test config migration from old format
    - Create config file with old format (root_dir, path fields)
    - Start application
    - Verify migration logic detects old format
    - Verify database is found or first-run flow is triggered
    - Verify config is updated to new format
    - Verify application works correctly after migration
    - _Requirements: 3.4_
