use std::env;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

/// Errors that can occur during path resolution operations
#[derive(Debug)]
pub enum PathResolverError {
    ExecutableLocationNotFound,
    RootDirectoryNotFound(PathBuf),
    RootDirectoryNotAccessible(PathBuf),
    PathNotUnderRoot(PathBuf),
    InvalidRelativePath(String),
    IoError(io::Error),
}

impl fmt::Display for PathResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathResolverError::ExecutableLocationNotFound => {
                write!(f, "Could not determine executable location")
            }
            PathResolverError::RootDirectoryNotFound(path) => {
                write!(f, "Root directory not found: {}", path.display())
            }
            PathResolverError::RootDirectoryNotAccessible(path) => {
                write!(f, "Root directory not accessible: {}", path.display())
            }
            PathResolverError::PathNotUnderRoot(path) => {
                write!(f, "Path is not under configured root directory: {}", path.display())
            }
            PathResolverError::InvalidRelativePath(path) => {
                write!(f, "Invalid relative path: {}", path)
            }
            PathResolverError::IoError(err) => {
                write!(f, "IO error: {}", err)
            }
        }
    }
}

impl std::error::Error for PathResolverError {}

impl From<io::Error> for PathResolverError {
    fn from(err: io::Error) -> Self {
        PathResolverError::IoError(err)
    }
}

/// PathResolver handles all path resolution logic for the application
/// 
/// It maintains two key directories:
/// - executable_dir: Where the application executable is located (used for database)
/// - root_dir: Configurable root directory for video files (from config.json)
pub struct PathResolver {
    executable_dir: PathBuf,
    root_dir: PathBuf,
}

impl PathResolver {
    /// Create a new PathResolver with optional configurable root directory
    /// 
    /// # Arguments
    /// * `config_root_dir` - Optional root directory path from config.json
    ///                      If None, defaults to executable directory
    /// 
    /// # Returns
    /// * `Result<Self, PathResolverError>` - New PathResolver or error
    pub fn new(config_root_dir: Option<&str>) -> Result<Self, PathResolverError> {
        // Get executable directory
        let executable_dir = Self::detect_executable_directory()?;
        
        // Determine root directory
        let root_dir = match config_root_dir {
            Some(root_path) => {
                let root_path_buf = PathBuf::from(root_path);
                
                // Handle both absolute and relative paths
                let resolved_root = if root_path_buf.is_absolute() {
                    root_path_buf
                } else {
                    executable_dir.join(&root_path_buf)
                };
                
                // Validate root directory exists and is accessible
                if !resolved_root.exists() {
                    return Err(PathResolverError::RootDirectoryNotFound(resolved_root));
                }
                
                if !resolved_root.is_dir() {
                    return Err(PathResolverError::RootDirectoryNotAccessible(resolved_root));
                }
                
                // Canonicalize to resolve any symlinks and get absolute path
                resolved_root.canonicalize()
                    .map_err(|_| PathResolverError::RootDirectoryNotAccessible(resolved_root))?
            }
            None => executable_dir.clone(),
        };
        
        Ok(PathResolver {
            executable_dir,
            root_dir,
        })
    }
    
    /// Detect the directory containing the application executable
    fn detect_executable_directory() -> Result<PathBuf, PathResolverError> {
        let exe_path = env::current_exe()
            .map_err(|_| PathResolverError::ExecutableLocationNotFound)?;
        
        let exe_dir = exe_path.parent()
            .ok_or(PathResolverError::ExecutableLocationNotFound)?
            .to_path_buf();
        
        // Canonicalize to get absolute path
        exe_dir.canonicalize()
            .map_err(|_| PathResolverError::ExecutableLocationNotFound)
    }
    
    /// Get the path where the database file should be stored
    /// Always returns a path in the executable directory
    pub fn get_database_path(&self) -> PathBuf {
        self.executable_dir.join("videos.db")
    }
    
    /// Convert an absolute path to a relative path from the configured root directory
    /// 
    /// # Arguments
    /// * `absolute_path` - The absolute path to convert
    /// 
    /// # Returns
    /// * `Result<PathBuf, PathResolverError>` - Relative path or error
    pub fn to_relative(&self, absolute_path: &Path) -> Result<PathBuf, PathResolverError> {
        // Canonicalize the input path to handle symlinks
        let canonical_path = absolute_path.canonicalize()
            .map_err(PathResolverError::IoError)?;
        
        // Validate that the path is under the root directory
        self.validate_path_under_root(&canonical_path)?;
        
        // Strip the root directory prefix to get relative path
        canonical_path.strip_prefix(&self.root_dir)
            .map(|p| p.to_path_buf())
            .map_err(|_| PathResolverError::PathNotUnderRoot(canonical_path))
    }
    
    /// Convert a relative path to an absolute path using the configured root directory
    /// 
    /// # Arguments
    /// * `relative_path` - The relative path to convert
    /// 
    /// # Returns
    /// * `PathBuf` - Absolute path
    pub fn to_absolute(&self, relative_path: &Path) -> PathBuf {
        self.root_dir.join(relative_path)
    }
    
    /// Resolve a path from config.json relative to the configured root directory
    /// 
    /// # Arguments
    /// * `config_path` - Path string from config file
    /// 
    /// # Returns
    /// * `PathBuf` - Resolved absolute path
    pub fn resolve_config_path(&self, config_path: &str) -> PathBuf {
        let path = PathBuf::from(config_path);
        
        if path.is_absolute() {
            path
        } else {
            self.root_dir.join(path)
        }
    }
    
    /// Validate that a path is under the configured root directory
    /// This enforces strict path validation to prevent directory traversal
    /// 
    /// # Arguments
    /// * `path` - The path to validate
    /// 
    /// # Returns
    /// * `Result<(), PathResolverError>` - Ok if valid, error if not under root
    pub fn validate_path_under_root(&self, path: &Path) -> Result<(), PathResolverError> {
        // Canonicalize both paths for accurate comparison
        let canonical_path = path.canonicalize()
            .map_err(PathResolverError::IoError)?;
        
        // Check if the path starts with the root directory
        if !canonical_path.starts_with(&self.root_dir) {
            return Err(PathResolverError::PathNotUnderRoot(canonical_path));
        }
        
        // Additional check: ensure no path components contain ".." 
        // This prevents directory traversal even in edge cases
        for component in path.components() {
            if let std::path::Component::ParentDir = component {
                return Err(PathResolverError::InvalidRelativePath(
                    format!("Path contains parent directory reference: {}", path.display())
                ));
            }
        }
        
        Ok(())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_path_resolver_creation_with_default_root() {
        let resolver = PathResolver::new(None).unwrap();
        
        // Should use executable directory as root when no config provided
        assert_eq!(resolver.root_dir, resolver.executable_dir);
    }
    
    #[test]
    fn test_database_path_always_in_executable_dir() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().to_str().unwrap();
        
        let resolver = PathResolver::new(Some(temp_path)).unwrap();
        let db_path = resolver.get_database_path();
        
        // Database should be in executable directory, not the configured root
        assert!(db_path.starts_with(&resolver.executable_dir));
        assert_eq!(db_path.file_name().unwrap(), "videos.db");
    }
    
    #[test]
    fn test_relative_absolute_conversion() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Create a test file
        let test_file = temp_path.join("test_video.mp4");
        fs::write(&test_file, "test").unwrap();
        
        let resolver = PathResolver::new(Some(temp_path.to_str().unwrap())).unwrap();
        
        // Convert to relative
        let relative = resolver.to_relative(&test_file).unwrap();
        assert_eq!(relative, PathBuf::from("test_video.mp4"));
        
        // Convert back to absolute
        let absolute = resolver.to_absolute(&relative);
        assert_eq!(absolute, test_file);
    }
    
    #[test]
    fn test_path_validation_rejects_outside_root() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        let resolver = PathResolver::new(Some(temp_path.to_str().unwrap())).unwrap();
        
        // Create a file outside the root directory
        let parent_dir = temp_path.parent().unwrap();
        let outside_path = parent_dir.join("outside.mp4");
        fs::write(&outside_path, "test").unwrap();
        
        let result = resolver.validate_path_under_root(&outside_path);
        assert!(result.is_err());
        
        match result {
            Err(PathResolverError::PathNotUnderRoot(_)) => {
                // Expected error type
            }
            Err(other_error) => {
                panic!("Expected PathNotUnderRoot error, got: {:?}", other_error);
            }
            Ok(_) => {
                panic!("Expected error, but validation passed");
            }
        }
    }
    
    #[test]
    fn test_config_path_resolution() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        let resolver = PathResolver::new(Some(temp_path.to_str().unwrap())).unwrap();
        
        // Test relative path resolution
        let relative_config_path = "Videos";
        let resolved = resolver.resolve_config_path(relative_config_path);
        assert_eq!(resolved, temp_path.join("Videos"));
        
        // Test absolute path (should remain unchanged)
        let absolute_config_path = "/usr/bin/vlc";
        let resolved_abs = resolver.resolve_config_path(absolute_config_path);
        assert_eq!(resolved_abs, PathBuf::from("/usr/bin/vlc"));
    }
}