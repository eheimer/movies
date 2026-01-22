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
    LastWatchedTime = 9,
    LastProgressTime = 10,
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
            9 => EpisodeField::LastWatchedTime,
            10 => EpisodeField::LastProgressTime,
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
            | EpisodeField::Series
            | EpisodeField::LastWatchedTime
            | EpisodeField::LastProgressTime => false,
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
            EpisodeField::LastWatchedTime => {
                if let Some(iso_datetime) = &details.last_watched_time {
                    crate::database::format_last_watched_time(iso_datetime)
                } else {
                    String::new()
                }
            }
            EpisodeField::LastProgressTime => {
                if let Some(progress_str) = &details.last_progress_time {
                    if let Ok(seconds) = progress_str.parse::<u64>() {
                        if seconds == 0 {
                            String::new()
                        } else {
                            crate::video_metadata::format_duration_hms(seconds)
                        }
                    } else {
                        // If parsing fails, return empty string
                        String::new()
                    }
                } else {
                    String::new()
                }
            }
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
            EpisodeField::LastWatchedTime => "Last Watched",
            EpisodeField::LastProgressTime => "Progress",
        }
    }
}
