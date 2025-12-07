# Requirements Document

**GitHub Issue:** #27

## Introduction

This feature adds the ability to remove database entries for episodes. This allows users to clean up their database by removing episode records, regardless of whether the video file still exists on disk. If the video file still exists, it will be rediscovered during the next scan.

## Glossary

- **Episode**: A database record representing a video file, which may be standalone or part of a series
- **Database Entry**: A record in the SQLite database containing metadata about an episode
- **Video File**: The actual media file on disk that an episode record references
- **Context Menu**: The F1 menu that displays available actions for the selected entry
- **Rescan**: The process of scanning the configured directory for video files and adding them to the database

## Requirements

### Requirement 1

**User Story:** As a user, I want to delete database entries for episodes, so that I can remove episode records from the database.

#### Acceptance Criteria

1. WHEN a user selects an episode entry THEN the system SHALL display a "Delete" option in the context menu (F1)
2. WHEN a user selects the Delete option from the context menu THEN the system SHALL remove the episode record from the database
3. WHEN an episode is deleted THEN the system SHALL refresh the display to reflect the removal
4. WHEN an episode is deleted THEN the system SHALL maintain the user's position in the list when possible
5. WHEN an episode with an existing video file is deleted and a rescan is performed THEN the system SHALL rediscover the video file and create a new database entry

### Requirement 2

**User Story:** As a user, I want the delete function to be menu-only, so that I cannot accidentally delete entries with a mistyped hotkey.

#### Acceptance Criteria

1. WHEN the Delete menu item is defined THEN the system SHALL NOT assign a hotkey to the Delete action
2. WHEN a user is in Browse mode THEN the system SHALL NOT respond to any hotkey for deletion
3. WHEN a user wants to delete an episode THEN the system SHALL require opening the F1 menu and selecting the Delete option

### Requirement 3

**User Story:** As a user, I want the delete function to work correctly in different viewing contexts, so that the display updates appropriately after deletion.

#### Acceptance Criteria

1. WHEN deleting an episode from the top-level view THEN the system SHALL reload all entries
2. WHEN deleting an episode from a series view THEN the system SHALL reload entries for that series
3. WHEN deleting an episode from a season view THEN the system SHALL reload entries for that season
4. WHEN the last episode in a view is deleted THEN the system SHALL adjust the selection to the previous item or the first item if none exists

### Requirement 4

**User Story:** As a user, I want the delete function to only appear for episodes, so that I don't accidentally try to delete series or season entries.

#### Acceptance Criteria

1. WHEN a user selects an episode entry THEN the system SHALL show the Delete option in the context menu
2. WHEN a user selects a series entry THEN the system SHALL NOT show the Delete option in the context menu
3. WHEN a user selects a season entry THEN the system SHALL NOT show the Delete option in the context menu
