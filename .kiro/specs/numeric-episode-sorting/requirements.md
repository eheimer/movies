# Requirements Document

## Introduction

This specification addresses a bug in the episode sorting logic where episodes are currently sorted lexicographically (as strings) rather than numerically. This causes incorrect ordering such as "1, 10, 2, 3, 4" instead of the expected "1, 2, 3, 4, 10". The fix will ensure episodes with episode numbers are sorted numerically first, followed by episodes without episode numbers sorted alphabetically by title.

## Glossary

- **Episode**: A video file entry in the database that may optionally have an episode_number field
- **Episode Number**: An optional integer field in the episode table that indicates the sequential position of an episode within a season or series
- **Database Module**: The `database.rs` module containing SQLite query functions
- **Lexicographic Sorting**: String-based alphabetical sorting where "10" comes before "2"
- **Numeric Sorting**: Integer-based sorting where 2 comes before 10

## Requirements

### Requirement 1

**User Story:** As a user browsing episodes, I want episodes with episode numbers to be sorted numerically, so that I see them in the correct sequential order (1, 2, 3, 10 instead of 1, 10, 2, 3).

#### Acceptance Criteria

1. WHEN the Database Module retrieves episodes without a series association, THE Database Module SHALL sort episodes with episode_number values numerically in ascending order
2. WHEN the Database Module retrieves episodes for a specific season, THE Database Module SHALL sort episodes with episode_number values numerically in ascending order
3. WHEN the Database Module retrieves episodes for a series without a season, THE Database Module SHALL sort episodes with episode_number values numerically in ascending order

### Requirement 2

**User Story:** As a user browsing episodes without episode numbers, I want those episodes to appear after numbered episodes and be sorted alphabetically by title, so that I can find unnumbered content easily.

#### Acceptance Criteria

1. WHEN the Database Module retrieves episodes, THE Database Module SHALL place episodes without episode_number values after episodes with episode_number values
2. WHEN the Database Module retrieves episodes without episode_number values, THE Database Module SHALL sort those episodes alphabetically by title in ascending order
3. WHEN the Database Module sorts episodes, THE Database Module SHALL treat NULL episode_number values as having no numeric position

### Requirement 3

**User Story:** As a developer, I want the sorting logic to be implemented at the database query level, so that sorting is efficient and consistent across all query functions.

#### Acceptance Criteria

1. THE Database Module SHALL implement numeric sorting using SQL ORDER BY clauses with CAST operations on the episode_number field
2. THE Database Module SHALL apply the numeric sorting logic to the get_entries function
3. THE Database Module SHALL apply the numeric sorting logic to the get_entries_for_series function
4. THE Database Module SHALL apply the numeric sorting logic to the get_entries_for_season function
5. THE Database Module SHALL maintain existing secondary sort criteria (name, year) for episodes with identical or NULL episode numbers
