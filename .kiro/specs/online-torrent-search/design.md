# Design Document

## Overview

This feature integrates torrent search functionality into the existing terminal-based video library manager. It adds two new modes (TorrentSearchInput and TorrentSearchResults) to the application's state machine, uses the `magneto` Rust crate to query The Pirate Bay, and leverages OS-level magnet link handlers to initiate downloads in the user's torrent client.

## Architecture

### High-Level Flow

```
Browse Mode → Menu Item "Search Online" → TorrentSearchInput Mode
                                                ↓
                                          Enter search query
                                                ↓
                                          Query PirateBay API
                                                ↓
                                          TorrentSearchResults Mode
                                                ↓
                                    Select result & press Enter
                                                ↓
                                    Open magnet link via OS handler
                                                ↓
                                          Back to Browse Mode
```

### Integration Points

1. **Mode System** (`src/util.rs`): Add two new mode variants
2. **Menu System** (`src/menu.rs`): Add new menu action and item
3. **Event Handlers** (`src/handlers.rs`): Add handlers for new modes
4. **Display System** (`src/display.rs`): Add rendering for new modes
5. **New Module** (`src/torrent_search.rs`): Encapsulate torrent search logic

## Components and Interfaces

### 1. Mode Enum Extension

Add to `src/util.rs`:

```rust
pub enum Mode {
    // ... existing modes
    TorrentSearchInput,
    TorrentSearchResults,
}
```

### 2. Torrent Search Module

New file `src/torrent_search.rs`:

```rust
use magneto::{Magneto, SearchRequest, Torrent, Category};

// Torrent search result with formatted display fields
pub struct TorrentResult {
    pub name: String,
    pub uploaded: String,
    pub size: String,
    pub seeders: u32,
    pub leechers: u32,
    pub magnet_link: String,
}

// Search The Pirate Bay for movies matching the query
pub async fn search_torrents(query: &str) -> Result<Vec<TorrentResult>, Box<dyn std::error::Error>> {
    // Initialize magneto with PirateBay provider only
    // Create search request with Movies category filter
    // Execute search and get top 5 results by seeders
    // Convert to TorrentResult format
}

// Open a magnet link using the OS default handler
pub fn open_magnet_link(magnet_link: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Use std::process::Command to call OS-specific opener
    // Linux: xdg-open
    // macOS: open
    // Windows: start
}
```

### 3. Menu System Updates

Add to `src/menu.rs`:

```rust
pub enum MenuAction {
    // ... existing actions
    SearchOnline,
}

// In get_menu_items():
MenuItem {
    label: "Search Online".to_string(),
    hotkey: Some(KeyCode::F(8)),  // Next available F-key
    action: MenuAction::SearchOnline,
    location: MenuLocation::ContextMenu,
}

// In is_item_available():
MenuAction::SearchOnline => {
    matches!(context.mode, Mode::Browse)
}
```

### 4. Event Handler Updates

Add to `src/handlers.rs`:

```rust
// In execute_menu_action():
MenuAction::SearchOnline => {
    *mode = Mode::TorrentSearchInput;
    *search_query = String::new();
    *redraw = true;
}

// New handler for TorrentSearchInput mode
pub fn handle_torrent_search_input(
    event: KeyEvent,
    mode: &mut Mode,
    search_query: &mut String,
    torrent_results: &mut Vec<TorrentResult>,
    redraw: &mut bool,
) {
    match event.code {
        KeyCode::Char(c) => {
            search_query.push(c);
            *redraw = true;
        }
        KeyCode::Backspace => {
            search_query.pop();
            *redraw = true;
        }
        KeyCode::Enter if !search_query.is_empty() => {
            // Execute async search
            // Store results in torrent_results
            *mode = Mode::TorrentSearchResults;
            *redraw = true;
        }
        KeyCode::Esc => {
            *mode = Mode::Browse;
            *redraw = true;
        }
        _ => {}
    }
}

// New handler for TorrentSearchResults mode
pub fn handle_torrent_search_results(
    event: KeyEvent,
    mode: &mut Mode,
    torrent_results: &[TorrentResult],
    selected_result: &mut usize,
    status_message: &mut String,
    redraw: &mut bool,
) {
    match event.code {
        KeyCode::Up => {
            if *selected_result > 0 {
                *selected_result -= 1;
                *redraw = true;
            }
        }
        KeyCode::Down => {
            if *selected_result < torrent_results.len() - 1 {
                *selected_result += 1;
                *redraw = true;
            }
        }
        KeyCode::Enter => {
            let result = &torrent_results[*selected_result];
            if let Err(e) = open_magnet_link(&result.magnet_link) {
                *status_message = format!("Error opening magnet link: {}", e);
            } else {
                *status_message = format!("Initiated download: {}", result.name);
                *mode = Mode::Browse;
            }
            *redraw = true;
        }
        KeyCode::Esc => {
            *mode = Mode::Browse;
            *redraw = true;
        }
        _ => {}
    }
}
```

### 5. Display Updates

Add to `src/display.rs`:

```rust
// Render TorrentSearchInput mode
pub fn draw_torrent_search_input(
    buffer: &mut ScreenBuffer,
    search_query: &str,
    theme: &Theme,
) {
    // Display header: "Search Online - The Pirate Bay"
    // Display input field with current query
    // Display instructions: "Enter: Search | ESC: Cancel"
}

// Render TorrentSearchResults mode
pub fn draw_torrent_search_results(
    buffer: &mut ScreenBuffer,
    results: &[TorrentResult],
    selected_index: usize,
    theme: &Theme,
) {
    // Display header: "Search Results (Top 5)"
    // Display table with columns: Title | Uploaded | Size | Seeds | Leeches
    // Highlight selected row
    // Display instructions: "↑↓: Navigate | Enter: Download | ESC: Cancel"
}
```

### 6. Main Loop Integration

Update `src/main.rs`:

```rust
// Add new state variables
let mut search_query = String::new();
let mut torrent_results: Vec<TorrentResult> = Vec::new();
let mut selected_torrent_result = 0;

// In event loop, add mode handlers
Mode::TorrentSearchInput => {
    handle_torrent_search_input(
        event,
        &mut mode,
        &mut search_query,
        &mut torrent_results,
        &mut redraw,
    );
}
Mode::TorrentSearchResults => {
    handle_torrent_search_results(
        event,
        &mut mode,
        &torrent_results,
        &mut selected_torrent_result,
        &mut status_message,
        &mut redraw,
    );
}

// In display section
Mode::TorrentSearchInput => {
    draw_torrent_search_input(&mut buffer, &search_query, &theme);
}
Mode::TorrentSearchResults => {
    draw_torrent_search_results(&mut buffer, &torrent_results, selected_torrent_result, &theme);
}
```

## Data Models

### TorrentResult

```rust
pub struct TorrentResult {
    pub name: String,          // Torrent title
    pub uploaded: String,      // Upload date (formatted)
    pub size: String,          // File size (formatted, e.g., "1.5 GB")
    pub seeders: u32,          // Number of seeders
    pub leechers: u32,         // Number of leechers
    pub magnet_link: String,   // Magnet URI
}
```

## Logging

All torrent search operations should be logged to the application log file for debugging and testing purposes:

1. **Search Initiation**: Log the search query and provider being used
2. **Search Results**: Log the number of results returned and details of each result (name, seeders, size)
3. **Result Selection**: Log which result the user selected
4. **Magnet Link Opening**: Log the magnet link being opened and the command used
5. **Errors**: Log all error conditions with full error details

Example log entries:
```
[INFO] Torrent search initiated: query="minority report", provider=PirateBay
[INFO] Torrent search returned 5 results
[DEBUG] Result 0: "Minority Report (2002)" seeds=150 size=1.5GB
[INFO] User selected torrent result 0: "Minority Report (2002)"
[INFO] Opening magnet link: magnet:?xt=urn:btih:...
[ERROR] Failed to open magnet link: No application registered for magnet URIs
```

## Error Handling

1. **Network Errors**: If the search request fails (network timeout, API unavailable), log the error, display error message, and return to Browse mode
2. **No Results**: If search returns zero results, log the query, display "No results found" message and allow user to try again or cancel
3. **Magnet Link Opening Errors**: If OS cannot open magnet link (no torrent client installed), log the error, display error message but remain in results mode so user can try another result

## Testing Strategy

### Unit Tests

- Test torrent search query construction with various inputs
- Test result parsing and formatting (size conversion, date formatting)
- Test OS-specific magnet link opening command construction
- Test mode transitions and state management

### Integration Tests

- Test full search flow from input to results display
- Test navigation in results list (up/down arrow keys)
- Test cancellation at each stage (ESC key)
- Test error handling for network failures

### Manual Testing

- Verify search results match expected format
- Verify magnet links open in system torrent client
- Verify status messages display correctly
- Test with various search queries (special characters, long titles, etc.)
