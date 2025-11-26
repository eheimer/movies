# Design Document

## Overview

This design reorganizes the menu helper display system to consolidate all menu helpers onto a single line in the header. The [F1] Menu helper will be moved to the front of the first line, the rescan functionality will be moved from [CTRL][L] to [S] in the context menu, and all support for a second line of menu helpers will be removed. Items that don't fit on the first line will automatically overflow into the context menu.

## Architecture

The changes will primarily affect three modules:

1. **menu.rs** - Update menu item definitions and add overflow logic
2. **display.rs** - Simplify header rendering to single line
3. **handlers.rs** - Update hotkey handling for rescan

### Current State

Currently, the system displays menu helpers across two lines in the header:
- Line 1: Context-specific instructions (e.g., "[/] filter, [↑]/[↓] navigate...")
- Line 2: "[F1] Menu, [CTRL][L] rescan"

The rescan action is defined as a FirstLine menu item with CTRL+L hotkey handling in browse mode.

### Target State

After reorganization:
- Line 1: "[F1] Menu" followed by FirstLine-preferred menu helpers that fit within terminal width
- Line 2: Removed entirely
- Rescan: Moved to ContextMenu-only with [S] hotkey
- Menu items will have three display behaviors:
  1. **ContextMenu-only**: Never displayed on first line (e.g., rescan, edit, toggle watched)
  2. **FirstLine-preferred**: Displayed on first line if space permits, overflow to context menu if not
  3. **Always visible**: Must always appear on first line (e.g., [F1] Menu)

## Components and Interfaces

### 1. Menu System (menu.rs)

#### Changes to MenuLocation Enum

Add a new variant to support FirstLine-preferred items:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum MenuLocation {
    FirstLine,        // Always visible on first line (e.g., [F1] Menu)
    FirstLinePreferred, // Prefer first line, overflow to context menu if needed
    ContextMenu,      // Only visible in F1 context menu
}
```

#### Changes to MenuItem Definition

The rescan menu item will be updated:
- Change `location` from `MenuLocation::FirstLine` to `MenuLocation::ContextMenu`
- Change `hotkey` from `KeyCode::Char('l')` to `KeyCode::Char('s')`

Currently, rescan is the only FirstLine item. After this change, there will be no FirstLine items defined in `define_all_menu_items()`. The [F1] Menu helper is hardcoded in the display logic, not defined as a menu item.

#### New Function: get_first_line_preferred_items

```rust
pub fn get_first_line_preferred_items(context: &MenuContext) -> Vec<MenuItem>
```

This function will:
1. Get all **available** menu items (filtered by `is_item_available()` based on context)
2. Filter to only those with `MenuLocation::FirstLinePreferred`
3. Return the filtered list **in the order they were defined**

**Important**: 
- The order of items in `define_all_menu_items()` determines their priority for first-line display
- Items defined earlier have higher priority and will be displayed first (less likely to overflow)
- **Availability checks still apply**: Items are only included if `is_item_available()` returns true for the current context
- This means FirstLinePreferred items may not appear at all if the context doesn't support them

#### New Function: calculate_menu_helper_width

```rust
fn calculate_menu_helper_width(item: &MenuItem) -> usize
```

This helper function calculates the display width for a menu item:
- Opening bracket: 1 char
- Hotkey text (e.g., "F2", "S"): variable
- Closing bracket: 1 char
- Space: 1 char
- Label text: variable
- Separator: 2 chars (", ")

Example: "[S] rescan, " = 12 chars

### 2. Display Module (display.rs)

#### Changes to draw_header Function

The `draw_header` function will be significantly modified:

1. **Remove second line logic**: All code that generates and displays the second line of menu helpers will be removed
2. **Single line rendering**: Only one instruction line will be rendered
3. **Menu helper integration**: The first line will include "[F1] Menu" at the start, followed by FirstLine-preferred helpers that fit
4. **Overflow calculation**: Determine which FirstLine-preferred items fit within terminal width

#### New Header Layout

```
[F1] Menu, [hardcoded helpers], [FirstLinePreferred items that fit]
<last action display line>
<empty line>
filter: <search text>
```

Example:
```
[F1] Menu, [/] filter, [↑]/[↓] navigate, [ENTER] play, [ESC] exit
```

#### Header Building Algorithm

The `draw_header` function will:

1. Get terminal width
2. Start with "[F1] Menu, " (fixed - always visible)
3. Build hardcoded context-specific helpers string based on mode/context
4. Calculate width used by "[F1] Menu, " + hardcoded helpers
5. Calculate remaining width available for FirstLine-preferred items
6. Get FirstLine-preferred menu items from menu system (in priority order, filtered by availability)
7. Iterate through FirstLine-preferred items in order:
   - Calculate width needed for this item
   - If it fits in remaining width, add it to the display string
   - If it doesn't fit, stop adding items (remaining items overflow to context menu)
8. Build final instruction string: "[F1] Menu, " + hardcoded helpers + FirstLine-preferred items
9. Print single line

**Display Order**: `[F1] Menu, [hardcoded helpers], [FirstLinePreferred items that fit]`

**Priority Order**: FirstLine-preferred items are processed in the order they appear in `define_all_menu_items()`. The first items in the list have the highest priority and will be displayed before later items.

#### Context-Specific Helpers (Hardcoded)

The existing context-specific helpers will remain hardcoded and integrated into the single line:
- These are NOT menu items, they're hardcoded display strings based on mode/context
- They appear AFTER "[F1] Menu" and BEFORE any FirstLine-preferred items
- They NEVER overflow to the context menu (always displayed)
- They change based on current mode and context
- Examples: "[/] filter", "[↑]/[↓] navigate", "[ENTER] play", "[ESC] exit"
- They are context-aware and may not appear in certain states (e.g., navigation helpers don't appear in filter mode)
- Their width is calculated first to determine how much space remains for FirstLine-preferred items

#### Removed Code

- All logic that builds a second line of instructions
- The loop that prints multiple instruction lines (will be simplified to single line)
- Special handling for CTRL+L display
- The `instructions` vector will be replaced with a single string

### 3. Handler Module (handlers.rs)

#### Changes to handle_browse_mode

Remove the CTRL+L hotkey handling:

```rust
// REMOVE THIS BLOCK:
KeyCode::Char('l') if modifiers.contains(event::KeyModifiers::CONTROL) && !*filter_mode => {
    // clear entries and filtered entries
    *entries = Vec::new();
    *filtered_entries = Vec::new();
    *mode = Mode::Entry;
    search.clear();
    *redraw = true;
}
```

The rescan functionality will now be handled through the menu system with the [S] hotkey, which is already supported by the existing menu hotkey handling logic.

#### Menu Hotkey Handling

**Important**: Hotkeys work regardless of menu item location or visibility. A user can press a hotkey in browse mode to execute an action directly, without opening the F1 menu first.

The existing code already checks for menu item hotkeys in browse mode:

```rust
// Check if the pressed key matches any available menu item hotkey
for item in &menu_items {
    if let Some(hotkey) = &item.hotkey {
        if *hotkey == code {
            // Execute the menu action directly
            execute_menu_action(...);
            return Ok(true);
        }
    }
}
```

This means:
- [S] will trigger rescan from browse mode (without opening F1 menu)
- [F2] will trigger edit from browse mode (without opening F1 menu)
- All other hotkeys work the same way

**No changes are needed to this hotkey handling logic**. It already works for all menu items regardless of their `MenuLocation`.

## Data Models

No changes to data models are required. The existing `MenuItem`, `MenuAction`, and `MenuLocation` structures are sufficient.

## Error Handling

No new error handling is required. The existing error handling patterns will be maintained:
- Terminal width calculation failures will fall back to conservative estimates
- Menu item rendering will gracefully handle edge cases (empty lists, narrow terminals)

## Testing Strategy

### Manual Testing

1. **Menu Helper Display**
   - Verify [F1] Menu appears at the front of line 1
   - Verify no second line of menu helpers is displayed
   - Test in different terminal widths to ensure proper layout

2. **Rescan Functionality**
   - Verify CTRL+L no longer triggers rescan
   - Verify [S] key triggers rescan in browse mode (without opening F1 menu)
   - Verify rescan appears in F1 context menu with [S] hotkey
   - Verify rescan can be triggered from within the F1 menu by pressing [S] or [ENTER]

3. **Hotkey Functionality**
   - Verify all menu item hotkeys work in browse mode without opening F1 menu
   - Verify hotkeys work regardless of whether item is displayed on first line or only in context menu
   - Test FirstLinePreferred items that overflow: verify their hotkeys still work

4. **Overflow Behavior**
   - Test in narrow terminal windows
   - Verify items that don't fit are accessible via F1 menu
   - Verify [F1] Menu helper never overflows

5. **Context-Specific Behavior**
   - Test in different modes (Browse, Edit, SeriesSelect, etc.)
   - Test in different view contexts (TopLevel, Series, Season)
   - Verify appropriate menu helpers are displayed

### Edge Cases

1. **Very narrow terminals**: Ensure at minimum "[F1] Menu" is always visible
2. **Filter mode**: Verify simplified display still works correctly
3. **Empty entry lists**: Verify header renders correctly
4. **Mode transitions**: Verify header updates correctly when switching modes

## Implementation Notes

### Phase 1: Update Menu Definition
- Modify rescan menu item in `define_all_menu_items()`
- Change location to ContextMenu
- Change hotkey to 's'

### Phase 2: Remove CTRL+L Handler
- Remove CTRL+L handling from `handle_browse_mode()`
- Verify menu system handles [S] hotkey automatically

### Phase 3: Simplify Header Display
- Modify `draw_header()` to build single line
- Remove second line rendering logic
- Add "[F1] Menu" prefix to first line
- Remove CTRL+L display logic

### Phase 4: Testing
- Test all modes and contexts
- Test different terminal widths
- Verify rescan works with [S] hotkey
- Verify no regressions in other functionality

## Design Decisions and Rationales

### Decision 1: [S] for Rescan
**Rationale**: The letter 'S' is mnemonic for "scan" and is not currently used as a hotkey in browse mode. It's easier to type than CTRL+L and fits the pattern of single-key hotkeys used by other menu actions.

### Decision 2: Always Show [F1] Menu First
**Rationale**: The F1 menu is the primary access point for all actions. Placing it first ensures users can always find it, and it serves as a visual anchor for the menu system.

### Decision 3: Remove Second Line Entirely
**Rationale**: The second line was only used for two items ([F1] Menu and [CTRL][L] rescan). With rescan moved to the context menu, the second line serves no purpose and removing it simplifies the UI and code.

### Decision 4: Automatic Overflow to Context Menu
**Rationale**: This provides a graceful degradation for narrow terminals while keeping the most important actions visible. Users can always access overflow items via F1.

### Decision 5: Three-Tier Menu Location System
**Rationale**: This provides developers with fine-grained control over menu item visibility:
- **ContextMenu**: For actions that are less frequently used or would clutter the first line
- **FirstLinePreferred**: For actions that are useful to see but not critical
- **FirstLine**: For critical items that must always be visible (currently only [F1] Menu)

This approach balances discoverability with UI cleanliness.

### Decision 6: Context-Specific Helpers Remain Hardcoded
**Rationale**: Navigation helpers like "[/] filter" and "[↑]/[↓] navigate" are mode-specific and not actions that can be executed via the menu system. They remain as hardcoded display strings that change based on the current mode and context.
