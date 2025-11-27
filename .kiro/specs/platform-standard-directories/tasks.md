# Implementation Plan

- [x] 1. Add directories dependency and verify build
  - Add `directories = "5.0"` to `Cargo.toml` dependencies section
  - Run `cargo build` to verify dependency resolves correctly
  - _Requirements: Functional Requirement 1_

- [x] 2. Create centralized path management module
  - [x] 2.1 Create `src/paths.rs` with AppPaths struct
    - Create new file `src/paths.rs`
    - Implement `AppPaths` struct with `config_file: PathBuf` and `database_file: PathBuf` fields
    - Add `mod paths;` declaration to `src/main.rs`
    - _Requirements: Functional Requirements 1, 2_

  - [x] 2.2 Implement AppPaths::new() with directory creation
    - Implement `AppPaths::new()` method that returns `Result<Self, String>`
    - Use `ProjectDirs::from("", "", "movies")` to get platform-appropriate directories
    - Call `std::fs::create_dir_all()` for both config_dir and data_dir
    - Return descriptive error messages on failure including the path and OS error
    - Set config_file to `config_dir.join("config.json")`
    - Set database_file to `data_dir.join("videos.sqlite")`
    - _Requirements: Functional Requirements 1, 2, 5, 6; Non-Functional Requirements 2, 3_

- [x] 3. Update database module for configurable path
  - [x] 3.1 Add database path initialization
    - Add `static mut DB_PATH: Option<PathBuf> = None` to `src/database.rs`
    - Add `static INIT: Once = Once::new()` for thread-safe initialization
    - Implement `pub fn set_database_path(path: PathBuf)` that sets DB_PATH using INIT.call_once
    - _Requirements: Functional Requirement 6_

  - [x] 3.2 Update lazy_static DB_CONN initialization
    - Modify `lazy_static! { pub static ref DB_CONN: ... }` block
    - Read DB_PATH and expect with clear error if not set
    - Open connection using the configured path
    - Ensure all schema creation SQL executes on first connection (series, season, episode tables)
    - _Requirements: Functional Requirements 4, 6_

- [x] 4. Update config module to accept path parameter
  - Modify `load_or_create_config()` signature to accept `config_path: &PathBuf` parameter
  - Remove all `std::env::current_exe()` logic
  - Update file reading/writing to use the provided config_path
  - Keep existing default config creation logic
  - _Requirements: Functional Requirement 3_

- [x] 5. Update main function to initialize paths
  - [x] 5.1 Initialize AppPaths at application start
    - Call `paths::AppPaths::new()` at the beginning of `main()`
    - Handle `Err` case by printing error message to stderr
    - Print helpful message about checking permissions
    - Call `std::process::exit(1)` on error
    - _Requirements: Functional Requirements 2, 5; Non-Functional Requirements 2, 3_

  - [x] 5.2 Wire up config and database initialization
    - Pass `app_paths.config_file` to `load_or_create_config()`
    - Call `database::set_database_path(app_paths.database_file)` before any database operations
    - Ensure database connection is established (access DB_CONN) before entering main event loop
    - _Requirements: Functional Requirements 3, 4_

- [ ]* 6. Manual testing and verification
  - Delete any existing `~/.config/movies` and `~/.local/share/movies` directories
  - Run `cargo run` and verify directories are created automatically
  - Verify `config.json` is created in `~/.config/movies/`
  - Verify `videos.sqlite` is created in `~/.local/share/movies/`
  - Test application functionality (browse, scan, watch tracking)
  - Test error handling by creating read-only config directory and running application
  - _Requirements: All functional requirements, Success Criteria_

- [ ]* 7. Code cleanup and quality checks
  - Remove any old `config.json` or `videos.db` files from development directories
  - Update README.md or other documentation that references file locations
  - Run `cargo clippy` and address any warnings
  - Run `cargo fmt` to ensure consistent code formatting
  - _Requirements: Non-Functional Requirement 3_
