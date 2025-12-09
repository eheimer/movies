# Design Document

## Overview

This design enhances the visual readability of the terminal UI by improving how watched/unwatched counts are displayed for series and seasons, and by introducing comprehensive text styling configuration. The current implementation displays counts inline with entry names using the same color, making them difficult to distinguish. This design will:

1. Separate count display from entry names using different colors and styling
2. Make count format more compact (`<watched>/<total> watched`)
3. Right-justify counts for better visual alignment
4. Add comprehensive text styling configuration for all UI elements

The implementation will modify the display module to format counts differently and extend the configuration system to support text styling attributes (color, italic, bold, underline) for all text types.

## Architecture

The solution follows the existing architecture pattern:

1. **Configuration Layer** (`src/config.rs`): Extended to include text style configurations for count display and other text types
2. **Display Layer** (`src/display.rs`): Modified to apply new count formatting and styling
3. **Database Layer** (`src/database.rs`): No changes needed (already provides count data)

The design maintains separation of concerns:
- Configuration manages user preferences
- Display handles rendering logic
- Database provides data

## Components and Interfaces

### 1. Configuration Extensions

Add new fields to the `Config` struct in `src/config.rs`:

```rust
pub struct Config {
    // ... existing fields ...
    
    // Count display styling
    #[serde(default = "default_count_fg")]
    pub count_fg: String,
    #[serde(default = "default_count_style")]
    pub count_style: String,
    
    // Additional text type styling (for future extensibility)
    #[serde(default = "default_header_fg")]
    pub header_fg: String,
    #[serde(default = "default_header_style")]
    pub header_style: String,
    
    #[serde(default = "default_help_fg")]
    pub help_fg: String,
    #[serde(default = "default_help_style")]
    pub help_style: String,
}
```

Default functions:
```rust
fn default_count_fg() -> String { "DarkGray".to_string() }
fn default_count_style() -> String { "italic".to_string() }
fn default_header_fg() -> String { "Black".to_string() }
fn default_header_style() -> String { "none".to_string() }
fn default_help_fg() -> String { "Reset".to_string() }
fn default_help_style() -> String { "none".to_string() }
```

### 2. Display Formatting Functions

Modify `format_series_display()` and `format_season_display()` in `src/display.rs`:

**Current signature:**
```rust
pub fn format_series_display(name: &str, series_id: usize) -> String
```

**New signature:**
```rust
pub fn format_series_display(name: &str, series_id: usize, terminal_width: usize, config: &Config) -> String
```

The function will:
1. Query counts from database
2. Calculate watched count (total - unwatched)
3. Format as `<watched>/<total> watched`
4. Apply styling from config
5. Right-justify within available width

**Implementation approach:**
```rust
pub fn format_series_display(name: &str, series_id: usize, terminal_width: usize, config: &Config) -> String {
    match crate::database::get_series_episode_counts(series_id) {
        Ok((total, unwatched)) => {
            let watched = total - unwatched;
            let count_text = format!("{}/{} watched", watched, total);
            
            // Apply styling to count
            let styled_count = apply_text_style(&count_text, &config.count_style);
            let colored_count = styled_count
                .with(string_to_fg_color_or_default(&config.count_fg));
            
            // Calculate spacing for right-justification
            let name_part = format!("[{}]", name);
            let spacing = terminal_width
                .saturating_sub(name_part.len())
                .saturating_sub(count_text.len())
                .saturating_sub(2); // Padding
            
            format!("{}{}{}", name_part, " ".repeat(spacing), colored_count)
        }
        Err(_) => format!("[{}] ? episodes", name)
    }
}
```

Similar changes for `format_season_display()`.

### 3. Caller Updates

Update call sites in `draw_screen()` to pass additional parameters:

```rust
Entry::Series { name, series_id } => {
    let formatted = format_series_display(name, *series_id, effective_col_width, config);
    // ... rest of rendering logic
}
```

## Data Models

No new data models required. The existing structures are sufficient:

- `Config`: Extended with new fields (backward compatible via serde defaults)
- `Entry`: No changes needed
- Database schema: No changes needed

## Test Cases

### Test Case 1: Count format change

When displaying a series with 5 total episodes and 2 unwatched, the formatted string should contain "3/5 watched" instead of "5 episodes (2 unwatched)".

**Validates: Requirements 1.3**

### Test Case 2: Count color distinction

When displaying a series or season entry, the count portion should be rendered in a different color than the entry name.

**Validates: Requirements 1.1**

### Test Case 3: Count italic styling

When displaying a series or season entry with default configuration, the count text should have italic styling applied.

**Validates: Requirements 1.2**

### Test Case 4: Right-justification

When displaying entries with varying name lengths, the count text should be right-justified within the column width, maintaining consistent alignment.

**Validates: Requirements 1.4**

### Test Case 5: Spacing preservation

When rendering count display, there should be at least one space between the entry name and the count text, even for long names.

**Validates: Requirements 1.5**

### Test Case 6: Config loading for count styles

When the configuration file contains count_fg and count_style values, the loaded Config should contain those values.

**Validates: Requirements 2.1**

### Test Case 7: Missing count style defaults

When count style fields are missing from the configuration file, the Config should use default values (DarkGray color, italic style).

**Validates: Requirements 2.2**

### Test Case 8: Style application

When rendering count text, the display module should apply both color and style attributes from the configuration.

**Validates: Requirements 2.3**

### Test Case 9: Invalid style handling

When the configuration contains an invalid style value, the application should log a warning and use default styling.

**Validates: Requirements 2.4**

### Test Case 10: Config persistence

When the configuration is saved to file, all text style fields should be included in the YAML output with inline documentation.

**Validates: Requirements 2.5**

### Test Case 11: Text type coverage

When defining configuration options, the Config struct should include style fields for series names, season names, episode names, counts, selected items, headers, help text, and status messages.

**Validates: Requirements 3.1**

### Test Case 12: Style attribute support

When defining text style configurations, each text type should support color, italic, bold, and underline properties.

**Validates: Requirements 3.2**

### Test Case 13: Schema validation

When the application initializes, it should successfully load configurations with all expected text type fields.

**Validates: Requirements 3.3**

### Test Case 14: Style combination

When multiple style attributes are configured for a text type (e.g., "bold,italic"), the rendered output should combine all attributes correctly.

**Validates: Requirements 3.5**

### Test Case 15: Compact format width

When calculating display width for a series entry, the width calculation should account for the new compact count format.

**Validates: Requirements 4.1**

### Test Case 16: Terminal width handling

When the terminal width is limited, the count display should not cause text wrapping or overflow.

**Validates: Requirements 4.2**

### Test Case 17: Consistent alignment

When displaying multiple entries with varying name lengths, all count texts should align to the same right-justified position.

**Validates: Requirements 4.3**

### Test Case 18: Minimal spacing

When rendering count text, the spacing should be minimal while maintaining readability (at least 1 space).

**Validates: Requirements 4.4**

## Error Handling

1. **Database Query Errors**: If count queries fail, fall back to "? episodes" format (existing behavior)
2. **Invalid Color Values**: Use default colors (existing `string_to_fg_color_or_default` behavior)
3. **Invalid Style Values**: Log warning and use "none" style
4. **Terminal Width Calculation**: Use `saturating_sub` to prevent underflow
5. **Configuration Parse Errors**: Use serde defaults for missing fields

Error handling follows existing patterns:
- Database errors return `Result` types
- Configuration errors log warnings and use defaults
- Display errors propagate as `io::Result`

## Testing Strategy

### Unit Testing

1. **Format Function Tests**:
   - Test `format_series_display` with various count combinations
   - Test `format_season_display` with various count combinations
   - Test right-justification logic with different widths
   - Test spacing calculation edge cases

2. **Configuration Tests**:
   - Test loading config with new fields
   - Test loading config with missing fields (defaults)
   - Test saving config includes new fields
   - Test invalid style value handling

3. **Style Application Tests**:
   - Test `apply_text_style` with new style combinations
   - Test color application with count-specific colors
   - Test style combination (bold+italic)

### Integration Testing

1. **End-to-End Display**:
   - Verify series entries display with new format
   - Verify season entries display with new format
   - Verify counts are right-justified
   - Verify styling is applied correctly

2. **Configuration Integration**:
   - Verify config changes affect display
   - Verify defaults work when fields missing
   - Verify config persistence

### Edge Cases

1. **Zero episodes**: Display "0/0 watched"
2. **All watched**: Display "5/5 watched"
3. **None watched**: Display "0/5 watched"
4. **Very long names**: Truncate name, preserve count
5. **Narrow terminal**: Gracefully handle limited width
6. **Database errors**: Fall back to "?" format

## Implementation Notes

1. **Backward Compatibility**: All new config fields use serde defaults, so existing config files continue to work
2. **Performance**: No additional database queries (reuse existing count functions)
3. **Styling**: Leverage existing `apply_text_style` function
4. **Color System**: Use existing color parsing infrastructure
5. **Terminal Width**: Pass from caller to avoid repeated queries

## Migration Path

1. Users with existing `config.yaml` files will automatically get defaults for new fields
2. Next config save will include new fields with inline documentation
3. No database migration needed
4. No breaking changes to existing functionality
