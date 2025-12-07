# Design Document: Logging System

## Overview

This design implements a comprehensive logging system for the movies application. The system will provide configurable logging with multiple log levels (error, warn, info, debug), automatic log file management with optional archival of previous logs, and integration throughout the application to capture user actions, warnings, and errors.

The logging system will be implemented as a new module (`src/logger.rs`) that provides a simple API for logging at different levels. The system will handle log file creation, rotation, and formatting automatically.

## Architecture

### High-Level Components

1. **Logger Module** (`src/logger.rs`): Core logging functionality
   - Log file initialization and management
   - Log level filtering
   - Timestamp formatting
   - Thread-safe logging operations

2. **Configuration Extension** (`src/config.rs`): Add logging configuration fields
   - `log_file`: Optional custom log file path
   - `log_level`: Configurable log level (error, warn, info, debug)

3. **Application Integration**: Integrate logging throughout existing modules
   - User actions (playing videos, editing metadata, etc.)
   - Error conditions
   - Warning conditions
   - Debug information for edge cases

### Component Interaction

```
┌─────────────────┐
│  Application    │
│  (main.rs,      │
│   handlers.rs,  │
│   database.rs)  │
└────────┬────────┘
         │
         │ log_info(), log_error(), etc.
         │
         ▼
┌─────────────────┐
│  Logger Module  │
│  (logger.rs)    │
└────────┬────────┘
         │
         │ reads config
         │
         ▼
┌─────────────────┐
│  Config         │
│  (config.rs)    │
└─────────────────┘
```

## Components and Interfaces

### Logger Module

The logger module will provide a global, thread-safe logging interface using `lazy_static` and `Mutex`, similar to the database connection pattern already used in the application.

```rust
// Public API
pub fn initialize_logger(log_file: PathBuf, log_level: LogLevel) -> Result<()>;
pub fn log_error(message: &str);
pub fn log_warn(message: &str);
pub fn log_info(message: &str);
pub fn log_debug(message: &str);
pub fn prompt_save_existing_log(log_path: &Path) -> io::Result<bool>;
```

### Configuration Extension

Add two new fields to the `Config` struct:

```rust
pub struct Config {
    // ... existing fields ...
    
    #[serde(default = "default_log_file")]
    pub log_file: Option<String>,
    
    #[serde(default = "default_log_level")]
    pub log_level: String,
}
```

### Log Levels

The system will support four hierarchical log levels:

- **error**: Only error messages
- **warn**: Warning and error messages
- **info**: Info, warning, and error messages (default)
- **debug**: All messages including debug

## Data Models

### LogLevel Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
}
```

### LogEntry Format

Each log entry will follow this format:
```
[YYYY-MM-DD HH:MM:SS] [LEVEL] message
```

Example:
```
[2025-12-07 14:32:15] [INFO] Playing video: /path/to/video.mp4
[2025-12-07 14:32:20] [ERROR] Failed to open database: permission denied
```

### Archived Log Filename Format

When saving an existing log file, it will be renamed to:
```
YYYY-MM-DD_HH-MM-SS-<original_filename>
```

Example:
```
2025-12-07_14-30-00-movies.log
```

## Testing Strategy

### Test Cases

*A test case is a specific scenario that validates expected behavior of the system.*

#### Test Case 1: Default log location

When no log_file is configured in config.json, the system should use `~/.local/share/movies/movies.log` as the default location.
**Validates: Requirements 1.1, 1.3**

#### Test Case 2: Custom log location

When a log_file path is specified in config.json, the system should create and write logs to that custom location.
**Validates: Requirements 2.1, 2.3**

#### Test Case 3: Log directory creation

When the log file's parent directory does not exist, the system should create it before writing logs.
**Validates: Requirements 1.2**

#### Test Case 4: Log file archival prompt

When the application starts and a log file already exists, the system should prompt the user to save it, and upon confirmation, rename it with a timestamp prefix.
**Validates: Requirements 1.3, 1.4**

#### Test Case 5: Log file recreation

When the user declines to save the existing log file, the system should empty and recreate it.
**Validates: Requirements 1.5**

#### Test Case 6: Timestamp inclusion

When any log entry is written, it should include a timestamp in the format `[YYYY-MM-DD HH:MM:SS]`.
**Validates: Requirements 1.6**

#### Test Case 7: Error level filtering

When log_level is set to "error", only error-level messages should be written to the log file.
**Validates: Requirements 3.1**

#### Test Case 8: Warn level filtering

When log_level is set to "warn", both warn and error messages should be written to the log file.
**Validates: Requirements 3.2**

#### Test Case 9: Info level filtering

When log_level is set to "info", info, warn, and error messages should be written to the log file.
**Validates: Requirements 3.3**

#### Test Case 10: Debug level filtering

When log_level is set to "debug", all messages (debug, info, warn, error) should be written to the log file.
**Validates: Requirements 3.4**

#### Test Case 11: Invalid log level handling

When an invalid log_level value is provided in config.json, the system should default to "info" level and log a warning.
**Validates: Requirements 2.5**

#### Test Case 12: User action logging

When a user performs actions (play video, assign to series, save changes, delete video, rescan), the system should log info-level entries with relevant details.
**Validates: Requirements 4.1, 4.2, 4.3, 4.4, 4.5**

#### Test Case 13: Edge case debug logging

When edge cases occur during operation, the system should log debug-level entries with sufficient context for troubleshooting.
**Validates: Requirements 5.1, 5.2, 5.3**

#### Test Case 14: Warning logging

When non-critical errors or fallback behaviors occur, the system should log warn-level entries.
**Validates: Requirements 6.1, 6.2, 6.3**

#### Test Case 15: Error logging

When serious errors occur, the system should log error-level entries with details before handling the failure.
**Validates: Requirements 7.1, 7.2, 7.3**

### Testing Approach

1. **Unit Testing**: Test individual logger functions with different log levels and configurations
2. **Integration Testing**: Test logging integration with existing application flows
3. **Manual Testing**: Verify log file creation, archival prompts, and log content during actual application use

## Error Handling

### Log File Creation Errors

If the log file cannot be created (permissions, disk space, invalid path):
- Log an error to stderr
- Continue application execution without logging to file
- Display a warning message to the user

### Log Write Errors

If writing to the log file fails:
- Silently continue (don't crash the application)
- Optionally log to stderr for debugging

### Configuration Errors

If log configuration is invalid:
- Use default values (info level, standard location)
- Log a warning about the invalid configuration

## Implementation Notes

### Thread Safety

The logger will use a global `Mutex<File>` similar to the database connection pattern:

```rust
lazy_static! {
    static ref LOG_FILE: Mutex<Option<File>> = Mutex::new(None);
    static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Info);
}
```

### Startup Sequence

1. Read configuration (including log_file and log_level)
2. Determine log file path (custom or default)
3. Check if log file exists
4. If exists, prompt user to save (Y/n)
5. If yes, rename with timestamp
6. Create/recreate log file
7. Initialize logger with file handle and log level
8. Log application startup

### Integration Points

The following locations will be instrumented with logging:

**Info Level:**
- `handlers.rs`: User actions (play video, edit, save, delete, assign to series, rescan)
- `database.rs`: Database operations (create, update, delete records)

**Warn Level:**
- `config.rs`: Configuration issues (missing optional fields, invalid values)
- `database.rs`: Non-critical database issues (missing optional data)
- `path_resolver.rs`: Path resolution fallbacks

**Error Level:**
- `database.rs`: Database errors (connection failures, query errors)
- `main.rs`: Critical initialization failures
- `handlers.rs`: Failed operations (video playback errors, save failures)

**Debug Level:**
- `database.rs`: Query details, data validation edge cases
- `handlers.rs`: State transitions, unexpected input
- `path_resolver.rs`: Path resolution details

### Performance Considerations

- Log writes are synchronous but should be fast (file I/O)
- Mutex contention should be minimal (short critical sections)
- Log level filtering happens before formatting (avoid unnecessary work)
- No buffering (immediate writes for crash debugging)

## Dependencies

No new external dependencies required. The implementation will use:
- `std::fs`: File operations
- `std::io`: I/O operations
- `std::sync::Mutex`: Thread-safe access
- `lazy_static`: Global logger instance
- `chrono` (new): Timestamp formatting

Add to `Cargo.toml`:
```toml
chrono = "0.4"
```

## Migration Path

### Backward Compatibility

The logging system is purely additive:
- Existing configurations without log_file/log_level will use defaults
- No breaking changes to existing functionality
- Users can opt-in to custom log locations/levels

### Configuration Migration

When an existing config.json is loaded:
1. If log_file is missing, use default location
2. If log_level is missing, use "info" as default
3. Save updated config with new fields (existing pattern in config.rs)

## Future Enhancements

Potential future improvements (not in scope for this feature):
- Log rotation based on file size
- Compression of archived logs
- Structured logging (JSON format)
- Remote logging support
- Log viewer/analyzer tool
