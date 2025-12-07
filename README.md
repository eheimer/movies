# Video Library Manager

Your personal video collection, organized and accessible from your terminal.

## What is this?

This is a simple program that helps you organize and browse your video collection. Think of it as a personal Netflix interface for your own movies and TV shows - but running right in your terminal. You can:

- Browse your entire video collection in one place
- Organize TV shows into series and seasons
- Track which episodes you've watched
- Search for videos by typing
- Launch videos in your favorite player (like VLC)

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

When you first run the program, you'll need to tell it where your videos are:

1. The program will create a `config.yaml` file
2. Open this file in any text editor
3. Find the line that says `root_dir:` and set it to your video folder path
   - Example: `root_dir: /home/yourname/Videos`
4. Make sure `video_player:` points to your video player
   - Example: `video_player: /usr/bin/vlc`

Save the file and run the program again.

## How to use it

### Adding your videos

Press **F1** to open the menu, then press **S** to rescan your video folder. The program will find all your video files and add them to the library. This might take a minute if you have a lot of videos.

### Browsing your collection

Use the **arrow keys** (up and down) to move through your videos. You'll see:
- Standalone episodes (videos not organized into series)
- TV series (which you can enter to see seasons)
- Seasons (which you can enter to see episodes)

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

The program creates a `videos.db` file in the same folder as the program. This database remembers all your organization and watched status. Your actual video files stay exactly where they are - the program just keeps track of them.

### Can I move my video files?

Yes! The program stores file locations relative to your `root_dir`, so as long as you update the `root_dir` in `config.yaml` when you move files, everything will still work.

### What video formats are supported?

By default: MP4, MKV, AVI, MOV, FLV, WMV, and WebM. You can add more formats in the `config.yaml` file.

### Can I customize the colors?

Yes! Edit the `config.yaml` file and change the `current_fg` and `current_bg` settings to your preferred colors.

### I moved/deleted some videos. How do I update the library?

Press **F1** to open the menu, then press **S** to rescan. The program will update its database to match what's actually in your video folder.

## Troubleshooting

**Problem:** The program won't start
- Make sure you've built it with `cargo build --release`
- Check that you have Rust installed correctly

**Problem:** Videos won't play
- Check that `video_player` in `config.yaml` points to the correct path
- Make sure your video player (like VLC) is installed

**Problem:** The program can't find my videos
- Check that `root_dir` in `config.yaml` points to the right folder
- Press **F1** then **S** to rescan for videos
- Make sure your video files have supported extensions (mp4, mkv, etc.)

**Problem:** The interface looks weird
- Make sure your terminal window is large enough (at least 80 characters wide)
- Try a different terminal emulator if colors don't display correctly

## For developers

If you're interested in the technical details, architecture, or want to contribute to the project, check out [CONTRIBUTING.md](CONTRIBUTING.md) for developer documentation.

## Contributing

This project was built with AI assistance, and we welcome feedback and contributions! If you find bugs, have ideas for improvements, or want to add features, please:

- Open an issue on GitHub
- Submit a pull request
- Share your thoughts on how to make this better

Don't hesitate to critique the code or suggest better approaches - that's how we all learn and improve!

## License

This project is open-source under the MIT License. Feel free to use it, modify it, and share it!
