use crossterm::style::Color;
use crate::theme::Theme;

pub mod episode;
pub mod category;
pub mod scrollbar;
pub mod browser;
pub mod header;

pub use category::*;
pub use scrollbar::Scrollbar;
pub use browser::Browser;
pub use header::{Header, HeaderContext, HotkeyHelper, LastActionLine, Breadcrumbs, FilterLine};

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

/// Renders a single-column multi-row Cell array to terminal output at specified position

pub fn render_cells_at_column(
    cells: &[Vec<Cell>], 
    column: usize, 
    start_row: usize
) -> std::io::Result<()> {
    use crossterm::style::Stylize;
    use crate::terminal::print_at;
    
    for (row_offset, row) in cells.iter().enumerate() {
        if let Some(cell) = row.first() {
            let mut styled = cell.character.to_string()
                .with(cell.fg_color)
                .on(cell.bg_color);
            
            // Apply text styles
            if cell.style.bold {
                styled = styled.bold();
            }
            if cell.style.italic {
                styled = styled.italic();
            }
            if cell.style.underlined {
                styled = styled.underlined();
            }
            if cell.style.dim {
                styled = styled.dim();
            }
            if cell.style.crossed_out {
                styled = styled.crossed_out();
            }
            
            print_at(column, start_row + row_offset, &format!("{}", styled))?;
        }
    }
    
    Ok(())
}
