use super::{Cell, Component, TextStyle};
use crate::dto::EpisodeDetail;
use crate::episode_field::EpisodeField;
use crate::theme::Theme;
use crossterm::style::Color;
use std::collections::HashSet;

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

/// Convert a color string to a background Color, with default fallback
fn string_to_bg_color_or_default(color: &str) -> Color {
    string_to_color(color).unwrap_or(Color::Reset)
}

/// Interactive component for editing episode details with visual feedback
pub struct EpisodeEditor {
    episode_details: EpisodeDetail,
    edit_field: EpisodeField,
    edit_cursor_pos: usize,
    season_number: Option<usize>,
    dirty_fields: HashSet<EpisodeField>,
    entry_location: String,
}

impl EpisodeEditor {
    /// Create a new EpisodeEditor component
    pub fn new(
        episode_details: EpisodeDetail,
        edit_field: EpisodeField,
        edit_cursor_pos: usize,
        season_number: Option<usize>,
        dirty_fields: HashSet<EpisodeField>,
        entry_location: String,
    ) -> Self {
        Self {
            episode_details,
            edit_field,
            edit_cursor_pos,
            season_number,
            dirty_fields,
            entry_location,
        }
    }

    /// Format a field line with highlighting for editing
    fn format_field_line_with_highlighting(&self, field: EpisodeField, theme: &Theme) -> Vec<Cell> {
        let display_name = field.display_name();
        let value = match field {
            EpisodeField::Path => self.extract_path_and_filename().0,
            EpisodeField::Filename => self.extract_path_and_filename().1,
            EpisodeField::Season => {
                // Use season_number if available, otherwise fall back to episode_details
                if let Some(season_num) = self.season_number {
                    season_num.to_string()
                } else {
                    field.get_field_value(&self.episode_details)
                }
            }
            _ => field.get_field_value(&self.episode_details),
        };
        
        let line = if value.is_empty() {
            format!("{}: ", display_name)
        } else {
            format!("{}: {}", display_name, value)
        };
        
        let mut cells = Vec::new();
        let is_current_field = field == self.edit_field;
        let is_dirty = self.is_field_dirty(field);
        let cursor_pos = self.calculate_cursor_position(field, &line);
        
        // Calculate where the field name ends (including ":" but not the space)
        let field_name_end = display_name.len() + 1; // +1 for ":"
        
        // Convert string to cells with proper highlighting
        for (char_index, ch) in line.chars().enumerate() {
            let mut cell_fg;
            let mut cell_bg = Color::Reset;
            
            // Determine colors based on position and field state
            if char_index < field_name_end {
                // This is part of the field name (e.g., "Title: ")
                if is_dirty {
                    // Apply dirty colors to field name only
                    cell_fg = string_to_fg_color_or_default(&theme.dirty_fg);
                    cell_bg = string_to_bg_color_or_default(&theme.dirty_bg);
                } else {
                    // Normal field name color
                    cell_fg = string_to_fg_color_or_default(&theme.episode_fg);
                }
            } else {
                // This is part of the field value
                cell_fg = string_to_fg_color_or_default(&theme.episode_fg);
            }
            
            // Highlight cursor position or entire value for current field (only if editable)
            if is_current_field && field.is_editable() {
                match field {
                    EpisodeField::Season | EpisodeField::EpisodeNumber => {
                        // For Season and EpisodeNumber, highlight all characters in the field value area
                        // Skip the space immediately after the colon, but highlight all value characters
                        if char_index > field_name_end {
                            // Find the start of the actual value (skip leading spaces)
                            let value_start_pos = line.chars()
                                .skip(field_name_end + 1) // Skip past "Field: "
                                .position(|c| c != ' ')
                                .map(|pos| field_name_end + 1 + pos)
                                .unwrap_or(field_name_end + 1);
                            
                            // Highlight if we're at or past the value start and it's not trailing spaces
                            if char_index >= value_start_pos && !value.is_empty() {
                                // Check if this character is part of the value (not trailing spaces)
                                let chars_after_colon: String = line.chars().skip(field_name_end + 1).collect();
                                let trimmed_value = chars_after_colon.trim_start();
                                let value_end_pos = value_start_pos + trimmed_value.trim_end().len();
                                
                                if char_index < value_end_pos {
                                    cell_bg = string_to_fg_color_or_default(&theme.episode_fg);
                                    cell_fg = Color::Reset;
                                }
                            }
                        }
                    }
                    _ => {
                        // For other fields, highlight only the cursor position
                        if let Some(cursor_position) = cursor_pos {
                            if char_index == cursor_position {
                                // Invert colors for cursor position
                                cell_bg = string_to_fg_color_or_default(&theme.episode_fg);
                                cell_fg = Color::Reset;
                            }
                        }
                    }
                }
            }
            
            cells.push(Cell::new(ch, cell_fg, cell_bg, TextStyle::new()));
        }
        
        cells
    }

    /// Check if a field is dirty (modified but not saved)
    fn is_field_dirty(&self, field: EpisodeField) -> bool {
        self.dirty_fields.contains(&field)
    }

    /// Calculate cursor position within field boundaries for the current edit field
    fn calculate_cursor_position(&self, field: EpisodeField, field_line: &str) -> Option<usize> {
        if field != self.edit_field {
            return None;
        }
        
        // Find the position after the field label (e.g., "Title: ")
        let display_name = field.display_name();
        let label_end = display_name.len() + 2; // +2 for ": "
        
        // Clamp cursor position to valid range within the field value
        let value_start = label_end;
        let value_end = field_line.len();
        let max_cursor_pos = if value_end > value_start {
            value_end - value_start
        } else {
            0
        };
        
        let clamped_cursor = self.edit_cursor_pos.min(max_cursor_pos);
        Some(value_start + clamped_cursor)
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

impl Component for EpisodeEditor {
    /// Renders the episode editor with field highlighting and dirty state indicators
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
            
            let mut row = self.format_field_line_with_highlighting(field, theme);
            
            // Truncate if necessary
            if row.len() > width {
                row.truncate(width);
            }
            
            // Pad row to width if needed
            while row.len() < width {
                let fg_color = string_to_fg_color_or_default(&theme.episode_fg);
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