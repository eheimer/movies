# Design Document

## Overview

This design introduces a context-sensitive menu system that replaces the current always-visible second header line with a collapsible menu accessible via F1. The menu will display available actions for the currently selected item and can be navigated using arrow keys or activated via hotkeys. The design emphasizes modularity to make it easy to add new menu items in the future.

## Architecture

### High-Level Changes

1. **New Mode**: Add `Mode::Menu` to represent when the context menu is active
2. **Menu System Module**: Create a new module `menu.rs` to encapsulate menu item definitions and logic
3. **Header Rendering**: Modify `display.rs` to show `[F1] Menu` on the second line instead of context-sensitive options
4. **Menu Rendering**: Add menu rendering logic to `display.rs` with double-line border similar to series selection
5. **Event Handling**: Add `handle_menu_mode` function in `handlers.rs` for menu navigation and activation

### State Management

The main loop will track additional state:

- `menu_mode_active: bool` - Whether the menu is currently displayed
- `menu_selection: usize` - Currently selected menu item index
- `remembered_item: usize` - The item that was selected when menu was opened

## Components and Interfaces

### 1. Menu Module (`src/menu.rs`)

This new module will define the menu system structure and logic.

#### MenuItem Structure

```rust
pub struct MenuItem {
    pub label: String,
    pub hotkey: Option<KeyCode>,
    pub action: MenuAction,
    pub location: MenuLocation,
}

pub enum MenuLocation {
    FirstLine,    // Always visible in first header line
    ContextMenu,  // Only visible in F1 context menu
}

pub enum MenuAction {
    Edit,
    ToggleWatched,
    AssignToSeries,
    RepeatAction,
    Rescan,
}
```

#### Menu Context

```rust
pub struct MenuContext {
    pub selected_entry: Option<Entry>,
    pub episode_detail: EpisodeDetail,
    pub last_action: Option<LastAction>,
    pub view_context: ViewContext,
}
```

#### Core Functions

```rust
// Get all menu items available for the current context
pub fn get_available_menu_items(context: &MenuContext) -> Vec<MenuItem>

// Get only first-line menu items
pub fn get_first_line_items(context: &MenuContext) -> Vec<MenuItem>

// Get only context menu items
pub fn get_context_menu_items(context: &MenuContext) -> Vec<MenuItem>

// Check if a menu item should be available based on context
fn is_item_available(item: &MenuItem, context: &MenuContext) -> bool
```

### 2. Mode Enum Update (`src/util.rs`)

Add new mode variant:

```rust
pub enum Mode {
    Browse,
    Edit,
    Entry,
    SeriesSelect,
    SeriesCreate,
    Menu,  // NEW: Context menu is active
}
```

### 3. Display Module Updates (`src/display.rs`)

#### Header Changes

Modify `draw_header` function:

- First line remains unchanged
- Second line always shows `[F1] Menu` (except in Edit/Entry/SeriesSelect/SeriesCreate modes)
- When in Menu mode, ESC text changes to `[ESC] close menu`

#### Menu Rendering

Add new function:

```rust
fn draw_context_menu(
    menu_items: &[MenuItem],
    selected_index: usize,
    config: &Config,
) -> io::Result<()>
```

This function will:

- Calculate menu position (centered on screen, below header)
- Draw double-line border using `╔═╗║╚╝` characters (similar to `draw_window` with `thick=true`)
- Display each menu item with its hotkey label (e.g., `[F2] edit`)
- Highlight the selected item using the current selection colors from config
- Size the menu dynamically based on number of items

### 4. Handler Module Updates (`src/handlers.rs`)

#### New Handler Function

```rust
pub fn handle_menu_mode(
    code: KeyCode,
    menu_items: &[MenuItem],
    menu_selection: &mut usize,
    mode: &mut Mode,
    redraw: &mut bool,
    // ... other state parameters needed to execute actions
) -> io::Result<()>
```

This handler will:

- Handle Up/Down arrow keys to navigate menu items (with wrapping)
- Handle Enter to execute the selected menu item's action
- Handle ESC to close menu and return to Browse mode
- Handle hotkey presses to directly execute menu items
- Execute the appropriate action based on MenuAction enum

#### Browse Mode Handler Update

Modify `handle_browse_mode` to:

- Handle F1 key press to enter Menu mode
- Remove F2, F3, F4, F5 handling (these will be handled in Menu mode)
- Keep CTRL+L for rescan (this stays on first line)

### 5. Main Loop Updates (`src/main.rs`)

Add state variables:

```rust
let mut menu_selection: usize = 0;
let mut remembered_item: usize = 0;
```

Add Menu mode case to event handling:

```rust
Mode::Menu => {
    let menu_context = MenuContext {
        selected_entry: filtered_entries.get(remembered_item).cloned(),
        episode_detail: edit_details.clone(),
        last_action: last_action.clone(),
        view_context: view_context.clone(),
    };
    let menu_items = menu::get_context_menu_items(&menu_context);

    handlers::handle_menu_mode(
        code,
        &menu_items,
        &mut menu_selection,
        &mut mode,
        &mut redraw,
        // ... pass other necessary state
    )?;
}
```

## Data Models

### MenuItem Definition

Menu items will be defined statically in the `menu.rs` module:

```rust
fn define_all_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem {
            label: "edit".to_string(),
            hotkey: Some(KeyCode::F(2)),
            action: MenuAction::Edit,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "toggle watched".to_string(),
            hotkey: Some(KeyCode::F(3)),
            action: MenuAction::ToggleWatched,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "assign to series".to_string(),
            hotkey: Some(KeyCode::F(4)),
            action: MenuAction::AssignToSeries,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "Repeat action".to_string(),
            hotkey: Some(KeyCode::F(5)),
            action: MenuAction::RepeatAction,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "rescan".to_string(),
            hotkey: Some(KeyCode::Char('l')),  // CTRL+L handled separately
            action: MenuAction::Rescan,
            location: MenuLocation::FirstLine,
        },
    ]
}
```

### Context-Based Filtering

The `get_available_menu_items` function will filter based on:

- **Edit**: Available only when selected entry is an Episode
- **Toggle Watched**: Available only when selected entry is an Episode
- **Assign to Series**: Available only when selected entry is an Episode without a series
- **Repeat Action**: Available only when `can_repeat_action` returns true
- **Rescan**: Always available

## Error Handling

- Menu rendering errors will be propagated up through `io::Result`
- Invalid menu selections will be clamped to valid range (0 to menu_items.len()-1)
- If no menu items are available, F1 will be a no-op (though this should never happen in practice)
- Menu action execution will use existing error handling patterns from current handlers

## Testing Strategy

### Manual Testing Scenarios

1. **Menu Display**

   - Verify F1 opens menu in Browse mode
   - Verify menu shows correct items based on selected entry type
   - Verify menu has double-line border
   - Verify ESC text changes when menu is open

2. **Menu Navigation**

   - Test up/down arrow navigation
   - Test wrapping at top and bottom of menu
   - Verify selection cursor highlights correctly

3. **Menu Actions**

   - Test each menu item via Enter key
   - Test each menu item via hotkey
   - Verify actions execute correctly and menu closes
   - Verify ESC closes menu without action

4. **Context Sensitivity**

   - Test with Episode without series (should show "assign to series")
   - Test with Episode with series (should not show "assign to series")
   - Test with Series selected (menu should have limited options)
   - Test with last action available (should show "Repeat action")

5. **Integration**
   - Verify first line header remains unchanged
   - Verify second line shows `[F1] Menu` in Browse mode
   - Verify existing functionality (play, navigate, filter) still works

### Edge Cases

- Empty filtered entries list
- Menu with single item
- Menu with many items (ensure it fits on screen)
- Rapid F1 presses
- Hotkey conflicts (ensure proper handling)

## Implementation Notes

### Modularity Goals

The design achieves modularity through:

1. **Centralized Definition**: All menu items defined in one place (`menu.rs`)
2. **Declarative Structure**: MenuItem struct clearly defines all properties
3. **Context-Based Logic**: Availability logic separated from rendering and handling
4. **Easy Extension**: Adding a new menu item requires:
   - Add new MenuAction variant
   - Add new MenuItem to `define_all_menu_items()`
   - Add availability logic to `is_item_available()`
   - Add action execution to `handle_menu_mode()`

### Rendering Consistency

The menu will use the same visual style as the series selection window:

- Double-line borders (`╔═╗║╚╝`)
- Same selection highlighting (using config colors)
- Similar positioning logic (centered on screen)

### State Transitions

```
Browse Mode + F1 → Menu Mode
Menu Mode + ESC → Browse Mode (restore focus to remembered item)
Menu Mode + Enter/Hotkey → Execute action → Browse Mode
```

### Performance Considerations

- Menu items are filtered on each redraw (minimal overhead, small list)
- Menu rendering only occurs when Mode::Menu is active
- No additional database queries required for menu display
