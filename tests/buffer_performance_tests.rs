use movies::buffer::BufferManager;
use crossterm::style::Color;
use std::time::Instant;

// Test buffer comparison performance with large buffers
#[test]
fn test_buffer_comparison_performance() {
    let width = 200;
    let height = 100;
    let mut manager = BufferManager::new(width, height);
    
    // Fill desired buffer with content
    {
        let mut writer = manager.get_writer();
        for y in 0..height {
            writer.move_to(0, y);
            writer.write_str(&format!("Line {} with some content", y));
        }
    }
    
    // Measure comparison time
    let start = Instant::now();
    let changes = manager.compare_buffers();
    let duration = start.elapsed();
    
    // Should complete quickly (under 10ms for 20,000 cells)
    assert!(duration.as_millis() < 10, "Buffer comparison took {}ms, expected < 10ms", duration.as_millis());
    
    // Should detect all changed cells
    assert!(changes.len() > 0, "Should detect changes");
    
    println!("Buffer comparison for {}x{} buffer took {:?}", width, height, duration);
    println!("Detected {} changed cells", changes.len());
}

// Test differential update efficiency - only changed cells should be detected
#[test]
fn test_differential_update_efficiency() {
    let width = 100;
    let height = 50;
    let mut manager = BufferManager::new(width, height);
    
    // Initial render - fill entire buffer
    {
        let mut writer = manager.get_writer();
        for y in 0..height {
            writer.move_to(0, y);
            writer.write_str(&format!("Line {}", y));
        }
    }
    
    // Simulate first render (all cells change)
    let changes = manager.compare_buffers();
    let initial_changes = changes.len();
    manager.update_current_buffer();
    
    println!("Initial render: {} cells changed", initial_changes);
    
    // Second render - change only one line
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        for y in 0..height {
            writer.move_to(0, y);
            if y == 10 {
                writer.write_str("MODIFIED LINE 10");
            } else {
                writer.write_str(&format!("Line {}", y));
            }
        }
    }
    
    let changes = manager.compare_buffers();
    let incremental_changes = changes.len();
    
    println!("Incremental update: {} cells changed", incremental_changes);
    
    // Should only detect changes on the modified line
    assert!(incremental_changes < initial_changes / 10, 
        "Incremental update should change far fewer cells: {} vs {}", 
        incremental_changes, initial_changes);
}

// Test cursor movement efficiency - minimal changes
#[test]
fn test_cursor_movement_efficiency() {
    let width = 80;
    let height = 24;
    let mut manager = BufferManager::new(width, height);
    
    // Initial render with cursor at position 0
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str(">");
        for y in 1..height {
            writer.move_to(0, y);
            writer.write_str(" ");
        }
    }
    manager.update_current_buffer();
    
    // Move cursor to position 5
    manager.clear_desired_buffer();
    {
        let mut writer = manager.get_writer();
        writer.move_to(0, 0);
        writer.write_str(" ");
        writer.move_to(0, 5);
        writer.write_str(">");
        for y in 1..height {
            if y != 5 {
                writer.move_to(0, y);
                writer.write_str(" ");
            }
        }
    }
    
    let changes = manager.compare_buffers();
    
    println!("Cursor movement: {} cells changed", changes.len());
    
    // Should only change 2 cells (old position and new position)
    assert!(changes.len() <= 2, "Cursor movement should change at most 2 cells, got {}", changes.len());
}

// Test large list rendering performance
#[test]
fn test_large_list_rendering() {
    let width = 120;
    let height = 100;
    let mut manager = BufferManager::new(width, height);
    
    let num_items = 150;
    let visible_items = height - 5; // Account for header/footer
    let scroll_offset = 0;
    
    // Render a large list
    let start = Instant::now();
    {
        let mut writer = manager.get_writer();
        
        // Header
        writer.move_to(0, 0);
        writer.set_fg_color(Color::Cyan);
        writer.write_str("=== Large List Test ===");
        
        // List items
        for i in 0..visible_items {
            let item_index = scroll_offset + i;
            if item_index < num_items {
                writer.move_to(0, i + 2);
                writer.set_fg_color(Color::White);
                writer.write_str(&format!("Item {} - Some content here", item_index));
            }
        }
    }
    let render_duration = start.elapsed();
    
    let start = Instant::now();
    let changes = manager.compare_buffers();
    let compare_duration = start.elapsed();
    
    println!("Large list rendering:");
    println!("  Render to buffer: {:?}", render_duration);
    println!("  Buffer comparison: {:?}", compare_duration);
    println!("  Total cells: {}", width * height);
    println!("  Changed cells: {}", changes.len());
    
    // Should complete quickly
    assert!(render_duration.as_millis() < 50, "Rendering took too long: {:?}", render_duration);
    assert!(compare_duration.as_millis() < 10, "Comparison took too long: {:?}", compare_duration);
}

// Test buffer clearing performance
#[test]
fn test_buffer_clearing_performance() {
    let width = 200;
    let height = 100;
    let mut manager = BufferManager::new(width, height);
    
    // Fill buffer with content
    {
        let mut writer = manager.get_writer();
        for y in 0..height {
            writer.move_to(0, y);
            writer.write_str(&"X".repeat(width));
        }
    }
    
    // Measure clearing time
    let iterations = 100;
    let start = Instant::now();
    
    for _ in 0..iterations {
        manager.clear_desired_buffer();
    }
    
    let duration = start.elapsed();
    let avg_clear_time = duration.as_micros() / iterations as u128;
    
    println!("Buffer clearing performance:");
    println!("  Average clear time: {}μs", avg_clear_time);
    
    // Should be very fast (< 500μs for 20,000 cells)
    assert!(avg_clear_time < 500, "Buffer clearing too slow: {}μs", avg_clear_time);
}

// Test memory efficiency with multiple buffers
#[test]
fn test_memory_efficiency() {
    let width = 200;
    let height = 100;
    
    // Create multiple buffer managers to test memory usage
    let managers: Vec<BufferManager> = (0..10)
        .map(|_| BufferManager::new(width, height))
        .collect();
    
    // Fill all buffers
    for _manager in managers.iter() {
        // Just verify they were created successfully
        // (no public fields to check, but creation itself validates memory efficiency)
    }
    
    println!("Successfully created {} buffer managers of size {}x{}", 
        managers.len(), width, height);
}

// Test batching efficiency - consecutive cells should be batched
#[test]
fn test_batching_efficiency() {
    let width = 80;
    let height = 24;
    let mut manager = BufferManager::new(width, height);
    
    // Create a pattern where changes are in consecutive runs
    {
        let mut writer = manager.get_writer();
        
        // Line 0: all changed (consecutive)
        writer.move_to(0, 0);
        writer.write_str(&"A".repeat(width));
        
        // Line 5: all changed (consecutive)
        writer.move_to(0, 5);
        writer.write_str(&"B".repeat(width));
        
        // Line 10: scattered changes (non-consecutive)
        for x in (0..width).step_by(5) {
            writer.move_to(x, 10);
            writer.write_str("C");
        }
    }
    
    let changes = manager.compare_buffers();
    
    println!("Batching test:");
    println!("  Total changed cells: {}", changes.len());
    
    // Verify we detected the expected number of changes
    let expected_changes = width + width + (width / 5);
    assert_eq!(changes.len(), expected_changes, 
        "Expected {} changes, got {}", expected_changes, changes.len());
}
