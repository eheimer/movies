# Requirements Document

## Introduction

This document specifies the requirements for fixing a UI overflow bug in the series selection window. Currently, when the list of available series exceeds the available screen space, the series list overflows its window boundaries instead of implementing scrolling behavior. This fix will implement scrolling functionality similar to the existing episode list behavior in browse mode.

## Glossary

- **SeriesSelectWindow**: The modal dialog window that displays when a user assigns an episode to a series, showing all available series in the database
- **ViewportHeight**: The maximum number of series items that can be displayed within the SeriesSelectWindow boundaries at one time
- **ScrollOffset**: The index of the first series item currently visible in the SeriesSelectWindow
- **TerminalHeight**: The total number of rows available in the terminal display
- **BrowseMode**: The primary application mode where users navigate through episodes, series, and seasons
- **SeriesSelectMode**: The application mode activated when assigning an episode to a series

## Requirements

### Requirement 1

**User Story:** As a user with many series in my database, I want the series selection window to display only the series that fit within the window boundaries, so that the UI remains properly formatted and readable.

#### Acceptance Criteria

1. WHEN THE SeriesSelectWindow is displayed, THE SeriesSelectWindow SHALL calculate the ViewportHeight based on available TerminalHeight
2. WHEN the total number of series exceeds the ViewportHeight, THE SeriesSelectWindow SHALL display only the series items that fit within the window boundaries
3. THE SeriesSelectWindow SHALL reserve space for the window title and borders when calculating ViewportHeight
4. THE SeriesSelectWindow SHALL prevent any series list content from rendering outside the window boundaries

### Requirement 2

**User Story:** As a user navigating through a long list of series, I want the window to scroll automatically as I move the selection cursor, so that I can access all series without the list overflowing.

#### Acceptance Criteria

1. WHEN the user presses the down arrow key AND the selected series is below the visible viewport, THE SeriesSelectWindow SHALL increment the ScrollOffset to bring the selected series into view
2. WHEN the user presses the up arrow key AND the selected series is above the visible viewport, THE SeriesSelectWindow SHALL decrement the ScrollOffset to bring the selected series into view
3. WHEN the selected series is within the visible viewport, THE SeriesSelectWindow SHALL maintain the current ScrollOffset
4. THE SeriesSelectWindow SHALL ensure the selected series remains visible at all times during navigation

### Requirement 3

**User Story:** As a user, I want the series selection scrolling behavior to work consistently with the episode list scrolling in browse mode, so that the interface feels cohesive and predictable.

#### Acceptance Criteria

1. THE SeriesSelectWindow SHALL implement scrolling logic equivalent to the scrolling logic used in BrowseMode for episode lists
2. WHEN the user navigates to a series below the current viewport, THE SeriesSelectWindow SHALL scroll the viewport to show the selected series
3. WHEN the user navigates to a series above the current viewport, THE SeriesSelectWindow SHALL scroll the viewport to show the selected series
4. THE SeriesSelectWindow SHALL use the same scrolling algorithm pattern as the `first_entry` tracking mechanism in BrowseMode

### Requirement 4

**User Story:** As a user, I want the series selection window to adapt to different terminal sizes, so that the scrolling behavior works correctly regardless of my terminal configuration.

#### Acceptance Criteria

1. THE SeriesSelectWindow SHALL recalculate ViewportHeight dynamically based on current TerminalHeight
2. WHEN the TerminalHeight changes, THE SeriesSelectWindow SHALL adjust the ViewportHeight accordingly
3. THE SeriesSelectWindow SHALL maintain a minimum ViewportHeight of 1 series item to ensure at least one series is always visible
4. THE SeriesSelectWindow SHALL account for the window's fixed UI elements (title, borders) when calculating available space for series items

### Requirement 5

**User Story:** As a user familiar with Vim keybindings, I want to use 'j' and 'k' keys to navigate through the series list, so that I can use consistent navigation patterns throughout the application.

#### Acceptance Criteria

1. WHEN the user presses the 'j' key in SeriesSelectMode, THE SeriesSelectWindow SHALL move the selection down by one series item
2. WHEN the user presses the 'k' key in SeriesSelectMode, THE SeriesSelectWindow SHALL move the selection up by one series item
3. THE SeriesSelectWindow SHALL apply the same scrolling behavior for 'j' and 'k' keys as it does for arrow keys
4. THE SeriesSelectWindow SHALL support both arrow key navigation and Vim-style navigation simultaneously
