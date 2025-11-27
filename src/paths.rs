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
