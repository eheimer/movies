use crossterm::style::Color;
use crate::theme::Theme;

pub mod episode;
pub mod category;
pub mod scrollbar;
pub mod browser;
pub mod header;
pub mod detail_panel;
pub mod metadata_display;
pub mod episode_editor;
pub mod status_bar;
pub mod context_menu;
pub mod series_creator;
pub mod series_selector;
pub mod series_select_window;

pub use category::*;
pub use scrollbar::Scrollbar;
pub use browser::Browser;
pub use detail_panel::DetailPanel;
pub use status_bar::StatusBar;
pub use context_menu::ContextMenu;
pub use series_creator::SeriesCreator;
pub use series_selector::SeriesSelector;
pub use series_select_window::SeriesSelectWindow;

/// Represents text styling attributes that can be applied to terminal output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub dim: bool,
    pub crossed_out: bool,
}

impl TextStyle {
    pub fn new() -> Self {
        Self {
            bold: false,
            italic: false,
            underlined: false,
            dim: false,
            crossed_out: false,
        }
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a single terminal cell with character and styling information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub character: char,
    pub fg_color: Color,
    pub bg_color: Color,
    pub style: TextStyle,
}

impl Cell {
    pub fn new(character: char, fg_color: Color, bg_color: Color, style: TextStyle) -> Self {
        Self {
            character,
            fg_color,
            bg_color,
            style,
        }
    }


}

/// Trait for components that can render themselves to a 2D array of Cells
///
/// Components are self-contained rendering units that produce terminal output
/// without performing direct I/O operations. This separation allows components
/// to be tested independently and composed into larger UI structures.
pub trait Component {
    /// Renders the component to a 2D array of Cells
    /// Renders the component to a 2D array of Cells
    /// * `is_selected` - Whether this component represents the currently selected item
    fn render(&self, width: usize, height: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>>;
}


