# Design Document

## Overview

This design addresses a bug in the browse mode filtering behavior where filter text persists inappropriately during navigation. The fix ensures that filter text is automatically cleared when:
1. Navigating into a Series (pressing ENTER on a Series entry)
2. Navigating into a Season (pressing ENTER on a Season entry)
3. Navigating back with ESC (while maintaining normal ESC navigation behavior)

The solution is minimal and focused, requiring changes only to the `handle_browse_mode` function in `src/handlers.rs`.

## Architecture

### Current Filter Implementation

The application maintains filter state through several variables in the main loop:
- `search: String` - Contains the filter text entered by the user
- `filtered_entries: Vec<Entry>` - The list of entries after applying the filter
- `filter_mode: bool` - Indicates whether the user is actively editing the filter text

The filtering logic in `main_loop` (src/main.rs) processes the `search` string on every redraw:
```rust
let search_terms: Vec<String> = search
    .to_lowercase()
    .split_whitespace()
    .map(String::from)
    .collect();

filtered_entries = entries
    .iter()
    .filter(|entry| {
        // Filter logic based on search_terms
    })
    .cloned()
    .collect();
```

### Current Navigation Behavior

Navigation in browse mode is handled by `handle_browse_mode` in `src/handlers.rs`:

1. **ENTER on Series**: Loads entries for that series and changes view context
2. **ENTER on Season**: Loads entries for that season and changes view context
3. **ENTER on Episode**: Plays the video (no context change)
4. **ESC**: Navigates back through contexts (Season → Series → TopLevel → Exit)

Currently, the `search` string is only cleared in specific ESC handlers, but not consistently across all navigation scenarios.

## Components and Interfaces

### Modified Component: `handle_browse_mode` Function

**Location**: `src/handlers.rs`

**Current Signature**:
```rust
pub fn handle_browse_mode(
    code: KeyCode,
    modifiers: event::KeyModifiers,
    current_item: &mut usize,
    first_entry: &mut usize,
    filtered_entries: &mut Vec<Entry>,
    entries: &mut Vec<Entry>,
    search: &mut String,
    // ... other parameters
) -> io::Result<bool>
```

**Modifications Required**:

Three specific match arms need to be updated to clear the `search` string:

1. **ENTER on Series Entry** (line ~645):
   - Add `search.clear();` before loading series entries
   - This ensures the series view shows all seasons/episodes unfiltered

2. **ENTER on Season Entry** (line ~675):
   - Add `search.clear();` before loading season entries
   - This ensures the season view shows all episodes unfiltered

3. **ESC handlers** (lines ~685-715):
   - The existing ESC handlers already clear search in some cases
   - Need to ensure ALL ESC navigation paths clear the search string
   - Three ESC handlers exist for different navigation contexts

## Data Models

No changes to data models are required. The fix operates on existing state variables:

- `search: String` - The filter text that needs to be cleared
- `filtered_entries: Vec<Entry>` - Will be automatically updated by the main loop's filter logic after `search` is cleared
- `view_context: ViewContext` - Already tracks navigation context (TopLevel, Series, Season)

## Error Handling

No new error handling is required. The changes are simple state mutations that cannot fail:
- `search.clear()` is an infallible operation on a String
- The existing error handling for database operations and entry loading remains unchanged

## Testing Strategy

### Manual Testing Scenarios

1. **Filter + Navigate into Series**:
   - Start at TopLevel view
   - Type `/` to enter filter mode
   - Enter filter text (e.g., "test")
   - Press ENTER to accept filter
   - Navigate to a filtered Series entry
   - Press ENTER to navigate into the Series
   - **Expected**: Filter text is cleared, all seasons/episodes in the series are visible

2. **Filter + Navigate into Season**:
   - Navigate into a Series view
   - Type `/` to enter filter mode
   - Enter filter text
   - Press ENTER to accept filter
   - Navigate to a filtered Season entry
   - Press ENTER to navigate into the Season
   - **Expected**: Filter text is cleared, all episodes in the season are visible

3. **Filter + ESC from Season**:
   - Navigate into a Season view
   - Type `/` to enter filter mode
   - Enter filter text
   - Press ENTER to accept filter
   - Press ESC to navigate back to Series view
   - **Expected**: Filter text is cleared, all seasons in the series are visible

4. **Filter + ESC from Series**:
   - Navigate into a Series view
   - Type `/` to enter filter mode
   - Enter filter text
   - Press ENTER to accept filter
   - Press ESC to navigate back to TopLevel view
   - **Expected**: Filter text is cleared, all top-level entries are visible

5. **Filter + ESC from TopLevel**:
   - At TopLevel view
   - Type `/` to enter filter mode
   - Enter filter text
   - Press ENTER to accept filter
   - Press ESC to exit application
   - **Expected**: Application exits normally

6. **Filter + Navigate into Episode** (no change expected):
   - Apply a filter
   - Navigate to an Episode entry
   - Press ENTER to play the video
   - **Expected**: Filter text remains, video plays

### Edge Cases

1. **Empty filter text**: Navigation should work normally (no-op for clear)
2. **Filter mode active**: ESC in filter mode should cancel filter mode, not navigate
3. **No entries**: Navigation with empty lists should not crash
4. **Rapid navigation**: Multiple ENTER/ESC presses should handle filter clearing correctly

### Verification Points

After each navigation action that should clear the filter:
1. The `search` string should be empty
2. The `filtered_entries` list should match the full `entries` list
3. The display should show all entries in the new context
4. The cursor position should be valid for the new entry list

## Implementation Notes

### Code Locations

The changes are localized to `src/handlers.rs` in the `handle_browse_mode` function:

1. **Line ~645** - ENTER on Series:
```rust
Entry::Series { series_id, .. } => {
    search.clear();  // ADD THIS LINE
    *current_item = 0;
    *entries = database::get_entries_for_series(*series_id)
        .expect("Failed to get entries for series");
    *filtered_entries = entries.clone();
    *view_context = ViewContext::Series { series_id: *series_id };
    *redraw = true;
}
```

2. **Line ~675** - ENTER on Season:
```rust
Entry::Season { season_id, .. } => {
    search.clear();  // ADD THIS LINE
    *current_item = 0;
    *entries = database::get_entries_for_season(*season_id)
        .expect("Failed to get entries for season");
    *filtered_entries = entries.clone();
    *view_context = ViewContext::Season { season_id: *season_id };
    *redraw = true;
}
```

3. **Lines ~685-715** - ESC handlers:
   - Review each ESC handler to ensure `search.clear()` is present
   - Some already have it, verify all three ESC navigation paths include it

### Why This Approach

1. **Minimal Changes**: Only adds `search.clear()` calls at strategic points
2. **Leverages Existing Logic**: The main loop's filter logic automatically handles the cleared search string
3. **No New State**: Doesn't introduce new variables or flags
4. **Consistent Behavior**: Uses the same clearing mechanism across all navigation scenarios
5. **No Side Effects**: Clearing the search string is a safe, isolated operation

### Alternative Approaches Considered

1. **Clear filter on any navigation**: Too aggressive, would clear filter when navigating to episodes
2. **Add a "should_clear_filter" flag**: Unnecessary complexity, direct clearing is simpler
3. **Modify filter logic in main loop**: Would require more extensive changes and testing

The chosen approach is the simplest and most maintainable solution that directly addresses the requirements.
