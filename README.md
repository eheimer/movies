# Video File Browser

## Overview

A terminal-based video library manager written in Rust. Browse, organize, and play your video collection with support for series/season/episode organization, metadata tracking, and a keyboard-driven interface.

## Features

### Core Functionality
- **Browse and Play**: Navigate your video library and launch videos in your configured player
- **Series Organization**: Organize videos into series, seasons, and episodes
- **Metadata Tracking**: Track watched status, year, length, and episode numbers
- **Real-time Search**: Filter entries by typing - search clears automatically when navigating
- **SQLite Database**: Persistent storage with relative path support for portability
- **Multi-level Navigation**: Browse top-level entries, drill into series and seasons

### Library Management
- **Import Videos**: Scan directories to add new videos to your library
- **Assign to Series**: Organize standalone episodes into series and seasons
- **Repeat Actions**: Quickly apply the same series/season assignment to multiple episodes
- **Clear Series Data**: Remove series/season assignments from episodes
- **Bulk Operations**: Mark all episodes as unwatched (context-aware: all, series, or season)
- **Rescan Library**: Refresh your library to detect new or removed files

### User Interface
- **Context Menu (F1)**: Access all available actions for the selected item
- **Function Key Shortcuts**: Quick access to common operations (F2-F7)
- **Color-coded Display**: Visual distinction between series, seasons, and episodes
- **Watched Indicators**: See at a glance which episodes you've watched
- **Smart Availability**: Menu items show/hide based on context and selection

## Dependencies

- **crossterm** (0.23): Terminal manipulation and event handling
- **colored** (2.0): Terminal color output
- **rusqlite** (0.26.0): SQLite database interface
- **serde** (1.0) + **serde_yaml**: Configuration serialization
- **walkdir** (2.3): Recursive directory traversal
- **lazy_static** (1.4): Global database connection management

## Installation

Ensure you have Rust installed. Then, clone the repository and build the project:

```sh
git clone <repository-url>
cd <project-folder>
cargo build --release
```

## Configuration

The application uses a `config.yaml` file in the project root. If it doesn't exist, it will be created with default values on first run.

Example `config.yaml`:

```yaml
# === Database Configuration ===
db_location: null

# === Color Configuration ===
current_fg: Black
current_bg: White

# === Video Configuration ===
video_extensions:
  - mp4
  - mkv
  - avi
  - mov
  - flv
  - wmv
  - webm
video_player: /usr/bin/vlc
```

### Configuration Options

The `config.yaml` file includes inline documentation for all settings. Key options include:

- **db_location**: Path to the SQLite database file (null uses default location)
- **current_fg/current_bg**: Colors for selected items
- **video_extensions**: Supported video file formats
- **video_player**: Path to your video player executable (e.g., VLC, mpv)
- **log_level**: Logging verbosity (error, warn, info, debug)

See the generated `config.yaml` file for complete documentation of all available settings.

### Database

The SQLite database (`videos.db`) is stored in the same directory as the executable. Video file paths are stored relative to `root_dir` for portability across systems.

## Usage

Run the program with:

```sh
cargo run --release
```

### Keyboard Controls

#### Browse Mode (Main Navigation)
- **Arrow Keys (Up/Down)**: Navigate through entries
- **Enter**: 
  - Play selected episode
  - Enter selected series (view seasons)
  - Enter selected season (view episodes)
- **Backspace**: Go back to previous view level
- **Esc**: Exit the application
- **Type characters**: Filter entries in real-time (clears automatically when navigating)
- **F1**: Open context menu
- **F2**: Edit episode details (when episode selected)
- **F3**: Toggle watched status (when episode selected)
- **F4**: Assign episode to series (when unassigned episode selected)
- **F5**: Repeat last series/season assignment (when available)
- **F6**: Clear series data from episode (when episode has series data)
- **F7**: Mark all episodes as unwatched (context-aware)
- **Ctrl+L**: Rescan library for new/removed files

#### Edit Mode (Episode Details)
- **Arrow Keys (Up/Down)**: Navigate between fields
- **Enter**: Edit selected field
- **Esc**: Return to browse mode without saving
- **Ctrl+S**: Save changes and return to browse mode

#### Entry Mode (Import Videos)
- **Type path**: Enter directory path to scan
- **Enter**: Import videos from entered path
- **Esc**: Cancel and return to browse mode
- **Backspace**: Delete last character

#### Series Selection Mode
- **Arrow Keys (Up/Down)**: Navigate series list
- **Enter**: Assign episode to selected series
- **Esc**: Cancel and return to browse mode
- **Type characters**: Filter series by name
- **Ctrl+N**: Create new series

#### Series Creation Mode
- **Type name**: Enter new series name
- **Enter**: Create series and assign episode
- **Esc**: Cancel and return to series selection

#### Menu Mode (F1 Context Menu)
- **Arrow Keys (Up/Down)**: Navigate menu items
- **Enter**: Execute selected action
- **Function Keys (F2-F7)**: Execute action directly
- **Esc**: Close menu and return to browse mode

### Workflow Examples

#### Organizing a New TV Series
1. Press **Ctrl+L** to scan for new files
2. Navigate to an episode and press **F4** (Assign to Series)
3. Press **Ctrl+N** to create a new series, enter the name
4. Select the season for this episode
5. For subsequent episodes, press **F5** (Repeat Action) to quickly assign them to the same series/season

#### Marking Episodes as Watched
1. Navigate to an episode
2. Press **F3** to toggle watched status
3. Or press **F1** and select "toggle watched"

#### Bulk Unwatching
1. Navigate to a series or season (or stay at top level)
2. Press **F7** to mark all episodes as unwatched in that context
3. Confirm the action

#### Removing Series Organization
1. Navigate to an episode that's part of a series
2. Press **F6** to clear all series/season data
3. The episode becomes standalone again

## Architecture

### Data Model
- **Episodes**: Individual video files (can be standalone or part of a series)
- **Series**: Collections of related episodes (e.g., TV shows)
- **Seasons**: Organizational units within a series
- **Metadata**: Each episode tracks name, location, watched status, year, length, and episode number

### Path Resolution
- Video file paths are stored relative to the configured `root_dir`
- This allows the database to be portable across different systems
- The database file itself is always stored in the executable directory
- Files outside the `root_dir` are skipped during import with a warning

### View Contexts
The application maintains context awareness for operations:
- **Top Level**: All series and standalone episodes
- **Series View**: All seasons within a selected series
- **Season View**: All episodes within a selected season

Context-aware operations (like "Unwatch All") automatically scope to the current view.

## Building from Source

```sh
# Clone the repository
git clone <repository-url>
cd <project-folder>

# Build release version
cargo build --release

# Run tests
cargo test

# Run with optimizations
cargo run --release
```

## License

This project is open-source under the MIT License.
