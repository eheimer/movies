use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

/// Errors that can occur during path resolution operations
#[derive(Debug)]
pub enum PathResolverError {
    PathNotUnderRoot(PathBuf),
    InvalidRelativePath(String),
    DatabaseNotFound(PathBuf),
    InvalidDatabasePath(PathBuf),
    IoError(io::Error),
}

impl fmt::Display for PathResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathResolverError::PathNotUnderRoot(path) => {
                write!(f, "Path is not under configured root directory: {}", path.display())
            }
            PathResolverError::InvalidRelativePath(path) => {
                write!(f, "Invalid relative path: {}", path)
            }
            PathResolverError::DatabaseNotFound(path) => {
                write!(f, "Database not found at: {}", path.display())
            }
            PathResolverError::InvalidDatabasePath(path) => {
                write!(f, "Invalid database path (no parent directory): {}", path.display())
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
/// It maintains the root directory for video files (from config.yaml)
pub struct PathResolver {
    root_dir: PathBuf,
}

impl PathResolver {
    /// Create a new PathResolver from database location
    /// 
    /// # Arguments
    /// * `db_path` - Path to the videos.sqlite database file
    /// 
    /// # Returns
    /// * `Result<Self, PathResolverError>` - New PathResolver or error
    pub fn from_database_path(db_path: &Path) -> Result<Self, PathResolverError> {
        // Validate database path exists
        if !db_path.exists() {
            crate::logger::log_warn(&format!(
                "Database not found at path: {}",
                db_path.display()
            ));
            return Err(PathResolverError::DatabaseNotFound(db_path.to_path_buf()));
        }
        
        // Get parent directory as root
        let root_dir = db_path.parent()
            .ok_or_else(|| {
                crate::logger::log_warn(&format!(
                    "Invalid database path (no parent directory): {}",
                    db_path.display()
                ));
                PathResolverError::InvalidDatabasePath(db_path.to_path_buf())
            })?
            .to_path_buf();
        
        // Canonicalize to resolve symlinks
        let canonical_root = match root_dir.canonicalize() {
            Ok(path) => path,
            Err(e) => {
                crate::logger::log_warn(&format!(
                    "Failed to canonicalize root directory {}: {}",
                    root_dir.display(),
                    e
                ));
                return Err(PathResolverError::IoError(e));
            }
        };
        
        Ok(PathResolver {
            root_dir: canonical_root,
        })
    }




    /// Get the root directory used for path resolution
    /// 
    /// # Returns
    /// * `&Path` - Reference to the root directory
    pub fn get_root_dir(&self) -> &Path {
        &self.root_dir
    }

    /// Convert an absolute path to a relative path from the configured root directory
    /// 
    /// # Arguments
    /// * `absolute_path` - The absolute path to convert
    /// 
    /// # Returns
    /// * `Result<PathBuf, PathResolverError>` - Relative path or error
    pub fn to_relative(&self, absolute_path: &Path) -> Result<PathBuf, PathResolverError> {
        crate::logger::log_debug(&format!(
            "PathResolver: Converting absolute path to relative: {}",
            absolute_path.display()
        ));
        
        // Canonicalize the input path to handle symlinks
        let canonical_path = match absolute_path.canonicalize() {
            Ok(path) => path,
            Err(e) => {
                crate::logger::log_warn(&format!(
                    "Failed to canonicalize path {}: {}",
                    absolute_path.display(),
                    e
                ));
                return Err(PathResolverError::IoError(e));
            }
        };
        
        // Validate that the path is under the root directory
        self.validate_path_under_root(&canonical_path)?;
        
        // Strip the root directory prefix to get relative path
        let relative_path = canonical_path.strip_prefix(&self.root_dir)
            .map(|p| p.to_path_buf())
            .map_err(|_| {
                crate::logger::log_warn(&format!(
                    "Failed to strip root prefix from path: {}",
                    canonical_path.display()
                ));
                PathResolverError::PathNotUnderRoot(canonical_path)
            })?;
        
        crate::logger::log_debug(&format!(
            "PathResolver: Converted to relative path: {}",
            relative_path.display()
        ));
        
        Ok(relative_path)
    }
    
    /// Convert a relative path to an absolute path using the configured root directory
    /// 
    /// # Arguments
    /// * `relative_path` - The relative path to convert
    /// 
    /// # Returns
    /// * `PathBuf` - Absolute path
    pub fn to_absolute(&self, relative_path: &Path) -> PathBuf {
        let absolute_path = self.root_dir.join(relative_path);
        crate::logger::log_debug(&format!(
            "PathResolver: Converting relative path '{}' to absolute: {}",
            relative_path.display(),
            absolute_path.display()
        ));
        absolute_path
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
        crate::logger::log_debug(&format!(
            "PathResolver: Validating path is under root: {}",
            path.display()
        ));
        
        // Canonicalize both paths for accurate comparison
        let canonical_path = path.canonicalize()
            .map_err(PathResolverError::IoError)?;
        
        // Check if the path starts with the root directory
        if !canonical_path.starts_with(&self.root_dir) {
            crate::logger::log_warn(&format!(
                "Path validation failed: {} is not under root directory {}",
                canonical_path.display(),
                self.root_dir.display()
            ));
            return Err(PathResolverError::PathNotUnderRoot(canonical_path));
        }
        
        // Additional check: ensure no path components contain ".." 
        // This prevents directory traversal even in edge cases
        for component in path.components() {
            if let std::path::Component::ParentDir = component {
                crate::logger::log_warn(&format!(
                    "Path validation failed: {} contains parent directory reference",
                    path.display()
                ));
                crate::logger::log_debug(&format!(
                    "PathResolver: Detected parent directory reference in path components"
                ));
                return Err(PathResolverError::InvalidRelativePath(
                    format!("Path contains parent directory reference: {}", path.display())
                ));
            }
        }
        
        crate::logger::log_debug(&format!(
            "PathResolver: Path validation successful for {}",
            path.display()
        ));
        
        Ok(())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_from_database_path_valid() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Create a test database file
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
        // Root should be the parent directory of the database
        let expected_root = temp_path.canonicalize().unwrap();
        assert_eq!(resolver.root_dir, expected_root);
    }
    
    #[test]
    fn test_from_database_path_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Try to create resolver with non-existent database
        let db_path = temp_path.join("nonexistent.sqlite");
        let result = PathResolver::from_database_path(&db_path);
        
        assert!(result.is_err());
        match result {
            Err(PathResolverError::DatabaseNotFound(_)) => {
                // Expected error type
            }
            _ => panic!("Expected DatabaseNotFound error"),
        }
    }
    
    #[test]
    fn test_get_root_dir() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Create a test database file
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
        // get_root_dir should return reference to root directory
        let root = resolver.get_root_dir();
        let expected_root = temp_path.canonicalize().unwrap();
        assert_eq!(root, expected_root);
    }
    
    #[test]
    fn test_relative_absolute_conversion() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Create a test database file
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        // Create a test video file
        let test_file = temp_path.join("test_video.mp4");
        fs::write(&test_file, "test").unwrap();
        
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
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
        
        // Create a test database file
        let db_path = temp_path.join("videos.sqlite");
        fs::write(&db_path, "test").unwrap();
        
        let resolver = PathResolver::from_database_path(&db_path).unwrap();
        
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
}