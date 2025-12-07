# Requirements Document

**GitHub Issue:** #28

## Introduction

This feature adds a persistent status line at the bottom of the terminal that displays real-time status messages to the user. The status line will show messages like "Scanning...", "Scanning complete", "Playing video: <title>", and other operational feedback. The status line must be reserved and protected from being overwritten by the main content area.

## Glossary

- **Application**: The terminal-based video library manager
- **Status Line**: The last line of the terminal reserved for displaying status messages
- **Status Message**: A text message displayed on the Status Line to inform the user of current operations
- **Main Content Area**: All terminal lines except the header and Status Line
- **Redraw**: The process of clearing and re-rendering the terminal display

## Requirements

### Requirement 1

**User Story:** As a user, I want to see a status line at the bottom of the terminal, so that I can receive real-time feedback about what the application is doing

#### Acceptance Criteria

1. THE Application SHALL reserve the last line of the terminal as the Status Line
2. THE Application SHALL prevent the Main Content Area from overwriting the Status Line
3. WHEN the Application redraws the screen, THE Application SHALL preserve the current Status Message on the Status Line
4. THE Application SHALL display Status Messages in a consistent location regardless of terminal size changes

### Requirement 2

**User Story:** As a user scanning for videos, I want to see scanning progress, so that I know the application is working and when the scan completes

#### Acceptance Criteria

1. WHEN the Application begins a scan operation, THE Application SHALL display "Scanning..." on the Status Line
2. WHEN the Application completes a scan operation, THE Application SHALL display "Scanning complete" on the Status Line
3. WHEN the Application completes a scan with new videos found, THE Application SHALL display "Scanning complete (N videos)" on the Status Line where N is the count of new videos
4. WHEN the Application completes a scan with no new videos, THE Application SHALL display "Scanning complete. No new videos found" on the Status Line

### Requirement 3

**User Story:** As a user playing a video, I want to see which video is playing, so that I have confirmation the correct video was launched

#### Acceptance Criteria

1. WHEN the Application launches a video player, THE Application SHALL display "Playing video: <title>" on the Status Line where <title> is the episode name
2. THE Application SHALL display the playing video message immediately when the video player is launched

### Requirement 4

**User Story:** As a user, I want status messages to persist until replaced, so that I can see the last operation that occurred

#### Acceptance Criteria

1. WHEN the Application displays a Status Message, THE Application SHALL keep that message visible until a new Status Message is displayed
2. THE Application SHALL NOT clear the Status Line during normal screen redraws
3. WHEN the Application exits a mode or returns to browse mode, THE Application SHALL preserve the current Status Message
4. THE Application SHALL maintain the Status Message across navigation actions (up, down, enter, backspace)

### Requirement 5

**User Story:** As a user, I want to see status messages for database operations, so that I understand what the application is doing during setup and rescans

#### Acceptance Criteria

1. WHEN the Application connects to an existing database, THE Application SHALL display "Connected to existing database at <path>" on the Status Line
2. WHEN the Application creates a new database, THE Application SHALL display "Creating new database..." on the Status Line
3. WHEN the Application completes database creation, THE Application SHALL display "Created new database and imported N videos" on the Status Line
4. WHEN the Application encounters an error during database operations, THE Application SHALL display an appropriate error message on the Status Line

### Requirement 6

**User Story:** As a developer, I want a centralized status message API, so that any part of the application can easily display status messages

#### Acceptance Criteria

1. THE Application SHALL provide a function to set the current Status Message
2. THE Application SHALL provide a function to clear the Status Message
3. THE Application SHALL provide a function to get the current Status Message
4. THE Application SHALL store the current Status Message in the application state
5. THE Application SHALL ensure the Status Message is thread-safe for concurrent access
