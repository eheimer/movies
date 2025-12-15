# Header Component Design Document

## Overview

The header component system will replace the monolithic `draw_header()` function in `display.rs` with a modular, composable architecture. The system consists of a main Header component that coordinates four specialized sub-components, each responsible for rendering a specific section of the header area.

## Architecture

### Component Hierarchy

```
Header (Container Component)
├── HotkeyHelper (Row 0)
├── LastActionLine (Row 1) 
├── FilterLine (Row 2)
└── Breadcrumbs (Row 3)
```

### Design Principles

1. **Stateless Components**: All sub-components receive data through constructor parameters
2. **Conditional Rendering**: Sub-components return empty Vec when they have no content
3. **Dynamic Height**: Header calculates total height based on active sub-components
4. **Cell-based Rendering**: Follows existing component patterns using Cell structures
5. **Theme Awareness**: All components respect the current theme configuration

## Components and Interfaces

### Header Component

The main container component that manages layout and coordinates sub-components.

```rust
pub struct Header {
    pub hotkey_helper: HotkeyHelper,
    pub last_action_line: LastActionLine,
    pub breadcrumbs: Breadcrumbs,
    pub filter_line: FilterLine,
}

impl Header {
    pub fn new(
        mode: &Mode,
        filter_mode: bool,
        is_dirty: bool,
        is_first_run: bool,
        terminal_width: usize,
        selected_entry: Option<&Entry>,
        edit_details: &EpisodeDetail,
        last_action: &Option<LastAction>,
        view_context: &ViewContext,
        filter_text: &str,
        filter_focused: bool,
    ) -> Self;
    
    pub fn calculate_height(&self) -> usize;
}
```

### HotkeyHelper Component

Renders the first line with menu hotkeys and context helpers.

```rust
pub struct HotkeyHelper {
    pub mode: Mode,
    pub filter_mode: bool,
    pub is_dirty: bool,
    pub is_first_run: bool,
    pub terminal_width: usize,
    pub selected_entry: Option<Entry>,
    pub edit_details: EpisodeDetail,
    pub last_action: Option<LastAction>,
}

impl HotkeyHelper {
    pub fn new(/* parameters */) -> Self;
    
    fn build_hardcoded_helpers(&self) -> String;
    fn add_first_line_preferred_items(&self, base_text: &str) -> String;
}
```

### LastActionLine Component

Displays repeatable actions with hotkey reminders.

```rust
pub struct LastActionLine {
    pub last_action: Option<LastAction>,
}

impl LastActionLine {
    pub fn new(last_action: Option<LastAction>) -> Self;
    
    fn format_last_action(&self) -> String;
}
```

### Breadcrumbs Component

Shows navigation context indicating current location in the library hierarchy.

```rust
pub struct Breadcrumbs {
    pub view_context: ViewContext,
}

impl Breadcrumbs {
    pub fn new(view_context: ViewContext) -> Self;
    
    fn format_breadcrumb(&self) -> String;
}
```

### FilterLine Component

Displays filter input with highlighting when active.

```rust
pub struct FilterLine {
    pub filter_text: String,
    pub filter_focused: bool,
    pub filter_active: bool,
}

impl FilterLine {
    pub fn new(filter_text: String, filter_focused: bool) -> Self;
    
    fn format_filter_display(&self) -> String;
}
```

## Data Models

### HeaderContext

A context structure to pass all necessary data to the Header component:

```rust
pub struct HeaderContext {
    pub mode: Mode,
    pub filter_mode: bool,
    pub is_dirty: bool,
    pub is_first_run: bool,
    pub terminal_width: usize,
    pub selected_entry: Option<Entry>,
    pub edit_details: EpisodeDetail,
    pub last_action: Option<LastAction>,
    pub view_context: ViewContext,
    pub filter_text: String,
    pub filter_focused: bool,
}
```

## Test Cases

*A test case is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Test cases serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Test Case 1: Header component composition

When a Header component is created, it should contain four sub-components in the correct vertical order.
**Validates: Requirements 1.1**

### Test Case 2: Dynamic height calculation

When sub-components have different visibility states, the Header component should calculate height correctly based only on active sub-components.
**Validates: Requirements 1.2**

### Test Case 3: Primary menu hotkey display

When the HotkeyHelper component renders, it should include "[F1] Menu" as the primary hotkey in the output.
**Validates: Requirements 3.1**

### Test Case 4: Empty sub-component handling

When sub-components have no content to display, they should return an empty Vec and the Header component should skip rendering them.
**Validates: Requirements 7.1, 7.2**

## Error Handling

### Component Error Scenarios

1. **Invalid Width/Height**: Components should handle zero or invalid dimensions gracefully
2. **Missing Data**: Sub-components should handle missing or invalid data without panicking
3. **Theme Errors**: Components should fall back to default colors if theme data is invalid
4. **Overflow Handling**: HotkeyHelper should gracefully handle cases where content exceeds terminal width

### Error Recovery Strategies

- Return empty Vec for invalid rendering scenarios
- Use default values for missing configuration
- Log warnings for recoverable errors
- Maintain application stability even with component failures

## Testing Strategy

### Unit Testing

Unit tests will verify specific examples and edge cases:

- Component construction with various parameter combinations
- Rendering output format validation
- Empty state handling
- Theme application correctness
- Width constraint enforcement

### Integration Testing

Integration tests will verify end-to-end behavior:

- Header component coordination with sub-components
- Integration with existing display system
- Theme system integration
- Menu system integration for hotkey helpers

### Edge Cases

- Zero terminal width scenarios
- Empty data states for all sub-components
- Maximum width overflow scenarios
- Invalid theme configurations
- Missing menu items or actions

The testing approach will use Rust's built-in testing framework with tests organized in the `tests/` directory following the pattern `header_component_tests.rs`.

## Implementation Notes

### Migration Strategy

1. Create new header component system alongside existing code
2. Implement all sub-components with comprehensive tests
3. Update display.rs to use new Header component
4. Remove old draw_header() function after verification
5. Update any dependent code to use new component system

### Performance Considerations

- Sub-components should minimize string allocations during rendering
- Cache formatted strings where appropriate
- Avoid unnecessary recalculations of static content
- Use efficient Cell array operations for rendering

### Future Extensibility

The modular design allows for:
- Adding new header sections as sub-components
- Customizing header layout through configuration
- Supporting different header themes or styles
- Implementing selective redrawing for performance optimization