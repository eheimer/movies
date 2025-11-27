# Requirements Document

## Introduction

This feature redesigns how the application manages database and configuration file locations to improve portability across different computers and mount points. The database will be stored alongside the video collection, while the configuration file will be stored in a platform-standard location pointing to the database. This eliminates fragility caused by hardcoded paths and makes it easy to access the same video collection from different computers with different mount points.

## Glossary

- **Application**: The terminal-based video library manager
- **Database**: The SQLite database file (videos.sqlite) containing video metadata
- **Config File**: The JSON configuration file (config.json) stored in ~/.config/movies/
- **Video Collection Directory**: The root directory containing video files and the database
- **db_location**: Configuration parameter storing the absolute path to videos.sqlite
- **Scan Operation**: The process of discovering video files and importing them into the database
- **First-Run**: The initial execution of the Application when no Config File exists
- **Rescan**: Subsequent scan operations after initial setup

## Requirements

### Requirement 1

**User Story:** As a user, I want the database to be stored with my video collection, so that the database and videos stay together when I move or backup my collection

#### Acceptance Criteria

1. WHEN the Application performs a Scan Operation for the first time, THE Application SHALL create the Database in the Video Collection Directory specified by the user
2. THE Application SHALL name the Database file "videos.sqlite"
3. THE Application SHALL store all video file paths in the Database as relative paths from the Video Collection Directory
4. WHEN the Database already exists in a directory, THE Application SHALL use the existing Database without creating a new one

### Requirement 2

**User Story:** As a user, I want the config file stored in a standard location on each computer, so that I can easily find and modify it

#### Acceptance Criteria

1. THE Application SHALL store the Config File at "~/.config/movies/config.json"
2. WHEN the Application starts and no Config File exists, THE Application SHALL create a default Config File with db_location set to null
3. THE Application SHALL include only the db_location parameter in the Config File
4. THE Application SHALL preserve other configuration options (colors, video_extensions, video_player) in the Config File

### Requirement 3

**User Story:** As a user, I want to point the config at my database location, so that I can access my video collection from different computers with different mount points

#### Acceptance Criteria

1. THE Config File SHALL contain a db_location parameter storing the absolute path to the Database
2. WHEN db_location is null, THE Application SHALL prompt the user to specify a Video Collection Directory during the next Scan Operation
3. WHEN db_location points to a valid Database, THE Application SHALL use that Database for all operations
4. WHEN the user changes db_location to a different path, THE Application SHALL switch to a different Database without affecting the original Database

### Requirement 4

**User Story:** As a user performing a first-time scan, I want to specify where my videos are located, so that the application can create or find the database in that location

#### Acceptance Criteria

1. WHEN db_location is null and the user initiates a Scan Operation, THE Application SHALL prompt the user to enter a Video Collection Directory path
2. WHEN the user provides a Video Collection Directory path, THE Application SHALL check if "videos.sqlite" exists in that directory
3. IF "videos.sqlite" exists in the specified directory, THEN THE Application SHALL use the existing Database
4. IF "videos.sqlite" does not exist in the specified directory, THEN THE Application SHALL create a new Database in that directory
5. AFTER creating or finding the Database, THE Application SHALL update the Config File with the absolute path to the Database in the db_location parameter

### Requirement 5

**User Story:** As a user performing subsequent rescans, I want the application to automatically rescan my video directory, so that I don't have to re-enter the path every time

#### Acceptance Criteria

1. WHEN db_location is not null and the user initiates a Rescan, THE Application SHALL automatically scan the parent directory of the Database
2. THE Application SHALL NOT prompt the user for a directory path during Rescan operations
3. THE Application SHALL use the Database location's parent directory as the Video Collection Directory for path resolution

### Requirement 6

**User Story:** As a user, I want path resolution to be based on the database location, so that relative paths in the database remain valid regardless of mount point changes

#### Acceptance Criteria

1. THE Application SHALL derive the Video Collection Directory from the parent directory of the Database file specified in db_location
2. WHEN resolving relative paths from the Database, THE Application SHALL use the Database's parent directory as the root
3. WHEN storing new video file paths, THE Application SHALL calculate paths relative to the Database's parent directory
4. THE Application SHALL maintain path resolution logic that works independently of the db_location value

### Requirement 7

**User Story:** As a user running the application for the first time, I want a smooth setup experience, so that I can quickly start using the application

#### Acceptance Criteria

1. WHEN the Application starts with no Config File, THE Application SHALL create the Config File with db_location set to null
2. WHEN the Application starts and db_location is null, THE Application SHALL automatically enter scan mode and prompt the user for a Video Collection Directory
3. THE Application SHALL NOT create the Database until the user provides a Video Collection Directory path
4. THE Application SHALL handle the null db_location state gracefully without errors or crashes

### Requirement 8

**User Story:** As a user moving my video collection to a different computer, I want to easily reconnect to my existing database, so that I preserve all my organization and metadata

#### Acceptance Criteria

1. WHEN the Application runs on a new computer with no Config File, THE Application SHALL create a default Config File with db_location set to null
2. WHEN the user provides a Video Collection Directory containing an existing Database, THE Application SHALL use the existing Database
3. THE Application SHALL update the Config File with the absolute path to the existing Database
4. AFTER updating the Config File, THE Application SHALL perform a Rescan of the Video Collection Directory to import any new video files
5. THE Application SHALL preserve all existing metadata and organization from the existing Database while adding newly discovered videos
