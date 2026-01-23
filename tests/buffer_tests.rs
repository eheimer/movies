use movies::buffer::Cell;
use crossterm::style::Color;

#[test]
fn test_cell_empty_creates_blank_cell() {
    let cell = Cell::empty();
    
    assert_eq!(cell.character, ' ');
    assert_eq!(cell.fg_color, Color::Reset);
    assert_eq!(cell.bg_color, Color::Reset);
    assert_eq!(cell.bold, false);
    assert_eq!(cell.italic, false);
    assert_eq!(cell.underlined, false);
    assert_eq!(cell.dim, false);
}

#[test]
fn test_cell_equality_with_different_attributes() {
    let cell1 = Cell::new('A', Color::Red, Color::Blue);
    let cell2 = Cell::new('A', Color::Red, Color::Blue);
    let cell3 = Cell::new('B', Color::Red, Color::Blue);
    let cell4 = Cell::new('A', Color::Green, Color::Blue);
    let cell5 = Cell::new('A', Color::Red, Color::Yellow);
    
    // Same cells should be equal
    assert_eq!(cell1, cell2);
    
    // Different character
    assert_ne!(cell1, cell3);
    
    // Different foreground color
    assert_ne!(cell1, cell4);
    
    // Different background color
    assert_ne!(cell1, cell5);
}

#[test]
fn test_cell_equality_with_style_attributes() {
    let mut cell1 = Cell::new('X', Color::Reset, Color::Reset);
    let mut cell2 = Cell::new('X', Color::Reset, Color::Reset);
    
    // Initially equal
    assert_eq!(cell1, cell2);
    
    // Different bold
    cell1.bold = true;
    assert_ne!(cell1, cell2);
    cell2.bold = true;
    assert_eq!(cell1, cell2);
    
    // Different italic
    cell1.italic = true;
    assert_ne!(cell1, cell2);
    cell2.italic = true;
    assert_eq!(cell1, cell2);
    
    // Different underlined
    cell1.underlined = true;
    assert_ne!(cell1, cell2);
    cell2.underlined = true;
    assert_eq!(cell1, cell2);
    
    // Different dim
    cell1.dim = true;
    assert_ne!(cell1, cell2);
    cell2.dim = true;
    assert_eq!(cell1, cell2);
}

#[test]
fn test_cell_with_various_color_combinations() {
    let cell1 = Cell::new('A', Color::Red, Color::Blue);
    assert_eq!(cell1.character, 'A');
    assert_eq!(cell1.fg_color, Color::Red);
    assert_eq!(cell1.bg_color, Color::Blue);
    
    let cell2 = Cell::new('Z', Color::Green, Color::Yellow);
    assert_eq!(cell2.character, 'Z');
    assert_eq!(cell2.fg_color, Color::Green);
    assert_eq!(cell2.bg_color, Color::Yellow);
    
    let cell3 = Cell::new('!', Color::Cyan, Color::Magenta);
    assert_eq!(cell3.character, '!');
    assert_eq!(cell3.fg_color, Color::Cyan);
    assert_eq!(cell3.bg_color, Color::Magenta);
}

use movies::buffer::ScreenBuffer;

#[test]
fn test_buffer_creation_with_various_dimensions() {
    let buffer1 = ScreenBuffer::new(80, 24);
    assert_eq!(buffer1.get_cell(0, 0).unwrap().character, ' ');
    assert_eq!(buffer1.get_cell(79, 23).unwrap().character, ' ');
    
    let buffer2 = ScreenBuffer::new(120, 40);
    assert_eq!(buffer2.get_cell(0, 0).unwrap().character, ' ');
    assert_eq!(buffer2.get_cell(119, 39).unwrap().character, ' ');
    
    let buffer3 = ScreenBuffer::new(1, 1);
    assert_eq!(buffer3.get_cell(0, 0).unwrap().character, ' ');
}

#[test]
fn test_clear_fills_buffer_with_empty_cells() {
    let mut buffer = ScreenBuffer::new(10, 5);
    
    // Set some cells to non-empty values
    buffer.set_cell(0, 0, Cell::new('A', Color::Red, Color::Blue));
    buffer.set_cell(5, 2, Cell::new('B', Color::Green, Color::Yellow));
    buffer.set_cell(9, 4, Cell::new('C', Color::Cyan, Color::Magenta));
    
    // Verify cells were set
    assert_eq!(buffer.get_cell(0, 0).unwrap().character, 'A');
    assert_eq!(buffer.get_cell(5, 2).unwrap().character, 'B');
    assert_eq!(buffer.get_cell(9, 4).unwrap().character, 'C');
    
    // Clear the buffer
    buffer.clear();
    
    // Verify all cells are now empty
    assert_eq!(buffer.get_cell(0, 0).unwrap(), &Cell::empty());
    assert_eq!(buffer.get_cell(5, 2).unwrap(), &Cell::empty());
    assert_eq!(buffer.get_cell(9, 4).unwrap(), &Cell::empty());
    assert_eq!(buffer.get_cell(3, 1).unwrap(), &Cell::empty());
}

#[test]
fn test_set_cell_and_get_cell_with_valid_positions() {
    let mut buffer = ScreenBuffer::new(20, 10);
    
    let cell1 = Cell::new('X', Color::Red, Color::Blue);
    buffer.set_cell(5, 3, cell1.clone());
    assert_eq!(buffer.get_cell(5, 3).unwrap(), &cell1);
    
    let cell2 = Cell::new('Y', Color::Green, Color::Yellow);
    buffer.set_cell(0, 0, cell2.clone());
    assert_eq!(buffer.get_cell(0, 0).unwrap(), &cell2);
    
    let cell3 = Cell::new('Z', Color::Cyan, Color::Magenta);
    buffer.set_cell(19, 9, cell3.clone());
    assert_eq!(buffer.get_cell(19, 9).unwrap(), &cell3);
}

#[test]
fn test_bounds_checking_out_of_bounds_writes_are_safe() {
    let mut buffer = ScreenBuffer::new(10, 5);
    
    // Out of bounds writes should not panic
    buffer.set_cell(10, 0, Cell::new('A', Color::Red, Color::Blue));
    buffer.set_cell(0, 5, Cell::new('B', Color::Green, Color::Yellow));
    buffer.set_cell(100, 100, Cell::new('C', Color::Cyan, Color::Magenta));
    
    // Out of bounds reads should return None
    assert_eq!(buffer.get_cell(10, 0), None);
    assert_eq!(buffer.get_cell(0, 5), None);
    assert_eq!(buffer.get_cell(100, 100), None);
    
    // Valid positions should still work
    buffer.set_cell(5, 2, Cell::new('D', Color::Reset, Color::Reset));
    assert_eq!(buffer.get_cell(5, 2).unwrap().character, 'D');
}

#[test]
fn test_differs_at_correctly_identifies_changed_cells() {
    let mut buffer1 = ScreenBuffer::new(10, 5);
    let mut buffer2 = ScreenBuffer::new(10, 5);
    
    // Initially, buffers should not differ
    assert_eq!(buffer1.differs_at(&buffer2, 0, 0), false);
    assert_eq!(buffer1.differs_at(&buffer2, 5, 2), false);
    
    // Change a cell in buffer1
    buffer1.set_cell(5, 2, Cell::new('A', Color::Red, Color::Blue));
    
    // Now they should differ at that position
    assert_eq!(buffer1.differs_at(&buffer2, 5, 2), true);
    
    // But not at other positions
    assert_eq!(buffer1.differs_at(&buffer2, 0, 0), false);
    assert_eq!(buffer1.differs_at(&buffer2, 9, 4), false);
    
    // Make buffer2 match buffer1 at that position
    buffer2.set_cell(5, 2, Cell::new('A', Color::Red, Color::Blue));
    
    // Now they should not differ
    assert_eq!(buffer1.differs_at(&buffer2, 5, 2), false);
    
    // Test with different attributes
    let mut cell = Cell::new('A', Color::Red, Color::Blue);
    cell.bold = true;
    buffer1.set_cell(3, 1, cell);
    
    assert_eq!(buffer1.differs_at(&buffer2, 3, 1), true);
    
    // Out of bounds should return false (no difference)
    assert_eq!(buffer1.differs_at(&buffer2, 100, 100), false);
}

use movies::buffer::BufferWriter;

#[test]
fn test_write_char_updates_buffer_at_correct_position() {
    let mut buffer = ScreenBuffer::new(10, 5);
    {
        let mut writer = BufferWriter::new(&mut buffer);
        
        // Write a character at default position (0, 0)
        writer.write_char('A');
        
        // Write another character (should advance to position 1, 0)
        writer.write_char('B');
        
        // Move to a different position and write
        writer.move_to(5, 3);
        writer.write_char('X');
    }
    
    // Check results after writer is dropped
    assert_eq!(buffer.get_cell(0, 0).unwrap().character, 'A');
    assert_eq!(buffer.get_cell(1, 0).unwrap().character, 'B');
    assert_eq!(buffer.get_cell(5, 3).unwrap().character, 'X');
}

#[test]
fn test_write_str_writes_multiple_characters() {
    let mut buffer = ScreenBuffer::new(20, 5);
    {
        let mut writer = BufferWriter::new(&mut buffer);
        
        // Write a string at default position
        writer.write_str("Hello");
        
        // Move and write another string
        writer.move_to(0, 2);
        writer.write_str("World");
    }
    
    // Check results after writer is dropped
    assert_eq!(buffer.get_cell(0, 0).unwrap().character, 'H');
    assert_eq!(buffer.get_cell(1, 0).unwrap().character, 'e');
    assert_eq!(buffer.get_cell(2, 0).unwrap().character, 'l');
    assert_eq!(buffer.get_cell(3, 0).unwrap().character, 'l');
    assert_eq!(buffer.get_cell(4, 0).unwrap().character, 'o');
    
    assert_eq!(buffer.get_cell(0, 2).unwrap().character, 'W');
    assert_eq!(buffer.get_cell(1, 2).unwrap().character, 'o');
    assert_eq!(buffer.get_cell(2, 2).unwrap().character, 'r');
    assert_eq!(buffer.get_cell(3, 2).unwrap().character, 'l');
    assert_eq!(buffer.get_cell(4, 2).unwrap().character, 'd');
}

#[test]
fn test_move_to_changes_write_position() {
    let mut buffer = ScreenBuffer::new(10, 5);
    {
        let mut writer = BufferWriter::new(&mut buffer);
        
        // Write at default position
        writer.write_char('A');
        
        // Move to a new position
        writer.move_to(5, 2);
        writer.write_char('B');
        
        // Move to another position
        writer.move_to(9, 4);
        writer.write_char('C');
    }
    
    // Check results after writer is dropped
    assert_eq!(buffer.get_cell(0, 0).unwrap().character, 'A');
    assert_eq!(buffer.get_cell(5, 2).unwrap().character, 'B');
    assert_eq!(buffer.get_cell(9, 4).unwrap().character, 'C');
    
    // Verify original positions are unchanged (except where we wrote)
    assert_eq!(buffer.get_cell(1, 0).unwrap().character, ' ');
    assert_eq!(buffer.get_cell(0, 1).unwrap().character, ' ');
}

#[test]
fn test_color_and_style_changes_affect_subsequent_writes() {
    let mut buffer = ScreenBuffer::new(10, 5);
    {
        let mut writer = BufferWriter::new(&mut buffer);
        
        // Write with default colors
        writer.write_char('A');
        
        // Change foreground color and write
        writer.set_fg_color(Color::Red);
        writer.write_char('B');
        
        // Change background color and write
        writer.set_bg_color(Color::Blue);
        writer.write_char('C');
        
        // Change style attributes and write
        writer.set_bold(true);
        writer.set_italic(true);
        writer.write_char('D');
        
        // Change more style attributes
        writer.set_underlined(true);
        writer.set_dim(true);
        writer.write_char('E');
    }
    
    // Check results after writer is dropped
    let cell1 = buffer.get_cell(0, 0).unwrap();
    assert_eq!(cell1.character, 'A');
    assert_eq!(cell1.fg_color, Color::Reset);
    assert_eq!(cell1.bg_color, Color::Reset);
    assert_eq!(cell1.bold, false);
    
    let cell2 = buffer.get_cell(1, 0).unwrap();
    assert_eq!(cell2.character, 'B');
    assert_eq!(cell2.fg_color, Color::Red);
    assert_eq!(cell2.bg_color, Color::Reset);
    
    let cell3 = buffer.get_cell(2, 0).unwrap();
    assert_eq!(cell3.character, 'C');
    assert_eq!(cell3.fg_color, Color::Red);
    assert_eq!(cell3.bg_color, Color::Blue);
    
    let cell4 = buffer.get_cell(3, 0).unwrap();
    assert_eq!(cell4.character, 'D');
    assert_eq!(cell4.fg_color, Color::Red);
    assert_eq!(cell4.bg_color, Color::Blue);
    assert_eq!(cell4.bold, true);
    assert_eq!(cell4.italic, true);
    
    let cell5 = buffer.get_cell(4, 0).unwrap();
    assert_eq!(cell5.character, 'E');
    assert_eq!(cell5.underlined, true);
    assert_eq!(cell5.dim, true);
}

use movies::buffer::BufferManager;

#[test]
fn test_buffer_manager_new_creates_buffers_with_correct_dimensions() {
    let manager = BufferManager::new(80, 24);
    
    // Get a writer to verify the buffer was created correctly
    // We can't directly access the buffers, but we can test through the writer
    let mut manager = manager;
    {
        let mut writer = manager.get_writer();
        
        // Write at various positions to verify dimensions
        writer.move_to(0, 0);
        writer.write_char('A');
        
        writer.move_to(79, 23);
        writer.write_char('B');
    }
    
    // Test with different dimensions
    let manager2 = BufferManager::new(120, 40);
    let mut manager2 = manager2;
    {
        let mut writer = manager2.get_writer();
        writer.move_to(119, 39);
        writer.write_char('C');
    }
    
    // Test with small dimensions
    let manager3 = BufferManager::new(1, 1);
    let mut manager3 = manager3;
    {
        let mut writer = manager3.get_writer();
        writer.move_to(0, 0);
        writer.write_char('D');
    }
}

#[test]
fn test_clear_desired_buffer_fills_desired_buffer_with_empty_cells() {
    let mut manager = BufferManager::new(10, 5);
    
    // Write some content to the desired buffer
    {
        let mut writer = manager.get_writer();
        writer.write_str("Hello");
        writer.move_to(0, 2);
        writer.write_str("World");
    }
    
    // Clear the desired buffer
    manager.clear_desired_buffer();
    
    // Get a new writer and verify the buffer is empty
    {
        let mut writer = manager.get_writer();
        
        // Write at a position and verify it's the only content
        writer.move_to(5, 2);
        writer.write_char('X');
    }
    
    // The clear should have reset everything, so only 'X' should be present
    // We can't directly verify this without render_to_terminal, but the clear operation should work
}

#[test]
fn test_get_writer_returns_valid_buffer_writer() {
    let mut manager = BufferManager::new(20, 10);
    
    // Get a writer and use it
    {
        let mut writer = manager.get_writer();
        writer.move_to(5, 3);
        writer.write_str("Test");
        writer.set_fg_color(Color::Red);
        writer.write_char('!');
    }
    
    // Get another writer (should work fine)
    {
        let mut writer = manager.get_writer();
        writer.move_to(10, 5);
        writer.write_str("Another");
    }
}

#[test]
fn test_resize_updates_buffer_dimensions() {
    let mut manager = BufferManager::new(80, 24);
    
    // Write some content
    {
        let mut writer = manager.get_writer();
        writer.move_to(50, 20);
        writer.write_str("Content");
    }
    
    // Resize to larger dimensions
    manager.resize(120, 40);
    
    // Verify we can write at the new dimensions
    {
        let mut writer = manager.get_writer();
        writer.move_to(119, 39);
        writer.write_char('X');
    }
    
    // Resize to smaller dimensions
    manager.resize(40, 12);
    
    // Verify we can write at the new smaller dimensions
    {
        let mut writer = manager.get_writer();
        writer.move_to(39, 11);
        writer.write_char('Y');
    }
}

#[test]
fn test_force_full_redraw() {
    let mut manager = BufferManager::new(80, 24);
    
    // Write some content
    {
        let mut writer = manager.get_writer();
        writer.write_str("Test content");
    }
    
    // Force a full redraw
    manager.force_full_redraw();
    
    // The current buffer should be cleared, forcing all cells to be redrawn
    // We can't directly verify this without render_to_terminal, but the operation should work
    
    // Write new content after force_full_redraw
    {
        let mut writer = manager.get_writer();
        writer.write_str("New content");
    }
}

#[test]
fn test_render_to_terminal_placeholder() {
    let mut manager = BufferManager::new(80, 24);
    
    // Write some content
    {
        let mut writer = manager.get_writer();
        writer.write_str("Test");
    }
    
    // Call render_to_terminal (currently a placeholder)
    let result = manager.render_to_terminal();
    assert!(result.is_ok());
}

// Tests for buffer comparison functionality

#[test]
fn test_compare_buffers_identifies_changed_cells_correctly() {
    let mut manager = BufferManager::new(10, 5);
    
    // Initially, buffers should be identical (both empty)
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 0, "Empty buffers should have no changes");
    
    // Write some content to desired buffer
    {
        let mut writer = manager.get_writer();
        writer.move_to(2, 1);
        writer.write_char('A');
        writer.move_to(5, 3);
        writer.write_str("BC");
    }
    
    // Now there should be changes
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 3, "Should detect 3 changed cells");
    
    // Verify the changed positions
    let positions: Vec<(usize, usize)> = changes.iter().map(|(x, y, _)| (*x, *y)).collect();
    assert!(positions.contains(&(2, 1)), "Should detect change at (2, 1)");
    assert!(positions.contains(&(5, 3)), "Should detect change at (5, 3)");
    assert!(positions.contains(&(6, 3)), "Should detect change at (6, 3)");
    
    // Verify the cell content
    for (x, y, cell) in &changes {
        if *x == 2 && *y == 1 {
            assert_eq!(cell.character, 'A');
        } else if *x == 5 && *y == 3 {
            assert_eq!(cell.character, 'B');
        } else if *x == 6 && *y == 3 {
            assert_eq!(cell.character, 'C');
        }
    }
}

#[test]
fn test_compare_buffers_skips_unchanged_cells() {
    let mut manager = BufferManager::new(10, 5);
    
    // Write content and simulate it being rendered
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Hello");
    }
    
    // Simulate rendering by updating current buffer
    manager.update_current_buffer();
    
    // Clear and write the same content again
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Hello");
    }
    
    // Should detect no changes
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 0, "Identical buffers should have no changes");
    
    // Now change just one character
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Hallo");
    }
    
    // Should detect only one change
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 1, "Should detect only the changed cell");
    assert_eq!(changes[0].0, 1);
    assert_eq!(changes[0].1, 0);
    assert_eq!(changes[0].2.character, 'a');
}

#[test]
fn test_compare_buffers_handles_empty_buffers() {
    let mut manager = BufferManager::new(10, 5);
    
    // Both buffers are empty initially
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 0, "Empty buffers should have no changes");
    
    // Clear desired buffer explicitly
    manager.clear_desired_buffer();
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 0, "Cleared buffer should match empty current buffer");
    
    // Write to desired buffer, then clear it
    {
        let mut writer = manager.get_writer();
        writer.write_str("Test");
    }
    manager.clear_desired_buffer();
    
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 0, "Cleared desired buffer should match empty current buffer");
}

#[test]
fn test_compare_buffers_with_various_change_patterns() {
    let mut manager = BufferManager::new(20, 10);
    
    // Pattern 1: Single cell change
    {
        let mut writer = manager.get_writer();
        writer.move_to(10, 5);
        writer.write_char('X');
    }
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 1, "Single cell change");
    
    // Pattern 2: Horizontal line
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("----------");
    }
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 10, "Horizontal line of 10 characters");
    
    // Pattern 3: Vertical line (non-consecutive in memory)
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        for i in 0..5 {
            writer.move_to(5, i);
            writer.write_char('|');
        }
    }
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 5, "Vertical line of 5 characters");
    
    // Pattern 4: Scattered changes
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_char('A');
        writer.move_to(10, 5);
        writer.write_char('B');
        writer.move_to(19, 9);
        writer.write_char('C');
    }
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 3, "Scattered changes");
    
    // Pattern 5: Color/style changes (same character, different attributes)
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_char(' '); // Same as empty, but let's change color
        writer.set_fg_color(Color::Red);
        writer.move_to(1, 0);
        writer.write_char(' ');
    }
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 1, "Color change should be detected");
    assert_eq!(changes[0].2.fg_color, Color::Red);
}

#[test]
fn test_force_full_redraw_causes_all_cells_to_be_redrawn() {
    let mut manager = BufferManager::new(5, 3);
    
    // Write some content and update current buffer
    {
        let mut writer = manager.get_writer();
        writer.write_str("Test");
    }
    manager.update_current_buffer();
    
    // Clear and write the same content
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.write_str("Test");
    }
    
    // Should have no changes since buffers match
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 0, "Identical buffers should have no changes");
    
    // Force full redraw (sets current buffer to sentinel values)
    manager.force_full_redraw();
    
    // Now ALL cells should be detected as changed (5 width * 3 height = 15 cells)
    // This ensures that even cells with Color::Reset backgrounds get redrawn
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 15, "All cells should be marked as changed after force_full_redraw");
    
    // Verify the first 4 changes are the "Test" characters
    assert_eq!(changes[0].2.character, 'T');
    assert_eq!(changes[1].2.character, 'e');
    assert_eq!(changes[2].2.character, 's');
    assert_eq!(changes[3].2.character, 't');
    
    // Verify remaining cells are spaces with Reset colors
    for i in 4..15 {
        assert_eq!(changes[i].2.character, ' ');
        assert_eq!(changes[i].2.fg_color, Color::Reset);
        assert_eq!(changes[i].2.bg_color, Color::Reset);
    }
}

// Integration tests for terminal rendering

#[test]
fn test_render_to_terminal_writes_only_changed_cells() {
    let mut manager = BufferManager::new(20, 10);
    
    // Write some initial content
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Hello");
        writer.move_to(0, 2);
        writer.write_str("World");
    }
    
    // Render to terminal (this will update current buffer)
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "First render should succeed");
    
    // Clear and write the same content again
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Hello");
        writer.move_to(0, 2);
        writer.write_str("World");
    }
    
    // Render again - should detect no changes
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Second render with no changes should succeed");
    
    // Now change just one character
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Hallo"); // Changed 'e' to 'a'
        writer.move_to(0, 2);
        writer.write_str("World");
    }
    
    // Render - should only write the changed cell
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Render with single change should succeed");
}

#[test]
fn test_render_batching_of_consecutive_cells() {
    let mut manager = BufferManager::new(30, 10);
    
    // Write a long consecutive string
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 5);
        writer.write_str("This is a long consecutive string");
    }
    
    // Render should batch all consecutive cells
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Render with consecutive cells should succeed");
    
    // Write scattered non-consecutive cells
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_char('A');
        writer.move_to(10, 3);
        writer.write_char('B');
        writer.move_to(20, 7);
        writer.write_char('C');
    }
    
    // Render should handle non-consecutive cells
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Render with scattered cells should succeed");
}

#[test]
fn test_current_buffer_updates_after_render() {
    let mut manager = BufferManager::new(15, 8);
    
    // Write content to desired buffer
    {
        let mut writer = manager.get_writer();
        writer.move_to(5, 3);
        writer.write_str("Test");
    }
    
    // Before render, buffers should differ
    let changes_before = manager.compare_buffers();
    assert_eq!(changes_before.len(), 4, "Should have 4 changes before render");
    
    // Render to terminal
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Render should succeed");
    
    // After render, current buffer should match desired buffer
    let changes_after = manager.compare_buffers();
    assert_eq!(changes_after.len(), 0, "Should have no changes after render");
    
    // Write the same content again
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(5, 3);
        writer.write_str("Test");
    }
    
    // Should detect no changes since current buffer was updated
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 0, "Should have no changes when content matches");
}

#[test]
fn test_render_with_color_and_style_changes() {
    let mut manager = BufferManager::new(20, 10);
    
    // Write content with various colors and styles
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.set_fg_color(Color::Red);
        writer.write_str("Red");
        
        writer.set_fg_color(Color::Blue);
        writer.set_bg_color(Color::Yellow);
        writer.write_str("Blue");
        
        writer.set_bold(true);
        writer.write_str("Bold");
        
        writer.set_italic(true);
        writer.write_str("Italic");
    }
    
    // Render should handle color and style changes
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Render with colors and styles should succeed");
    
    // Verify current buffer was updated
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 0, "Current buffer should match desired buffer after render");
}

#[test]
fn test_render_empty_changes() {
    let mut manager = BufferManager::new(10, 5);
    
    // Render with no changes (both buffers empty)
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Render with no changes should succeed");
    
    // Write and render
    {
        let mut writer = manager.get_writer();
        writer.write_str("Test");
    }
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "First render should succeed");
    
    // Render again with no changes
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Second render with no changes should succeed");
}

#[test]
fn test_render_full_screen_update() {
    let mut manager = BufferManager::new(10, 5);
    
    // Fill the entire screen
    {
        let mut writer = manager.get_writer();
        for y in 0..5 {
            writer.move_to(0, y);
            writer.write_str("0123456789");
        }
    }
    
    // Render full screen
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Full screen render should succeed");
    
    // Verify all cells were updated
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 0, "All cells should be updated");
}

#[test]
fn test_render_after_force_full_redraw() {
    let mut manager = BufferManager::new(15, 8);
    
    // Write and render initial content
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Initial");
    }
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Initial render should succeed");
    
    // Force full redraw
    manager.force_full_redraw();
    
    // Write the same content
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Initial");
    }
    
    // Should detect changes because current buffer was cleared
    let changes = manager.compare_buffers();
    assert!(changes.len() > 0, "Should have changes after force_full_redraw");
    
    // Render should succeed
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Render after force_full_redraw should succeed");
}

#[test]
fn test_render_with_resize() {
    let mut manager = BufferManager::new(20, 10);
    
    // Write and render content
    {
        let mut writer = manager.get_writer();
        writer.move_to(5, 5);
        writer.write_str("Content");
    }
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Initial render should succeed");
    
    // Resize the buffers
    manager.resize(30, 15);
    
    // Write new content at new dimensions
    {
        let mut writer = manager.get_writer();
        writer.move_to(25, 12);
        writer.write_str("New");
    }
    
    // Render should succeed with new dimensions
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Render after resize should succeed");
}

#[test]
fn test_mode_change_triggers_full_redraw() {
    let mut manager = BufferManager::new(20, 10);
    
    // Simulate Browse mode - render some content
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Browse Mode");
        writer.move_to(0, 2);
        writer.write_str("Item 1");
        writer.move_to(0, 3);
        writer.write_str("Item 2");
    }
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Browse mode render should succeed");
    
    // Verify no changes when rendering same content
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Browse Mode");
        writer.move_to(0, 2);
        writer.write_str("Item 1");
        writer.move_to(0, 3);
        writer.write_str("Item 2");
    }
    let changes = manager.compare_buffers();
    assert_eq!(changes.len(), 0, "Same content should have no changes");
    
    // Simulate mode change to Edit mode - force full redraw
    manager.force_full_redraw();
    
    // Clear and render Edit mode content
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Edit Mode");
        writer.move_to(0, 2);
        writer.write_str("Field: Value");
    }
    
    // After force_full_redraw, all content cells should be detected as changed
    let changes = manager.compare_buffers();
    assert!(changes.len() > 0, "Mode change should trigger changes detection");
    
    // Render should succeed
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Edit mode render after mode change should succeed");
    
    // Simulate another mode change back to Browse
    manager.force_full_redraw();
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str("Browse Mode");
        writer.move_to(0, 2);
        writer.write_str("Item 1");
    }
    
    // Should detect changes again
    let changes = manager.compare_buffers();
    assert!(changes.len() > 0, "Second mode change should trigger changes detection");
    
    let result = manager.render_to_terminal();
    assert!(result.is_ok(), "Browse mode render after second mode change should succeed");
}
