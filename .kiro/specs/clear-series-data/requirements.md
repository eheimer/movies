# Requirements Document

## Introduction

This document specifies the requirements for adding a menu option to clear series, season, and episode number data from an episode. The feature provides users with a quick way to remove organizational metadata when an episode should no longer be associated with a series.

## Glossary

- **System**: The video library manager application
- **Episode**: An individual video file entry in the database
- **Series**: A collection of related episodes (e.g., TV shows)
- **Season**: An organizational unit within a series that groups related episodes
- **Episode Number**: A numeric field identifying an episode's position within a season
- **Series Data**: The collective metadata including series assignment, season assignment, and episode number
- **Context Menu**: The menu displayed when the user presses F1 in Browse mode
- **Menu Item**: An actionable option displayed in the context menu with an associated hotkey
- **Browse Mode**: The application mode where users navigate through entries

## Requirements

### Requirement 1

**User Story:** As a user, I want a menu option to clear series data, so that I can quickly remove series, season, and episode number assignments from an episode.

#### Acceptance Criteria

1. WHEN the user is in Browse mode with an episode selected, IF the episode has a series assigned OR a season assigned OR a non-empty episode number, THEN the System SHALL display a "Clear Series Data" menu item with hotkey F6 in the context menu
2. WHEN the user is in Browse mode with an episode selected, IF the episode has no series AND no season AND an empty or zero episode number, THEN the System SHALL NOT display the "Clear Series Data" menu item
3. WHEN the user presses F6 in Browse mode, IF the "Clear Series Data" menu item is available, THEN the System SHALL execute the clear series data action
4. WHEN the user selects "Clear Series Data" from the context menu, THEN the System SHALL execute the clear series data action

### Requirement 2

**User Story:** As a user, I want the clear series data action to remove all organizational metadata, so that the episode becomes standalone without any series associations.

#### Acceptance Criteria

1. WHEN the System executes the clear series data action, THEN the System SHALL set the episode's series assignment to NULL in the database
2. WHEN the System executes the clear series data action, THEN the System SHALL set the episode's season assignment to NULL in the database
3. WHEN the System executes the clear series data action, THEN the System SHALL set the episode's episode number to an empty string in the database
4. WHEN the System executes the clear series data action, THEN the System SHALL reload the entries list to reflect the updated episode data
5. WHEN the System executes the clear series data action, THEN the System SHALL return to Browse mode
6. WHEN the System executes the clear series data action, THEN the System SHALL trigger a screen redraw to display the updated state
