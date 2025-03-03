// the EpisodeField enum is used to represent the fields of an episode
// it combines immutable physical attributes like path and filename with
// mutable logical attributes like title and watched status
// the enum also provides a method to check if a field is editable

use crate::dto::EpisodeDetail;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            EpisodeField::Path | EpisodeField::Filename | EpisodeField::Watched | EpisodeField::Length | EpisodeField::Series => false,
            _ => true,
        }
    }
}

impl EpisodeField {
    pub fn get_field_value<'a>(self, details: &'a EpisodeDetail) -> &'a str {
        match self {
            EpisodeField::Path => "", // Assuming Path is not part of EpisodeDetail
            EpisodeField::Filename => "", // Assuming Filename is not part of EpisodeDetail
            EpisodeField::Title => &details.title,
            EpisodeField::Year => &details.year,
            EpisodeField::Watched => &details.watched,
            EpisodeField::Length => &details.length,
            EpisodeField::Series => "", // Assuming Series is not a simple string field
            EpisodeField::Season => "", // Assuming Season is not a simple string field
            EpisodeField::EpisodeNumber => &details.episode_number,
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