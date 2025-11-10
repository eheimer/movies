---
inclusion: always
---

# Product Overview

A terminal-based video file browser and library manager written in Rust. The application provides a TUI (Terminal User Interface) for browsing, organizing, and playing video files with support for series/season/episode organization.

## Core Features

- Browse video files from a configured directory
- Organize videos into series, seasons, and episodes
- Track watched status and metadata (year, length, episode numbers)
- Real-time search/filtering by typing
- Play videos using a configurable external player (e.g., VLC)
- SQLite database for persistent metadata storage
- Keyboard-driven navigation with multiple modes (Browse, Edit, Entry, SeriesSelect, SeriesCreate)

## Key Concepts

- **Episodes**: Individual video files that can be standalone or part of a series
- **Series**: Collections of related episodes (e.g., TV shows)
- **Seasons**: Organizational units within a series
- **Path Resolution**: Relative path storage in database with configurable root directory for portability
