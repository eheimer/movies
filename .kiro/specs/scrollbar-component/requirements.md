# Requirements Document

**GitHub Issue:** #53

## Introduction

This specification defines the refactoring of the existing scrollbar functionality to conform to the component framework established in issue #51. The current scrollbar implementation (`src/scrollbar.rs`) predates the component framework and performs direct terminal I/O operations. This refactoring will transform it into a proper component that produces Cell arrays, making it testable, reusable, and consistent with other components in the system.

The scrollbar component will maintain all existing functionality (vertical scrollbar with height, item count, and position properties) while adopting the component architecture pattern. Future enhancements like horizontal scrollbar support are noted but not included in this specification.

## Glossary

- **Scrollbar Component**: A component that renders a vertical scrollbar with track and indicator
- **ScrollBarState**: Data structure containing scrollbar position and dimension calculations
- **Track**: The full vertical area where the scrollbar can appear
- **Indicator**: The movable portion of the scrollbar showing current position
- **Component Framework**: The architecture defined in issue #51 using Cell arrays and the Component trait
- **Cell**: A single terminal character position with styling (from component framework)
- **Visible Items**: The number of items that fit in the current viewport
- **Total Items**: The complete count of items in the scrollable list

## Requirements

### Requirement 1

**User Story:** As a developer, I want the scrollbar to implement the Component trait, so that it follows the same pattern as other components in the system.

#### Acceptance Criteria

1. WHEN the Scrollbar struct is created THEN the system SHALL implement the Component trait
2. WHEN the render method is called THEN the system SHALL return a 2D array of Cell objects
3. WHEN the render method is called THEN the system SHALL accept height as a parameter
4. WHEN the render method is called THEN the system SHALL accept theme data as a parameter
5. WHEN the render method is called THEN the system SHALL not perform direct terminal I/O operations

### Requirement 2

**User Story:** As a developer, I want the Scrollbar component to encapsulate scrollbar state, so that position and dimension calculations are part of the component.

#### Acceptance Criteria

1. WHEN the Scrollbar component is created THEN the system SHALL store total item count
2. WHEN the Scrollbar component is created THEN the system SHALL store visible item count
3. WHEN the Scrollbar component is created THEN the system SHALL store current position within the list
4. WHEN the Scrollbar component renders THEN the system SHALL calculate indicator position based on scroll position
5. WHEN the Scrollbar component renders THEN the system SHALL calculate indicator height proportional to visible/total ratio

### Requirement 3

**User Story:** As a developer, I want the scrollbar to be hidden when not needed, so that screen space is used efficiently.

#### Acceptance Criteria

1. WHEN total items is less than or equal to visible items THEN the system SHALL return an empty Cell array
2. WHEN total items is zero THEN the system SHALL return an empty Cell array
3. WHEN available height is zero THEN the system SHALL return an empty Cell array
4. WHEN scrollbar is needed THEN the system SHALL return a Cell array with track and indicator characters

### Requirement 4

**User Story:** As a developer, I want the scrollbar to render track and indicator characters, so that users can see their position in the list.

#### Acceptance Criteria

1. WHEN the Scrollbar component renders THEN the system SHALL use scrollbar_track_char from theme for track positions
2. WHEN the Scrollbar component renders THEN the system SHALL use scrollbar_indicator_char from theme for indicator positions
3. WHEN the Scrollbar component renders THEN the system SHALL apply scrollbar_fg color from theme
4. WHEN the Scrollbar component renders THEN the system SHALL apply scrollbar_bg color from theme
5. WHEN the Scrollbar component renders THEN the system SHALL ensure indicator stays within track bounds

### Requirement 5

**User Story:** As a developer, I want the scrollbar indicator to move proportionally, so that it accurately represents the current scroll position.

#### Acceptance Criteria

1. WHEN the scroll position is at the top THEN the system SHALL position the indicator at the track start
2. WHEN the scroll position is at the bottom THEN the system SHALL position the indicator at the track end
3. WHEN the scroll position is in the middle THEN the system SHALL position the indicator proportionally within the track
4. WHEN the indicator height is calculated THEN the system SHALL ensure a minimum height of 1 row
5. WHEN the indicator would extend past track end THEN the system SHALL clamp it to stay within bounds

### Requirement 6

**User Story:** As a developer, I want to preserve the existing calculate_scrollbar_state function, so that existing code continues to work during migration.

#### Acceptance Criteria

1. WHEN the scrollbar module is refactored THEN the system SHALL maintain the calculate_scrollbar_state function
2. WHEN calculate_scrollbar_state is called THEN the system SHALL return a ScrollBarState with the same structure
3. WHEN the Scrollbar component is created from ScrollBarState THEN the system SHALL use the calculated values
4. WHEN existing code calls calculate_scrollbar_state THEN the system SHALL continue to work without modification

### Requirement 7

**User Story:** As a developer, I want the scrollbar component to be testable without terminal interaction, so that I can verify rendering logic independently.

#### Acceptance Criteria

1. WHEN the Scrollbar component renders THEN the system SHALL produce Cell arrays that can be inspected
2. WHEN testing scrollbar rendering THEN the system SHALL allow verification of track character positions
3. WHEN testing scrollbar rendering THEN the system SHALL allow verification of indicator character positions
4. WHEN testing scrollbar rendering THEN the system SHALL allow verification of color application
5. WHEN testing scrollbar rendering THEN the system SHALL not require terminal I/O operations

### Requirement 8

**User Story:** As a user, I want the scrollbar to continue working in browse mode, so that the refactoring does not break existing functionality.

#### Acceptance Criteria

1. WHEN the application displays lists in browse mode THEN the system SHALL render the scrollbar using the Scrollbar component
2. WHEN the scrollbar is rendered using the Scrollbar component THEN the system SHALL produce visually identical output to the previous implementation
3. WHEN the user scrolls through lists THEN the system SHALL update the scrollbar indicator position correctly
4. WHEN lists have different sizes THEN the system SHALL adjust scrollbar indicator height appropriately
5. WHEN all items fit on screen THEN the system SHALL hide the scrollbar
