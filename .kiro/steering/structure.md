---
inclusion: always
---

# Project Structure

## Source Organization

```
src/
├── main.rs              # Application entry point, main event loop
├── handlers.rs          # Keyboard event handlers for each mode
├── database.rs          # SQLite operations and queries
├── display.rs           # Terminal UI rendering
├── config.rs            # Configuration file management
├── dto.rs               # Data transfer objects (EpisodeDetail, Series, Season)
├── episode_field.rs     # Episode field enumeration for editing
├── lib.rs               # Library crate root
├── logger.rs            # Logging functionality
├── menu.rs              # Menu system (MenuAction, MenuItem, context menu)
├── path_resolver.rs     # Path resolution logic (relative/absolute conversion)
├── paths.rs             # Application path management
├── terminal.rs          # Terminal initialization and restoration
├── util.rs              # Utility functions (Entry enum, Mode enum, helpers)
└── video_metadata.rs    # Video metadata extraction
```

## Key Patterns

1. **Mode-based State Machine**: Browse, Edit, Entry, SeriesSelect, SeriesCreate modes with mode-specific handlers
2. **Path Resolution**: Database stores relative paths; PathResolver converts to/from absolute; root_dir in `config.yaml`
3. **Global DB Connection**: `lazy_static` `DB_CONN` mutex for thread-safe SQLite access
4. **Entry Enum**: Unified representation of Series/Season/Episode for consistent UI handling

## Configuration

`config.yaml` in project root with: root_dir, path, colors, video_extensions, video_player, log_file, log_level

## Code Conventions

- Use `Result` types; prefer `expect()` over `unwrap()`
- Database operations return `Result<T, Box<dyn std::error::Error>>`
- Mutable state passed as `&mut` references
