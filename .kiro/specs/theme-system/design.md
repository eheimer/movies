# Design Document: Theme System

## Overview

This design introduces a theme system that separates visual styling configuration from core application settings. The system:

1. Creates a `Theme` struct to hold all color and style options
2. Adds an `active_theme` field to the `Config` struct
3. Implements theme file loading/saving in YAML format
4. Updates the application to use theme settings for all visual rendering

The theme system provides clean separation of concerns with visual styling in dedicated theme files and application configuration in config.yaml.

## Architecture

### Component Structure

```
┌─────────────────────────────────────────────────────────────┐
│                      Application Startup                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
         ┌─────────────────────────────┐
         │   Load config.yaml          │
         │   (contains active_theme)   │
         └─────────────┬───────────────┘
                       │
                       ▼
         ┌─────────────────────────────┐
         │   Load Theme File           │
         │   (from active_theme)       │
         └─────────────┬───────────────┘
                       │
                       ▼
         ┌─────────────────────────────┐
         │   Application runs with     │
         │   Config + Theme            │
         └─────────────────────────────┘
```

### Data Flow

1. **Startup**: Application loads `config.yaml` from config directory
2. **Theme Loading**: Application reads `active_theme` field and loads corresponding theme file
3. **Migration (if needed)**: If style fields exist in config, extract them to a new theme file
4. **Runtime**: Display functions access theme settings through the `Theme` struct
5. **Persistence**: Theme changes are saved to the theme file, config changes to config.yaml

## Components and Interfaces

### Theme Struct

```rust
#[derive(Deserialize, Serialize, Clone)]
pub struct Theme {
    // Current selection colors
    pub current_fg: String,
    pub current_bg: String,
    
    // Dirty state colors
    pub dirty_fg: String,
    pub dirty_bg: String,
    
    // Watched episode indicator
    pub watched_indicator: String,
    pub watched_fg: String,
    pub watched_style: String,
    
    // Unwatched episode indicator
    pub unwatched_indicator: String,
    pub unwatched_fg: String,
    pub unwatched_style: String,
    
    // New episode colors
    pub new_fg: String,
    pub new_bg: String,
    
    // Invalid episode colors
    pub invalid_fg: String,
    pub invalid_bg: String,
    
    // Series entry colors
    pub series_fg: String,
    pub series_bg: String,
    
    // Season entry colors
    pub season_fg: String,
    pub season_bg: String,
    
    // Episode entry colors
    pub episode_fg: String,
    pub episode_bg: String,
    
    // Status line colors
    pub status_fg: String,
    pub status_bg: String,
    
    // Scroll bar configuration
    pub scrollbar_track_char: String,
    pub scrollbar_indicator_char: String,
    pub scrollbar_fg: String,
    pub scrollbar_bg: String,
    
    // Count display styling
    pub count_fg: String,
    pub count_style: String,
    
    // Header text styling
    pub header_fg: String,
    pub header_style: String,
    
    // Help text styling
    pub help_fg: String,
    pub help_style: String,
}
```

### Updated Config Struct

```rust
#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_location: Option<String>,
    
    // Theme configuration
    #[serde(default = "default_active_theme")]
    pub active_theme: String,
    
    // Logging configuration
    #[serde(default = "default_log_file")]
    pub log_file: Option<String>,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    
    // Video configuration
    pub video_extensions: Vec<String>,
    pub video_player: String,
}
```

### Theme Module Functions

```rust
// Load theme from file
pub fn load_theme(theme_path: &PathBuf) -> Theme;

// Save theme to file
pub fn save_theme(theme: &Theme, theme_path: &PathBuf);

// Generate YAML with comments for theme file
fn generate_theme_yaml_with_comments(theme: &Theme) -> String;

// Create default theme
impl Default for Theme;
```

### Integration Points

1. **main.rs**: Load theme after loading config
2. **display.rs**: Accept `&Theme` parameter instead of accessing config fields
3. **scrollbar.rs**: Accept `&Theme` parameter for scrollbar rendering
4. **handlers.rs**: Pass theme to display functions

## Data Models

### Theme File Format (THEME-default.yaml)

```yaml
# === Color Configuration ===
# Valid colors: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, DarkGray, Reset
# Reset means use the terminal's default color

# Current selection colors (highlighted item in browse mode)
current_fg: Black
current_bg: White

# Dirty state colors (items with unsaved changes)
dirty_fg: Black
dirty_bg: White

# Watched episode indicator
# Unicode character displayed for watched episodes
watched_indicator: "●"
# Foreground color for watched indicator
watched_fg: Green
# Style for watched indicator (none, bold, dim, italic, underline)
watched_style: none

# Unwatched episode indicator
# Unicode character displayed for unwatched episodes
unwatched_indicator: "○"
# Foreground color for unwatched indicator
unwatched_fg: Reset
# Style for unwatched indicator (none, bold, dim, italic, underline)
unwatched_style: none

# New episode colors (when title matches filename)
new_fg: Green
new_bg: Reset

# Invalid episode colors (when video file doesn't exist)
invalid_fg: Red
invalid_bg: Reset

# Series entry colors (for series items in browse mode)
series_fg: Blue
series_bg: Reset

# Season entry colors (for season items in browse mode)
season_fg: Blue
season_bg: Reset

# Episode entry colors (for episode items in normal state)
episode_fg: Reset
episode_bg: Reset

# Status line colors (bottom status bar)
status_fg: White
status_bg: DarkGray

# Scroll bar configuration
# Character used for the scroll bar track
scrollbar_track_char: "│"
# Character used for the scroll bar indicator
scrollbar_indicator_char: "█"
# Foreground color for scroll bar
scrollbar_fg: White
# Background color for scroll bar
scrollbar_bg: Reset

# Count display styling (watched/unwatched counts for series and seasons)
# Foreground color for count text
count_fg: DarkGray
# Style for count text (none, bold, dim, italic, underline)
count_style: italic

# Header text styling
# Foreground color for header text
header_fg: Black
# Style for header text (none, bold, dim, italic, underline)
header_style: none

# Help text styling
# Foreground color for help text
help_fg: Reset
# Style for help text (none, bold, dim, italic, underline)
help_style: none
```

### Updated Config File Format (config.yaml)

```yaml
# === Database Configuration ===
# Path to the SQLite database file
# Set to null to use default location in executable directory
db_location: null

# === Theme Configuration ===
# Name of the active theme file (without path)
# Theme files are stored in the same directory as this config file
# Default: THEME-default.yaml
active_theme: THEME-default.yaml

# === Logging Configuration ===
# Log file location
# Set to null to use default location (app.log in executable directory)
log_file: null

# Log level - controls verbosity of logging
# Valid levels:
#   error - Only log errors
#   warn  - Log warnings and errors
#   info  - Log informational messages, warnings, and errors (default)
#   debug - Log all messages including detailed debugging information
# Invalid values will default to info
log_level: info

# === Video Configuration ===
# File extensions recognized as video files
video_extensions:
  - mp4
  - mkv
  - avi
  - mov
  - flv
  - wmv
  - webm

# Path to external video player executable
video_player: /usr/bin/vlc
```

## Test Cases

*A test case is a characteristic or behavior that should be verified through testing - essentially, a statement about what the system should do.*

### Test Case 1: Theme file loading

When a theme file exists and is valid, the system should load all style settings from that file.
**Validates: Requirements 1.1, 1.3**

### Test Case 2: Default theme creation

When no theme file exists, the system should create THEME-default.yaml with default values.
**Validates: Requirements 1.2, 4.1, 4.3, 4.4**

### Test Case 3: Theme file path resolution

When loading a theme, the system should look for the theme file in the same directory as config.yaml.
**Validates: Requirements 1.5**

### Test Case 4: Config active_theme field

When config.yaml is loaded, it should contain an active_theme field specifying which theme to use.
**Validates: Requirements 1.4, 4.2**

### Test Case 5: Migration detection

When config.yaml contains style fields (current_fg, watched_indicator, etc.), the system should detect that migration is needed.
**Validates: Requirements 2.1**

### Test Case 6: Style extraction during migration

When migration occurs, all style fields should be extracted from config and placed into a new theme file.
**Validates: Requirements 2.1, 3.1, 3.2, 3.3, 3.4**

### Test Case 7: Config cleanup during migration

When migration occurs, all style fields should be removed from config.yaml.
**Validates: Requirements 2.2, 5.1, 5.2**

### Test Case 8: Active theme setting during migration

When migration occurs, the active_theme field should be set to THEME-default.yaml.
**Validates: Requirements 2.3**

### Test Case 9: File persistence after migration

When migration completes, both the updated config.yaml and new theme file should be saved to disk.
**Validates: Requirements 2.4**

### Test Case 10: Theme contains all required fields

When a theme file is created, it should contain all color and style fields defined in the Theme struct.
**Validates: Requirements 3.1, 3.2, 3.3, 3.4**

### Test Case 11: Theme YAML format with comments

When a theme file is generated, it should use YAML format with inline documentation comments.
**Validates: Requirements 3.5**

### Test Case 12: Config excludes style options

When config.yaml is saved after migration, it should not contain any color or style fields.
**Validates: Requirements 5.1, 5.2**

### Test Case 13: Separate loading of config and theme

When the application starts, it should load config settings and theme settings separately.
**Validates: Requirements 5.3**

### Test Case 14: Theme file validation

When loading a theme, the system should validate that the specified theme file exists before attempting to load it.
**Validates: Requirements 5.4**

### Test Case 15: Display functions use theme

When rendering the UI, display functions should use theme settings instead of config settings for all visual styling.
**Validates: Requirements 1.3, 5.3**

## Error Handling

### Theme File Not Found
- **Scenario**: Specified theme file doesn't exist
- **Handling**: Log warning, create default theme file, use default theme
- **User Impact**: Application continues with default appearance

### Theme File Parse Error
- **Scenario**: Theme file contains invalid YAML
- **Handling**: Log error with details, fall back to default theme, attempt to save valid default theme
- **User Impact**: Application continues with default appearance, invalid theme file preserved for user inspection

### Migration Failure
- **Scenario**: Cannot write theme file or updated config during migration
- **Handling**: Log error, continue with in-memory theme, preserve original config
- **User Impact**: Application runs but changes not persisted

### Missing Theme Fields
- **Scenario**: Theme file missing some fields
- **Handling**: Use serde defaults for missing fields, log warning
- **User Impact**: Missing fields use default values

### Invalid Color Values
- **Scenario**: Theme contains invalid color names
- **Handling**: Handled by existing `string_to_fg_color_or_default` function, falls back to Reset
- **User Impact**: Invalid colors display as terminal default

## Testing Strategy

### Unit Testing

1. **Theme Loading Tests**
   - Load valid theme file with all fields
   - Load theme file with missing optional fields (should use defaults)
   - Load non-existent theme file (should create default)
   - Load invalid YAML theme file (should fall back to defaults)

2. **Theme Saving Tests**
   - Save theme with all fields populated
   - Verify YAML format includes comments
   - Verify file permissions and location

3. **Migration Tests**
   - Detect config with style fields (needs migration)
   - Detect config without style fields (no migration needed)
   - Extract all style fields from config to theme
   - Remove style fields from config after migration
   - Set active_theme field during migration

4. **Config Tests**
   - Load config with active_theme field
   - Load config without active_theme field (should use default)
   - Verify config excludes style fields after migration

5. **Integration Tests**
   - Full migration flow: old config → new config + theme file
   - Application startup with theme system
   - Display functions using theme instead of config

### Edge Cases

1. **Empty theme file**: Should use all defaults
2. **Partial theme file**: Should merge with defaults
3. **Concurrent file access**: Handled by OS file locking
4. **Theme file in use**: Standard file I/O error handling
5. **Unicode characters in theme**: Already supported in current config
6. **Very long color names**: Validated by color parsing function

### Test Data

- Valid theme files with various color combinations
- Invalid YAML files for error handling
- Old-format config files for migration testing
- New-format config files for normal operation

## Implementation Notes

### Theme File Management

The application expects theme files to exist in the config directory. If a referenced theme file is missing, the application will create a default theme file automatically. This ensures the application can always start successfully even if theme files are deleted or missing.

### Performance Considerations

- Theme loaded once at startup (no runtime overhead)
- Theme file size ~2KB (negligible)
- File I/O only during startup and theme changes
- No impact on rendering performance

### Future Extensibility

- Multiple theme files can coexist in config directory
- Users can create custom themes by copying and modifying THEME-default.yaml
- Future feature: Theme switching without restart (reload theme file)
- Future feature: Theme editor UI
