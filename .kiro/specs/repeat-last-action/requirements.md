# Requirements Document

## Introduction

This feature enables users to quickly repeat their last series or season assignment action using the F5 key. The system will track the most recent assignment operation and display it on screen, allowing users to efficiently batch-assign multiple episodes to the same series or season without navigating through menus repeatedly.

## Glossary

- **Application**: The terminal-based video file browser and library manager
- **User**: A person interacting with the Application
- **Episode**: An individual video file entry in the Application
- **Series**: A collection of related Episodes
- **Season**: An organizational unit within a Series
- **Assignment Action**: The operation of associating an Episode with a Series or Season
- **Last Action State**: The stored information about the most recently performed Assignment Action
- **Selected Item**: The Episode currently highlighted by the User in the browse interface
- **Action Display**: The text line showing the Last Action State between the menu and filter line

## Requirements

### Requirement 1

**User Story:** As a user, I want to repeat my last series assignment using F5, so that I can quickly add multiple episodes to the same series without navigating menus each time

#### Acceptance Criteria

1. WHEN the User presses F5 AND the Last Action State contains a series assignment AND the Selected Item is not already assigned to that series, THE Application SHALL assign the Selected Item to the series stored in the Last Action State
2. WHEN the User assigns an Episode to a Series, THE Application SHALL update the Last Action State to store the series assignment with the series identifier
3. IF the User presses F5 AND the Selected Item is already assigned to the series in the Last Action State, THEN THE Application SHALL not perform any assignment operation
4. IF the User presses F5 AND no Last Action State exists, THEN THE Application SHALL not perform any assignment operation

### Requirement 2

**User Story:** As a user, I want to repeat my last season assignment using F5, so that I can quickly add multiple episodes from the same series to the same season

#### Acceptance Criteria

1. WHEN the User presses F5 AND the Last Action State contains a season assignment AND the Selected Item belongs to the same series as the Last Action State AND the Selected Item is not already assigned to that season, THE Application SHALL assign the Selected Item to the season stored in the Last Action State
2. WHEN the User presses F5 AND the Last Action State contains a season assignment AND the Selected Item does not belong to any series, THE Application SHALL assign the Selected Item to both the series and the season stored in the Last Action State
3. WHEN the User assigns an Episode to a Season, THE Application SHALL update the Last Action State to store the season assignment with both the season identifier and series identifier
4. IF the User presses F5 AND the Last Action State contains a season assignment AND the Selected Item belongs to a different series than the Last Action State, THEN THE Application SHALL not perform any assignment operation
5. IF the User presses F5 AND the Selected Item is already assigned to the season in the Last Action State, THEN THE Application SHALL not perform any assignment operation

### Requirement 3

**User Story:** As a user, I want to see what my last action was displayed on screen, so that I know what will happen when I press F5

#### Acceptance Criteria

1. WHEN the Last Action State contains a series assignment AND the Selected Item can be assigned to that series, THE Application SHALL display the text "Last action: [Series Name]" on the Action Display line
2. WHEN the Last Action State contains a season assignment AND the Selected Item can be assigned to that season, THE Application SHALL display the text "Last action: [Series Name], Season [Season Number]" on the Action Display line
3. WHEN no Last Action State exists OR the Selected Item cannot be assigned using the Last Action State, THE Application SHALL display an empty Action Display line
4. THE Application SHALL position the Action Display line between the menu line and the filter line
5. WHEN the Last Action State changes OR the Selected Item changes, THE Application SHALL update the Action Display line immediately

### Requirement 4

**User Story:** As a user, I want the F5 action to only work on valid items, so that I don't accidentally create invalid assignments

#### Acceptance Criteria

1. WHEN the User presses F5 AND the Selected Item is a Series or Season entry, THE Application SHALL not perform any assignment operation
2. WHEN the User presses F5 AND the Selected Item is an Episode entry, THE Application SHALL validate the Episode against the Last Action State before performing assignment
3. IF the User presses F5 AND the validation fails for any reason, THEN THE Application SHALL not modify the database
4. WHEN the User successfully repeats an action with F5, THE Application SHALL refresh the display to show the updated assignment

### Requirement 5

**User Story:** As a user, I want to see the F5 repeat action option in the menu only when it's available, so that I know when I can use this feature

#### Acceptance Criteria

1. WHEN the Last Action State exists AND the Selected Item can be assigned using the Last Action State, THE Application SHALL display "[F5] Repeat action" in the menu
2. WHEN no Last Action State exists OR the Selected Item cannot be assigned using the Last Action State, THE Application SHALL not display "[F5] Repeat action" in the menu
3. WHEN the Last Action State changes OR the Selected Item changes, THE Application SHALL update the menu display immediately
