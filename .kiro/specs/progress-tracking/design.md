# Design Document: Progress Tracking

## Overview

This design extends the video library manager with comprehensive progress tracking functionality. The system will automatically track viewing progress during playback, mark episodes as watched when a configurable threshold is reached, and provide resume functionality for partially watched content. The implementation integrates seamlessly with the existing SQLite database, configuration system, and video player integration.

## Architecture

The progress tracking system consists of four main components:

1. **Database Layer**: Extended schema with new progress fields and operations
2. **Configuration Layer**: New watched threshold setting with validation
3. **Player Plugin System**: Trait-based architecture for player-specific progress tracking
4. **UI Integration**: Display of progress information in episode details

The system uses a plugin architecture where each video player has its own progress tracking implementation. The plugin trait defines two key operations:
- **Launch with resume**: Start playback at a specific position
- **Retrieve progress**: Get the final playback position after playback ends

**Initial Implementation: Celluloid/mpv Plugin**
- Uses mpv's `--save-position-on-quit` feature
- Reads watch-later files from `~/.local/share/celluloid/watch_later/` after playback
- Parses the watch-later file to extract the final playback position
- No active monitoring during playback required

The system maintains backward compatibility with existing data while adding new functionality through database schema migration and configuration defaults.

## Components and Interfaces

### Database Schema Extensions

**New Episode Table Fields:**
- `last_watched_time`: TEXT field storing ISO 8601 datetime when episode was last marked as watched
- `last_progress_time`: INTEGER field storing progress in seconds from start of episode

**Migration Strategy:**
- Add new columns with NULL defaults to preserve existing data
- No data loss during schema updates
- Graceful handling of NULL values in all operations

### Configuration Enhancement

**New Config Field:**
```rust
pub struct Config {
    // ... existing fields ...
    #[serde(default = "default_watched_threshold")]
    pub watched_threshold: u8,
}

fn default_watched_threshold() -> u8 {
    95
}
```

**Validation Rules:**
- Threshold must be between 1 and 100 percent
- Invalid values default to 95% with warning log
- Configuration reloading supported for runtime updates

### Progress Tracking Service

**Player Plugin Trait:**
```rust
pub trait PlayerPlugin {
    // Returns the command and arguments to launch the player with optional resume position
    fn launch_command(&self, file_path: &Path, start_time: Option<u64>) -> (String, Vec<String>);
    
    // Retrieves the final playback position after the player exits
    // Returns None if position couldn't be determined
    fn get_final_position(&self, file_path: &Path) -> Result<Option<u64>, Box<dyn std::error::Error>>;
}
```

**Celluloid/mpv Plugin Implementation:**
```rust
pub struct CelluloidPlugin {
    watch_later_dir: PathBuf,
}

impl PlayerPlugin for CelluloidPlugin {
    fn launch_command(&self, file_path: &Path, start_time: Option<u64>) -> (String, Vec<String>) {
        let mut args = vec!["--save-position-on-quit".to_string()];
        if let Some(time) = start_time {
            args.push(format!("--start={}", time));
        }
        args.push(file_path.to_string_lossy().to_string());
        ("celluloid".to_string(), args)
    }
    
    fn get_final_position(&self, file_path: &Path) -> Result<Option<u64>, Box<dyn std::error::Error>> {
        // Calculate MD5 hash of file path to find watch-later file
        // Read watch-later file and parse "start=" line
        // Return position in seconds
    }
}
```

**Progress Tracking Workflow:**
1. Before launch: Query database for existing progress
2. Launch player: Use plugin's `launch_command()` with resume position
3. Wait for player: Block until player process exits
4. After exit: Use plugin's `get_final_position()` to retrieve progress
5. Update database: Store progress and check watched threshold
6. Auto-mark watched: If progress exceeds threshold, mark as watched

### Video Player Integration

**Enhanced Launch Function:**
```rust
pub fn run_video_player_with_plugin(
    plugin: &dyn PlayerPlugin,
    file_path: &Path,
    start_time: Option<u64>
) -> io::Result<ExitStatus>
```

**Playback Workflow:**
1. Get player plugin based on configuration
2. Query database for existing progress
3. Launch player using plugin's command with resume position
4. Wait for player process to complete (blocking)
5. Retrieve final position using plugin's method
6. Update database with new progress
7. Check if watched threshold exceeded and auto-mark if needed

**Watch-Later File Handling (Celluloid/mpv):**
- Watch-later directory: `~/.local/share/celluloid/watch_later/`
- File naming: MD5 hash of absolute file path
- File format: Plain text with key=value pairs
- Relevant key: `start=<seconds>` (playback position)
- Parse logic: Read file, find "start=" line, extract numeric value

### Database Operations

**New Functions:**
```rust
// Progress management
pub fn update_episode_progress(episode_id: usize, progress_seconds: u64) -> Result<(), Box<dyn std::error::Error>>;
pub fn get_episode_progress(episode_id: usize) -> Result<Option<u64>, Box<dyn std::error::Error>>;
pub fn mark_episode_watched_with_timestamp(episode_id: usize) -> Result<(), Box<dyn std::error::Error>>;
pub fn reset_episode_progress(episode_id: usize) -> Result<(), Box<dyn std::error::Error>>;

// Progress-based watched status
pub fn check_and_mark_watched_if_threshold_met(
    episode_id: usize, 
    progress_seconds: u64, 
    total_duration: u64, 
    threshold: u8
) -> Result<bool, Box<dyn std::error::Error>>;

// Enhanced episode details
pub fn get_episode_detail_with_progress(id: usize) -> Result<EpisodeDetailWithProgress, Box<dyn std::error::Error>>;
```

**Extended Data Transfer Object:**
```rust
#[derive(Debug, Clone)]
pub struct EpisodeDetailWithProgress {
    // ... existing EpisodeDetail fields ...
    pub last_watched_time: Option<String>,
    pub last_progress_time: Option<u64>,
}
```

## Data Models

### Database Schema Changes

**Episode Table Migration:**
```sql
ALTER TABLE episode ADD COLUMN last_watched_time TEXT;
ALTER TABLE episode ADD COLUMN last_progress_time INTEGER;
```

**Data Types:**
- `last_watched_time`: ISO 8601 formatted datetime string (e.g., "2024-01-15T14:30:00Z")
- `last_progress_time`: Integer seconds from episode start (e.g., 3600 for 1 hour)

### Progress State Management

**State Transitions:**
1. **Unwatched, No Progress**: `watched=false, last_progress_time=NULL`
2. **Partially Watched**: `watched=false, last_progress_time>0`
3. **Auto-Watched**: `watched=true, last_watched_time=NOW(), last_progress_time=0`
4. **Manually Watched**: `watched=true, last_watched_time=NOW(), last_progress_time=preserved`

**Validation Rules:**
- Progress time cannot exceed episode duration
- Invalid progress data resets to zero with error logging
- Watched episodes with zero progress start from beginning

## Error Handling

### Database Error Recovery

**Connection Failures:**
- Graceful degradation when database unavailable
- Progress tracking continues in memory with periodic retry
- Batch updates when connection restored

**Schema Migration Errors:**
- Rollback capability for failed migrations
- Detailed error logging with recovery suggestions
- Fallback to read-only mode if migration fails

### Video Player Integration Errors

**Player Launch Failures:**
- Fallback to standard launch without resume position
- User notification of resume failure with option to retry
- Logging of player compatibility issues

**Watch-Later File Access Errors:**
- Handle missing watch-later directory gracefully
- Log warning if watch-later file cannot be read
- Continue operation without progress update if file unavailable
- Validate watch-later file format and handle parse errors

**Progress Retrieval Failures:**
- Return None if position cannot be determined
- Log detailed error information for debugging
- Continue normal operation without progress update

### Configuration Validation Errors

**Invalid Threshold Values:**
- Automatic correction to valid range (1-100)
- Warning notification to user
- Detailed logging of configuration issues

## Testing Strategy

### Unit Testing

Focus on simple, easily testable functionality:

**Configuration Management:**
- Threshold validation (valid range 1-100%)
- Default value handling

**Progress Calculations:**
- Time format conversions (seconds to HH:MM:SS)
- Threshold percentage calculations

**Database Operations:**
- Basic progress update and retrieval functions
- Simple validation of data types and ranges

### Manual Testing

The majority of testing will be done through manual user testing to verify:

**Video Player Integration:**
- Resume functionality works with different video players
- Progress tracking during actual video playback
- Automatic watched status detection

**UI Integration:**
- Progress information displays correctly
- User interface updates appropriately
- Error handling in real-world scenarios

**End-to-End Workflows:**
- Complete video watching experience
- Configuration changes take effect
- Database persistence across application restarts

### Testing Philosophy

- Prefer manual testing over complex integration tests
- Write unit tests only for easily testable, isolated functionality
- Focus on user-facing functionality validation through actual usage
- Avoid testing complex interactions between multiple system components