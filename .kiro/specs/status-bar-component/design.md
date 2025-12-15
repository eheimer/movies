# Status Bar Component Design Document

## Overview

This design document outlines the extraction of status bar rendering logic from the main display module into a dedicated StatusBar component. The component will follow the established component architecture pattern, implementing the Component trait and returning Cell arrays for consistent rendering behavior.

The StatusBar component will encapsulate all status bar rendering logic, including message formatting, text truncation, padding, and theme application. This refactoring improves code organization and maintainability while preserving the exact visual appearance and behavior of the current implementation.

## Architecture

The StatusBar component will be implemented as a standalone component in the `src/components/` directory, following the same patterns established by other components like Scrollbar and Header. The component will:

1. Accept message text and theme configuration as constructor parameters
2. Implement the Component trait with a render method that returns Cell arrays
3. Handle all text processing internally (truncation, padding, UTF-8 character counting)
4. Apply theme colors for foreground and background styling

## Components and Interfaces

### StatusBar Component

The main StatusBar component will be a simple struct containing:

```rust
pub struct StatusBar {
    message: String,
}
```

### Component Trait Implementation

The StatusBar will implement the Component trait:

```rust
impl Component for StatusBar {
    fn render(&self, width: usize, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>>
}
```

### Integration Points

The StatusBar component will integrate with the existing system through:

1. **Display Module**: The `draw_screen` function will create and use a StatusBar component instead of calling `draw_status_line`
2. **Theme System**: The component will use theme.status_fg and theme.status_bg for styling
3. **Terminal Interface**: The component will return Cell arrays that get rendered using the existing cell rendering infrastructure

## Data Models

### Input Data

- **message**: String containing the status message to display
- **theme**: Reference to Theme object containing status bar colors
- **width**: Terminal width for proper text formatting
- **height**: Expected to be 1 for status bar (single row)

### Output Data

- **Cell Array**: 2D vector of Cell objects representing the formatted status bar
- Each Cell contains character, foreground color, background color, and text style

## Testing Strategy

### Unit Testing

The StatusBar component will have comprehensive unit tests covering:

1. **Message Formatting**: Test that messages are properly formatted and padded
2. **Text Truncation**: Verify that long messages are truncated to fit terminal width
3. **UTF-8 Handling**: Ensure multi-byte characters are counted correctly for visual width
4. **Theme Application**: Confirm that theme colors are applied correctly to all cells
5. **Edge Cases**: Test empty messages, zero width, and boundary conditions

### Integration Testing

Integration tests will verify:

1. **Display Integration**: Ensure the StatusBar component integrates properly with the display module
2. **Visual Consistency**: Confirm that the refactored implementation produces identical output to the original
3. **Theme Integration**: Verify that theme changes are reflected in the status bar appearance

## Error Handling

The StatusBar component will handle edge cases gracefully:

1. **Empty Messages**: Display empty padded line when message is empty
2. **Zero Width**: Return empty Cell array when width is zero
3. **Oversized Messages**: Truncate messages that exceed terminal width based on visual character count
4. **Invalid Theme Colors**: Fall back to default colors if theme colors are invalid

The component will not perform any I/O operations or error logging, maintaining the separation of concerns established by the component architecture.