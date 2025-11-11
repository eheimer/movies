# Design Document

## Overview

This design addresses the removal of unused code that generates compiler warnings during `cargo build`. The cleanup will be performed systematically, analyzing each warning to determine if the code is truly unused or if it represents functionality that should be preserved for future use.

Based on the current warnings, we have identified:

- 1 unused variable assignment (`entry_path`)
- 2 unused functions (`import_episode`, `pad_string_as_number`)
- 2 unused methods (`get_root_dir`, `get_executable_dir`)
- 2 unused struct fields (`series_id` in Entry::Season, `episode_number` in Entry::Episode)

## Architecture

The cleanup will follow a conservative approach:

1. Analyze each warning to understand the context
2. Verify the code is not used anywhere in the codebase
3. Remove all truly unused code
4. Remove obsolete code comments and debug logging that no longer provide value
5. Validate that all tests pass after removal

## Components and Interfaces

### 1. Unused Variable Assignment: `entry_path`

**Location**: `src/main.rs`

**Analysis**: The variable `entry_path` is declared and assigned but the assignment is never read. However, the variable itself IS used in the Entry mode for user input. The issue is that there's an initial assignment that gets overwritten.

**Solution**: Remove the initial assignment on line 33 since it's immediately overwritten on line 62 when entries are empty.

### 2. Unused Function: `import_episode`

**Location**: `src/database.rs` (line 110)

**Analysis**: This function imports episodes using absolute paths. The codebase has migrated to using `import_episode_relative` which handles path resolution properly. The old function is no longer called.

**Solution**: Remove the `import_episode` function entirely as it's superseded by `import_episode_relative`.

### 3. Unused Methods: `get_root_dir` and `get_executable_dir`

**Location**: `src/path_resolver.rs` (lines 211 and 219)

**Analysis**: These are public getter methods on the `PathResolver` struct. They are not currently used in the codebase.

**Solution**: Remove these unused methods. They can be easily added back if needed in the future.

### 4. Unused Function: `pad_string_as_number`

**Location**: `src/util.rs` (line 74)

**Analysis**: This function was intended for sorting episode numbers but is not currently used. The TODO comment suggests it might be integrated into `get_all_entries` for sorting, but this has not been implemented.

**Solution**: Remove the function and its associated TODO comment. If episode number padding is needed in the future, it can be reimplemented when the sorting logic is added.

### 5. Unused Struct Field: `series_id` in Entry::Season

**Location**: `src/util.rs` (Entry enum, Season variant)

**Analysis**: The `series_id` field in the Season variant is populated when creating Season entries but is never read. Looking at the code, seasons are always queried in the context of a specific series, so the series_id is redundant.

**Solution**: Remove the `series_id` field from the Entry::Season variant. Update all locations where Season entries are created to remove this field.

### 6. Unused Struct Field: `episode_number` in Entry::Episode

**Location**: `src/util.rs` (Entry enum, Episode variant)

**Analysis**: The `episode_number` field is populated from the database but never displayed or used in the UI. Episode numbers are only relevant in the context of editing episode details, not in the browse list.

**Solution**: Remove the `episode_number` field from the Entry::Episode variant. Update all locations where Episode entries are created to remove this field.

## Data Models

### Modified Entry Enum

```rust
pub enum Entry {
    Series {
        series_id: usize,
        name: String,
    },
    Season {
        season_id: usize,
        number: usize,
        // series_id removed
    },
    Episode {
        episode_id: usize,
        name: String,
        location: String,
        // episode_number removed
    },
}
```

## Error Handling

No new error handling is required. The changes are purely removals of unused code. All existing error handling remains intact.

## Testing Strategy

1. Compile the code after each change to ensure no new errors are introduced
2. Run `cargo test` to verify all 5 existing tests pass (located in path_resolver.rs)
3. Verify that `cargo build` produces zero warnings related to unused code

## Obsolete Comments and Debug Logging

### TODO Comments

**Location**: `src/util.rs` (line 72-74)

**Analysis**: The TODO comment above `pad_string_as_number` references moving the function or integrating it into sorting logic. Since the function itself is unused and will be removed, this TODO is obsolete.

**Solution**: Remove the TODO comment along with the function.

### Debug Logging

**Analysis**: After reviewing the codebase, the existing `eprintln!` statements serve legitimate purposes:

- Error reporting for config file issues
- Panic handler output
- User-facing warnings about skipped files
- Error messages for path resolution failures

**Solution**: No debug logging needs to be removed. All existing logging provides value for error handling and user feedback.

## Implementation Order

1. Remove unused struct fields first (Entry::Season.series_id, Entry::Episode.episode_number)
   - Update database.rs query functions
   - Update handlers.rs usage
2. Remove unused variable assignment (entry_path initial assignment)
3. Remove unused functions and their TODO comments (import_episode, pad_string_as_number with TODO)
4. Remove unused methods (get_root_dir, get_executable_dir)
5. Final verification with cargo build and cargo test

## Risk Assessment

**Low Risk Changes**:

- Removing `import_episode` (superseded by newer function)
- Removing `pad_string_as_number` (never integrated)
- Removing unused struct fields (never read)
- Removing unused variable assignment (immediately overwritten)

**Medium Risk Changes**:

- Removing `get_root_dir` and `get_executable_dir` (public API methods, but unused)

All changes are reversible through git history if needed.
