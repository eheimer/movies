# Requirements Document

## Introduction

This feature adds dirty record tracking and visual feedback in EDIT mode. When a user modifies any field, the field names are highlighted with configurable colors, and the save changes menu option is conditionally displayed based on whether the record has been modified. This provides clear visual indication of unsaved changes and streamlines the user interface.

## Glossary

- **Application**: The terminal-based video file browser and library manager
- **EDIT Mode**: The application mode where users can modify episode metadata fields
- **Dirty Record**: A record (episode) that has one or more modified fields that have not been saved to the database
- **Dirty Field**: A field whose value has been modified but not yet saved to the database
- **Field Name**: The label displayed for an editable field (e.g., "Name:", "Year:", "Length:")
- **Field Value**: The text content displayed for an editable field (e.g., episode name, year, length)
- **Menu Item**: A keyboard shortcut and action label displayed at the bottom of the screen (e.g., "[F2] save changes")
- **Config File**: The config.json file that stores application configuration settings
- **Color Configuration**: The foreground and background color settings for visual display

## Requirements

### Requirement 1

**User Story:** As a user editing episode metadata, I want to see which fields I've modified, so that I can easily identify unsaved changes before saving or canceling.

#### Acceptance Criteria

1. WHEN a user modifies a field value in EDIT mode, THE Application SHALL display that Field Name using the dirty foreground color and dirty background color
2. WHILE a field remains unmodified in EDIT mode, THE Application SHALL display that Field Name using the standard colors
3. WHEN a user saves changes in EDIT mode, THE Application SHALL revert all Field Name displays to standard colors
4. WHEN a user cancels changes in EDIT mode, THE Application SHALL revert all Field Name displays to standard colors

### Requirement 2

**User Story:** As a user, I want to configure the colors used for dirty fields, so that I can customize the visual appearance to my preferences.

#### Acceptance Criteria

1. THE Application SHALL read dirty_fg and dirty_bg color values from the Config File
2. WHERE the Config File does not contain dirty_fg, THE Application SHALL use "Black" as the default dirty foreground color
3. WHERE the Config File does not contain dirty_bg, THE Application SHALL use "White" as the default dirty background color
4. WHEN the Config File is created for the first time, THE Application SHALL include dirty_fg set to "Black" and dirty_bg set to "White"

### Requirement 3

**User Story:** As a user, I want the dirty field highlighting to work consistently across all editable fields, so that I have a uniform editing experience.

#### Acceptance Criteria

1. THE Application SHALL apply dirty field highlighting to all editable fields in EDIT mode including episode name, year, length, episode number, and season number
2. THE Application SHALL track the modified state independently for each field
3. WHEN a user navigates between fields in EDIT mode, THE Application SHALL maintain the dirty state of each field
4. THE Application SHALL display the currently selected field cursor position correctly regardless of dirty state

### Requirement 4

**User Story:** As a user, I want to see the save changes option only when I have unsaved modifications, so that the interface is cleaner and I'm not confused about whether I need to save.

#### Acceptance Criteria

1. WHEN the Dirty Record state is true in EDIT mode, THE Application SHALL display the "[F2] save changes" Menu Item
2. WHEN the Dirty Record state is false in EDIT mode, THE Application SHALL NOT display the "[F2] save changes" Menu Item
3. WHEN a user modifies any field value in EDIT mode, THE Application SHALL set the Dirty Record state to true
4. WHEN a user saves changes in EDIT mode, THE Application SHALL set the Dirty Record state to false
5. WHEN a user cancels changes in EDIT mode, THE Application SHALL set the Dirty Record state to false
6. WHEN a user enters EDIT mode for a record, THE Application SHALL initialize the Dirty Record state to false
