mod config;
mod display;

use config::{Config, read_config};
use display::{initialize_terminal, restore_terminal, draw_screen, get_terminal_size};
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Child, Stdio};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use walkdir::WalkDir;

fn run_video_player(config: &Config, file_path: &Path) -> io::Result<Child> {
    Command::new(&config.video_player)
        .arg(file_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

fn main_loop(entries: Vec<PathBuf>, config: Config) -> io::Result<()> {
    let mut current_item = 0;
    let mut redraw = true;
    let mut search: String = String::new();
    let mut filtered_entries: Vec<PathBuf> = entries.clone();
    let mut window_start = 0;
    let mut playing_file: Option<PathBuf> = None;

    // Create a channel to communicate between the thread and the main loop
    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();

    loop {
        
        if redraw {
            // Split the search string into terms
            let search_terms: Vec<String> = search.to_lowercase().split_whitespace().map(String::from).collect();

            // Filter entries based on the search terms (case-insensitive)
            filtered_entries = entries.iter()
                .filter(|entry| {
                    let file_name = entry.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                    search_terms.iter().all(|term| file_name.contains(term))
                })
                .cloned()
                .collect();

            // Sort the filtered entries by filename
            filtered_entries.sort_by(|a, b| a.file_name().unwrap_or_default().to_string_lossy().cmp(&b.file_name().unwrap_or_default().to_string_lossy()));

            // Ensure current_item is within bounds
            if current_item >= filtered_entries.len() {
                current_item = if filtered_entries.is_empty() { 0 } else { filtered_entries.len() - 1 };
            }

            draw_screen(&filtered_entries, current_item, &search, &config, window_start, playing_file.as_ref())?;
            redraw = false;
        }

        // Check for messages from the thread
        if let Ok(_) = rx.try_recv() {
            playing_file = None;
            redraw = true;
        }

        // Poll for events with a timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => {
                        if current_item > 0 {
                            current_item -= 1;
                            if current_item < window_start {
                                window_start = current_item;
                            }
                            redraw = true;
                        }
                    }
                    KeyCode::Down => {
                        if current_item < filtered_entries.len() - 1 {
                            current_item += 1;
                            let (_, rows) = get_terminal_size()?;
                            let max_lines = rows as usize - 6;
                            if current_item >= window_start + max_lines {
                                window_start = current_item - max_lines + 1;
                            }
                            redraw = true;
                        }
                    }
                    KeyCode::Enter => {
                        if playing_file.is_none() {
                            let selected = current_item;
                            let mut player_process = Some(run_video_player(&config, &filtered_entries[selected])?);
                            playing_file = Some(filtered_entries[selected].clone());

                            // Spawn a thread to wait for the process to finish
                            let tx = tx.clone();
                            thread::spawn(move || {
                                if let Some(mut process) = player_process.take() {
                                    process.wait().ok();
                                    tx.send(()).ok();
                                }
                            });
                        }
                        redraw = true;
                    }
                    KeyCode::Esc => break,
                    KeyCode::Backspace => {
                        //if backspace is pressed, I want to remove the last character from the search string
                        search.pop();
                        redraw = true;
                    }
                    _ => {
                        //if a displayable character is pressed, I want to add it to the search string
                        if let KeyCode::Char(c) = code {
                            search.push(c);
                            redraw = true;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let config_path = "config.json";
    let config = read_config(config_path);

    let path = if Path::new(&config.path).exists() {
        &config.path
    } else {
        eprintln!("Warning: Configured path '{}' does not exist. Using current directory instead.", config.path);
        "."
    };

    let entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path().extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |ext| config.video_extensions.contains(&ext.to_lowercase()))
        })
        .map(|e| e.into_path())
        .collect();

    initialize_terminal()?;
    let result = main_loop(entries, config);
    restore_terminal()?;
    result
}