# Requirements Document

**GitHub Issue:** #57

## Introduction

This feature involves creating a modular header component system that extracts header rendering logic from the monolithic `draw_header()` function in `display.rs` into reusable, composable components. The header component will manage layout and dispatch to specialized sub-components for different header sections.

## Glossary

- **Header Component**: Container component that manages layout and coordinates sub-components
- **HotkeyHelper Component**: Displays first line with menu hotkeys and context helpers
- **LastActionLine Component**: Shows repeatable actions with hotkey reminders
- **Breadcrumbs Component**: Displays navigation context showing current location
- **FilterLine Component**: Shows filter input display with active state highlighting
- **Cell-based Rendering**: Existing rendering pattern using Cell structures for terminal output
- **Sub-component**: Individual header section component that renders a single row
- **Terminal Application**: The movies library manager application

## Requirements

### Requirement 1

**User Story:** As a developer, I want a modular header component system, so that header rendering logic is organized into maintainable, reusable components.

#### Acceptance Criteria

1. THE Header Component SHALL compose four sub-components vertically in a single container
2. THE Header Component SHALL calculate total height dynamically based on active sub-components
3. THE Header Component SHALL follow existing Cell-based rendering patterns
4. THE Header Component SHALL be theme-aware and width-constrained like existing components
5. THE Header Component SHALL replace the monolithic draw_header() function in display.rs

### Requirement 2

**User Story:** As a developer, I want stateless sub-components, so that they are predictable and easy to test.

#### Acceptance Criteria

1. THE HotkeyHelper Component SHALL receive all needed data as constructor parameters
2. THE LastActionLine Component SHALL receive all needed data as constructor parameters  
3. THE Breadcrumbs Component SHALL receive all needed data as constructor parameters
4. THE FilterLine Component SHALL receive all needed data as constructor parameters
5. THE Sub-components SHALL NOT maintain internal state between render calls

### Requirement 3

**User Story:** As a user, I want the header to display relevant hotkey information, so that I can quickly access available actions.

#### Acceptance Criteria

1. WHEN the Terminal Application displays the header, THE HotkeyHelper Component SHALL show "[F1] Menu" as the primary hotkey
2. WHEN context-specific helpers are available, THE HotkeyHelper Component SHALL display them alongside the menu hotkey
3. WHEN overflow menu items exist, THE HotkeyHelper Component SHALL include them in the display
4. THE HotkeyHelper Component SHALL reference the menu system in menu.rs for hotkey logic

### Requirement 4

**User Story:** As a user, I want to see repeatable actions in the header, so that I can quickly repeat the last action I performed.

#### Acceptance Criteria

1. WHEN a repeatable action has been performed, THE LastActionLine Component SHALL display the action with its hotkey
2. WHEN no repeatable action is available, THE LastActionLine Component SHALL return an empty display
3. THE LastActionLine Component SHALL format the display as "Press [hotkey] to repeat: action description"

### Requirement 5

**User Story:** As a user, I want to see my current navigation context, so that I understand where I am in the library hierarchy.

#### Acceptance Criteria

1. WHEN browsing at the top level, THE Breadcrumbs Component SHALL display appropriate top-level context
2. WHEN browsing within a series, THE Breadcrumbs Component SHALL display "Browsing [Series Name]"
3. WHEN browsing within a season, THE Breadcrumbs Component SHALL display "Browsing [Series Name] -> [Season N]"
4. THE Breadcrumbs Component SHALL format navigation context clearly for user understanding

### Requirement 6

**User Story:** As a user, I want to see filter status in the header, so that I know when filters are active and what they contain.

#### Acceptance Criteria

1. WHEN a filter is active, THE FilterLine Component SHALL display the filter input with highlighting
2. WHEN no filter is active, THE FilterLine Component SHALL return an empty display
3. THE FilterLine Component SHALL provide visual indication when the filter input is focused
4. THE FilterLine Component SHALL display the current filter text clearly

### Requirement 7

**User Story:** As a developer, I want conditional rendering of sub-components, so that the header only shows relevant information and maintains optimal screen space usage.

#### Acceptance Criteria

1. WHEN a sub-component has no content to display, THE Sub-component SHALL return an empty Vec
2. THE Header Component SHALL skip rendering of sub-components that return empty content
3. THE Header Component SHALL recalculate height when sub-components change their visibility
4. THE Terminal Application SHALL maintain proper layout when header height changes dynamically