# Design Document

## Overview

The Context Menu Component is a reusable UI component that extracts the context menu rendering logic from the main display module. It follows the established component architecture pattern, implementing the Component trait to render context menus as 2D Cell arrays. The component maintains all existing functionality while providing better code organization and testability.

## Architecture

The Context Menu Component follows the same architectural patterns as other components in the system:

- **Component Trait Implementation**: Implements the Component trait with a render method that returns Vec<Vec<Cell>>
- **Separation of Concerns**: Handles only rendering logic, not menu item filtering or action execution
- **Theme Integration**: Uses the existing theme system for consistent styling
- **Terminal Abstraction**: Works with Cell arrays rather than direct terminal I/O

## Components and Interfaces

### ContextMenu Struct

```rust
pub struct ContextMenu {
    menu_items: Vec<MenuItem>,
    selected_index: usize,
}
```

**Public Methods:**
- `new(menu_items: Vec<MenuItem>, selected_index: usize) -> Self`
- `render(&self, width: usize, height: usize, theme: &Theme, _is_selected: bool) -> Vec<Vec<Cell>>` (Component trait)

**Private Methods:**
- `calculate_menu_dimensions(&self) -> (usize, usize)`
- `format_hotkey(hotkey: &Option<KeyCode>) -> String`
- `create_menu_item_cells(&self, item: &MenuItem, is_selected: bool, content_width: usize, theme: &Theme) -> Vec<Cell>`

### Integration Points

- **Display Module**: Replaces the existing `draw_context_menu` function with component usage
- **Menu Module**: Continues to use existing MenuItem and MenuAction types
- **Theme System**: Uses existing theme colors for menu styling
- **Terminal System**: Integrates with existing Cell rendering infrastructure

## Data Models

### Input Data

- **MenuItem**: Existing struct from menu module containing label, hotkey, action, and location
- **Theme**: Existing theme configuration for colors and styling
- **Selection State**: Index of currently selected menu item

### Output Data

- **Cell Array**: 2D vector of Cell structs representing the rendered menu
- **Menu Dimensions**: Calculated width and height based on content

## Error Handling

The component follows the established error handling patterns:

- **Graceful Degradation**: Returns empty Cell array for invalid dimensions or empty menu items
- **Overflow Protection**: Uses saturating arithmetic to prevent underflow when calculating dimensions
- **UTF-8 Safety**: Properly handles multi-byte characters in menu labels
- **Boundary Checking**: Validates selection index against menu item count

## Testing Strategy

### Unit Testing

The component will have comprehensive unit tests covering:

1. **Rendering Logic**: Verify correct Cell array generation for various menu configurations
2. **Dimension Calculation**: Test menu sizing with different label and hotkey combinations  
3. **Selection Highlighting**: Ensure selected items are properly styled
4. **Edge Cases**: Empty menus, oversized content, invalid selection indices
5. **Theme Integration**: Verify proper color application from theme configuration

### Integration Testing

Integration tests will verify:

1. **Display Module Integration**: Ensure seamless replacement of existing menu rendering
2. **Menu Item Compatibility**: Verify all existing MenuItem types render correctly
3. **Theme Compatibility**: Test with various theme configurations
4. **Terminal Rendering**: Validate Cell arrays render correctly to terminal output
## 
Test Cases

*A test case is a specific scenario that verifies expected behavior of the system. Test cases serve as concrete examples that validate the system meets its requirements.*

### Test Case 1: UTF-8 character width calculation

When menu items contain multi-byte UTF-8 characters, the component should calculate visual width correctly for proper alignment.
**Validates: Requirements 1.5**

### Test Case 2: Cell array structure

When the render method is called, it should return a properly structured 2D Cell array with correct dimensions.
**Validates: Requirements 2.4**

### Test Case 3: Border character rendering

When rendering the menu window, all border cells should contain the correct double-line Unicode characters (╔, ╗, ╚, ╝, ═, ║).
**Validates: Requirements 3.4**

### Test Case 4: Label and hotkey alignment

When menu items are rendered, labels should appear at the left side of each row and hotkeys should appear at the right side with proper spacing.
**Validates: Requirements 3.2**

### Test Case 5: Selection highlighting

When a menu item is selected, that item's cells should use the theme's current foreground and background colors while unselected items use default colors.
**Validates: Requirements 3.3**

### Test Case 6: Dimension calculation

When calculating menu dimensions, the width should accommodate the longest label plus spacing plus the longest hotkey, and height should equal the number of items plus border rows.
**Validates: Requirements 3.5**

### Test Case 7: Menu item preservation

When menu items are provided to the component, all items should be rendered without filtering or modification.
**Validates: Requirements 4.1**

### Test Case 8: Hotkey formatting

When formatting hotkeys, F-keys should display as [F2], [F3], etc. and character keys should display as [S], [E], etc.
**Validates: Requirements 4.2**

### Test Case 9: Arithmetic safety

When terminal width is smaller than calculated menu width, the component should use saturating arithmetic to prevent underflow and handle the situation gracefully.
**Validates: Requirements 4.5**