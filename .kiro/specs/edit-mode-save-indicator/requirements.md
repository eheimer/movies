# Requirements Document

**GitHub Issue:** #46

## Introduction

This feature enhances the edit mode header to display a save option when fields have been modified. Currently, users in edit mode only see the cancel option, making it unclear that F2 can be used to save changes when fields are dirty.

## Glossary

- **Edit Mode**: The application mode where users can modify episode field values
- **Dirty Field**: A field whose value has been changed but not yet saved to the database
- **Header**: The top line of the terminal display showing available keyboard shortcuts

## Requirements

### Requirement 1

**User Story:** As a user editing episode fields, I want to see a save option in the header when I've made changes, so that I know I can press F2 to save my modifications.

#### Acceptance Criteria

1. WHEN a user enters edit mode with no field modifications THEN the system SHALL display only the cancel option in the header
2. WHEN a user modifies any field value in edit mode THEN the system SHALL display both cancel and save options in the header
3. WHEN the save option is displayed THEN the system SHALL show it as "[F2] save" following "[ESC] cancel"
