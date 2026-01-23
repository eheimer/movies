# Requirements Document

**GitHub Issue:** #60

## Introduction

This feature enhances the user experience by automatically positioning the cursor on the first unwatched episode when entering a new browsing context. This eliminates the need for users to manually search for their next unwatched content, making the application more intuitive for sequential viewing workflows.

## Glossary

- **Browse_Mode**: The application state where users navigate through series, seasons, and episodes
- **View_Context**: The current browsing scope (TopLevel, Series, or Season)
- **Unwatched_Episode**: An episode with watched status set to false
- **Selected_Index**: The cursor position in the current entry list
- **Context_Entry**: The action of navigating into a new view context (entering a series, entering a season, or returning to top level)

## Requirements

### Requirement 1: Auto-select on Series Entry

**User Story:** As a user, I want the cursor to automatically position on the first unwatched episode when I enter a series, so that I can quickly continue watching from where I left off.

#### Acceptance Criteria

1. WHEN a user enters a series view from the top level, THE System SHALL set the selected index to the first unwatched episode if one exists
2. WHEN a user enters a series view and all episodes are watched, THE System SHALL set the selected index to the first entry
3. WHEN a user enters a series view and no episodes exist, THE System SHALL handle the empty state gracefully

### Requirement 2: Auto-select on Season Entry

**User Story:** As a user, I want the cursor to automatically position on the first unwatched episode when I enter a season, so that I don't have to manually find my next episode.

#### Acceptance Criteria

1. WHEN a user enters a season view from a series, THE System SHALL set the selected index to the first unwatched episode if one exists
2. WHEN a user enters a season view and all episodes are watched, THE System SHALL set the selected index to the first entry
3. WHEN a user enters a season view and no episodes exist, THE System SHALL handle the empty state gracefully

### Requirement 3: Auto-select on Top Level Entry

**User Story:** As a user, I want the cursor to automatically position on the first series with unwatched content when I return to the top level, so that I can easily find content to watch.

#### Acceptance Criteria

1. WHEN a user returns to the top level view, THE System SHALL set the selected index to the first series containing unwatched episodes if one exists
2. WHEN a user returns to the top level view and all content is watched, THE System SHALL set the selected index to the first entry
3. WHEN a user returns to the top level view and no entries exist, THE System SHALL handle the empty state gracefully

### Requirement 4: Preserve Manual Selection

**User Story:** As a user, I want my manual cursor movements to be preserved within the current context, so that auto-selection doesn't interfere with my browsing.

#### Acceptance Criteria

1. WHEN a user manually moves the cursor within the current view context, THE System SHALL NOT automatically reposition the cursor
2. WHEN a user navigates back and forth within the same context, THE System SHALL remember the last selected position
3. WHEN a user enters a new context, THE System SHALL apply auto-selection logic regardless of previous position

### Requirement 5: Episode Ordering

**User Story:** As a user, I want unwatched episodes to be identified based on their natural ordering, so that I watch content in the intended sequence.

#### Acceptance Criteria

1. WHEN determining the first unwatched episode, THE System SHALL use the current sort order of the entry list
2. WHEN episodes are sorted by episode number, THE System SHALL select the first unwatched episode by episode number
3. WHEN entries are filtered, THE System SHALL only consider visible entries for auto-selection
