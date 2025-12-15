# Design Document

## Overview

The DetailPanel component system extracts episode detail rendering logic from the monolithic `draw_detail_window()` function in `display.rs` into a composable, reusable component architecture. This refactoring creates three main components:

1. **DetailPanel**: A container component that switches between sub-components based on application mode
2. **MetadataDisplay**: A read-only component for displaying episode details with proper field styling
3. **EpisodeEditor**: An interactive component with cursor positioning, field highlighting, and dirty state indicators

The design follows the existing component architecture patterns established by the Header, Browser, and other components in the system, using the `Component` trait and `Cell`-based rendering.

## Architecture

### Component Hierarchy

```
DetailPanel (Container)
├── MetadataDisplay (Read-only view)
└── EpisodeEditor (Interactive editing view)
```

### Integration Points

- **display.rs**: The `draw_detail_window()` function will be replaced with DetailPanel component usage
- **EpisodeField**: All components will use the existing EpisodeField enum for field definitions and metadata
- **Theme**: Components will accept theme parameters for consistent styling with the rest of the application
- **Component trait**: All components implement the standard Component trait for consistent rendering

### State Management

The DetailPanel receives all necessary state as parameters:
- Current mode (Browse/Edit)
- Episode details (EpisodeDetail struct)
- Edit state (current field, cursor position, dirty fields)
- Layout parameters (position, dimensions)

## Components and Interfaces

### DetailPanel

The main container component that orchestrates sub-component rendering based on application mode.

```rust
pub struct DetailPanel {
    mode: Mode,
    episode_details: EpisodeDetail,
    edit_field: EpisodeField,
    edit_cursor_pos: usize,
    season_number: Option<usize>,
    dirty_fields: HashSet<EpisodeField>,
    position: (usize, usize), // (col, row)
    dimensions: (usize, usize), // (width, height)
}

impl DetailPanel {
    pub fn new(
        mode: Mode,
        episode_details: EpisodeDetail,
        edit_field: EpisodeField,
        edit_cursor_pos: usize,
        season_number: Option<usize>,
        dirty_fields: HashSet<EpisodeField>,
        position: (usize, usize),
        dimensions: (usize, usize),
    ) -> Self;
}
```

### MetadataDisplay

A read-only component for displaying episode metadata with consistent field layout and styling.

```rust
pub struct MetadataDisplay {
    episode_details: EpisodeDetail,
    season_number: Option<usize>,
    entry_location: String, // For path/filename extraction
}

impl MetadataDisplay {
    pub fn new(
        episode_details: EpisodeDetail,
        season_number: Option<usize>,
        entry_location: String,
    ) -> Self;
    
    fn format_field_line(&self, field: EpisodeField) -> String;
    fn extract_path_and_filename(&self) -> (String, String);
}
```

### EpisodeEditor

An interactive component for editing episode details with visual feedback for dirty fields and cursor positioning.

```rust
pub struct EpisodeEditor {
    episode_details: EpisodeDetail,
    edit_field: EpisodeField,
    edit_cursor_pos: usize,
    season_number: Option<usize>,
    dirty_fields: HashSet<EpisodeField>,
    entry_location: String, // For path/filename extraction
}

impl EpisodeEditor {
    pub fn new(
        episode_details: EpisodeDetail,
        edit_field: EpisodeField,
        edit_cursor_pos: usize,
        season_number: Option<usize>,
        dirty_fields: HashSet<EpisodeField>,
        entry_location: String,
    ) -> Self;
    
    fn format_field_line_with_highlighting(&self, field: EpisodeField, theme: &Theme) -> Vec<Cell>;
    fn is_field_dirty(&self, field: EpisodeField) -> bool;
    fn calculate_cursor_position(&self, field: EpisodeField) -> (usize, usize);
}
```

## Data Models

### Field Rendering Data

```rust
struct FieldRenderData {
    field: EpisodeField,
    display_name: String,
    value: String,
    is_editable: bool,
    is_dirty: bool,
}
```

### Component State

The components maintain minimal state and rely on parameters passed during construction. This ensures components remain stateless and can be easily tested and composed.



## Error Handling

### Input Validation

- **Invalid EpisodeField values**: Components will handle invalid field indices gracefully by using EpisodeField::from() with bounds checking
- **Missing episode data**: Components will render empty or default values for missing episode detail fields
- **Invalid cursor positions**: EpisodeEditor will clamp cursor positions to valid field boundaries

### Rendering Errors

- **Insufficient terminal space**: Components will truncate content appropriately when window dimensions are too small
- **Theme color failures**: Components will fall back to default colors if theme color parsing fails
- **Cell rendering errors**: Components will handle Cell creation errors gracefully and continue rendering

### State Consistency

- **Dirty field mismatches**: Components will validate dirty field sets against actual EpisodeField values
- **Mode/component mismatches**: DetailPanel will handle unexpected mode values by defaulting to Browse mode

## Testing Strategy

The testing approach will focus on essential functionality:

- **Component Creation**: Verify components can be created with required parameters
- **Basic Rendering**: Ensure components produce expected output for typical use cases
- **Mode Switching**: Verify DetailPanel correctly switches between sub-components

Testing will use Rust's built-in testing capabilities with the existing test structure in the `tests/` directory.