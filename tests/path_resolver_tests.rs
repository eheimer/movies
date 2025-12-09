use movies::path_resolver::*;
use std::fs;
use std::path::PathBuf;
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
    assert_eq!(resolver.get_root_dir(), expected_root);
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
