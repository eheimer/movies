# Contributing to Movies

Thank you for your interest in contributing to this project! This document provides technical information for developers working on the codebase.

## Development Setup

### Prerequisites

- Rust (2021 edition or later)
- Cargo (comes with Rust)
- SQLite (bundled via rusqlite)

### Building the Project

```bash
# Build in debug mode
cargo build

# Build optimized release version
cargo build --release

# Run the application
cargo run

# Run with release optimizations
cargo run --release
```

## Testing

### Running Tests

```bash
# Run all tests (unit and integration)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

### Doctest Workaround

**Important:** If you encounter a shared library error when running doctests, you'll need to set the `LD_LIBRARY_PATH` environment variable.

**Error you might see:**
```
rustdoc: error while loading shared libraries: libLLVM.so.20.1-rust-1.90.0-stable: cannot open shared object file
```

**Solution:**

Set the library path before running doctests:

```bash
export LD_LIBRARY_PATH=/usr/lib/rustlib/x86_64-unknown-linux-gnu/lib:$LD_LIBRARY_PATH
cargo test --doc
```

Or run it as a one-liner:

```bash
LD_LIBRARY_PATH=/usr/lib/rustlib/x86_64-unknown-linux-gnu/lib:$LD_LIBRARY_PATH cargo test --doc
```

**Why this happens:**

The LLVM library used by rustdoc exists at `/usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/` but isn't in the dynamic linker's standard search path. This is a system configuration issue with certain Rust installations, not a problem with the code.

**Note:** As of now, there are no doctests in the codebase, so this workaround is only needed if you add documentation examples with code blocks in the future.

## Code Style

### Formatting

Format your code before committing:

```bash
cargo fmt
```

### Linting

Run clippy to catch common mistakes:

```bash
cargo clippy
```

### Compiler Warnings

The project maintains zero compiler warnings. Before submitting changes:

```bash
cargo build
```

Ensure the build completes with no warnings.

## Project Structure

See `.kiro/steering/structure.md` for detailed information about the codebase organization and architecture patterns.

## Making Changes

1. Create a new branch for your feature or fix
2. Make your changes
3. Run tests: `cargo test`
4. Format code: `cargo fmt`
5. Check for warnings: `cargo build`
6. Run clippy: `cargo clippy`
7. Commit your changes with a clear message
8. Submit a pull request

## Questions?

Feel free to open an issue for questions or discussions about the codebase.

## Technical Architecture

### Dependencies

- **crossterm** (0.23): Terminal manipulation and event handling
- **colored** (2.0): Terminal color output
- **rusqlite** (0.26.0): SQLite database interface with bundled feature
- **serde** (1.0) + **serde_yaml**: Configuration serialization/deserialization
- **walkdir** (2.3): Recursive directory traversal
- **lazy_static** (1.4): Global static database connection

### Data Model

The application uses a SQLite database with three main tables:

- **Episodes**: Individual video files (can be standalone or part of a series)
  - Tracks: name, location, watched status, year, length, episode number
- **Series**: Collections of related episodes (e.g., TV shows)
- **Seasons**: Organizational units within a series

### Path Resolution Strategy

Video file paths are stored relative to the configured `root_dir` for portability:

- Database stores relative paths (e.g., `TV Shows/Series/episode.mp4`)
- PathResolver converts between relative and absolute paths at runtime
- This allows the database to be portable across different systems
- Files outside the `root_dir` are skipped during import with a warning
- The database file itself is always stored in the executable directory

### View Contexts

The application maintains context awareness for operations:

- **Top Level**: All series and standalone episodes
- **Series View**: All seasons within a selected series
- **Season View**: All episodes within a selected season

Context-aware operations (like "Unwatch All") automatically scope to the current view.

### Architecture Patterns

#### Mode-based State Machine

The application operates in distinct modes with mode-specific event handlers:

- **Browse**: Main navigation mode
- **Edit**: Episode detail editing
- **Entry**: Directory path entry for importing videos
- **SeriesSelect**: Selecting a series for episode assignment
- **SeriesCreate**: Creating a new series
- **Menu**: Context menu navigation (F1)

#### Global Database Connection

Uses `lazy_static` for a thread-safe global SQLite connection via `DB_CONN` mutex. This provides:

- Single connection shared across the application
- Thread-safe access through mutex locking
- Lazy initialization on first use

#### Entry Enum Pattern

A unified `Entry` enum represents different browsable items (Series, Season, Episode) for consistent UI handling. This allows the same navigation code to work with different entity types.

### Module Organization

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

Each module has a single, focused responsibility:

- Database operations isolated in `database.rs`
- UI rendering separated from business logic
- Event handling organized by application mode

### Configuration

The `config.yaml` file in the project root contains:

- `root_dir`: Base directory for video files
- `db_location`: Path to SQLite database (null uses default)
- `current_fg/current_bg`: Colors for selected items
- `video_extensions`: Supported video file formats
- `video_player`: Path to video player executable
- `log_level`: Logging verbosity

The file is auto-created with defaults if missing and includes inline documentation.

### Data Flow

1. User input → Event handlers (`handlers.rs`)
2. Handlers update state and call database operations (`database.rs`)
3. Database operations modify SQLite and return updated data
4. Main loop triggers redraw with new state
5. Display module renders UI (`display.rs`)

### Error Handling Conventions

- Use `Result` types for operations that can fail
- Prefer `.expect()` with descriptive messages over `.unwrap()`
- Database operations return `Result<T>` or `Result<T, Box<dyn std::error::Error>>`
- Path operations use `PathBuf` and `Path` types
- Mutable state passed as `&mut` references in handlers

### Menu System Architecture

The centralized menu system provides:

- **First-line items**: Always visible in the header (e.g., rescan with CTRL+L)
- **Context menu items**: Accessible via F1 menu, with function key hotkeys (F2-F7, etc.)
- **Context-aware availability**: Menu items show/hide based on selected entry type and state

Key components:

- `MenuItem` struct: Defines label, hotkey, action, and location
- `MenuAction` enum: Identifies the action to perform
- `MenuLocation` enum: Determines where the item appears (FirstLine or ContextMenu)
- `MenuContext` struct: Provides context for availability checks

For detailed information on adding new menu features, see `.kiro/steering/menu-features.md`.
