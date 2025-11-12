# Requirements Document

## Introduction

This feature adds a menu option to clear the "watched" flag for all episodes in the current view. The user can invoke this action via a menu item labeled "Unwatch All" or by pressing the F7 hotkey. The scope of the operation depends on the current context: if viewing a specific season, it clears watched status for all episodes in that season; if viewing a series, it clears watched status for all episodes across all seasons in that series; if viewing the top-level episode list (no series/season context), it clears watched status for all standalone episodes.

## Glossary

- **Application**: The terminal-based video file browser and library manager
- **Episode**: An individual video file that can be standalone or part of a series
- **Series**: A collection of related episodes (e.g., TV shows)
- **Season**: An organizational unit within a series
- **Watched Flag**: A boolean metadata field indicating whether an episode has been viewed
- **Current View**: The browsing context the user is currently in (series view, season view, or top-level episode list)
- **Menu**: The keyboard-driven interface for executing commands
- **Unwatch All Action**: The operation that clears the watched flag for all episodes in the current view

## Requirements

### Requirement 1

**User Story:** As a user browsing episodes within a season, I want to mark all episodes in that season as unwatched, so that I can reset my viewing progress for that season.

#### Acceptance Criteria

1. WHEN the user presses F7 while viewing a season, THE Application SHALL clear the watched flag for all episodes in that season
2. WHEN the user selects "Unwatch All" from the menu while viewing a season, THE Application SHALL clear the watched flag for all episodes in that season
3. WHEN the Unwatch All Action completes for a season, THE Application SHALL refresh the display to show the updated watched status
4. WHEN the user presses F7 while viewing a season, THE Application SHALL update the database to persist the unwatched status for all episodes in that season

### Requirement 2

**User Story:** As a user browsing a series, I want to mark all episodes across all seasons as unwatched, so that I can reset my viewing progress for the entire series.

#### Acceptance Criteria

1. WHEN the user presses F7 while viewing a series, THE Application SHALL clear the watched flag for all episodes across all seasons in that series
2. WHEN the user selects "Unwatch All" from the menu while viewing a series, THE Application SHALL clear the watched flag for all episodes across all seasons in that series
3. WHEN the Unwatch All Action completes for a series, THE Application SHALL refresh the display to show the updated watched status
4. WHEN the user presses F7 while viewing a series, THE Application SHALL update the database to persist the unwatched status for all episodes in that series

### Requirement 3

**User Story:** As a user browsing the top-level episode list, I want to mark all standalone episodes as unwatched, so that I can reset my viewing progress for episodes not organized into series.

#### Acceptance Criteria

1. WHEN the user presses F7 while viewing the top-level episode list, THE Application SHALL clear the watched flag for all standalone episodes
2. WHEN the user selects "Unwatch All" from the menu while viewing the top-level episode list, THE Application SHALL clear the watched flag for all standalone episodes
3. WHEN the Unwatch All Action completes for the top-level episode list, THE Application SHALL refresh the display to show the updated watched status
4. WHEN the user presses F7 while viewing the top-level episode list, THE Application SHALL update the database to persist the unwatched status for all standalone episodes

### Requirement 4

**User Story:** As a user, I want the F7 hotkey to be consistently mapped to the Unwatch All action across all browsing contexts, so that I can quickly reset watched status without navigating menus.

#### Acceptance Criteria

1. WHEN the user presses F7 in any browsing mode, THE Application SHALL execute the Unwatch All Action for the current view context
2. THE Application SHALL display "Unwatch All" as the menu item label for this action
3. THE Application SHALL associate the F7 key with the Unwatch All Action in the keyboard event handler
