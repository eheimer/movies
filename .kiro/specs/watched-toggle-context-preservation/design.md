# Design Document

## Overview

This design addresses the bug where toggling the watched status of an episode causes the view to reset to the top-level, losing the user's navigation context. The solution introduces explicit view context tracking to ensure that entry reloads maintain the current view (top-level, series, or season).

## Architecture

The fix will be implemented primarily in the main event loop (`main.rs`) and the browse mode handler (`handlers.rs`). The solution introduces a new enum to track the current view context and modifies the F3 handler to use this context when reloading entries.

### Key Components

1. **ViewContext Enum** (new): Tracks whether the user is viewing top-level entries, a specific series, or a specific season
2. **Main Loop**: Maintains the current view context and updates it during navigation
3. **Browse Mode Handler**: Uses view context to reload appropriate entries after toggling watched status

## Components and Interfaces

### ViewContext Enum

A new enum will be added to `util.rs` to represent the current view state:

```rust
#[derive(Debug, Clone)]
pub enum ViewContext {
    TopLevel,
    Series { series_id: usize },
    Season { season_id: usize },
}
```

This enum explicitly captures which view the user is currently in, making it easy to determine the correct database query for reloading entries.

### Main Loop Modifications

The main loop in `main.rs` will:

1. Add a `view_context` variable initialized to `ViewContext::TopLevel`
2. Update `view_context` when the user navigates into a series (Enter on Series entry)
3. Update `view_context` when the user navigates into a season (Enter on Season entry)
4. Update `view_context` when the user navigates back (Esc key)
5. Pass `view_context` to the browse mode handler

### Browse Mode Handler Modifications

The `handle_browse_mode` function in `handlers.rs` will:

1. Accept a mutable reference to `view_context` as a parameter
2. Update `view_context` when handling Enter key on Series/Season entries
3. Update `view_context` when handling Esc key to navigate back
4. Use `view_context` in the F3 handler to determine which database query to call:
   - `ViewContext::TopLevel` → `database::get_entries()`
   - `ViewContext::Series { series_id }` → `database::get_entries_for_series(series_id)`
   - `ViewContext::Season { season_id }` → `database::get_entries_for_season(season_id)`

## Data Models

### ViewContext

```rust
pub enum ViewContext {
    TopLevel,                      // Viewing all series and standalone episodes
    Series { series_id: usize },   // Viewing seasons and episodes within a series
    Season { season_id: usize },   // Viewing episodes within a season
}
```

## Error Handling

No new error handling is required. The existing error handling for database operations will continue to work as-is. The view context tracking is purely in-memory state management.

## Testing Strategy

### Manual Testing

1. **Series View Test**:

   - Navigate into a series
   - Select an episode
   - Press F3 to toggle watched status
   - Verify the view remains in the series (showing seasons and episodes for that series)
   - Verify the same episode remains selected

2. **Season View Test**:

   - Navigate into a series, then into a season
   - Select an episode
   - Press F3 to toggle watched status
   - Verify the view remains in the season (showing episodes for that season)
   - Verify the same episode remains selected

3. **Top-Level View Test**:

   - At the top level, select a standalone episode
   - Press F3 to toggle watched status
   - Verify the view remains at the top level
   - Verify the same episode remains selected

4. **Navigation Test**:
   - Verify that Esc key correctly updates view context when navigating back
   - Verify that Enter key correctly updates view context when navigating forward

### Edge Cases

1. Empty views (no episodes in a season/series)
2. Single episode in a view
3. Rapid toggling of watched status
4. Toggling while search filter is active
