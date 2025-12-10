use super::{Cell, Component, TextStyle};
use crate::theme::Theme;
use crossterm::style::Color;

/// Scrollbar component that renders a vertical scrollbar with track and indicator
///
/// This component encapsulates the rendering logic for scrollbars, including
/// position calculations, visibility logic, and proper bounds checking.
pub struct Scrollbar {
    /// Total number of items in the scrollable list
    pub total_items: usize,
    /// Number of items that fit in the current viewport
    pub visible_items: usize,
    /// Index of the first visible item (0-based scroll position)
    pub first_visible_index: usize,
}

impl Scrollbar {
    /// Create a new Scrollbar component
    pub fn new(total_items: usize, visible_items: usize, first_visible_index: usize) -> Self {
        Self {
            total_items,
            visible_items,
            first_visible_index,
        }
    }
}

impl Component for Scrollbar {
    /// Renders the scrollbar component to a 2D array of Cells
    ///
    /// Note: For scrollbars, the `width` parameter is treated as `height` since
    /// scrollbars are vertical components that span multiple rows but only one column.
    ///
    /// # Parameters
    ///
    /// * `height` - Height in rows for the scrollbar track (passed as `width` for trait compatibility)
    /// * `theme` - Theme object containing scrollbar colors and characters
    /// * `is_selected` - Ignored for scrollbars (always rendered the same way)
    ///
    /// # Returns
    ///
    /// A 2D array of Cells where:
    /// * The outer Vec represents rows (height of the scrollbar)
    /// * Each inner Vec contains exactly one Cell (single column)
    /// * Returns empty Vec if scrollbar is not needed
    fn render(&self, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
        // Step 1: Determine visibility - return empty if scrollbar not needed
        if self.total_items <= self.visible_items || self.total_items == 0 || height == 0 {
            return vec![];
        }

        // Step 2: Calculate indicator dimensions
        let indicator_height = std::cmp::max(1, (self.visible_items * height) / self.total_items);

        // Step 3: Calculate indicator position
        let indicator_travel_range = height.saturating_sub(indicator_height);
        let scrollable_items = self.total_items.saturating_sub(self.visible_items);
        
        let indicator_offset = if scrollable_items > 0 {
            (self.first_visible_index * indicator_travel_range) / scrollable_items
        } else {
            0
        };

        // Step 4: Clamp indicator to bounds
        let indicator_start = std::cmp::min(indicator_offset, height.saturating_sub(indicator_height));

        // Step 5: Get theme colors and characters
        let fg_color = string_to_fg_color_or_default(&theme.scrollbar_fg);
        let bg_color = string_to_bg_color_or_default(&theme.scrollbar_bg);
        let track_char = get_first_char_or_default(&theme.scrollbar_track_char, '│');
        let indicator_char = get_first_char_or_default(&theme.scrollbar_indicator_char, '█');
        let text_style = TextStyle::new();

        // Step 6: Build Cell array
        let mut cells = Vec::with_capacity(height);
        
        for row in 0..height {
            let character = if row >= indicator_start && row < indicator_start + indicator_height {
                indicator_char
            } else {
                track_char
            };
            
            let cell = Cell::new(character, fg_color, bg_color, text_style);
            cells.push(vec![cell]); // Single column per row
        }

        cells
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

/// Get the first character from a string, or return a default character if empty
fn get_first_char_or_default(s: &str, default: char) -> char {
    s.chars().next().unwrap_or(default)
}