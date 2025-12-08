# Design Document

## Overview

The ASCII splash screen feature adds a startup screen to the movies application that displays for several seconds before the main interface loads. The implementation will use pre-generated ASCII art stored as a string constant, display it using crossterm for terminal manipulation, and integrate into the existing application startup sequence in `main.rs`.

## Architecture

The splash screen will be implemented as a standalone module (`src/splash.rs`) that is called once during application startup, before the main event loop begins. The module will:

1. Clear the terminal and hide the cursor
2. Render centered ASCII art and tagline
3. Wait for a fixed duration (2-3 seconds)
4. Return control to main for normal application flow

The design maintains separation of concerns by keeping splash screen logic isolated from the main application logic, and reuses existing terminal manipulation infrastructure (crossterm).

## Components and Interfaces

### Splash Module (`src/splash.rs`)

**Public Interface:**
```rust
pub fn show_splash_screen() -> Result<(), Box<dyn std::error::Error>>
```

**Internal Functions:**
```rust
fn get_ascii_art() -> &'static str
fn get_tagline() -> &'static str
fn center_text(text: &str, terminal_width: u16) -> String
fn render_splash(ascii_art: &str, tagline: &str, terminal_width: u16, terminal_height: u16)
```

### Integration Point (`src/main.rs`)

The splash screen will be called immediately after terminal initialization and before the main event loop:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::init()?;
    
    // Show splash screen
    splash::show_splash_screen()?;
    
    // Existing initialization code...
    let mut mode = Mode::Browse;
    // ... rest of main loop
}
```

## Data Models

### ASCII Art Storage

The ASCII art will be stored as a static string constant. For a script-style "movies" text that scales to terminal width, we'll use a medium-sized ASCII art (approximately 100 characters wide) that can be centered on most terminal displays:

```
                                                           ██                         ██████
     ████                                                 ████                      ███   ██
  █ ██████              ██████        ███        █         █           ███        ████    ██
██████ ███ ████        ████ ███       ███       ███       ███       ███████       ███     ██
██████ ████████       ███ █████       ████     █████     ████      ████  ██       ████            ████
█████  █████████     ████  ███████████████     ██  ██████████      ███████       ███████      ██████
█████  ████ ███     █████    ███ ██    ███    ███        ████     ████          ███  ███████████
████   ███  ███    ██████    ██        ███    ██         ████    █████        ███      █████
████   ███  ███    ██ ███    ██        ████   ██         ████   ██████       ███     ████████
████        ███  ███   ███████          ███  ██           ███  ███  ████ █████     ███    ████
███         ██████       ███             ██████           ███ ███     █████       ███     ███
             ███                           ██               ███                   ██     ███
                                                                                  █████████
                                                                                    ███
```

### Display Parameters

- **Display Duration**: 2.5 seconds (configurable constant)
- **Vertical Centering**: ASCII art positioned at 40% from top of screen
- **Horizontal Centering**: Both ASCII art and tagline centered based on terminal width
- **Tagline Offset**: 2 lines below ASCII art

## Test Cases

### Test Case 1: Splash screen displays on startup

When the application starts, the splash screen should be displayed before the main browse screen appears.
**Validates: Requirements 1.1**

### Test Case 2: Splash screen duration

When the splash screen is shown, it should remain visible for at least 2 seconds before transitioning.
**Validates: Requirements 1.2**

### Test Case 3: ASCII art content

When the splash screen renders, it should display ASCII art containing the text "movies".
**Validates: Requirements 2.1**

### Test Case 4: Tagline content

When the splash screen renders, it should display the tagline "-- written by Eric Heimerman (with a little bit of help from Kiro)".
**Validates: Requirements 3.1**

### Test Case 5: Tagline positioning

When the splash screen displays, the tagline should appear below the ASCII art.
**Validates: Requirements 3.2**

### Test Case 6: Text centering

When the splash screen renders, both the ASCII art and tagline should be horizontally centered.
**Validates: Requirements 2.4, 3.3**

### Test Case 7: Terminal cleanup

When the splash screen completes, the terminal should be cleared before the main browse screen renders.
**Validates: Requirements 4.3**

### Test Case 8: Module separation

When examining the codebase, the splash screen logic should be in a separate module from main application logic.
**Validates: Requirements 4.1**

## Error Handling

The splash screen function returns a `Result` type to handle potential errors:

- **Terminal size query failures**: If terminal dimensions cannot be determined, use default values (80x24)
- **Rendering failures**: Propagate crossterm errors up to main for consistent error handling
- **Timing failures**: Use `std::thread::sleep` which is infallible for the duration wait

Error handling follows the existing pattern in the codebase of returning `Result<(), Box<dyn std::error::Error>>` and using `.expect()` with descriptive messages at call sites.

## Testing Strategy

### Unit Testing

1. **Text centering logic**: Test `center_text()` function with various terminal widths and text lengths
   - Empty text
   - Text wider than terminal
   - Text narrower than terminal
   - Odd and even width terminals

2. **ASCII art retrieval**: Verify `get_ascii_art()` returns non-empty string containing "movies"

3. **Tagline retrieval**: Verify `get_tagline()` returns the correct attribution text

### Integration Testing

1. **Startup sequence**: Verify splash screen is called before main event loop in integration test
2. **Terminal state**: Verify terminal is properly cleared after splash screen completes
3. **Visual verification**: Manual testing to confirm appearance and timing

### Edge Cases

1. **Very narrow terminals**: Test behavior when terminal width is less than ASCII art width
2. **Very short terminals**: Test behavior when terminal height is insufficient for full display
3. **Terminal resize during splash**: Document that splash screen uses initial terminal size (resize during 2.5 second display is acceptable edge case)
