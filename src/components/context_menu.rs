use super::{Cell, Component, TextStyle};
use crate::menu::MenuItem;
use crate::theme::Theme;
use crossterm::event::KeyCode;
use crossterm::style::Color;

/// Context menu component for displaying available actions with hotkeys
///
/// Renders a bordered menu window with menu items showing labels and hotkeys.
/// Supports selection highlighting and proper positioning within the terminal.
pub struct ContextMenu {
    menu_items: Vec<MenuItem>,
    selected_index: usize,
}

impl ContextMenu {
    /// Create a new ContextMenu component
    pub fn new(menu_items: Vec<MenuItem>, selected_index: usize) -> Self {
        Self {
            menu_items,
            selected_index,
        }
    }

    /// Calculate the dimensions needed for the menu based on content
    fn calculate_menu_dimensions(&self) -> (usize, usize) {
        if self.menu_items.is_empty() {
            return (0, 0);
        }

        // Find the maximum width needed for content
        let mut max_content_width = 0;
        for item in &self.menu_items {
            // Use visual width for proper UTF-8 character handling
            let label_width = self.visual_width(&item.label);
            let hotkey_text = self.format_hotkey(&item.hotkey);
            let hotkey_width = self.visual_width(&hotkey_text);
            // Content width = label + space + hotkey
            let content_width = label_width.saturating_add(1).saturating_add(hotkey_width);
            max_content_width = max_content_width.max(content_width);
        }

        // Total width = border + padding + content + padding + border
        // Use saturating arithmetic to prevent overflow
        let total_width = max_content_width.saturating_add(4); // 2 for borders, 2 for padding
        let total_height = self.menu_items.len().saturating_add(2); // 2 for top and bottom borders

        (total_width, total_height)
    }

    /// Format a hotkey for display
    fn format_hotkey(&self, hotkey: &Option<KeyCode>) -> String {
        match hotkey {
            Some(KeyCode::F(n)) => format!("[F{}]", n),
            Some(KeyCode::Char(c)) => format!("[{}]", c.to_uppercase()),
            _ => String::new(),
        }
    }

    /// Calculate visual width of a string, handling multi-byte UTF-8 characters
    fn visual_width(&self, text: &str) -> usize {
        // Use unicode_width to get proper visual width for UTF-8 characters
        // For now, use a simple implementation that counts grapheme clusters
        text.chars().count()
    }

    /// Create cells for a single menu item row
    fn create_menu_item_cells(
        &self,
        item: &MenuItem,
        is_selected: bool,
        content_width: usize,
        theme: &Theme,
    ) -> Vec<Cell> {
        let mut cells = Vec::new();

        // Determine colors based on selection
        let (fg_color, bg_color) = if is_selected {
            (
                string_to_fg_color_or_default(&theme.current_fg),
                string_to_bg_color_or_default(&theme.current_bg),
            )
        } else {
            (Color::Reset, Color::Reset)
        };

        // Left border
        cells.push(Cell::new('║', Color::Reset, Color::Reset, TextStyle::new()));

        // Left padding
        cells.push(Cell::new(' ', fg_color, bg_color, TextStyle::new()));

        // Label (left-justified)
        for ch in item.label.chars() {
            cells.push(Cell::new(ch, fg_color, bg_color, TextStyle::new()));
        }

        // Calculate spacing between label and hotkey using visual width
        let hotkey_text = self.format_hotkey(&item.hotkey);
        let label_visual_width = self.visual_width(&item.label);
        let hotkey_visual_width = self.visual_width(&hotkey_text);
        let used_width = label_visual_width.saturating_add(hotkey_visual_width);
        let spacing = content_width.saturating_sub(used_width);

        // Add spacing
        for _ in 0..spacing {
            cells.push(Cell::new(' ', fg_color, bg_color, TextStyle::new()));
        }

        // Hotkey (right-justified)
        for ch in hotkey_text.chars() {
            cells.push(Cell::new(ch, fg_color, bg_color, TextStyle::new()));
        }

        // Right padding
        cells.push(Cell::new(' ', fg_color, bg_color, TextStyle::new()));

        // Right border
        cells.push(Cell::new('║', Color::Reset, Color::Reset, TextStyle::new()));

        cells
    }
}

impl Component for ContextMenu {
    /// Renders the context menu to a 2D array of Cells
    fn render(&self, _width: usize, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
        // Handle empty menu gracefully
        if self.menu_items.is_empty() {
            return vec![];
        }

        let (menu_width, menu_height) = self.calculate_menu_dimensions();

        // Handle cases where menu doesn't fit or has invalid dimensions
        if menu_width == 0 || menu_height == 0 || menu_height > height {
            return vec![];
        }

        let mut result = Vec::with_capacity(menu_height);
        // Use saturating arithmetic for dimension calculations
        let content_width = menu_width.saturating_sub(4); // Subtract borders and padding

        // Top border - use saturating arithmetic to prevent underflow
        let mut top_border = Vec::new();
        top_border.push(Cell::new('╔', Color::Reset, Color::Reset, TextStyle::new()));
        let border_fill_width = menu_width.saturating_sub(2);
        for _ in 0..border_fill_width {
            top_border.push(Cell::new('═', Color::Reset, Color::Reset, TextStyle::new()));
        }
        top_border.push(Cell::new('╗', Color::Reset, Color::Reset, TextStyle::new()));
        result.push(top_border);

        // Menu items
        for (index, item) in self.menu_items.iter().enumerate() {
            let is_selected = index == self.selected_index;
            let item_cells = self.create_menu_item_cells(item, is_selected, content_width, theme);
            result.push(item_cells);
        }

        // Bottom border - use saturating arithmetic to prevent underflow
        let mut bottom_border = Vec::new();
        bottom_border.push(Cell::new('╚', Color::Reset, Color::Reset, TextStyle::new()));
        for _ in 0..border_fill_width {
            bottom_border.push(Cell::new('═', Color::Reset, Color::Reset, TextStyle::new()));
        }
        bottom_border.push(Cell::new('╝', Color::Reset, Color::Reset, TextStyle::new()));
        result.push(bottom_border);

        result
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

/// Convert a color string to a Color enum value
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
        "darkgray" => Some(Color::DarkGrey),
        "reset" => Some(Color::Reset),
        _ => None,
    }
}