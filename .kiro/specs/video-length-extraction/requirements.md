# Requirements Document

**GitHub Issue:** #12

## Introduction

This feature enables automatic extraction of video duration from episode files during rescan operations. The system will read video file metadata to populate the episode length field in the database, supporting multiple video formats (mkv, avi, mp4) with an extensible architecture for future format additions. The implementation will be self-contained within the Rust application without requiring external OS utilities.

## Glossary

- **Episode**: A video file entry in the database with associated metadata including length
- **Rescan Operation**: The process of scanning the configured directory for new video files and updating the database
- **Video Length**: The duration of a video file in minutes, stored in the episode table
- **Video Metadata**: Information embedded in video file containers (mkv, avi, mp4) including duration
- **System**: The video library manager application

## Requirements

### Requirement 1

**User Story:** As a user, I want the system to automatically extract video duration from files, so that I don't have to manually enter episode lengths.

#### Acceptance Criteria

1. WHEN the System scans a video file with mkv extension THEN the System SHALL extract the duration from the file metadata
2. WHEN the System scans a video file with avi extension THEN the System SHALL extract the duration from the file metadata
3. WHEN the System scans a video file with mp4 extension THEN the System SHALL extract the duration from the file metadata
4. WHEN the System extracts duration from a video file THEN the System SHALL store the duration in minutes in the episode length field
5. WHEN the System cannot extract duration from a video file THEN the System SHALL leave the episode length field unchanged

### Requirement 2

**User Story:** As a user, I want the rescan operation to update missing episode lengths, so that existing episodes without length data get populated automatically.

#### Acceptance Criteria

1. WHEN the System performs a rescan operation THEN the System SHALL identify all episodes with null length values
2. WHEN the System performs a rescan operation THEN the System SHALL identify all episodes with zero length values
3. WHEN the System identifies an episode with missing length THEN the System SHALL attempt to extract duration from the corresponding video file
4. WHEN the System successfully extracts duration during rescan THEN the System SHALL update the episode length field in the database
5. WHEN the System fails to extract duration during rescan THEN the System SHALL preserve the existing length value

### Requirement 3

**User Story:** As a developer, I want the video metadata extraction to be extensible, so that I can add support for additional video formats in the future.

#### Acceptance Criteria

1. WHEN the System architecture is designed THEN the System SHALL separate format-specific parsing logic from the core extraction interface
2. WHEN a new video format needs support THEN the System SHALL allow adding format handlers without modifying existing extraction code
3. WHEN the System processes a video file THEN the System SHALL select the appropriate format handler based on file extension

### Requirement 4

**User Story:** As a user, I want the system to extract video duration when I play a video with missing length, so that the metadata gets populated through normal usage.

#### Acceptance Criteria

1. WHEN a user plays an episode with null length value THEN the System SHALL attempt to extract duration from that specific video file before launching the player
2. WHEN a user plays an episode with zero length value THEN the System SHALL attempt to extract duration from that specific video file before launching the player
3. WHEN the System successfully extracts duration before playback THEN the System SHALL update the episode length field in the database
4. WHEN the System fails to extract duration before playback THEN the System SHALL proceed with launching the video player
5. WHEN the System extracts duration before playback THEN the System SHALL process only the selected episode

### Requirement 6

**User Story:** As a user, I want the system to extract video duration when I enter edit mode for an episode with missing length, so that I can see the correct duration while editing.

#### Acceptance Criteria

1. WHEN a user enters edit mode for an episode with null length value THEN the System SHALL attempt to extract duration from that specific video file
2. WHEN a user enters edit mode for an episode with zero length value THEN the System SHALL attempt to extract duration from that specific video file
3. WHEN the System successfully extracts duration in edit mode THEN the System SHALL update the episode length field in the database
4. WHEN the System successfully extracts duration in edit mode THEN the System SHALL display the extracted length in the edit interface
5. WHEN the System extracts duration in edit mode THEN the System SHALL process only the selected episode

### Requirement 5

**User Story:** As a user, I want video length extraction to work without external dependencies, so that the application remains portable and self-contained.

#### Acceptance Criteria

1. WHEN the System extracts video metadata THEN the System SHALL use only Rust libraries and dependencies
2. WHEN the System is deployed THEN the System SHALL NOT require external utilities like ffmpeg or mediainfo to be installed
3. WHEN the System reads video files THEN the System SHALL parse container formats directly using Rust-based parsers

### Requirement 7

**User Story:** As a user, I want episode duration displayed in hours, minutes, and seconds format, so that I can easily understand the video length.

#### Acceptance Criteria

1. WHEN the System displays an episode with length data THEN the System SHALL format the duration as hh:mm:ss
2. WHEN the System displays duration less than one hour THEN the System SHALL display the format as 00:mm:ss
3. WHEN the System displays duration in edit mode THEN the System SHALL use the hh:mm:ss format
4. WHEN the System displays duration in browse mode THEN the System SHALL use the hh:mm:ss format
