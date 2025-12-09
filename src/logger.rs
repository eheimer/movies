use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::Local;
use lazy_static::lazy_static;

/// Log levels in hierarchical order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
}

lazy_static! {
    pub static ref LOG_FILE: Mutex<Option<File>> = Mutex::new(None);
    pub static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Info);
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
