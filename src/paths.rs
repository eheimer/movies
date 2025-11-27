use directories::ProjectDirs;
use std::path::PathBuf;

pub struct AppPaths {
    pub config_file: PathBuf,
}

impl AppPaths {
    pub fn new() -> Result<Self, String> {
        let proj_dirs = ProjectDirs::from("", "", "movies")
            .ok_or("Failed to determine application directories")?;
        
        let config_dir = proj_dirs.config_dir();
        
        // Create config directory if it doesn't exist
        std::fs::create_dir_all(config_dir)
            .map_err(|e| format!("Failed to create config directory {}: {}", 
                config_dir.display(), e))?;
        
        Ok(AppPaths {
            config_file: config_dir.join("config.json"),
        })
    }
}
