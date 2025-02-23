mod config;
mod display;
mod database;
mod util;

use config::{Config, read_config};
use database::{initialize_database, get_entries};
use display::{initialize_terminal, restore_terminal, draw_screen};
use util::Entry;
use std::io;
use std::path::Path;
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

fn main_loop(mut entries: Vec<Entry>, config: Config) -> io::Result<()> {
    let mut current_item = 0;
    let mut redraw = true;
    let mut search: String = String::new();
    let mut filtered_entries: Vec<Entry> = entries.clone();
    let mut playing_file: Option<String> = None;
    let mut entry_mode = false;
    let mut entry_path = String::new();
    let mut first_entry = 0;

    // Create a channel to communicate between the thread and the main loop
    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();

    loop {
        if redraw {
            // Split the search string into terms
            let search_terms: Vec<String> = search.to_lowercase().split_whitespace().map(String::from).collect();

            // Filter entries based on the search terms (case-insensitive)
            filtered_entries = entries.iter()
                .filter(|entry| {
                    let name = match entry {
                        Entry::Series { name, .. } => name,
                        Entry::Episode { name, .. } => name,
                    };
                    let name_lowercase = name.to_lowercase();
                    search_terms.iter().all(|term| name_lowercase.contains(term))
                })
                .cloned()
                .collect();

            // Sort the filtered entries by name
            filtered_entries.sort_by(|a, b| {
                let name_a = match a {
                    Entry::Series { name, .. } => name,
                    Entry::Episode { name, .. } => name,
                };
                let name_b = match b {
                    Entry::Series { name, .. } => name,
                    Entry::Episode { name, .. } => name,
                };
                name_a.cmp(name_b)
            });

            // Ensure current_item is within bounds
            if current_item >= filtered_entries.len() {
                current_item = if filtered_entries.is_empty() { 0 } else { filtered_entries.len() - 1 };
            }

            draw_screen(&filtered_entries, current_item, &mut first_entry, &search, &config, entry_mode, &entry_path)?;
            redraw = false;
        }

        // Check for messages from the thread
        if let Ok(_) = rx.try_recv() {
            playing_file = None;
            redraw = true;
        }

        // Poll for events with a timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                if entry_mode {
                    match code {
                        KeyCode::Enter => {
                            // Scan the entered path for video files and insert them into the database
                            let path = Path::new(&entry_path);

                            let new_entries: Vec<_> = WalkDir::new(path)
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
                                println!("Importing files from {}...", entry_path);
                            for entry in &new_entries {
                                let location = entry.to_string_lossy().to_string();
                                let name = entry.file_name().unwrap_or_default().to_string_lossy().to_string();
                                let watched = false;

                                database::import_episode(&location, &name, watched).expect("Failed to import episode");
                            }

                            // Reload entries from the database
                            entries = database::get_entries().expect("Failed to get entries");
                            filtered_entries = entries.clone();
                            entry_mode = false;
                            redraw = true;
                        }
                        KeyCode::Esc => {
                            entry_mode = false;
                            redraw = true;
                        }
                        KeyCode::Backspace => {
                            entry_path.pop();
                            redraw = true;
                        }
                        KeyCode::Char(c) => {
                            entry_path.push(c);
                            redraw = true;
                        }
                        _ => {}
                    }
                } else {
                    match code {
                        KeyCode::Char('s') if modifiers.contains(event::KeyModifiers::CONTROL) => {
                            entry_mode = true;
                            entry_path.clear();
                            redraw = true;
                        }
                        KeyCode::Up => {
                            if current_item > 0 {
                                current_item -= 1;
                                if current_item < first_entry {
                                    first_entry = current_item;
                                }
                                redraw = true;
                            }
                        }
                        KeyCode::Down => {
                            if current_item < filtered_entries.len() - 1 {
                                current_item += 1;
                                redraw = true;
                            }
                        }
                        KeyCode::Enter => {
                            if playing_file.is_none() {
                                let selected = current_item;
                                let selected_entry = &filtered_entries[selected];
                                let file_path = match selected_entry {
                                    Entry::Episode { location, .. } => location,
                                    _ => return Ok(()),
                                };
                                let mut player_process = Some(run_video_player(&config, Path::new(file_path))?);
                                playing_file = Some(file_path.clone());

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
                            // If backspace is pressed, remove the last character from the search string
                            search.pop();
                            redraw = true;
                        }
                        _ => {
                            // If a displayable character is pressed, add it to the search string
                            if let KeyCode::Char(c) = code {
                                search.push(c);
                                redraw = true;
                            }
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

    initialize_database("videos.db").expect("Failed to initialize database");

    let entries = get_entries().expect("Failed to get entries");

    if entries.is_empty() {
        println!("No videos found, press CTRL-S to scan for files");
    } else {
        for entry in &entries {
            match entry {
                Entry::Series { id, name } => println!("Series: {} (ID: {})", name, id),
                Entry::Episode { id, name, location } => println!("Episode: {} (ID: {}, Location: {})", name, id, location),
            }
        }
    }

    initialize_terminal()?;
    let result = main_loop(entries, config);
    restore_terminal()?;
    result
}