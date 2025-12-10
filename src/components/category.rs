use super::{Cell, Component, TextStyle};
use crate::theme::Theme;
use crossterm::style::Color;

/// Type of category being displayed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CategoryType {
    Series,
    Season,
}

/// Category component that renders series and season information
///
/// This component encapsulates the rendering logic for category entries,
/// displaying title, episode count, and watched count information.
#[derive(Clone)]
pub struct Category {
    pub title: String,
    pub episode_count: usize,
    pub watched_count: usize,
    pub category_type: CategoryType,
}

impl Category {
    /// Create a new Category component
    pub fn new(
        title: String,
        episode_count: usize,
        watched_count: usize,
        category_type: CategoryType,
    ) -> Self {
        Self {
            title,
            episode_count,
            watched_count,
            category_type,
        }
    }
}

impl Component for Category {
    fn render(&self, width: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>> {
        // Handle edge case: width of 0
        if width == 0 {
            return vec![vec![]];
        }

        // Use right-justified format with styled count for both selected and unselected
        // Format: "Title  X/Y watched" where count is right-justified
        
        let count_text = format!("{}/{} watched", self.watched_count, self.episode_count);
        let count_visual_len = count_text.chars().count();
        
        // Calculate available space for title (reserve space for count + spacing)
        let min_spacing = 1;
        let available_for_title = width
            .saturating_sub(count_visual_len)
            .saturating_sub(min_spacing);
        
        // Truncate title if needed
        let truncated_title = truncate_string(&self.title, available_for_title);
        let title_len = truncated_title.chars().count();
        
        // Calculate actual spacing needed
        let spacing = width
            .saturating_sub(title_len)
            .saturating_sub(count_visual_len)
            .max(1); // Ensure at least 1 space
        
        // Determine colors based on selection state
        let (title_fg, title_bg, count_fg, count_bg, count_style) = if is_selected {
            // When selected, use selection colors for everything
            let fg = string_to_fg_color_or_default(&theme.current_fg);
            let bg = string_to_bg_color_or_default(&theme.current_bg);
            (fg, bg, fg, bg, TextStyle::new())
        } else {
            // When not selected, use different colors for title vs count
            let (title_fg, title_bg) = match self.category_type {
                CategoryType::Series => (
                    string_to_fg_color_or_default(&theme.series_fg),
                    string_to_bg_color_or_default(&theme.series_bg),
                ),
                CategoryType::Season => (
                    string_to_fg_color_or_default(&theme.season_fg),
                    string_to_bg_color_or_default(&theme.season_bg),
                ),
            };
            
            let count_fg = string_to_fg_color_or_default(&theme.count_fg);
            let count_bg = string_to_bg_color_or_default(&theme.episode_bg);
            let count_style = parse_text_style(&theme.count_style);
            
            (title_fg, title_bg, count_fg, count_bg, count_style)
        };
        
        let title_style = TextStyle::new();
        
        // Build the cell array
        let mut cells = Vec::new();
        
        // Add title cells
        for ch in truncated_title.chars() {
            cells.push(Cell::new(ch, title_fg, title_bg, title_style));
        }
        
        // Add spacing cells (use title colors)
        for _ in 0..spacing {
            cells.push(Cell::new(' ', title_fg, title_bg, title_style));
        }
        
        // Add count cells (with count styling)
        for ch in count_text.chars() {
            cells.push(Cell::new(ch, count_fg, count_bg, count_style));
        }
        
        // Ensure we don't exceed the width (truncate if necessary)
        if cells.len() > width {
            cells.truncate(width);
        }
        
        // Return as single-row 2D array
        vec![cells]
    }
}

/// Format a category string with title, episode count, and watched count
///
/// # Arguments
/// * `title` - The category title (series or season name)
/// * `episode_count` - Total number of episodes in the category
/// * `watched_count` - Number of watched episodes
///
/// # Returns
/// * `String` - Formatted string: "Title (X episodes) [Y watched]" or "Title (X episodes)"
fn format_category_string(title: &str, episode_count: usize, watched_count: usize) -> String {
    let base = format!("{} ({} episodes)", title, episode_count);
    
    if watched_count > 0 {
        format!("{} [{} watched]", base, watched_count)
    } else {
        base
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

/// Parse a text style string into a TextStyle struct
///
/// # Arguments
/// * `style` - Style string (e.g., "bold", "italic", "bold,italic")
///
/// # Returns
/// * `TextStyle` - The parsed text style
fn parse_text_style(style: &str) -> TextStyle {
    let mut text_style = TextStyle::new();
    
    if style.is_empty() || style.to_lowercase() == "none" {
        return text_style;
    }
    
    // Split by comma to support multiple styles
    for style_part in style.split(',') {
        let style_part = style_part.trim().to_lowercase();
        match style_part.as_str() {
            "bold" => text_style.bold = true,
            "italic" => text_style.italic = true,
            "underline" | "underlined" => text_style.underlined = true,
            "dim" => text_style.dim = true,
            _ => {
                // Ignore unknown styles silently (or could log if logger is available)
            }
        }
    }
    
    text_style
}
