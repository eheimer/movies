# Requirements Document

**GitHub Issue:** #34

## Introduction

This feature enhances the visual readability of the terminal UI by improving the display of watched/unwatched counts for series and seasons, and introducing comprehensive text styling configuration options. The current implementation displays counts in a way that makes them difficult to distinguish from series/season names, reducing overall readability. This feature will provide better visual separation, more compact formatting, and user-configurable styling for all text types in the application.

## Glossary

- **Application**: The terminal-based video library manager system
- **Series**: A collection of seasons representing a TV show or video series
- **Season**: A collection of episodes within a series
- **Episode**: An individual video file entry
- **Watched Count**: The number of episodes marked as watched within a series or season
- **Total Count**: The total number of episodes within a series or season
- **Text Style**: Visual formatting attributes including color, italic, bold, and underline
- **Configuration File**: The `config.yaml` file that stores user preferences
- **Display Module**: The component responsible for rendering UI elements to the terminal

## Requirements

### Requirement 1

**User Story:** As a user, I want watched/unwatched counts to be visually distinct from series and season names, so that I can quickly scan and understand the status of my library.

#### Acceptance Criteria

1. WHEN the Application displays a series or season entry THEN the Application SHALL render the watched/unwatched count in a different color from the entry name
2. WHEN the Application displays a series or season entry THEN the Application SHALL apply italic styling to the watched/unwatched count text
3. WHEN the Application displays a series or season entry THEN the Application SHALL format the count as `<watched>/<total> watched` instead of the previous format
4. WHEN the Application displays a series or season entry THEN the Application SHALL right-justify the count text instead of placing it directly after the entry name
5. WHEN the Application renders the count display THEN the Application SHALL maintain proper spacing between the entry name and the right-justified count

### Requirement 2

**User Story:** As a user, I want to configure text styles for all UI elements, so that I can customize the appearance of the application to my preferences.

#### Acceptance Criteria

1. WHEN the Configuration File is loaded THEN the Application SHALL read text style settings for all defined text types
2. WHEN a text style configuration is missing THEN the Application SHALL use sensible default styling for that text type
3. WHEN the Application renders any text element THEN the Application SHALL apply the configured style attributes from the Configuration File
4. WHEN the Configuration File contains invalid style values THEN the Application SHALL log a warning and use default styling for that text type
5. WHEN the user modifies the Configuration File THEN the Application SHALL apply the new styles on the next launch

### Requirement 3

**User Story:** As a developer, I want a comprehensive set of configurable text types, so that users have fine-grained control over the application's appearance.

#### Acceptance Criteria

1. WHEN defining text style configuration options THEN the Application SHALL support styles for series names, season names, episode names, watched counts, selected items, headers, help text, and status messages
2. WHEN defining text style attributes THEN the Application SHALL support color, italic, bold, and underline properties for each text type
3. WHEN the Application initializes THEN the Application SHALL validate that all text type configurations conform to the expected schema
4. WHEN rendering different text types THEN the Application SHALL apply the appropriate style configuration to each text element
5. WHEN multiple style attributes are configured for a text type THEN the Application SHALL combine all attributes correctly in the rendered output

### Requirement 4

**User Story:** As a user, I want the count display to be more compact, so that I can see more information on screen without clutter.

#### Acceptance Criteria

1. WHEN the Application calculates the display width for a series or season entry THEN the Application SHALL account for the new compact count format
2. WHEN the terminal width is limited THEN the Application SHALL ensure the count display does not cause text wrapping or overflow
3. WHEN displaying entries with varying name lengths THEN the Application SHALL maintain consistent right-justification of count text
4. WHEN the count text is rendered THEN the Application SHALL use minimal spacing while maintaining readability
