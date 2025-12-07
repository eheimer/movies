# Design Document

## Overview

This design converts the application's configuration system from JSON to YAML format with comprehensive inline documentation. The implementation will use the `serde_yaml` crate for YAML parsing and serialization, maintain backward compatibility with existing JSON configs through automatic migration, and provide a self-documenting configuration file that includes descriptions and valid options for each setting.

## Architecture

### Component Structure

```
src/config.rs (modified)
├── Config struct (unchanged)
├── read_config() - Updated to handle YAML and migration
├── save_config() - Updated to write YAML with comments
├── migrate_json_to_yaml() - New function for migration
└── generate_yaml_with_comments() - New function for documented output
```

### Dependencies

Add to Cargo.toml:
```toml
serde_yaml = "0.9"
```

The `serde_yaml` crate provides:
- YAML parsing and serialization
- Compatible with existing `serde` derive macros
- Support for preserving structure during round-trips

### Migration Strategy

1. On startup, check for config files in this order:
   - config.yaml exists → use it
   - config.yaml missing, config.json exists → migrate to YAML
   - Neither exists → create default config.yaml

2. Migration process:
   - Read config.json using existing JSON parser
   - Write config.yaml with inline documentation
   - Rename config.json to config.json.backup

## Components and Interfaces

### Config Struct

The existing `Config` struct remains unchanged. All fields already use serde attributes that work with both JSON and YAML:

```rust
#[derive(Deserialize, Serialize)]
pub struct Config {
    // ... existing fields ...
}
```

### Configuration File Reading

```rust
pub fn read_config(config_dir: &Path) -> Config
```

Updated logic:
1. Check for config.yaml
2. If found, parse with `serde_yaml::from_str()`
3. If not found, check for config.json
4. If config.json found, migrate to YAML
5. If neither found, create default config.yaml
6. Handle parsing errors gracefully with fallback to defaults

### Configuration File Writing

```rust
pub fn save_config(config: &Config, config_path: &PathBuf)
```

Updated to:
1. Generate YAML string with inline comments
2. Write to config.yaml
3. Handle write errors with appropriate logging

### YAML Generation with Comments

```rust
fn generate_yaml_with_comments(config: &Config) -> String
```

This function will:
1. Manually construct YAML string with comments
2. Group related settings (colors, logging, video, etc.)
3. Include descriptive comments above each setting
4. List valid options where applicable
5. Use consistent formatting (2-space indentation)

Example output structure:
```yaml
# Database location (path to SQLite database file)
# Set to null to use default location in executable directory
db_location: null

# === Color Configuration ===
# Valid colors: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, DarkGray, Reset

# Current selection colors (highlighted item in browse mode)
current_fg: Black
current_bg: Yellow

# ... more settings with comments ...
```

### JSON to YAML Migration

```rust
fn migrate_json_to_yaml(json_path: &PathBuf, yaml_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>>
```

This function will:
1. Read and parse config.json
2. Generate config.yaml with comments using the loaded config
3. Write config.yaml
4. Rename config.json to config.json.backup
5. Return error if any step fails

## Data Models

No changes to the `Config` struct. The existing structure works with both JSON and YAML serialization.

### File Paths

- Old: `config.json` in project root
- New: `config.yaml` in project root
- Backup: `config.json.backup` after migration

## Test Cases

### Test Case 1: YAML config loading

When config.yaml exists with valid YAML syntax, the system should successfully parse and load all configuration values.
**Validates: Requirements 1.1, 1.2**

### Test Case 2: Default YAML creation

When config.yaml does not exist and config.json does not exist, the system should create a new config.yaml file with default values and inline documentation.
**Validates: Requirements 1.3, 1.5**

### Test Case 3: JSON to YAML migration

When config.json exists and config.yaml does not exist, the system should read settings from config.json, create config.yaml with those settings and documentation, and rename config.json to config.json.backup.
**Validates: Requirements 2.1, 2.2, 2.3, 2.4**

### Test Case 4: YAML takes precedence

When both config.yaml and config.json exist, the system should use config.yaml and ignore config.json.
**Validates: Requirements 2.5**

### Test Case 5: Invalid YAML handling

When config.yaml contains invalid YAML syntax, the system should display an error message, fall back to default configuration values, and log a warning.
**Validates: Requirements 6.1, 6.2, 6.3**

### Test Case 6: Missing fields use defaults

When config.yaml is missing optional fields, the system should use default values for those fields.
**Validates: Requirements 6.5**

### Test Case 7: Unknown fields ignored

When config.yaml contains unknown fields not in the Config struct, the system should ignore those fields and continue loading valid fields.
**Validates: Requirements 6.4**

### Test Case 8: YAML formatting

When the system writes config.yaml, it should use consistent 2-space indentation, group related settings with blank lines, and place comments above each setting.
**Validates: Requirements 7.1, 7.2, 7.3**

### Test Case 9: Color documentation completeness

When config.yaml is generated, all color settings should include comments listing valid color values and explaining what visual element is being colored.
**Validates: Requirements 3.1, 3.2, 3.3, 3.4**

### Test Case 10: Log level documentation

When config.yaml contains the log_level setting, it should include comments listing all valid levels (error, warn, info, debug) with explanations of what each level shows.
**Validates: Requirements 4.1, 4.2, 4.3, 4.4, 4.5, 4.6**

### Test Case 11: All settings documented

When config.yaml is generated, all settings (db_location, video_extensions, video_player, indicators, log_file) should include descriptive comments.
**Validates: Requirements 5.1, 5.2, 5.3, 5.4, 5.5**

## Error Handling

### YAML Parsing Errors

- Catch `serde_yaml::Error` when parsing config.yaml
- Display user-friendly error message to stderr
- Log detailed error with `log_warn()`
- Fall back to default configuration
- Continue application startup

### File I/O Errors

- Handle file read/write errors gracefully
- Log errors with context (which file, what operation)
- For read errors: fall back to defaults
- For write errors: warn user but continue (config in memory is valid)

### Migration Errors

- If migration fails, leave config.json intact
- Log the error with details
- Fall back to reading config.json directly
- User can manually fix and retry

## Testing Strategy

### Unit Testing

1. **YAML parsing tests**: Verify config.yaml with various valid configurations loads correctly
2. **Default creation tests**: Verify default config.yaml is created with proper structure and comments
3. **Migration tests**: Verify config.json is correctly converted to config.yaml with all values preserved
4. **Error handling tests**: Verify invalid YAML syntax triggers appropriate error handling
5. **Backward compatibility tests**: Verify existing config.json continues to work
6. **Comment preservation tests**: Verify generated YAML includes all required documentation
7. **Field default tests**: Verify missing optional fields use correct defaults
8. **Unknown field tests**: Verify unknown fields in YAML are ignored without errors

### Integration Testing

1. Test full application startup with config.yaml
2. Test migration flow from config.json to config.yaml
3. Test application behavior with malformed config.yaml
4. Verify all config values are correctly applied to application behavior

### Manual Testing

1. Create config.yaml manually and verify it loads
2. Edit config.yaml and verify changes are respected
3. Delete config.yaml and verify default is created
4. Verify inline documentation is helpful and accurate
5. Test with existing config.json to verify migration

## Implementation Notes

### Comment Generation Strategy

Since `serde_yaml` doesn't preserve comments during serialization, we'll manually construct the YAML string with comments. This gives us full control over formatting and documentation.

### Grouping Strategy

Group settings logically:
1. Database configuration
2. Current selection colors
3. Dirty state colors
4. Watched/unwatched indicators
5. Episode state colors (new, invalid)
6. Entry type colors (series, season, episode)
7. Status line colors
8. Logging configuration
9. Video configuration

### Path Handling

The config file location logic in `main.rs` will need minor updates:
- Change `config.json` references to `config.yaml`
- Keep the same directory (project root or executable directory)

### Backward Compatibility Period

After migration:
- Keep config.json.backup for user reference
- Document migration in release notes
- Consider removing JSON support in future major version
