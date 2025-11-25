# Requirements Document

## Introduction

This feature adds vim-style keyboard navigation to the video browser application, allowing users to navigate the list using `k` (up) and `j` (down) keys in addition to the existing arrow keys. This provides a more ergonomic navigation option for vim users who prefer keeping their hands on the home row.

## Glossary

- **Application**: The terminal-based video file browser and library manager
- **Browse Mode**: The primary navigation mode where users can move through the list of entries
- **Filter Mode**: The search/filter mode where users type to filter entries in real-time
- **Navigation Keys**: Keyboard inputs used to move the selection cursor up or down in the list
- **Vim Keys**: The `j` and `k` keys traditionally used in vim for down and up navigation respectively

## Requirements

### Requirement 1

**User Story:** As a vim user, I want to use `k` and `j` keys to navigate up and down in the list, so that I can keep my hands on the home row for more efficient navigation

#### Acceptance Criteria

1. WHEN the user presses the `k` key in Browse mode, THE Application SHALL move the selection cursor up by one position
2. WHEN the user presses the `j` key in Browse mode, THE Application SHALL move the selection cursor down by one position
3. THE Application SHALL continue to support arrow key navigation (Up and Down) alongside vim key navigation
4. WHEN the user is in Filter mode, THE Application SHALL ignore `k` and `j` key presses for navigation purposes
5. THE Application SHALL treat `k` and `j` keys as regular character input when in Filter mode

### Requirement 2

**User Story:** As a user, I want the vim navigation keys to work seamlessly with existing navigation behavior, so that the feature integrates naturally without disrupting current functionality

#### Acceptance Criteria

1. WHEN the selection cursor is at the top of the list and the user presses `k`, THE Application SHALL keep the cursor at the top position
2. WHEN the selection cursor is at the bottom of the list and the user presses `j`, THE Application SHALL keep the cursor at the bottom position
3. THE Application SHALL update the display immediately after processing `k` or `j` key presses
4. THE Application SHALL maintain the same scrolling behavior for vim keys as it does for arrow keys

### Requirement 3

**User Story:** As a user, I want the vim navigation keys to be available only in Browse mode, so that typing `j` or `k` in Filter mode works as expected for filtering

#### Acceptance Criteria

1. WHEN the Application is in Browse mode, THE Application SHALL process `k` and `j` as navigation commands
2. WHEN the Application is in Filter mode, THE Application SHALL process `k` and `j` as filter text input
3. WHEN the Application is in Edit mode, THE Application SHALL not process `k` and `j` as navigation commands
4. WHEN the Application is in Menu mode, THE Application SHALL not process `k` and `j` as navigation commands

### Requirement 4

**User Story:** As a user, I want the vim navigation feature to be implemented without adding menu helpers, so that the interface remains clean and uncluttered

#### Acceptance Criteria

1. THE Application SHALL not display `k` or `j` key hints in the first-line menu
2. THE Application SHALL not display `k` or `j` key hints in the context menu (F1)
3. THE Application SHALL not add any visual indicators for vim navigation keys in the UI
