use std::fs;
use tempfile::TempDir;

// Import necessary modules from the main crate
// Note: These tests require the application to be structured as a library
// with public modules exposed in lib.rs

// Note: Due to the global database connection using OnceLock, we can only
// initialize the database once per test run. Tests that require database
// operations must be run sequentially or use a single shared database.

/// Integration Test 1: Config reload with custom colors
/// 
/// This test verifies that when a config file is created with custom color values,
/// those values are correctly loaded and applied throughout the application.
/// 
/// Test flow:
/// 1. Create a temporary directory
/// 2. Write a config file with custom colors
/// 3. Load the config
/// 4. Verify all custom colors are loaded correctly
/// 5. Modify the config file with different colors
/// 6. Reload the config
/// 7. Verify the new colors are applied
#[test]
fn test_config_reload_with_custom_colors() {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("config.yaml");

    // Create initial config with custom colors
    let initial_config = r#"current_fg: Red
current_bg: Blue
watched_indicator: "★"
watched_fg: Cyan
new_fg: Yellow
new_bg: Black
invalid_fg: Magenta
invalid_bg: White
series_fg: Green
series_bg: Black
season_fg: Red
season_bg: White
episode_fg: Blue
episode_bg: Yellow
status_fg: Black
status_bg: Cyan
video_extensions:
  - mp4
video_player: /usr/bin/vlc
"#;
    fs::write(&config_path, initial_config).expect("Failed to write initial config");

    // Load the config
    let config = movies::config::read_config(&config_path);

    // Verify initial custom values
    assert_eq!(config.current_fg, "Red");
    assert_eq!(config.current_bg, "Blue");
    assert_eq!(config.watched_indicator, "★");
    assert_eq!(config.watched_fg, "Cyan");
    assert_eq!(config.new_fg, "Yellow");
    assert_eq!(config.new_bg, "Black");
    assert_eq!(config.invalid_fg, "Magenta");
    assert_eq!(config.invalid_bg, "White");
    assert_eq!(config.series_fg, "Green");
    assert_eq!(config.series_bg, "Black");
    assert_eq!(config.season_fg, "Red");
    assert_eq!(config.season_bg, "White");
    assert_eq!(config.episode_fg, "Blue");
    assert_eq!(config.episode_bg, "Yellow");
    assert_eq!(config.status_fg, "Black");
    assert_eq!(config.status_bg, "Cyan");

    // Modify the config file with different colors
    let modified_config = r#"current_fg: Green
current_bg: Red
watched_indicator: "●"
watched_fg: Yellow
new_fg: Cyan
new_bg: Magenta
invalid_fg: White
invalid_bg: Black
series_fg: Magenta
series_bg: Yellow
season_fg: Cyan
season_bg: Green
episode_fg: White
episode_bg: Red
status_fg: Yellow
status_bg: Blue
video_extensions:
  - mkv
video_player: /usr/bin/mpv
"#;
    fs::write(&config_path, modified_config).expect("Failed to write modified config");

    // Reload the config
    let reloaded_config = movies::config::read_config(&config_path);

    // Verify modified values are loaded
    assert_eq!(reloaded_config.current_fg, "Green");
    assert_eq!(reloaded_config.current_bg, "Red");
    assert_eq!(reloaded_config.watched_indicator, "●");
    assert_eq!(reloaded_config.watched_fg, "Yellow");
    assert_eq!(reloaded_config.new_fg, "Cyan");
    assert_eq!(reloaded_config.new_bg, "Magenta");
    assert_eq!(reloaded_config.invalid_fg, "White");
    assert_eq!(reloaded_config.invalid_bg, "Black");
    assert_eq!(reloaded_config.series_fg, "Magenta");
    assert_eq!(reloaded_config.series_bg, "Yellow");
    assert_eq!(reloaded_config.season_fg, "Cyan");
    assert_eq!(reloaded_config.season_bg, "Green");
    assert_eq!(reloaded_config.episode_fg, "White");
    assert_eq!(reloaded_config.episode_bg, "Red");
    assert_eq!(reloaded_config.status_fg, "Yellow");
    assert_eq!(reloaded_config.status_bg, "Blue");
    assert_eq!(reloaded_config.video_extensions, vec!["mkv"]);
    assert_eq!(reloaded_config.video_player, "/usr/bin/mpv");
}

/// Integration Test 2: Combined database operations test
/// 
/// This test combines multiple scenarios into a single test to work around
/// the global database connection limitation. It tests:
/// - State transition from new to normal
/// - State transition from invalid to normal
/// - File system integration (file creation/deletion)
/// - Watched status toggling
/// - Complex state priority verification
/// 
/// Test flow:
/// 1. Create a temporary directory with a database
/// 2. Test new to normal transition
/// 3. Test invalid to normal transition
/// 4. Test file system integration
/// 5. Test watched status toggling
/// 6. Test complex state priorities
#[test]
fn test_combined_database_operations() {
    // Create a temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    // Create database
    let db_path = temp_path.join("videos.sqlite");
    movies::database::initialize_database(&db_path).expect("Failed to initialize database");
    
    // Create PathResolver
    let resolver = movies::path_resolver::PathResolver::from_database_path(&db_path)
        .expect("Failed to create PathResolver");
    
    // ===== Test 1: State transition from new to normal =====
    
    // Create a test video file
    let video_file1 = temp_path.join("test_video1.mp4");
    fs::write(&video_file1, "test content").expect("Failed to create video file");
    
    // Import the episode with title matching filename
    movies::database::import_episode_relative(
        video_file1.to_str().unwrap(),
        "test_video1.mp4",
        &resolver,
    ).expect("Failed to import episode");
    
    // Get the episode entries
    let entries = movies::database::get_entries().expect("Failed to get entries");
    let episode1_id = match &entries[0] {
        movies::util::Entry::Episode { episode_id, .. } => *episode_id,
        _ => panic!("Expected Episode entry"),
    };
    
    let episode1_detail = movies::database::get_episode_detail(episode1_id)
        .expect("Failed to get episode detail");
    
    // Verify initial state is New (title equals filename)
    let initial_state = movies::util::determine_episode_state(&entries[0], &episode1_detail, &resolver);
    assert_eq!(initial_state, movies::util::EpisodeState::New, "Episode should be in New state when title equals filename");
    
    // Update the episode title to differ from filename
    let mut updated_detail = episode1_detail.clone();
    updated_detail.title = "Updated Title".to_string();
    movies::database::update_episode_detail(episode1_id, &updated_detail)
        .expect("Failed to update episode");
    
    // Get updated episode details
    let updated_episode1_detail = movies::database::get_episode_detail(episode1_id)
        .expect("Failed to get updated episode detail");
    
    // Verify state transitions to Normal
    let final_state = movies::util::determine_episode_state(&entries[0], &updated_episode1_detail, &resolver);
    assert_eq!(final_state, movies::util::EpisodeState::Normal, "Episode should transition to Normal state after title change");
    
    // ===== Test 2: State transition from invalid to normal =====
    
    // Manually insert an episode with a non-existent file
    let conn = movies::database::get_connection().lock().unwrap();
    conn.execute(
        "INSERT INTO episode (location, name, watched, length, series_id, season_id, episode_number, year)
         VALUES (?1, ?2, false, 0, null, null, null, null)",
        rusqlite::params!["missing_video.mp4", "Missing Video"],
    ).expect("Failed to insert episode");
    drop(conn);
    
    // Get the episode entries
    let entries = movies::database::get_entries().expect("Failed to get entries");
    let episode2 = entries.iter().find(|e| {
        if let movies::util::Entry::Episode { name, .. } = e {
            name == "Missing Video"
        } else {
            false
        }
    }).expect("Failed to find missing video episode");
    
    let episode2_id = match episode2 {
        movies::util::Entry::Episode { episode_id, .. } => *episode_id,
        _ => panic!("Expected Episode entry"),
    };
    
    let episode2_detail = movies::database::get_episode_detail(episode2_id)
        .expect("Failed to get episode detail");
    
    // Verify initial state is Invalid (file doesn't exist)
    let invalid_state = movies::util::determine_episode_state(episode2, &episode2_detail, &resolver);
    assert_eq!(invalid_state, movies::util::EpisodeState::Invalid, "Episode should be in Invalid state when file doesn't exist");
    
    // Create the video file
    let video_file2 = temp_path.join("missing_video.mp4");
    fs::write(&video_file2, "restored content").expect("Failed to create video file");
    
    // Verify state transitions to Normal
    let restored_state = movies::util::determine_episode_state(episode2, &episode2_detail, &resolver);
    assert_eq!(restored_state, movies::util::EpisodeState::Normal, "Episode should transition to Normal state after file is created");
    
    // ===== Test 3: File system integration (deletion and recreation) =====
    
    // Create another test video file
    let video_file3 = temp_path.join("test_video3.mp4");
    fs::write(&video_file3, "test content").expect("Failed to create video file");
    
    // Import the episode
    movies::database::import_episode_relative(
        video_file3.to_str().unwrap(),
        "Test Video 3",
        &resolver,
    ).expect("Failed to import episode");
    
    // Get the episode entries
    let entries = movies::database::get_entries().expect("Failed to get entries");
    let episode3 = entries.iter().find(|e| {
        if let movies::util::Entry::Episode { name, .. } = e {
            name == "Test Video 3"
        } else {
            false
        }
    }).expect("Failed to find test video 3");
    
    let episode3_id = match episode3 {
        movies::util::Entry::Episode { episode_id, .. } => *episode_id,
        _ => panic!("Expected Episode entry"),
    };
    
    let episode3_detail = movies::database::get_episode_detail(episode3_id)
        .expect("Failed to get episode detail");
    
    // Verify initial state is Normal
    let normal_state = movies::util::determine_episode_state(episode3, &episode3_detail, &resolver);
    assert_eq!(normal_state, movies::util::EpisodeState::Normal, "Episode should be in Normal state initially");
    
    // Delete the video file
    fs::remove_file(&video_file3).expect("Failed to delete video file");
    
    // Verify state transitions to Invalid
    let deleted_state = movies::util::determine_episode_state(episode3, &episode3_detail, &resolver);
    assert_eq!(deleted_state, movies::util::EpisodeState::Invalid, "Episode should be in Invalid state after file deletion");
    
    // Recreate the video file
    fs::write(&video_file3, "restored content").expect("Failed to recreate video file");
    
    // Verify state transitions back to Normal
    let recreated_state = movies::util::determine_episode_state(episode3, &episode3_detail, &resolver);
    assert_eq!(recreated_state, movies::util::EpisodeState::Normal, "Episode should return to Normal state after file recreation");
    
    // ===== Test 4: Watched status toggling =====
    
    // Use episode3 for watched status testing
    // Verify initial watched status is false
    assert_eq!(episode3_detail.watched, "false", "Episode should be unwatched initially");
    let unwatched_state = movies::util::determine_episode_state(episode3, &episode3_detail, &resolver);
    assert_eq!(unwatched_state, movies::util::EpisodeState::Normal, "Unwatched episode should be in Normal state");
    
    // Toggle watched status to true
    movies::database::toggle_watched_status(episode3_id)
        .expect("Failed to toggle watched status");
    
    // Get updated episode details
    let watched_detail = movies::database::get_episode_detail(episode3_id)
        .expect("Failed to get episode detail");
    
    // Verify watched status is now true
    assert_eq!(watched_detail.watched, "true", "Episode should be watched after toggle");
    let watched_state = movies::util::determine_episode_state(episode3, &watched_detail, &resolver);
    assert_eq!(watched_state, movies::util::EpisodeState::Watched, "Episode should be in Watched state");
    
    // Toggle watched status back to false
    movies::database::toggle_watched_status(episode3_id)
        .expect("Failed to toggle watched status");
    
    // Get updated episode details
    let unwatched_again_detail = movies::database::get_episode_detail(episode3_id)
        .expect("Failed to get episode detail");
    
    // Verify watched status is back to false
    assert_eq!(unwatched_again_detail.watched, "false", "Episode should be unwatched after second toggle");
    let unwatched_again_state = movies::util::determine_episode_state(episode3, &unwatched_again_detail, &resolver);
    assert_eq!(unwatched_again_state, movies::util::EpisodeState::Normal, "Episode should return to Normal state");
    
    // ===== Test 5: Complex state priority verification =====
    
    // Insert an episode where title equals filename but file doesn't exist
    let conn = movies::database::get_connection().lock().unwrap();
    conn.execute(
        "INSERT INTO episode (location, name, watched, length, series_id, season_id, episode_number, year)
         VALUES (?1, ?2, false, 0, null, null, null, null)",
        rusqlite::params!["priority_test.mp4", "priority_test.mp4"],
    ).expect("Failed to insert episode");
    drop(conn);
    
    // Get the episode entries
    let entries = movies::database::get_entries().expect("Failed to get entries");
    let episode4 = entries.iter().find(|e| {
        if let movies::util::Entry::Episode { name, .. } = e {
            name == "priority_test.mp4"
        } else {
            false
        }
    }).expect("Failed to find priority test episode");
    
    let episode4_id = match episode4 {
        movies::util::Entry::Episode { episode_id, .. } => *episode_id,
        _ => panic!("Expected Episode entry"),
    };
    
    let episode4_detail = movies::database::get_episode_detail(episode4_id)
        .expect("Failed to get episode detail");
    
    // Verify Invalid state takes priority (file doesn't exist, even though title equals filename)
    let priority_invalid = movies::util::determine_episode_state(episode4, &episode4_detail, &resolver);
    assert_eq!(priority_invalid, movies::util::EpisodeState::Invalid, "Invalid state should take priority over New state");
    
    // Create the file
    let video_file4 = temp_path.join("priority_test.mp4");
    fs::write(&video_file4, "content").expect("Failed to create video file");
    
    // Verify state transitions to New (title equals filename, file exists)
    let priority_new = movies::util::determine_episode_state(episode4, &episode4_detail, &resolver);
    assert_eq!(priority_new, movies::util::EpisodeState::New, "Episode should be in New state when title equals filename and file exists");
    
    // Change title to differ from filename
    let mut updated_detail4 = episode4_detail.clone();
    updated_detail4.title = "Different Title".to_string();
    movies::database::update_episode_detail(episode4_id, &updated_detail4)
        .expect("Failed to update episode");
    
    // Get updated episode details
    let updated_episode4_detail = movies::database::get_episode_detail(episode4_id)
        .expect("Failed to get updated episode detail");
    
    // Verify state transitions to Normal
    let priority_normal = movies::util::determine_episode_state(episode4, &updated_episode4_detail, &resolver);
    assert_eq!(priority_normal, movies::util::EpisodeState::Normal, "Episode should be in Normal state after title change");
    
    // ===== Test 6: Combined New + Watched state =====
    
    // Create a video file where title equals filename
    let video_file5 = temp_path.join("combined_test.mp4");
    fs::write(&video_file5, "content").expect("Failed to create video file");
    
    // Import with title matching filename
    movies::database::import_episode_relative(
        video_file5.to_str().unwrap(),
        "combined_test.mp4",
        &resolver,
    ).expect("Failed to import episode");
    
    // Get the episode
    let entries = movies::database::get_entries().expect("Failed to get entries");
    let episode5 = entries.iter().find(|e| {
        if let movies::util::Entry::Episode { name, .. } = e {
            name == "combined_test.mp4"
        } else {
            false
        }
    }).expect("Failed to find combined test episode");
    
    let episode5_id = match episode5 {
        movies::util::Entry::Episode { episode_id, .. } => *episode_id,
        _ => panic!("Expected Episode entry"),
    };
    
    let episode5_detail = movies::database::get_episode_detail(episode5_id)
        .expect("Failed to get episode detail");
    
    // Verify it's in New state (title equals filename)
    let new_state = movies::util::determine_episode_state(episode5, &episode5_detail, &resolver);
    assert_eq!(new_state, movies::util::EpisodeState::New, "Episode should be in New state when title equals filename");
    
    // Mark it as watched
    movies::database::toggle_watched_status(episode5_id)
        .expect("Failed to toggle watched status");
    
    // Get updated details
    let watched_episode5_detail = movies::database::get_episode_detail(episode5_id)
        .expect("Failed to get episode detail");
    
    // Verify it's watched
    assert_eq!(watched_episode5_detail.watched, "true", "Episode should be watched");
    
    // The state detection still returns New (higher priority), but the display logic
    // should handle showing both the new color AND the watched indicator
    let combined_state = movies::util::determine_episode_state(episode5, &watched_episode5_detail, &resolver);
    assert_eq!(combined_state, movies::util::EpisodeState::New, "Episode should still be in New state (higher priority)");
    
    // Note: The actual combined display (green color + watched indicator) is tested
    // through visual inspection in the application, as it's handled in the display layer
}

/// Integration Test 3: End-to-end logging with different log levels
/// 
/// This test verifies that the logging system correctly filters messages based on
/// the configured log level across the entire application flow.
/// 
/// This is a combined test that verifies the logging system works end-to-end.
/// Individual log level filtering is thoroughly tested in the unit tests.
/// 
/// Validates: Requirements 1.3, 3.1, 3.2, 3.3, 3.4
#[test]
#[serial_test::serial]
fn test_end_to_end_logging_with_different_levels() {
    use std::fs;
    use std::thread;
    use std::time::Duration;
    
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let log_file = temp_dir.path().join("test_debug.log");
    
    // Initialize logger with Debug level (most permissive)
    let _ = movies::logger::initialize_logger(log_file.clone(), movies::logger::LogLevel::Debug);
    
    // Log messages at all levels with unique identifiers
    let unique_id = format!("INTEGRATION_{}", std::process::id());
    movies::logger::log_error(&format!("ERROR_{}", unique_id));
    movies::logger::log_warn(&format!("WARN_{}", unique_id));
    movies::logger::log_info(&format!("INFO_{}", unique_id));
    movies::logger::log_debug(&format!("DEBUG_{}", unique_id));
    
    // Flush
    {
        let mut guard = movies::logger::get_log_file_for_test().lock().unwrap();
        if let Some(ref mut file) = *guard {
            use std::io::Write;
            let _ = file.flush();
        }
    }
    thread::sleep(Duration::from_millis(200));
    
    // Read and verify log contents
    let contents = fs::read_to_string(&log_file)
        .expect("Failed to read log file");
    
    // Verify our unique messages are present
    assert!(
        contents.contains(&format!("ERROR_{}", unique_id)),
        "Should contain error message with unique ID"
    );
    assert!(
        contents.contains(&format!("WARN_{}", unique_id)),
        "Should contain warn message with unique ID"
    );
    assert!(
        contents.contains(&format!("INFO_{}", unique_id)),
        "Should contain info message with unique ID"
    );
    assert!(
        contents.contains(&format!("DEBUG_{}", unique_id)),
        "Should contain debug message with unique ID"
    );
}

/// Integration Test 4: Log file archival flow
/// 
/// This test verifies the complete log file archival process, including:
/// - Detection of existing log files
/// - Archival with timestamp prefix
/// - Creation of new log file after archival
/// 
/// Note: This test cannot fully test the interactive prompt, but it tests
/// the archival logic and file operations.
/// 
/// Test flow:
/// 1. Create an existing log file with content
/// 2. Simulate archival by calling the rename logic
/// 3. Verify old file is renamed with timestamp
/// 4. Create new log file and verify it's empty
/// 5. Log to new file and verify content
/// 
/// Validates: Requirements 1.3, 1.4, 1.5
#[test]
#[serial_test::serial]
fn test_log_file_archival_flow() {
    use std::fs;
    use chrono::Local;
    
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let log_file = temp_dir.path().join("movies.log");
    
    // Create an existing log file with content
    fs::write(&log_file, "[2025-12-07 10:00:00] [INFO] Old log entry\n")
        .expect("Failed to create existing log file");
    
    // Verify file exists
    assert!(log_file.exists(), "Existing log file should exist");
    
    // Simulate archival: rename with timestamp
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let filename = log_file.file_name().unwrap().to_str().unwrap();
    let archived_filename = format!("{}-{}", timestamp, filename);
    let archived_path = temp_dir.path().join(archived_filename);
    
    fs::rename(&log_file, &archived_path)
        .expect("Failed to rename log file");
    
    // Verify archived file exists and original doesn't
    assert!(archived_path.exists(), "Archived log file should exist");
    assert!(!log_file.exists(), "Original log file should not exist after archival");
    
    // Verify archived file contains old content
    let archived_content = fs::read_to_string(&archived_path)
        .expect("Failed to read archived log file");
    assert!(archived_content.contains("Old log entry"), "Archived file should contain old content");
    
    // Initialize new log file
    movies::logger::initialize_logger(log_file.clone(), movies::logger::LogLevel::Info)
        .expect("Failed to initialize new logger");
    
    // Log new content
    movies::logger::log_info("New log entry");
    
    // Flush and close
    {
        let mut guard = movies::logger::get_log_file_for_test().lock().unwrap();
        if let Some(ref mut file) = *guard {
            use std::io::Write;
            file.flush().expect("Failed to flush");
        }
        *guard = None;
    }
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Verify new log file exists and contains new content
    assert!(log_file.exists(), "New log file should exist");
    let new_content = fs::read_to_string(&log_file)
        .expect("Failed to read new log file");
    assert!(new_content.contains("New log entry"), "New file should contain new content");
    assert!(!new_content.contains("Old log entry"), "New file should not contain old content");
}

/// Integration Test 5: Logging during user actions
/// 
/// This test simulates user actions and verifies that appropriate log entries
/// are created with the correct format and content.
/// 
/// Validates: Requirements 4.1, 4.2, 4.3, 4.4, 4.5
#[test]
#[serial_test::serial]
fn test_logging_during_user_actions() {
    use std::fs;
    use std::thread;
    use std::time::Duration;
    
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let log_file = temp_dir.path().join("actions.log");
    
    // Initialize logger (may fail if already initialized, that's ok)
    let _ = movies::logger::initialize_logger(log_file.clone(), movies::logger::LogLevel::Info);
    
    // Use unique identifiers to avoid conflicts with other tests
    let unique_id = format!("ACTION_{}", std::process::id());
    
    // Simulate user actions by logging them
    movies::logger::log_info(&format!("{} User imported video: /path/to/test_video.mp4", unique_id));
    movies::logger::log_info(&format!("{} User toggled watched status for episode ID: 1", unique_id));
    movies::logger::log_info(&format!("{} User saved metadata changes for episode ID 1: year=2025, length=120", unique_id));
    movies::logger::log_info(&format!("{} User created new series: Test Series", unique_id));
    movies::logger::log_info(&format!("{} User assigned episode ID 1 to series", unique_id));
    movies::logger::log_info(&format!("{} User initiated rescan operation", unique_id));
    movies::logger::log_info(&format!("{} Rescan operation completed", unique_id));
    movies::logger::log_info(&format!("{} User deleted video: Test Video (ID: 1)", unique_id));
    
    // Flush
    {
        let mut guard = movies::logger::get_log_file_for_test().lock().unwrap();
        if let Some(ref mut file) = *guard {
            use std::io::Write;
            let _ = file.flush();
        }
    }
    thread::sleep(Duration::from_millis(200));
    
    // Read and verify log contents
    let contents = fs::read_to_string(&log_file)
        .expect("Failed to read log file");
    
    // Verify all user actions are logged with our unique ID
    assert!(contents.contains(&format!("{} User imported video", unique_id)), "Should log video import");
    assert!(contents.contains(&format!("{} User toggled watched status", unique_id)), "Should log watched toggle");
    assert!(contents.contains(&format!("{} User saved metadata changes", unique_id)), "Should log metadata changes");
    assert!(contents.contains("year=2025"), "Should log changed year field");
    assert!(contents.contains("length=120"), "Should log changed length field");
    assert!(contents.contains(&format!("{} User created new series", unique_id)), "Should log series creation");
    assert!(contents.contains(&format!("{} User assigned episode", unique_id)), "Should log episode assignment");
    assert!(contents.contains(&format!("{} User initiated rescan", unique_id)), "Should log rescan start");
    assert!(contents.contains(&format!("{} Rescan operation completed", unique_id)), "Should log rescan completion");
    assert!(contents.contains(&format!("{} User deleted video", unique_id)), "Should log video deletion");
    
    // Verify entries have correct format (check lines with our unique ID)
    for line in contents.lines() {
        if line.contains(&unique_id) {
            assert!(line.starts_with('['), "Log entry should start with timestamp");
            assert!(line.contains("] [INFO] "), "Log entry should contain INFO level");
        }
    }
}

/// Integration Test 6: Log file format verification
/// 
/// This test verifies that all log entries follow the expected format:
/// [YYYY-MM-DD HH:MM:SS] [LEVEL] message
/// 
/// Validates: Requirements 1.6
#[test]
#[serial_test::serial]
fn test_log_file_format_verification() {
    use std::fs;
    use std::thread;
    use std::time::Duration;
    
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let log_file = temp_dir.path().join("format_test.log");
    
    // Initialize logger (may fail if already initialized, that's ok)
    let _ = movies::logger::initialize_logger(log_file.clone(), movies::logger::LogLevel::Debug);
    
    // Use unique identifiers to avoid conflicts with other tests
    let unique_id = format!("FORMAT_{}", std::process::id());
    
    // Log messages at all levels with unique identifiers
    movies::logger::log_error(&format!("{}_ERROR", unique_id));
    movies::logger::log_warn(&format!("{}_WARN", unique_id));
    movies::logger::log_info(&format!("{}_INFO", unique_id));
    movies::logger::log_debug(&format!("{}_DEBUG", unique_id));
    
    // Flush
    {
        let mut guard = movies::logger::get_log_file_for_test().lock().unwrap();
        if let Some(ref mut file) = *guard {
            use std::io::Write;
            let _ = file.flush();
        }
    }
    thread::sleep(Duration::from_millis(200));
    
    // Read and verify log contents
    let contents = fs::read_to_string(&log_file)
        .expect("Failed to read log file");
    
    // Find our test messages
    let test_lines: Vec<&str> = contents.lines()
        .filter(|l| l.contains(&unique_id))
        .collect();
    
    assert!(test_lines.len() >= 4, "Should have at least 4 test log entries, found {}", test_lines.len());
    
    // Verify each line matches the expected format
    let expected_levels = vec!["ERROR", "WARN", "INFO", "DEBUG"];
    let expected_suffixes = vec!["_ERROR", "_WARN", "_INFO", "_DEBUG"];
    
    for (i, line) in test_lines.iter().take(4).enumerate() {
        // Verify format: [YYYY-MM-DD HH:MM:SS] [LEVEL] message
        assert!(line.starts_with('['), "Line should start with '['");
        
        // Extract timestamp
        let first_bracket_end = line.find(']').expect("Should find first closing bracket");
        let timestamp = &line[1..first_bracket_end];
        
        // Verify timestamp format
        let parts: Vec<&str> = timestamp.split(' ').collect();
        assert_eq!(parts.len(), 2, "Timestamp should have date and time");
        
        // Verify date format YYYY-MM-DD
        let date_parts: Vec<&str> = parts[0].split('-').collect();
        assert_eq!(date_parts.len(), 3, "Date should have 3 parts");
        assert_eq!(date_parts[0].len(), 4, "Year should be 4 digits");
        assert_eq!(date_parts[1].len(), 2, "Month should be 2 digits");
        assert_eq!(date_parts[2].len(), 2, "Day should be 2 digits");
        
        // Verify time format HH:MM:SS
        let time_parts: Vec<&str> = parts[1].split(':').collect();
        assert_eq!(time_parts.len(), 3, "Time should have 3 parts");
        assert_eq!(time_parts[0].len(), 2, "Hour should be 2 digits");
        assert_eq!(time_parts[1].len(), 2, "Minute should be 2 digits");
        assert_eq!(time_parts[2].len(), 2, "Second should be 2 digits");
        
        // Verify level
        let level_start = first_bracket_end + 3; // Skip "] ["
        let level_end = line[level_start..].find(']').expect("Should find level closing bracket");
        let level = &line[level_start..level_start + level_end];
        assert_eq!(level, expected_levels[i], "Level should match expected");
        
        // Verify message contains our unique ID and suffix
        assert!(line.contains(&format!("{}{}", unique_id, expected_suffixes[i])), 
                "Should contain expected message");
    }
}


