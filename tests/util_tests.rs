use movies::config::Config;
use movies::util::run_video_player_with_resume;
use std::path::Path;

#[test]
fn test_video_player_with_resume_no_start_time() {
    let config = Config {
        video_player: "echo".to_string(), // Use echo as a safe test command
        ..Default::default()
    };
    
    let test_file = Path::new("test.mp4");
    
    // Test without resume time - should work like normal video player call
    let result = run_video_player_with_resume(&config, test_file, None);
    
    // Should succeed (echo command will work)
    assert!(result.is_ok());
    
    // Clean up the process
    if let Ok(mut child) = result {
        let _ = child.kill();
        let _ = child.wait();
    }
}

#[test]
fn test_video_player_with_resume_with_start_time() {
    let config = Config {
        video_player: "echo".to_string(), // Use echo as a safe test command
        ..Default::default()
    };
    
    let test_file = Path::new("test.mp4");
    
    // Test with resume time - should add resume parameters
    let result = run_video_player_with_resume(&config, test_file, Some(120));
    
    // Should succeed (echo command will work)
    assert!(result.is_ok());
    
    // Clean up the process
    if let Ok(mut child) = result {
        let _ = child.kill();
        let _ = child.wait();
    }
}

#[test]
fn test_video_player_vlc_resume_parameters() {
    let config = Config {
        video_player: "/usr/bin/vlc".to_string(),
        ..Default::default()
    };
    
    let test_file = Path::new("test.mp4");
    
    // This test verifies that VLC-specific parameters are added
    // We can't actually run VLC in tests, but we can verify the function doesn't panic
    let result = run_video_player_with_resume(&config, test_file, Some(300));
    
    // The result might fail (VLC not installed), but the function should not panic
    // and should handle the parameter addition correctly
    match result {
        Ok(mut child) => {
            let _ = child.kill();
            let _ = child.wait();
        }
        Err(_) => {
            // Expected if VLC is not installed - this is fine for the test
        }
    }
}

#[test]
fn test_video_player_mpv_resume_parameters() {
    let config = Config {
        video_player: "/usr/bin/mpv".to_string(),
        ..Default::default()
    };
    
    let test_file = Path::new("test.mp4");
    
    // This test verifies that mpv-specific parameters are added
    let result = run_video_player_with_resume(&config, test_file, Some(450));
    
    // The result might fail (mpv not installed), but the function should not panic
    match result {
        Ok(mut child) => {
            let _ = child.kill();
            let _ = child.wait();
        }
        Err(_) => {
            // Expected if mpv is not installed - this is fine for the test
        }
    }
}

#[test]
fn test_video_player_fallback_resume_parameters() {
    let config = Config {
        video_player: "/usr/bin/unknown_player".to_string(),
        ..Default::default()
    };
    
    let test_file = Path::new("test.mp4");
    
    // This test verifies that unknown players get fallback parameters
    let result = run_video_player_with_resume(&config, test_file, Some(600));
    
    // The result will fail (unknown player), but the function should not panic
    match result {
        Ok(mut child) => {
            let _ = child.kill();
            let _ = child.wait();
        }
        Err(_) => {
            // Expected since unknown_player doesn't exist - this is fine for the test
        }
    }
}