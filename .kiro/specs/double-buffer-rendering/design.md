# Design Document

## Overview

The double-buffer rendering system adds a thin abstraction layer between the existing display logic and terminal I/O. Instead of writing directly to the terminal, the system writes to a "desired buffer" representing the target screen state. This buffer is compared with the "current buffer" (what's actually on screen), and only the differences are written to the terminal.

**Key Design Principle:** Minimal changes to existing code. The buffer layer wraps terminal operations without requiring architectural changes to components, layout logic, or state management.

## Architecture

### Current Architecture
```
display.rs (draw_screen) → terminal I/O (print_at, etc.) → Terminal
```

### New Architecture
```
display.rs (draw_screen) → BufferWriter → Desired Buffer
                                              ↓
                                         Compare with Current Buffer
                                              ↓
                                         Write differences → Terminal
                                              ↓
                                         Update Current Buffer
```

### Critical Design Decision: Empty Buffer Start

**Problem from previous implementation:** Components drew over existing buffer content, creating visual artifacts.

**Solution:** Each frame starts with a completely empty Desired_Buffer. All components write to this clean slate, ensuring no overlap or artifacts.

```rust
// Start of each frame
desired_buffer.clear();  // Fill with empty/blank cells

// Components write to empty buffer
draw_header(&mut buffer_writer);
draw_browser(&mut buffer_writer);
draw_detail_panel(&mut buffer_writer);

// Now compare and update
buffer_manager.render_to_terminal();
```

## Components and Interfaces

### BufferManager

The main coordinator that manages both buffers and orchestrates the rendering pipeline.

```rust
pub struct BufferManager {
    current_buffer: ScreenBuffer,
    desired_buffer: ScreenBuffer,
    width: usize,
    height: usize,
}

impl BufferManager {
    /// Create new buffer manager with terminal dimensions
    pub fn new(width: usize, height: usize) -> Self;
    
    /// Clear desired buffer to empty state (start of frame)
    pub fn clear_desired_buffer(&mut self);
    
    /// Get a writer for drawing to the desired buffer
    pub fn get_writer(&mut self) -> BufferWriter;
    
    /// Compare buffers and write differences to terminal
    pub fn render_to_terminal(&mut self) -> io::Result<()>;
    
    /// Handle terminal resize
    pub fn resize(&mut self, width: usize, height: usize);
    
    /// Force full redraw (for mode changes)
    pub fn force_full_redraw(&mut self);
}
```

### ScreenBuffer

Represents the state of the terminal as a 2D array of cells.

```rust
pub struct ScreenBuffer {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl ScreenBuffer {
    pub fn new(width: usize, height: usize) -> Self;
    
    /// Clear all cells to empty/blank state
    pub fn clear(&mut self);
    
    /// Set a cell at position (x, y)
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell);
    
    /// Get a cell at position (x, y)
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell>;
    
    /// Check if two buffers differ at position (x, y)
    pub fn differs_at(&self, other: &ScreenBuffer, x: usize, y: usize) -> bool;
}
```

### Cell

Represents a single terminal character with styling.

```rust
#[derive(Clone, PartialEq, Eq)]
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
    pub fn empty() -> Self;
    
    /// Create a cell with character and colors
    pub fn new(character: char, fg_color: Color, bg_color: Color) -> Self;
}
```

### BufferWriter

A wrapper that intercepts terminal write operations and directs them to the desired buffer.

```rust
pub struct BufferWriter<'a> {
    buffer: &'a mut ScreenBuffer,
    current_x: usize,
    current_y: usize,
    current_fg: Color,
    current_bg: Color,
    current_style: TextStyle,
}

impl<'a> BufferWriter<'a> {
    /// Write a character at current position
    pub fn write_char(&mut self, c: char);
    
    /// Write a string at current position
    pub fn write_str(&mut self, s: &str);
    
    /// Move cursor to position
    pub fn move_to(&mut self, x: usize, y: usize);
    
    /// Set foreground color
    pub fn set_fg_color(&mut self, color: Color);
    
    /// Set background color
    pub fn set_bg_color(&mut self, color: Color);
    
    /// Set text style
    pub fn set_style(&mut self, style: TextStyle);
}
```

## Integration with Existing Code

### Minimal Changes to display.rs

```rust
// Before (current code)
pub fn draw_screen(
    entries: &[Entry],
    current_item: usize,
    // ... 21 more parameters
) -> io::Result<()> {
    clear_screen()?;
    print_at(0, 0, "Header")?;
    // ... more direct terminal writes
}

// After (with buffer layer)
pub fn draw_screen(
    entries: &[Entry],
    current_item: usize,
    // ... 21 more parameters
    buffer_manager: &mut BufferManager,  // Add one parameter
) -> io::Result<()> {
    // Clear desired buffer (start with empty slate)
    buffer_manager.clear_desired_buffer();
    
    // Get writer for this frame
    let mut writer = buffer_manager.get_writer();
    
    // Existing rendering logic, but write to buffer instead of terminal
    writer.move_to(0, 0);
    writer.write_str("Header");
    // ... more writes to buffer
    
    // Compare and update terminal
    buffer_manager.render_to_terminal()?;
    
    Ok(())
}
```

### Changes to main.rs

```rust
// In main_loop, create buffer manager once
let (terminal_width, terminal_height) = get_terminal_size()?;
let mut buffer_manager = BufferManager::new(terminal_width, terminal_height);

loop {
    if redraw {
        draw_screen(
            &filtered_entries,
            current_item,
            // ... all existing parameters
            &mut buffer_manager,  // Pass buffer manager
        )?;
        redraw = false;
    }
    
    // Handle terminal resize
    if let Event::Resize(width, height) = event {
        buffer_manager.resize(width as usize, height as usize);
        redraw = true;
    }
}
```

## Data Models

### Buffer State Machine

```
[Application Start]
       ↓
[Initialize Buffers] → Current Buffer (empty), Desired Buffer (empty)
       ↓
[Frame Start]
       ↓
[Clear Desired Buffer] → All cells set to empty/blank
       ↓
[Render Components] → Write to Desired Buffer
       ↓
[Compare Buffers] → Identify changed cells
       ↓
[Write to Terminal] → Only changed cells
       ↓
[Update Current Buffer] → Copy Desired → Current
       ↓
[Wait for Event]
       ↓
[Frame Start] (repeat)
```

### Cell Comparison Logic

Two cells are considered different if ANY of these differ:
- Character content
- Foreground color
- Background color
- Bold attribute
- Italic attribute
- Underlined attribute
- Dim attribute

## Error Handling

### Buffer Bounds Checking
- All buffer operations check bounds before writing
- Out-of-bounds writes are silently ignored (logged in debug mode)
- Prevents crashes from layout calculation errors

### Terminal I/O Errors
- Terminal write errors are propagated to caller
- Buffer state remains consistent even if terminal write fails
- Retry logic can be added at application level

### Resize Handling
- Terminal resize clears both buffers
- Forces full redraw on next frame
- Handles edge case of very small terminal sizes

## Testing Strategy

### Unit Testing
- Test buffer creation and initialization
- Test cell comparison logic
- Test buffer clearing (ensures empty state)
- Test bounds checking
- Test cell equality with various attributes

### Integration Testing
- Test full render cycle (clear → write → compare → update)
- Test that desired buffer starts empty each frame
- Test differential updates reduce terminal I/O
- Test terminal resize handling
- Test mode change full redraws

### Performance Testing
- Measure buffer comparison time
- Measure terminal I/O reduction (before/after)
- Verify no performance regression on large lists
- Test with various terminal sizes

## Test Cases

### Test Case 1: Empty buffer initialization

When starting a new frame, the desired buffer should be completely empty with all cells set to blank/space characters with default colors.
**Validates: Requirements 3.1, 3.2**

### Test Case 2: Differential update efficiency

When only a small portion of the screen changes (e.g., cursor movement), the system should write only the changed cells to the terminal, not the entire screen.
**Validates: Requirements 1.1, 1.2, 4.3**

### Test Case 3: No visual artifacts

When components render to the desired buffer, there should be no overlapping content or visual artifacts from previous frames.
**Validates: Requirements 3.3, 3.4, 3.5**

### Test Case 4: Terminal resize handling

When the terminal is resized, both buffers should be recreated with new dimensions and a full redraw should occur.
**Validates: Requirements 4.4, 6.1**

### Test Case 5: Mode change full redraw

When switching between modes (Browse, Edit, Menu, etc.), a full screen redraw should occur to ensure clean visual state.
**Validates: Requirements 6.2**

### Test Case 6: Cursor positioning efficiency

When editing text or navigating, cursor position updates should not trigger full screen redraws.
**Validates: Requirements 1.3, 5.2**

### Test Case 7: Buffer state consistency

When terminal I/O errors occur, the buffer state should remain consistent and not corrupt the current buffer.
**Validates: Requirements 5.5**
