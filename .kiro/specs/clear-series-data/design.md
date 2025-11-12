# Design Document

## Overview

This design adds a new menu option "Clear Series Data" with hotkey F6 that allows users to quickly remove series, season, and episode number assignments from an episode. The menu item is conditionally visible only when the selected episode has at least one of these fields populated.

## Architecture

The implementation follows the existing menu system architecture:

1. Add a new `MenuAction::ClearSeriesData` variant to the menu action enum
2. Add a new menu item definition with F6 hotkey
3. Implement conditional visibility logic based on episode metadata
4. Add a new database function to clear series-related fields
5. Implement the action handler in `execute_menu_action`

## Components and Interfaces

### Modified Component: `src/menu.rs`

**New MenuAction Variant:**

```rust
pub enum MenuAction {
    Edit,
    ToggleWatched,
    AssignToSeries,
    RepeatAction,
    Rescan,
    ClearSeriesData,  // NEW
}
```

**New Menu Item Definition:**

Add to `define_all_menu_items()`:

```rust
MenuItem {
    label: "Clear Series Data".to_string(),
    hotkey: Some(KeyCode::F(6)),
    action: MenuAction::ClearSeriesData,
    location: MenuLocation::ContextMenu,
}
```

**Conditional Visibility Logic:**

Add to `is_item_available()`:

```rust
MenuAction::ClearSeriesData => {
    // Available only when selected entry is an Episode with series data
    if let Some(Entry::Episode { .. }) = context.selected_entry {
        // Check if any series-related field is populated
        context.episode_detail.series.is_some()
            || context.episode_detail.season.is_some()
            || (!context.episode_detail.episode_number.is_empty()
                && context.episode_detail.episode_number != "0")
    } else {
        false
    }
}
```

### New Component: Database Function

**Location:** `src/database.rs`

**Function Signature:**

```rust
pub fn clear_series_data(episode_id: usize) -> Result<(), Box<dyn std::error::Error>>
```

**Implementation:**

```rust
pub fn clear_series_data(episode_id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_CONN.lock().unwrap();
    let conn = conn
        .as_ref()
        .expect("Database connection is not initialized");

    conn.execute(
        "UPDATE episode SET series_id = NULL, season_id = NULL, episode_number = NULL WHERE id = ?1",
        params![episode_id],
    )?;

    Ok(())
}
```

### Modified Component: `src/handlers.rs`

**Location:** `execute_menu_action` function

**New Action Handler:**

Add to the match statement in `execute_menu_action`:

```rust
MenuAction::ClearSeriesData => {
    // Clear series, season, and episode number for the remembered episode
    if let Entry::Episode { episode_id, .. } = filtered_entries[remembered_item] {
        database::clear_series_data(episode_id)
            .expect("Failed to clear series data");

        // Reload entries based on current view context
        *entries = match view_context {
            ViewContext::TopLevel => database::get_entries().expect("Failed to get entries"),
            ViewContext::Series { series_id } => database::get_entries_for_series(*series_id)
                .expect("Failed to get entries for series"),
            ViewContext::Season { season_id } => database::get_entries_for_season(*season_id)
                .expect("Failed to get entries for season"),
        };
        *filtered_entries = entries.clone();
        *mode = Mode::Browse;
        *redraw = true;
    }
}
```

## Data Flow

1. User navigates to an episode in Browse mode
2. User presses F1 to open context menu OR presses F6 directly
3. System checks if episode has series data via `is_item_available()`
4. If series data exists, "Clear Series Data [F6]" menu item is displayed
5. User selects the menu item or presses F6
6. `execute_menu_action` is called with `MenuAction::ClearSeriesData`
7. `database::clear_series_data()` sets series_id, season_id, and episode_number to NULL
8. Entries are reloaded based on current view context
9. System returns to Browse mode and triggers redraw
10. Episode now appears without series/season/episode number metadata

## Error Handling

The implementation uses existing error handling patterns:

- Database operations use `Result<(), Box<dyn std::error::Error>>`
- Errors are propagated with `expect()` for consistency with existing code
- No new error conditions are introduced

## Testing Strategy

### Manual Testing Scenarios

1. **Episode with all series data populated:**

   - Navigate to episode with series, season, and episode number
   - Press F1 to open context menu
   - Verify "Clear Series Data [F6]" is visible
   - Select the option or press F6
   - Verify all three fields are cleared
   - Verify episode remains in the list

2. **Episode with only series assigned:**

   - Navigate to episode with series but no season
   - Press F1
   - Verify "Clear Series Data [F6]" is visible
   - Execute the action
   - Verify series is cleared

3. **Episode with no series data:**

   - Navigate to standalone episode
   - Press F1
   - Verify "Clear Series Data [F6]" is NOT visible
   - Press F6 directly
   - Verify nothing happens (hotkey not registered)

4. **Episode with episode number "0":**

   - Navigate to episode with episode_number = "0"
   - Press F1
   - Verify "Clear Series Data [F6]" is NOT visible (0 treated as empty)

5. **Clear from different view contexts:**

   - Test clearing from TopLevel view
   - Test clearing from Series view
   - Test clearing from Season view
   - Verify entries reload correctly in each context

6. **Hotkey from Browse mode:**
   - Navigate to episode with series data
   - Press F6 directly (without opening menu)
   - Verify action executes immediately
   - Verify menu item availability check is respected

### Edge Cases

- Episode with empty string episode_number (should not show menu item)
- Episode with episode_number = "0" (should not show menu item)
- Episode with only season but no series (should show menu item)
- Clearing episode while in Season view (episode should disappear from season list)
- Clearing episode while in Series view (episode should move to "no season" section)

## Implementation Notes

### Consistency with Existing Patterns

The implementation follows established patterns:

- Menu action enum variant naming: `ClearSeriesData`
- Menu item label format: "Clear Series Data"
- Hotkey assignment: F6 (next available function key)
- Database function naming: `clear_series_data`
- Error handling: `expect()` with descriptive messages
- Entry reloading: Match on `view_context` to reload appropriate entries

### Database Cleanup

The existing `initialize_database()` function already includes cleanup logic that will:

- Remove orphaned seasons (seasons with no episodes)
- Remove orphaned series (series with no episodes)

This means clearing series data from episodes will automatically trigger cleanup of unused series/seasons on next database initialization.

### Minimal Change Principle

The implementation requires:

1. One new enum variant in `MenuAction`
2. One new menu item in `define_all_menu_items()`
3. One new case in `is_item_available()`
4. One new database function `clear_series_data()`
5. One new case in `execute_menu_action()`

This maintains code simplicity and follows the existing architecture without introducing new patterns or complexity.
