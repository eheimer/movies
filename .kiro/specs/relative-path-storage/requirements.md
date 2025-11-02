# Requirements Document

## Introduction

This feature converts the movie database application from storing absolute file paths to storing relative paths. The application executable will be located at the root of an external hard drive, and all movie files will be referenced relative to this executable location. This enables the application to work seamlessly across different operating systems (Linux, Windows) where the external drive may be mounted at different locations.

## Glossary

- **Movie_Database_Application**: The Rust-based application that manages movie file metadata and paths
- **Executable_Directory**: The directory where the application executable is located (used for database file location)
- **Database_Location**: Always located in the same directory as the executable, regardless of Root_Directory_Path configuration
- **Relative_Path**: File path expressed relative to the Executable_Root location
- **Absolute_Path**: Complete file path from the system root (e.g., /media/eric/External\ Storage/movies/file.mp4)
- **External_HDD**: The portable hard drive containing both the application and movie files
- **Database_Schema**: The SQLite database structure storing movie metadata and file paths
- **Configuration_File**: The config.json file containing application settings including the video directory path and root directory
- **Video_Directory_Path**: The path to the directory containing video files, stored in the Configuration_File
- **Root_Directory_Path**: The configurable base directory path stored in the Configuration_File, used as the reference point for all relative path calculations

## Requirements

### Requirement 1

**User Story:** As a user with an external HDD containing movies and the application, I want the application to work on both Linux and Windows systems, so that I can access my movie collection regardless of which computer I plug the drive into.

#### Acceptance Criteria

1. WHEN the External_HDD is mounted on Linux, THE Movie_Database_Application SHALL resolve movie file paths correctly using relative path resolution
2. WHEN the External_HDD is mounted on Windows, THE Movie_Database_Application SHALL resolve movie file paths correctly using relative path resolution
3. THE Movie_Database_Application SHALL store all movie file paths as Relative_Path entries in the Database_Schema
4. WHEN the application starts, THE Movie_Database_Application SHALL read the Root_Directory_Path from the Configuration_File to determine the base directory for relative path calculations
5. THE Movie_Database_Application SHALL convert existing Absolute_Path entries to Relative_Path entries during database migration
6. THE Movie_Database_Application SHALL always store the database file in the Executable_Directory, regardless of the Root_Directory_Path configuration

### Requirement 2

**User Story:** As a user adding new movies to the database, I want the application to automatically store relative paths, so that the database remains portable across systems.

#### Acceptance Criteria

1. WHEN a user adds a new movie file, THE Movie_Database_Application SHALL calculate the Relative_Path from the Root_Directory_Path
2. THE Movie_Database_Application SHALL store only the Relative_Path in the Database_Schema
3. WHEN displaying file paths to users, THE Movie_Database_Application SHALL show the full resolved path for clarity
4. IF a movie file is not located under the Root_Directory_Path, THEN THE Movie_Database_Application SHALL display an appropriate error message
5. THE Movie_Database_Application SHALL read the Video_Directory_Path from the Configuration_File as a Relative_Path
6. THE Movie_Database_Application SHALL resolve the Video_Directory_Path relative to the Root_Directory_Path when scanning for movies

### Requirement 3

**User Story:** As a user starting fresh with the updated application, I want to initialize a new database with relative path storage, so that all future movie entries use the portable path format from the beginning.

#### Acceptance Criteria

1. THE Movie_Database_Application SHALL initialize a new Database_Schema designed for Relative_Path storage
2. THE Movie_Database_Application SHALL provide a clear way to start with a fresh database
3. WHEN starting with a new database, THE Movie_Database_Application SHALL create the necessary database tables optimized for relative path storage
4. THE Movie_Database_Application SHALL not attempt to migrate existing absolute path data
5. THE Movie_Database_Application SHALL update the Configuration_File to use relative Video_Directory_Path format (e.g., "/Videos" instead of absolute paths)
6. THE Movie_Database_Application SHALL always create and access the database file in the Executable_Directory, enabling multiple application instances with separate databases

### Requirement 4

**User Story:** As a developer testing the application, I want to configure the root directory path in the config file, so that I can test the application with different root directories without moving the executable.

#### Acceptance Criteria

1. THE Movie_Database_Application SHALL read a Root_Directory_Path setting from the Configuration_File
2. WHEN the Root_Directory_Path is specified in the Configuration_File, THE Movie_Database_Application SHALL use this path as the base for all relative path calculations while keeping the database in the Executable_Directory
3. THE Movie_Database_Application SHALL validate that the Root_Directory_Path exists and is accessible
4. WHEN the Root_Directory_Path is not specified in the Configuration_File, THE Movie_Database_Application SHALL default to using the Executable_Directory as the root for relative path calculations
5. THE Movie_Database_Application SHALL support both absolute and relative Root_Directory_Path values in the Configuration_File

### Requirement 5

**User Story:** As a developer maintaining the application, I want clear error handling for path resolution issues, so that users receive helpful feedback when files cannot be found.

#### Acceptance Criteria

1. WHEN a Relative_Path cannot be resolved to an existing file, THE Movie_Database_Application SHALL provide a descriptive error message
2. THE Movie_Database_Application SHALL distinguish between missing files and path resolution errors
3. WHEN the Root_Directory_Path is not configured or invalid, THE Movie_Database_Application SHALL display an appropriate error and exit gracefully
4. THE Movie_Database_Application SHALL log path resolution attempts for debugging purposes
