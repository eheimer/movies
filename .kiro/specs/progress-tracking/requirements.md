# Requirements Document

## Introduction

This specification defines the progress tracking functionality for the video library manager. The system will track viewing progress, automatically mark episodes as watched when a threshold is reached, and provide resume functionality for partially watched episodes.

## Glossary

- **System**: The video library manager application
- **Episode**: A video file entry in the database
- **Progress_Time**: The timestamp representing how far into an episode the user has watched
- **Last_Watched_Time**: The datetime when an episode was last marked as watched
- **Watched_Threshold**: The configurable percentage of episode completion that triggers automatic watched status
- **Progress_Update_Interval**: The time interval (10 seconds) for updating progress during playback

## Requirements

### Requirement 1: Configuration Enhancement

**User Story:** As a user, I want to configure the watched threshold percentage, so that I can customize when episodes are automatically marked as watched.

#### Acceptance Criteria

1. THE System SHALL add a watched_threshold field to the configuration file with a default value of 95
2. THE System SHALL validate that the watched_threshold is between 1 and 100 percent
3. WHEN the configuration is invalid, THE System SHALL use the default threshold value and log a warning
4. THE System SHALL reload the threshold value when the configuration file is updated

### Requirement 2: Database Schema Enhancement

**User Story:** As a developer, I want to extend the episode database schema to store progress tracking data, so that the system can persist viewing progress and watched timestamps.

#### Acceptance Criteria

1. THE System SHALL add a last_watched_time field to the episode table to store datetime values
2. THE System SHALL add a last_progress_time field to the episode table to store time duration values
3. WHEN the database schema is updated, THE System SHALL preserve existing episode data
4. THE System SHALL handle null values for new fields in existing episodes

### Requirement 3: Progress Tracking During Playback

**User Story:** As a user, I want the system to automatically track my viewing progress while watching episodes, so that I can resume from where I left off.

#### Acceptance Criteria

1. THE System SHALL support player-specific progress tracking mechanisms through a plugin architecture
2. WHEN an episode finishes playing, THE System SHALL retrieve the final playback position from the player
3. WHEN playback position data is available, THE System SHALL store it in the last_progress_time field
4. THE System SHALL handle different video players with different progress tracking capabilities

### Requirement 4: Automatic Watched Status Management

**User Story:** As a user, I want episodes to be automatically marked as watched when I've seen most of the content, so that I don't have to manually track completion.

#### Acceptance Criteria

1. WHEN an episode's progress exceeds the configured watched_threshold percentage of total duration, THE System SHALL automatically mark the episode as watched
2. WHEN an episode is marked as watched automatically, THE System SHALL update the last_watched_time to the current datetime
3. WHEN an episode is marked as watched automatically, THE System SHALL reset the last_progress_time to zero
4. THE System SHALL only auto-mark episodes as watched once per viewing session

### Requirement 5: Resume Playback Functionality

**User Story:** As a user, I want to resume watching episodes from where I left off, so that I don't have to manually seek to my previous position.

#### Acceptance Criteria

1. WHEN a user plays an episode with existing progress, THE System SHALL start playback at the last_progress_time position
2. WHEN a user plays an episode without existing progress, THE System SHALL start playback from the beginning
3. WHEN a user plays a watched episode with zero progress, THE System SHALL start playback from the beginning
4. THE System SHALL provide the resume position to the video player during launch

### Requirement 6: Progress Data Management

**User Story:** As a user, I want the system to handle progress data consistently, so that my viewing history is accurate and reliable.

#### Acceptance Criteria

1. WHEN an episode is manually marked as watched, THE System SHALL update the last_watched_time and reset last_progress_time to zero
2. WHEN an episode is manually marked as unwatched, THE System SHALL preserve the last_progress_time value
3. THE System SHALL validate that progress times do not exceed episode duration
4. WHEN progress data is corrupted or invalid, THE System SHALL reset it to zero and continue operation

### Requirement 7: Progress Display in User Interface

**User Story:** As a user, I want to see progress information when browsing episodes, so that I can quickly identify partially watched content and when episodes were last viewed.

#### Acceptance Criteria

1. WHEN viewing episode details, THE System SHALL display the last_watched_time as read-only information if the episode has been watched
2. WHEN viewing episode details, THE System SHALL display the last_progress_time as read-only information if the episode has been partially watched
3. THE System SHALL format progress time as HH:MM:SS for display
4. THE System SHALL format last watched time as a human-readable date and time
5. WHEN an episode has no progress data, THE System SHALL not display progress fields in the details
6. THE System SHALL prevent user editing of progress and last watched time fields

### Requirement 8: Player-Specific Plugin Architecture

**User Story:** As a user, I want progress tracking to work with different video players, so that the feature adapts to my preferred player's capabilities.

#### Acceptance Criteria

1. THE System SHALL define a trait-based plugin interface for video player integrations
2. WHEN launching a video player with resume position, THE System SHALL use the player-specific plugin to pass the start time
3. WHEN playback completes, THE System SHALL use the player-specific plugin to retrieve the final playback position
4. THE System SHALL support Celluloid/mpv through reading watch-later files after playback
5. THE System SHALL allow easy addition of new player plugins without modifying core code