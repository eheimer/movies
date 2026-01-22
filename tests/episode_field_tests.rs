use movies::dto::EpisodeDetail;
use movies::episode_field::EpisodeField;

// Helper function to create EpisodeDetail with default progress fields
fn create_episode_detail(
    title: &str,
    year: &str,
    watched: &str,
    length: &str,
    episode_number: &str,
) -> EpisodeDetail {
    EpisodeDetail {
        title: title.to_string(),
        year: year.to_string(),
        watched: watched.to_string(),
        length: length.to_string(),
        series: None,
        season: None,
        episode_number: episode_number.to_string(),
        last_watched_time: None,
        last_progress_time: None,
    }
}

/// Test Case 23: Duration display format
/// When displaying episode duration, the system should format it as "hh:mm:ss"
/// Validates: Requirements 7.1, 7.3, 7.4
#[test]
fn test_length_formatting_standard() {
    let mut details = create_episode_detail("Test", "2024", "false", "6330", "1");

    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "01:45:30");

    // Test another duration
    details.length = "3661".to_string(); // 1 hour 1 minute 1 second
    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "01:01:01");
}

/// Test Case 24: Duration display for short videos
/// When displaying duration less than one hour, the system should format it as "00:mm:ss"
/// Validates: Requirements 7.2
#[test]
fn test_length_formatting_short_duration() {
    let details = create_episode_detail("Test", "2024", "false", "2730", "1");

    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "00:45:30");
}

#[test]
fn test_length_formatting_empty() {
    let details = create_episode_detail("Test", "2024", "false", "", "1");

    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "");
}

#[test]
fn test_length_formatting_zero() {
    let details = create_episode_detail("Test", "2024", "false", "0", "1");

    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "");
}

#[test]
fn test_length_formatting_long_duration() {
    let details = create_episode_detail("Test", "2024", "false", "90000", "1");

    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "25:00:00");
}

#[test]
fn test_length_formatting_invalid_value() {
    let details = create_episode_detail("Test", "2024", "false", "invalid", "1");

    // Should return the original value if parsing fails
    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "invalid");
}
/// Test Case 25: Progress field display
/// When an episode has progress information, the system should display it correctly
/// Validates: Requirements 7.1, 7.2, 7.3
#[test]
fn test_progress_field_display() {
    let mut details = create_episode_detail("Test", "2024", "false", "3600", "1");
    
    // Test with progress data - progress time stored as seconds in database
    details.last_watched_time = Some("2024-01-15T14:30:00Z".to_string()); // ISO 8601 format
    details.last_progress_time = Some("5025".to_string()); // 5025 seconds = 01:23:45
    
    let last_watched = EpisodeField::LastWatchedTime.get_field_value(&details);
    let progress = EpisodeField::LastProgressTime.get_field_value(&details);
    
    assert_eq!(last_watched, "2024-01-15 14:30:00"); // Formatted for display
    assert_eq!(progress, "01:23:45"); // Formatted as HH:MM:SS
}

/// Test Case 26: Empty progress field display
/// When an episode has no progress information, the system should display empty values
/// Validates: Requirements 7.5
#[test]
fn test_empty_progress_field_display() {
    let details = create_episode_detail("Test", "2024", "false", "3600", "1");
    
    let last_watched = EpisodeField::LastWatchedTime.get_field_value(&details);
    let progress = EpisodeField::LastProgressTime.get_field_value(&details);
    
    assert_eq!(last_watched, "");
    assert_eq!(progress, "");
}

/// Test Case 27: Progress field display names
/// When displaying progress fields, the system should use appropriate display names
/// Validates: Requirements 7.1, 7.2
#[test]
fn test_progress_field_display_names() {
    assert_eq!(EpisodeField::LastWatchedTime.display_name(), "Last Watched");
    assert_eq!(EpisodeField::LastProgressTime.display_name(), "Progress");
}

/// Test Case 28: Progress fields are not editable
/// When in edit mode, progress fields should not be editable by the user
/// Validates: Requirements 7.6
#[test]
fn test_progress_fields_not_editable() {
    assert!(!EpisodeField::LastWatchedTime.is_editable());
    assert!(!EpisodeField::LastProgressTime.is_editable());
}