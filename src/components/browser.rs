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
    /// Total height available for the browser component
    pub height: usize,
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
    ///
    /// # Arguments
    /// * `top_left` - Position of the top-left corner (column, row)
    /// * `width` - Total width available for the browser component
    /// * `height` - Total height available for the browser component
    /// * `categories` - Collection of category components to display
    /// * `episodes` - Collection of episode components to display
    ///
    /// # Returns
    /// * `Browser` - A new browser component with default selection and scroll state
    pub fn new(
        top_left: (usize, usize),
        width: usize,
        height: usize,
        categories: Vec<Category>,
        episodes: Vec<Episode>,
    ) -> Self {
        Self {
            top_left,
            width,
            height,
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
    pub fn needs_scrollbar(&self) -> bool {
        self.total_items() > self.height
    }

    /// Get the width available for content (accounting for scrollbar if needed)
    pub fn content_width(&self) -> usize {
        if self.needs_scrollbar() {
            self.width.saturating_sub(1) // Reserve 1 column for scrollbar
        } else {
            self.width
        }
    }

    /// Get the number of items that can fit in the viewport
    pub fn visible_items(&self) -> usize {
        std::cmp::min(self.height, self.total_items())
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
    pub fn clamp_first_visible_item(&mut self) {
        let total = self.total_items();
        if total <= self.height {
            // All items fit, no scrolling needed
            self.first_visible_item = 0;
        } else {
            // Ensure we don't scroll past the last screenful
            let max_first_visible = total.saturating_sub(self.height);
            self.first_visible_item = std::cmp::min(self.first_visible_item, max_first_visible);
        }
    }

    /// Ensure the selected item is visible in the viewport by adjusting scroll position
    pub fn ensure_selection_visible(&mut self) {
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
        let last_visible_item = self.first_visible_item + self.height - 1;
        if self.selected_item > last_visible_item {
            self.first_visible_item = self.selected_item.saturating_sub(self.height - 1);
        }

        self.clamp_first_visible_item();
    }

    /// Set the selected item with bounds checking
    /// 
    /// # Arguments
    /// * `index` - The index of the item to select
    pub fn set_selected_item(&mut self, index: usize) {
        self.selected_item = index;
        self.clamp_selected_item();
        self.ensure_selection_visible();
    }

    /// Get the currently selected item index
    pub fn get_selected_item(&self) -> usize {
        self.selected_item
    }

    /// Move selection up by one item with bounds checking
    pub fn move_selection_up(&mut self) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
            self.ensure_selection_visible();
        }
    }

    /// Move selection down by one item with bounds checking
    pub fn move_selection_down(&mut self) {
        let total = self.total_items();
        if total > 0 && self.selected_item < total - 1 {
            self.selected_item += 1;
            self.ensure_selection_visible();
        }
    }

    /// Check if the item at the given index is currently selected
    pub fn is_item_selected(&self, index: usize) -> bool {
        index == self.selected_item
    }

    /// Move selection up by one item (alias for move_selection_up for consistency)
    pub fn move_up(&mut self) {
        self.move_selection_up();
    }

    /// Move selection down by one item (alias for move_selection_down for consistency)
    pub fn move_down(&mut self) {
        self.move_selection_down();
    }

    /// Move selection up by one page (viewport height)
    pub fn page_up(&mut self) {
        if self.selected_item > 0 {
            let page_size = self.height;
            if self.selected_item >= page_size {
                self.selected_item -= page_size;
            } else {
                self.selected_item = 0;
            }
            self.clamp_selected_item();
            self.ensure_selection_visible();
        }
    }

    /// Move selection down by one page (viewport height)
    pub fn page_down(&mut self) {
        let total = self.total_items();
        if total > 0 && self.selected_item < total - 1 {
            let page_size = self.height;
            self.selected_item = std::cmp::min(self.selected_item + page_size, total - 1);
            self.clamp_selected_item();
            self.ensure_selection_visible();
        }
    }

    /// Get the number of categories
    pub fn category_count(&self) -> usize {
        self.categories.len()
    }

    /// Get the number of episodes
    pub fn episode_count(&self) -> usize {
        self.episodes.len()
    }

    /// Check if the selected item is a category
    pub fn is_selected_category(&self) -> bool {
        self.selected_item < self.categories.len()
    }

    /// Check if the selected item is an episode
    pub fn is_selected_episode(&self) -> bool {
        self.selected_item >= self.categories.len() && self.selected_item < self.total_items()
    }

    /// Get the index of the selected category (if selected item is a category)
    pub fn get_selected_category_index(&self) -> Option<usize> {
        if self.is_selected_category() {
            Some(self.selected_item)
        } else {
            None
        }
    }

    /// Get the index of the selected episode (if selected item is an episode)
    pub fn get_selected_episode_index(&self) -> Option<usize> {
        if self.is_selected_episode() {
            Some(self.selected_item - self.categories.len())
        } else {
            None
        }
    }

    /// Get a reference to the selected category (if selected item is a category)
    pub fn get_selected_category(&self) -> Option<&Category> {
        self.get_selected_category_index().and_then(|idx| self.categories.get(idx))
    }

    /// Get a reference to the selected episode (if selected item is an episode)
    pub fn get_selected_episode(&self) -> Option<&Episode> {
        self.get_selected_episode_index().and_then(|idx| self.episodes.get(idx))
    }

    /// Convert a global item index to category index (if it's a category)
    pub fn global_index_to_category_index(&self, global_index: usize) -> Option<usize> {
        if global_index < self.categories.len() {
            Some(global_index)
        } else {
            None
        }
    }

    /// Convert a global item index to episode index (if it's an episode)
    pub fn global_index_to_episode_index(&self, global_index: usize) -> Option<usize> {
        if global_index >= self.categories.len() && global_index < self.total_items() {
            Some(global_index - self.categories.len())
        } else {
            None
        }
    }

    /// Convert a category index to global item index
    pub fn category_index_to_global_index(&self, category_index: usize) -> Option<usize> {
        if category_index < self.categories.len() {
            Some(category_index)
        } else {
            None
        }
    }

    /// Convert an episode index to global item index
    pub fn episode_index_to_global_index(&self, episode_index: usize) -> Option<usize> {
        if episode_index < self.episodes.len() {
            Some(self.categories.len() + episode_index)
        } else {
            None
        }
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
    /// # Parameters
    /// * `width` - Maximum width for rendering (should match self.width)
    /// * `theme` - Theme object containing colors and styling
    /// * `is_selected` - Whether this browser component is selected (typically always true for main browser)
    ///
    /// # Returns
    /// * `Vec<Vec<Cell>>` - 2D array of cells representing the rendered browser
    fn render(&self, _width: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
        // Handle edge cases
        if self.width == 0 || self.height == 0 {
            return vec![];
        }

        let total_items = self.total_items();
        if total_items == 0 {
            // Return empty rows for the full height
            return vec![vec![]; self.height];
        }

        // Calculate layout
        let needs_scrollbar = self.needs_scrollbar();
        let content_width = self.content_width();
        
        // Ensure we have valid scroll position
        let mut browser_copy = Browser {
            top_left: self.top_left,
            width: self.width,
            height: self.height,
            categories: self.categories.clone(),
            episodes: self.episodes.clone(),
            selected_item: self.selected_item,
            first_visible_item: self.first_visible_item,
        };
        browser_copy.clamp_selected_item();
        browser_copy.clamp_first_visible_item();

        let mut result = Vec::with_capacity(self.height);

        // Render visible items
        for row in 0..self.height {
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
                let rendered = component.render(content_width, theme, is_item_selected);
                
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
                self.visible_items(),
                browser_copy.first_visible_item,
            );
            
            let scrollbar_cells = scrollbar.render(self.height, theme, false);
            
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