# Design Document

## Overview

This design addresses the episode sorting bug by modifying SQL queries in the `database.rs` module to use numeric sorting instead of string-based sorting for the `episode_number` field. The solution leverages SQLite's `CAST` function and `ORDER BY` clause ordering to achieve the desired behavior: episodes with episode numbers sorted numerically first, followed by episodes without episode numbers sorted alphabetically by title.

## Architecture

The fix is isolated to the `database.rs` module and requires no changes to other modules. Two query functions will be modified:

1. `get_entries()` - Retrieves episodes not part of any series
2. `get_entries_for_season()` - Retrieves episodes within a specific season

Note: `get_entries_for_series()` does not require modification because episodes cannot have an episode_number without also having a season_id (enforced by database constraint).

## Components and Interfaces

### Modified Functions

#### `get_entries()`

**Current Behavior:**

```sql
ORDER BY episode_number, name
```

Where `episode_number` is cast to TEXT, causing lexicographic sorting.

**New Behavior:**

```sql
ORDER BY
  CASE WHEN episode_number IS NULL THEN 1 ELSE 0 END,
  CAST(episode_number AS INTEGER),
  name
```

This three-level sort ensures:

1. Episodes with episode_number come first (0), NULL values come last (1)
2. Non-NULL episode_numbers are sorted numerically
3. Episodes with same episode_number (or all NULLs) are sorted alphabetically by name

#### `get_entries_for_season(season_id)`

**Current Behavior:**

```sql
ORDER BY episode_number, name
```

Where `episode_number` is cast to TEXT.

**New Behavior:**

```sql
ORDER BY
  CASE WHEN episode_number IS NULL THEN 1 ELSE 0 END,
  CAST(episode_number AS INTEGER),
  name
```

Same three-level sort as `get_entries()`.

### No Interface Changes

The function signatures remain unchanged. The Entry enum and all calling code remain unaffected since this is purely a query-level modification.

## Data Models

No changes to data models. The `episode_number` field remains an optional INTEGER in the database schema.

## Error Handling

No new error conditions are introduced. SQLite's `CAST` function handles NULL values gracefully, and the `CASE` expression explicitly manages NULL sorting behavior.

## Testing Strategy

### Manual Testing Approach

1. **Setup Test Data**: Create episodes with various episode numbers (1, 2, 3, 10, 11, 20) and some without episode numbers
2. **Test Standalone Episodes**: Browse episodes not in a series and verify numeric ordering
3. **Test Season Episodes**: Browse episodes in a season and verify numeric ordering
4. **Test NULL Handling**: Verify episodes without episode_numbers appear after numbered episodes and are sorted by title
5. **Edge Cases**: Test with single-digit, double-digit, and triple-digit episode numbers

### Expected Results

- Episodes should display in order: 1, 2, 3, 4, 10, 11, 20
- Episodes without numbers should appear after numbered episodes
- Alphabetical sorting by title should work for episodes without numbers
