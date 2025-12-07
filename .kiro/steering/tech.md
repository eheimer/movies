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

Standard Cargo: `cargo build`, `cargo run`, `cargo test`, `cargo clippy`

## Database

SQLite `videos.db` in executable directory with tables: `series`, `season`, `episode`. Stores relative paths.
