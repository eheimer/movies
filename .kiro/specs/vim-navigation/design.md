# Design Document

## Overview

This design adds vim-style keyboard navigation to the video browser application by extending the existing keyboard event handling in Browse mode. The implementation will add support for `k` (up) and `j` (down) keys to work alongside the existing arrow key navigation, but only when not in filter mode.

## Architecture

The feature will be implemented entirely within the existing event handling architecture:

- **Event Handler Layer**: Modify `handle_browse_mode()` in `src/handlers.rs` to recognize `k` and `j` key presses
- **Mode-Based Filtering**: Leverage the existing `filter_mode` flag to conditionally enable vim navigation
- **Navigation Logic Reuse**: Use the same navigation logic as arrow keys to ensure consistent behavior

## Components and Interfaces

### Modified Components

#### `src/handlers.rs::handle_browse_mode()`

This function already handles keyboard events in Browse mode. We will add new match arms for vim keys:

**Current Structure:**
- Takes `KeyCode` as input
- Checks `filter_mode` flag to determine if filtering is active
- Handles arrow keys (`KeyCode::Up`, `KeyCode::Down`) with `!*filter_mode` guard
- Updates `current_item` and `first_entry` for scrolling

**Modifications:**
- Add `KeyCode::Char('k')` match arm with `!*filter_mode` guard
- Add `KeyCode::Char('j')` match arm with `!*filter_mode` guard
- Both will execute the same logic as their arrow key equivalents

### Key Behavior Mapping

| Vim Key | Arrow Key Equivalent | Action |
|---------|---------------------|--------|
| `k` | `Up` | Move selection cursor up by one |
| `j` | `Down` | Move selection cursor down by one |

### Filter Mode Integration

The existing `filter_mode` boolean flag controls whether the application is in filtering mode:

- **When `filter_mode == false`**: Vim keys trigger navigation
- **When `filter_mode == true`**: Vim keys are treated as regular character input for filtering

This is already implemented for arrow keys using the pattern:
```rust
KeyCode::Up if !*filter_mode => { /* navigation logic */ }
```

We will use the same pattern for vim keys.

## Data Models

No new data models are required. The feature uses existing state variables:

- `current_item: usize` - Index of currently selected item
- `first_entry: usize` - Index of first visible item (for scrolling)
- `filter_mode: bool` - Whether filter mode is active
- `redraw: bool` - Triggers UI refresh

## Error Handling

No new error handling is required. The feature reuses existing navigation logic which already handles:

- Boundary conditions (top/bottom of list)
- Empty lists
- Scrolling behavior

The vim key handlers will inherit this error handling by executing the same code paths as arrow keys.

## Testing Strategy

### Manual Testing Scenarios

1. **Basic Navigation in Browse Mode**
   - Start application in Browse mode
   - Press `k` repeatedly - cursor should move up
   - Press `j` repeatedly - cursor should move down
   - Verify behavior matches arrow keys

2. **Boundary Conditions**
   - Navigate to top of list, press `k` - cursor should stay at top
   - Navigate to bottom of list, press `j` - cursor should stay at bottom

3. **Filter Mode Exclusion**
   - Enter filter mode with `/`
   - Type `k` and `j` - they should appear in the filter text
   - Verify they do NOT trigger navigation
   - Exit filter mode with Enter or Esc
   - Press `k` and `j` - navigation should work again

4. **Mode Isolation**
   - Enter Edit mode (F2) - `k` and `j` should not navigate
   - Enter Menu mode (F1) - `k` and `j` should not navigate
   - Enter SeriesSelect mode - `k` and `j` should not navigate

5. **Scrolling Behavior**
   - With a long list, navigate with `j` past visible area
   - Verify scrolling matches arrow key behavior
   - Navigate with `k` back up
   - Verify scrolling matches arrow key behavior

6. **Mixed Navigation**
   - Use arrow keys and vim keys interchangeably
   - Verify seamless transition between input methods

### Integration Testing

- Verify vim navigation works correctly in all view contexts:
  - TopLevel view (all series and standalone episodes)
  - Series view (seasons and episodes within a series)
  - Season view (episodes within a season)

## Implementation Notes

### Code Location

All changes will be in `src/handlers.rs`, specifically in the `handle_browse_mode()` function.

### Implementation Pattern

The implementation will follow this pattern for each vim key:

```rust
KeyCode::Char('k') if !*filter_mode => {
    // Execute same logic as KeyCode::Up
    if *current_item > 0 {
        *current_item -= 1;
        if *current_item < *first_entry {
            *first_entry = *current_item;
        }
        *redraw = true;
    }
}

KeyCode::Char('j') if !*filter_mode => {
    // Execute same logic as KeyCode::Down
    if *current_item < filtered_entries.len() - 1 {
        *current_item += 1;
        *redraw = true;
    }
}
```

### Why Not Extract to a Function?

While the navigation logic is duplicated, extracting it to a separate function would add complexity for minimal benefit:
- The logic is only 4-5 lines per direction
- It's already duplicated between arrow keys and will be duplicated for vim keys
- Keeping it inline maintains consistency with the existing codebase style
- The match statement pattern is idiomatic Rust for event handling

### No UI Changes

Per requirements, no menu helpers or visual indicators will be added. The feature is "invisible" - users who know vim keys can use them, others continue using arrow keys.

## Performance Considerations

- **Zero Performance Impact**: The feature adds two additional match arms in the event handler, which has negligible performance impact
- **No Additional Allocations**: No new data structures or memory allocations
- **Same Code Path**: Vim keys execute the same logic as arrow keys, so performance characteristics are identical

## Compatibility

- **Backward Compatible**: Existing functionality is unchanged
- **No Breaking Changes**: All existing keyboard shortcuts continue to work
- **Additive Feature**: Only adds new key bindings, doesn't modify existing ones

## Future Enhancements

Potential future enhancements (not part of this spec):
- Add `gg` for jump to top
- Add `G` for jump to bottom
- Add `Ctrl+d` / `Ctrl+u` for half-page scrolling
- Add number prefixes (e.g., `5j` to move down 5 items)

These are not included in the current design to keep the implementation minimal and focused.
