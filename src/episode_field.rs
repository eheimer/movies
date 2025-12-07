// the EpisodeField enum is used to represent the fields of an episode
// it combines immutable physical attributes like path and filename with
// mutable logical attributes like title and watched status
// the enum also provides a method to check if a field is editable

use crate::dto::EpisodeDetail;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EpisodeField {
    Path = 0,
    Filename = 1,
    Title = 2,
    Year = 3,
    Watched = 4,
    Length = 5,
    Series = 6,
    Season = 7,
    EpisodeNumber = 8,
}

impl From<usize> for EpisodeField {
    fn from(value: usize) -> Self {
        match value {
            0 => EpisodeField::Path,
            1 => EpisodeField::Filename,
            2 => EpisodeField::Title,
            3 => EpisodeField::Year,
            4 => EpisodeField::Watched,
            5 => EpisodeField::Length,
            6 => EpisodeField::Series,
            7 => EpisodeField::Season,
            8 => EpisodeField::EpisodeNumber,
            _ => panic!("Invalid EditField value"),
        }
    }
}

impl From<EpisodeField> for usize {
    fn from(field: EpisodeField) -> Self {
        field as usize
    }
}

// editable refers to whether the field can be modified in edit mode
// some of the fields that return false (series) cannot be edited in
// edit mode, but can be edited in other ways (series select mode)
// length is not editable because at some point this will be populated
// from the file itself
// some fields must use other logic to determine if they are editable
// or not.  For example, if a series has not been selected, then
// season should not be editable.
impl EpisodeField {
    pub fn is_editable(self) -> bool {
        match self {
            EpisodeField::Path
            | EpisodeField::Filename
            | EpisodeField::Watched
            | EpisodeField::Length
            | EpisodeField::Series => false,
            _ => true,
        }
    }
}

impl EpisodeField {
    pub fn get_field_value(self, details: &EpisodeDetail) -> String {
        match self {
            EpisodeField::Path => String::new(), // Assuming Path is not part of EpisodeDetail
            EpisodeField::Filename => String::new(), // Assuming Filename is not part of EpisodeDetail
            EpisodeField::Title => details.title.clone(),
            EpisodeField::Year => details.year.clone(),
            EpisodeField::Watched => details.watched.clone(),
            EpisodeField::Length => {
                // Format length as hh:mm:ss if it's a valid number
                if details.length.is_empty() {
                    String::new()
                } else if let Ok(seconds) = details.length.parse::<u64>() {
                    if seconds == 0 {
                        String::new()
                    } else {
                        crate::video_metadata::format_duration_hms(seconds)
                    }
                } else {
                    // If parsing fails, return the original value
                    details.length.clone()
                }
            }
            EpisodeField::Series => {
                if let Some(series) = &details.series {
                    series.name.clone()
                } else {
                    String::new()
                }
            }
            EpisodeField::Season => {
                if let Some(season) = &details.season {
                    season.number.to_string()
                } else {
                    String::new()
                }
            } // Assuming Season is not a simple string field
            EpisodeField::EpisodeNumber => details.episode_number.clone(),
        }
    }
}

impl EpisodeField {
    pub fn display_name(self) -> &'static str {
        match self {
            EpisodeField::Path => "Path",
            EpisodeField::Filename => "Filename",
            EpisodeField::Title => "Title",
            EpisodeField::Year => "Year",
            EpisodeField::Watched => "Watched",
            EpisodeField::Length => "Length",
            EpisodeField::Series => "Series",
            EpisodeField::Season => "Season",
            EpisodeField::EpisodeNumber => "Ep #",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
