# Design Document

## Overview

This feature adds a "repeat last action" capability using the F5 key, allowing users to quickly assign multiple episodes to the same series or season without repeatedly navigating through selection menus. The system will track the most recent assignment operation (series or season) and display it on screen when applicable. The F5 action will only be available when the selected item can be validly assigned using the stored action.

## Architecture

### State Management

The repeat action feature requires adding new state to the main event loop in `main.rs`:

1. **LastAction enum**: Represents the type of last action performed

   - `SeriesAssignment { series_id: usize, series_name: String }`
   - `SeasonAssignment { series_id: usize, series_name: String, season_id: usize, season_number: usize }`

2. **last_action state variable**: `Option<LastAction>` stored in the main loop

### Validation Logic

The system must validate whether the F5 action can be performed based on:

1. **Episode-only validation**: Selected item must be an Episode (not Series or Season)
2. **Series assignment validation**:
   - Episode must not already be assigned to the target series
3. **Season assignment validation**:
   - Episode must not already be assigned to the target season
   - Episodes can be moved between series/seasons freely (reassignment is allowed)

### Display Integration

The feature requires modifications to the display system:

1. **Action display line**: New line between menu and filter showing "Last action: [details]"
2. **Menu modification**: Conditionally show "[F5] Repeat action" in the menu
3. **Dynamic updates**: Display updates when selection changes or last action changes

## Components and Interfaces

### 1. LastAction Enum (util.rs)

```rust
#[derive(Debug, Clone)]
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

impl LastAction {
    pub fn format_display(&self) -> String {
        match self {
            LastAction::SeriesAssignment { series_name, .. } => {
                format!("Last action: {}", series_name)
            }
            LastAction::SeasonAssignment { series_name, season_number, .. } => {
                format!("Last action: {}, Season {}", series_name, season_number)
            }
        }
    }
}
```

### 2. Validation Functions (util.rs or handlers.rs)

```rust
pub fn can_repeat_action(
    last_action: &Option<LastAction>,
    selected_entry: &Entry,
    episode_detail: &EpisodeDetail,
) -> bool {
    // Returns true if F5 can be used with the current selection
    // Implements validation logic described in Architecture section
}
```

### 3. Handler Modifications

#### handle_browse_mode (handlers.rs)

Add F5 key handler:

- Check if `can_repeat_action` returns true
- Execute appropriate assignment based on `LastAction` type
- Update entries and redraw
- Maintain the same `last_action` (don't update it)

#### handle_series_select_mode (handlers.rs)

On successful series assignment (Enter key):

- Update `last_action` to `LastAction::SeriesAssignment`
- Store series_id and series_name

#### handle_edit_mode (handlers.rs)

On successful save (F2 key) when season is assigned:

- Update `last_action` to `LastAction::SeasonAssignment`
- Store series_id, series_name, season_id, and season_number

### 4. Display Modifications (display.rs)

#### draw_header function

Add parameter: `last_action_display: &str`

Modify to:

1. Print the last action display line at row 2 (between menu and filter)
2. Adjust filter line to row 4 (was row 3)
3. Conditionally include "[F5] Repeat action" in menu based on whether last_action_display is empty

Update HEADER_SIZE constant from 5 to 6 to account for new line.

#### draw_screen function

Add parameter: `last_action: &Option<LastAction>`

Calculate `last_action_display` string:

- If `can_repeat_action` returns true: format using `last_action.format_display()`
- Otherwise: empty string

Pass `last_action_display` to `draw_header`.

### 5. Main Loop Modifications (main.rs)

Add state variable:

```rust
let mut last_action: Option<LastAction> = None;
```

Pass `last_action` to:

- `draw_screen` function
- `handle_browse_mode` function (for F5 handling)
- `handle_series_select_mode` function (for updating on series assignment)
- `handle_edit_mode` function (for updating on season assignment)

## Data Models

### LastAction Enum

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

### No Database Changes

This feature requires no database schema modifications. All state is maintained in memory during the application session.

## Error Handling

### Validation Failures

When F5 is pressed but validation fails:

- No action is performed
- No error message is displayed
- This may occur in normal operation but the keypress should simply be ignored if it's not valid

### Database Operation Failures

When assignment operations fail:

- Use existing error handling patterns (`.expect()` with descriptive messages)
- Follow the same error handling as existing series/season assignment code

### Edge Cases

1. **Last action references deleted series/season**:
   - Validation will fail (series/season won't exist in episode_detail)
   - F5 will not be shown
2. **Episode already assigned**:

   - Validation prevents showing F5
   - F5 keypress despite no display should simply be ignored
   - No duplicate assignments possible

3. **Empty entries list**:
   - F5 not shown (no valid selection)
   - F5 keypress despite no display should simply be ignored
   - last_action preserved for when entries are loaded

## Testing Strategy

### Manual Testing Scenarios

1. **Series Assignment Repeat**:

   - Assign episode A to series X
   - Verify "Last action: X" appears
   - Select unassigned episode B
   - Verify [F5] appears in menu
   - Press F5
   - Verify episode B is assigned to series X
   - Verify last action still shows series X

2. **Season Assignment Repeat (Same Series)**:

   - Assign episode A to series X, season 1
   - Verify "Last action: X, Season 1" appears
   - Select episode B already in series X
   - Verify [F5] appears in menu
   - Press F5
   - Verify episode B is assigned to season 1
   - Verify episode B remains in series X

3. **Season Assignment Repeat (No Series)**:

   - Assign episode A to series X, season 1
   - Select unassigned episode C
   - Verify [F5] appears in menu
   - Press F5
   - Verify episode C is assigned to both series X and season 1

4. **Season Assignment Repeat (Different Series)**:

   - Assign episode A to series X, season 1
   - Select episode D already in series Y
   - Verify [F5] appears in menu
   - Press F5
   - Verify episode D is reassigned to series X and season 1

5. **Invalid Repeat (Already Assigned)**:

   - Assign episode A to series X
   - Select episode A again
   - Verify [F5] does NOT appear in menu
   - Verify last action display is empty

6. **Display Updates**:
   - Verify last action line appears between menu and filter
   - Verify filter line moves down appropriately
   - Verify display updates when selection changes
   - Verify display updates when last action changes

### Integration Testing

- Test with view context changes (TopLevel, Series, Season views)
- Test with search filtering active
- Test with empty entries list
- Test mode transitions (Browse → Edit → Browse)

### Boundary Conditions

- First action after application start (no last_action)
- Switching between series and season assignments
- Rapid F5 presses on multiple episodes
- F5 with non-episode selections (Series, Season entries)
