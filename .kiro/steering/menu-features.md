# Menu Feature Implementation Guide

This document describes the established patterns for adding new menu features to the application.

## Overview

The application uses a centralized menu system with:
- **First-line items**: Always visible in the header (e.g., rescan with CTRL+L)
- **Context menu items**: Accessible via F1 menu, with function key hotkeys (F2-F7, etc.)
- **Context-aware availability**: Menu items show/hide based on selected entry type and state

## Adding a New Menu Feature

Follow these steps to add a new menu feature consistently:

### 1. Define the Menu Action (src/menu.rs)

Add a new variant to the `MenuAction` enum:

```rust
#[derive(Debug, Clone)]
pub enum MenuAction {
    Edit,
    ToggleWatched,
    AssignToSeries,
    RepeatAction,
    Rescan,
    ClearSeriesData,
    UnwatchAll,
    YourNewAction,  // Add your new action here
}
```

### 2. Define the Menu Item (src/menu.rs)

Add a new `MenuItem` in the `define_all_menu_items()` function:

```rust
MenuItem {
    label: "Your Action Label".to_string(),
    hotkey: Some(KeyCode::F(8)),  // Choose next available F-key
    action: MenuAction::YourNewAction,
    location: MenuLocation::ContextMenu,  // or FirstLine
}
```

**Hotkey Guidelines:**
- F1: Reserved for opening the context menu
- F2-F7: Currently assigned to context menu actions
- F8+: Available for new context menu actions
- CTRL+L: Reserved for rescan (first-line action)
- Choose hotkeys that don't conflict with existing ones

**Location Guidelines:**
- Use `MenuLocation::ContextMenu` for most actions (accessible via F1)
- Use `MenuLocation::FirstLine` only for frequently-used, always-available actions

### 3. Define Availability Logic (src/menu.rs)

Add a match arm in the `is_item_available()` function to control when your action is available:

```rust
fn is_item_available(item: &MenuItem, context: &MenuContext) -> bool {
    match &item.action {
        // ... existing actions ...
        MenuAction::YourNewAction => {
            // Define when this action should be available
            // Examples:
            
            // Available only for episodes:
            matches!(context.selected_entry, Some(Entry::Episode { .. }))
            
            // Available only for episodes with series data:
            if let Some(Entry::Episode { .. }) = context.selected_entry {
                context.episode_detail.series.is_some()
            } else {
                false
            }
            
            // Available in specific view contexts:
            matches!(context.view_context, ViewContext::Season { .. })
            
            // Always available:
            true
        }
    }
}
```

**Common Availability Patterns:**
- Episode-only: `matches!(context.selected_entry, Some(Entry::Episode { .. }))`
- Series-only: `matches!(context.selected_entry, Some(Entry::Series { .. }))`
- Season-only: `matches!(context.selected_entry, Some(Entry::Season { .. }))`
- Episode with series: Check `context.episode_detail.series.is_some()`
- Episode without series: Check `context.episode_detail.series.is_none()`
- Context-specific: Check `context.view_context` (TopLevel, Series, Season)
- Always available: `true`

### 4. Implement the Action Handler (src/handlers.rs)

Add a match arm in the `execute_menu_action()` function:

```rust
fn execute_menu_action(
    action: &MenuAction,
    // ... parameters ...
) {
    match action {
        // ... existing actions ...
        MenuAction::YourNewAction => {
            // Implement your action logic here
            
            // Common patterns:
            
            // 1. Get the episode ID if needed:
            if let Entry::Episode { episode_id, .. } = filtered_entries[remembered_item] {
                // Perform database operation
                database::your_operation(episode_id)
                    .expect("Failed to perform operation");
            }
            
            // 2. Reload entries based on current view context:
            *entries = match view_context {
                ViewContext::TopLevel => 
                    database::get_entries().expect("Failed to get entries"),
                ViewContext::Series { series_id } => 
                    database::get_entries_for_series(*series_id)
                        .expect("Failed to get entries for series"),
                ViewContext::Season { season_id } => 
                    database::get_entries_for_season(*season_id)
                        .expect("Failed to get entries for season"),
            };
            
            // 3. Update filtered entries and return to Browse mode:
            *filtered_entries = entries.clone();
            *mode = Mode::Browse;
            *redraw = true;
        }
    }
}
```

**Action Implementation Patterns:**

1. **Simple toggle/update on current episode:**
   - Get episode_id from `filtered_entries[remembered_item]`
   - Call database function
   - Reload entries based on view_context
   - Return to Browse mode

2. **Mode transition (e.g., to Edit mode):**
   - Set `*mode = Mode::YourNewMode`
   - Initialize any required state
   - Set `*redraw = true`

3. **Bulk operations (e.g., unwatch all):**
   - Determine scope from `view_context`
   - Call appropriate database function for each context
   - Reload entries
   - Return to Browse mode

4. **Operations with last_action tracking:**
   - Perform the operation
   - Update `*last_action` with relevant details
   - Reload entries
   - Return to Browse mode

### 5. Add Database Operations (src/database.rs)

If your action requires new database operations, add them to `database.rs`:

```rust
pub fn your_operation(episode_id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_CONN.lock().unwrap();
    conn.execute(
        "UPDATE episode SET field = ? WHERE id = ?",
        params![value, episode_id],
    )?;
    Ok(())
}
```

**Database Operation Guidelines:**
- Use the global `DB_CONN` mutex for database access
- Return `Result<T, Box<dyn std::error::Error>>` for error handling
- Use parameterized queries to prevent SQL injection
- Consider adding bulk operations for context-aware actions (e.g., `_in_season`, `_in_series`, `_standalone`)

## Testing Your New Feature

1. **Build and run:**
   ```bash
   cargo build
   cargo run
   ```

2. **Test availability:**
   - Navigate to different entry types (Series, Season, Episode)
   - Open the F1 menu and verify your action appears only when expected
   - Test in different view contexts (TopLevel, Series view, Season view)

3. **Test functionality:**
   - Execute your action via hotkey
   - Execute your action via F1 menu selection
   - Verify the action performs correctly
   - Verify entries reload properly
   - Verify the UI returns to Browse mode

4. **Test edge cases:**
   - Test with empty database
   - Test with episodes that have/don't have series data
   - Test in different view contexts

## Menu System Architecture

### Key Components

1. **MenuItem struct**: Defines label, hotkey, action, and location
2. **MenuAction enum**: Identifies the action to perform
3. **MenuLocation enum**: Determines where the item appears (FirstLine or ContextMenu)
4. **MenuContext struct**: Provides context for availability checks

### Data Flow

1. User presses F1 or a hotkey
2. `get_context_menu_items()` filters available items based on current context
3. Menu displays available items
4. User selects an item (Enter or hotkey)
5. `execute_menu_action()` performs the action
6. Entries reload and UI updates

### Hotkey Handling

Hotkeys work in two places:
1. **Browse mode**: Direct hotkey execution (checked in `handle_browse_mode`)
2. **Menu mode**: Hotkey selection within F1 menu (checked in `handle_menu_mode`)

Both paths call `execute_menu_action()` for consistent behavior.

## Best Practices

1. **Naming**: Use clear, action-oriented labels (e.g., "toggle watched", not "watched")
2. **Availability**: Be specific about when actions are available to avoid confusion
3. **Error handling**: Use `.expect()` with descriptive messages for database operations
4. **Consistency**: Always reload entries and return to Browse mode after actions
5. **Context awareness**: Consider view_context when reloading entries
6. **Hotkeys**: Choose intuitive hotkeys that don't conflict with existing ones
7. **Testing**: Test in all view contexts and with different entry types

## Common Pitfalls

1. **Forgetting to reload entries**: Always reload after database changes
2. **Not handling view_context**: Use the match pattern to reload correctly
3. **Incorrect availability logic**: Test with different entry types and states
4. **Hotkey conflicts**: Check existing hotkeys before assigning new ones
5. **Not returning to Browse mode**: Always set `*mode = Mode::Browse` after actions
6. **Missing redraw**: Always set `*redraw = true` after state changes
