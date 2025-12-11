use super::{Cell, Component, TextStyle, Category, Scrollbar};
use super::episode::Episode;
use crate::theme::Theme;
use crossterm::style::Color;

/// Browser component that serves as the main display element for the episode browser application
///
/// This component integrates category components, episode components, and an optional scrollbar
/// to provide a unified browsing interface for video content. It manages layout, selection state,
/// and scrolling behavior within the terminal-based interface.
pub struct Browser {
    /// Position of the top-left corner (column, row)
    pub top_left: (usize, usize),
    /// Total width available for the browser component
    pub width: usize,
    /// Collection of category components to display
    pub categories: Vec<Category>,
    /// Collection of episode components to display
    pub episodes: Vec<Episode>,
    /// Index of the currently selected item (across both categories and episodes)
    pub selected_item: usize,
    /// Index of the first visible item in the viewport
    pub first_visible_item: usize,
}

impl Browser {
    /// Create a new Browser component
    pub fn new(
        top_left: (usize, usize),
        width: usize,
        categories: Vec<Category>,
        episodes: Vec<Episode>,
    ) -> Self {
        Self {
            top_left,
            width,
            categories,
            episodes,
            selected_item: 0,
            first_visible_item: 0,
        }
    }

    /// Get the total number of items (categories + episodes)
    pub fn total_items(&self) -> usize {
        self.categories.len() + self.episodes.len()
    }

    /// Check if a scrollbar is needed based on content size vs available height
    pub fn needs_scrollbar(&self, height: usize) -> bool {
        self.total_items() > height
    }

    /// Get the width available for content (accounting for scrollbar if needed)
    pub fn content_width(&self, height: usize) -> usize {
        if self.needs_scrollbar(height) {
            self.width.saturating_sub(1) // Reserve 1 column for scrollbar
        } else {
            self.width
        }
    }

    /// Get the number of items that can fit in the viewport
    pub fn visible_items(&self, height: usize) -> usize {
        std::cmp::min(height, self.total_items())
    }

    /// Clamp the selected item to valid bounds
    pub fn clamp_selected_item(&mut self) {
        let total = self.total_items();
        if total == 0 {
            self.selected_item = 0;
        } else {
            self.selected_item = std::cmp::min(self.selected_item, total - 1);
        }
    }

    /// Clamp the first visible item to valid scroll bounds
    pub fn clamp_first_visible_item(&mut self, height: usize) {
        let total = self.total_items();
        if total <= height {
            // All items fit, no scrolling needed
            self.first_visible_item = 0;
        } else {
            // Ensure we don't scroll past the last screenful
            let max_first_visible = total.saturating_sub(height);
            self.first_visible_item = std::cmp::min(self.first_visible_item, max_first_visible);
        }
    }

    /// Ensure the selected item is visible in the viewport by adjusting scroll position
    pub fn ensure_selection_visible(&mut self, height: usize) {
        self.clamp_selected_item();
        
        let total = self.total_items();
        if total == 0 {
            return;
        }

        // If selected item is above the viewport, scroll up
        if self.selected_item < self.first_visible_item {
            self.first_visible_item = self.selected_item;
        }
        
        // If selected item is below the viewport, scroll down
        let last_visible_item = self.first_visible_item + height - 1;
        if self.selected_item > last_visible_item {
            self.first_visible_item = self.selected_item.saturating_sub(height - 1);
        }

        self.clamp_first_visible_item(height);
    }

    /// Set the selected item with bounds checking
    pub fn set_selected_item(&mut self, index: usize) {
        self.selected_item = index;
        self.clamp_selected_item();
        // Note: viewport adjustment is deferred to render time when height is available
    }



    /// Get the component at the specified index (category or episode)
    fn get_component_at_index(&self, index: usize) -> Option<&dyn Component> {
        if index < self.categories.len() {
            Some(&self.categories[index])
        } else {
            let episode_index = index - self.categories.len();
            if episode_index < self.episodes.len() {
                Some(&self.episodes[episode_index])
            } else {
                None
            }
        }
    }
}

impl Component for Browser {
    /// Renders the browser component to a 2D array of Cells
    ///
    /// The browser component coordinates the rendering of categories, episodes, and an optional
    /// scrollbar within the specified viewport. It handles selection highlighting, viewport
    /// scrolling, and proper component positioning.
    ///
    /// Render the browser component
    fn render(&self, _width: usize, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
        // Handle edge cases
        if self.width == 0 || height == 0 {
            return vec![];
        }

        let total_items = self.total_items();
        if total_items == 0 {
            // Return empty rows for the full height
            return vec![vec![]; height];
        }

        // Calculate layout
        let needs_scrollbar = self.needs_scrollbar(height);
        let content_width = self.content_width(height);
        
        // Ensure we have valid scroll position
        let mut browser_copy = Browser {
            top_left: self.top_left,
            width: self.width,
            categories: self.categories.clone(),
            episodes: self.episodes.clone(),
            selected_item: self.selected_item,
            first_visible_item: self.first_visible_item,
        };
        browser_copy.clamp_selected_item();
        browser_copy.clamp_first_visible_item(height);
        browser_copy.ensure_selection_visible(height);

        let mut result = Vec::with_capacity(height);

        // Render visible items
        for row in 0..height {
            let item_index = browser_copy.first_visible_item + row;
            
            if item_index >= total_items {
                // No more items, render empty row
                result.push(vec![]);
                continue;
            }

            // Determine if this item is selected
            let is_item_selected = item_index == browser_copy.selected_item;

            // Get and render the component
            if let Some(component) = browser_copy.get_component_at_index(item_index) {
                let rendered = component.render(content_width, 1, theme, is_item_selected);
                
                // Take the first row of the rendered component (components should render single rows)
                if let Some(first_row) = rendered.first() {
                    let mut row_cells = first_row.clone();
                    
                    // Ensure the row doesn't exceed content width
                    if row_cells.len() > content_width {
                        row_cells.truncate(content_width);
                    }
                    
                    // Pad the row to content width if needed
                    while row_cells.len() < content_width {
                        row_cells.push(Cell::new(' ', Color::Reset, Color::Reset, TextStyle::new()));
                    }
                    
                    result.push(row_cells);
                } else {
                    // Component returned empty, use empty row
                    result.push(vec![]);
                }
            } else {
                // No component found, render empty row
                result.push(vec![]);
            }
        }

        // Add scrollbar if needed
        if needs_scrollbar {
            let scrollbar = Scrollbar::new(
                total_items,
                self.visible_items(height),
                browser_copy.first_visible_item,
            );
            
            let scrollbar_cells = scrollbar.render(1, height, theme, false);
            
            // Append scrollbar cells to each row
            for (row_index, scrollbar_row) in scrollbar_cells.iter().enumerate() {
                if row_index < result.len() {
                    if let Some(scrollbar_cell) = scrollbar_row.first() {
                        result[row_index].push(scrollbar_cell.clone());
                    }
                }
            }
            
            // Ensure all rows have the scrollbar column (in case scrollbar returned fewer rows)
            for row in result.iter_mut() {
                if row.len() == content_width {
                    // Add empty scrollbar cell if scrollbar didn't provide one
                    row.push(Cell::new(' ', Color::Reset, Color::Reset, TextStyle::new()));
                }
            }
        }

        result
    }
}