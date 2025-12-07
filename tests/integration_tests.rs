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
    let config_path = temp_dir.path().join("config.json");

    // Create initial config with custom colors
    let initial_config = r#"{
  "current_fg": "Red",
  "current_bg": "Blue",
  "watched_indicator": "★",
  "watched_fg": "Cyan",
  "new_fg": "Yellow",
  "new_bg": "Black",
  "invalid_fg": "Magenta",
  "invalid_bg": "White",
  "series_fg": "Green",
  "series_bg": "Black",
  "season_fg": "Red",
  "season_bg": "White",
  "episode_fg": "Blue",
  "episode_bg": "Yellow",
  "status_fg": "Black",
  "status_bg": "Cyan",
  "video_extensions": ["mp4"],
  "video_player": "/usr/bin/vlc"
}"#;
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
    let modified_config = r#"{
  "current_fg": "Green",
  "current_bg": "Red",
  "watched_indicator": "●",
  "watched_fg": "Yellow",
  "new_fg": "Cyan",
  "new_bg": "Magenta",
  "invalid_fg": "White",
  "invalid_bg": "Black",
  "series_fg": "Magenta",
  "series_bg": "Yellow",
  "season_fg": "Cyan",
  "season_bg": "Green",
  "episode_fg": "White",
  "episode_bg": "Red",
  "status_fg": "Yellow",
  "status_bg": "Blue",
  "video_extensions": ["mkv"],
  "video_player": "/usr/bin/mpv"
}"#;
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


