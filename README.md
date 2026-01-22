# Video Library Manager

```
                                                           ██                         ██████
     ████                                                  ██                       ███   ██
  █ ██████              ██████        ███        █                     ███        ████    ██
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
                -- written by Eric Heimerman (with a little help from Kiro)         ███
```

Your personal video collection, organized and accessible from your terminal.

## Important: Your Videos Are Safe

**This program does not modify your video files.** It only reads file information and writes to its own `videos.sqlite` database. The program does not need write access to your video files themselves - only to the database file in your video folder.

That said, this software is provided as-is with no guarantees. Always maintain backups of important files.

## What is this?

This is a simple program that helps you organize and browse your video collection. Think of it as a personal Netflix interface for your own movies and TV shows - but running right in your terminal. You can:

- Browse your entire video collection in one place
- Organize TV shows into series and seasons
- Track which episodes you've watched with automatic progress tracking
- Resume watching from where you left off (with Celluloid player)
- Search for videos by typing
- Launch videos in your favorite player (like VLC or Celluloid)

Perfect for anyone with a large collection of movies or TV shows who wants a better way to keep track of what they have and what they've watched.

## Why use this?

If you have folders full of video files scattered across your computer, this tool brings them all together. Instead of clicking through folders trying to remember where you put that one episode, you can:

- See your entire collection at a glance
- Organize TV shows properly (no more "Season 1 Episode 3" in the filename)
- Remember what you've already watched
- Find videos quickly by searching
- Keep everything organized even if you move files around

## Getting Started

### What you'll need

- A computer running Linux, macOS, or Windows
- Rust programming language installed ([get it here](https://www.rust-lang.org/tools/install))
- A video player like VLC ([download VLC](https://www.videolan.org/))

### Installation

1. Download this project (or clone it if you know git):
   ```sh
   git clone <repository-url>
   cd <project-folder>
   ```

2. Build the program:
   ```sh
   cargo build --release
   ```

3. Run it:
   ```sh
   cargo run --release
   ```

The first time you run it, the program will create a configuration file and database for you.

### First-time setup

The first time you run the program, it will:

1. Create a `config.yaml` file in your system's config directory:
   - Linux: `~/.config/movies/config.yaml`
   - macOS: `~/Library/Application Support/movies/config.yaml`
   - Windows: `%APPDATA%\movies\config.yaml`
2. Create a `movies.log` file in your system's data directory:
   - Linux: `~/.local/share/movies/movies.log`
   - macOS: `~/Library/Application Support/movies/movies.log`
   - Windows: `%APPDATA%\movies\movies.log`
3. Prompt you to enter the path to your video folder
4. Create a `videos.sqlite` database file in your video folder

Just enter the path where your videos are stored (e.g., `/home/yourname/Videos`) and the program will be ready to use!

**Note:** The video player defaults to `/usr/bin/vlc`. If you use a different player, edit the `video_player:` setting in `config.yaml`. For automatic progress tracking and resume functionality, use Celluloid (`/usr/bin/celluloid`). See the [Configuration Guide](docs/CONFIGURATION.md) for all available options.

## How to use it

### Adding your videos

Press **F1** to open the menu, then press **S** to rescan your video folder. The program will find all your video files and add them to the library. This might take a minute if you have a lot of videos.

### Browsing your collection

Use the **arrow keys** (up and down) to move through your videos. You'll see:
- Standalone episodes (videos not organized into series)
- TV series (which you can enter to see seasons)
- Seasons (which you can enter to see episodes)

When you have more items than fit on screen, a scroll bar appears on the right side showing your position in the list. The indicator moves as you scroll, helping you understand how much content is above and below your current position.

Press **Enter** to:
- Play a video
- Open a series to see its seasons
- Open a season to see its episodes

Press **Esc** to go back to the previous screen.

### Searching for videos

Press **/** to enter search mode, then start typing. As you type, the list will filter to show only matching videos. Press **Enter** to accept the filter, or **Esc** to cancel and clear the search.

### Organizing TV shows

Have a bunch of TV show episodes? Here's how to organize them:

1. Navigate to an episode
2. Press **F4** (or **F1** for the menu, then select "assign to series")
3. Choose an existing series or press **+** to create a new one
4. Type the series name and press **Enter**

For the next episode from the same show, just press **F5** to quickly assign it to the same series and season!

### Tracking what you've watched

Navigate to any episode and press **F3** to mark it as watched (or unwatched). Watched episodes show a special indicator so you can see at a glance what you've already seen.

**Progress Tracking:** The program automatically tracks your viewing progress when using Celluloid as your video player:
- When you start watching an episode, it's automatically marked as unwatched
- Progress is saved when you exit Celluloid with **Shift+Q** (not the X button)
- When you reach 95% of the video, it's automatically marked as watched
- Toggling watched status (F3) resets progress and starts fresh next time
- The `last_watched_time` field preserves when you last watched an episode

**Note:** To save progress, you must exit Celluloid using **Shift+Q**. Closing with the X button does not save your position.

Want to rewatch a whole series? Press **F7** to mark all episodes as unwatched. This works on:
- The entire library (if you're at the top level)
- Just one series (if you're viewing a series)
- Just one season (if you're viewing a season)

### Editing episode details

Navigate to an episode and press **F2** to edit its details:
- Name
- Year
- Length (in minutes)
- Episode number
- Season number

Use the arrow keys to move between fields, type to edit, and press **F2** again to save your changes (or **Esc** to cancel).

## Quick reference

### Main controls

| Key | What it does |
|-----|--------------|
| **Arrow keys** | Move up and down through your videos |
| **Enter** | Play video or open series/season |
| **Esc** | Go back to previous screen / Exit the program |
| **/** | Enter search/filter mode |
| **F1** | Open menu to see all available actions |

### Quick actions (when viewing an episode)

| Key | What it does |
|-----|--------------|
| **F2** | Edit episode details |
| **F3** | Mark as watched/unwatched |
| **F4** | Organize into a series |
| **F5** | Repeat last organization (quick assign) |
| **F6** | Remove from series (make standalone) |
| **F7** | Mark all as unwatched |

**Tip:** Press **F1** anytime to see a menu of what you can do with the currently selected item.

## Common questions

### Where is my data stored?

The program creates a `videos.sqlite` file in your video folder (the `db_location` you specified). This database remembers all your organization and watched status. 

**Your actual video files are never modified** - the program only reads them and stores metadata in the database. The program only needs write access to the `videos.sqlite` database file, not to your video files.

Configuration is stored in your system's config directory (`~/.config/movies` on Linux), and logs are in your system's data directory (`~/.local/share/movies` on Linux).

### Can I move my video files?

Yes! The program stores file locations relative to your `db_location`. When you move your video collection:
1. Move the entire folder (including the `videos.sqlite` database file)
2. Update `db_location` in `config.yaml` to the new path
3. All your organization and watched status will be preserved

### What video formats are supported?

By default: MP4, MKV, AVI, MOV, FLV, WMV, and WebM. You can add more in `config.yaml`.

### Can I customize colors and appearance?

Yes! Edit `config.yaml` to change colors, scrollbar characters, and the watched indicator. See the [Configuration Guide](docs/CONFIGURATION.md) for details.

### I moved/deleted some videos. How do I update the library?

Press **F1** to open the menu, then press **S** to rescan. The program will update its database to match what's actually in your video folder.

## Troubleshooting

Having issues? Check the [Troubleshooting Guide](docs/TROUBLESHOOTING.md) for solutions to common problems:

- Installation and build issues
- Video playback problems
- Library scanning issues
- Display and terminal problems
- Database and performance issues

## For developers

If you're interested in the technical details, architecture, or want to contribute to the project, check out [CONTRIBUTING.md](CONTRIBUTING.md) for developer documentation.

## Feedback & Contributions

This project was built with AI assistance, and we welcome your feedback and contributions!

### Report Issues or Request Features

- 🐛 **Found a bug?** [Report it here](https://github.com/eheimer/movies/issues/new?template=bug_report.md)
- 💡 **Have a feature idea?** [Request it here](https://github.com/eheimer/movies/issues/new?template=feature_request.md)
- ❓ **Have a question?** [Ask here](https://github.com/eheimer/movies/issues/new?template=question.md)

### Contribute Code

Want to contribute code? Check out [CONTRIBUTING.md](CONTRIBUTING.md) for developer documentation and guidelines.

Don't hesitate to critique the code or suggest better approaches - that's how we all learn and improve!

## License

This project is open-source under the MIT License. Feel free to use it, modify it, and share it!
