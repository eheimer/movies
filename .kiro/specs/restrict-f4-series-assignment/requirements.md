# Requirements Document

## Introduction

This feature addresses a usability issue where the [F4] "assign to series" functionality is available for episodes that already have a series assigned. When a user presses [F4] on an episode that already belongs to a series and selects a different series, it causes problems with the episode's existing series/season relationships. This enhancement restricts the [F4] functionality to only be available for episodes without an existing series assignment, preventing accidental reassignment issues. Users who need to reassign an episode to a different series will first need to clear the series data using the functionality from issue #4, then use [F4] to assign to a new series.

## Glossary

- **Episode**: An individual video file entry in the database that may be standalone or part of a series
- **Series**: A collection of related episodes (e.g., a TV show)
- **Season**: An organizational unit within a series containing episodes
- **Browse Mode**: The primary application mode where users navigate through entries
- **Series Assignment**: The process of linking an episode to a series via the [F4] key
- **Application**: The terminal-based video library manager system
- **User**: A person interacting with the terminal application

## Requirements

### Requirement 1

**User Story:** As a user, I want the [F4] key to only trigger series assignment for episodes without an existing series, so that I avoid accidentally corrupting episode data by reassigning to a different series.

#### Acceptance Criteria

1. WHEN THE User presses [F4] in Browse Mode AND the selected entry is an Episode with no series assigned, THE Application SHALL enter SeriesSelect mode
2. WHEN THE User presses [F4] in Browse Mode AND the selected entry is an Episode with a series already assigned, THE Application SHALL remain in Browse Mode without entering SeriesSelect mode
3. WHEN THE User presses [F4] in Browse Mode AND the selected entry is not an Episode, THE Application SHALL remain in Browse Mode without entering SeriesSelect mode

### Requirement 2

**User Story:** As a user, I want visual feedback indicating whether [F4] series assignment is available for the selected episode, so that I understand why the key press has no effect on episodes already assigned to a series.

#### Acceptance Criteria

1. WHEN THE Application displays the Browse Mode interface AND the selected entry is an Episode with no series assigned, THE Application SHALL display the [F4] key hint in the interface
2. WHEN THE Application displays the Browse Mode interface AND the selected entry is an Episode with a series already assigned, THE Application SHALL not display the [F4] key hint in the interface
3. WHEN THE Application displays the Browse Mode interface AND the selected entry is not an Episode, THE Application SHALL not display the [F4] key hint in the interface

### Requirement 3

**User Story:** As a user, I want the [F5] repeat action functionality to continue working correctly, so that I can still efficiently assign multiple unassigned episodes to the same series.

#### Acceptance Criteria

1. WHEN THE User presses [F5] in Browse Mode AND the last action was a series assignment AND the selected entry is an Episode with no series assigned, THE Application SHALL assign the Episode to the same series as the last action
2. WHEN THE User presses [F5] in Browse Mode AND the last action was a series assignment AND the selected entry is an Episode with a series already assigned, THE Application SHALL not modify the Episode's series assignment
3. WHEN THE User presses [F5] in Browse Mode AND the last action was a season assignment AND the selected entry is an Episode with no series assigned, THE Application SHALL assign the Episode to the same series and season as the last action
