# Requirements Document

**GitHub Issue:** #52

## Introduction

This specification defines the Category component for the movies terminal application. The Category component will be used to render both series and season entries in the browse interface, and can eventually be extended for any hierarchical categorization display. This component builds on the component framework established in issue #51, implementing the Component trait to produce Cell arrays for terminal rendering.

The Category component displays a single-line entry showing the category title, total episode count, and watched episode count. It will replace the current inline rendering logic for series and season entries, maintaining visual consistency with the existing implementation while enabling better testability and reusability.

## Glossary

- **Category Component**: A component that renders hierarchical category information (series or season) with episode counts
- **Component**: A self-contained rendering unit that implements the Component trait and produces a 2D array of terminal cells
- **Cell**: A single terminal character position containing a character and styling information
- **Episode Count**: The total number of episodes within a category (series or season)
- **Watched Count**: The number of episodes marked as watched within a category
- **Series**: A top-level category representing a TV show or video series
- **Season**: A second-level category representing a season within a series
- **Browse Mode**: The primary application mode where users navigate through series, seasons, and episodes
- **Theme**: Configuration object containing color and style preferences

## Requirements

### Requirement 1

**User Story:** As a developer, I want a Category component that renders category information, so that I can replace the current inline rendering logic for series and season entries.

#### Acceptance Criteria

1. WHEN the Category component is created THEN the system SHALL store the category title
2. WHEN the Category component is created THEN the system SHALL store the total episode count
3. WHEN the Category component is created THEN the system SHALL store the watched episode count
4. WHEN the Category component is created THEN the system SHALL store the category type (series or season)
5. WHEN the Category component implements the Component trait THEN the system SHALL provide a render method that returns a 2D Cell array

### Requirement 2

**User Story:** As a user, I want series and season entries to display their title and episode counts, so that I can see how many episodes are in each category and how many I've watched.

#### Acceptance Criteria

1. WHEN a Category component renders THEN the system SHALL display the category title
2. WHEN a Category component renders THEN the system SHALL display the total episode count in the format "(X episodes)"
3. WHEN a Category component renders THEN the system SHALL display the watched count in the format "[Y watched]"
4. WHEN a Category component renders with zero watched episodes THEN the system SHALL omit the watched count display
5. WHEN a Category component renders THEN the system SHALL format the output as: "Title (X episodes) [Y watched]"

### Requirement 3

**User Story:** As a user, I want category entries to be visually consistent with the current implementation, so that the refactoring does not change the user experience.

#### Acceptance Criteria

1. WHEN a series entry is rendered using the Category component THEN the system SHALL produce output identical to the current series rendering
2. WHEN a season entry is rendered using the Category component THEN the system SHALL produce output identical to the current season rendering
3. WHEN a Category component renders with selection state true THEN the system SHALL apply current selection colors from theme
4. WHEN a Category component renders with selection state false THEN the system SHALL apply default category colors from theme
5. WHEN a Category component renders THEN the system SHALL truncate text to fit within the specified width

### Requirement 4

**User Story:** As a developer, I want the Category component to follow the established component architecture, so that it integrates seamlessly with the existing component framework.

#### Acceptance Criteria

1. WHEN the Category component is implemented THEN the system SHALL place it in the src/components directory
2. WHEN the Category component is implemented THEN the system SHALL implement the Component trait
3. WHEN the Category component renders THEN the system SHALL accept width constraints as parameters
4. WHEN the Category component renders THEN the system SHALL accept theme data as parameters
5. WHEN the Category component renders THEN the system SHALL accept selection state as a parameter
6. WHEN the Category component renders THEN the system SHALL not perform direct terminal I/O operations

### Requirement 5

**User Story:** As a user, I want the browse mode to continue working correctly with series and season entries, so that the refactoring does not break existing functionality.

#### Acceptance Criteria

1. WHEN the application displays series entries in browse mode THEN the system SHALL use the Category component for rendering
2. WHEN the application displays season entries in browse mode THEN the system SHALL use the Category component for rendering
3. WHEN the user navigates through categories THEN the system SHALL highlight the selected category correctly
4. WHEN the user filters entries THEN the system SHALL display filtered category results correctly
5. WHEN the user enters a series or season THEN the system SHALL display the appropriate child entries

### Requirement 6

**User Story:** As a developer, I want the Category component to be testable independently, so that I can verify its behavior without terminal interaction.

#### Acceptance Criteria

1. WHEN the Category component renders THEN the system SHALL produce Cell arrays that can be verified programmatically
2. WHEN the Category component is tested THEN the system SHALL allow verification of title formatting
3. WHEN the Category component is tested THEN the system SHALL allow verification of episode count formatting
4. WHEN the Category component is tested THEN the system SHALL allow verification of watched count formatting
5. WHEN the Category component is tested THEN the system SHALL allow verification of color application
