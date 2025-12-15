use super::{Cell, Component, TextStyle, Scrollbar};
use crate::dto::Series;
use crate::theme::Theme;
use crate::util::truncate_string;
use crossterm::style::Color;

/// Sub-component for displaying the series selection interface with scrolling support
pub struct SeriesSelector {
    series_list: Vec<Series>,
    selected_index: Option<usize>,
    first_visible_index: usize,
    visible_items: usize,
}

impl SeriesSelector {
    /// Create a new SeriesSelector component
    pub fn new(
        series_list: Vec<Series>,
        selected_index: Option<usize>,
        first_visible_index: usize,
        visible_items: usize,
    ) -> Self {
        Self {
            series_list,
            selected_index,
            first_visible_index,
            visible_items,
        }
    }
    

    
    /// Create scrollbar component for viewport management
    fn create_scrollbar(&self) -> Scrollbar {
        Scrollbar::new(
            self.series_list.len(),
            self.visible_items,
            self.first_visible_index,
        )
    }
    

}

impl Component for SeriesSelector {
    /// Renders the SeriesSelector component with series list, selection highlighting, and scrollbar
    fn render(&self, width: usize, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
        let mut cells = Vec::new();
        
        // Calculate how many series items we can display (subtract 1 for prompt row)
        let series_display_height = height.saturating_sub(1);
        
        // Create scrollbar and check if it's needed
        let scrollbar = self.create_scrollbar();
        let scrollbar_cells = scrollbar.render(1, series_display_height, theme, false);
        let scrollbar_visible = !scrollbar_cells.is_empty();
        
        // Calculate effective width for content (reserve space for scrollbar if needed)
        let content_width = if scrollbar_visible {
            width.saturating_sub(1)
        } else {
            width
        };
        
        // First row: prompt text with inverted colors (black on white)
        let prompt_text = "Choose a series or [+] to create";
        let mut prompt_row = Vec::new();
        
        // Truncate prompt if it's too long for the content width
        let display_prompt = if prompt_text.len() > content_width {
            &prompt_text[..content_width]
        } else {
            prompt_text
        };
        
        // Add prompt characters with inverted styling
        for ch in display_prompt.chars() {
            prompt_row.push(Cell::new(
                ch,
                Color::Black,
                Color::White,
                TextStyle::new(),
            ));
        }
        
        // Fill remaining content width with spaces (inverted background)
        while prompt_row.len() < content_width {
            prompt_row.push(Cell::new(
                ' ',
                Color::Black,
                Color::White,
                TextStyle::new(),
            ));
        }
        
        // Add empty space for scrollbar column in prompt row (scrollbar doesn't appear here)
        if scrollbar_visible {
            prompt_row.push(Cell::new(
                ' ',
                Color::Reset,
                Color::Reset,
                TextStyle::new(),
            ));
        }
        
        cells.push(prompt_row);
        
        // Render series items
        let end_index = (self.first_visible_index + series_display_height).min(self.series_list.len());
        
        for (display_row, series_index) in (self.first_visible_index..end_index).enumerate() {
            if display_row >= series_display_height {
                break;
            }
            
            let series = &self.series_list[series_index];
            
            // Use the effective width for formatting (accounting for scrollbar)
            let formatted_text = {
                let label = format!("[{}] {}", series_index + 1, series.name);
                truncate_string(&label, content_width)
            };
            
            let mut series_row = Vec::new();
            
            // Determine if this series is selected
            let is_selected = Some(series_index) == self.selected_index;
            
            // Get colors based on selection state
            let (fg_color, bg_color) = if is_selected {
                (
                    string_to_fg_color_or_default(&theme.current_fg),
                    string_to_bg_color_or_default(&theme.current_bg),
                )
            } else {
                (Color::Reset, Color::Reset)
            };
            
            // Add series text characters
            for ch in formatted_text.chars() {
                series_row.push(Cell::new(
                    ch,
                    fg_color,
                    bg_color,
                    TextStyle::new(),
                ));
            }
            
            // Fill remaining content width with spaces (maintaining selection background)
            while series_row.len() < content_width {
                series_row.push(Cell::new(
                    ' ',
                    fg_color,
                    bg_color,
                    TextStyle::new(),
                ));
            }
            
            // Add scrollbar cell for this row if needed
            if scrollbar_visible {
                let scrollbar_row_index = display_row; // Direct mapping to scrollbar rows
                if let Some(scrollbar_row) = scrollbar_cells.get(scrollbar_row_index) {
                    if let Some(scrollbar_cell) = scrollbar_row.first() {
                        series_row.push(scrollbar_cell.clone());
                    }
                } else {
                    // Add empty space if no scrollbar cell available for this row
                    series_row.push(Cell::new(
                        ' ',
                        Color::Reset,
                        Color::Reset,
                        TextStyle::new(),
                    ));
                }
            }
            
            cells.push(series_row);
        }
        
        // Fill remaining height with empty rows if needed
        while cells.len() < height {
            let mut empty_row = Vec::new();
            
            // Fill content width with empty spaces
            for _ in 0..content_width {
                empty_row.push(Cell::new(
                    ' ',
                    Color::Reset,
                    Color::Reset,
                    TextStyle::new(),
                ));
            }
            
            // Add scrollbar cell for empty rows if needed
            if scrollbar_visible {
                let scrollbar_row_index = cells.len() - 1; // -1 because first row is prompt
                if let Some(scrollbar_row) = scrollbar_cells.get(scrollbar_row_index) {
                    if let Some(scrollbar_cell) = scrollbar_row.first() {
                        empty_row.push(scrollbar_cell.clone());
                    }
                } else {
                    empty_row.push(Cell::new(
                        ' ',
                        Color::Reset,
                        Color::Reset,
                        TextStyle::new(),
                    ));
                }
            }
            
            cells.push(empty_row);
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