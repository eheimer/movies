# Implementation Plan: Double-Buffer Rendering

## Overview

This implementation adds a double-buffer rendering layer to improve visual performance by reducing unnecessary screen redraws. The approach is incremental and maintains compatibility with existing code.

## Tasks

- [x] 1. Create buffer module with core data structures
  - Create src/buffer.rs module
  - Implement Cell struct with character, colors, and style attributes
  - Implement Cell::empty() for blank cells
  - Implement Cell equality comparison (PartialEq)
  - _Requirements: 3.1, 4.2_

- [x] 1.1 Write unit tests for Cell
  - Test Cell::empty() creates blank cell
  - Test Cell equality with different attributes
  - Test Cell with various color combinations
  - _Requirements: 3.1, 4.2_

- [x] 2. Implement ScreenBuffer
  - Create ScreenBuffer struct with 2D cell array
  - Implement ScreenBuffer::new() to create buffer with dimensions
  - Implement clear() to fill buffer with empty cells
  - Implement set_cell() and get_cell() with bounds checking
  - Implement differs_at() to compare cells between buffers
  - _Requirements: 3.1, 3.2, 4.1_

- [x] 2.1 Write unit tests for ScreenBuffer
  - Test buffer creation with various dimensions
  - Test clear() fills buffer with empty cells
  - Test set_cell() and get_cell() with valid positions
  - Test bounds checking (out-of-bounds writes are safe)
  - Test differs_at() correctly identifies changed cells
  - _Requirements: 3.1, 3.2, 4.1_

- [x] 3. Implement BufferWriter
  - Create BufferWriter struct that wraps mutable ScreenBuffer reference
  - Implement write_char() to write single character at current position
  - Implement write_str() to write string at current position
  - Implement move_to() to change cursor position
  - Implement set_fg_color(), set_bg_color(), set_style() for styling
  - _Requirements: 2.1, 2.2, 3.2_

- [x] 3.1 Write unit tests for BufferWriter
  - Test write_char() updates buffer at correct position
  - Test write_str() writes multiple characters
  - Test move_to() changes write position
  - Test color and style changes affect subsequent writes
  - _Requirements: 2.1, 2.2, 3.2_

- [x] 4. Implement BufferManager core functionality
  - Create BufferManager struct with current and desired buffers
  - Implement new() to create manager with terminal dimensions
  - Implement clear_desired_buffer() to reset desired buffer to empty
  - Implement get_writer() to return BufferWriter for desired buffer
  - Implement resize() to recreate buffers with new dimensions
  - _Requirements: 2.1, 3.1, 6.1_

- [x] 4.1 Write unit tests for BufferManager initialization
  - Test new() creates buffers with correct dimensions
  - Test clear_desired_buffer() fills desired buffer with empty cells
  - Test get_writer() returns valid BufferWriter
  - Test resize() updates buffer dimensions
  - _Requirements: 2.1, 3.1, 6.1_

- [x] 5. Implement buffer comparison and differential update
  - Implement compare_buffers() to identify changed cells
  - Collect list of (x, y, cell) tuples for changed positions
  - Optimize comparison to skip unchanged regions
  - Handle terminal resize as special case (all cells changed)
  - _Requirements: 4.1, 4.2, 4.3, 4.4_

- [x] 5.1 Write unit tests for buffer comparison
  - Test comparison identifies changed cells correctly
  - Test comparison skips unchanged cells
  - Test comparison handles empty buffers
  - Test comparison with various change patterns
  - _Requirements: 4.1, 4.2, 4.3_

- [x] 6. Implement terminal write optimization
  - Implement render_to_terminal() to write changed cells
  - Batch consecutive cells on same row for efficiency
  - Use cursor positioning to minimize escape sequences
  - Update current buffer after successful writes
  - Handle terminal I/O errors gracefully
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [x] 6.1 Write integration tests for terminal rendering
  - Test render_to_terminal() writes only changed cells
  - Test batching of consecutive cells
  - Test current buffer updates after render
  - Test error handling preserves buffer state
  - _Requirements: 5.1, 5.3, 5.4, 5.5_

- [x] 7. Add force_full_redraw() for mode changes
  - Implement force_full_redraw() to mark all cells as changed
  - Clear current buffer to force comparison mismatch
  - Ensure clean state when switching modes
  - _Requirements: 6.2_

- [x] 7.1 Write unit tests for force_full_redraw
  - Test force_full_redraw() causes all cells to be redrawn
  - Test mode change scenarios trigger full redraw
  - _Requirements: 6.2_

- [x] 8. Checkpoint - Ensure all buffer module tests pass
  - Run all buffer module tests
  - Verify no regressions
  - Ask user if questions arise

- [x] 9. Integrate BufferManager into main.rs
  - Add BufferManager initialization in main_loop
  - Pass buffer_manager to draw_screen function
  - Handle terminal resize events by calling buffer_manager.resize()
  - Handle mode changes by calling force_full_redraw()
  - _Requirements: 2.1, 6.1, 6.2_

- [x] 9.1 Write integration tests for main.rs changes
  - Test BufferManager is created with correct terminal size
  - Test resize events update buffer dimensions
  - Test mode changes trigger full redraw
  - _Requirements: 2.1, 6.1, 6.2_

- [x] 10. Update draw_screen to use BufferWriter
  - Add buffer_manager parameter to draw_screen signature
  - Call clear_desired_buffer() at start of function
  - Get BufferWriter from buffer_manager
  - Replace direct terminal writes with BufferWriter calls
  - Call render_to_terminal() at end of function
  - _Requirements: 2.1, 2.2, 2.3, 3.1_

- [x] 10.1 Test draw_screen with buffer integration
  - Test draw_screen writes to buffer instead of terminal
  - Test desired buffer starts empty each frame
  - Test render_to_terminal is called at end
  - _Requirements: 2.1, 2.2, 3.1_

- [x] 11. Replace terminal I/O calls with BufferWriter
  - Replace print_at() calls with writer.move_to() + writer.write_str()
  - Replace clear_screen() with buffer_manager.clear_desired_buffer()
  - Replace move_cursor() with writer.move_to()
  - Keep show_cursor() and hide_cursor() as direct terminal calls
  - _Requirements: 2.2, 2.3, 2.4_

- [x] 11.1 Test terminal I/O replacement
  - Test all rendering produces same visual output
  - Test no direct terminal writes remain (except cursor visibility)
  - Test buffer writes are correctly translated to terminal
  - _Requirements: 2.2, 2.3, 2.4_

- [x] 12. Handle special cases (cursor, status line)
  - Ensure cursor positioning works correctly in Edit mode
  - Ensure status line updates work correctly
  - Ensure filter mode cursor positioning works
  - Test cursor visibility (show/hide) still works
  - _Requirements: 6.4, 6.5_

- [x] 12.1 Test special case handling
  - Test cursor positioning in Edit mode
  - Test status line updates
  - Test filter mode cursor
  - Test cursor visibility
  - _Requirements: 6.4, 6.5_

- [x] 13. Performance testing and optimization
  - Measure rendering performance before and after
  - Verify reduced terminal I/O operations
  - Test with large lists (100+ items)
  - Test with rapid navigation (arrow key spam)
  - Optimize buffer comparison if needed
  - _Requirements: 1.1, 1.2, 4.3, 5.1_

- [x] 13.1 Document performance improvements
  - Record terminal I/O reduction metrics
  - Document any performance bottlenecks found
  - Document optimization decisions
  - _Requirements: 1.1, 1.2_

- [x] 14. Final integration testing
  - Test all UI modes (Browse, Edit, Entry, SeriesSelect, SeriesCreate, Menu)
  - Test navigation (arrow keys, page up/down)
  - Test editing (text input, cursor movement)
  - Test terminal resize in all modes
  - Test mode switching
  - Verify no visual artifacts or flicker
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 6.1, 6.2, 6.3_

- [x] 14.1 User acceptance testing
  - Verify smooth visual updates
  - Verify no flicker during navigation
  - Verify no visual artifacts
  - Verify all existing functionality works
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5_

- [x] 15. Final Checkpoint - Ensure all tests pass
  - Run full test suite
  - Verify no regressions
  - Ask user if questions arise

- [ ] 16. Documentation and cleanup
  - Add module-level documentation to src/buffer.rs
  - Document BufferManager API
  - Document integration points in display.rs
  - Remove any debug logging added during development
  - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

## Notes

- All existing function signatures remain unchanged except draw_screen (adds one parameter)
- No changes to component rendering logic
- No changes to state management
- Buffer layer is completely independent and testable
- Each task builds incrementally on previous tasks
- Checkpoints ensure stability before proceeding
