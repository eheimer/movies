# Requirements Document

**GitHub Issue:** #36

## Introduction

This feature converts the application's configuration file format from JSON to YAML while adding comprehensive inline documentation. The YAML format will include descriptions for each setting and examples of possible options, making the configuration file self-documenting and more user-friendly.

## Glossary

- **Application**: The terminal-based video file browser and library manager
- **Config_File**: The configuration file that stores application settings (currently config.json, will become config.yaml)
- **YAML**: YAML Ain't Markup Language, a human-readable data serialization format
- **Inline_Documentation**: Comments within the YAML file that describe each setting
- **Backward_Compatibility**: The ability to read and migrate existing JSON config files

## Requirements

### Requirement 1

**User Story:** As a user, I want the configuration file to be in YAML format with inline documentation, so that I can understand what each setting does and what values are valid without consulting external documentation.

#### Acceptance Criteria

1. WHEN the Application starts THEN the System SHALL attempt to read configuration from config.yaml
2. WHEN config.yaml contains valid YAML syntax THEN the System SHALL parse and load all configuration values
3. WHEN config.yaml is missing THEN the System SHALL create a new config.yaml file with default values and inline documentation
4. WHEN config.yaml contains inline comments THEN the System SHALL preserve those comments when reading the configuration
5. WHEN the System writes config.yaml THEN the System SHALL include inline documentation describing each setting and listing valid options

### Requirement 2

**User Story:** As a user upgrading from a previous version, I want my existing config.json to be automatically migrated to config.yaml, so that I don't lose my customized settings.

#### Acceptance Criteria

1. WHEN config.json exists and config.yaml does not exist THEN the System SHALL read settings from config.json
2. WHEN the System reads settings from config.json THEN the System SHALL create config.yaml with those settings and inline documentation
3. WHEN the System creates config.yaml from config.json THEN the System SHALL preserve all user-customized values
4. WHEN the System successfully migrates config.json to config.yaml THEN the System SHALL rename config.json to config.json.backup
5. WHEN both config.yaml and config.json exist THEN the System SHALL use config.yaml and ignore config.json

### Requirement 3

**User Story:** As a user, I want inline documentation for color settings, so that I understand what colors are available and what each color setting controls.

#### Acceptance Criteria

1. WHEN config.yaml contains color settings THEN the System SHALL include comments listing all valid color values
2. WHEN config.yaml contains foreground color settings THEN the System SHALL include comments explaining the visual element being colored
3. WHEN config.yaml contains background color settings THEN the System SHALL include comments explaining the visual element being colored
4. WHEN the System documents color settings THEN the System SHALL list valid color options including: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, DarkGray, Reset

### Requirement 4

**User Story:** As a user, I want inline documentation for the log_level setting, so that I understand all available logging levels and how the application treats each level.

#### Acceptance Criteria

1. WHEN config.yaml contains log_level setting THEN the System SHALL include comments listing all valid log levels: error, warn, info, debug
2. WHEN config.yaml documents log_level THEN the System SHALL explain that error shows only errors
3. WHEN config.yaml documents log_level THEN the System SHALL explain that warn shows warnings and errors
4. WHEN config.yaml documents log_level THEN the System SHALL explain that info shows informational messages, warnings, and errors
5. WHEN config.yaml documents log_level THEN the System SHALL explain that debug shows all messages including detailed debugging information
6. WHEN config.yaml documents log_level THEN the System SHALL explain that invalid values default to info

### Requirement 5

**User Story:** As a user, I want inline documentation for all configuration settings, so that I can understand the purpose and valid values for each setting without external documentation.

#### Acceptance Criteria

1. WHEN config.yaml contains db_location setting THEN the System SHALL include comments explaining it specifies the database file path
2. WHEN config.yaml contains video_extensions setting THEN the System SHALL include comments explaining it lists file extensions recognized as video files
3. WHEN config.yaml contains video_player setting THEN the System SHALL include comments explaining it specifies the path to the external video player executable
4. WHEN config.yaml contains indicator settings THEN the System SHALL include comments explaining the Unicode characters used for watched and unwatched status
5. WHEN config.yaml contains log_file setting THEN the System SHALL include comments explaining that null uses the default location

### Requirement 6

**User Story:** As a developer, I want the YAML parsing to handle errors gracefully, so that users receive helpful error messages when their config file has syntax errors.

#### Acceptance Criteria

1. WHEN config.yaml contains invalid YAML syntax THEN the System SHALL display an error message indicating the syntax error
2. WHEN config.yaml contains invalid YAML syntax THEN the System SHALL fall back to default configuration values
3. WHEN the System falls back to defaults due to YAML errors THEN the System SHALL log a warning message
4. WHEN config.yaml contains unknown fields THEN the System SHALL ignore those fields and continue loading
5. WHEN config.yaml is missing required fields THEN the System SHALL use default values for those fields

### Requirement 7

**User Story:** As a user, I want the YAML config file to be well-formatted and readable, so that I can easily edit it manually.

#### Acceptance Criteria

1. WHEN the System writes config.yaml THEN the System SHALL use consistent indentation of 2 spaces
2. WHEN the System writes config.yaml THEN the System SHALL group related settings together with blank lines between groups
3. WHEN the System writes config.yaml THEN the System SHALL place inline comments above each setting
4. WHEN the System writes config.yaml THEN the System SHALL format list values with proper YAML list syntax
5. WHEN the System writes config.yaml THEN the System SHALL use null for optional fields that have no value
