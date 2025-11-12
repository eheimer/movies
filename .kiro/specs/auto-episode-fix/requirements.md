# Requirements Document

## Introduction

This document specifies the requirements for fixing a bug in the automatic episode number entry feature when entering edit mode. Currently, the system auto-fills episode numbers without checking if a season is assigned, and fails to mark the field as dirty when auto-filling occurs.

## Glossary

- **Edit Mode**: The application mode where episode metadata can be modified
- **Episode Number**: A numeric field identifying an episode's position within a season
- **Season**: An organizational unit within a series that groups related episodes
- **Dirty Field**: A field that has been modified from its original value, requiring save confirmation
- **Auto-fill**: The automatic population of the episode number field when entering edit mode

## Requirements

### Requirement 1

**User Story:** As a user, I want the episode number auto-fill to only occur when a season is assigned, so that standalone episodes without seasons don't get incorrect episode numbers.

#### Acceptance Criteria

1. WHEN the user enters edit mode for an episode, IF the episode has no season assigned, THEN the System SHALL NOT auto-fill the episode number field
2. WHEN the user enters edit mode for an episode, IF the episode has a season assigned AND the episode number is empty or zero, THEN the System SHALL auto-fill the episode number with the next available number for that season
3. WHEN the user enters edit mode for an episode, IF the episode has a season assigned AND the episode number is already set to a non-zero value, THEN the System SHALL NOT modify the episode number field

### Requirement 2

**User Story:** As a user, I want auto-filled episode numbers to be marked as dirty, so that I can see which fields have been modified and need to be saved.

#### Acceptance Criteria

1. WHEN the System auto-fills the episode number field upon entering edit mode, THEN the System SHALL mark the EpisodeNumber field as dirty
2. WHEN the System marks the EpisodeNumber field as dirty, THEN the System SHALL add EpisodeField::EpisodeNumber to the dirty_fields collection
3. WHEN the user saves the episode in edit mode, THEN the System SHALL persist the auto-filled episode number to the database
