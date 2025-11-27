# Design: Platform-Standard Directories

## Architecture Changes

### New Dependency

Add to `Cargo.toml`:
```toml
directories = "5.0"
```

### Module Changes

#### 1. New Module: `src/paths.rs`

Create a centralized path management module:

```rust
use directories::ProjectDirs;
use std::path::PathBuf;

pub struct AppPaths {
    pub config_file: PathBuf,
    pub database_file: PathBuf,
}

impl AppPaths {
    pub fn new() -> Result<Self, String> {
        let proj_dirs = ProjectDirs::from("", "", "movies")
            .ok_or("Failed to determine application directories")?;
        
        let config_dir = proj_dirs.config_dir();
        let data_dir = proj_dirs.data_dir();
        
        // Create directories if they don't exist
        std::fs::create_dir_all(config_dir)
            .map_err(|e| format!("Failed to create config directory {}: {}", 
                config_dir.display(), e))?;
        
        std::fs::create_dir_all(data_dir)
            .map_err(|e| format!("Failed to create data directory {}: {}", 
                data_dir.display(), e))?;
        
        Ok(AppPaths {
            config_file: config_dir.join("config.json"),
            database_file: data_dir.join("videos.sqlite"),
        })
    }
}
```

#### 2. Update `src/config.rs`

**Current approach:**
- Uses `std::env::current_exe()` to get executable directory
- Creates `config.json` in that directory

**New approach:**
- Accept `PathBuf` parameter for config file location
- Remove executable directory logic

Changes:
```rust
pub fn load_or_create_config(config_path: &PathBuf) -> Config {
    if config_path.exists() {
        // Load existing config
        let config_str = std::fs::read_to_string(config_path)
            .expect("Failed to read config file");
        serde_json::from_str(&config_str)
            .expect("Failed to parse config file")
    } else {
        // Create default config
        let default_config = Config::default();
        let config_json = serde_json::to_string_pretty(&default_config)
            .expect("Failed to serialize default config");
        std::fs::write(config_path, config_json)
            .expect("Failed to write config file");
        default_config
    }
}
```

#### 3. Update `src/database.rs`

**Current approach:**
- Uses `lazy_static` with hardcoded `videos.db` path in executable directory

**New approach:**
- Initialize database connection with provided path
- Keep `lazy_static` pattern but initialize with correct path

Changes:
```rust
use std::sync::Mutex;
use lazy_static::lazy_static;
use rusqlite::Connection;
use std::path::PathBuf;

lazy_static! {
    pub static ref DB_CONN: Mutex<Connection> = {
        // This will be initialized from main
        panic!("Database not initialized. Call init_database first.");
    };
}

static mut DB_PATH: Option<PathBuf> = None;

pub fn init_database(db_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        DB_PATH = Some(db_path.clone());
    }
    
    let conn = Connection::open(&db_path)?;
    
    // Create tables if they don't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS series (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS season (
            id INTEGER PRIMARY KEY,
            series_id INTEGER NOT NULL,
            season_number INTEGER NOT NULL,
            FOREIGN KEY (series_id) REFERENCES series(id),
            UNIQUE(series_id, season_number)
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS episode (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            watched INTEGER NOT NULL DEFAULT 0,
            year INTEGER,
            length INTEGER,
            season_id INTEGER,
            episode_number INTEGER,
            FOREIGN KEY (season_id) REFERENCES season(id)
        )",
        [],
    )?;
    
    Ok(())
}

pub fn get_connection() -> Result<Connection, Box<dyn std::error::Error>> {
    let db_path = unsafe {
        DB_PATH.as_ref()
            .ok_or("Database not initialized")?
    };
    Ok(Connection::open(db_path)?)
}
```

**Alternative approach (simpler):**
Keep the `lazy_static` pattern but use `std::sync::Once` to initialize:

```rust
use std::sync::{Mutex, Once};
use lazy_static::lazy_static;
use rusqlite::Connection;
use std::path::PathBuf;

static INIT: Once = Once::new();
static mut DB_PATH: Option<PathBuf> = None;

lazy_static! {
    pub static ref DB_CONN: Mutex<Connection> = {
        let db_path = unsafe {
            DB_PATH.as_ref()
                .expect("Database path not set. Call set_database_path first.")
        };
        
        let conn = Connection::open(db_path)
            .expect("Failed to open database");
        
        // Initialize schema
        conn.execute(
            "CREATE TABLE IF NOT EXISTS series (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            )",
            [],
        ).expect("Failed to create series table");
        
        // ... other tables ...
        
        Mutex::new(conn)
    };
}

pub fn set_database_path(path: PathBuf) {
    INIT.call_once(|| {
        unsafe {
            DB_PATH = Some(path);
        }
    });
}
```

#### 4. Update `src/main.rs`

**Current approach:**
- Calls `load_or_create_config()` with no parameters
- Database initializes automatically via `lazy_static`

**New approach:**
- Initialize `AppPaths` first
- Pass paths to config and database initialization
- Handle errors gracefully

Changes:
```rust
mod paths;

fn main() {
    // Initialize application paths
    let app_paths = match paths::AppPaths::new() {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Please ensure you have write permissions to your home directory.");
            std::process::exit(1);
        }
    };
    
    // Load or create config
    let config = config::load_or_create_config(&app_paths.config_file);
    
    // Initialize database
    database::set_database_path(app_paths.database_file);
    
    // Rest of main() continues as before...
}
```

## Error Handling Strategy

### Directory Creation Failures

If `create_dir_all` fails:
1. Show clear error message with the path that failed
2. Show the underlying OS error
3. Suggest checking permissions
4. Exit with non-zero status code

Example error message:
```
Error: Failed to create config directory /home/user/.config/movies: Permission denied
Please ensure you have write permissions to your home directory.
```

### Database Initialization Failures

If database creation fails:
1. Show clear error message with database path
2. Show the underlying error
3. Exit with non-zero status code

## Testing Strategy

### Manual Testing

Test on each platform:
1. Delete any existing config/database directories
2. Run application
3. Verify directories are created in correct locations
4. Verify config file is created with defaults
5. Verify database file is created with schema
6. Verify application functions normally

### Permission Testing

Test permission failures:
1. Create config directory with no write permissions
2. Run application
3. Verify clear error message is shown

## Implementation Order

1. Add `directories` dependency to `Cargo.toml`
2. Create `src/paths.rs` module
3. Update `src/database.rs` to accept path parameter
4. Update `src/config.rs` to accept path parameter
5. Update `src/main.rs` to use new path initialization
6. Test on Linux
7. Test on other platforms if available

## Rollback Plan

If issues arise:
1. Revert changes to `main.rs`, `config.rs`, `database.rs`
2. Remove `paths.rs` module
3. Remove `directories` dependency
4. Application returns to previous behavior
