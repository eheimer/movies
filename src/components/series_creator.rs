use super::{Cell, Component, TextStyle};
use crate::theme::Theme;
use crossterm::style::Color;

/// Sub-component for handling new series name input with text editing capabilities
pub struct SeriesCreator {
    text: String,
    cursor_position: usize,
    field_width: usize,
}

impl SeriesCreator {
    /// Create a new SeriesCreator component
    pub fn new(text: String, cursor_position: usize, field_width: usize) -> Self {
        Self {
            text,
            cursor_position,
            field_width,
        }
    }
    
    /// Render the prompt text for series creation mode
    fn render_prompt(&self) -> String {
        "Type the series name and press [ENTER]:".to_string()
    }
    
    /// Render the input field with the current text
    fn render_input_field(&self) -> String {
        self.text.clone()
    }
}

impl Component for SeriesCreator {
    /// Renders the SeriesCreator component with prompt and text input field
    fn render(&self, width: usize, height: usize, _theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>> {
        let mut cells = Vec::new();
        
        // Use the actual height provided, don't force minimum
        let available_height = height;
        
        // First row: prompt text with inverted colors (black on white)
        let prompt_text = self.render_prompt();
        let mut prompt_row = Vec::new();
        
        // Truncate prompt if it's too long for the width
        let display_prompt = if prompt_text.len() > width {
            &prompt_text[..width]
        } else {
            &prompt_text
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
        
        // Fill remaining width with spaces (inverted background)
        while prompt_row.len() < width {
            prompt_row.push(Cell::new(
                ' ',
                Color::Black,
                Color::White,
                TextStyle::new(),
            ));
        }
        
        cells.push(prompt_row);
        
        // Second row: text input field (only if we have height for it)
        if available_height > 1 {
            let input_text = self.render_input_field();
            let mut input_row = Vec::new();
            
            // Ensure cursor position is within bounds
            let cursor_pos = self.cursor_position.min(input_text.len());
            
            // Truncate input text if it's too long for the width
            let display_text = if input_text.len() > width {
                &input_text[..width]
            } else {
                &input_text
            };
            
            // Handle empty text case - always show cursor at position 0
            if input_text.is_empty() {
                input_row.push(Cell::new(
                    ' ',
                    Color::White,
                    Color::Black,
                    TextStyle::new(),
                ));
            } else {
                // Add input text characters with cursor highlighting
                for (i, ch) in display_text.chars().enumerate() {
                    let is_cursor = i == cursor_pos;
                    let (fg_color, bg_color) = if is_cursor {
                        // Cursor position: inverted colors (white on black)
                        (Color::White, Color::Black)
                    } else {
                        // Normal text
                        (Color::Reset, Color::Reset)
                    };
                    
                    input_row.push(Cell::new(
                        ch,
                        fg_color,
                        bg_color,
                        TextStyle::new(),
                    ));
                }
                
                // If cursor is at the end of text, show cursor as space
                if cursor_pos >= input_text.len() && input_row.len() < width {
                    input_row.push(Cell::new(
                        ' ',
                        Color::White,
                        Color::Black,
                        TextStyle::new(),
                    ));
                }
            }
            
            // Fill remaining width with spaces
            while input_row.len() < width {
                input_row.push(Cell::new(
                    ' ',
                    Color::Reset,
                    Color::Reset,
                    TextStyle::new(),
                ));
            }
            
            cells.push(input_row);
        }
        
        // Fill remaining height with empty rows if needed
        while cells.len() < available_height {
            let mut empty_row = Vec::new();
            for _ in 0..width {
                empty_row.push(Cell::new(
                    ' ',
                    Color::Reset,
                    Color::Reset,
                    TextStyle::new(),
                ));
            }
            cells.push(empty_row);
        }
        
        cells
    }
}