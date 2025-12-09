use movies::dto::EpisodeDetail;
use movies::episode_field::EpisodeField;

/// Test Case 23: Duration display format
/// When displaying episode duration, the system should format it as "hh:mm:ss"
/// Validates: Requirements 7.1, 7.3, 7.4
#[test]
fn test_length_formatting_standard() {
    let mut details = EpisodeDetail {
        title: "Test".to_string(),
        year: "2024".to_string(),
        watched: "false".to_string(),
        length: "6330".to_string(), // 1 hour 45 minutes 30 seconds
        series: None,
        season: None,
        episode_number: "1".to_string(),
    };

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
    let details = EpisodeDetail {
        title: "Test".to_string(),
        year: "2024".to_string(),
        watched: "false".to_string(),
        length: "2730".to_string(), // 45 minutes 30 seconds
        series: None,
        season: None,
        episode_number: "1".to_string(),
    };

    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "00:45:30");
}

#[test]
fn test_length_formatting_empty() {
    let details = EpisodeDetail {
        title: "Test".to_string(),
        year: "2024".to_string(),
        watched: "false".to_string(),
        length: "".to_string(),
        series: None,
        season: None,
        episode_number: "1".to_string(),
    };

    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "");
}

#[test]
fn test_length_formatting_zero() {
    let details = EpisodeDetail {
        title: "Test".to_string(),
        year: "2024".to_string(),
        watched: "false".to_string(),
        length: "0".to_string(),
        series: None,
        season: None,
        episode_number: "1".to_string(),
    };

    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "");
}

#[test]
fn test_length_formatting_long_duration() {
    let details = EpisodeDetail {
        title: "Test".to_string(),
        year: "2024".to_string(),
        watched: "false".to_string(),
        length: "90000".to_string(), // 25 hours
        series: None,
        season: None,
        episode_number: "1".to_string(),
    };

    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "25:00:00");
}

#[test]
fn test_length_formatting_invalid_value() {
    let details = EpisodeDetail {
        title: "Test".to_string(),
        year: "2024".to_string(),
        watched: "false".to_string(),
        length: "invalid".to_string(),
        series: None,
        season: None,
        episode_number: "1".to_string(),
    };

    // Should return the original value if parsing fails
    let formatted = EpisodeField::Length.get_field_value(&details);
    assert_eq!(formatted, "invalid");
}
