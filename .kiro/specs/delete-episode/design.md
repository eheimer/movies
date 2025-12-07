# Design Document

## Overview

This feature adds a "Delete" menu action that removes episode database entries. The delete function will be accessible only through the F1 context menu (no hotkey) to prevent accidental deletions. The implementation follows the established menu feature pattern used by other actions like "toggle watched" and "clear series data".

## Architecture

The delete functionality integrates into the existing menu system architecture:

1. **Menu System** (`src/menu.rs`): Define the Delete action and menu item
2. **Database Layer** (`src/database.rs`): Implement the delete operation
3. **Handler Layer** (`src/handlers.rs`): Execute the delete action and refresh the display

The implementation follows the same pattern as existing menu actions, ensuring consistency with the codebase.

## Components and Interfaces

### Menu Action Definition

Add a new `MenuAction::Delete` variant to the `MenuAction` enum in `src/menu.rs`. This action will:
- Have no hotkey (to prevent accidental deletion)
- Only appear in the context menu (F1)
- Only be available when an episode is selected

### Database Operation

Add a `delete_episode(episode_id: usize)` function to `src/database.rs` that:
- Takes an episode ID as input
- Deletes the episode record from the database
- Returns `Result<(), Box<dyn std::error::Error>>`
- Uses the global `DB_CONN` mutex for database access

### Menu Action Handler

Extend the `execute_menu_action()` function in `src/handlers.rs` to handle `MenuAction::Delete`:
- Extract the episode_id from the remembered item
- Call `database::delete_episode(episode_id)`
- Reload entries based on the current view context (TopLevel, Series, or Season)
- Update filtered_entries
- Return to Browse mode
- Trigger a redraw

## Data Models

No new data models are required. The feature operates on existing structures:
- `Entry::Episode` - Contains the episode_id needed for deletion
- `ViewContext` - Determines which entries to reload after deletion
- `Mode` - Manages the application state transitions

## Testing Strategy

### Unit Tests

1. **Database deletion test**: Verify that `delete_episode()` removes the episode record
   - Create a test episode
   - Delete it
   - Verify it no longer exists in the database

2. **Menu availability test**: Verify Delete action only appears for episodes
   - Test with Episode entry: Delete should be available
   - Test with Series entry: Delete should not be available
   - Test with Season entry: Delete should not be available

3. **No hotkey test**: Verify Delete menu item has no hotkey assigned
   - Check that the MenuItem for Delete has `hotkey: None`

### Integration Tests

1. **Delete and reload test**: Verify entries reload correctly after deletion
   - Delete an episode from top-level view
   - Verify entries are reloaded from database
   - Delete an episode from series view
   - Verify series entries are reloaded
   - Delete an episode from season view
   - Verify season entries are reloaded

2. **Selection adjustment test**: Verify selection adjusts appropriately after deletion
   - Delete the last item in a list
   - Verify selection moves to previous item or first item if none exists

## Error Handling

The delete operation will use Rust's `Result` type for error handling:

- Database errors will be propagated using `?` operator
- The handler will use `.expect()` with descriptive messages for database operations
- If deletion fails, the error will be logged and the application will remain in a consistent state

Error scenarios:
- Episode ID does not exist: Database will return an error
- Database connection issues: Will be caught by the global DB_CONN mutex
- Foreign key constraints: SQLite will handle cascading deletes if configured

## Implementation Notes

### Following Established Patterns

The implementation will follow the exact pattern used by existing menu actions:

1. **Menu Definition Pattern** (from `MenuAction::ClearSeriesData`):
   ```rust
   MenuItem {
       label: "Delete".to_string(),
       hotkey: None,  // No hotkey for safety
       action: MenuAction::Delete,
       location: MenuLocation::ContextMenu,
   }
   ```

2. **Availability Pattern** (from `MenuAction::Edit`):
   ```rust
   MenuAction::Delete => {
       matches!(context.selected_entry, Some(Entry::Episode { .. }))
   }
   ```

3. **Handler Pattern** (from `MenuAction::ToggleWatched`):
   ```rust
   MenuAction::Delete => {
       if let Entry::Episode { episode_id, .. } = filtered_entries[remembered_item] {
           database::delete_episode(episode_id)
               .expect("Failed to delete episode");
           
           // Reload entries based on current view context
           *entries = match view_context {
               ViewContext::TopLevel => database::get_entries().expect("Failed to get entries"),
               ViewContext::Series { series_id, .. } => database::get_entries_for_series(*series_id)
                   .expect("Failed to get entries for series"),
               ViewContext::Season { season_id, .. } => database::get_entries_for_season(*season_id)
                   .expect("Failed to get entries for season"),
           };
           *filtered_entries = entries.clone();
           *mode = Mode::Browse;
           *redraw = true;
       }
   }
   ```

### Database Considerations

The SQLite database has foreign key relationships:
- Episodes reference series (series_id)
- Episodes reference seasons (season_id)

When an episode is deleted:
- The series and season records remain intact
- Only the episode record is removed
- If the video file still exists, it will be rediscovered on the next rescan

### User Experience

- Delete appears in the F1 menu only when an episode is selected
- No confirmation dialog (user must deliberately open menu and select Delete)
- After deletion, the display refreshes to show the updated list
- Selection remains on the same position if possible, or adjusts to the last item if needed
