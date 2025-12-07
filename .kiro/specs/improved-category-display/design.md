# Design Document

## Overview

This feature enhances the display formatting for series and season entries in the terminal UI by adding episode count and unwatched count information. The current implementation displays series as `[<name>]` and seasons as `Season <number>`. The new implementation will display series as `[<series title>] <x> episodes (<y> unwatched)` and seasons as `<season title> - <x> episodes (<y> unwatched)`.

## Architecture

The changes will be localized to the display rendering logic in `src/display.rs` and will require new database query functions in `src/database.rs` to efficiently retrieve episode counts.

### Component Interaction

```
draw_screen (display.rs)
    ↓
Entry::Series or Entry::Season
    ↓
format_series_display() or format_season_display() [new functions]
    ↓
get_series_episode_counts() or get_season_episode_counts() [new database functions]
    ↓
Formatted display string
```

## Components and Interfaces

### Display Module (`src/display.rs`)

**New Functions:**

```rust
/// Format a series entry with episode counts
/// 
/// # Arguments
/// * `name` - The series name
/// * `series_id` - The series ID for querying counts
/// 
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Formatted string or error
fn format_series_display(name: &str, series_id: usize) -> Result<String, Box<dyn std::error::Error>>;

/// Format a season entry with episode counts
/// 
/// # Arguments
/// * `number` - The season number
/// * `season_id` - The season ID for querying counts
/// 
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Formatted string or error
fn format_season_display(number: usize, season_id: usize) -> Result<String, Box<dyn std::error::Error>>;
```

**Modified Functions:**

The `draw_screen` function will be updated to call the new formatting functions when rendering Series and Season entries.

### Database Module (`src/database.rs`)

**New Functions:**

```rust
/// Get episode counts for a series
/// 
/// # Arguments
/// * `series_id` - The series ID
/// 
/// # Returns
/// * `Result<(usize, usize), Box<dyn std::error::Error>>` - (total_episodes, unwatched_episodes) or error
pub fn get_series_episode_counts(series_id: usize) -> Result<(usize, usize), Box<dyn std::error::Error>>;

/// Get episode counts for a season
/// 
/// # Arguments
/// * `season_id` - The season ID
/// 
/// # Returns
/// * `Result<(usize, usize), Box<dyn std::error::Error>>` - (total_episodes, unwatched_episodes) or error
pub fn get_season_episode_counts(season_id: usize) -> Result<(usize, usize), Box<dyn std::error::Error>>;
```

## Data Models

No changes to existing data models are required. The implementation will use existing `Entry`, `Series`, and `Season` types.

## Test Cases

### Test Case 1: Series display format

When rendering a series entry, the display string should follow the format "[<series title>] <x> episodes (<y> unwatched)".
**Validates: Requirements 1.1**

### Test Case 2: Series episode count accuracy

When calculating episode counts for a series, the total should include all episodes across all seasons and standalone episodes within that series.
**Validates: Requirements 1.2**

### Test Case 3: Series unwatched count accuracy

When calculating unwatched counts for a series, the count should include all episodes where watched status is false or null across all seasons and standalone episodes.
**Validates: Requirements 1.3**

### Test Case 4: Series with zero unwatched episodes

When a series has all episodes watched, the display should show "(0 unwatched)".
**Validates: Requirements 1.4**

### Test Case 5: Series with special characters

When a series title contains special characters (brackets, quotes, unicode), they should display correctly within the format.
**Validates: Requirements 1.5**

### Test Case 6: Season display format

When rendering a season entry, the display string should follow the format "<season title> - <x> episodes (<y> unwatched)".
**Validates: Requirements 2.1**

### Test Case 7: Season episode count accuracy

When calculating episode counts for a season, the total should include only episodes that belong to that specific season.
**Validates: Requirements 2.2**

### Test Case 8: Season unwatched count accuracy

When calculating unwatched counts for a season, the count should include only episodes in that season where watched status is false or null.
**Validates: Requirements 2.3**

### Test Case 9: Season with zero unwatched episodes

When a season has all episodes watched, the display should show "(0 unwatched)".
**Validates: Requirements 2.4**

### Test Case 10: Season with special characters

When a season title contains special characters, they should display correctly within the format.
**Validates: Requirements 2.5**

### Test Case 11: Display format consistency

When rendering multiple series and season entries, the spacing and punctuation should be consistent across all entries.
**Validates: Requirements 3.1, 3.2, 3.3**

### Test Case 12: Dynamic count updates

When an episode's watched status changes, the next render should reflect the updated counts immediately.
**Validates: Requirements 3.4**

### Test Case 13: Filtered view count accuracy

When entries are filtered or searched, the displayed counts should remain accurate for visible entries.
**Validates: Requirements 3.5**

## Error Handling

### Database Query Errors

If database queries for episode counts fail:
- Log the error
- Display the entry with a fallback format (e.g., "[<series title>] ? episodes")
- Continue rendering other entries

### Edge Cases

1. **Empty series/seasons**: Display "0 episodes (0 unwatched)"
2. **Database connection issues**: Use fallback display format
3. **Null/missing data**: Treat as unwatched (false)

## Testing Strategy

### Unit Testing

1. **Database count functions**:
   - Test `get_series_episode_counts` with various series configurations
   - Test `get_season_episode_counts` with various season configurations
   - Test with empty series/seasons
   - Test with all watched/unwatched episodes
   - Test with mixed watched status

2. **Display formatting functions**:
   - Test `format_series_display` with different count combinations
   - Test `format_season_display` with different count combinations
   - Test with special characters in titles
   - Test with very long titles (truncation)

### Integration Testing

1. **End-to-end display**:
   - Create test database with series, seasons, and episodes
   - Verify display strings match expected format
   - Toggle watched status and verify counts update
   - Test filtering and search maintain correct counts

### Edge Case Testing

1. Test with series containing no episodes
2. Test with seasons containing no episodes
3. Test with all episodes watched
4. Test with all episodes unwatched
5. Test with series titles containing brackets, quotes, unicode
6. Test with very large episode counts (100+)

## Implementation Notes

### Performance Considerations

- Database queries for counts should be efficient (use COUNT aggregation)
- Consider caching counts if rendering performance becomes an issue
- Queries should use indexes on series_id and season_id

### SQL Query Design

**Series episode counts:**
```sql
SELECT 
    COUNT(*) as total,
    SUM(CASE WHEN watched = 0 OR watched IS NULL THEN 1 ELSE 0 END) as unwatched
FROM episode
WHERE series_id = ?
```

**Season episode counts:**
```sql
SELECT 
    COUNT(*) as total,
    SUM(CASE WHEN watched = 0 OR watched IS NULL THEN 1 ELSE 0 END) as unwatched
FROM episode
WHERE season_id = ?
```

### Display Truncation

The existing `truncate_string` function will handle long display strings. The format should be applied before truncation to ensure consistent behavior.

### Color Application

The existing color application logic for series and season entries will remain unchanged. Colors are applied after the display string is formatted.
