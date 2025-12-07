# Design Document

## Overview

This design enhances the visual presentation of the terminal-based video library manager by adding configurable color schemes and visual indicators. The system will provide clear visual feedback for watched status, new episodes (where title equals filename), invalid episodes (where the video file doesn't exist), and improved distinction between series, seasons, and episodes. All color enhancements will be user-configurable through the config.json file with sensible defaults.

The design maintains the existing terminal UI architecture while extending the color configuration system and adding visual state detection logic.

## Architecture

The enhancement follows the existing modular architecture:

1. **Configuration Layer** (`config.rs`): Extended to include new color configuration options
2. **Display Layer** (`display.rs`): Enhanced to apply colors based on entry state and type
3. **State Detection**: New logic to determine episode states (new, invalid, watched)
4. **Path Resolution** (`path_resolver.rs`): Used to validate file existence for invalid state detection

### Key Design Decisions

1. **Configurable Colors**: All visual enhancements use configurable colors to support different terminal themes and user preferences
2. **Default Values**: Sensible defaults ensure the application works well out-of-the-box
3. **Backward Compatibility**: Missing config fields automatically populate with defaults
4. **Performance**: File existence checks are performed only during rendering, not during database queries
5. **State Priority**: When multiple states apply (e.g., new AND watched), a priority system determines which visual indicator takes precedence

## Components and Interfaces

### 1. Enhanced Config Structure

```rust
pub struct Config {
    pub db_location: Option<String>,
    
    // Current selection colors (existing)
    pub current_fg: String,
    pub current_bg: String,
    
    // Dirty field colors (existing)
    pub dirty_fg: String,
    pub dirty_bg: String,
    
    // New: Watched episode indicator
    pub watched_indicator: String,
    pub watched_fg: String,
    
    // New: New episode colors (title == filename)
    pub new_fg: String,
    pub new_bg: String,
    
    // New: Invalid episode colors (file doesn't exist)
    pub invalid_fg: String,
    pub invalid_bg: String,
    
    // New: Series entry colors
    pub series_fg: String,
    pub series_bg: String,
    
    // New: Season entry colors
    pub season_fg: String,
    pub season_bg: String,
    
    // New: Episode entry colors (normal state)
    pub episode_fg: String,
    pub episode_bg: String,
    
    // New: Status line colors
    pub status_fg: String,
    pub status_bg: String,
    
    // Existing fields
    pub video_extensions: Vec<String>,
    pub video_player: String,
}
```

### 2. Episode State Detection

```rust
pub enum EpisodeState {
    Normal,
    Watched,
    New,      // title == filename
    Invalid,  // file doesn't exist
}

pub fn determine_episode_state(
    entry: &Entry,
    episode_detail: &EpisodeDetail,
    resolver: &PathResolver,
) -> EpisodeState {
    // Priority order: Invalid > New > Watched > Normal
    // 1. Check if file exists
    // 2. Check if title equals filename
    // 3. Check watched status
    // 4. Default to normal
}
```

### 3. Display Enhancement Functions

```rust
// Get the appropriate color for an entry based on its type and state
pub fn get_entry_colors(
    entry: &Entry,
    episode_detail: Option<&EpisodeDetail>,
    config: &Config,
    resolver: &PathResolver,
) -> (Color, Color) {
    // Returns (foreground, background) tuple
}

// Format an episode name with watched indicator if applicable
pub fn format_episode_with_indicator(
    name: &str,
    is_watched: bool,
    config: &Config,
) -> String {
    // Prepends watched indicator if watched
}
```

## Data Models

### Episode State Priority

When determining visual presentation, states are evaluated in this priority order:

1. **Invalid** (highest priority): File doesn't exist - shows in red (configurable)
2. **New**: Title equals filename - shows in green (configurable)
3. **Watched**: Episode marked as watched - shows indicator symbol
4. **Normal** (lowest priority): Default episode appearance

### Color Configuration Defaults

```json
{
  "current_fg": "Black",
  "current_bg": "Yellow",
  "dirty_fg": "Black",
  "dirty_bg": "White",
  "watched_indicator": "✓",
  "watched_fg": "Green",
  "new_fg": "Green",
  "new_bg": "Reset",
  "invalid_fg": "Red",
  "invalid_bg": "Reset",
  "series_fg": "Blue",
  "series_bg": "Reset",
  "season_fg": "Blue",
  "season_bg": "Reset",
  "episode_fg": "Reset",
  "episode_bg": "Reset",
  "status_fg": "White",
  "status_bg": "DarkGray"
}
```

Note: "Reset" means use the terminal's default color.


## Test Cases

### Test Case 1: Watched indicator presence

When an episode has watched status set to true, the formatted display string should contain the configured watched indicator character.
**Validates: Requirements 1.1**

### Test Case 2: Unwatched indicator absence

When an episode has watched status set to false, the formatted display string should not contain the watched indicator character.
**Validates: Requirements 1.2**

### Test Case 3: Watched indicator distinctness

When displaying a watched episode, the watched indicator should be separated from the episode name by whitespace or other delimiter.
**Validates: Requirements 1.4**

### Test Case 4: New episode color application

When an episode's title field equals its filename (extracted from location), the display should apply the configured new_fg and new_bg colors.
**Validates: Requirements 2.1**

### Test Case 5: Invalid episode color application

When an episode's file path does not exist on disk, the display should apply the configured invalid_fg and invalid_bg colors.
**Validates: Requirements 2.2**

### Test Case 6: State transition from new to normal

When an episode that was previously new (title == filename) has its title changed to differ from the filename, the next render should not apply new episode colors.
**Validates: Requirements 2.5**

### Test Case 7: State transition from invalid to normal

When an episode that was previously invalid (file doesn't exist) has its file created at the expected path, the next render should not apply invalid episode colors.
**Validates: Requirements 2.6**

### Test Case 8: Config color loading

When the config file contains valid values for color configuration fields, the loaded Config should contain those values.
**Validates: Requirements 3.1, 4.5, 6.4**

### Test Case 9: Missing config field defaults

When color configuration fields are missing from the config file, the loaded Config should contain the default values for those fields.
**Validates: Requirements 3.2, 7.2**

### Test Case 10: Invalid color fallback

When a color configuration field contains an invalid color name, the color parsing function should return the default color for that field.
**Validates: Requirements 3.4, 7.4**

### Test Case 11: Series entry coloring

When displaying a series entry that is not selected, the display should apply the configured series_fg and series_bg colors.
**Validates: Requirements 4.1**

### Test Case 12: Season entry coloring

When displaying a season entry that is not selected, the display should apply the configured season_fg and season_bg colors.
**Validates: Requirements 4.2**

### Test Case 13: Episode entry coloring

When displaying an episode entry in normal state (not new, not invalid, not watched) that is not selected, the display should apply the configured episode_fg and episode_bg colors.
**Validates: Requirements 4.3**

### Test Case 14: Selection highlight override

When an entry (series, season, or episode) is currently selected, the display should apply current_fg and current_bg colors, overriding the entry type colors.
**Validates: Requirements 4.4**

### Test Case 15: Menu selection highlighting

When a menu item is currently selected, the display should apply current_fg and current_bg colors.
**Validates: Requirements 5.5**

## Error Handling

### Configuration Errors

1. **Missing Config File**: If config.json doesn't exist, create it with default values
2. **Malformed JSON**: Log error and use default Config values
3. **Invalid Color Names**: Fall back to default colors and optionally log warning
4. **Missing Color Fields**: Populate with defaults and update config file

### File System Errors

1. **File Existence Check Failures**: If path resolution fails, treat as invalid episode
2. **Permission Errors**: If file exists but can't be accessed, don't mark as invalid
3. **Path Resolution Errors**: Handle gracefully, fall back to treating as invalid

### Display Errors

1. **Terminal Size Changes**: Recalculate layout on each render
2. **Color Application Failures**: Fall back to no color (terminal default)
3. **Unicode Indicator Issues**: Provide ASCII fallback for watched indicator

## Testing Strategy

### Unit Testing

Unit tests will verify specific examples and edge cases:

1. **Color Parsing Tests**:
   - Test string_to_color with all standard color names (black, red, green, yellow, blue, magenta, cyan, white)
   - Test with invalid color names to verify fallback behavior
   - Test "Reset" as a special color name
   - Validates: Test Cases 8, 9, 10

2. **State Detection Tests**:
   - Test determine_episode_state with title matching filename (new state)
   - Test with non-existent file path (invalid state)
   - Test with watched status true (watched state)
   - Test with normal episode (normal state)
   - Test state priority (invalid > new > watched > normal)
   - Validates: Test Cases 4, 5, 6, 7

3. **Config Default Tests**:
   - Test that Config::default() includes all required color fields
   - Test that missing fields in config file are populated with defaults
   - Test that malformed config falls back to defaults
   - Validates: Test Cases 8, 9

4. **Indicator Formatting Tests**:
   - Test format_episode_with_indicator with watched=true includes indicator
   - Test format_episode_with_indicator with watched=false excludes indicator
   - Test that indicator is separated from name by whitespace
   - Validates: Test Cases 1, 2, 3

5. **Entry Color Application Tests**:
   - Test series entry gets series_fg/series_bg colors
   - Test season entry gets season_fg/season_bg colors
   - Test normal episode gets episode_fg/episode_bg colors
   - Test new episode gets new_fg/new_bg colors
   - Test invalid episode gets invalid_fg/invalid_bg colors
   - Validates: Test Cases 4, 5, 11, 12, 13

6. **Selection Highlighting Tests**:
   - Test selected series uses current_fg/current_bg
   - Test selected season uses current_fg/current_bg
   - Test selected episode uses current_fg/current_bg
   - Test selected menu item uses current_fg/current_bg
   - Validates: Test Cases 14, 15

### Integration Testing

Integration tests will verify end-to-end behavior:

1. **Config Reload Integration**:
   - Create a config file with custom colors
   - Load config and verify colors are applied
   - Modify config file and restart application
   - Verify new colors are applied

2. **State Transition Integration**:
   - Create an episode with title == filename
   - Verify it displays with new colors
   - Edit title to differ from filename
   - Verify new colors are removed

3. **File System Integration**:
   - Create an episode pointing to non-existent file
   - Verify it displays with invalid colors
   - Create the file at the expected path
   - Verify invalid colors are removed

4. **Watched Status Integration**:
   - Create an unwatched episode
   - Verify no watched indicator
   - Toggle watched status
   - Verify watched indicator appears

### Edge Cases

1. **Empty Config**: Test with completely empty config.json
2. **Partial Config**: Test with config missing only some color fields
3. **Unicode Indicators**: Test watched indicator with Unicode characters
4. **Long Filenames**: Test new state detection with very long filenames
5. **Special Characters**: Test with filenames containing special characters
6. **Path Edge Cases**: Test with relative paths, absolute paths, symlinks
7. **Terminal Size**: Test status line with very narrow terminal widths

## Implementation Notes

### Color System Extensions

The existing color system in `display.rs` provides `string_to_color`, `string_to_fg_color_or_default`, and `string_to_bg_color_or_default` functions. These will be extended to:

1. Support "Reset" as a special color name that returns the terminal default
2. Provide consistent error handling for invalid color names
3. Cache color parsing results if performance becomes an issue

### State Detection Logic

Episode state detection will follow this algorithm:

```rust
fn determine_episode_state(
    entry: &Entry,
    episode_detail: &EpisodeDetail,
    resolver: &PathResolver,
) -> EpisodeState {
    // Extract location from entry
    let location = match entry {
        Entry::Episode { location, .. } => location,
        _ => return EpisodeState::Normal,
    };
    
    // Priority 1: Check if file exists
    let absolute_path = resolver.to_absolute(Path::new(location));
    if !absolute_path.exists() {
        return EpisodeState::Invalid;
    }
    
    // Priority 2: Check if title equals filename
    let filename = location.rsplit('/').next().unwrap_or("");
    if episode_detail.title == filename {
        return EpisodeState::New;
    }
    
    // Priority 3: Check watched status
    if episode_detail.watched == "true" {
        return EpisodeState::Watched;
    }
    
    // Default: Normal state
    EpisodeState::Normal
}
```

### Performance Considerations

1. **File Existence Checks**: Performed only during rendering, not during database queries
2. **Color Parsing**: Results could be cached if parsing becomes a bottleneck
3. **State Detection**: Computed on-demand during rendering, not stored in database
4. **Config Loading**: Happens once at startup, not on every render

### Backward Compatibility

The design maintains backward compatibility by:

1. Providing defaults for all new config fields
2. Auto-populating missing fields in existing config files
3. Preserving existing color behavior (current_fg/current_bg, dirty_fg/dirty_bg)
4. Not changing database schema or data structures

### Future Enhancements

Potential future improvements:

1. **RGB Color Support**: Extend color parsing to support RGB values
2. **Theme Presets**: Provide pre-defined color themes (dark, light, high-contrast)
3. **Dynamic Color Adjustment**: Detect terminal background and adjust colors automatically
4. **Custom Indicators**: Allow users to configure watched indicator character
5. **State Combinations**: Support visual indicators for multiple states (e.g., new AND watched)
