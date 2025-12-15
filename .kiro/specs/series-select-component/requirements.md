# Requirements Document

**GitHub Issue:** #62

## Introduction

This document specifies the requirements for creating a SeriesSelectWindow component that extracts series selection and creation rendering logic from display.rs into reusable, composable components. The SeriesSelectWindow will provide both series selection from existing series and creation of new series through specialized sub-components, similar to the DetailPanel component architecture.

## Glossary

- **SeriesSelectWindow**: Container component that switches between sub-components based on application mode (SeriesSelect vs SeriesCreate)
- **SeriesSelector**: Sub-component for displaying and navigating through existing series with scrolling support
- **SeriesCreator**: Sub-component for entering new series names with text input handling
- **Series**: Database entity representing a TV series with id and name fields
- **SeriesSelect Mode**: Application state for selecting from existing series list
- **SeriesCreate Mode**: Application state for creating a new series by entering its name
- **Viewport**: Visible portion of the series list when scrolling is required

## Requirements

### Requirement 1

**User Story:** As a developer, I want a SeriesSelectWindow component that can switch between selection and creation modes, so that I can provide both series selection and creation functionality in a modular way.

#### Acceptance Criteria

1. WHEN the application is in SeriesSelect mode, THE SeriesSelectWindow SHALL render the SeriesSelector sub-component
2. WHEN the application is in SeriesCreate mode, THE SeriesSelectWindow SHALL render the SeriesCreator sub-component
3. WHEN the mode changes between SeriesSelect and SeriesCreate, THE SeriesSelectWindow SHALL switch sub-components while preserving window positioning
4. THE SeriesSelectWindow SHALL accept mode, series list, selection state, and window dimensions as parameters
5. THE SeriesSelectWindow SHALL handle window borders and layout positioning consistently across sub-components

### Requirement 2

**User Story:** As a user, I want to select from existing series in a scrollable list, so that I can assign episodes to the correct series efficiently.

#### Acceptance Criteria

1. WHEN SeriesSelector displays the series list, THE system SHALL show series with numbered labels in format "[N] Series Name"
2. WHEN the series list exceeds the window height, THE system SHALL provide scrolling with a scrollbar indicator
3. WHEN navigating through series, THE system SHALL highlight the currently selected series with theme colors
4. WHEN the selected series is outside the visible area, THE system SHALL automatically scroll to keep it visible
5. THE SeriesSelector SHALL truncate long series names to fit within the available window width

### Requirement 3

**User Story:** As a user, I want to create new series by typing their names, so that I can add series that don't exist in the database.

#### Acceptance Criteria

1. WHEN SeriesCreator is active, THE system SHALL display a text input field for entering the series name
2. WHEN typing in the series name field, THE system SHALL handle character input, cursor movement, and text editing operations
3. WHEN using keyboard shortcuts, THE system SHALL support word-based navigation with Ctrl+Left/Right
4. WHEN editing text, THE system SHALL support standard operations like Home, End, Backspace, and Delete
5. THE SeriesCreator SHALL show the cursor position within the text input field

### Requirement 4

**User Story:** As a developer, I want the SeriesSelectWindow components to replace existing display logic, so that the codebase becomes more modular and maintainable.

#### Acceptance Criteria

1. WHEN SeriesSelectWindow components are implemented, THE system SHALL extract logic from draw_series_window() function in display.rs
2. WHEN components are integrated, THE system SHALL maintain identical visual appearance and behavior to the original implementation
3. WHEN using the new components, THE system SHALL support the same window positioning and sizing calculations
4. THE new components SHALL handle the same state parameters as the original implementation including series_selection and first_series
5. THE refactored code SHALL maintain compatibility with existing keyboard navigation and scrolling behavior

### Requirement 5

**User Story:** As a user, I want consistent window behavior across series operations, so that the interface feels cohesive and predictable.

#### Acceptance Criteria

1. WHEN displaying the series window, THE system SHALL center the window horizontally within the available sidebar space
2. WHEN calculating window dimensions, THE system SHALL adjust height based on series count while respecting terminal boundaries
3. WHEN drawing window borders, THE system SHALL use thick borders for SeriesCreate mode and thin borders for SeriesSelect mode
4. WHEN positioning window content, THE system SHALL maintain consistent padding and alignment within borders
5. THE system SHALL handle edge cases like empty series lists and very small terminal sizes gracefully

### Requirement 6

**User Story:** As a developer, I want composable series window components, so that I can enhance the system with features like series search or validation in the future.

#### Acceptance Criteria

1. WHEN components are designed, THE system SHALL separate concerns between container logic and display rendering
2. WHEN adding new features, THE component architecture SHALL support extension without modifying existing component interfaces
3. WHEN rendering components, THE system SHALL allow selective redrawing of individual sub-components
4. THE component design SHALL support parameterized styling and layout options through theme integration
5. THE components SHALL maintain clear separation between data handling, user input processing, and presentation logic