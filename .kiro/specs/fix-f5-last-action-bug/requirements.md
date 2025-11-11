# Requirements Document

## Introduction

This specification addresses a bug in the video library manager where the F5 "repeat last action" functionality is not available after creating a new series and assigning an episode to it. The system correctly tracks the last action when assigning episodes to existing series, but fails to update the last action properties when a new series is created during the assignment process.

## Glossary

- **Episode**: An individual video file that can be standalone or part of a series
- **Series**: A collection of related episodes (e.g., TV shows)
- **Season**: An organizational unit within a series
- **Last Action**: A tracked state that stores the most recent series or season assignment operation
- **F5 Key**: The keyboard shortcut that repeats the last series or season assignment action
- **SeriesCreate Mode**: The application mode where users create a new series and assign an episode to it
- **SeriesSelect Mode**: The application mode where users select an existing series to assign an episode to
- **Edit Mode**: The application mode where users edit episode details including series and season assignments
- **Application**: The terminal-based video library manager system

## Requirements

### Requirement 1

**User Story:** As a user organizing my video library, I want the F5 repeat action to work after creating a new series, so that I can quickly assign multiple episodes to the newly created series without re-entering the series name.

#### Acceptance Criteria

1. WHEN a user creates a new series via SeriesCreate Mode and assigns an episode to it, THE Application SHALL update the last action state with the series assignment details.

2. WHEN a user creates a new series and assigns an episode to it, THE Application SHALL store the series ID and series name in the last action state.

3. WHEN a user presses F5 after creating a new series and assigning an episode, THE Application SHALL display the last action information in the UI.

4. WHEN a user presses F5 on an unassigned episode after creating a new series, THE Application SHALL assign the selected episode to the newly created series.

5. WHEN a user creates a season assignment during Edit Mode for a newly created series, THE Application SHALL update the last action state with the season assignment details including series ID, series name, season ID, and season number.

### Requirement 2

**User Story:** As a user, I want consistent behavior for the F5 repeat action regardless of whether I selected an existing series or created a new one, so that the interface is predictable and efficient.

#### Acceptance Criteria

1. WHEN a user assigns an episode to an existing series via SeriesSelect Mode, THE Application SHALL update the last action state identically to when creating a new series.

2. WHEN a user assigns an episode to a season in Edit Mode, THE Application SHALL update the last action state with season assignment details regardless of whether the series was newly created or pre-existing.

3. WHEN the last action state is updated, THE Application SHALL maintain the state until a new series or season assignment occurs.

4. WHEN a user navigates to a different episode after any series or season assignment, THE Application SHALL preserve the last action state for F5 functionality.
