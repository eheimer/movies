# Requirements Document

**GitHub Issue:** #44

## Introduction

This feature introduces a theme system for the video library manager application. Currently, all color and style configuration options are stored directly in the main `config.yaml` file. This feature will separate visual styling concerns into dedicated theme files, allowing users to easily switch between different visual themes while keeping core application configuration separate.

## Glossary

- **Application**: The terminal-based video library manager
- **Config File**: The main `config.yaml` file containing application settings
- **Theme File**: A YAML file containing only color and style configuration (format: `THEME-{name}.yaml`)
- **Theme System**: The mechanism for loading and applying theme files
- **Style Options**: Color and text styling settings including foreground/background colors, indicators, and text styles
- **Config Directory**: The directory where configuration files are stored (`~/.config/movies` on Linux)
- **Migration**: The one-time process of moving style options from existing config.yaml to a theme file

## Requirements

### Requirement 1

**User Story:** As a user, I want my visual styling options stored in a separate theme file, so that I can easily manage and switch between different visual themes.

#### Acceptance Criteria

1. WHEN the Application starts, THE Application SHALL load style options from a theme file specified in the config file
2. WHEN a theme file is missing, THE Application SHALL create a default theme file with current default values
3. WHEN the Application loads a theme file, THE Application SHALL apply all color and style settings from that file
4. THE Config File SHALL contain an `active_theme` field that specifies which theme file to load
5. THE Theme File SHALL be stored in the same directory as the Config File

### Requirement 2

**User Story:** As a user with an existing configuration, I want my current style settings automatically migrated to a theme file, so that I don't lose my customizations.

#### Acceptance Criteria

1. WHEN the Application starts with an old-format Config File containing style options, THE Application SHALL extract those style options into a new Theme File
2. WHEN migration occurs, THE Application SHALL remove the migrated style options from the Config File
3. WHEN migration occurs, THE Application SHALL set the `active_theme` field in the Config File to reference the new Theme File
4. WHEN migration completes, THE Application SHALL save both the updated Config File and the new Theme File

### Requirement 3

**User Story:** As a developer, I want the theme file structure to contain all visual styling options, so that themes can fully customize the application appearance.

#### Acceptance Criteria

1. THE Theme File SHALL contain all foreground and background color settings for entries (current, dirty, new, invalid, series, season, episode, status)
2. THE Theme File SHALL contain watched and unwatched indicator characters and their styling (color and text style)
3. THE Theme File SHALL contain scrollbar configuration (track character, indicator character, colors)
4. THE Theme File SHALL contain text styling options (count, header, help text colors and styles)
5. THE Theme File SHALL use YAML format with inline documentation comments

### Requirement 4

**User Story:** As a user, I want the default theme to match the current application appearance, so that the visual experience remains consistent after the theme system is introduced.

#### Acceptance Criteria

1. THE default Theme File SHALL be named `THEME-default.yaml`
2. WHEN no theme is specified in the Config File, THE Application SHALL use `THEME-default.yaml`
3. THE default Theme File SHALL contain the same color and style values as the current application defaults
4. WHEN the default Theme File does not exist, THE Application SHALL create it with default values

### Requirement 5

**User Story:** As a developer, I want the configuration structure to be clean and maintainable, so that the codebase remains easy to understand and modify.

#### Acceptance Criteria

1. THE Config File SHALL NOT contain any color or style options after migration
2. THE Config File SHALL only contain non-visual settings (database location, video extensions, video player, logging configuration)
3. THE Application SHALL load theme settings separately from config settings
4. THE Application SHALL validate that the specified theme file exists before attempting to load it

### Requirement 6

**User Story:** As a developer, I want migration code removed after implementation, so that the codebase doesn't carry unnecessary backward compatibility code.

#### Acceptance Criteria

1. WHEN the feature is complete and tested, THE migration code SHALL be removed from the codebase
2. THE Application SHALL assume theme files exist and SHALL NOT attempt migration after cleanup
3. THE Application SHALL create default theme files if missing, but SHALL NOT migrate from old config format
