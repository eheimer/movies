---
inclusion: always
---

# Technology Stack

## Language & Edition

- Rust 2021 edition

## Core Dependencies

- **crossterm** (0.23): Terminal manipulation and event handling
- **colored** (2.0): Terminal color output
- **rusqlite** (0.26.0): SQLite database interface with bundled feature
- **serde** (1.0) + **serde_yaml**: Configuration serialization/deserialization
- **walkdir** (2.3): Recursive directory traversal
- **lazy_static** (1.4): Global static database connection

## Dev Dependencies

- **tempfile** (3.0): Temporary directories for testing

## Build System

Standard Cargo-based Rust project.

### Common Commands

```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run the application
cargo run

# Run with release optimizations
cargo run --release

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Database

- SQLite database stored as `videos.db` in the executable directory
- Schema includes tables: `series`, `season`, `episode`
- Relative path storage for video file locations (relative to configured root_dir)
