# Configuration Guide

## Configuration File Location

The `config.yaml` file is created in your system's config directory when you first run the application:

- **Linux:** `~/.config/movies/config.yaml`
- **macOS:** `~/Library/Application Support/movies/config.yaml`
- **Windows:** `%APPDATA%\movies\config.yaml`

## Basic Settings

### Database Location (Video Root Directory)

```yaml
db_location: /home/yourname/Videos
```

This is the root directory for your video collection. The program:
- Looks for all video files in this directory and its subdirectories
- Stores the `videos.sqlite` database file in this directory
- Stores all file paths relative to this directory for portability

This is set the first time you run the program when it prompts you for your video folder path.

**Important:** There can be only one video root directory, and the database must always be stored in this location.

### Video Player

```yaml
video_player: /usr/bin/vlc
```

Path to your preferred video player executable. Common options:
- Linux: `/usr/bin/vlc`, `/usr/bin/mpv`
- macOS: `/Applications/VLC.app/Contents/MacOS/VLC`
- Windows: `C:\Program Files\VideoLAN\VLC\vlc.exe`

## Appearance

### Colors

Customize the colors for the currently selected item:

```yaml
current_fg: "Black"
current_bg: "White"
```

Available colors: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, Reset

### Scrollbar

When lists are longer than the screen, a scrollbar appears on the right side:

```yaml
scrollbar_track_char: "│"
scrollbar_indicator_char: "█"
scrollbar_fg: "White"
scrollbar_bg: "Reset"
```

- `scrollbar_track_char`: Character for the scrollbar track
- `scrollbar_indicator_char`: Character showing your position
- `scrollbar_fg`: Foreground color
- `scrollbar_bg`: Background color

### Watched Indicator

```yaml
watched_indicator: "✓"
```

Character displayed next to episodes you've marked as watched.

## Video Formats

```yaml
video_extensions:
  - mp4
  - mkv
  - avi
  - mov
  - flv
  - wmv
  - webm
```

Add or remove extensions to match your video collection.

## Logging

```yaml
log_file: null
log_level: Info
```

By default the log file is stored in your system's data directory:
- **Linux:** `~/.local/share/movies/movies.log`
- **macOS:** `~/Library/Application Support/movies/movies.log`
- **Windows:** `%APPDATA%\movies\movies.log`

Log levels: Error, Warn, Info, Debug, Trace

## Example Configuration

```yaml
db_location: /home/user/Videos
current_fg: "Black"
current_bg: "Cyan"
scrollbar_track_char: "│"
scrollbar_indicator_char: "█"
scrollbar_fg: "Cyan"
scrollbar_bg: "Reset"
watched_indicator: "✓"
video_extensions:
  - mp4
  - mkv
  - avi
  - mov
video_player: /usr/bin/vlc
log_file: null
log_level: Info
```

## Path Portability

The program stores video file paths relative to `db_location`. This means:

- You can move your entire video collection to a new location
- The `videos.sqlite` database file moves with your videos
- Either update the db_location or the next time the program starts, it will prompt you for the new location and all of your organization and watched status will be preserved
- The video collection can be shared across different systems
- I personally use this to store my videos on a USB drive along with the videos.sqlite database.  Whichever computer I plug it into, then, my collection will be available as long as I have the movies executable installed.

