# Design Document

## Overview

This feature implements dirty record tracking in EDIT mode to provide visual feedback when fields are modified and to conditionally display the save changes menu option. The design tracks which fields have been modified, highlights their field names with configurable colors, and shows/hides the "[F2] save changes" menu item based on whether any changes have been made.

## Architecture

The implementation follows the existing architecture patterns:

1. **Configuration Extension**: Add `dirty_fg` and `dirty_bg` color settings to `Config` struct
2. **State Tracking**: Add dirty field tracking to the main application state
3. **Display Logic**: Modify `draw_detail_window` to apply dirty colors to field names
4. **Menu Rendering**: Modify `draw_header` to conditionally display save changes option
5. **Handler Updates**: Update edit mode handlers to track dirty state

## Components and Interfaces

### 1. Configuration (src/config.rs)

**Changes:**

- Add `dirty_fg: String` field to `Config` struct
- Add `dirty_bg: String` field to `Config` struct
- Update `Default` implementation to set defaults: `dirty_fg: "Black"`, `dirty_bg: "White"`

**Interface:**

```rust
pub struct Config {
    // ... existing fields ...
    pub dirty_fg: String,
    pub dirty_bg: String,
}
```

### 2. Dirty State Tracking (src/main.rs)

**New State Variables:**

- `original_edit_details: Option<EpisodeDetail>` - Stores the original values when entering EDIT mode
- `dirty_fields: HashSet<EpisodeField>` - Tracks which fields have been modified

**State Management:**

- When entering EDIT mode: Clone current `edit_details` to `original_edit_details` and clear `dirty_fields`
- When modifying a field: Compare current value with original and update `dirty_fields`
- When saving/canceling: Clear `original_edit_details` and `dirty_fields`

### 3. Display Module (src/display.rs)

**Modified Functions:**

#### `draw_header`

- Add parameter: `is_dirty: bool`
- Modify EDIT mode instructions to conditionally include "[F2] save changes" based on `is_dirty`
- Current: `"[\u{2191}]/[\u{2193}] change field, [ESC] cancel, [F2] save changes"`
- New: `"[\u{2191}]/[\u{2193}] change field, [ESC] cancel"` + conditionally append `", [F2] save changes"`

#### `draw_detail_window`

- Add parameter: `dirty_fields: &HashSet<EpisodeField>`
- Add parameter: `config: &Config`
- When rendering field names in EDIT mode:
  - Check if field is in `dirty_fields`
  - If dirty: Apply `dirty_fg` and `dirty_bg` colors to field name
  - If not dirty: Use default colors for field name

**Rendering Logic:**

```rust
// For each field line
let field_name = field.display_name();
let field_name_display = if edit_mode && dirty_fields.contains(&field) {
    format!("{}:", field_name)
        .with(string_to_fg_color_or_default(&config.dirty_fg))
        .on(string_to_bg_color_or_default(&config.dirty_bg))
} else {
    format!("{}:", field_name).stylize()
};
```

#### `draw_screen`

- Add parameters: `dirty_fields: &HashSet<EpisodeField>`, pass through to `draw_detail_window` and `draw_header`
- Calculate `is_dirty` as `!dirty_fields.is_empty()`
- Pass `is_dirty` to `draw_header`

### 4. Handler Module (src/handlers.rs)

**Modified Functions:**

#### `handle_edit_mode`

- Add parameters: `original_edit_details: &EpisodeDetail`, `dirty_fields: &mut HashSet<EpisodeField>`
- After any field modification (Char, Backspace, Delete, +, -):
  - Compare current field value with original value
  - If different: Insert field into `dirty_fields`
  - If same: Remove field from `dirty_fields`
- On F2 (save): Clear `dirty_fields` after saving
- On Esc (cancel): Clear `dirty_fields` when canceling

**Dirty Detection Logic:**

```rust
fn update_dirty_state(
    field: EpisodeField,
    current_details: &EpisodeDetail,
    original_details: &EpisodeDetail,
    dirty_fields: &mut HashSet<EpisodeField>,
    season_number: &Option<usize>,
) {
    let current_value = field.get_field_value(current_details);
    let original_value = field.get_field_value(original_details);

    // Special handling for Season field
    let is_dirty = if field == EpisodeField::Season {
        let original_season = original_details.season.as_ref().map(|s| s.number);
        season_number != &original_season
    } else {
        current_value != original_value
    };

    if is_dirty {
        dirty_fields.insert(field);
    } else {
        dirty_fields.remove(&field);
    }
}
```

#### `handle_browse_mode`

- When entering EDIT mode (F2):
  - Clone `edit_details` to `original_edit_details`
  - Clear `dirty_fields`

### 5. Main Application Loop (src/main.rs)

**State Initialization:**

```rust
let mut original_edit_details: Option<EpisodeDetail> = None;
let mut dirty_fields: HashSet<EpisodeField> = HashSet::new();
```

**Mode Transitions:**

- Browse → Edit: Set `original_edit_details = Some(edit_details.clone())`
- Edit → Browse (save/cancel): Set `original_edit_details = None`, clear `dirty_fields`

**Draw Call:**

- Pass `&dirty_fields` to `draw_screen`

## Data Models

### EpisodeDetail Comparison

The dirty state is determined by comparing field values between `edit_details` and `original_edit_details`:

- **Title**: String comparison
- **Year**: String comparison
- **Length**: String comparison (not editable, but tracked)
- **Episode Number**: String comparison
- **Season**: Compare `season_number` with `original_edit_details.season.as_ref().map(|s| s.number)`

### HashSet<EpisodeField>

Used to track dirty fields efficiently:

- O(1) insertion, removal, and lookup
- Automatically handles duplicates
- Can be easily checked for emptiness to determine overall dirty state

## Error Handling

No new error conditions are introduced. The feature uses existing error handling patterns:

- Config parsing errors fall back to defaults
- Color parsing uses existing `string_to_color` with fallback to defaults

## Testing Strategy

### Manual Testing Scenarios

1. **Configuration Loading**

   - Verify default colors are applied when config.json doesn't have dirty_fg/dirty_bg
   - Verify custom colors are loaded correctly from config.json
   - Test with invalid color names (should fall back to defaults)

2. **Dirty Field Highlighting**

   - Enter EDIT mode and modify a field
   - Verify field name changes to dirty colors
   - Modify field back to original value
   - Verify field name returns to normal colors
   - Test with multiple fields modified simultaneously

3. **Save Changes Menu**

   - Enter EDIT mode without modifications
   - Verify "[F2] save changes" is NOT displayed
   - Modify any field
   - Verify "[F2] save changes" IS displayed
   - Save changes
   - Verify menu returns to not showing save option

4. **State Persistence**

   - Modify multiple fields
   - Navigate between fields with Up/Down
   - Verify dirty highlighting persists across field navigation
   - Cancel with ESC
   - Re-enter EDIT mode
   - Verify no fields are marked dirty

5. **Special Field Handling**
   - Test Season field with +/- keys
   - Test Episode Number field with +/- keys
   - Verify dirty state updates correctly for numeric fields

### Edge Cases

1. **Empty to Empty**: Modifying an empty field and deleting back to empty should not be dirty
2. **Season Assignment**: Changing season number should mark Season field as dirty
3. **Series Context**: Dirty state should work correctly when editing episodes within series/season views
4. **Rapid Edits**: Multiple rapid edits should correctly update dirty state
5. **Non-editable Fields**: Non-editable fields should never be marked dirty

## Implementation Notes

### Color Application

- Field names are rendered as `"<name>:"` format
- Only the field name portion gets colored, not the value
- Dirty colors are applied using crossterm's `with()` and `on()` methods

### Performance Considerations

- HashSet operations are O(1) for typical use cases
- Cloning EpisodeDetail on EDIT mode entry is acceptable (small struct)
- String comparisons for dirty detection are minimal overhead

### Backward Compatibility

- Existing config.json files without dirty_fg/dirty_bg will use defaults
- No database schema changes required
- No breaking changes to existing functionality
