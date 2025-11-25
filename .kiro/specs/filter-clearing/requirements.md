# Requirements Document

## Introduction

This specification addresses a bug in the browse mode filtering behavior. Currently, when users apply a filter by typing text in browse mode, the filter text persists when navigating into series/seasons or when pressing ESC. This creates a confusing user experience where the displayed list appears filtered without visible indication. The filter text should be automatically cleared in specific navigation scenarios to ensure users see unfiltered content when entering new contexts or canceling filter operations.

## Glossary

- **Browser**: The terminal-based video library application
- **Browse Mode**: The primary navigation mode where users can view and select entries
- **Filter Text**: Text entered by the user that filters the displayed list of entries
- **Series Entry**: A collection of related episodes organized as a TV series
- **Season Entry**: An organizational unit within a series containing episodes
- **Episode Entry**: An individual video file that may be standalone or part of a series
- **Navigation Context**: The current view level (TopLevel, Series view, or Season view)

## Requirements

### Requirement 1

**User Story:** As a user browsing filtered content, I want the filter to clear when I navigate into a series, so that I see all seasons and episodes without the previous filter applied

#### Acceptance Criteria

1. WHEN the user presses ENTER on a Series Entry while filter text is present, THE Browser SHALL clear the filter text
2. WHEN the user presses ENTER on a Series Entry while filter text is present, THE Browser SHALL display all seasons and episodes within that series without filtering
3. WHEN the user navigates into a Series Entry with no filter text present, THE Browser SHALL maintain the existing behavior without modification

### Requirement 2

**User Story:** As a user browsing filtered content, I want the filter to clear when I navigate into a season, so that I see all episodes in that season without the previous filter applied

#### Acceptance Criteria

1. WHEN the user presses ENTER on a Season Entry while filter text is present, THE Browser SHALL clear the filter text
2. WHEN the user presses ENTER on a Season Entry while filter text is present, THE Browser SHALL display all episodes within that season without filtering
3. WHEN the user navigates into a Season Entry with no filter text present, THE Browser SHALL maintain the existing behavior without modification

### Requirement 3

**User Story:** As a user who has entered filter text, I want the filter to clear when I press ESC to navigate back, so that the previous navigation context displays unfiltered content

#### Acceptance Criteria

1. WHEN the user presses ESC while filter text is present in Browse Mode, THE Browser SHALL clear the filter text
2. WHEN the user presses ESC while filter text is present in Browse Mode, THE Browser SHALL execute the normal ESC navigation behavior (navigate from Season to Series, from Series to TopLevel, or exit the application)
3. WHEN the user presses ESC while filter text is present in Browse Mode, THE Browser SHALL display all entries in the resulting navigation context without filtering
4. WHEN the user presses ESC while no filter text is present in Browse Mode, THE Browser SHALL maintain the existing navigation behavior without modification

### Requirement 4

**User Story:** As a user navigating with filters, I want consistent filter clearing behavior across all navigation actions, so that I have a predictable and intuitive experience

#### Acceptance Criteria

1. WHEN filter text is cleared by any navigation action, THE Browser SHALL update the filtered entries list to match the full entries list
2. WHEN filter text is cleared by any navigation action, THE Browser SHALL trigger a display redraw to show the updated list
3. WHEN the user navigates into an Episode Entry while filter text is present, THE Browser SHALL NOT clear the filter text
