# Menu Feature Implementation

## Overview

Menu system with first-line items (always visible, e.g., CTRL+L rescan) and context menu items (F1 menu with F2-F7+ hotkeys). Items show/hide based on entry type and state.

## Adding a Menu Feature

### 1. Add MenuAction variant (src/menu.rs)

```rust
pub enum MenuAction {
    YourNewAction,  // Add here
}
```

### 2. Add MenuItem (src/menu.rs)

```rust
MenuItem {
    label: "Your Action Label".to_string(),
    hotkey: Some(KeyCode::F(8)),  // F8+ available
    action: MenuAction::YourNewAction,
    location: MenuLocation::ContextMenu,  // or FirstLine for frequent actions
}
```

### 3. Define Availability (src/menu.rs)

Add match arm in `is_item_available()`:

```rust
MenuAction::YourNewAction => {
    matches!(context.selected_entry, Some(Entry::Episode { .. }))
}
```

**Patterns:**
- Episode/Series/Season only: `matches!(context.selected_entry, Some(Entry::Episode { .. }))`
- Episode with series: `context.episode_detail.series.is_some()`
- View context: `matches!(context.view_context, ViewContext::Season { .. })`

### 4. Implement Handler (src/handlers.rs)

Add match arm in `execute_menu_action()`:

```rust
MenuAction::YourNewAction => {
    if let Entry::Episode { episode_id, .. } = filtered_entries[remembered_item] {
        database::your_operation(episode_id).expect("Failed");
    }
    *entries = match view_context {
        ViewContext::TopLevel => database::get_entries().expect("Failed"),
        ViewContext::Series { series_id } => database::get_entries_for_series(*series_id).expect("Failed"),
        ViewContext::Season { season_id } => database::get_entries_for_season(*season_id).expect("Failed"),
    };
    *filtered_entries = entries.clone();
    *mode = Mode::Browse;
    *redraw = true;
}
```

### 5. Add Database Operation (src/database.rs)

```rust
pub fn your_operation(episode_id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_CONN.lock().unwrap();
    conn.execute("UPDATE episode SET field = ? WHERE id = ?", params![value, episode_id])?;
    Ok(())
}
```

## Key Points

- Always reload entries after database changes using view_context match pattern
- Return to Browse mode and set `*redraw = true`
- Use `.expect()` with descriptive messages
- Test in different view contexts (TopLevel, Series, Season)
