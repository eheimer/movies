# Design Document

## Overview

The "Unwatch All" feature adds a context-aware menu action that clears the watched flag for all episodes in the current view. The feature integrates into the existing menu system with F7 as the hotkey and "Unwatch All" as the menu label. The scope of the operation depends on the ViewContext: Season, Series, or TopLevel (standalone episodes).

## Architecture

The feature follows the existing architecture patterns:

1. **Menu System Integration**: Add a new MenuAction variant and menu item definition
2. **Database Layer**: Add a new database function to perform bulk watched status updates
3. **Handler Layer**: Integrate the action into the execute_menu_action function
4. **Context-Aware Behavior**: Use ViewContext to determine which episodes to update

### Component Interaction Flow

```
User presses F7 → handle_browse_mode checks hotkey
                 ↓
         execute_menu_action(UnwatchAll)
                 ↓
         Determine scope from ViewContext
                 ↓
         database::unwatch_all_in_context()
                 ↓
         Reload entries and refresh display
```

## Components and Interfaces

### 1. Menu System (src/menu.rs)

**New MenuAction Variant**:

```rust
pub enum MenuAction {
    // ... existing variants
    UnwatchAll,
}
```

**New Menu Item Definition**:

```rust
MenuItem {
    label: "Unwatch All".to_string(),
    hotkey: Some(KeyCode::F(7)),
    action: MenuAction::UnwatchAll,
    location: MenuLocation::ContextMenu,
}
```

**Availability Logic**:
The UnwatchAll action should be available in all contexts where episodes exist:

- TopLevel view: Available if there are standalone episodes
- Series view: Available if the series has episodes
- Season view: Available if the season has episodes

The availability check will be implemented in `is_item_available()` function.

### 2. Database Layer (src/database.rs)

**New Function**:

```rust
pub fn unwatch_all_in_season(season_id: usize) -> Result<(), Box<dyn std::error::Error>>
```

- Updates all episodes in the specified season to set watched = false
- SQL: `UPDATE episode SET watched = false WHERE season_id = ?1`

```rust
pub fn unwatch_all_in_series(series_id: usize) -> Result<(), Box<dyn std::error::Error>>
```

- Updates all episodes across all seasons in the specified series to set watched = false
- SQL: `UPDATE episode SET watched = false WHERE series_id = ?1`

```rust
pub fn unwatch_all_standalone() -> Result<(), Box<dyn std::error::Error>>
```

- Updates all standalone episodes (not part of any series) to set watched = false
- SQL: `UPDATE episode SET watched = false WHERE series_id IS NULL`

### 3. Handler Layer (src/handlers.rs)

**Integration Point**: `execute_menu_action()` function

Add a new match arm for `MenuAction::UnwatchAll`:

```rust
MenuAction::UnwatchAll => {
    // Determine scope based on view_context
    match view_context {
        ViewContext::Season { season_id } => {
            database::unwatch_all_in_season(*season_id)
                .expect("Failed to unwatch all episodes in season");
        }
        ViewContext::Series { series_id } => {
            database::unwatch_all_in_series(*series_id)
                .expect("Failed to unwatch all episodes in series");
        }
        ViewContext::TopLevel => {
            database::unwatch_all_standalone()
                .expect("Failed to unwatch all standalone episodes");
        }
    }

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
```

## Data Models

No new data models are required. The feature operates on existing Episode records in the database, updating the `watched` boolean field.

## Error Handling

The feature follows the existing error handling patterns:

1. Database operations return `Result<(), Box<dyn std::error::Error>>`
2. Errors are handled with `.expect()` calls with descriptive messages
3. If a database operation fails, the application will panic with an error message
4. No special error recovery is needed as the operations are straightforward SQL updates

## Testing Strategy

### Manual Testing Scenarios

1. **Season Context**:

   - Navigate to a season with multiple episodes
   - Mark some episodes as watched
   - Press F7 or select "Unwatch All" from menu
   - Verify all episodes in the season are marked as unwatched
   - Verify episodes in other seasons are unaffected

2. **Series Context**:

   - Navigate to a series with multiple seasons
   - Mark episodes as watched across different seasons
   - Press F7 or select "Unwatch All" from menu
   - Verify all episodes across all seasons are marked as unwatched
   - Verify standalone episodes are unaffected

3. **TopLevel Context**:

   - View the top-level episode list
   - Mark some standalone episodes as watched
   - Press F7 or select "Unwatch All" from menu
   - Verify all standalone episodes are marked as unwatched
   - Verify episodes in series are unaffected

4. **Menu Integration**:

   - Open context menu (F1) in each view context
   - Verify "Unwatch All" appears in the menu
   - Verify F7 hotkey is displayed
   - Verify the action executes correctly from the menu

5. **Hotkey Functionality**:
   - Test F7 hotkey in Browse mode for each context
   - Verify it executes the action without opening the menu
   - Verify the display refreshes to show updated watched status

### Edge Cases

1. Empty views (no episodes): Action should be available but have no effect
2. All episodes already unwatched: Action should complete successfully with no visible change
3. Mixed watched/unwatched episodes: Only watched episodes should be affected
4. Database errors: Application should panic with descriptive error message

## Implementation Notes

1. The F7 hotkey is already checked in `handle_browse_mode()` through the hotkey matching loop, so no additional hotkey handling is needed
2. The menu item will automatically appear in the context menu when available
3. The action follows the same pattern as existing actions (ToggleWatched, ClearSeriesData)
4. No changes to the display layer are required; the existing rendering will show updated watched status
5. The feature is non-destructive and can be easily reversed by manually marking episodes as watched again
