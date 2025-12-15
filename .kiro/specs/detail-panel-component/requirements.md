# Requirements Document

**GitHub Issue:** #58

## Introduction

This document specifies the requirements for creating a DetailPanel component that extracts episode detail rendering logic from display.rs into reusable, composable components. The DetailPanel will provide both read-only metadata display and interactive episode editing capabilities through specialized sub-components.

## Glossary

- **DetailPanel**: Container component that switches between sub-components based on application mode
- **MetadataDisplay**: Read-only sub-component for displaying episode details with proper field styling
- **EpisodeEditor**: Interactive sub-component with cursor positioning, field highlighting, and dirty state indicators
- **EpisodeField**: Enumeration of editable episode fields (title, series, season, episode number, etc.)
- **Dirty Field**: A field that has been modified but not yet saved to the database
- **Mode**: Application state determining whether the detail panel shows read-only or editable content

## Requirements

### Requirement 1

**User Story:** As a developer, I want a DetailPanel component that can switch between display modes, so that I can provide both read-only and editable views of episode details.

#### Acceptance Criteria

1. WHEN the application is in Browse mode, THE DetailPanel SHALL render the MetadataDisplay sub-component
2. WHEN the application is in Edit mode, THE DetailPanel SHALL render the EpisodeEditor sub-component
3. WHEN the mode changes, THE DetailPanel SHALL switch between sub-components without losing episode data
4. THE DetailPanel SHALL accept mode, episode details, and editing state as parameters
5. THE DetailPanel SHALL handle window borders and layout positioning consistently across sub-components

### Requirement 2

**User Story:** As a user, I want to view episode metadata in a clean, read-only format, so that I can easily see all episode information without editing controls.

#### Acceptance Criteria

1. WHEN MetadataDisplay renders episode details, THE system SHALL display all episode fields with appropriate styling
2. WHEN displaying episode information, THE system SHALL show series name, season number, episode number, title, and path
3. WHEN episode data contains long text, THE system SHALL truncate text appropriately to fit within window boundaries
4. WHEN displaying non-editable fields, THE system SHALL render path and filename information clearly
5. THE MetadataDisplay SHALL maintain consistent field layout and spacing

### Requirement 3

**User Story:** As a user, I want to edit episode details with visual feedback, so that I can modify episode information and see which fields have been changed.

#### Acceptance Criteria

1. WHEN EpisodeEditor is active, THE system SHALL highlight the currently selected field for editing
2. WHEN a field has been modified, THE system SHALL visually indicate the field as dirty
3. WHEN navigating between fields, THE system SHALL show cursor position within the current field
4. WHEN editing text fields, THE system SHALL handle text input and cursor movement within field boundaries
5. THE EpisodeEditor SHALL prevent editing of non-editable fields like path and filename

### Requirement 4

**User Story:** As a developer, I want the DetailPanel components to replace existing display logic, so that the codebase becomes more modular and maintainable.

#### Acceptance Criteria

1. WHEN DetailPanel components are implemented, THE system SHALL extract logic from draw_detail_window() function in display.rs
2. WHEN components are integrated, THE system SHALL maintain identical visual appearance and behavior
3. WHEN using the new components, THE system SHALL support all existing episode field types from EpisodeField enum
4. THE new components SHALL handle the same state parameters as the original implementation
5. THE refactored code SHALL maintain compatibility with existing edit cursor positioning and dirty field tracking

### Requirement 5

**User Story:** As a developer, I want composable detail components, so that I can enhance the system with features like field validation or different view modes in the future.

#### Acceptance Criteria

1. WHEN components are designed, THE system SHALL separate concerns between container and display logic
2. WHEN adding new features, THE component architecture SHALL support extension without modifying existing component interfaces
3. WHEN rendering components, THE system SHALL allow selective redrawing of individual components
4. THE component design SHALL support parameterized styling and layout options
5. THE components SHALL maintain clear separation between data handling and presentation logic