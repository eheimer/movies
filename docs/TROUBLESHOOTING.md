# Troubleshooting

## Installation Issues

### The program won't start

**Symptoms:** Error when running `cargo run` or the executable doesn't launch

**Solutions:**
- Verify Rust is installed: `rustc --version`
- Rebuild the project: `cargo clean && cargo build --release`
- Check for build errors in the output
- Ensure you're in the project directory

### Build fails with dependency errors

**Solutions:**
- Update Rust: `rustup update`
- Clear cargo cache: `cargo clean`
- Delete `Cargo.lock` and rebuild: `rm Cargo.lock && cargo build`

## Video Playback Issues

### Videos won't play

**Symptoms:** Nothing happens when you press Enter on a video, or error message appears

**Solutions:**
1. Verify video player path in `config.yaml`:
   ```yaml
   video_player: /usr/bin/vlc
   ```
2. Test the player manually:
   ```bash
   /usr/bin/vlc /path/to/video.mp4
   ```
3. Check that the video player is installed:
   - Linux: `which vlc` or `which mpv`
   - macOS: Check `/Applications/VLC.app` exists
   - Windows: Check `C:\Program Files\VideoLAN\VLC\vlc.exe` exists

### Video player opens but video doesn't load

**Solutions:**
- Verify the video file still exists at its original location
- Check file permissions (ensure you can read the file)
- Try playing the file directly with your video player
- Rescan the library (F1 → S) to update file locations

## Library Issues

### The program can't find my videos

**Symptoms:** Empty library after scanning, or "No videos found" message

**Solutions:**
1. Verify `db_location` in `config.yaml` points to the correct folder
2. Check that your video files have supported extensions:
   - Default: mp4, mkv, avi, mov, flv, wmv, webm
   - Add more in `config.yaml` under `video_extensions`
3. Ensure the directory path is absolute (starts with `/` on Linux/macOS or `C:\` on Windows)
4. Check file permissions on the video directory
5. Press F1 → S to rescan

### Videos disappeared after moving files

**Solutions:**
- Update `db_location` in `config.yaml` to the new location
- Make sure you moved the `videos.sqlite` database file with your videos
- Press F1 → S to rescan the library
- If you moved individual files (not the whole directory), you may need to reorganize them

### Duplicate entries in library

**Solutions:**
- This can happen if files were moved within the `db_location`
- Press F1 → S to rescan - the program will update paths
- If duplicates persist, check for symbolic links or mounted drives

## Display Issues

### The interface looks weird or garbled

**Solutions:**
- Ensure your terminal window is at least 80 characters wide
- Try a different terminal emulator (e.g., GNOME Terminal, iTerm2, Windows Terminal)
- Check that your terminal supports UTF-8 encoding
- Verify your terminal supports colors: `echo $TERM` should show something like `xterm-256color`

### Colors don't display correctly

**Solutions:**
- Try a terminal with better color support
- Adjust colors in `config.yaml`:
  ```yaml
  current_fg: "White"
  current_bg: "Blue"
  ```
- Use simpler colors (Black, White, Red, Green, Blue)

### Scrollbar characters look wrong

**Solutions:**
- Your terminal may not support Unicode characters
- Change to ASCII characters in `config.yaml`:
  ```yaml
  scrollbar_track_char: "|"
  scrollbar_indicator_char: "#"
  ```

## Database Issues

### "Database is locked" error

**Solutions:**
- Close any other instances of the program
- Check for stale lock files
- Restart the program

### Lost all my organization/watched status

**Solutions:**
- Check if `videos.sqlite` exists in your video folder (the `db_location` directory)
- If you moved your videos, make sure you moved `videos.sqlite` with them
- Check for `videos.sqlite.backup` files
- Unfortunately, if the database is deleted, organization is lost (but video files are safe)

### Database corruption

**Symptoms:** Crashes, errors about malformed database

**Solutions:**
1. Backup the current database: `cp videos.sqlite videos.sqlite.backup`
2. Try SQLite recovery:
   ```bash
   sqlite3 videos.sqlite ".recover" | sqlite3 videos_recovered.sqlite
   ```
3. If recovery fails, delete `videos.sqlite` and rescan (you'll lose organization)

## Performance Issues

### Slow scanning with large libraries

**Expected behavior:** Scanning thousands of videos takes time

**Tips:**
- First scan is always slower (building the database)
- Subsequent scans are faster (only checking for changes)
- Consider organizing videos into subdirectories
- SSD storage is much faster than HDD

### Sluggish navigation

**Solutions:**
- Check if your terminal is running slowly (try a different one)
- Ensure you're running the release build: `cargo run --release`
- Check system resources (CPU, memory)

## Configuration Issues

### Config file not found

**Solutions:**
- The program creates `config.yaml` automatically on first run
- If deleted, just run the program again
- Check you're running from the correct directory

### Changes to config.yaml not taking effect

**Solutions:**
- Restart the program after editing config
- Check YAML syntax (indentation matters!)
- Verify file was saved after editing

## Platform-Specific Issues

### Linux: Permission denied errors

**Solutions:**
- Check file permissions: `ls -la`
- Ensure video player is executable: `chmod +x /usr/bin/vlc`
- Run with appropriate permissions (avoid sudo unless necessary)

### macOS: "App is from an unidentified developer"

**Solutions:**
- Right-click the executable and select "Open"
- Or: System Preferences → Security & Privacy → Allow

### Windows: Antivirus blocking execution

**Solutions:**
- Add the executable to antivirus exceptions
- Build from source to avoid false positives

## Still Having Issues?

If none of these solutions work:

1. Check the log file (`movies.log`) for detailed error messages
2. Open an issue on GitHub with:
   - Your operating system and version
   - Rust version (`rustc --version`)
   - Error messages or unexpected behavior
   - Steps to reproduce the problem
3. Include relevant parts of your `config.yaml` (remove sensitive paths)
