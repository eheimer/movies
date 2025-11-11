# Design Document

## Overview

This design addresses the bug where the `last_action` state is not updated when creating a new series via `handle_series_create_mode`. The fix ensures that F5 repeat functionality works consistently whether a user selects an existing series or creates a new one.

The root cause is that `handle_series_create_mode` in `src/handlers.rs` does not update the `last_action` parameter after successfully creating and assigning a series, while `handle_series_select_mode` correctly does so.

## Architecture

The fix involves modifying the `handle_series_create_mode` function to accept and update the `last_action` parameter, mirroring the behavior already implemented in `handle_series_select_mode`.

### Current Flow (Buggy)

1. User presses F4 on an unassigned episode → enters SeriesSelect Mode
2. User presses '+' to create new series → enters SeriesCreate Mode
3. User types series name and presses Enter
4. `handle_series_create_mode` calls `database::create_series_and_assign()`
5. Episode detail is updated, entries are reloaded
6. **BUG**: `last_action` is NOT updated
7. User navigates to next episode and presses F5 → nothing happens (F5 not available)

### Fixed Flow

1. User presses F4 on an unassigned episode → enters SeriesSelect Mode
2. User presses '+' to create new series → enters SeriesCreate Mode
3. User types series name and presses Enter
4. `handle_series_create_mode` calls `database::create_series_and_assign()`
5. Episode detail is updated, entries are reloaded
6. **FIX**: `last_action` is updated with SeriesAssignment containing series_id and series_name
7. User navigates to next episode and presses F5 → episode is assigned to the newly created series

## Components and Interfaces

### Modified Function Signature

**File**: `src/handlers.rs`

**Function**: `handle_series_create_mode`

**Current Signature**:

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
    view_context: &ViewContext,
)
```

**New Signature**:

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
    view_context: &ViewContext,
    last_action: &mut Option<crate::util::LastAction>,  // NEW PARAMETER
)
```

### Modified Logic

**Location**: `handle_series_create_mode` function, `KeyCode::Enter` match arm

**Current Code**:

```rust
KeyCode::Enter => {
    // save the new series to the database
    *episode_detail = database::create_series_and_assign(new_series, episode_id)
        .expect("Failed to create series");
    // reload the series list
    *series = database::get_all_series().expect("Failed to get series");
    // Reload entries based on current view context
    *entries = match view_context { /* ... */ };
    *filtered_entries = entries.clone();
    *mode = Mode::Browse;
    *redraw = true;
}
```

**New Code**:

```rust
KeyCode::Enter => {
    // save the new series to the database
    *episode_detail = database::create_series_and_assign(new_series, episode_id)
        .expect("Failed to create series");

    // Update last_action with the series assignment
    if let Some(series) = &episode_detail.series {
        *last_action = Some(crate::util::LastAction::SeriesAssignment {
            series_id: series.id,
            series_name: series.name.clone(),
        });
    }

    // reload the series list
    *series = database::get_all_series().expect("Failed to get series");
    // Reload entries based on current view context
    *entries = match view_context { /* ... */ };
    *filtered_entries = entries.clone();
    *mode = Mode::Browse;
    *redraw = true;
}
```

### Call Site Update

**File**: `src/main.rs`

**Location**: Main event loop, `Mode::SeriesCreate` match arm

**Current Code**:

```rust
Mode::SeriesCreate => {
    if let Some(id) = selected_entry_id {
        handlers::handle_series_create_mode(
            code,
            modifiers,
            &mut mode,
            &mut redraw,
            &mut new_series,
            &mut edit_cursor_pos,
            &mut series,
            id,
            &mut edit_details,
            &mut entries,
            &mut filtered_entries,
            &view_context,
        );
    } else {
        // selected entry is a series, change mode back to browse
        mode = Mode::Browse;
        redraw = true;
    }
}
```

**New Code**:

```rust
Mode::SeriesCreate => {
    if let Some(id) = selected_entry_id {
        handlers::handle_series_create_mode(
            code,
            modifiers,
            &mut mode,
            &mut redraw,
            &mut new_series,
            &mut edit_cursor_pos,
            &mut series,
            id,
            &mut edit_details,
            &mut entries,
            &mut filtered_entries,
            &view_context,
            &mut last_action,  // NEW ARGUMENT
        );
    } else {
        // selected entry is a series, change mode back to browse
        mode = Mode::Browse;
        redraw = true;
    }
}
```

## Data Models

No changes to data models are required. The existing `LastAction` enum already supports the `SeriesAssignment` variant:

```rust
pub enum LastAction {
    SeriesAssignment {
        series_id: usize,
        series_name: String,
    },
    SeasonAssignment {
        series_id: usize,
        series_name: String,
        season_id: usize,
        season_number: usize,
    },
}
```

## Error Handling

The fix maintains existing error handling patterns:

- `database::create_series_and_assign()` uses `.expect()` for database errors
- The `last_action` update is conditional on `episode_detail.series` being `Some`, preventing panics if the database operation fails to populate the series field

## Testing Strategy

### Manual Testing Scenarios

1. **Create new series and verify F5 works**:

   - Start application
   - Select an unassigned episode
   - Press F4 to enter series selection
   - Press '+' to create new series
   - Type series name and press Enter
   - Navigate to another unassigned episode
   - Verify F5 is available (last action displayed in UI)
   - Press F5 and verify episode is assigned to the new series

2. **Create new series with season and verify F5 works**:

   - Select an unassigned episode
   - Press F2 to enter edit mode
   - Navigate to Series field and press F4
   - Press '+' to create new series
   - Type series name and press Enter
   - Navigate to Season field and press '+' to create season
   - Press F2 to save
   - Navigate to another unassigned episode
   - Verify F5 is available with season information
   - Press F5 and verify episode is assigned to the series and season

3. **Verify existing series selection still works**:

   - Select an unassigned episode
   - Press F4 to enter series selection
   - Select an existing series (not '+')
   - Press Enter
   - Navigate to another unassigned episode
   - Press F5 and verify assignment works

4. **Verify last action persists across navigation**:
   - Create a new series and assign an episode
   - Navigate through multiple episodes
   - Verify last action remains displayed
   - Press F5 on an unassigned episode and verify assignment works

### Edge Cases

- Creating a series with an empty name (existing validation should prevent this)
- Database failure during series creation (existing error handling applies)
- Pressing F5 when no last action exists (existing logic handles this)
- Pressing F5 on an episode already assigned to the last action series (existing `can_repeat_action` logic handles this)
