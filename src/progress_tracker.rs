use crate::database;
use crate::config::Config;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use std::process::Child;

/// Progress tracking service for monitoring video playback
pub struct ProgressTracker {
    episode_id: usize,
    total_duration: u64,
    watched_threshold: u8,
    update_interval: Duration,
    stop_sender: Option<mpsc::Sender<()>>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl std::fmt::Debug for ProgressTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgressTracker")
            .field("episode_id", &self.episode_id)
            .field("total_duration", &self.total_duration)
            .field("watched_threshold", &self.watched_threshold)
            .field("update_interval", &self.update_interval)
            .field("stop_sender", &self.stop_sender.is_some())
            .field("thread_handle", &self.thread_handle.is_some())
            .finish()
    }
}

impl ProgressTracker {
    /// Create a new progress tracker for an episode
    pub fn new(episode_id: usize, total_duration: u64, threshold: u8) -> Self {
        Self {
            episode_id,
            total_duration,
            watched_threshold: threshold,
            update_interval: Duration::from_secs(10),
            stop_sender: None,
            thread_handle: None,
        }
    }

    /// Start progress tracking in a background thread
    pub fn start_tracking(&mut self, player_process: Arc<Mutex<Child>>) -> Result<(), Box<dyn std::error::Error>> {
        // Create communication channel for stopping the tracker
        let (stop_sender, stop_receiver) = mpsc::channel();
        self.stop_sender = Some(stop_sender);

        let episode_id = self.episode_id;
        let total_duration = self.total_duration;
        let watched_threshold = self.watched_threshold;
        let update_interval = self.update_interval;

        // Spawn background thread for progress tracking
        let handle = thread::spawn(move || {
            let mut start_time = Instant::now();
            let mut last_progress = 0u64;
            let mut auto_watched_triggered = false;

            crate::logger::log_info(&format!(
                "Started progress tracking for episode {} (duration: {}s, threshold: {}%)",
                episode_id, total_duration, watched_threshold
            ));

            loop {
                // Check if we should stop tracking
                if stop_receiver.try_recv().is_ok() {
                    crate::logger::log_debug(&format!(
                        "Progress tracking stopped for episode {} at {}s",
                        episode_id, last_progress
                    ));
                    break;
                }

                // Check if video player process is still running
                {
                    let mut process = match player_process.lock() {
                        Ok(p) => p,
                        Err(_) => {
                            crate::logger::log_warn(&format!(
                                "Failed to lock player process for episode {}. Stopping progress tracking.",
                                episode_id
                            ));
                            break;
                        }
                    };

                    match process.try_wait() {
                        Ok(Some(_)) => {
                            // Process has exited
                            crate::logger::log_info(&format!(
                                "Video player exited for episode {}. Final progress: {}s",
                                episode_id, last_progress
                            ));
                            break;
                        }
                        Ok(None) => {
                            // Process is still running, continue tracking
                        }
                        Err(e) => {
                            crate::logger::log_warn(&format!(
                                "Error checking player process status for episode {}: {}. Stopping progress tracking.",
                                episode_id, e
                            ));
                            break;
                        }
                    }
                }

                // Calculate current progress based on elapsed time
                let elapsed = start_time.elapsed();
                let current_progress = last_progress + elapsed.as_secs();

                // Don't exceed total duration
                let current_progress = current_progress.min(total_duration);

                // Update progress in database
                if let Err(e) = database::update_episode_progress(episode_id, current_progress) {
                    crate::logger::log_error(&format!(
                        "Failed to update progress for episode {}: {}",
                        episode_id, e
                    ));
                } else {
                    crate::logger::log_debug(&format!(
                        "Updated progress for episode {}: {}s / {}s ({}%)",
                        episode_id,
                        current_progress,
                        total_duration,
                        if total_duration > 0 { (current_progress * 100) / total_duration } else { 0 }
                    ));
                }

                // Check if we should auto-mark as watched
                if !auto_watched_triggered && total_duration > 0 {
                    let progress_percentage = (current_progress * 100) / total_duration;
                    if progress_percentage >= watched_threshold as u64 {
                        if let Err(e) = database::mark_episode_watched_with_timestamp(episode_id) {
                            crate::logger::log_error(&format!(
                                "Failed to auto-mark episode {} as watched: {}",
                                episode_id, e
                            ));
                        } else {
                            crate::logger::log_info(&format!(
                                "Auto-marked episode {} as watched ({}% >= {}% threshold)",
                                episode_id, progress_percentage, watched_threshold
                            ));
                            auto_watched_triggered = true;
                        }
                    }
                }

                // Update tracking state
                last_progress = current_progress;
                start_time = Instant::now();

                // Sleep for update interval
                thread::sleep(update_interval);
            }
        });

        self.thread_handle = Some(handle);
        Ok(())
    }

    /// Stop progress tracking
    pub fn stop_tracking(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Send stop signal
        if let Some(sender) = self.stop_sender.take() {
            sender.send(())?;
        }

        // Wait for thread to finish
        if let Some(handle) = self.thread_handle.take() {
            handle.join().map_err(|_| "Failed to join progress tracking thread")?;
        }

        crate::logger::log_debug(&format!(
            "Progress tracking stopped for episode {}",
            self.episode_id
        ));

        Ok(())
    }
}

impl Drop for ProgressTracker {
    fn drop(&mut self) {
        // Ensure tracking is stopped when tracker is dropped
        let _ = self.stop_tracking();
    }
}

/// Create and start progress tracking for an episode
pub fn start_progress_tracking(
    episode_id: usize,
    total_duration: u64,
    config: &Config,
    player_process: Arc<Mutex<Child>>,
) -> Result<ProgressTracker, Box<dyn std::error::Error>> {
    let mut tracker = ProgressTracker::new(episode_id, total_duration, config.watched_threshold);
    tracker.start_tracking(player_process)?;
    Ok(tracker)
}