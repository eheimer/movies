# Design Document

## Overview

This feature enhances the episode editing workflow by automatically positioning the cursor on the episode number field and pre-filling it with the next available episode number when entering edit mode for an episode that:

1. Is already assigned to a series
2. Does not yet have an episode number assigned

This reduces the workflow for assigning sequential episode numbers from multiple keystrokes to just two F2 presses.

## Architecture

The feature integrates into the existing edit mode entry flow in the browse mode handler. When F2 is pressed on an episode, the system will:

1. Check if the episode has a series assigned AND no episode number
2. If both conditions are met:
   - Calculate the next available episode number for that series/season combination
   - Set the initial cursor position to the episode number field
   - Pre-fill the episode number field with the calculated value
3. If conditions are not met, proceed with normal edit mode behavior (cursor on first field)

## Components and Interfaces

### Modified Components

#### 1. `handle_browse_mode` (src/handlers.rs)

The F2 key handler in browse mode needs to be enhanced to:

- Detect if the episode has a series but no episode number
- Calculate the next available episode number
- Set the initial `edit_field` to `EpisodeField::EpisodeNumber` instead of `EpisodeField::Title`
- Pre-fill `edit_details.episode_number` with the calculated value

**Current behavior:**

```rust
KeyCode::F(2) => {
    if let Entry::Episode { .. } = filtered_entries[*current_item] {
        *mode = Mode::Edit;
        let episode_id = match &filtered_entries[*current_item] {
            Entry::Episode { episode_id, .. } => *episode_id,
            _ => 0,
        };
        *edit_details = database::get_episode_detail(episode_id)
            .expect("Failed to get entry details");
        *season_number = edit_details.season.as_ref().map(|season| season.number);
        *redraw = true;
    }
}
```

**Enhanced behavior:**

```rust
KeyCode::F(2) => {
    if let Entry::Episode { .. } = filtered_entries[*current_item] {
        *mode = Mode::Edit;
        let episode_id = match &filtered_entries[*current_item] {
            Entry::Episode { episode_id, .. } => *episode_id,
            _ => 0,
        };
        *edit_details = database::get_episode_detail(episode_id)
            .expect("Failed to get entry details");
        *season_number = edit_details.season.as_ref().map(|season| season.number);

        // NEW: Auto-fill episode number if series is assigned but episode number is not
        if edit_details.series.is_some()
            && (edit_details.episode_number.is_empty() || edit_details.episode_number == "0") {
            // Calculate next available episode number
            let next_episode = database::get_next_available_episode_number(
                edit_details.series.as_ref().unwrap().id,
                season_number.as_ref().copied()
            ).unwrap_or(1);

            // Pre-fill the episode number
            edit_details.episode_number = next_episode.to_string();

            // Set cursor to episode number field
            *edit_field = EpisodeField::EpisodeNumber;
            *edit_cursor_pos = edit_details.episode_number.len();
        } else {
            // Normal behavior: start at first field
            *edit_field = EpisodeField::Title;
            *edit_cursor_pos = 0;
        }

        *redraw = true;
    }
}
```

#### 2. New Database Function: `get_next_available_episode_number` (src/database.rs)

A new database query function that calculates the next available episode number for a given series and optional season.

**Function signature:**

```rust
pub fn get_next_available_episode_number(
    series_id: usize,
    season_number: Option<usize>
) -> Result<usize>
```

**Algorithm:**

1. Query all episode numbers for the given series_id and season_number combination
2. Filter out NULL and empty episode numbers
3. Convert to integers and sort
4. Find the first gap in the sequence starting from 1
5. If no gaps exist, return max + 1
6. If no episodes exist, return 1

**SQL Query Logic:**

```sql
-- For episodes with a season
SELECT CAST(episode_number AS INTEGER) as ep_num
FROM episode
WHERE series_id = ?1
  AND season_id = (SELECT id FROM season WHERE series_id = ?1 AND number = ?2)
  AND episode_number IS NOT NULL
  AND episode_number != ''
ORDER BY ep_num

-- For episodes without a season (season_number is None)
SELECT CAST(episode_number AS INTEGER) as ep_num
FROM episode
WHERE series_id = ?1
  AND season_id IS NULL
  AND episode_number IS NOT NULL
  AND episode_number != ''
ORDER BY ep_num
```

### Data Models

No changes to existing data models. The feature uses existing fields:

- `EpisodeDetail.series` (Option<Series>)
- `EpisodeDetail.episode_number` (String)
- `EpisodeDetail.season` (Option<Season>)

### State Management

The feature modifies the initial state when entering edit mode:

- `edit_field`: Set to `EpisodeField::EpisodeNumber` instead of `EpisodeField::Title` when conditions are met
- `edit_details.episode_number`: Pre-filled with calculated value
- `edit_cursor_pos`: Set to the end of the pre-filled episode number string

## Error Handling

### Database Query Errors

If `get_next_available_episode_number` fails:

- Return a default value of 1 using `.unwrap_or(1)`
- Log error to stderr (optional)
- Continue with edit mode entry

### Invalid Episode Number Data

If existing episode numbers in the database cannot be parsed as integers:

- Skip those entries when calculating the next available number
- Continue with remaining valid episode numbers

### Edge Cases

1. **No existing episodes in series/season**: Return 1
2. **All sequential numbers taken (1, 2, 3, ...)**: Return max + 1
3. **Gaps in sequence (1, 2, 4, 5)**: Return first gap (3)
4. **Episode already has a number**: Normal edit mode behavior (no auto-fill)
5. **Episode has no series**: Normal edit mode behavior (no auto-fill)
6. **Season is assigned but no season_number**: Treat as no season (query episodes with NULL season_id)

## Testing Strategy

### Unit Testing Approach

Since the codebase doesn't currently have unit tests, testing will be manual and focused on integration testing through the application UI.

### Manual Test Cases

1. **Basic auto-fill scenario**

   - Setup: Episode with series assigned, no episode number
   - Action: Press F2
   - Expected: Cursor on episode number field, pre-filled with "1"
   - Action: Press F2 again
   - Expected: Episode number saved, return to browse mode

2. **Sequential numbering**

   - Setup: Series with episodes 1, 2, 3 already numbered
   - Action: Edit new episode in same series
   - Expected: Pre-filled with "4"

3. **Gap filling**

   - Setup: Series with episodes 1, 2, 4, 5
   - Action: Edit new episode in same series
   - Expected: Pre-filled with "3"

4. **Episode with existing number**

   - Setup: Episode with series and episode number already assigned
   - Action: Press F2
   - Expected: Normal edit mode, cursor on title field, no auto-fill

5. **Episode without series**

   - Setup: Episode with no series assigned
   - Action: Press F2
   - Expected: Normal edit mode, cursor on title field

6. **Season-specific numbering**

   - Setup: Series with Season 1 having episodes 1-3, Season 2 having episodes 1-2
   - Action: Edit new episode in Season 2
   - Expected: Pre-filled with "3" (specific to Season 2)

7. **User modification of pre-filled value**

   - Setup: Episode with auto-filled episode number "4"
   - Action: User changes to "10" and presses F2
   - Expected: Episode number saved as "10"

8. **Empty series (first episode)**
   - Setup: New series with no episodes numbered yet
   - Action: Edit first episode
   - Expected: Pre-filled with "1"

### Integration Points to Verify

1. Edit mode entry from browse mode
2. Database query for existing episode numbers
3. Episode number field editing behavior
4. Save operation when accepting pre-filled value
5. Cursor positioning and navigation in edit mode

## Implementation Notes

### Cursor Position

When auto-filling, the cursor should be positioned at the end of the pre-filled number string. This allows the user to:

- Press F2 immediately to accept
- Press Backspace to clear and type a different number
- Use arrow keys to navigate and edit

### Episode Number Validation

The existing episode number field accepts string input. The new function will:

- Parse existing episode numbers as integers for calculation
- Return an integer that will be converted to string for display
- Invalid/unparseable episode numbers in the database will be ignored during calculation

### Performance Considerations

The database query for `get_next_available_episode_number` will:

- Use indexed columns (series_id, season_id) for efficient filtering
- Return a small result set (only episode numbers for one series/season)
- Execute only once per edit mode entry
- Have negligible performance impact

### Backward Compatibility

This feature is fully backward compatible:

- No database schema changes
- No changes to existing episode data
- Existing edit mode behavior preserved for episodes that don't meet the criteria
- No impact on other application modes or features
