use movies::player_plugin::{PlayerPlugin, CelluloidPlugin, GenericPlayerPlugin, create_player_plugin};
use std::path::Path;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_celluloid_launch_command_without_resume() {
    let plugin = CelluloidPlugin::new();
    let file_path = Path::new("/path/to/video.mp4");
    
    // When no progress data exists (None), let Celluloid handle resume
    let (command, args) = plugin.launch_command(file_path, None);
    
    assert_eq!(command, "celluloid");
    assert_eq!(args.len(), 2); // save-position-on-quit, file path
    assert_eq!(args[0], "--mpv-save-position-on-quit");
    assert_eq!(args[1], "/path/to/video.mp4");
}

#[test]
fn test_celluloid_launch_command_with_resume() {
    let plugin = CelluloidPlugin::new();
    let file_path = Path::new("/path/to/video.mp4");
    
    // When progress is explicitly 0, pass --mpv-start=0 to override watch-later
    let (command, args) = plugin.launch_command(file_path, Some(0));
    
    assert_eq!(command, "celluloid");
    assert_eq!(args.len(), 3); // save-position-on-quit, start=0, file path
    assert_eq!(args[0], "--mpv-save-position-on-quit");
    assert_eq!(args[1], "--mpv-start=0");
    assert_eq!(args[2], "/path/to/video.mp4");
}

#[test]
fn test_celluloid_launch_command_with_nonzero_progress() {
    let plugin = CelluloidPlugin::new();
    let file_path = Path::new("/path/to/video.mp4");
    
    // When progress is non-zero, ignore it and let Celluloid handle resume
    let (command, args) = plugin.launch_command(file_path, Some(120));
    
    assert_eq!(command, "celluloid");
    assert_eq!(args.len(), 2); // save-position-on-quit, file path (no --mpv-start)
    assert_eq!(args[0], "--mpv-save-position-on-quit");
    assert_eq!(args[1], "/path/to/video.mp4");
}

#[test]
fn test_celluloid_get_final_position_file_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let watch_later_dir = temp_dir.path().join("watch_later");
    fs::create_dir_all(&watch_later_dir).unwrap();
    
    let plugin = CelluloidPlugin::with_watch_later_dir(watch_later_dir);
    let file_path = Path::new("/nonexistent/video.mp4");
    
    let result = plugin.get_final_position(file_path).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_celluloid_parse_watch_later_file() {
    let temp_dir = TempDir::new().unwrap();
    let watch_later_dir = temp_dir.path().join("watch_later");
    fs::create_dir_all(&watch_later_dir).unwrap();
    
    // Create a test video file
    let video_file = temp_dir.path().join("test_video.mp4");
    fs::write(&video_file, b"fake video content").unwrap();
    
    // Calculate the hash for the video file
    let plugin = CelluloidPlugin::with_watch_later_dir(watch_later_dir.clone());
    let absolute_path = video_file.canonicalize().unwrap();
    let path_str = absolute_path.to_string_lossy();
    let digest = md5::compute(path_str.as_bytes());
    let hash = format!("{:X}", digest);
    
    // Create watch-later file with progress data
    let watch_later_file = watch_later_dir.join(&hash);
    let watch_later_content = "start=245.5\nvolume=100\n";
    fs::write(&watch_later_file, watch_later_content).unwrap();
    
    // Test retrieving the position
    let result = plugin.get_final_position(&video_file).unwrap();
    assert_eq!(result, Some(245));
}

#[test]
fn test_celluloid_parse_watch_later_file_no_start() {
    let temp_dir = TempDir::new().unwrap();
    let watch_later_dir = temp_dir.path().join("watch_later");
    fs::create_dir_all(&watch_later_dir).unwrap();
    
    // Create a test video file
    let video_file = temp_dir.path().join("test_video.mp4");
    fs::write(&video_file, b"fake video content").unwrap();
    
    // Calculate the hash for the video file
    let plugin = CelluloidPlugin::with_watch_later_dir(watch_later_dir.clone());
    let absolute_path = video_file.canonicalize().unwrap();
    let path_str = absolute_path.to_string_lossy();
    let digest = md5::compute(path_str.as_bytes());
    let hash = format!("{:X}", digest);
    
    // Create watch-later file without start position
    let watch_later_file = watch_later_dir.join(&hash);
    let watch_later_content = "volume=100\nfullscreen=yes\n";
    fs::write(&watch_later_file, watch_later_content).unwrap();
    
    // Test retrieving the position - should return Err for invalid file
    let result = plugin.get_final_position(&video_file);
    assert!(result.is_err());
}

#[test]
fn test_generic_player_vlc_launch_command() {
    let plugin = GenericPlayerPlugin::new("/usr/bin/vlc".to_string());
    let file_path = Path::new("/path/to/video.mp4");
    
    let (command, args) = plugin.launch_command(file_path, Some(60));
    
    assert_eq!(command, "/usr/bin/vlc");
    assert!(args.contains(&"--start-time=60".to_string()));
    assert!(args.contains(&"/path/to/video.mp4".to_string()));
}

#[test]
fn test_generic_player_mpv_launch_command() {
    let plugin = GenericPlayerPlugin::new("/usr/bin/mpv".to_string());
    let file_path = Path::new("/path/to/video.mp4");
    
    let (command, args) = plugin.launch_command(file_path, Some(90));
    
    assert_eq!(command, "/usr/bin/mpv");
    assert!(args.contains(&"--start=90".to_string()));
    assert!(args.contains(&"/path/to/video.mp4".to_string()));
}

#[test]
fn test_generic_player_no_resume_support() {
    let plugin = GenericPlayerPlugin::new("/usr/bin/vlc".to_string());
    let file_path = Path::new("/path/to/video.mp4");
    
    let result = plugin.get_final_position(file_path).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_create_player_plugin_celluloid() {
    let plugin = create_player_plugin("/usr/bin/celluloid");
    let file_path = Path::new("/path/to/video.mp4");
    
    let (command, args) = plugin.launch_command(file_path, None);
    
    assert_eq!(command, "celluloid");
    assert!(args.contains(&"--mpv-save-position-on-quit".to_string()));
}

#[test]
fn test_create_player_plugin_generic() {
    let plugin = create_player_plugin("/usr/bin/vlc");
    let file_path = Path::new("/path/to/video.mp4");
    
    let (command, args) = plugin.launch_command(file_path, None);
    
    assert_eq!(command, "/usr/bin/vlc");
    assert!(args.contains(&"/path/to/video.mp4".to_string()));
}
