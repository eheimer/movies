# Requirements Document

## Introduction

This feature addresses the current behavior where adding an episode to a series or season automatically navigates the user to that series or season view. The desired behavior is to maintain the user's current view level after the episode assignment operation, providing a more predictable and less disruptive user experience.

## Glossary

- **Video Browser**: The terminal-based application that manages video file organization
- **Episode**: An individual video file that can be standalone or part of a series
- **Series**: A collection of related episodes (e.g., TV shows)
- **Season**: An organizational unit within a series
- **View Level**: The current browsing context in the application (e.g., all series, episodes within a series, episodes within a season)
- **Episode Assignment**: The operation of adding an episode to a series or season
- **Browse Mode**: The application mode where users navigate through series, seasons, and episodes

## Requirements

### Requirement 1

**User Story:** As a user organizing my video library, I want to remain at my current view level after adding an episode to a series, so that I can continue organizing multiple episodes without losing my place.

#### Acceptance Criteria

1. WHEN the user adds an episode to a series from the all-episodes view, THE Video Browser SHALL maintain the all-episodes view after the operation completes
2. WHEN the user adds an episode to a series from a season view, THE Video Browser SHALL maintain the current season view after the operation completes
3. WHEN the user adds an episode to a series from a series view, THE Video Browser SHALL maintain the current series view after the operation completes
4. WHEN the episode assignment operation completes, THE Video Browser SHALL refresh the current view to reflect the updated episode data
5. WHEN the episode assignment operation completes, THE Video Browser SHALL maintain the user's cursor position on the same episode or the nearest valid entry

### Requirement 2

**User Story:** As a user organizing my video library, I want to remain at my current view level after adding an episode to a season, so that I can efficiently batch-organize episodes without navigation interruptions.

#### Acceptance Criteria

1. WHEN the user adds an episode to a season from the all-episodes view, THE Video Browser SHALL maintain the all-episodes view after the operation completes
2. WHEN the user adds an episode to a season from a different season view, THE Video Browser SHALL maintain the current season view after the operation completes
3. WHEN the user adds an episode to a season from a series view, THE Video Browser SHALL maintain the current series view after the operation completes
4. WHEN the episode assignment to season operation completes, THE Video Browser SHALL update the displayed episode information without changing the view level

### Requirement 3

**User Story:** As a user, I want the application to provide consistent navigation behavior across all episode assignment operations, so that I can predict how the interface will respond to my actions.

#### Acceptance Criteria

1. THE Video Browser SHALL apply the same view-level preservation behavior for both series and season assignment operations
2. WHEN any episode assignment operation fails, THE Video Browser SHALL maintain the current view level and display an error message
3. THE Video Browser SHALL preserve the current view level regardless of whether the assigned series or season is currently visible in the view
4. WHEN the user performs multiple consecutive episode assignments, THE Video Browser SHALL maintain the same view level throughout all operations
