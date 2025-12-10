use super::{Cell, Component, TextStyle};
use crate::theme::Theme;
use crossterm::style::Color;

/// Episode component that renders episode information
///
/// This component encapsulates the rendering logic for episode entries,
/// including watched indicators, state-based coloring, and text truncation.
#[derive(Clone)]
pub struct Episode {
    pub name: String,
    pub is_watched: bool,
    pub file_exists: bool,
    pub is_new: bool,
}

impl Episode {
    /// Create a new Episode component
    pub fn new(name: String, is_watched: bool, file_exists: bool, is_new: bool) -> Self {
        Self {
            name,
            is_watched,
            file_exists,
            is_new,
        }
    }
}

impl Component for Episode {
    fn render(&self, width: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>> {
        // Handle edge case: width of 0
        if width == 0 {
            return vec![vec![]];
        }

        // Step 1: Determine base state colors (priority order)
        let (base_fg, base_bg) = if !self.file_exists {
            // Invalid (file doesn't exist) - highest priority
            (
                string_to_fg_color_or_default(&theme.invalid_fg),
                string_to_bg_color_or_default(&theme.invalid_bg),
            )
        } else if self.is_new {
            // New episode (regardless of watched status)
            (
                string_to_fg_color_or_default(&theme.new_fg),
                string_to_bg_color_or_default(&theme.new_bg),
            )
        } else {
            // Normal episode
            (
                string_to_fg_color_or_default(&theme.episode_fg),
                string_to_bg_color_or_default(&theme.episode_bg),
            )
        };

        // Step 2: Apply indicator and style
        let formatted_name = format_episode_with_indicator(&self.name, self.is_watched, theme);

        // Step 3: Truncate to width
        let truncated_name = truncate_string(&formatted_name, width);

        // Step 4: Apply selection override if needed
        let (final_fg, final_bg) = if is_selected {
            (
                string_to_fg_color_or_default(&theme.current_fg),
                string_to_bg_color_or_default(&theme.current_bg),
            )
        } else {
            (base_fg, base_bg)
        };

        // Step 5: Determine text style
        let text_style = if self.is_watched {
            parse_text_style(&theme.watched_style)
        } else {
            parse_text_style(&theme.unwatched_style)
        };

        // Step 6: Convert to Cell array
        let cells: Vec<Cell> = truncated_name
            .chars()
            .map(|ch| Cell::new(ch, final_fg, final_bg, text_style))
            .collect();

        // Return as single-row 2D array
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

/// Parse a style string into a TextStyle struct
///
/// # Arguments
/// * `style` - Style string: "none", "bold", "italic", "underline", "strikethrough", "dim"
///            Multiple styles can be combined with commas: "bold,italic"
///
/// # Returns
/// * `TextStyle` - The parsed text style
fn parse_text_style(style: &str) -> TextStyle {
    if style.is_empty() || style.to_lowercase() == "none" {
        return TextStyle::new();
    }

    let mut text_style = TextStyle::new();

    // Split by comma to support multiple styles
    for style_part in style.split(',') {
        let style_part = style_part.trim().to_lowercase();
        match style_part.as_str() {
            "bold" => text_style.bold = true,
            "italic" => text_style.italic = true,
            "underline" | "underlined" => text_style.underlined = true,
            "strikethrough" | "crossed_out" => text_style.crossed_out = true,
            "dim" => text_style.dim = true,
            _ => {
                // Silently ignore unknown style values
            }
        }
    }

    text_style
}

/// Format an episode name with watched indicator and style if applicable
///
/// # Arguments
/// * `name` - The episode name to format
/// * `is_watched` - Whether the episode has been watched
/// * `theme` - Theme containing the watched/unwatched indicators and styles
///
/// # Returns
/// * `String` - The formatted episode name with indicator
fn format_episode_with_indicator(name: &str, is_watched: bool, theme: &Theme) -> String {
    if is_watched {
        // Add indicator if configured (empty string means no indicator)
        if theme.watched_indicator.is_empty() {
            name.to_string()
        } else {
            format!("{} {}", theme.watched_indicator, name)
        }
    } else {
        // Add unwatched indicator if configured (empty string means no indicator)
        if theme.unwatched_indicator.is_empty() {
            name.to_string()
        } else {
            format!("{} {}", theme.unwatched_indicator, name)
        }
    }
}

/// Truncate a string to fit within a specified width, accounting for Unicode characters
///
/// # Arguments
/// * `s` - The string to truncate
/// * `max_width` - Maximum width in characters
///
/// # Returns
/// * `String` - The truncated string
fn truncate_string(s: &str, max_width: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_width {
        s.to_string()
    } else {
        s.chars().take(max_width).collect()
    }
}
