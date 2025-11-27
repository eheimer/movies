# Platform-Standard Directories

## Overview

Move configuration and database files from the executable directory to platform-standard locations using the `directories` crate.

## Current Behavior

- `config.json` created in executable directory
- `videos.db` created in executable directory
- Not following platform conventions

## Desired Behavior

Use platform-appropriate directories:

**Linux (XDG):**
- Config: `~/.config/movies/config.json`
- Database: `~/.local/share/movies/videos.sqlite`

**macOS:**
- Config: `~/Library/Application Support/movies/config.json`
- Database: `~/Library/Application Support/movies/videos.sqlite`

**Windows:**
- Config: `%APPDATA%\movies\config.json`
- Database: `%APPDATA%\movies\videos.sqlite`

## Requirements

### Functional Requirements

1. **Use `directories` crate** to get platform-appropriate paths
2. **Create directories automatically** on first run if they don't exist
3. **Initialize default config** if config file doesn't exist
4. **Initialize database schema** if database doesn't exist
5. **Handle permission errors gracefully** with clear error messages
6. **Rename database file** from `videos.db` to `videos.sqlite`

### Non-Functional Requirements

1. **No backward compatibility needed** - no migration from old locations
2. **Fail fast** - if directories can't be created, show error and exit
3. **Clear error messages** - tell users exactly what went wrong and where

## Success Criteria

- Application creates `~/.config/movies/` and `~/.local/share/movies/` on Linux
- Config and database files are created in correct locations on first run
- Application works identically to before, just with different file locations
- Clear error messages if directory creation fails due to permissions

## Out of Scope

- Migration from old file locations (no existing users)
- Configuration UI for changing locations
- Support for custom directory locations
