# Design Document

## Overview

This design converts the movie database application from storing absolute file paths to storing relative paths. The application will use a configurable root directory (specified in config.json) for video file path resolution, while always storing the database file in the same directory as the executable. This allows multiple application instances to have separate databases while potentially pointing to the same video collection.

## Architecture

### Path Resolution Strategy

The application will implement a centralized path resolution system with the following components:

1. **Executable Directory Detection**: Determine the directory containing the application executable for database location
2. **Configurable Root Directory**: Read the root directory path from config.json for video file resolution
3. **Relative Path Storage**: Store all file paths in the database as relative paths from the configured root directory
4. **Runtime Path Resolution**: Convert relative paths to absolute paths using the configured root directory
5. **Dual Directory Management**: Database always in executable directory, video files resolved from configured root

### Key Design Decisions

- **Separation of Concerns**: Database location (executable directory) is separate from video file root (configurable)
- **Configurable Root Directory**: Allow specifying the root directory in config.json for flexible testing and deployment
- **Database Isolation**: Each executable instance maintains its own database for independent operation
- **Backward Compatibility**: No migration of existing data - start fresh with new database schema
- **Minimal Code Changes**: Implement path resolution in a centralized location to minimize changes across the codebase
- **Cross-Platform Support**: Use Rust's standard library path handling for Windows/Linux compatibility

## Components and Interfaces

### 1. Path Resolution Module (`src/path_resolver.rs`)

A new module that handles all path resolution logic:

```rust
pub struct PathResolver {
    executable_dir: PathBuf,
    root_dir: PathBuf,
}

impl PathResolver {
    pub fn new(config_root_dir: Option<&str>) -> Result<Self, PathResolverError>
    pub fn get_database_path(&self) -> PathBuf
    pub fn to_relative(&self, absolute_path: &Path) -> Result<PathBuf, PathResolverError>
    pub fn to_absolute(&self, relative_path: &Path) -> PathBuf
    pub fn resolve_config_path(&self, config_path: &str) -> PathBuf
    pub fn validate_path_under_root(&self, path: &Path) -> Result<(), PathResolverError>
    pub fn get_root_dir(&self) -> &Path
}
```

### 2. Updated Configuration Module (`src/config.rs`)

Enhanced to work with relative paths:

```rust
impl Config {
    pub fn get_resolved_path(&self, resolver: &PathResolver) -> PathBuf
}

pub fn read_config(config_path: &str, resolver: &PathResolver) -> Config
```

### 3. Updated Database Module (`src/database.rs`)

Modified to store and retrieve relative paths:

```rust
pub fn import_episode_relative(
    conn: &Connection,
    relative_location: &str,
    name: &str
) -> Result<()>

pub fn get_episode_absolute_location(
    conn: &Connection,
    episode_id: usize,
    resolver: &PathResolver
) -> Result<PathBuf>
```

### 4. Updated Handlers Module (`src/handlers.rs`)

Modified to use relative path storage:

- `scan_directory_for_videos()`: Convert found absolute paths to relative before storage
- `update_database_with_videos()`: Store relative paths in database
- Video playback: Resolve relative paths to absolute before launching player

## Data Models

### Database Schema Changes

The existing database schema remains unchanged, but the interpretation of the `location` field changes:

- **Before**: `location` contains absolute paths (e.g., `/media/eric/External Storage/Videos/movie.mp4`)
- **After**: `location` contains relative paths (e.g., `Videos/movie.mp4`)

### Configuration File Changes

The config.json structure is enhanced with a new root_dir field:

```json
{
  "root_dir": "/media/eric/External Storage", // New: configurable root directory
  "path": "Videos", // Relative to root_dir
  "current_fg": "Black",
  "current_bg": "Yellow",
  "video_extensions": ["mp4", "mkv", "avi", "mov", "flv", "wmv", "webm"],
  "video_player": "/usr/bin/vlc" // Remains absolute - external tool
}
```

**Behavior**:

- If `root_dir` is specified, use it as the base for all relative path calculations
- If `root_dir` is not specified, default to the executable directory
- Database file is always created in the executable directory regardless of `root_dir`

## Error Handling

### PathResolverError Types

```rust
pub enum PathResolverError {
    ExecutableLocationNotFound,
    RootDirectoryNotFound(PathBuf),
    RootDirectoryNotAccessible(PathBuf),
    PathNotUnderRoot(PathBuf),
    InvalidRelativePath(String),
    IoError(std::io::Error),
}
```

### Error Scenarios

1. **Executable location cannot be determined**: Application exits with clear error message
2. **Configured root directory does not exist**: Application exits with clear error message
3. **Configured root directory is not accessible**: Application exits with clear error message
4. **File not under configured root**: Display error when adding movies outside the root directory - STRICTLY ENFORCE this validation
5. **Relative path contains escape sequences**: Reject any path with `../` or `..\\` components
6. **Relative path resolution fails**: Log error and skip problematic entries
7. **Config path resolution fails**: Fall back to executable directory and warn user

### Error Recovery

- Invalid relative paths in database are logged and skipped during loading
- Missing video files are handled gracefully with appropriate user feedback
- Configuration errors fall back to sensible defaults

## Testing Strategy

### Core Functionality Tests

1. **PathResolver Core Tests**:

   - Test basic relative/absolute path conversion
   - Test strict validation (reject paths outside root)
   - Test executable root detection

2. **Integration Test**:
   - Test end-to-end workflow: scan directory → store relative paths → resolve for playback

### Manual Verification

- Test portability by moving application to different directory
- Verify video playback works with resolved paths

## Implementation Notes

### Directory Detection and Configuration

**Executable Directory Detection**: Use `std::env::current_exe()` to get the executable path, then use `parent()` to get the directory. This approach works reliably across platforms.

**Root Directory Configuration**: Read the `root_dir` field from config.json. If not specified, default to the executable directory. Validate that the configured root directory exists and is accessible.

### Path Conversion Logic

- **To Relative**: Use `Path::strip_prefix()` to remove the configured root directory from absolute paths
- **To Absolute**: Use `Path::join()` to combine configured root directory with relative paths
- **Database Path**: Always use executable directory for database file location
- **Strict Validation**: ALL video files MUST be located under the configured root directory
  - Reject any paths that would result in `../` or `..\\` components
  - Return error for any file not under the configured root directory
  - No relative paths should escape the root directory boundary

### Cross-Platform Considerations

- Use `std::path::Path` and `PathBuf` for all path operations
- Normalize path separators using Rust's path handling
- Test path resolution on both Unix and Windows systems

### Performance Considerations

- Cache both executable directory and configured root directory paths to avoid repeated system calls
- Use efficient path operations to minimize overhead during directory scanning
- Consider lazy loading of path resolution for better startup performance
- Validate root directory configuration once at startup rather than on each path operation
