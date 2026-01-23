# Design Document: Auto-select First Unwatched Episode

## Overview

This feature automatically positions the cursor on the first unwatched episode when entering a new browsing context (series, season, or top level). This improves the user experience by eliminating manual searching for the next unwatched content, making sequential viewing workflows more efficient.

The implementation will add a helper function to find the first unwatched entry in a list and apply it at context entry points in the browse mode handler.

## Architecture

The feature integrates into the existing browse mode navigation system by:

1. Adding a utility function to find the first unwatched entry index
2. Applying auto-selection logic when entering new contexts (series, season, top level)
3. Preserving manual cursor movements within the current context
4. Respecting filtered entry lists (only considering visible entries)

## Components and Interfaces

### New Utility Function

```rust
// In src/util.rs or src/handlers.rs
pub fn find_first_unwatched_index(entries: &[Entry]) -> Option<usize>
```

This function:
- Takes a slice of Entry items
- Returns the index of the first unwatched episode, or None if all are watched
- For Series entries: checks if the series contains any unwatched episodes
- For Season entries: checks if the season contains any unwatched episodes
- For Episode entries: checks the watched status directly

### Modified Handler Functions

The `handle_browse_mode` function in `src/handlers.rs` will be modified at three key points:

1. **Series Entry** (when user presses Enter on a Series)
   - After loading entries with `database::get_entries_for_series()`
   - Before setting `*current_item = 0`
   - Apply auto-selection to position cursor on first unwatched episode

2. **Season Entry** (when user presses Enter on a Season)
   - After loading entries with `database::get_entries_for_season()`
   - Before setting `*current_item = 0`
   - Apply auto-selection to position cursor on first unwatched episode

3. **Top Level Entry** (when user presses Esc to return to top level)
   - After loading entries with `database::get_entries()`
   - Before setting `*current_item = 0`
   - Apply auto-selection to position cursor on first series with unwatched content

### Database Query Functions

For Series and Season entries, we need to check if they contain unwatched episodes. The existing functions will be used:

- `database::get_series_episode_counts(series_id)` - returns (total, unwatched) counts
- `database::get_season_episode_counts(season_id)` - returns (total, unwatched) counts

## Data Models

No new data models are required. The feature uses existing structures:

- `Entry` enum (Series, Season, Episode)
- `EpisodeDetail` struct (contains watched status)
- `ViewContext` enum (TopLevel, Series, Season)

## Testing Strategy

### Unit Tests

Test the `find_first_unwatched_index` function with various scenarios:

1. **Empty list** - should return None
2. **All watched** - should return None
3. **First unwatched** - should return index 0
4. **Middle unwatched** - should return correct index
5. **Mixed entry types** - should handle Series, Season, and Episode entries
6. **Series with unwatched episodes** - should identify series containing unwatched content
7. **Season with unwatched episodes** - should identify seasons containing unwatched content

### Integration Tests

Test the complete auto-selection behavior:

1. **Enter series with unwatched episodes** - cursor should position on first unwatched
2. **Enter series with all watched** - cursor should position on first entry
3. **Enter season with unwatched episodes** - cursor should position on first unwatched
4. **Enter season with all watched** - cursor should position on first entry
5. **Return to top level** - cursor should position on first series with unwatched content
6. **Manual navigation preserved** - moving cursor manually should not trigger auto-selection
7. **Filtered entries respected** - auto-selection should only consider visible entries

### Edge Cases

1. **Empty context** - entering a series/season with no episodes should handle gracefully
2. **Single entry** - should work correctly with only one entry
3. **Filter active** - auto-selection should respect current filter
4. **All content watched** - should fall back to first entry

## Error Handling

The feature has minimal error handling requirements:

- If `find_first_unwatched_index` returns None, fall back to index 0 (first entry)
- Database query errors are already handled by existing error handling in the handlers
- No new error conditions are introduced

## Implementation Notes

### Context Entry Detection

Auto-selection should only trigger when entering a NEW context, not when:
- Moving cursor within current context
- Returning to the same context (e.g., Esc then Enter on same series)
- Filtering entries within current context

The implementation will apply auto-selection immediately after:
- Loading new entries with `database::get_entries_for_*()` functions
- Setting the new `view_context`
- Before setting `*redraw = true`

### Performance Considerations

- The `find_first_unwatched_index` function iterates through entries once (O(n))
- For Series/Season entries, it makes database queries to check unwatched counts
- This is acceptable since context entry is an infrequent operation
- The function should be efficient and avoid unnecessary database calls

### Compatibility

The feature maintains backward compatibility:
- No changes to database schema
- No changes to configuration format
- No changes to existing keyboard shortcuts
- Existing navigation behavior is preserved (only adds auto-selection on context entry)
