# Video File Browser

## Overview

This is a terminal-based video file browser written in Rust. It allows users to navigate through a directory of video files, filter them by name, and play selected videos using a specified video player.

## Features

- Browse video files in a specified directory
- Filter files dynamically by typing
- Navigate using arrow keys
- Play videos using a configurable video player
- Uses `crossterm` for a terminal-based UI
- Configurable via a `config.json` file

## Dependencies

This program requires the following Rust crates:

- `crossterm` (for terminal manipulation)
- `serde` and `serde_json` (for configuration management)
- `walkdir` (for directory traversal)

## Installation

Ensure you have Rust installed. Then, clone the repository and build the project:

```sh
git clone <repository-url>
cd <project-folder>
cargo build --release
```

## Configuration

A `config.json` file is used to configure the program. If it does not exist, it will be created with default values.

Example `config.json`:

```json
{
  "path": "./videos",
  "current_fg": "Black",
  "current_bg": "White",
  "video_extensions": ["mp4", "mkv", "avi", "mov", "flv", "wmv", "webm"],
  "video_player": "/usr/bin/vlc"
}
```

- `path`: The directory to scan for video files
- `current_fg`: Foreground color for selection
- `current_bg`: Background color for selection
- `video_extensions`: Supported video file formats
- `video_player`: Path to the video player executable

## Usage

Run the program with:

```sh
cargo run --release
```

### Controls

- `Up/Down Arrow Keys`: Navigate the file list
- `Enter`: Play selected video
- `Esc`: Exit the application
- `Backspace`: Remove the last character from the search filter
- Typing: Filters files by name

## License

This project is open-source under the MIT License.
