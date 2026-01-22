use std::fs;
use std::path::{Path, PathBuf};

/// Trait for video player plugins that support progress tracking
pub trait PlayerPlugin {
    /// Returns the command and arguments to launch the player with optional resume position
    fn launch_command(&self, file_path: &Path, start_time: Option<u64>) -> (String, Vec<String>);
    
    /// Retrieves the final playback position after the player exits
    /// Returns None if position couldn't be determined
    fn get_final_position(&self, file_path: &Path) -> Result<Option<u64>, Box<dyn std::error::Error>>;
    
    /// Clean up any watch-later or progress files to prevent stale data
    fn cleanup_progress_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Default implementation does nothing for players without progress tracking
        Ok(())
    }
    
    /// Delete the watch-later file for a specific video to force restart from beginning
    fn delete_watch_later_file(&self, _file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Default implementation does nothing for players without watch-later support
        Ok(())
    }
}

/// Celluloid/mpv plugin implementation using watch-later files
pub struct CelluloidPlugin {
    watch_later_dir: PathBuf,
}

impl CelluloidPlugin {
    /// Create a new Celluloid plugin using Celluloid's default watch-later directory
    pub fn new() -> Self {
        // Celluloid stores watch-later files in ~/.config/celluloid/watch_later
        let watch_later_dir = dirs::config_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
            .join("celluloid")
            .join("watch_later");
        
        Self { watch_later_dir }
    }
    
    /// Create a new Celluloid plugin with custom watch-later directory
    pub fn with_watch_later_dir(watch_later_dir: PathBuf) -> Self {
        Self { watch_later_dir }
    }
    
    /// Calculate MD5 hash of file path for watch-later file lookup
    fn calculate_path_hash(&self, file_path: &Path) -> String {
        // Get absolute path
        let absolute_path = file_path.canonicalize()
            .unwrap_or_else(|_| file_path.to_path_buf());
        
        // Calculate MD5 hash of the absolute path string
        let path_str = absolute_path.to_string_lossy();
        
        crate::logger::log_debug(&format!(
            "Calculating hash for path: {}",
            path_str
        ));
        
        let digest = md5::compute(path_str.as_bytes());
        
        // Convert to uppercase hex string
        let hash = format!("{:X}", digest);
        
        crate::logger::log_debug(&format!(
            "Calculated hash: {}",
            hash
        ));
        
        hash
    }
    
    /// Parse watch-later file to extract playback position
    fn parse_watch_later_file(&self, file_path: &Path) -> Result<Option<u64>, Box<dyn std::error::Error>> {
        // Read the watch-later file
        let content = fs::read_to_string(file_path)?;
        
        // Check if file is empty or too small to be valid
        if content.trim().is_empty() {
            crate::logger::log_warn(&format!(
                "Watch-later file is empty: {}",
                file_path.display()
            ));
            return Err("Empty watch-later file".into());
        }
        
        // Check for "redirect entry" which indicates an invalid/placeholder file
        if content.contains("# redirect entry") {
            crate::logger::log_warn(&format!(
                "Watch-later file is a redirect entry (invalid): {}",
                file_path.display()
            ));
            return Err("Redirect entry watch-later file".into());
        }
        
        // Look for "start=" line
        for line in content.lines() {
            if let Some(value_str) = line.strip_prefix("start=") {
                // Parse the numeric value (may be float)
                if let Ok(seconds) = value_str.trim().parse::<f64>() {
                    return Ok(Some(seconds as u64));
                }
            }
        }
        
        // No start position found - file exists but has no valid position
        crate::logger::log_warn(&format!(
            "Watch-later file has no start= line: {}",
            file_path.display()
        ));
        Err("No start position in watch-later file".into())
    }
    
    /// Clean up watch-later files - only removes invalid/empty files
    pub fn cleanup_watch_later_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Don't clean up valid watch-later files - Celluloid needs them for resume
        // Only remove files that are empty or invalid
        Ok(())
    }
    
    /// Delete the watch-later file for a specific video file
    /// Used when marking an episode as watched to ensure it starts from beginning next time
    pub fn delete_watch_later_file(&self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let hash = self.calculate_path_hash(file_path);
        let watch_later_file = self.watch_later_dir.join(&hash);
        
        if watch_later_file.exists() {
            fs::remove_file(&watch_later_file)?;
            crate::logger::log_debug(&format!(
                "Deleted watch-later file for {}: {}",
                file_path.display(),
                watch_later_file.display()
            ));
        }
        
        Ok(())
    }
}

impl Default for CelluloidPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerPlugin for CelluloidPlugin {
    fn launch_command(&self, file_path: &Path, start_time: Option<u64>) -> (String, Vec<String>) {
        // Let Celluloid/mpv manage resume position, but pass --mpv-start=0
        // when we explicitly want to start from the beginning (e.g., after marking as watched)
        
        let mut args = vec![];
        
        // Celluloid uses --mpv-OPTION=VALUE format to pass options to mpv
        args.push("--mpv-save-position-on-quit".to_string());
        
        // If start_time is Some(0), explicitly tell Celluloid to start from beginning
        // This overrides any existing watch-later file
        if let Some(0) = start_time {
            args.push("--mpv-start=0".to_string());
        }
        // If start_time is None, let Celluloid handle resume automatically
        // If start_time is Some(non-zero), ignore it and let Celluloid handle it
        
        args.push(file_path.to_string_lossy().to_string());
        
        ("celluloid".to_string(), args)
    }
    
    fn get_final_position(&self, file_path: &Path) -> Result<Option<u64>, Box<dyn std::error::Error>> {
        // Calculate MD5 hash of file path
        let hash = self.calculate_path_hash(file_path);
        
        // Construct watch-later file path
        let watch_later_file = self.watch_later_dir.join(&hash);
        
        // Log the watch-later directory and expected file
        crate::logger::log_debug(&format!(
            "Looking for watch-later file: {} (hash: {})",
            watch_later_file.display(),
            hash
        ));
        
        // List files in watch-later directory for debugging
        if let Ok(entries) = std::fs::read_dir(&self.watch_later_dir) {
            let files: Vec<String> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().to_string())
                .collect();
            crate::logger::log_debug(&format!(
                "Files in watch-later directory: {:?}",
                files
            ));
        }
        
        // Check if watch-later file exists
        if !watch_later_file.exists() {
            crate::logger::log_debug(&format!(
                "Watch-later file not found for {}: {}",
                file_path.display(),
                watch_later_file.display()
            ));
            return Ok(None);
        }
        
        // Parse the watch-later file
        match self.parse_watch_later_file(&watch_later_file) {
            Ok(position) => {
                if let Some(pos) = position {
                    crate::logger::log_debug(&format!(
                        "Retrieved final position {}s from watch-later file for {}",
                        pos,
                        file_path.display()
                    ));
                }
                Ok(position)
            }
            Err(e) => {
                crate::logger::log_warn(&format!(
                    "Failed to parse watch-later file for {}: {}",
                    file_path.display(),
                    e
                ));
                Err(e)
            }
        }
    }
    
    fn cleanup_progress_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cleanup_watch_later_files()
    }
    
    fn delete_watch_later_file(&self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.delete_watch_later_file(file_path)
    }
}

/// Generic player plugin for players that don't have specific progress tracking support
pub struct GenericPlayerPlugin {
    player_command: String,
}

impl GenericPlayerPlugin {
    /// Create a new generic player plugin
    pub fn new(player_command: String) -> Self {
        Self { player_command }
    }
}

impl PlayerPlugin for GenericPlayerPlugin {
    fn launch_command(&self, file_path: &Path, start_time: Option<u64>) -> (String, Vec<String>) {
        let mut args = vec![];
        
        // Try to add resume position based on common player formats
        if let Some(seconds) = start_time {
            // Extract player name from path
            let player_name = Path::new(&self.player_command)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            if player_name.contains("vlc") {
                args.push(format!("--start-time={}", seconds));
            } else if player_name.contains("mpv") {
                args.push(format!("--start={}", seconds));
            } else if player_name.contains("mplayer") {
                args.push("-ss".to_string());
                args.push(seconds.to_string());
            } else if player_name.contains("ffplay") {
                args.push("-ss".to_string());
                args.push(seconds.to_string());
            } else {
                // Fallback: try common -ss format
                args.push("-ss".to_string());
                args.push(seconds.to_string());
            }
        }
        
        args.push(file_path.to_string_lossy().to_string());
        
        (self.player_command.clone(), args)
    }
    
    fn get_final_position(&self, _file_path: &Path) -> Result<Option<u64>, Box<dyn std::error::Error>> {
        // Generic players don't support progress retrieval
        Ok(None)
    }
}

/// Create a player plugin based on the configured video player
pub fn create_player_plugin(player_path: &str) -> Box<dyn PlayerPlugin> {
    // Extract player name from path
    let player_name = Path::new(player_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // Check if it's Celluloid
    if player_name.contains("celluloid") {
        Box::new(CelluloidPlugin::new())
    } else {
        // Use generic plugin for other players
        Box::new(GenericPlayerPlugin::new(player_path.to_string()))
    }
}
