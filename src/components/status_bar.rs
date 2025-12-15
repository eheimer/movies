use super::{Cell, Component, TextStyle};
use crate::theme::Theme;
use crossterm::style::Color;

/// StatusBar component that renders status messages at the bottom of the terminal
///
/// This component encapsulates status bar rendering logic, including message formatting,
/// text truncation, padding, and theme application. It follows the established component
/// architecture pattern used by other UI components.
pub struct StatusBar {
    /// The status message to display
    message: String,
}

impl StatusBar {
    /// Create a new StatusBar component with the given message
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Component for StatusBar {
    /// Renders the status bar component to a 2D array of Cells
    /// * `width` - Terminal width for proper text formatting
    /// * `height` - Expected to be 1 for status bar (single row)
    /// * `theme` - Theme object containing status bar colors
    /// * `_is_selected` - Ignored for status bars (always rendered the same way)
    fn render(&self, width: usize, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
        // Return empty if no height allocated
        if height == 0 || width == 0 {
            return vec![];
        }

        // Get status line colors from theme
        let status_fg = string_to_fg_color_or_default(&theme.status_fg);
        let status_bg = string_to_bg_color_or_default(&theme.status_bg);
        let text_style = TextStyle::new();

        // Calculate visual width (accounting for multi-byte UTF-8 characters)
        let visual_width = self.message.chars().count();
        
        // Truncate if message is too long (based on visual width)
        let truncated_message = if visual_width > width {
            self.message.chars().take(width).collect::<String>()
        } else {
            self.message.clone()
        };

        // Create cells for the message
        let mut cells: Vec<Cell> = truncated_message
            .chars()
            .map(|c| Cell::new(c, status_fg, status_bg, text_style))
            .collect();

        // Pad to terminal width based on visual width
        let current_visual_width = cells.len();
        let padding_needed = width.saturating_sub(current_visual_width);
        
        // Add padding spaces
        for _ in 0..padding_needed {
            cells.push(Cell::new(' ', status_fg, status_bg, text_style));
        }

        // Return single row (status bar is always one row)
        vec![cells]
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

/// Convert a color string to a Color enum
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