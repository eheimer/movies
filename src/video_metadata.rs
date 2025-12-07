use std::path::Path;
use std::error::Error;
use crate::database;

/// Extract duration in seconds from a video file
/// Supports MKV, MP4, and AVI formats
/// Falls back to ffprobe if native parsing fails
pub fn extract_duration_seconds(file_path: &Path) -> Result<u64, Box<dyn Error>> {
    // Detect format based on file extension
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or("File has no extension")?
        .to_lowercase();
    
    // Try native parser first
    let native_result = match extension.as_str() {
        "mkv" => extract_mkv_duration(file_path),
        "mp4" | "m4v" => extract_mp4_duration(file_path),
        "avi" => extract_avi_duration(file_path),
        _ => Err(format!("Unsupported video format: {}", extension).into()),
    };
    
    // If native parser succeeds, return the result
    if let Ok(duration) = native_result {
        return Ok(duration);
    }
    
    // If native parser fails, try ffprobe as fallback
    crate::logger::log_debug(&format!(
        "Native parser failed for {}, trying ffprobe fallback",
        file_path.display()
    ));
    
    extract_duration_with_ffprobe(file_path)
}

/// Extract duration using ffprobe (fallback method)
/// This requires ffprobe to be installed on the system
fn extract_duration_with_ffprobe(file_path: &Path) -> Result<u64, Box<dyn Error>> {
    use std::process::Command;
    
    // Try to run ffprobe
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(file_path)
        .output()
        .map_err(|e| format!("Failed to run ffprobe (is it installed?): {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ffprobe failed: {}", stderr).into());
    }
    
    // Parse the duration from stdout
    let duration_str = String::from_utf8_lossy(&output.stdout);
    let duration_float: f64 = duration_str
        .trim()
        .parse()
        .map_err(|e| format!("Failed to parse ffprobe output '{}': {}", duration_str.trim(), e))?;
    
    // Convert to seconds (round up)
    let duration_seconds = duration_float.ceil() as u64;
    
    Ok(duration_seconds)
}

/// Extract duration from MKV files
fn extract_mkv_duration(file_path: &Path) -> Result<u64, Box<dyn Error>> {
    use std::fs::File;
    use matroska::Matroska;
    
    let file = File::open(file_path)
        .map_err(|e| format!("Failed to open MKV file: {}", e))?;
    
    let matroska = Matroska::open(file)
        .map_err(|e| format!("Failed to parse MKV file: {}", e))?;
    
    // Get duration from the info segment (in nanoseconds)
    let duration = matroska.info.duration
        .ok_or("MKV file does not contain duration information")?;
    
    // Convert Duration to seconds
    let duration_seconds = duration.as_secs();
    
    Ok(duration_seconds)
}

/// Extract duration from MP4 files
fn extract_mp4_duration(file_path: &Path) -> Result<u64, Box<dyn Error>> {
    use std::fs::File;
    use mp4::{Mp4Reader, TrackType};
    
    let file = File::open(file_path)
        .map_err(|e| format!("Failed to open MP4 file: {}", e))?;
    
    let size = file.metadata()?.len();
    let reader = Mp4Reader::read_header(file, size)
        .map_err(|e| format!("Failed to parse MP4 file: {}", e))?;
    
    // Try to get duration from movie header first (mvhd)
    if reader.duration().as_secs() > 0 {
        return Ok(reader.duration().as_secs());
    }
    
    // Fallback: Get duration from the first video track
    for track_id in reader.tracks().keys() {
        let track = reader.tracks().get(track_id)
            .ok_or("Failed to get track")?;
        
        if track.track_type()? == TrackType::Video {
            let duration_seconds = track.duration().as_secs();
            if duration_seconds > 0 {
                return Ok(duration_seconds);
            }
        }
    }
    
    // If we get here, try any track with a duration
    for track_id in reader.tracks().keys() {
        let track = reader.tracks().get(track_id)
            .ok_or("Failed to get track")?;
        
        let duration_seconds = track.duration().as_secs();
        if duration_seconds > 0 {
            return Ok(duration_seconds);
        }
    }
    
    Err("No duration information found in MP4 file".into())
}

/// Extract duration from AVI files
fn extract_avi_duration(file_path: &Path) -> Result<u64, Box<dyn Error>> {
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};
    use riff::Chunk;
    
    let mut file = File::open(file_path)
        .map_err(|e| format!("Failed to open AVI file: {}", e))?;
    
    // Read the RIFF chunk
    let chunk = Chunk::read(&mut file, 0)
        .map_err(|e| format!("Failed to parse AVI file: {}", e))?;
    
    // Verify it's an AVI file
    if chunk.id().as_str() != "RIFF" {
        return Err("Not a valid RIFF file".into());
    }
    
    // Read the form type (should be "AVI ")
    file.seek(SeekFrom::Start(8))?;
    let mut form_type = [0u8; 4];
    file.read_exact(&mut form_type)?;
    
    if &form_type != b"AVI " {
        return Err("Not a valid AVI file".into());
    }
    
    // Look for the 'avih' (AVI header) chunk
    file.seek(SeekFrom::Start(12))?;
    
    loop {
        let pos = file.stream_position()?;
        
        match Chunk::read(&mut file, pos) {
            Ok(subchunk) => {
                if subchunk.id().as_str() == "LIST" {
                    // Read LIST type
                    let list_pos = file.stream_position()?;
                    let mut list_type = [0u8; 4];
                    file.read_exact(&mut list_type)?;
                    
                    if &list_type == b"hdrl" {
                        // Found header list, look for avih
                        loop {
                            let inner_pos = file.stream_position()?;
                            if inner_pos >= list_pos + subchunk.len() as u64 {
                                break;
                            }
                            
                            match Chunk::read(&mut file, inner_pos) {
                                Ok(inner_chunk) => {
                                    if inner_chunk.id().as_str() == "avih" {
                                        // Found AVI header, read microseconds per frame
                                        let mut header = [0u8; 56];
                                        file.read_exact(&mut header)?;
                                        
                                        // First 4 bytes are microseconds per frame
                                        let us_per_frame = u32::from_le_bytes([
                                            header[0], header[1], header[2], header[3]
                                        ]);
                                        
                                        // Bytes 16-19 are total frames
                                        let total_frames = u32::from_le_bytes([
                                            header[16], header[17], header[18], header[19]
                                        ]);
                                        
                                        if us_per_frame == 0 || total_frames == 0 {
                                            return Err("Invalid AVI header data".into());
                                        }
                                        
                                        // Calculate duration in seconds
                                        let duration_us = us_per_frame as u64 * total_frames as u64;
                                        let duration_seconds = duration_us / 1_000_000;
                                        
                                        return Ok(duration_seconds);
                                    }
                                    
                                    // Skip to next chunk
                                    let next_pos = inner_pos + 8 + inner_chunk.len() as u64;
                                    file.seek(SeekFrom::Start(next_pos))?;
                                }
                                Err(_) => break,
                            }
                        }
                    }
                    
                    // Skip to next chunk
                    let next_pos = pos + 8 + subchunk.len() as u64;
                    file.seek(SeekFrom::Start(next_pos))?;
                } else {
                    // Skip to next chunk
                    let next_pos = pos + 8 + subchunk.len() as u64;
                    file.seek(SeekFrom::Start(next_pos))?;
                }
            }
            Err(_) => break,
        }
        
        // Check if we've reached the end of the file
        if file.stream_position()? >= file.metadata()?.len() {
            break;
        }
    }
    
    Err("AVI header not found in file".into())
}

/// Extract duration and update episode length in database
/// 
/// # Arguments
/// * `episode_id` - The ID of the episode to update
/// * `file_path` - Path to the video file
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Ok if successful, error otherwise
/// 
/// This function extracts the duration from the video file and updates
/// the episode.length field in the database with the duration in seconds.
/// If extraction fails, the database is not updated and an error is returned.
pub fn extract_and_update_episode_length(
    episode_id: usize,
    file_path: &Path,
) -> Result<(), Box<dyn Error>> {
    // Extract duration in seconds
    let duration_seconds = match extract_duration_seconds(file_path) {
        Ok(duration) => {
            crate::logger::log_debug(&format!(
                "Successfully extracted duration for episode {}: {} seconds ({})",
                episode_id,
                duration,
                format_duration_hms(duration)
            ));
            duration
        }
        Err(e) => {
            crate::logger::log_warn(&format!(
                "Failed to extract duration for episode {} from {}: {}",
                episode_id,
                file_path.display(),
                e
            ));
            return Err(e);
        }
    };
    
    // Update database
    let conn = database::get_connection().lock().unwrap();
    conn.execute(
        "UPDATE episode SET length = ?1 WHERE id = ?2",
        rusqlite::params![duration_seconds as i64, episode_id],
    )?;
    
    crate::logger::log_info(&format!(
        "Updated episode {} length to {} seconds ({})",
        episode_id,
        duration_seconds,
        format_duration_hms(duration_seconds)
    ));
    
    Ok(())
}

/// Format duration in seconds as "hh:mm:ss"
/// 
/// # Arguments
/// * `seconds` - Duration in seconds
/// 
/// # Returns
/// * `String` - Formatted duration as "hh:mm:ss"
/// 
/// # Examples
/// * 3661 seconds -> "01:01:01"
/// * 2730 seconds -> "00:45:30"
/// * 90000 seconds -> "25:00:00"
pub fn format_duration_hms(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}
