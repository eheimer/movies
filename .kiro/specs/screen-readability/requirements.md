# Requirements Document

**GitHub Issue:** #32

## Introduction

This feature enhances the visual presentation of the terminal-based video library manager to improve readability and user experience. The application currently uses minimal color coding (blue for series/seasons, yellow highlight for current selection). This enhancement will add configurable color schemes and visual indicators to make the interface more welcoming and informative while maintaining the clean terminal aesthetic.

## Glossary

- **Application**: The terminal-based video library manager system
- **Episode**: An individual video file entry that may be standalone or part of a series
- **Series**: A collection of related episodes organized into seasons
- **Season**: An organizational unit within a series containing episodes
- **Invalid Episode**: An episode with missing or incomplete metadata (e.g., missing title, year, or episode number when part of a series)
- **Watched Status**: A boolean indicator showing whether an episode has been viewed
- **Current Selection**: The currently highlighted item in the browse list
- **Config File**: The config.json file containing user preferences and color settings
- **Terminal UI**: The text-based user interface rendered in the terminal
- **Color Scheme**: A set of foreground and background color combinations for different UI elements

## Requirements

### Requirement 1

**User Story:** As a user, I want to see visual indicators for watched episodes, so that I can quickly identify which videos I have already viewed.

#### Acceptance Criteria

1. WHEN an episode is marked as watched THEN the Application SHALL display a visual indicator next to the episode name in the browse list
2. WHEN an episode is marked as unwatched THEN the Application SHALL display no watched indicator or display an unwatched indicator
3. WHEN the user toggles the watched status THEN the Application SHALL immediately update the visual indicator
4. WHEN displaying the watched indicator THEN the Application SHALL use a character or symbol that is clearly distinguishable from the episode name

### Requirement 2

**User Story:** As a user, I want to see visual indicators for new and invalid episodes, so that I can quickly identify which entries need attention.

#### Acceptance Criteria

1. WHEN an episode title is identical to its filename THEN the Application SHALL classify it as new and display it with a distinct color scheme
2. WHEN the video file referenced by an episode does not exist on disk THEN the Application SHALL classify it as invalid and display it with a distinct color scheme
3. WHEN displaying new episodes THEN the Application SHALL use colors that are configurable in the config file with a default of green
4. WHEN displaying invalid episodes THEN the Application SHALL use colors that are configurable in the config file with a default of red
5. WHEN a new episode title is edited to differ from the filename THEN the Application SHALL immediately remove the new indicator
6. WHEN an invalid episode file is restored or the path is corrected THEN the Application SHALL immediately remove the invalid indicator

### Requirement 3

**User Story:** As a user, I want to configure the colors used throughout the application, so that I can customize the interface to my preferences and terminal theme.

#### Acceptance Criteria

1. WHEN the Application reads the config file THEN the Application SHALL load all color settings for UI elements
2. WHEN a color setting is missing from the config file THEN the Application SHALL use a default color value
3. WHEN the user modifies the config file THEN the Application SHALL apply the new colors on the next application start
4. WHEN invalid color names are specified THEN the Application SHALL fall back to default colors
5. WHEN the Application writes default config THEN the Application SHALL include all color configuration options with sensible defaults

### Requirement 4

**User Story:** As a user, I want series and season entries to be visually distinct from episodes, so that I can quickly understand the hierarchy and organization of my library.

#### Acceptance Criteria

1. WHEN displaying a series entry THEN the Application SHALL use a distinct color scheme for the series name
2. WHEN displaying a season entry THEN the Application SHALL use a distinct color scheme for the season label
3. WHEN displaying an episode entry THEN the Application SHALL use a distinct color scheme for the episode name
4. WHEN the user selects a series or season THEN the Application SHALL apply the current selection highlight colors
5. WHEN series and season colors are configured THEN the Application SHALL read them from the config file

### Requirement 5

**User Story:** As a user, I want the filter input and menu to be visually clear, so that I can easily see when I am in filter mode or viewing the menu.

#### Acceptance Criteria

1. WHEN the user activates filter mode THEN the Application SHALL highlight the filter label with distinct colors
2. WHEN the user types in filter mode THEN the Application SHALL display the filter text clearly
3. WHEN the user opens the context menu THEN the Application SHALL display the menu with a distinct border style
4. WHEN the menu is displayed THEN the Application SHALL use colors that make menu items easily readable
5. WHEN a menu item is selected THEN the Application SHALL highlight it using the current selection colors

### Requirement 6

**User Story:** As a user, I want the status line at the bottom of the screen to be visually distinct, so that I can easily see status messages and feedback.

#### Acceptance Criteria

1. WHEN the Application displays a status message THEN the Application SHALL render it in the status line at the bottom of the terminal
2. WHEN the status line is empty THEN the Application SHALL display it with a neutral background
3. WHEN the status line contains a message THEN the Application SHALL use colors that make the message easily readable
4. WHEN status line colors are configured THEN the Application SHALL read them from the config file
5. WHEN the terminal is resized THEN the Application SHALL adjust the status line width accordingly

### Requirement 7

**User Story:** As a developer, I want a consistent color configuration system, so that adding new color-coded UI elements is straightforward and maintainable.

#### Acceptance Criteria

1. WHEN adding a new color configuration option THEN the Application SHALL define it in the Config struct with a default value
2. WHEN the config file is missing a color field THEN the Application SHALL populate it with the default value
3. WHEN parsing color strings THEN the Application SHALL support standard color names (black, red, green, yellow, blue, magenta, cyan, white)
4. WHEN an unsupported color name is encountered THEN the Application SHALL log a warning and use the default color
5. WHEN color configuration is accessed THEN the Application SHALL provide helper functions for consistent color application
