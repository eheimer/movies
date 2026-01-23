use crossterm::style::Color;
use std::io::{self, Write};
use crossterm::{
    cursor,
    execute,
    style::{Attribute, SetAttribute, SetBackgroundColor, SetForegroundColor},
};

/// Represents a single terminal cell with character, colors, and style attributes
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cell {
    pub character: char,
    pub fg_color: Color,
    pub bg_color: Color,
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub dim: bool,
}

impl Cell {
    /// Create an empty/blank cell (space with default colors)
    pub fn empty() -> Self {
        Cell {
            character: ' ',
            fg_color: Color::Reset,
            bg_color: Color::Reset,
            bold: false,
            italic: false,
            underlined: false,
            dim: false,
        }
    }

    /// Create a cell with character and colors
    pub fn new(character: char, fg_color: Color, bg_color: Color) -> Self {
        Cell {
            character,
            fg_color,
            bg_color,
            bold: false,
            italic: false,
            underlined: false,
            dim: false,
        }
    }
}

/// Represents the state of the terminal as a 2D array of cells
pub struct ScreenBuffer {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl ScreenBuffer {
    /// Create a new buffer with specified dimensions
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![Cell::empty(); width]; height];
        ScreenBuffer {
            cells,
            width,
            height,
        }
    }

    /// Clear all cells to empty/blank state
    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = Cell::empty();
            }
        }
    }

    /// Set a cell at position (x, y) with bounds checking
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if y < self.height && x < self.width {
            self.cells[y][x] = cell;
        }
    }

    /// Get a cell at position (x, y) with bounds checking
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if y < self.height && x < self.width {
            Some(&self.cells[y][x])
        } else {
            None
        }
    }

    /// Check if two buffers differ at position (x, y)
    pub fn differs_at(&self, other: &ScreenBuffer, x: usize, y: usize) -> bool {
        match (self.get_cell(x, y), other.get_cell(x, y)) {
            (Some(cell1), Some(cell2)) => cell1 != cell2,
            _ => false,
        }
    }
}

/// A wrapper that intercepts terminal write operations and directs them to the desired buffer
pub struct BufferWriter<'a> {
    buffer: &'a mut ScreenBuffer,
    current_x: usize,
    current_y: usize,
    current_fg: Color,
    current_bg: Color,
    current_bold: bool,
    current_italic: bool,
    current_underlined: bool,
    current_dim: bool,
}

impl<'a> BufferWriter<'a> {
    /// Create a new BufferWriter for the given buffer
    pub fn new(buffer: &'a mut ScreenBuffer) -> Self {
        BufferWriter {
            buffer,
            current_x: 0,
            current_y: 0,
            current_fg: Color::Reset,
            current_bg: Color::Reset,
            current_bold: false,
            current_italic: false,
            current_underlined: false,
            current_dim: false,
        }
    }

    /// Write a character at current position
    pub fn write_char(&mut self, c: char) {
        let cell = Cell {
            character: c,
            fg_color: self.current_fg,
            bg_color: self.current_bg,
            bold: self.current_bold,
            italic: self.current_italic,
            underlined: self.current_underlined,
            dim: self.current_dim,
        };
        self.buffer.set_cell(self.current_x, self.current_y, cell);
        self.current_x += 1;
    }

    /// Write a string at current position
    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    /// Move cursor to position
    pub fn move_to(&mut self, x: usize, y: usize) {
        self.current_x = x;
        self.current_y = y;
    }

    /// Set foreground color
    pub fn set_fg_color(&mut self, color: Color) {
        self.current_fg = color;
    }

    /// Set background color
    pub fn set_bg_color(&mut self, color: Color) {
        self.current_bg = color;
    }

    /// Set bold style
    pub fn set_bold(&mut self, bold: bool) {
        self.current_bold = bold;
    }

    /// Set italic style
    pub fn set_italic(&mut self, italic: bool) {
        self.current_italic = italic;
    }

    /// Set underlined style
    pub fn set_underlined(&mut self, underlined: bool) {
        self.current_underlined = underlined;
    }

    /// Set dim style
    pub fn set_dim(&mut self, dim: bool) {
        self.current_dim = dim;
    }
}

/// Main coordinator that manages both buffers and orchestrates the rendering pipeline
pub struct BufferManager {
    current_buffer: ScreenBuffer,
    desired_buffer: ScreenBuffer,
    width: usize,
    height: usize,
}

impl BufferManager {
    /// Create new buffer manager with terminal dimensions
    pub fn new(width: usize, height: usize) -> Self {
        BufferManager {
            current_buffer: ScreenBuffer::new(width, height),
            desired_buffer: ScreenBuffer::new(width, height),
            width,
            height,
        }
    }

    /// Clear desired buffer to empty state (start of frame)
    pub fn clear_desired_buffer(&mut self) {
        self.desired_buffer.clear();
    }

    /// Get a writer for drawing to the desired buffer
    pub fn get_writer(&mut self) -> BufferWriter<'_> {
        BufferWriter::new(&mut self.desired_buffer)
    }

    /// Handle terminal resize
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.current_buffer = ScreenBuffer::new(width, height);
        self.desired_buffer = ScreenBuffer::new(width, height);
    }

    /// Force full redraw (for mode changes)
    /// Sets current buffer to a sentinel value that will never match desired buffer
    pub fn force_full_redraw(&mut self) {
        // Use a sentinel cell that will never match any real cell
        // This ensures all cells are marked as changed
        let sentinel = Cell {
            character: '\0',  // Null character - never used in real content
            fg_color: Color::Reset,
            bg_color: Color::Reset,
            bold: false,
            italic: false,
            underlined: false,
            dim: false,
        };
        
        for row in &mut self.current_buffer.cells {
            for cell in row {
                *cell = sentinel.clone();
            }
        }
    }

    /// Compare buffers and identify changed cells
    /// Returns a list of (x, y, cell) tuples for positions that differ
    pub fn compare_buffers(&self) -> Vec<(usize, usize, Cell)> {
        let mut changes = Vec::new();
        
        for y in 0..self.height {
            for x in 0..self.width {
                if self.current_buffer.differs_at(&self.desired_buffer, x, y) {
                    if let Some(cell) = self.desired_buffer.get_cell(x, y) {
                        changes.push((x, y, cell.clone()));
                    }
                }
            }
        }
        
        changes
    }

    /// Update current buffer to match desired buffer (after successful render)
    pub fn update_current_buffer(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.desired_buffer.get_cell(x, y) {
                    self.current_buffer.set_cell(x, y, cell.clone());
                }
            }
        }
    }

    /// Compare buffers and write differences to terminal
    pub fn render_to_terminal(&mut self) -> io::Result<()> {
        let changes = self.compare_buffers();
        
        if changes.is_empty() {
            return Ok(());
        }
        
        let mut stdout = io::stdout();
        
        // Group consecutive cells on the same row for batching
        let mut batches: Vec<Vec<(usize, usize, Cell)>> = Vec::new();
        let mut current_batch: Vec<(usize, usize, Cell)> = Vec::new();
        
        for (x, y, cell) in changes {
            if let Some((last_x, last_y, _)) = current_batch.last() {
                // Check if this cell is consecutive on the same row
                if y == *last_y && x == last_x + 1 {
                    current_batch.push((x, y, cell));
                } else {
                    // Start a new batch
                    if !current_batch.is_empty() {
                        batches.push(current_batch);
                    }
                    current_batch = vec![(x, y, cell)];
                }
            } else {
                // First cell
                current_batch.push((x, y, cell));
            }
        }
        
        // Don't forget the last batch
        if !current_batch.is_empty() {
            batches.push(current_batch);
        }
        
        // Write each batch to the terminal
        for batch in batches {
            if batch.is_empty() {
                continue;
            }
            
            // Move cursor to the start of the batch
            let (start_x, start_y, _) = &batch[0];
            execute!(stdout, cursor::MoveTo(*start_x as u16, *start_y as u16))?;
            
            // Track current style to minimize escape sequences
            let mut current_fg: Option<Color> = None;
            let mut current_bg: Option<Color> = None;
            let mut current_bold = false;
            let mut current_italic = false;
            let mut current_underlined = false;
            let mut current_dim = false;
            
            // Write all cells in the batch
            for (_, _, cell) in batch {
                // Update foreground color if changed
                if current_fg.as_ref() != Some(&cell.fg_color) {
                    execute!(stdout, SetForegroundColor(cell.fg_color))?;
                    current_fg = Some(cell.fg_color);
                }
                
                // Update background color if changed
                if current_bg.as_ref() != Some(&cell.bg_color) {
                    execute!(stdout, SetBackgroundColor(cell.bg_color))?;
                    current_bg = Some(cell.bg_color);
                }
                
                // Update bold attribute if changed
                if current_bold != cell.bold {
                    if cell.bold {
                        execute!(stdout, SetAttribute(Attribute::Bold))?;
                    } else {
                        execute!(stdout, SetAttribute(Attribute::NormalIntensity))?;
                    }
                    current_bold = cell.bold;
                }
                
                // Update italic attribute if changed
                if current_italic != cell.italic {
                    if cell.italic {
                        execute!(stdout, SetAttribute(Attribute::Italic))?;
                    } else {
                        execute!(stdout, SetAttribute(Attribute::NoItalic))?;
                    }
                    current_italic = cell.italic;
                }
                
                // Update underlined attribute if changed
                if current_underlined != cell.underlined {
                    if cell.underlined {
                        execute!(stdout, SetAttribute(Attribute::Underlined))?;
                    } else {
                        execute!(stdout, SetAttribute(Attribute::NoUnderline))?;
                    }
                    current_underlined = cell.underlined;
                }
                
                // Update dim attribute if changed
                if current_dim != cell.dim {
                    if cell.dim {
                        execute!(stdout, SetAttribute(Attribute::Dim))?;
                    } else {
                        execute!(stdout, SetAttribute(Attribute::NormalIntensity))?;
                    }
                    current_dim = cell.dim;
                }
                
                // Write the character
                write!(stdout, "{}", cell.character)?;
            }
            
            // Reset attributes after each batch to avoid style bleeding
            execute!(stdout, SetAttribute(Attribute::Reset))?;
        }
        
        // Flush to ensure all writes are sent to terminal
        stdout.flush()?;
        
        // Update current buffer to match desired buffer after successful write
        self.update_current_buffer();
        
        Ok(())
    }
}
