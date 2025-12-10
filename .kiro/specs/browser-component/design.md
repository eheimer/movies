# Browser Component Design Document

## Overview

The Browser Component serves as the main display element for the episode browser application, integrating category components, episode components, and an optional scrollbar into a unified browsing interface. This component follows the established Component trait pattern and manages the layout, selection state, and scrolling behavior for collections of video content within the terminal-based interface.

The component builds upon the existing component architecture, utilizing the Category, Episode, and Scrollbar components while providing higher-level coordination and layout management.

## Architecture

The Browser Component follows a composite pattern, containing and coordinating multiple child components:

```
Browser Component
├── Categories Collection (Category components)
├── Episodes Collection (Episode components)  
└── Optional Scrollbar (Scrollbar component)
```

### Component Hierarchy

- **Browser Component**: Main container implementing the Component trait
- **Child Components**: Category, Episode, and Scrollbar components managed by the browser
- **Layout Manager**: Internal logic for positioning and sizing child components
- **Selection Manager**: Tracks and updates selection state across all items
- **Scroll Manager**: Handles viewport calculations and scrollbar visibility

## Components and Interfaces

### Browser Component Structure

```rust
pub struct Browser {
    pub top_left: (usize, usize),     // (column, row) position
    pub width: usize,                  // Total width available
    pub height: usize,                 // Total height available
    pub categories: Vec<Category>,     // Category components to display
    pub episodes: Vec<Episode>,        // Episode components to display
    pub selected_item: usize,          // Index of currently selected item
    pub first_visible_item: usize,     // Index of first visible item
}
```

### Component Trait Implementation

The Browser Component implements the existing Component trait:

```rust
impl Component for Browser {
    fn render(&self, width: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>>;
}
```

### Integration Points

- **Category Component**: Renders series and season entries using existing CategoryType enum
- **Episode Component**: Renders individual episodes with watched status and styling
- **Scrollbar Component**: Displays when content exceeds available height
- **Theme System**: Uses existing theme colors and styling for consistent appearance

## Data Models

### Browser State

```rust
pub struct Browser {
    // Position and dimensions
    pub top_left: (usize, usize),
    pub width: usize,
    pub height: usize,
    
    // Content collections
    pub categories: Vec<Category>,
    pub episodes: Vec<Episode>,
    
    // Navigation state
    pub selected_item: usize,
    pub first_visible_item: usize,
}
```

### Item Indexing

Items are indexed sequentially across both collections:
- Categories: indices 0 to categories.len() - 1
- Episodes: indices categories.len() to categories.len() + episodes.len() - 1

### Viewport Calculations

```rust
struct ViewportInfo {
    total_items: usize,           // categories.len() + episodes.len()
    visible_items: usize,         // items that fit in height
    needs_scrollbar: bool,        // total_items > visible_items
    scrollbar_width: usize,       // 1 if scrollbar needed, 0 otherwise
    content_width: usize,         // width - scrollbar_width
}
```

## Test Cases

*A test case is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Test cases serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Test Case 1: Content fits without scrollbar

When total items fit within available height, the browser should render all items without displaying a scrollbar.
**Validates: Requirements 1.2**

### Test Case 2: Content exceeds height triggers scrollbar

When total items exceed available height, the browser should display a scrollbar and adjust content width accordingly.
**Validates: Requirements 1.3**

### Test Case 3: Selection highlighting

When an item is selected, the browser should render that specific item with selection highlighting while other items use normal styling.
**Validates: Requirements 2.2**

### Test Case 4: Viewport scrolling

When the selected item is outside the current viewport, the browser should adjust the first visible item to bring the selection into view.
**Validates: Requirements 2.3**

### Test Case 5: Component positioning

When rendering child components, the browser should position each component relative to its top-left coordinates and within its width/height boundaries.
**Validates: Requirements 1.1, 1.4**

### Test Case 6: Empty state handling

When no categories or episodes are available, the browser should render an empty viewport without errors or selection indicators.
**Validates: Requirements 2.5**

### Test Case 7: Scroll bounds enforcement

When scrolling, the browser should prevent the first visible item from going below zero or above the maximum valid scroll position.
**Validates: Requirements 3.3, 3.4**

### Test Case 8: Child component integration

When rendering, the browser should utilize existing Category, Episode, and Scrollbar components without modifying their internal behavior.
**Validates: Requirements 4.1, 4.2, 4.3**

## Error Handling

### Invalid Dimensions
- Zero width or height: Return empty cell array
- Negative coordinates: Clamp to zero

### Invalid Selection
- Selected item out of bounds: Clamp to valid range [0, total_items)
- No items available: Set selected_item to 0

### Scroll Position Validation
- First visible item out of bounds: Clamp to valid scroll range
- Invalid viewport calculations: Fall back to showing from item 0

### Component Integration Errors
- Empty component collections: Handle gracefully with empty rendering
- Component rendering failures: Skip failed components, continue with others

## Testing Strategy

### Unit Testing
- Test viewport calculations with various content sizes
- Test selection bounds checking and clamping
- Test scrollbar visibility logic
- Test component positioning calculations
- Test empty state handling

### Integration Testing  
- Test rendering with real Category and Episode components
- Test theme integration and color application
- Test scrollbar integration and positioning
- Test selection highlighting across component boundaries
- Test scroll behavior with mixed content types

### Edge Cases
- Zero dimensions and empty content
- Single item collections
- Maximum size collections that exceed typical terminal dimensions
- Rapid selection changes and scroll updates
