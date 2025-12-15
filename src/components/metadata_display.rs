use super::{Cell, Component, TextStyle};
use crate::dto::EpisodeDetail;
use crate::episode_field::EpisodeField;
use crate::theme::Theme;
use crossterm::style::Color;

/// Convert a color string to a Color, with default fallback
fn string_to_color(color: &str) -> Option<Color> {
    match color.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "darkgray" | "dark_gray" => Some(Color::DarkGrey),
        "reset" => Some(Color::Reset),
        _ => None,
    }
}

/// Convert a color string to a foreground Color, with default fallback
fn string_to_fg_color_or_default(color: &str) -> Color {
    string_to_color(color).unwrap_or(Color::Reset)
}

/// Read-only component for displaying episode metadata with consistent field layout
pub struct MetadataDisplay {
    episode_details: EpisodeDetail,
    season_number: Option<usize>,
    entry_location: String,
}

impl MetadataDisplay {
    /// Create a new MetadataDisplay component
    pub fn new(
        episode_details: EpisodeDetail,
        season_number: Option<usize>,
        entry_location: String,
    ) -> Self {
        Self {
            episode_details,
            season_number,
            entry_location,
        }
    }

    /// Format a field line for display
    fn format_field_line(&self, field: EpisodeField) -> String {
        let display_name = field.display_name();
        let value = match field {
            EpisodeField::Path => self.extract_path_and_filename().0,
            EpisodeField::Filename => self.extract_path_and_filename().1,
            _ => field.get_field_value(&self.episode_details),
        };
        
        if value.is_empty() {
            format!("{}: ", display_name)
        } else {
            format!("{}: {}", display_name, value)
        }
    }

    /// Extract path and filename from entry location
    fn extract_path_and_filename(&self) -> (String, String) {
        use std::path::Path;
        
        let path = Path::new(&self.entry_location);
        let filename = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();
        
        let parent_path = path.parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string();
            
        (parent_path, filename)
    }
}

impl Component for MetadataDisplay {
    /// Renders the metadata display with all episode fields
    fn render(&self, width: usize, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
        let mut result = Vec::new();
        
        // Define the fields to display in order
        let fields = [
            EpisodeField::Path,
            EpisodeField::Filename,
            EpisodeField::Title,
            EpisodeField::Year,
            EpisodeField::Watched,
            EpisodeField::Length,
            EpisodeField::Series,
            EpisodeField::Season,
            EpisodeField::EpisodeNumber,
        ];
        
        for (row_index, &field) in fields.iter().enumerate() {
            if row_index >= height {
                break;
            }
            
            let line = self.format_field_line(field);
            let mut row = Vec::new();
            
            // Convert string to cells, truncating if necessary
            let chars: Vec<char> = line.chars().take(width).collect();
            let fg_color = string_to_fg_color_or_default(&theme.episode_fg);
            for ch in chars {
                row.push(Cell::new(ch, fg_color, Color::Reset, TextStyle::new()));
            }
            
            // Pad row to width if needed
            while row.len() < width {
                row.push(Cell::new(' ', fg_color, Color::Reset, TextStyle::new()));
            }
            
            result.push(row);
        }
        
        // Fill remaining rows if needed
        while result.len() < height {
            let mut empty_row = Vec::new();
            let fg_color = string_to_fg_color_or_default(&theme.episode_fg);
            for _ in 0..width {
                empty_row.push(Cell::new(' ', fg_color, Color::Reset, TextStyle::new()));
            }
            result.push(empty_row);
        }
        
        result
    }
}