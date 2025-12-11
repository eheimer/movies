# Requirements Document

**GitHub Issue:** #56

## Introduction

This specification defines the refactoring of the Component trait to accept dimensions (width, height) instead of just width. This change will simplify component implementations by removing internal height management and prepare the foundation for horizontal scrollbars. The refactoring affects the Component trait interface and all existing component implementations (Episode, Category, Browser, Scrollbar).

## Glossary

- **Component**: A self-contained rendering unit that implements the Component trait
- **Dimensions**: A tuple or struct containing width and height values for rendering constraints
- **Component Trait**: The interface that all renderable components must implement
- **Browser Component**: A multi-line component that displays lists of entries with scrolling
- **Single-line Component**: Components like Category and Episode that render to a single row
- **Scrollbar Component**: A component that renders vertical scrollbars for list navigation
- **Render Method**: The primary method in the Component trait that produces terminal output

## Requirements

### Requirement 1

**User Story:** As a developer, I want the Component trait to accept dimensions instead of just width, so that components receive complete layout information.

#### Acceptance Criteria

1. WHEN the Component trait is updated THEN the render method SHALL accept width and height parameters
2. WHEN the Component trait is updated THEN the render method SHALL maintain backward compatibility for theme and selection parameters
3. WHEN the Component trait is updated THEN the method signature SHALL be consistent across all implementations
4. WHEN components implement the updated trait THEN the system SHALL provide both width and height constraints
5. WHEN the render method is called THEN the system SHALL pass valid positive dimensions

### Requirement 2

**User Story:** As a developer, I want single-line components to ignore height gracefully, so that Episode and Category components remain simple.

#### Acceptance Criteria

1. WHEN the Episode component receives height parameter THEN the system SHALL ignore the height value
2. WHEN the Category component receives height parameter THEN the system SHALL ignore the height value
3. WHEN single-line components render THEN the system SHALL return exactly one row regardless of height
4. WHEN single-line components render THEN the system SHALL respect the width constraint
5. WHEN single-line components render THEN the system SHALL maintain existing visual output

### Requirement 3

**User Story:** As a developer, I want the Browser component simplified by removing internal height management, so that layout responsibility is centralized.

#### Acceptance Criteria

1. WHEN the Browser component is updated THEN the system SHALL remove internal height calculation logic
2. WHEN the Browser component renders THEN the system SHALL use the provided height parameter
3. WHEN the Browser component renders THEN the system SHALL respect both width and height constraints
4. WHEN the Browser component renders THEN the system SHALL maintain existing scrolling behavior
5. WHEN the Browser component renders THEN the system SHALL produce output that fits within the specified dimensions

### Requirement 4

**User Story:** As a developer, I want the Scrollbar component updated to use provided dimensions, so that it prepares for horizontal scrollbar support.

#### Acceptance Criteria

1. WHEN the Scrollbar component is updated THEN the system SHALL use the provided height parameter
2. WHEN the Scrollbar component renders THEN the system SHALL respect both width and height constraints
3. WHEN the Scrollbar component renders THEN the system SHALL maintain existing vertical scrollbar functionality
4. WHEN the Scrollbar component is updated THEN the system SHALL prepare the foundation for horizontal scrollbar support
5. WHEN the Scrollbar component renders THEN the system SHALL produce output that fits within the specified dimensions

### Requirement 5

**User Story:** As a user, I want all existing functionality to continue working correctly, so that the refactoring does not break the application.

#### Acceptance Criteria

1. WHEN the application runs after refactoring THEN the system SHALL display all components correctly
2. WHEN users navigate through the interface THEN the system SHALL maintain existing behavior
3. WHEN users interact with scrollable content THEN the system SHALL scroll correctly
4. WHEN the application renders different screen sizes THEN the system SHALL adapt layout appropriately
5. WHEN components are displayed THEN the system SHALL maintain visual consistency with previous versions

### Requirement 6

**User Story:** As a developer, I want clear separation between layout calculation and component rendering, so that the system is more maintainable.

#### Acceptance Criteria

1. WHEN display code calls component render methods THEN the system SHALL provide calculated dimensions
2. WHEN components render THEN the system SHALL not perform internal dimension calculations
3. WHEN layout changes occur THEN the system SHALL recalculate dimensions at the display level
4. WHEN components are tested THEN the system SHALL allow dimension specification in tests
5. WHEN new components are added THEN the system SHALL follow the consistent dimension-passing pattern