# Design Document

## Overview

This feature adds a conditional save indicator to the edit mode header. When users modify episode fields, the header will display "[F2] save" to make the save functionality more discoverable. The implementation requires a simple modification to the existing header rendering logic in `src/display.rs`.

## Architecture

The change is localized to the `draw_header` function in `src/display.rs`. The function already receives an `is_dirty` parameter that tracks whether any fields have been modified. We'll use this parameter to conditionally append the save option to the Edit mode header string.

## Components and Interfaces

### Modified Component: `draw_header` function

**Current behavior:**
- Edit mode header shows: `"[↑]/[↓] change field, [ESC] cancel"`
- The `is_dirty` parameter is passed but only used for other modes

**New behavior:**
- When `is_dirty` is false: `"[↑]/[↓] change field, [ESC] cancel"`
- When `is_dirty` is true: `"[↑]/[↓] change field, [ESC] cancel, [F2] save"`

**Implementation location:**
- File: `src/display.rs`
- Function: `draw_header`
- Lines: ~230-235 (Edit mode case in match statement)

## Data Models

No data model changes required. The existing `is_dirty` boolean parameter already provides the necessary state information.

## Test Cases

### Test Case 1: Clean state header

When edit mode is entered with no modifications, the header should display only cancel option without save option.
**Validates: Requirements 1.1**

### Test Case 2: Dirty state header

When any field is modified in edit mode, the header should display both cancel and save options with save appearing after cancel.
**Validates: Requirements 1.2**

### Test Case 3: Save option format

When the save option is displayed, it should be formatted as ", [F2] save" following the cancel option.
**Validates: Requirements 1.3**

## Error Handling

No error handling changes required. The modification is purely presentational and uses existing state.

## Testing Strategy

### Manual Testing
- Enter edit mode and verify header shows only cancel option
- Modify a field and verify save option appears
- Verify the format matches: "[ESC] cancel, [F2] save"
- Test across different field types to ensure consistency

### Integration Testing
- Verify the change doesn't affect other modes
- Ensure header width calculations still work correctly
- Test with various terminal widths
