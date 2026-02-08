use crate::dto::EpisodeDetail;
use crate::menu::{MenuContext, get_first_line_preferred_items, calculate_menu_helper_width};
use crate::util::{Entry, LastAction, Mode, ViewContext};
use crate::components::{Component, Cell, TextStyle};
use crate::theme::Theme;

use crossterm::event::KeyCode;
use crossterm::style::Color;

/// Context structure containing all data needed by header components
#[derive(Clone)]
pub struct HeaderContext {
    pub mode: Mode,
    pub filter_mode: bool,
    pub is_dirty: bool,
    pub is_first_run: bool,
    pub terminal_width: usize,
    pub selected_entry: Option<Entry>,
    pub edit_details: EpisodeDetail,
    pub last_action: Option<LastAction>,
    pub view_context: ViewContext,
    pub filter_text: String,
    pub filter_focused: bool,
}

impl HeaderContext {
    /// Creates a new HeaderContext with the provided parameters
    pub fn new(
        mode: Mode,
        filter_mode: bool,
        is_dirty: bool,
        is_first_run: bool,
        terminal_width: usize,
        selected_entry: Option<Entry>,
        edit_details: EpisodeDetail,
        last_action: Option<LastAction>,
        view_context: ViewContext,
        filter_text: String,
        filter_focused: bool,
    ) -> Self {
        Self {
            mode,
            filter_mode,
            is_dirty,
            is_first_run,
            terminal_width,
            selected_entry,
            edit_details,
            last_action,
            view_context,
            filter_text,
            filter_focused,
        }
    }
}

/// Component that displays first line with menu hotkeys and context helpers
pub struct HotkeyHelper {
    mode: Mode,
    filter_mode: bool,
    is_dirty: bool,
    is_first_run: bool,
    terminal_width: usize,
    selected_entry: Option<Entry>,
    edit_details: EpisodeDetail,
    last_action: Option<LastAction>,
    view_context: ViewContext,
}

impl HotkeyHelper {
    /// Creates a new HotkeyHelper component with all needed parameters
    pub fn new(
        mode: Mode,
        filter_mode: bool,
        is_dirty: bool,
        is_first_run: bool,
        terminal_width: usize,
        selected_entry: Option<Entry>,
        edit_details: EpisodeDetail,
        last_action: Option<LastAction>,
        view_context: ViewContext,
    ) -> Self {
        Self {
            mode,
            filter_mode,
            is_dirty,
            is_first_run,
            terminal_width,
            selected_entry,
            edit_details,
            last_action,
            view_context,
        }
    }

    /// Builds hardcoded helper text based on current mode and context
    fn build_hardcoded_helpers(&self) -> String {
        match &self.mode {
            Mode::Browse => {
                // When in filter mode, show simplified menu helpers
                if self.filter_mode {
                    "[ENTER] accept, [ESC] cancel".to_string()
                } else {
                    // Determine context based on view_context
                    match &self.view_context {
                        ViewContext::TopLevel => {
                            "[/] filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] play, [ESC] exit".to_string()
                        }
                        ViewContext::Series { .. } => {
                            "[/] filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] show episodes, [ESC] exit".to_string()
                        }
                        ViewContext::Season { .. } => {
                            "[/] filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] play, [ESC] back".to_string()
                        }
                    }
                }
            }
            Mode::Edit => {
                let mut instruction = "[\u{2191}]/[\u{2193}] change field, [ESC] cancel".to_string();
                if self.is_dirty {
                    instruction.push_str(", [F2] save");
                }
                instruction
            }
            Mode::Entry => {
                // Check if we're in first-run state (no entries and no database)
                if self.is_first_run {
                    "Welcome! Enter the path to your video collection directory, [ESC] cancel".to_string()
                } else {
                    "Enter a file path to scan, [ESC] cancel".to_string()
                }
            }
            Mode::SeriesSelect => {
                "[\u{2191}]/[\u{2193}] navigate, [ENTER] select, [ESC] cancel, [+] create a new series, [CTRL][-] deselect series".to_string()
            }
            Mode::SeriesCreate => "Type a series name, [ENTER] save, [ESC] cancel".to_string(),
            Mode::Menu => {
                "[\u{2191}]/[\u{2193}] navigate, [ENTER] select, [ESC] close menu".to_string()
            }
            Mode::TorrentSearchInput => {
                "Enter: Search | ESC: Cancel".to_string()
            }
            Mode::TorrentSearchResults => {
                "[\u{2191}]/[\u{2193}]: Navigate | Enter: Download | ESC: Cancel".to_string()
            }
        }
    }

    /// Adds first-line preferred menu items to the base text, handling terminal width constraints
    fn add_first_line_preferred_items(&self, base_text: &str) -> String {
        // Only add first-line preferred items in Browse mode when not in filter mode
        if !matches!(self.mode, Mode::Browse) || self.filter_mode {
            return base_text.to_string();
        }

        let menu_context = MenuContext {
            selected_entry: self.selected_entry.clone(),
            episode_detail: self.edit_details.clone(),
            mode: self.mode.clone(),
            last_action: self.last_action.clone(),
        };

        let first_line_preferred = get_first_line_preferred_items(&menu_context);
        
        // Calculate remaining width for FirstLinePreferred items
        let used_width = base_text.len();
        let remaining_width = self.terminal_width.saturating_sub(used_width);
        
        let mut result = base_text.to_string();
        let mut available_width = remaining_width;
        let mut first_item = true;
        
        for item in first_line_preferred {
            let item_width = calculate_menu_helper_width(&item);
            
            if item_width <= available_width {
                // Add separator before each item
                if first_item {
                    result.push_str(", ");
                    available_width = available_width.saturating_sub(2);
                    first_item = false;
                }
                
                // Format the menu item: "[hotkey] label, "
                let hotkey_str = self.format_hotkey(&item.hotkey);
                let item_str = format!("{} {}, ", hotkey_str, item.label);
                result.push_str(&item_str);
                
                available_width = available_width.saturating_sub(item_width);
            } else {
                // Item doesn't fit, stop adding items
                break;
            }
        }
        
        result
    }

    /// Formats a hotkey for display
    fn format_hotkey(&self, hotkey: &Option<KeyCode>) -> String {
        match hotkey {
            Some(KeyCode::F(n)) => format!("[F{}]", n),
            Some(KeyCode::Char(c)) => format!("[{}]", c.to_uppercase()),
            Some(KeyCode::Enter) => "[ENTER]".to_string(),
            Some(KeyCode::Esc) => "[ESC]".to_string(),
            _ => "".to_string(),
        }
    }

    /// Renders the hotkey helper line, returning formatted string with proper width handling
    pub fn render(&self) -> String {
        // Start with "[F1] Menu, "
        let mut header = "[F1] Menu, ".to_string();
        
        // Add hardcoded helpers
        let hardcoded_helpers = self.build_hardcoded_helpers();
        header.push_str(&hardcoded_helpers);
        
        // Add first-line preferred items if applicable
        header = self.add_first_line_preferred_items(&header);
        
        // Remove trailing ", " if present
        if header.ends_with(", ") {
            header.truncate(header.len() - 2);
        }
        
        // Calculate visual width (accounting for multi-byte UTF-8 characters)
        let visual_width = header.chars().count();
        
        // Pad to terminal width based on visual width, not byte length
        let padding_needed = self.terminal_width.saturating_sub(visual_width);
        
        // Add padding spaces
        for _ in 0..padding_needed {
            header.push(' ');
        }
        
        header
    }
}

/// Component that displays repeatable actions with hotkey reminders
pub struct LastActionLine {
    last_action: Option<LastAction>,
    selected_entry: Option<Entry>,
    edit_details: EpisodeDetail,
}

impl LastActionLine {
    /// Creates a new LastActionLine component
    pub fn new(last_action: Option<LastAction>, selected_entry: Option<Entry>, edit_details: EpisodeDetail) -> Self {
        Self { 
            last_action,
            selected_entry,
            edit_details,
        }
    }

    /// Formats the last action for display
    fn format_last_action(&self) -> String {
        // Check if we can repeat the action using the same logic as the original implementation
        if let Some(entry) = &self.selected_entry {
            if crate::util::can_repeat_action(&self.last_action, entry, &self.edit_details) {
                if let Some(action) = &self.last_action {
                    return action.format_display();
                }
            }
        }
        String::new()
    }

    /// Renders the last action line, returning formatted string or empty if no action
    pub fn render(&self) -> String {
        self.format_last_action()
    }
}

/// Component that displays navigation context showing current location
pub struct Breadcrumbs {
    view_context: ViewContext,
}

impl Breadcrumbs {
    /// Creates a new Breadcrumbs component
    pub fn new(view_context: ViewContext) -> Self {
        Self { view_context }
    }

    /// Formats the breadcrumb display based on current view context
    fn format_breadcrumb(&self) -> String {
        match &self.view_context {
            ViewContext::TopLevel => {
                // No breadcrumbs at top level (matches original implementation)
                String::new()
            }
            ViewContext::Series { series_name, .. } => {
                format!("Browsing [{}]", series_name)
            }
            ViewContext::Season { series_name, season_number, .. } => {
                format!("Browsing [{}] -> [season {}]", series_name, season_number)
            }
        }
    }

    /// Renders the breadcrumb line, returning formatted string
    pub fn render(&self) -> String {
        self.format_breadcrumb()
    }
}

/// Component that displays filter input with highlighting when active
pub struct FilterLine {
    filter_text: String,
    filter_focused: bool,
}

impl FilterLine {
    /// Creates a new FilterLine component
    pub fn new(filter_text: String, filter_focused: bool) -> Self {
        Self {
            filter_text,
            filter_focused,
        }
    }

    /// Formats the filter display with highlighting for active state
    fn format_filter_display(&self) -> String {
        // Show filter line when filter_mode is true OR filter string is not empty
        if !self.filter_focused && self.filter_text.is_empty() {
            return String::new();
        }

        // Format as "filter: {text}" to match original implementation
        format!("filter: {}", self.filter_text)
    }

    /// Renders the filter line, returning formatted string or empty if no filter active
    pub fn render(&self) -> String {
        self.format_filter_display()
    }
}

/// Main Header component that composes all four sub-components
pub struct Header {
    pub hotkey_helper: HotkeyHelper,
    pub last_action_line: LastActionLine,
    pub breadcrumbs: Breadcrumbs,
    pub filter_line: FilterLine,
}

impl Header {
    /// Creates a new Header component with all sub-components
    pub fn new(context: &HeaderContext) -> Self {
        let hotkey_helper = HotkeyHelper::new(
            context.mode.clone(),
            context.filter_mode,
            context.is_dirty,
            context.is_first_run,
            context.terminal_width,
            context.selected_entry.clone(),
            context.edit_details.clone(),
            context.last_action.clone(),
            context.view_context.clone(),
        );

        let last_action_line = LastActionLine::new(
            context.last_action.clone(),
            context.selected_entry.clone(),
            context.edit_details.clone(),
        );
        let breadcrumbs = Breadcrumbs::new(context.view_context.clone());
        let filter_line = FilterLine::new(context.filter_text.clone(), context.filter_focused);

        Self {
            hotkey_helper,
            last_action_line,
            breadcrumbs,
            filter_line,
        }
    }

    /// Returns the fixed height for the header (always 4 rows)
    pub fn calculate_height(&self) -> usize {
        4 // Fixed height: HotkeyHelper, LastActionLine, FilterLine, Breadcrumbs
    }

    /// Converts a string to a vector of Cells with the given colors and style
    fn string_to_cells(&self, text: &str, fg_color: Color, bg_color: Color, style: TextStyle) -> Vec<Cell> {
        text.chars()
            .map(|c| Cell::new(c, fg_color, bg_color, style))
            .collect()
    }
}

impl Component for Header {
    /// Renders the header component to a 2D array of Cells (fixed 4-row layout)
    fn render(&self, width: usize, height: usize, _theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
        let mut rows = Vec::new();

        // Get theme colors for header text
        let header_fg = Color::Black; // First line uses black text on white background
        let header_bg = Color::White; // First line uses white background
        let normal_fg = Color::White; // Use white text for visibility on dark terminals
        let normal_bg = Color::Black; // Use black background for contrast
        let header_style = TextStyle::new(); // Use default style for now

        // Row 0: HotkeyHelper (always present) with white background
        let hotkey_text = self.hotkey_helper.render();
        let mut cells = self.string_to_cells(&hotkey_text, header_fg, header_bg, header_style);
        cells.truncate(width);
        while cells.len() < width {
            cells.push(Cell::new(' ', header_fg, header_bg, header_style));
        }
        rows.push(cells);

        // Row 1: LastActionLine (always allocated, may be empty) with normal colors
        let last_action_text = self.last_action_line.render();
        let mut cells = self.string_to_cells(&last_action_text, normal_fg, normal_bg, header_style);
        cells.truncate(width);
        while cells.len() < width {
            cells.push(Cell::new(' ', normal_fg, normal_bg, header_style));
        }
        rows.push(cells);

        // Row 2: FilterLine (always allocated, may be empty) with normal colors
        let filter_text = self.filter_line.render();
        let mut cells = self.string_to_cells(&filter_text, normal_fg, normal_bg, header_style);
        cells.truncate(width);
        while cells.len() < width {
            cells.push(Cell::new(' ', normal_fg, normal_bg, header_style));
        }
        rows.push(cells);

        // Row 3: Breadcrumbs (always allocated, may be empty) with normal colors
        let breadcrumb_text = self.breadcrumbs.render();
        let mut cells = self.string_to_cells(&breadcrumb_text, normal_fg, normal_bg, header_style);
        cells.truncate(width);
        while cells.len() < width {
            cells.push(Cell::new(' ', normal_fg, normal_bg, header_style));
        }
        rows.push(cells);

        // Ensure we return exactly the requested height (should be 4)
        rows.truncate(height);
        while rows.len() < height {
            let empty_row = vec![Cell::new(' ', normal_fg, normal_bg, header_style); width];
            rows.push(empty_row);
        }

        rows
    }
}