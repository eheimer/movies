# Requirements Document

## Introduction

This specification addresses a bug in the video library browser where toggling the watched status of an episode (via F3 key) causes the view to reset to the top-level browse mode, losing the current navigation context (series or season view). The fix will preserve the user's current view context when toggling watched status.

## Glossary

- **Application**: The terminal-based video file browser and library manager
- **Episode**: An individual video file that can be standalone or part of a series
- **Series**: A collection of related episodes (e.g., TV shows)
- **Season**: An organizational unit within a series
- **Browse Mode**: The application mode where users navigate through series, seasons, and episodes
- **View Context**: The current navigation state indicating whether the user is viewing top-level entries, a specific series, or a specific season
- **Watched Status**: A boolean flag indicating whether an episode has been watched

## Requirements

### Requirement 1

**User Story:** As a user browsing episodes within a series, I want the watched status toggle to preserve my current series view, so that I can continue browsing episodes in the same series without losing my place.

#### Acceptance Criteria

1. WHEN the user is viewing episodes within a series AND presses F3 to toggle watched status on an episode, THEN the Application SHALL reload entries for the current series view
2. WHEN the user toggles watched status within a series view, THEN the Application SHALL maintain the series context in the entries list
3. WHEN the user toggles watched status within a series view, THEN the Application SHALL keep the user positioned at the same episode in the list

### Requirement 2

**User Story:** As a user browsing episodes within a season, I want the watched status toggle to preserve my current season view, so that I can continue browsing episodes in the same season without losing my place.

#### Acceptance Criteria

1. WHEN the user is viewing episodes within a season AND presses F3 to toggle watched status on an episode, THEN the Application SHALL reload entries for the current season view
2. WHEN the user toggles watched status within a season view, THEN the Application SHALL maintain the season context in the entries list
3. WHEN the user toggles watched status within a season view, THEN the Application SHALL keep the user positioned at the same episode in the list

### Requirement 3

**User Story:** As a user browsing top-level entries, I want the watched status toggle to maintain the top-level view, so that my browsing experience remains consistent.

#### Acceptance Criteria

1. WHEN the user is viewing top-level entries (series and standalone episodes) AND presses F3 to toggle watched status on an episode, THEN the Application SHALL reload top-level entries
2. WHEN the user toggles watched status at the top level, THEN the Application SHALL maintain the top-level context in the entries list
3. WHEN the user toggles watched status at the top level, THEN the Application SHALL keep the user positioned at the same entry in the list

### Requirement 4

**User Story:** As a developer maintaining the codebase, I want the view context to be explicitly tracked, so that the application can correctly determine which entries to reload after state changes.

#### Acceptance Criteria

1. THE Application SHALL track the current view context (top-level, series view, or season view)
2. THE Application SHALL use the tracked view context to determine the appropriate database query when reloading entries
3. THE Application SHALL update the view context whenever the user navigates between different views
