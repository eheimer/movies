# Implementation Plan

- [x] 1. Create Theme struct and module foundation
  - Create new `src/theme.rs` module file
  - Define `Theme` struct with all color and style fields matching current config fields
  - Implement `Default` trait for Theme with current default values
  - Implement serde `Deserialize` and `Serialize` traits
  - Add `Clone` derive for Theme struct
  - Export theme module in `src/lib.rs`
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 4.3_

- [x] 2. Implement theme file I/O operations
  - Implement `load_theme(theme_path: &PathBuf) -> Theme` function
  - Implement `save_theme(theme: &Theme, theme_path: &PathBuf)` function
  - Implement `generate_theme_yaml_with_comments(theme: &Theme) -> String` helper function
  - Handle file not found by creating default theme
  - Handle YAML parse errors by falling back to defaults and logging warnings
  - _Requirements: 1.1, 1.2, 1.3, 3.5, 4.4_

- [x] 2.1 Write unit tests for theme I/O
  - Test loading valid theme file with all fields
  - Test loading theme file with missing fields (should use defaults)
  - Test loading non-existent theme file (should create default)
  - Test loading invalid YAML (should fall back to defaults)
  - Test saving theme with all fields
  - Test YAML generation includes comments
  - _Requirements: 1.1, 1.2, 1.3, 3.5_

- [x] 3. Update Config struct for theme system
  - Add `active_theme: String` field to Config struct with serde default
  - Implement `default_active_theme()` function returning "THEME-default.yaml"
  - Convert all existing style fields to `Option<String>` for migration support
  - Add `#[serde(skip_serializing_if = "Option::is_none")]` to legacy style fields
  - Update `Config::default()` to include `active_theme` field
  - Update `generate_yaml_with_comments` to include active_theme documentation
  - Update `generate_yaml_with_comments` to skip style fields (they'll be None after migration)
  - _Requirements: 1.4, 2.2, 4.2, 5.1, 5.2_

- [x] 3.1 Write unit tests for updated Config
  - Test loading config with active_theme field
  - Test loading config without active_theme field (should use default)
  - Test config serialization excludes None style fields
  - Test config YAML generation includes active_theme documentation
  - _Requirements: 1.4, 4.2, 5.1_

- [x] 4. Implement migration functionality
  - Implement `needs_migration(config: &Config) -> bool` function (checks if current_fg is Some)
  - Implement `migrate_config_to_theme(config: &mut Config, config_dir: &Path) -> Theme` function
  - Extract all style fields from config Options into Theme struct
  - Set all style Option fields to None in config
  - Set `active_theme` field in config to "THEME-default.yaml"
  - Save theme file to config directory
  - Save updated config file
  - Log migration actions
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 5.1_

- [x] 4.1 Write unit tests for migration
  - Test migration detection with old-format config
  - Test migration detection with new-format config (no migration needed)
  - Test style field extraction from config to theme
  - Test config cleanup after migration (style fields removed)
  - Test active_theme field set during migration
  - Test full migration flow with file I/O
  - _Requirements: 2.1, 2.2, 2.3, 2.4_

- [x] 5. Integrate theme loading into application startup
  - Update `main.rs` to load theme after loading config
  - Check if migration is needed after loading config
  - Perform migration if needed
  - Construct theme file path from config directory and active_theme field
  - Load theme using `load_theme` function
  - Pass theme to display and handler functions
  - Handle theme loading errors gracefully
  - _Requirements: 1.1, 1.3, 1.5, 2.1, 5.3, 5.4_

- [x] 6. Update display.rs to use Theme
  - Add `theme: &Theme` parameter to `draw_screen` function
  - Update all color/style references to use `theme.field` instead of `config.field`
  - Update `format_episode_with_indicator` to accept `&Theme` instead of `&Config`
  - Update `format_series_display` to accept `&Theme` parameter
  - Update `format_season_display` to accept `&Theme` parameter
  - Update all `string_to_fg_color_or_default` calls to use theme fields
  - Update all `string_to_bg_color_or_default` calls to use theme fields
  - Update all `apply_text_style` calls to use theme fields
  - _Requirements: 1.3, 5.3_

- [x] 6.1 Write unit tests for display with theme
  - Test format_episode_with_indicator uses theme indicators
  - Test format_episode_with_indicator uses theme styles
  - Test series/season display uses theme colors
  - _Requirements: 1.3_

- [x] 7. Update scrollbar.rs to use Theme
  - Add `theme: &Theme` parameter to `draw_scrollbar` function
  - Update scrollbar color references to use `theme.scrollbar_fg` and `theme.scrollbar_bg`
  - Update scrollbar character references to use `theme.scrollbar_track_char` and `theme.scrollbar_indicator_char`
  - _Requirements: 1.3, 5.3_

- [x] 8. Update handlers.rs to pass Theme
  - Update all calls to `draw_screen` to pass `&theme` parameter
  - Update any other display function calls to pass theme
  - Ensure theme is available in handler context
  - _Requirements: 1.3, 5.3_

- [x] 9. Update existing config tests
  - Update tests that check for style fields in config to expect None values
  - Update tests that verify config serialization to expect no style fields
  - Update tests that check default config to include active_theme field
  - Fix any tests that directly access config style fields
  - _Requirements: 5.1, 5.2_

- [x] 10. Integration testing and validation
  - Test full application startup with new theme system
  - Test application startup with old config (migration path)
  - Test application startup with new config (normal path)
  - Verify all UI elements render correctly with theme
  - Test theme file creation when missing
  - Test error handling for invalid theme files
  - _Requirements: 1.1, 1.2, 1.3, 2.1, 2.2, 2.3, 2.4, 5.3, 5.4_

- [x] 11. Remove migration code
  - Remove `needs_migration` function
  - Remove `migrate_config_to_theme` function
  - Remove migration logic from main.rs startup
  - Remove Option wrappers from Config style fields (delete the fields entirely)
  - Remove migration-related tests
  - Update documentation to reflect final implementation
  - _Requirements: 6.1, 6.2, 6.3_
