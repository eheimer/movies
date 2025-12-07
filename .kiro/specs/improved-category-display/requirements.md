# Requirements Document

**GitHub Issue:** #33

## Introduction

This feature improves the display format for series and season entries in the terminal UI to provide clearer information about episode counts and unwatched status. The current display format will be updated to show total episode counts and unwatched episode counts in a consistent, readable format.

## Glossary

- **System**: The terminal-based video file browser application
- **Series**: A collection of related episodes, potentially organized into seasons
- **Season**: An organizational unit within a series containing episodes
- **Episode**: An individual video file that may be standalone or part of a series/season
- **Unwatched Episode**: An episode where the watched status field is false or null
- **Display String**: The formatted text representation of an entry shown in the terminal UI

## Requirements

### Requirement 1

**User Story:** As a user browsing series entries, I want to see the total episode count and unwatched count in a clear format, so that I can quickly assess how much content is available and what remains to be watched.

#### Acceptance Criteria

1. WHEN the system displays a series entry THEN the system SHALL format it as "[<series title>] <x> episodes (<y> unwatched)"
2. WHEN calculating episode counts for a series THEN the system SHALL include all episodes across all seasons and standalone episodes within that series
3. WHEN calculating unwatched counts for a series THEN the system SHALL count all episodes where the watched status is false or null across all seasons and standalone episodes
4. WHEN a series has zero unwatched episodes THEN the system SHALL display "(0 unwatched)" in the format
5. WHEN a series title contains special characters THEN the system SHALL display them correctly within the brackets

### Requirement 2

**User Story:** As a user browsing season entries, I want to see the episode count and unwatched count for that specific season, so that I can understand the scope of that season without including other seasons' data.

#### Acceptance Criteria

1. WHEN the system displays a season entry THEN the system SHALL format it as "<season title> - <x> episodes (<y> unwatched)"
2. WHEN calculating episode counts for a season THEN the system SHALL include only episodes that belong to that specific season
3. WHEN calculating unwatched counts for a season THEN the system SHALL count only episodes in that season where the watched status is false or null
4. WHEN a season has zero unwatched episodes THEN the system SHALL display "(0 unwatched)" in the format
5. WHEN a season title contains special characters THEN the system SHALL display them correctly

### Requirement 3

**User Story:** As a user, I want the display format to be consistent and readable, so that I can quickly scan through my library and understand the status of my content.

#### Acceptance Criteria

1. WHEN the system renders series and season entries THEN the system SHALL use consistent spacing and punctuation in the format strings
2. WHEN episode counts are displayed THEN the system SHALL use the word "episodes" for clarity
3. WHEN unwatched counts are displayed THEN the system SHALL enclose them in parentheses with the word "unwatched"
4. WHEN the system updates the display after watched status changes THEN the system SHALL immediately reflect the new counts
5. WHEN entries are filtered or searched THEN the system SHALL maintain the correct display format for all visible entries
