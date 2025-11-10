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
├── path_resolver.rs     # Path resolution logic (relative/absolute conversion)
├── terminal.rs          # Terminal initialization and restoration
└── util.rs              # Utility functions (Entry enum, Mode enum, helpers)
```

## Architecture Patterns

### Module Organization

- Each module has a single, focused responsibility
- Database operations isolated in `database.rs` with lazy_static connection
- UI rendering separated from business logic
- Event handling organized by application mode

### Key Design Patterns

1. **Mode-based State Machine**: Application operates in distinct modes (Browse, Edit, Entry, SeriesSelect, SeriesCreate) with mode-specific event handlers

2. **Path Resolution Strategy**:

   - Database stores relative paths (portable across systems)
   - PathResolver converts between relative and absolute paths
   - Root directory configurable via `config.json`
   - Database always stored in executable directory

3. **Global Database Connection**: Uses `lazy_static` for thread-safe global SQLite connection via `DB_CONN` mutex

4. **Entry Enum Pattern**: Unified `Entry` enum represents different browsable items (Series, Season, Episode) for consistent UI handling

### Configuration

- `config.json` in project root
- Contains: root_dir, path, colors, video_extensions, video_player
- Auto-created with defaults if missing

### Data Flow

1. User input → Event handlers (`handlers.rs`)
2. Handlers update state and call database operations (`database.rs`)
3. Database operations modify SQLite and return updated data
4. Main loop triggers redraw with new state
5. Display module renders UI (`display.rs`)

## Code Style Conventions

- Use `Result` types for error handling
- Prefer `expect()` with descriptive messages over `unwrap()`
- Database operations return `Result<T>` or `Result<T, Box<dyn std::error::Error>>`
- Path operations use `PathBuf` and `Path` types
- Mutable state passed as `&mut` references in handlers
