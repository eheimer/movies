# Design Document

## Overview

This design implements a restriction on the [F4] series assignment functionality to prevent it from being triggered on episodes that already have a series assigned. The solution involves modifying the keyboard event handler in Browse mode to check the episode's series assignment status before entering SeriesSelect mode, and updating the UI display logic to conditionally show the [F4] key hint based on the selected episode's state.

The design maintains backward compatibility with the [F5] repeat action functionality while ensuring that users cannot accidentally corrupt episode data by reassigning episodes to different series. This creates a clearer workflow where users must first clear series data (via issue #4 functionality) before reassigning to a new series.

## Architecture

### Affected Components

1. **handlers.rs** - `handle_browse_mode` function

   - Add conditional logic to F4 key handler
   - Check if selected entry is an Episode with no series assigned
   - Only enter SeriesSelect mode if conditions are met

2. **display.rs** - `draw_header` function

   - Modify Browse mode instruction generation
   - Conditionally include [F4] hint based on selected entry state
   - Pass additional context about selected entry to header rendering

3. **util.rs** - `can_repeat_action` function (verification only)
   - Existing logic already prevents repeating series assignments to episodes with series
   - No modifications needed, but verify behavior aligns with new F4 restrictions

### Data Flow

```
User presses F4 in Browse Mode
    ↓
handle_browse_mode receives KeyCode::F(4)
    ↓
Check: Is selected entry an Episode?
    ↓ No → Do nothing
    ↓ Yes
    ↓
Check: Does episode have series assigned? (edit_details.series.is_some())
    ↓ Yes → Do nothing
    ↓ No
    ↓
Enter SeriesSelect mode
```

## Components and Interfaces

### Modified Functions

#### 1. `handle_browse_mode` in handlers.rs

**Current Behavior:**

```rust
KeyCode::F(4) => {
    // enter series select mode
    *mode = Mode::SeriesSelect;
    *redraw = true;
}
```

**New Behavior:**

```rust
KeyCode::F(4) => {
    // Only allow series assignment for episodes without an existing series
    if let Entry::Episode { .. } = filtered_entries[*current_item] {
        if edit_details.series.is_none() {
            *mode = Mode::SeriesSelect;
            *redraw = true;
        }
    }
}
```

**Rationale:** This prevents the mode transition when the episode already has a series, effectively disabling the F4 functionality for assigned episodes.

#### 2. `draw_header` in display.rs

**Current Signature:**

```rust
fn draw_header(
    mode: &Mode,
    filter: &String,
    series_selected: bool,
    season_selected: bool,
    series_filter_active: bool,
    last_action_display: &str,
    is_dirty: bool,
) -> io::Result<()>
```

**New Signature:**

```rust
fn draw_header(
    mode: &Mode,
    filter: &String,
    series_selected: bool,
    season_selected: bool,
    series_filter_active: bool,
    last_action_display: &str,
    is_dirty: bool,
    selected_entry: Option<&Entry>,
    edit_details: &EpisodeDetail,
) -> io::Result<()>
```

**Modified Logic:**

- Add parameter to receive the currently selected entry
- Add parameter to receive episode details for series check
- In Browse mode instruction generation, conditionally include "[F4] assign to series" based on:
  - Selected entry is an Episode
  - Episode has no series assigned (edit_details.series.is_none())

**Example Implementation:**

```rust
Mode::Browse => {
    // Determine if F4 should be shown
    let show_f4 = if let Some(Entry::Episode { .. }) = selected_entry {
        edit_details.series.is_none()
    } else {
        false
    };

    // Build instruction string
    let mut line2 = "[F2] edit, [F3] toggle watched".to_string();
    if show_f4 {
        line2.push_str(", [F4] assign to series");
    }
    if show_f5 {
        line2.push_str(", [F5] Repeat action");
    }
    line2.push_str(", [CTRL][L] rescan");

    // ... rest of logic
}
```

#### 3. `draw_screen` in display.rs

**Modification:**

- Update call to `draw_header` to pass the selected entry and edit_details
- Extract selected entry: `let selected_entry = entries.get(current_item);`
- Pass to draw_header: `draw_header(..., selected_entry, edit_details)?;`

### Unchanged Components

#### `can_repeat_action` in util.rs

The existing logic already handles the restriction correctly:

```rust
LastAction::SeriesAssignment { series_id, .. } => {
    match &episode_detail.series {
        Some(series) => series.id != *series_id,
        None => true, // Unassigned episodes can be assigned
    }
}
```

This ensures [F5] repeat action only works on episodes without a series (or with a different series), which aligns with the new F4 restrictions.

## Data Models

No changes to data models are required. The design uses existing fields:

- `EpisodeDetail.series: Option<Series>` - Used to check if episode has series assigned
- `Entry` enum - Used to identify episode entries

## Error Handling

No new error conditions are introduced. The design uses simple conditional checks that cannot fail:

- Pattern matching on `Entry` enum (exhaustive)
- Checking `Option<Series>` with `is_none()` (safe)

If the user presses F4 on an episode with a series, the key press is silently ignored (no mode change, no error message). This is consistent with other disabled key behaviors in the application.

## Testing Strategy

### Manual Testing Scenarios

1. **F4 on unassigned episode**

   - Select an episode with no series
   - Verify [F4] hint appears in header
   - Press F4
   - Verify SeriesSelect mode is entered

2. **F4 on assigned episode**

   - Select an episode with a series assigned
   - Verify [F4] hint does NOT appear in header
   - Press F4
   - Verify mode remains in Browse (no change)

3. **F4 on series entry**

   - Select a series (not an episode)
   - Verify [F4] hint does NOT appear in header
   - Press F4
   - Verify mode remains in Browse (no change)

4. **F4 on season entry**

   - Select a season (not an episode)
   - Verify [F4] hint does NOT appear in header
   - Press F4
   - Verify mode remains in Browse (no change)

5. **F5 repeat action with unassigned episode**

   - Assign an episode to a series (creating a last action)
   - Select a different unassigned episode
   - Verify [F5] hint appears
   - Press F5
   - Verify episode is assigned to the same series

6. **F5 repeat action with assigned episode**

   - Have a last action from previous series assignment
   - Select an episode already assigned to a series
   - Verify [F5] hint does NOT appear
   - Press F5
   - Verify no change occurs

7. **UI consistency across view contexts**
   - Test F4 behavior in TopLevel view
   - Test F4 behavior in Series view
   - Test F4 behavior in Season view
   - Verify consistent behavior across all contexts

### Edge Cases

1. **Empty entries list** - No crash when no entries exist
2. **Rapid F4 presses** - No race conditions or mode corruption
3. **F4 during video playback** - Behavior consistent with other keys
4. **Navigation after F4 press** - Cursor position maintained when F4 is ignored

## Implementation Notes

### Code Style Consistency

- Follow existing pattern matching style in handlers.rs
- Use existing color configuration system for any new UI elements
- Maintain consistent error handling patterns (expect with descriptive messages)
- Follow existing naming conventions for variables and functions

### Performance Considerations

- The series check (`edit_details.series.is_none()`) is O(1) and has no performance impact
- No additional database queries are required
- UI rendering overhead is negligible (one conditional string concatenation)

### Future Compatibility

This design is compatible with the planned issue #4 enhancement (clearing series data):

- Once series data is cleared, the episode becomes unassigned
- F4 functionality will automatically become available again
- No additional changes needed to support the clear functionality
