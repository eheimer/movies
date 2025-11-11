# Requirements Document

## Introduction

This feature enhances the episode editing workflow by automatically positioning the cursor on the episode number field and pre-filling it with the next available episode number when editing an episode that is already part of a series. This streamlines the process of assigning sequential episode numbers, reducing the workflow from multiple keystrokes to just two F2 presses.

## Glossary

- **Episode Editor**: The edit mode interface where episode metadata can be modified
- **Episode Field**: Individual editable attributes of an episode (series, season, episode number, year, length, watched status)
- **Next Available Episode Number**: The lowest positive integer not currently assigned to any episode within the same series and season, starting from 1
- **Series Context**: An episode that has been assigned to a specific series (series field is not empty/null)
- **Cursor Position**: The currently selected field in the Episode Editor that will receive user input

## Requirements

### Requirement 1

**User Story:** As a user organizing episodes, I want the cursor to automatically jump to the episode number field when I edit an episode that's already in a series but doesn't have an episode number yet, so that I can quickly assign episode numbers without navigating through other fields.

#### Acceptance Criteria

1. WHEN the user enters edit mode for an episode, IF the episode has a series assigned AND the episode does not have an episode number assigned, THEN the Episode Editor SHALL position the cursor on the episode number field
2. WHEN the user enters edit mode for an episode, IF the episode does not have a series assigned OR the episode already has an episode number assigned, THEN the Episode Editor SHALL position the cursor on the first field (series field)
3. THE Episode Editor SHALL determine series assignment by checking whether the episode's series field contains a non-null, non-empty value
4. THE Episode Editor SHALL determine episode number assignment by checking whether the episode's episode number field contains a non-null, non-zero value

### Requirement 2

**User Story:** As a user assigning episode numbers sequentially, I want the episode number field to be pre-filled with the next available number, so that I can accept it with a single keypress instead of typing the number manually.

#### Acceptance Criteria

1. WHEN the Episode Editor positions the cursor on the episode number field for an episode with a series assigned AND no episode number assigned, THE Episode Editor SHALL calculate the next available episode number for that series and season
2. THE Episode Editor SHALL define the next available episode number as the lowest positive integer starting from 1 that is not currently assigned to any episode within the same series and season combination
3. WHEN calculating the next available episode number, THE Episode Editor SHALL query all episodes that match both the series name and season number of the episode being edited
4. WHEN the next available episode number is calculated, THE Episode Editor SHALL pre-fill the episode number field with this value
5. THE Episode Editor SHALL allow the user to modify or replace the pre-filled episode number value
6. WHEN the episode already has an episode number assigned, THE Episode Editor SHALL NOT calculate or pre-fill any value

### Requirement 3

**User Story:** As a user managing a large video library, I want to assign episode numbers with minimal keystrokes, so that I can efficiently organize my content.

#### Acceptance Criteria

1. WHEN the user presses the edit key (F2) on an episode with a series assigned AND no episode number assigned, and the episode number field is pre-filled with the next available number, THE Episode Editor SHALL allow the user to accept the pre-filled value by pressing the edit key (F2) again
2. THE Episode Editor SHALL complete the episode number assignment workflow within two keypresses when the pre-filled value is accepted
3. WHEN the user accepts the pre-filled episode number, THE Episode Editor SHALL save the episode number to the database and return to browse mode

### Requirement 4

**User Story:** As a user correcting episode assignments, I want the auto-fill behavior to work correctly even when there are gaps in episode numbering, so that I can fill in missing episodes accurately.

#### Acceptance Criteria

1. WHEN calculating the next available episode number, IF there are gaps in the existing episode sequence (e.g., episodes 1, 2, 4, 5 exist), THEN the Episode Editor SHALL return the lowest missing number (3 in this example)
2. WHEN calculating the next available episode number, IF all sequential numbers starting from 1 are assigned, THEN the Episode Editor SHALL return the next sequential number after the highest assigned episode
3. THE Episode Editor SHALL recalculate the next available episode number each time edit mode is entered to reflect the current database state
