# Design Document

## Overview

This design addresses a bug in the automatic episode number entry feature when entering edit mode. The fix ensures that episode number auto-fill only occurs when a season is assigned and properly marks the field as dirty to indicate unsaved changes.

## Architecture

The fix is localized to the `execute_menu_action` function in `src/handlers.rs`, specifically within the `MenuAction::Edit` branch. This function is called when:

- User presses F2 (Edit hotkey) in Browse mode
- User selects "Edit" from the context menu (F1)

## Components and Interfaces

### Modified Component: `execute_menu_action` function

**Location:** `src/handlers.rs` (approximately line 1000-1030)

**Current Behavior:**

```rust
// Auto-fill episode number if series is assigned but episode number is not
if edit_details.series.is_some()
    && (edit_details.episode_number.is_empty() || edit_details.episode_number == "0")
{
    // Calculate and set episode number
    // Does NOT check for season assignment
    // Does NOT mark field as dirty
}
```

**New Behavior:**

```rust
// Auto-fill episode number if BOTH series AND season are assigned, and episode number is not set
if edit_details.series.is_some()
    && season_number.is_some()  // NEW: Check season is assigned
    && (edit_details.episode_number.is_empty() || edit_details.episode_number == "0")
{
    // Calculate and set episode number
    // Mark field as dirty
    dirty_fields.insert(EpisodeField::EpisodeNumber);  // NEW: Mark as dirty
}
```

### Data Flow

1. User triggers edit mode (F2 or context menu)
2. `execute_menu_action` is called with `MenuAction::Edit`
3. System retrieves episode details from database
4. System extracts season number from episode details
5. **NEW:** System checks if season is assigned (`season_number.is_some()`)
6. If season exists AND episode number is empty/zero:
   - Calculate next available episode number
   - Set episode number field
   - **NEW:** Mark EpisodeNumber field as dirty
   - Set cursor to episode number field
7. Enter Edit mode with prepared state

## Error Handling

No new error handling required. The fix uses existing safe operations:

- `season_number.is_some()` is a safe Option check
- `dirty_fields.insert()` is an infallible HashSet operation

## Testing Strategy

### Manual Testing Scenarios

1. **Episode with no series assigned:**

   - Enter edit mode
   - Verify episode number is NOT auto-filled
   - Verify cursor starts at Title field

2. **Episode with series but no season:**

   - Enter edit mode
   - Verify episode number is NOT auto-filled
   - Verify cursor starts at Title field

3. **Episode with series and season, empty episode number:**

   - Enter edit mode
   - Verify episode number IS auto-filled with next available number
   - Verify EpisodeNumber field shows as dirty (visual indicator)
   - Verify cursor is positioned at episode number field

4. **Episode with series and season, existing episode number:**

   - Enter edit mode
   - Verify episode number is NOT modified
   - Verify field is NOT marked as dirty
   - Verify cursor starts at Title field

5. **Save auto-filled episode number:**
   - Enter edit mode (triggers auto-fill)
   - Press F2 to save
   - Exit and re-enter edit mode
   - Verify episode number persists

### Edge Cases

- Episode number is "0" (treated as empty)
- Episode number is already set to a valid value (should not auto-fill)
- Multiple episodes in same season (should get sequential numbers)

## Implementation Notes

### Key Variables in Scope

- `edit_details`: EpisodeDetail struct containing episode metadata
- `season_number`: Option<usize> extracted from edit_details.season
- `dirty_fields`: HashSet<EpisodeField> tracking modified fields
- `edit_field`: Current field cursor position
- `edit_cursor_pos`: Character position within field

### Minimal Change Principle

The fix requires only two additions to existing code:

1. Add `&& season_number.is_some()` to the conditional check
2. Add `dirty_fields.insert(EpisodeField::EpisodeNumber);` after setting the value

This maintains code simplicity and minimizes risk of introducing new bugs.
