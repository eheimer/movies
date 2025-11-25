# Requirements Document

## Introduction

This feature introduces a toggle filter mode that changes the current auto-type-to-filter behavior to require explicit activation via the `/` key. This provides users with more control over when filtering is active and prevents accidental filtering when typing other commands or navigating.

## Glossary

- **Filter Mode**: A state where the Application accepts character input to filter the displayed entries list
- **Browse Mode**: The default navigation state where the Application displays entries and accepts navigation commands
- **Application**: The terminal-based video file browser and library manager
- **Filter String**: The text input used to filter displayed entries
- **Menu Helper**: A visual indicator displayed in the Application header showing available keyboard commands

## Requirements

### Requirement 1

**User Story:** As a user, I want to explicitly activate filter mode by pressing `/`, so that I can prevent accidental filtering while navigating

#### Acceptance Criteria

1. WHEN the user presses the `/` key in Browse Mode, THE Application SHALL enter Filter Mode
2. WHILE in Browse Mode and not in Filter Mode, THE Application SHALL NOT add typed characters to the Filter String
3. WHILE in Filter Mode, THE Application SHALL add typed characters to the Filter String

### Requirement 2

**User Story:** As a user, I want to see different menu helpers based on whether filter mode is active, so that I understand which commands are available

#### Acceptance Criteria

1. WHILE NOT in Filter Mode, THE Application SHALL display the menu helper `[/] filter` instead of `type to filter`
2. WHILE in Filter Mode, THE Application SHALL change the `[ENTER] play` menu helper to `[ENTER] accept filter`
3. WHILE in Filter Mode, THE Application SHALL change the `[ESC] back` menu helper to `[ESC] cancel filter`
4. WHILE in Filter Mode, THE Application SHALL remove all menu helpers except `[ENTER]` and `[ESC]`
5. WHEN entering Filter mode, THE Application SHALL remove any existing text from the Filter String
6. WHEN exiting Filter Mode, THE Application SHALL restore all normal menu helpers
7. WHILE in Filter Mode, typing in the filter field SHALL update the displayed episodes in real-time as it currently does

### Requirement 3

**User Story:** As a user, I want to accept or cancel the filter, so that I can control when the filter is applied

#### Acceptance Criteria

1. WHEN the user presses `[ENTER]` in Filter Mode, THE Application SHALL exit Filter Mode and maintain the current Filter String
2. WHEN the user presses `[ESC]` in Filter Mode, THE Application SHALL clear the Filter String and exit Filter Mode
3. WHEN the user presses `[ENTER]` in Filter Mode, THE Application SHALL return to Browse Mode
4. WHEN the user presses `[ESC]` in Filter Mode, THE Application SHALL return to Browse Mode

### Requirement 4

**User Story:** As a user, I want the filter line to be hidden when not in use, so that the interface is cleaner

#### Acceptance Criteria

1. WHEN NOT in Filter Mode AND the Filter String length is 0, THE Application SHALL hide the `filter:` line
2. WHEN in Filter Mode, THE Application SHALL display the `filter:` line regardless of Filter String length
3. WHEN the filter is accepted AND the Filter String length is greater than 0, THE Application SHALL display the `filter:` line
4. WHEN the filter is canceled, THE Application SHALL hide the `filter:` line

### Requirement 5

**User Story:** As a user, I want clear visual feedback about which UI element is active, so that I understand where my input will go

#### Acceptance Criteria

1. WHILE in Filter Mode, THE Application SHALL remove the highlight from the currently selected item in the episode list
2. WHILE in Filter Mode, THE Application SHALL apply highlight styling to the "filter:" label text only (not the entire filter line)
3. WHILE NOT in Filter Mode, THE Application SHALL apply highlight styling to the currently selected item in the episode list
4. WHILE NOT in Filter Mode, THE Application SHALL NOT apply highlight styling to the "filter:" label
