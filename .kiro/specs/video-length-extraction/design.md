# Design Document

## Overview

This feature adds automatic video duration extraction to the application using pure Rust libraries. The system will extract duration metadata from video files (mkv, avi, mp4) at three key points: during rescan operations, when entering edit mode, and before playing a video. The implementation uses an extensible architecture that allows adding support for additional video formats without modifying core extraction logic.

## Architecture

### High-Level Design

The video length extraction system consists of three main components:

1. **Video Metadata Extractor**: Core module that coordinates format detection and duration extraction
2. **Format Handlers**: Format-specific parsers for mkv, avi, and mp4 containers
3. **Integration Points**: Hooks into existing rescan, edit mode, and playback workflows

### Component Interaction

```
┌─────────────────┐
│  User Actions   │
│ (Play/Edit/     │
│  Rescan)        │
└────────┬────────┘
         │
         v
┌─────────────────────────┐
│ Video Metadata          │
│ Extractor               │
│ - Detect format         │
│ - Route to handler      │
│ - Convert to minutes    │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Format Handlers         │
│ - MKV Handler           │
│ - AVI Handler           │
│ - MP4 Handler           │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Database Update         │
│ - Update episode.length │
└─────────────────────────┘
```

## Components and Interfaces

### 1. Video Metadata Extractor Module

**Location**: `src/video_metadata.rs`

**Core Function**:
```rust
pub fn extract_duration_seconds(file_path: &Path) -> Result<u64, Box<dyn std::error::Error>>
```

This function:
- Takes a file path as input
- Detects the video format based on file extension
- Routes to the appropriate format handler
- Returns duration in seconds
- Returns an error if extraction fails

**Helper Function**:
```rust
pub fn extract_and_update_episode_length(episode_id: usize, file_path: &Path) -> Result<(), Box<dyn std::error::Error>>
```

This function:
- Extracts duration using `extract_duration_seconds`
- Updates the database episode.length field with seconds value
- Used by all integration points

**Display Formatting Function**:
```rust
pub fn format_duration_hms(seconds: u64) -> String
```

This function:
- Takes duration in seconds
- Returns formatted string as "hh:mm:ss"
- Handles durations less than one hour (e.g., "00:45:30")

### 2. Format Handlers

Each format handler implements duration extraction for a specific container format:

**MKV Handler** (using `matroska` crate):
```rust
fn extract_mkv_duration(file_path: &Path) -> Result<u64, Box<dyn std::error::Error>>
```

**MP4 Handler** (using `mp4parse` crate):
```rust
fn extract_mp4_duration(file_path: &Path) -> Result<u64, Box<dyn std::error::Error>>
```

**AVI Handler** (using `avi` or manual parsing):
```rust
fn extract_avi_duration(file_path: &Path) -> Result<u64, Box<dyn std::error::Error>>
```

### 3. Integration Points

**Rescan Operation** (`src/database.rs`):
- Modify `rescan_directory` function
- After scanning for new files, query episodes with length = 0 or NULL
- For each episode, call `extract_and_update_episode_length`
- Continue on errors (don't fail entire rescan if one file fails)

**Edit Mode Entry** (`src/handlers.rs`):
- In `handle_browse_mode` when transitioning to Edit mode
- Check if current episode has length = 0 or NULL
- If so, call `extract_and_update_episode_length`
- Reload episode detail to show updated length

**Video Playback** (`src/handlers.rs`):
- In `handle_browse_mode` when Enter key is pressed
- Check if current episode has length = 0 or NULL
- If so, call `extract_and_update_episode_length`
- Proceed with playback regardless of extraction result

## Data Models

### Database Schema

The `episode` table's `length` field will store duration in seconds:

```sql
CREATE TABLE episode (
    id INTEGER PRIMARY KEY,
    ...
    length INTEGER,  -- Duration in seconds
    ...
)
```

### Internal Data Structures

**Duration Representation**:
- Extracted as `u64` seconds from video files
- Stored as `i32` seconds in database
- Displayed as `String` in "hh:mm:ss" format

**Conversion Logic**:
```rust
// Seconds to hh:mm:ss
let hours = seconds / 3600;
let minutes = (seconds % 3600) / 60;
let secs = seconds % 60;
format!("{:02}:{:02}:{:02}", hours, minutes, secs)
```

## Test Cases

### Test Case 1: MKV duration extraction

When a valid MKV file is processed, the system should successfully extract the duration in seconds.
**Validates: Requirements 1.1**

### Test Case 2: AVI duration extraction

When a valid AVI file is processed, the system should successfully extract the duration in seconds.
**Validates: Requirements 1.2**

### Test Case 3: MP4 duration extraction

When a valid MP4 file is processed, the system should successfully extract the duration in seconds.
**Validates: Requirements 1.3**

### Test Case 4: Duration storage in seconds

When duration is extracted in seconds, the system should store it directly in the database as seconds without conversion.
**Validates: Requirements 1.4**

### Test Case 5: Extraction failure handling

When duration extraction fails for a video file, the system should return an error without crashing and leave the episode length unchanged.
**Validates: Requirements 1.5**

### Test Case 6: Rescan identifies null lengths

When rescan operation runs, the system should query and identify all episodes where length IS NULL.
**Validates: Requirements 2.1**

### Test Case 7: Rescan identifies zero lengths

When rescan operation runs, the system should query and identify all episodes where length = 0.
**Validates: Requirements 2.2**

### Test Case 8: Rescan updates missing lengths

When rescan identifies an episode with missing length, the system should attempt extraction and update the database on success.
**Validates: Requirements 2.3, 2.4**

### Test Case 9: Rescan continues on extraction failure

When rescan fails to extract duration for one episode, the system should continue processing remaining episodes.
**Validates: Requirements 2.5**

### Test Case 10: Format handler selection

When processing a video file, the system should select the correct format handler based on file extension (.mkv → MKV handler, .mp4 → MP4 handler, .avi → AVI handler).
**Validates: Requirements 3.1, 3.3**

### Test Case 11: Adding new format support

When a new video format needs support, a developer should be able to add a new format handler function without modifying the core extraction logic.
**Validates: Requirements 3.2**

### Test Case 12: No external dependencies

When the application is built, it should only use Rust crates and not require external utilities like ffmpeg or mediainfo.
**Validates: Requirements 5.1, 5.2, 5.3**

### Test Case 13: Play triggers extraction for null length

When a user plays an episode with NULL length, the system should extract duration before launching the player.
**Validates: Requirements 4.1**

### Test Case 14: Play triggers extraction for zero length

When a user plays an episode with length = 0, the system should extract duration before launching the player.
**Validates: Requirements 4.2**

### Test Case 15: Play updates database on successful extraction

When extraction succeeds before playback, the system should update the episode.length field in the database.
**Validates: Requirements 4.3**

### Test Case 16: Play proceeds on extraction failure

When extraction fails before playback, the system should still launch the video player.
**Validates: Requirements 4.4**

### Test Case 17: Play processes only selected episode

When play triggers extraction, the system should only process the currently selected episode, not all episodes.
**Validates: Requirements 4.5**

### Test Case 18: Edit mode triggers extraction for null length

When a user enters edit mode for an episode with NULL length, the system should extract duration from the video file.
**Validates: Requirements 6.1**

### Test Case 19: Edit mode triggers extraction for zero length

When a user enters edit mode for an episode with length = 0, the system should extract duration from the video file.
**Validates: Requirements 6.2**

### Test Case 20: Edit mode updates database on successful extraction

When extraction succeeds in edit mode, the system should update the episode.length field in the database.
**Validates: Requirements 6.3**

### Test Case 21: Edit mode displays extracted length

When extraction succeeds in edit mode, the system should display the extracted length in the edit interface.
**Validates: Requirements 6.4**

### Test Case 22: Edit mode processes only selected episode

When edit mode triggers extraction, the system should only process the currently selected episode.
**Validates: Requirements 6.5**

### Test Case 23: Duration display format

When displaying episode duration, the system should format it as "hh:mm:ss" (e.g., "01:45:30" for 1 hour 45 minutes 30 seconds).
**Validates: Requirements 7.1, 7.3, 7.4**

### Test Case 24: Duration display for short videos

When displaying duration less than one hour, the system should format it as "00:mm:ss" (e.g., "00:45:30" for 45 minutes 30 seconds).
**Validates: Requirements 7.2**

## Error Handling

### Error Categories

1. **File Access Errors**: File not found, permission denied
2. **Parse Errors**: Corrupted file, unsupported codec, malformed container
3. **Unsupported Format**: File extension not recognized

### Error Handling Strategy

- All extraction functions return `Result<T, Box<dyn std::error::Error>>`
- Integration points catch errors and log them (using eprintln! or similar)
- Errors during rescan don't stop the entire operation
- Errors during play/edit don't prevent the user action (play still launches, edit still opens)
- Database updates are only performed on successful extraction

### Error Messages

- Clear, actionable error messages for debugging
- Include file path in error context
- Distinguish between "file not found" vs "parse failed" vs "unsupported format"

## Testing Strategy

### Unit Testing

1. **Format Handler Tests**:
   - Test each format handler with sample video files
   - Test with corrupted/invalid files
   - Test with files missing duration metadata

2. **Duration Conversion Tests**:
   - Test hh:mm:ss formatting with various durations
   - Test formatting for durations < 1 hour
   - Test formatting for durations > 24 hours

3. **Format Detection Tests**:
   - Test extension-based format detection
   - Test case-insensitive extension matching (.MKV, .mkv)

### Integration Testing

1. **Rescan Integration**:
   - Create test database with episodes having NULL/0 lengths
   - Run rescan and verify lengths are updated
   - Verify rescan continues after extraction failures

2. **Edit Mode Integration**:
   - Select episode with NULL/0 length
   - Enter edit mode
   - Verify length is extracted and displayed

3. **Playback Integration**:
   - Select episode with NULL/0 length
   - Trigger playback
   - Verify length is extracted before player launches

### Edge Cases

1. **Empty/Zero Duration Files**: Files that report 0 duration
2. **Very Long Videos**: Files with duration > 24 hours
3. **Missing Metadata**: Files without duration information
4. **Concurrent Access**: Multiple operations trying to update same episode
5. **Invalid File Paths**: Relative paths that don't resolve correctly

## Implementation Notes

### Rust Crates for Video Parsing

Based on research, the following crates are recommended:

1. **matroska** (or **ebml-iterable**): For MKV files
   - Pure Rust implementation
   - Supports reading duration from MKV metadata

2. **mp4parse** (or **mp4**): For MP4 files
   - Pure Rust MP4 parser
   - Extracts duration from moov atom

3. **avi** crate or manual parsing: For AVI files
   - AVI format is simpler, may require manual parsing
   - Duration can be calculated from frame count and frame rate

### Database Query for Missing Lengths

```sql
SELECT id, path FROM episode WHERE length IS NULL OR length = 0
```

### Display Integration

The current display code in `src/display.rs` shows episode length. Update the formatting to use `format_duration_hms` when displaying the length field:

```rust
// Current: displays minutes as integer
// New: format seconds as hh:mm:ss
let formatted = format_duration_hms(length_seconds);
```

### Performance Considerations

- Video file parsing can be slow for large files
- Consider adding a progress indicator during rescan
- Cache results to avoid re-parsing the same file
- Process files sequentially during rescan (don't parallelize initially)

### Future Extensibility

To add support for a new format (e.g., .webm):

1. Add a new handler function: `extract_webm_duration`
2. Update the format detection logic in `extract_duration_seconds`
3. Add the extension to the supported formats list
4. Add unit tests for the new format

No changes to integration points or database schema required.
