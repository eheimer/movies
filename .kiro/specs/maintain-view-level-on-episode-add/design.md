# Design Document

## Overview

This feature modifies the episode assignment workflow to maintain the user's current view level after adding an episode to a series or season. Currently, when a user assigns an episode to a series or season through the SeriesSelect or SeriesCreate modes, the application automatically navigates to that series or season view. This behavior disrupts batch organization workflows where users want to assign multiple episodes without losing their place.

The solution involves tracking the current view context and using it to reload the appropriate entries after assignment operations complete, rather than always navigating to the newly assigned series/season.

## Architecture

### Current Behavior

The application currently operates with the following flow:

1. User is in Browse mode at any view level (TopLevel, Series, or Season)
2. User presses F4 to enter SeriesSelect mode for an episode
3. User selects or creates a series
4. `assign_series()` or `create_series_and_assign()` is called
5. Entries are reloaded using `get_entries()` (always returns TopLevel view)
6. Application returns to Browse mode at TopLevel view

The same pattern occurs when assigning episodes to seasons through the Edit mode.

### Proposed Behavior

The modified flow will:

1. User is in Browse mode at any view level (TopLevel, Series, or Season)
2. User presses F4 to enter SeriesSelect mode for an episode
3. User selects or creates a series
4. `assign_series()` or `create_series_and_assign()` is called
5. Entries are reloaded based on the **current view_context** (TopLevel, Series, or Season)
6. Application returns to Browse mode at the **same view level**

## Components and Interfaces

### ViewContext (Existing)

The `ViewContext` enum in `src/util.rs` already tracks the current view level:

```rust
pub enum ViewContext {
    TopLevel,
    Series { series_id: usize },
    Season { season_id: usize },
}
```

This will be used to determine which database query to execute after assignment operations.

### Handler Functions (Modifications Required)

#### `handle_series_select_mode()` in `src/handlers.rs`

**Current signature:**

```rust
pub fn handle_series_select_mode(
    code: KeyCode,
    series_selection: &mut Option<usize>,
    mode: &mut Mode,
    redraw: &mut bool,
    series: &mut Vec<Series>,
    episode_id: usize,
    episode_detail: &mut EpisodeDetail,
    entries: &mut Vec<Entry>,
    filtered_entries: &mut Vec<Entry>,
)
```

**Modification needed:**

- Add `view_context: &ViewContext` parameter
- Replace hardcoded `get_entries()` call with context-aware entry loading

#### `handle_series_create_mode()` in `src/handlers.rs`

**Current signature:**

```rust
pub fn handle_series_create_mode(
    code: KeyCode,
    modifiers: event::KeyModifiers,
    mode: &mut Mode,
    redraw: &mut bool,
    new_series: &mut String,
    edit_cursor_pos: &mut usize,
    series: &mut Vec<Series>,
    episode_id: usize,
    episode_detail: &mut EpisodeDetail,
    entries: &mut Vec<Entry>,
    filtered_entries: &mut Vec<Entry>,
)
```

**Modification needed:**

- Add `view_context: &ViewContext` parameter
- Replace hardcoded `get_entries()` call with context-aware entry loading

#### `handle_edit_mode()` in `src/handlers.rs`

**Current signature:**

```rust
pub fn handle_edit_mode(
    code: KeyCode,
    modifiers: event::KeyModifiers,
    current_item: usize,
    filtered_entries: &mut Vec<Entry>,
    edit_details: &mut EpisodeDetail,
    season_number: &mut Option<usize>,
    entries: &mut Vec<Entry>,
    mode: &mut Mode,
    edit_field: &mut EpisodeField,
    edit_cursor_pos: &mut usize,
    redraw: &mut bool,
)
```

**Modification needed:**

- Add `view_context: &ViewContext` parameter
- Replace conditional entry loading logic with context-aware entry loading

### Main Loop (Modifications Required)

The `main_loop()` function in `src/main.rs` needs to pass the `view_context` to the handler functions.

## Data Models

No changes to data models are required. The existing `ViewContext` enum provides all necessary state tracking.

## Error Handling

### Assignment Operation Failures

If `assign_series()`, `create_series_and_assign()`, or `create_season_and_assign()` fail:

1. The error will propagate through the Result type
2. The view context will remain unchanged
3. The user will stay in the current mode and view level
4. An error message should be displayed (future enhancement)

### Invalid View Context States

The following scenarios need to be handled:

1. **Series deleted while viewing it**: If the current view context references a series that no longer exists, fall back to TopLevel view
2. **Season deleted while viewing it**: If the current view context references a season that no longer exists, fall back to the parent Series view or TopLevel if series is also gone

For this feature, we'll implement basic error handling by catching database errors and falling back to `get_entries()` (TopLevel view) if context-specific queries fail.

## Testing Strategy

### Manual Testing Scenarios

1. **TopLevel View Preservation**

   - Start at TopLevel view with multiple standalone episodes
   - Assign an episode to a series
   - Verify view remains at TopLevel showing all series and standalone episodes

2. **Series View Preservation**

   - Navigate into a series view
   - Assign an episode within that series to a season
   - Verify view remains at the same series view

3. **Season View Preservation**

   - Navigate into a season view
   - Edit an episode's metadata (which may trigger season assignment)
   - Verify view remains at the same season view

4. **Cross-Series Assignment**

   - Navigate into Series A
   - Assign an episode to Series B
   - Verify view remains at Series A (not navigating to Series B)

5. **New Series Creation**

   - Start at any view level
   - Create a new series and assign an episode to it
   - Verify view remains at the original view level

6. **Season Creation from Edit Mode**
   - Navigate into a series view
   - Edit an episode and assign it to a new season
   - Verify view remains at the series view (not navigating to the new season)

### Edge Cases

1. **Last Episode in View**: Assign the last visible episode in a view to a different series/season
2. **Filtered View**: Perform assignment while search filter is active
3. **Cursor Position**: Verify cursor remains on the same episode after assignment (or nearest valid entry if episode is no longer visible)

## Implementation Notes

### Context-Aware Entry Loading Pattern

A helper function or inline logic will be used to reload entries based on view context:

```rust
match view_context {
    ViewContext::TopLevel => {
        *entries = database::get_entries().expect("Failed to get entries");
    }
    ViewContext::Series { series_id } => {
        *entries = database::get_entries_for_series(*series_id)
            .expect("Failed to get entries for series");
    }
    ViewContext::Season { season_id } => {
        *entries = database::get_entries_for_season(*season_id)
            .expect("Failed to get entries for season");
    }
}
```

This pattern will be applied in three locations:

1. `handle_series_select_mode()` after series assignment
2. `handle_series_create_mode()` after series creation and assignment
3. `handle_edit_mode()` after episode detail updates (F2 save)

### Cursor Position Preservation

After reloading entries, the cursor position should be maintained on the same episode if it's still visible in the view. If the episode is no longer visible (e.g., moved to a different series), the cursor should remain at the same index or move to the nearest valid entry.

The current implementation already handles this through the `current_item` index, which is not reset during these operations.

### Search Filter Interaction

The search filter should be preserved and reapplied after assignment operations. The main loop already handles filtering during the redraw phase:

1. The `search` string is maintained across mode changes
2. On redraw, entries are filtered based on search terms
3. `filtered_entries` is updated with the filtered results

After reloading entries based on view context, setting `redraw = true` triggers the main loop to automatically reapply the search filter. This means:

- If the user had typed "star" to filter episodes
- After assigning an episode to a series
- The view stays at the same level
- The "star" filter is automatically reapplied to the reloaded entries
- Only matching entries are displayed

No additional code is needed beyond setting the redraw flag, as the main loop's existing filter logic handles this correctly. The user experience is seamless - the search filter remains active and continues to work as expected.

## Dependencies

No new dependencies are required. The feature uses existing:

- `ViewContext` enum for state tracking
- Database query functions (`get_entries()`, `get_entries_for_series()`, `get_entries_for_season()`)
- Handler functions and main loop structure
