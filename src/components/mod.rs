use crossterm::style::Color;
use crate::theme::Theme;

pub mod episode;
pub mod category;
pub mod scrollbar;
pub mod browser;

pub use category::*;
pub use scrollbar::Scrollbar;
pub use browser::Browser;

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

    /// Converts the Cell to crossterm's StyledContent for terminal output
    pub fn to_styled_content(&self) -> crossterm::style::StyledContent<char> {
        use crossterm::style::{Attribute, Stylize};
        
        let mut content = crossterm::style::StyledContent::new(
            crossterm::style::ContentStyle::new(),
            self.character,
        );
        
        content = content.with(self.fg_color).on(self.bg_color);
        
        if self.style.bold {
            content = content.attribute(Attribute::Bold);
        }
        if self.style.italic {
            content = content.attribute(Attribute::Italic);
        }
        if self.style.underlined {
            content = content.attribute(Attribute::Underlined);
        }
        if self.style.dim {
            content = content.attribute(Attribute::Dim);
        }
        if self.style.crossed_out {
            content = content.attribute(Attribute::CrossedOut);
        }
        
        content
    }
}

/// Trait for components that can render themselves to a 2D array of Cells
///
/// Components are self-contained rendering units that produce terminal output
/// without performing direct I/O operations. This separation allows components
/// to be tested independently and composed into larger UI structures.
pub trait Component {
    /// Renders the component to a 2D array of Cells
    ///
    /// # Parameters
    ///
    /// * `width` - Maximum width in characters for the rendered output.
    ///             Components should truncate or wrap content to fit within this constraint.
    /// * `theme` - Theme object containing color and style preferences.
    ///             Components use this to resolve colors and styling attributes.
    /// * `is_selected` - Whether this component represents the currently selected item.
    ///                   When true, components typically apply selection highlight colors.
    ///
    /// # Returns
    ///
    /// A 2D array of Cells where:
    /// * The outer Vec represents rows (vertical dimension)
    /// * Each inner Vec represents columns (horizontal dimension)
    /// * Single-line components return a Vec with one row
    /// * Multi-line components return multiple rows
    /// * Components must respect the width constraint for each row
    fn render(&self, width: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>>;
}

/// Renders a single-column multi-row Cell array to terminal output at specified position
///
/// This helper function converts Cell arrays (typically from vertical components like scrollbars)
/// to terminal output by positioning each row at the specified column and consecutive rows.
/// Each Cell's styling (colors, text attributes) is preserved in the terminal output.
///
/// # Parameters
///
/// * `cells` - The 2D array of Cells to render (expected to be single-column, multi-row)
/// * `column` - The column position where the output should be rendered
/// * `start_row` - The starting row position for the first Cell
///
/// # Returns
///
/// * `std::io::Result<()>` - Ok if successful, error if terminal I/O fails
///
/// # Example
///
/// ```rust,no_run
/// use movies::components::{render_cells_at_column, Cell, TextStyle};
/// use crossterm::style::Color;
///
/// # fn main() -> std::io::Result<()> {
/// let cells = vec![
///     vec![Cell::new('│', Color::White, Color::Black, TextStyle::new())],
///     vec![Cell::new('█', Color::Red, Color::Black, TextStyle::new())],
///     vec![Cell::new('│', Color::White, Color::Black, TextStyle::new())],
/// ];
/// 
/// // Render at column 10, starting from row 5
/// render_cells_at_column(&cells, 10, 5)?;
/// # Ok(())
/// # }
/// ```
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
