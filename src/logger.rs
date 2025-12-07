use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use chrono::Local;
use lazy_static::lazy_static;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

/// Log levels in hierarchical order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
}

lazy_static! {
    static ref LOG_FILE: Mutex<Option<File>> = Mutex::new(None);
    static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Info);
}

/// Initialize the logger with a log file path and log level
pub fn initialize_logger(log_file: PathBuf, log_level: LogLevel) -> io::Result<()> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = log_file.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create or truncate the log file
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_file)?;

    // Set the global log file and log level
    *LOG_FILE.lock().unwrap() = Some(file);
    *LOG_LEVEL.lock().unwrap() = log_level;

    Ok(())
}

/// Write a log entry with timestamp and level
fn write_log(level: LogLevel, message: &str) {
    // Check if this message should be logged based on configured level
    let configured_level = *LOG_LEVEL.lock().unwrap();
    if level > configured_level {
        return;
    }

    // Format timestamp
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    
    // Format level
    let level_str = match level {
        LogLevel::Error => "ERROR",
        LogLevel::Warn => "WARN",
        LogLevel::Info => "INFO",
        LogLevel::Debug => "DEBUG",
    };

    // Format log entry
    let log_entry = format!("[{}] [{}] {}\n", timestamp, level_str, message);

    // Write to log file
    if let Some(ref mut file) = *LOG_FILE.lock().unwrap() {
        let _ = file.write_all(log_entry.as_bytes());
        let _ = file.flush();
    }
}

/// Log an error message
pub fn log_error(message: &str) {
    write_log(LogLevel::Error, message);
}

/// Log a warning message
pub fn log_warn(message: &str) {
    write_log(LogLevel::Warn, message);
}

/// Log an info message
pub fn log_info(message: &str) {
    write_log(LogLevel::Info, message);
}

/// Log a debug message
pub fn log_debug(message: &str) {
    write_log(LogLevel::Debug, message);
}

/// Get access to the log file for testing purposes
/// This is only used in integration tests to flush and close the file
pub fn get_log_file_for_test() -> &'static Mutex<Option<File>> {
    &LOG_FILE
}

/// Prompt the user to save an existing log file
/// Returns Ok(true) if user wants to save, Ok(false) if not, or an error
pub fn prompt_save_existing_log(log_path: &Path) -> io::Result<bool> {
    use std::time::Duration;
    
    // Display prompt
    print!("Save existing log file? (Y/n): ");
    io::stdout().flush()?;

    // Read user input
    let result = loop {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                // Yes: Y, y, or Enter
                KeyCode::Char('Y') | KeyCode::Char('y') | KeyCode::Enter => {
                    println!("Y");
                    
                    // Generate timestamp for archived filename
                    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
                    
                    // Get the original filename
                    let filename = log_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .ok_or_else(|| io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Invalid log file path"
                        ))?;
                    
                    // Create new filename with timestamp
                    let archived_filename = format!("{}-{}", timestamp, filename);
                    
                    // Get the parent directory
                    let parent = log_path.parent().ok_or_else(|| io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Log file has no parent directory"
                    ))?;
                    
                    // Create full path for archived file
                    let archived_path = parent.join(archived_filename);
                    
                    // Rename the file
                    std::fs::rename(log_path, &archived_path)?;
                    
                    break Ok(true);
                }
                // No: N, n, or Esc
                KeyCode::Char('N') | KeyCode::Char('n') | KeyCode::Esc => {
                    println!("n");
                    break Ok(false);
                }
                // Ignore other keys
                _ => continue,
            }
        }
    };
    
    // Clear any pending events from the buffer to prevent them from being
    // processed by the main application loop
    while event::poll(Duration::from_millis(0))? {
        event::read()?;
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use tempfile::TempDir;

    /// Test log level filtering - error level only logs errors
    /// Validates: Requirements 3.1
    #[test]
    #[serial_test::serial]
    fn test_log_level_error_filtering() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let log_file = temp_dir.path().join("test_error.log");

        // Initialize logger with Error level
        initialize_logger(log_file.clone(), LogLevel::Error)
            .expect("Failed to initialize logger");

        // Log messages at different levels with unique identifiers
        log_error("ERROR_TEST_1");
        log_warn("WARN_TEST_1");
        log_info("INFO_TEST_1");
        log_debug("DEBUG_TEST_1");

        // Explicitly flush and close the file
        {
            let mut guard = LOG_FILE.lock().unwrap();
            if let Some(ref mut file) = *guard {
                file.flush().expect("Failed to flush");
            }
            *guard = None;
        }
        thread::sleep(Duration::from_millis(100));

        // Read log file contents
        let contents = fs::read_to_string(&log_file)
            .expect("Failed to read log file");

        // Verify only error message is logged
        assert!(contents.contains("ERROR_TEST_1"), "Should contain error message");
        assert!(!contents.contains("WARN_TEST_1"), "Should not contain warning message");
        assert!(!contents.contains("INFO_TEST_1"), "Should not contain info message");
        assert!(!contents.contains("DEBUG_TEST_1"), "Should not contain debug message");
    }

    /// Test log level filtering - warn level logs warn and error
    /// Validates: Requirements 3.2
    #[test]
    #[serial_test::serial]
    fn test_log_level_warn_filtering() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let log_file = temp_dir.path().join("test_warn.log");

        // Initialize logger with Warn level
        initialize_logger(log_file.clone(), LogLevel::Warn)
            .expect("Failed to initialize logger");

        // Log messages at different levels with unique identifiers
        log_error("ERROR_TEST_2");
        log_warn("WARN_TEST_2");
        log_info("INFO_TEST_2");
        log_debug("DEBUG_TEST_2");

        // Explicitly flush and close the file
        {
            let mut guard = LOG_FILE.lock().unwrap();
            if let Some(ref mut file) = *guard {
                file.flush().expect("Failed to flush");
            }
            *guard = None;
        }
        thread::sleep(Duration::from_millis(100));

        // Read log file contents
        let contents = fs::read_to_string(&log_file)
            .expect("Failed to read log file");

        // Verify error and warn messages are logged
        assert!(contents.contains("ERROR_TEST_2"), "Should contain error message");
        assert!(contents.contains("WARN_TEST_2"), "Should contain warning message");
        assert!(!contents.contains("INFO_TEST_2"), "Should not contain info message");
        assert!(!contents.contains("DEBUG_TEST_2"), "Should not contain debug message");
    }

    /// Test log level filtering - info level logs info, warn, and error
    /// Validates: Requirements 3.3
    #[test]
    #[serial_test::serial]
    fn test_log_level_info_filtering() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let log_file = temp_dir.path().join("test_info.log");

        // Initialize logger with Info level
        initialize_logger(log_file.clone(), LogLevel::Info)
            .expect("Failed to initialize logger");

        // Log messages at different levels with unique identifiers
        log_error("ERROR_TEST_3");
        log_warn("WARN_TEST_3");
        log_info("INFO_TEST_3");
        log_debug("DEBUG_TEST_3");

        // Explicitly flush and close the file
        {
            let mut guard = LOG_FILE.lock().unwrap();
            if let Some(ref mut file) = *guard {
                file.flush().expect("Failed to flush");
            }
            *guard = None;
        }
        thread::sleep(Duration::from_millis(100));

        // Read log file contents
        let contents = fs::read_to_string(&log_file)
            .expect("Failed to read log file");

        // Verify error, warn, and info messages are logged
        assert!(contents.contains("ERROR_TEST_3"), "Should contain error message");
        assert!(contents.contains("WARN_TEST_3"), "Should contain warning message");
        assert!(contents.contains("INFO_TEST_3"), "Should contain info message");
        assert!(!contents.contains("DEBUG_TEST_3"), "Should not contain debug message");
    }

    /// Test log level filtering - debug level logs all messages
    /// Validates: Requirements 3.4
    #[test]
    #[serial_test::serial]
    fn test_log_level_debug_filtering() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let log_file = temp_dir.path().join("test_debug.log");

        // Initialize logger with Debug level
        initialize_logger(log_file.clone(), LogLevel::Debug)
            .expect("Failed to initialize logger");

        // Log messages at different levels with unique identifiers
        log_error("ERROR_TEST_4");
        log_warn("WARN_TEST_4");
        log_info("INFO_TEST_4");
        log_debug("DEBUG_TEST_4");

        // Explicitly flush and close the file
        {
            let mut guard = LOG_FILE.lock().unwrap();
            if let Some(ref mut file) = *guard {
                file.flush().expect("Failed to flush");
            }
            *guard = None;
        }
        thread::sleep(Duration::from_millis(100));

        // Read log file contents
        let contents = fs::read_to_string(&log_file)
            .expect("Failed to read log file");

        // Verify all messages are logged
        assert!(contents.contains("ERROR_TEST_4"), "Should contain error message");
        assert!(contents.contains("WARN_TEST_4"), "Should contain warning message");
        assert!(contents.contains("INFO_TEST_4"), "Should contain info message");
        assert!(contents.contains("DEBUG_TEST_4"), "Should contain debug message");
    }

    /// Test timestamp formatting in log entries
    /// Validates: Requirements 1.6
    #[test]
    #[serial_test::serial]
    fn test_timestamp_formatting() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let log_file = temp_dir.path().join("test_timestamp.log");

        // Initialize logger
        initialize_logger(log_file.clone(), LogLevel::Info)
            .expect("Failed to initialize logger");

        // Log a message with unique identifier
        log_info("TIMESTAMP_TEST_5");

        // Explicitly flush and close the file
        {
            let mut guard = LOG_FILE.lock().unwrap();
            if let Some(ref mut file) = *guard {
                file.flush().expect("Failed to flush");
            }
            *guard = None;
        }
        thread::sleep(Duration::from_millis(100));

        // Read log file contents
        let contents = fs::read_to_string(&log_file)
            .expect("Failed to read log file");

        // Verify timestamp format [YYYY-MM-DD HH:MM:SS]
        let lines: Vec<&str> = contents.lines().collect();
        assert!(lines.len() >= 1, "Should have at least one log entry");
        
        // Find the line with our test message
        let line = lines.iter()
            .find(|l| l.contains("TIMESTAMP_TEST_5"))
            .expect("Should find test message");
        
        // Check for timestamp pattern [YYYY-MM-DD HH:MM:SS]
        assert!(line.starts_with('['), "Log entry should start with '['");
        assert!(line.contains("] [INFO]"), "Log entry should contain '] [INFO]'");
        
        // Extract timestamp part
        let timestamp_end = line.find(']').expect("Should find closing bracket");
        let timestamp = &line[1..timestamp_end];
        
        // Verify timestamp format: YYYY-MM-DD HH:MM:SS
        let parts: Vec<&str> = timestamp.split(' ').collect();
        assert_eq!(parts.len(), 2, "Timestamp should have date and time parts");
        
        // Verify date format YYYY-MM-DD
        let date_parts: Vec<&str> = parts[0].split('-').collect();
        assert_eq!(date_parts.len(), 3, "Date should have year, month, day");
        assert_eq!(date_parts[0].len(), 4, "Year should be 4 digits");
        assert_eq!(date_parts[1].len(), 2, "Month should be 2 digits");
        assert_eq!(date_parts[2].len(), 2, "Day should be 2 digits");
        
        // Verify time format HH:MM:SS
        let time_parts: Vec<&str> = parts[1].split(':').collect();
        assert_eq!(time_parts.len(), 3, "Time should have hour, minute, second");
        assert_eq!(time_parts[0].len(), 2, "Hour should be 2 digits");
        assert_eq!(time_parts[1].len(), 2, "Minute should be 2 digits");
        assert_eq!(time_parts[2].len(), 2, "Second should be 2 digits");
    }

    /// Test log entry formatting
    /// Validates: Requirements 1.6
    #[test]
    #[serial_test::serial]
    fn test_log_entry_formatting() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let log_file = temp_dir.path().join("test_format.log");

        // Initialize logger
        initialize_logger(log_file.clone(), LogLevel::Debug)
            .expect("Failed to initialize logger");

        // Log messages at different levels with unique identifiers
        log_error("FORMAT_ERROR_6");
        log_warn("FORMAT_WARN_6");
        log_info("FORMAT_INFO_6");
        log_debug("FORMAT_DEBUG_6");

        // Explicitly flush, sync, and close the file
        {
            let mut guard = LOG_FILE.lock().unwrap();
            if let Some(ref mut file) = *guard {
                file.flush().expect("Failed to flush");
                file.sync_all().expect("Failed to sync");
            }
            *guard = None;
        }
        thread::sleep(Duration::from_millis(100));

        // Read log file contents
        let contents = fs::read_to_string(&log_file)
            .expect("Failed to read log file");

        // Verify each message appears with correct format: [timestamp] [LEVEL] message
        assert!(contents.contains("] [ERROR] FORMAT_ERROR_6"), "Should contain formatted error");
        assert!(contents.contains("] [WARN] FORMAT_WARN_6"), "Should contain formatted warning");
        assert!(contents.contains("] [INFO] FORMAT_INFO_6"), "Should contain formatted info");
        assert!(contents.contains("] [DEBUG] FORMAT_DEBUG_6"), "Should contain formatted debug");
    }

    /// Test log file creation
    /// Validates: Requirements 1.2
    #[test]
    #[serial_test::serial]
    fn test_log_file_creation() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let log_file = temp_dir.path().join("test_create.log");

        // Verify file doesn't exist yet
        assert!(!log_file.exists(), "Log file should not exist before initialization");

        // Initialize logger
        initialize_logger(log_file.clone(), LogLevel::Info)
            .expect("Failed to initialize logger");

        // Verify file was created
        assert!(log_file.exists(), "Log file should exist after initialization");
    }

    /// Test directory creation for log file
    /// Validates: Requirements 1.2
    #[test]
    #[serial_test::serial]
    fn test_log_directory_creation() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let log_dir = temp_dir.path().join("nested").join("log").join("directory");
        let log_file = log_dir.join("test.log");

        // Verify directory doesn't exist yet
        assert!(!log_dir.exists(), "Log directory should not exist before initialization");

        // Initialize logger
        initialize_logger(log_file.clone(), LogLevel::Info)
            .expect("Failed to initialize logger");

        // Verify directory and file were created
        assert!(log_dir.exists(), "Log directory should exist after initialization");
        assert!(log_file.exists(), "Log file should exist after initialization");
    }

    /// Test that logger handles missing file gracefully (write errors)
    /// Validates: Requirements 1.2
    #[test]
    fn test_write_without_initialization() {
        // Reset the global logger state by setting it to None
        *LOG_FILE.lock().unwrap() = None;

        // Try to log without initializing - should not panic
        log_info("This should not crash");
        log_error("This should also not crash");
        
        // If we get here, the test passed (no panic occurred)
    }
}
