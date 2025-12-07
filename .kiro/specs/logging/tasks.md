# Implementation Plan

- [x] 1. Add chrono dependency and create logger module structure
  - Add `chrono = "0.4"` to Cargo.toml dependencies
  - Create `src/logger.rs` with module structure
  - Add `mod logger;` to `src/lib.rs` or `src/main.rs`
  - Define `LogLevel` enum with Error, Warn, Info, Debug variants
  - Implement `PartialOrd` for LogLevel to enable hierarchical filtering
  - _Requirements: 2.2, 3.1, 3.2, 3.3, 3.4_

- [x] 2. Implement core logger functionality
  - Create global `LOG_FILE` and `LOG_LEVEL` using `lazy_static` and `Mutex`
  - Implement `initialize_logger(log_file: PathBuf, log_level: LogLevel)` function
  - Implement log file creation with parent directory creation
  - Implement timestamp formatting using chrono
  - Implement log entry formatting: `[YYYY-MM-DD HH:MM:SS] [LEVEL] message`
  - _Requirements: 1.2, 1.6, 2.2_

- [x] 3. Implement log level filtering and logging functions
  - Implement `log_error(message: &str)` function
  - Implement `log_warn(message: &str)` function
  - Implement `log_info(message: &str)` function
  - Implement `log_debug(message: &str)` function
  - Add log level filtering logic (only write if message level <= configured level)
  - Handle write errors gracefully (don't crash application)
  - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [x] 4. Implement log file archival functionality
  - Implement `prompt_save_existing_log(log_path: &Path) -> io::Result<bool>` function
  - Display prompt to user: "Save existing log file? (Y/n)"
  - Handle user input (Y/y/Enter = yes, N/n = no, Esc = no)
  - Implement log file renaming with timestamp format: `YYYY-MM-DD_HH-MM-SS-<filename>`
  - Handle file rename errors gracefully
  - _Requirements: 1.3, 1.4, 1.5_

- [x] 5. Extend configuration with logging fields
  - Add `log_file: Option<String>` field to Config struct with serde default
  - Add `log_level: String` field to Config struct with serde default
  - Implement `default_log_file()` returning None (use standard location)
  - Implement `default_log_level()` returning "info"
  - Add `parse_log_level(level_str: &str) -> LogLevel` helper function
  - Handle invalid log level values by defaulting to Info
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

- [x] 5.1 Write unit tests for configuration
  - Test default log file location when not configured
  - Test custom log file location when configured
  - Test default log level when not configured
  - Test valid log level parsing (error, warn, info, debug)
  - Test invalid log level defaults to info
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

- [x] 6. Integrate logger initialization in main.rs
  - Determine log file path (custom from config or default `~/.local/share/movies/movies.log`)
  - Check if log file exists before initialization
  - Call `prompt_save_existing_log()` if file exists
  - Rename existing log file if user confirms
  - Parse log level from config
  - Call `initialize_logger()` with path and level
  - Log warning if invalid log level was provided
  - Log application startup message at info level
  - Handle logger initialization errors gracefully
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 2.4, 2.5_

- [x] 7. Add info-level logging for user actions in handlers.rs
  - Log when user plays a video (include video path/name)
  - Log when user assigns video to series/season (include video and series/season details)
  - Log when user saves metadata changes (include changed fields)
  - Log when user deletes a video (include video details)
  - Log when user performs rescan operation (start and completion)
  - Log when user toggles watched status
  - Log when user creates new series
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [x] 8. Add warn-level logging for non-critical issues
  - Log warnings in config.rs for configuration issues
  - Log warnings in database.rs for missing optional data
  - Log warnings in path_resolver.rs for path resolution fallbacks
  - Log warning when invalid log level is provided
  - _Requirements: 6.1, 6.2, 6.3_

- [x] 9. Add error-level logging for serious errors
  - Log errors in database.rs for database operation failures
  - Log errors in main.rs for critical initialization failures
  - Log errors in handlers.rs for failed operations (video playback, save failures)
  - Include error details and context in error messages
  - _Requirements: 7.1, 7.2, 7.3_

- [x] 10. Add debug-level logging for edge cases
  - Log debug information in database.rs for query details and data validation
  - Log debug information in handlers.rs for state transitions and unexpected input
  - Log debug information in path_resolver.rs for path resolution details
  - Include sufficient context data for troubleshooting
  - _Requirements: 5.1, 5.2, 5.3_

- [x] 11. Write unit tests for logger module
  - Test log level filtering (error only logs errors, info logs info/warn/error, etc.)
  - Test timestamp formatting
  - Test log entry formatting
  - Test log file creation and directory creation
  - Test handling of write errors
  - _Requirements: 1.2, 1.6, 3.1, 3.2, 3.3, 3.4_

- [x] 12. Write integration tests for logging
  - Test end-to-end logging with different log levels
  - Test log file archival flow
  - Test logging during user actions
  - Verify log file contents match expected format
  - _Requirements: 1.3, 1.4, 1.5, 4.1, 4.2, 4.3, 4.4, 4.5_

- [x] 13. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
