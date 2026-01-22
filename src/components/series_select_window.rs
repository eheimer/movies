use super::{Cell, Component, TextStyle, SeriesSelector, SeriesCreator};
use crate::dto::Series;
use crate::theme::Theme;
use crate::util::Mode;
use crossterm::style::Color;
use crossterm::terminal::size as get_terminal_size;
use std::io;

// Constants from display.rs for positioning calculations
const COL1_WIDTH: usize = 45;
const DETAIL_HEIGHT: usize = 13; // Increased from 11 to accommodate progress tracking fields
const SERIES_WIDTH: usize = 40;

/// Container component that switches between sub-components based on application mode
pub struct SeriesSelectWindow {
    mode: Mode,
    series_list: Vec<Series>,
    series_selection: Option<usize>,
    new_series_text: String,
    edit_cursor_pos: usize,
    first_visible_series: usize,
    window_width: usize,
    window_height: usize,
}

impl SeriesSelectWindow {
    /// Create a new SeriesSelectWindow component
    pub fn new(
        mode: Mode,
        series_list: Vec<Series>,
        series_selection: Option<usize>,
        new_series_text: String,
        edit_cursor_pos: usize,
        first_visible_series: usize,
        window_width: usize,
        window_height: usize,
    ) -> Self {
        Self {
            mode,
            series_list,
            series_selection,
            new_series_text,
            edit_cursor_pos,
            first_visible_series,
            window_width,
            window_height,
        }
    }
    
    /// Calculate window dimensions based on series count and terminal size
    pub fn calculate_dimensions(
        series_count: usize,
        header_height: usize,
        mode: &Mode,
    ) -> io::Result<(usize, usize)> {
        let window_width = SERIES_WIDTH + 2; // Include borders
        
        // Calculate available height for the terminal
        let (_, terminal_height) = get_terminal_size()?;
        let terminal_height = terminal_height as usize;
        let start_row = header_height + DETAIL_HEIGHT;
        let max_height = terminal_height.saturating_sub(start_row + 2); // Adjust for borders
        
        let window_height = match mode {
            Mode::SeriesCreate => 4, // Fixed height for series creation
            _ => {
                // Dynamic height based on series count, minimum 4, maximum available
                (series_count + 3).min(max_height).max(4)
            }
        };
        
        Ok((window_width, window_height))
    }
    
    /// Calculate horizontal centering position within sidebar
    pub fn calculate_horizontal_position(
        window_width: usize,
        header_height: usize,
    ) -> io::Result<(usize, usize)> {
        let start_col = COL1_WIDTH + 2;
        let start_row = header_height + DETAIL_HEIGHT;
        let sidebar_width = Self::get_sidebar_width()?;
        
        // Center the window horizontally within the sidebar
        let horizontal_position = start_col + (sidebar_width.saturating_sub(window_width) / 2);
        
        Ok((horizontal_position, start_row))
    }
    
    /// Get sidebar width (extracted from display.rs logic)
    fn get_sidebar_width() -> io::Result<usize> {
        let (cols, _) = get_terminal_size()?;
        let cols = cols as usize;
        let sidebar_width = cols.saturating_sub(COL1_WIDTH + 2);
        const MIN_COL2_WIDTH: usize = 20;
        Ok(sidebar_width.max(MIN_COL2_WIDTH))
    }
    
    /// Handle edge cases for window positioning and sizing
    pub fn handle_edge_cases(
        &mut self,
        terminal_width: usize,
        terminal_height: usize,
        header_height: usize,
    ) -> io::Result<()> {
        // Handle empty series list
        if self.series_list.is_empty() && matches!(self.mode, Mode::SeriesSelect) {
            // Ensure minimum window size even with empty list
            self.window_height = self.window_height.max(4);
        }
        
        // Handle very small terminal sizes
        let min_terminal_width = COL1_WIDTH + 2 + 20; // Minimum viable width
        let min_terminal_height = header_height + DETAIL_HEIGHT + 6; // Minimum viable height
        
        if terminal_width < min_terminal_width || terminal_height < min_terminal_height {
            // Adjust window dimensions for small terminals
            let available_width = terminal_width.saturating_sub(COL1_WIDTH + 4);
            self.window_width = self.window_width.min(available_width).max(20);
            
            let available_height = terminal_height.saturating_sub(header_height + DETAIL_HEIGHT + 2);
            self.window_height = self.window_height.min(available_height).max(3);
        }
        
        Ok(())
    }
    
    /// Calculate content alignment and padding within window borders
    pub fn calculate_content_alignment(&self) -> (usize, usize, usize, usize) {
        // Content area is inside the borders
        let content_width = self.window_width.saturating_sub(2);
        let content_height = self.window_height.saturating_sub(2);
        
        // Padding is 1 character on each side due to borders
        let horizontal_padding = 1;
        let vertical_padding = 1;
        
        (content_width, content_height, horizontal_padding, vertical_padding)
    }
    
    /// Determine if thick borders should be used based on mode
    fn use_thick_borders(&self) -> bool {
        matches!(self.mode, Mode::SeriesCreate)
    }
    
    /// Calculate the correct first_visible_series to keep the selected item visible
    fn calculate_viewport_for_selection(&self, visible_items: usize) -> usize {
        if let Some(selected) = self.series_selection {
            // If selection is above visible area, scroll up
            if selected < self.first_visible_series {
                return selected;
            }
            // If selection is below visible area, scroll down
            else if selected >= self.first_visible_series + visible_items {
                let desired_first_visible = selected.saturating_sub(visible_items - 1);
                // Ensure we don't scroll past the last item
                let max_first_visible = self.series_list.len().saturating_sub(visible_items);
                return desired_first_visible.min(max_first_visible);
            }
        }
        // If no selection or selection is already visible, keep current viewport
        // But ensure we don't scroll past the last item
        let max_first_visible = self.series_list.len().saturating_sub(visible_items);
        self.first_visible_series.min(max_first_visible)
    }
    
    /// Create the appropriate sub-component based on current mode
    fn create_sub_component(&self) -> Box<dyn Component> {
        match self.mode {
            Mode::SeriesSelect => {
                // Calculate content dimensions (inside borders)
                let _content_width = self.window_width.saturating_sub(2);
                let content_height = self.window_height.saturating_sub(2);
                
                // Calculate visible items (subtract 1 for prompt row)
                let visible_items = content_height.saturating_sub(1).max(1);
                
                // Calculate the correct first_visible_series to keep selection visible
                let adjusted_first_visible = self.calculate_viewport_for_selection(visible_items);
                
                Box::new(SeriesSelector::new(
                    self.series_list.clone(),
                    self.series_selection,
                    adjusted_first_visible,
                    visible_items,
                ))
            }
            Mode::SeriesCreate => {
                // Calculate content dimensions (inside borders)
                let _content_width = self.window_width.saturating_sub(2);
                
                Box::new(SeriesCreator::new(
                    self.new_series_text.clone(),
                    self.edit_cursor_pos,
                ))
            }
            _ => {
                // Default to SeriesSelector for other modes
                let _content_width = self.window_width.saturating_sub(2);
                let content_height = self.window_height.saturating_sub(2);
                let visible_items = content_height.saturating_sub(1).max(1);
                
                // Calculate the correct first_visible_series to keep selection visible
                let adjusted_first_visible = self.calculate_viewport_for_selection(visible_items);
                
                Box::new(SeriesSelector::new(
                    self.series_list.clone(),
                    self.series_selection,
                    adjusted_first_visible,
                    visible_items,
                ))
            }
        }
    }
    
    /// Render window borders around the content
    fn render_borders(&self, content_cells: Vec<Vec<Cell>>, _theme: &Theme) -> Vec<Vec<Cell>> {
        let mut result = Vec::new();
        let thick = self.use_thick_borders();
        
        // Choose border characters based on thickness
        let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) = if thick {
            ('╔', '╗', '╚', '╝', '═', '║')
        } else {
            ('┌', '┐', '└', '┘', '─', '│')
        };
        
        // Top border
        let mut top_border = Vec::new();
        top_border.push(Cell::new(top_left, Color::Reset, Color::Reset, TextStyle::new()));
        for _ in 1..self.window_width.saturating_sub(1) {
            top_border.push(Cell::new(horizontal, Color::Reset, Color::Reset, TextStyle::new()));
        }
        if self.window_width > 1 {
            top_border.push(Cell::new(top_right, Color::Reset, Color::Reset, TextStyle::new()));
        }
        result.push(top_border);
        
        // Content rows with side borders
        for (row_index, content_row) in content_cells.iter().enumerate() {
            if row_index >= self.window_height.saturating_sub(2) {
                break; // Don't exceed available content height
            }
            
            let mut bordered_row = Vec::new();
            
            // Left border
            bordered_row.push(Cell::new(vertical, Color::Reset, Color::Reset, TextStyle::new()));
            
            // Content (ensure we don't exceed window width)
            let max_content_width = self.window_width.saturating_sub(2);
            for (col_index, cell) in content_row.iter().enumerate() {
                if col_index >= max_content_width {
                    break;
                }
                bordered_row.push(cell.clone());
            }
            
            // Fill remaining content width if needed
            while bordered_row.len() < self.window_width.saturating_sub(1) {
                bordered_row.push(Cell::new(' ', Color::Reset, Color::Reset, TextStyle::new()));
            }
            
            // Right border
            if self.window_width > 1 {
                bordered_row.push(Cell::new(vertical, Color::Reset, Color::Reset, TextStyle::new()));
            }
            
            result.push(bordered_row);
        }
        
        // Fill remaining content height with empty bordered rows if needed
        let content_height = self.window_height.saturating_sub(2);
        while result.len() < content_height + 1 { // +1 for top border
            let mut empty_row = Vec::new();
            
            // Left border
            empty_row.push(Cell::new(vertical, Color::Reset, Color::Reset, TextStyle::new()));
            
            // Empty content
            for _ in 0..self.window_width.saturating_sub(2) {
                empty_row.push(Cell::new(' ', Color::Reset, Color::Reset, TextStyle::new()));
            }
            
            // Right border
            if self.window_width > 1 {
                empty_row.push(Cell::new(vertical, Color::Reset, Color::Reset, TextStyle::new()));
            }
            
            result.push(empty_row);
        }
        
        // Bottom border
        if result.len() < self.window_height {
            let mut bottom_border = Vec::new();
            bottom_border.push(Cell::new(bottom_left, Color::Reset, Color::Reset, TextStyle::new()));
            for _ in 1..self.window_width.saturating_sub(1) {
                bottom_border.push(Cell::new(horizontal, Color::Reset, Color::Reset, TextStyle::new()));
            }
            if self.window_width > 1 {
                bottom_border.push(Cell::new(bottom_right, Color::Reset, Color::Reset, TextStyle::new()));
            }
            result.push(bottom_border);
        }
        
        result
    }
}

impl Component for SeriesSelectWindow {
    /// Renders the SeriesSelectWindow with appropriate sub-component and borders
    fn render(&self, width: usize, height: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>> {
        // Use the provided dimensions, but respect our configured window size
        let actual_width = width.min(self.window_width);
        let actual_height = height.min(self.window_height);
        
        // Calculate content alignment and padding
        let (content_width, content_height, _h_padding, _v_padding) = self.calculate_content_alignment();
        
        // Ensure content fits within actual dimensions
        let final_content_width = content_width.min(actual_width.saturating_sub(2));
        let final_content_height = content_height.min(actual_height.saturating_sub(2));
        
        // Create and render the appropriate sub-component
        let sub_component = self.create_sub_component();
        let content_cells = sub_component.render(final_content_width, final_content_height, theme, is_selected);
        
        // Add borders around the content
        self.render_borders(content_cells, theme)
    }
}