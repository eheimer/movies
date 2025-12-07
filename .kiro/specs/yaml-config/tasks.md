# Implementation Plan

- [x] 1. Add YAML dependency and update imports
  - Add `serde_yaml = "0.9"` to Cargo.toml dependencies
  - Add `use serde_yaml` import to src/config.rs
  - _Requirements: 1.1, 1.2_

- [x] 2. Implement YAML generation with inline documentation
  - Create `generate_yaml_with_comments()` function that manually constructs YAML string
  - Include comment blocks for each configuration section (database, colors, logging, video)
  - Document all valid color values: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, DarkGray, Reset
  - Document log levels with explanations: error (errors only), warn (warnings and errors), info (informational, warnings, errors), debug (all messages including debugging)
  - Document all other settings (db_location, video_extensions, video_player, indicators, log_file)
  - Use 2-space indentation and group related settings with blank lines
  - _Requirements: 1.5, 3.1, 3.2, 3.3, 3.4, 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 5.1, 5.2, 5.3, 5.4, 5.5, 7.1, 7.2, 7.3, 7.4, 7.5_

- [ ]* 2.1 Write unit tests for YAML generation
  - Test that generated YAML includes all required comments
  - Test that generated YAML has proper formatting (indentation, grouping)
  - Test that generated YAML includes color documentation
  - Test that generated YAML includes log level documentation
  - _Requirements: 1.5, 3.1, 4.1, 7.1, 7.2, 7.3_

- [x] 3. Implement JSON to YAML migration function
  - Create `migrate_json_to_yaml()` function
  - Read config.json using existing JSON parser
  - Generate config.yaml with inline documentation using the loaded config
  - Write config.yaml to disk
  - Rename config.json to config.json.backup on success
  - Return error if any step fails without modifying config.json
  - _Requirements: 2.1, 2.2, 2.3, 2.4_

- [ ]* 3.1 Write unit tests for migration
  - Test successful migration from config.json to config.yaml
  - Test that all values are preserved during migration
  - Test that config.json is renamed to config.json.backup
  - Test that migration errors leave config.json intact
  - _Requirements: 2.1, 2.2, 2.3, 2.4_

- [x] 4. Update save_config() to write YAML
  - Modify `save_config()` to call `generate_yaml_with_comments()`
  - Write YAML string to config.yaml instead of config.json
  - Update error messages to reference config.yaml
  - _Requirements: 1.5, 7.1, 7.2, 7.3_

- [ ]* 4.1 Write unit tests for save_config()
  - Test that save_config() creates valid YAML file
  - Test that saved YAML includes inline documentation
  - Test error handling when write fails
  - _Requirements: 1.5_

- [x] 5. Update read_config() to handle YAML and migration
  - Check for config.yaml first
  - If config.yaml exists, parse with `serde_yaml::from_str()`
  - If config.yaml missing, check for config.json
  - If config.json exists, call `migrate_json_to_yaml()` then read config.yaml
  - If neither exists, create default config.yaml
  - Handle YAML parsing errors: display error message, log warning, fall back to defaults
  - Handle unknown fields gracefully (serde will ignore them by default)
  - Use default values for missing optional fields (existing serde defaults)
  - _Requirements: 1.1, 1.2, 1.3, 2.1, 2.5, 6.1, 6.2, 6.3, 6.4, 6.5_

- [ ]* 5.1 Write unit tests for read_config()
  - Test reading valid config.yaml
  - Test creating default config.yaml when no config exists
  - Test migration when only config.json exists
  - Test that config.yaml takes precedence over config.json
  - Test invalid YAML syntax handling (error message, fallback to defaults)
  - Test missing optional fields use defaults
  - Test unknown fields are ignored
  - _Requirements: 1.1, 1.2, 1.3, 2.1, 2.5, 6.1, 6.2, 6.3, 6.4, 6.5_

- [x] 6. Update main.rs to use config.yaml
  - Change config file path from "config.json" to "config.yaml"
  - Update any error messages or logging that reference config.json
  - _Requirements: 1.1_

- [ ]* 6.1 Write integration tests
  - Test full application startup with config.yaml
  - Test application startup with config.json (migration flow)
  - Test application startup with no config file
  - Test application behavior with malformed config.yaml
  - _Requirements: 1.1, 1.2, 1.3, 2.1, 6.1, 6.2_

- [x] 7. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 8. Remove JSON config support and migration code
  - Remove `migrate_json_to_yaml()` function from src/config.rs
  - Remove JSON migration logic from `read_config()` function
  - Remove config.json checking and fallback code
  - Simplify `read_config()` to only handle config.yaml
  - Remove any references to config.json in comments and error messages
  - Remove `serde_json` dependency from Cargo.toml if no longer used elsewhere
  - Update tests to remove JSON-related test cases
  - _Requirements: 1.1, 1.2, 1.3_
