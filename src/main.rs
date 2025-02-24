mod config;
mod display;
mod database;
mod util;

use config::{Config, read_config};
use database::{initialize_database, get_entries, EntryDetails};
use display::{draw_screen, initialize_terminal, restore_terminal};
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
    let mut edit_mode = false;
    let mut entry_path = String::new();
    let mut first_entry = 0;
    let mut edit_field = 0;
    let mut edit_cursor_pos: usize = 0;
    let mut edit_details = EntryDetails {
        title: String::new(),
        year: String::new(),
        watched: String::new(),
        length: String::new(),
        series: String::new(),
        season: String::new(),
        episode_number: String::new(),
    };

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

            draw_screen(&filtered_entries, current_item, &mut first_entry, &search, &config, entry_mode, &entry_path, edit_mode, &edit_details, edit_field, edit_cursor_pos)?;
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
                            display::load_videos(&entry_path, new_entries.len())?;
                            for entry in &new_entries {
                                let location = entry.to_string_lossy().to_string();
                                let name = entry.file_name().unwrap_or_default().to_string_lossy().to_string();

                                database::import_episode(&location, &name).expect("Failed to import episode");
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
                        _ => (),
                    }
                } else if edit_mode {
                    match code {
                        KeyCode::Char('e') if modifiers.contains(event::KeyModifiers::CONTROL) => {
                            let _ = database::update_entry_details(&filtered_entries[current_item], &edit_details);
                            entries = database::get_entries().expect("Failed to get entries");
                            filtered_entries = entries.clone();
                            edit_mode = false;
                            edit_cursor_pos = 0;
                            redraw = true;
                        }
                        KeyCode::Up  => {
                            if edit_field == 0 {
                                edit_field = 6;
                            } else {
                                edit_field -= 1;
                            }
                            edit_cursor_pos = 0;
                            redraw = true;
                        }
                        KeyCode::Down => {
                            edit_field = (edit_field + 1) % 7;
                            edit_cursor_pos = 0;
                            redraw = true;
                        }
                        KeyCode::Left if modifiers.contains(event::KeyModifiers::CONTROL) => {
                            // jump back in the current field by words (separated by spaces)
                            let field = match edit_field {
                                0 => &edit_details.title,
                                1 => &edit_details.year,
                                2 => &edit_details.watched,
                                3 => &edit_details.length,
                                4 => &edit_details.series,
                                5 => &edit_details.season,
                                6 => &edit_details.episode_number,
                                _ => &String::new(),
                            };
                            if edit_cursor_pos > 0 {
                                let mut i = edit_cursor_pos - 1;
                                while i > 0 && field.chars().nth(i - 1).unwrap() == ' ' {
                                    i -= 1;
                                }
                                while i > 0 && field.chars().nth(i - 1).unwrap() != ' ' {
                                    i -= 1;
                                }
                                edit_cursor_pos = i;
                                redraw = true;
                            }
                        }
                        KeyCode::Left => {
                            if(edit_cursor_pos > 0){
                                edit_cursor_pos -= 1;
                            }
                            redraw = true;
                        }
                        KeyCode::Right if modifiers.contains(event::KeyModifiers::CONTROL) => {
                            // jump forward in the current field by words (separated by spaces)
                            let field = match edit_field {
                                0 => &edit_details.title,
                                1 => &edit_details.year,
                                2 => &edit_details.watched,
                                3 => &edit_details.length,
                                4 => &edit_details.series,
                                5 => &edit_details.season,
                                6 => &edit_details.episode_number,
                                _ => &String::new(),
                            };
                            if edit_cursor_pos < field.len() {
                                let mut i = edit_cursor_pos;
                                while i < field.len() && field.chars().nth(i).unwrap() != ' ' {
                                    i += 1;
                                }
                                while i < field.len() && field.chars().nth(i).unwrap() == ' ' {
                                    i += 1;
                                }
                                edit_cursor_pos = i;
                                redraw = true;
                            }
                        }
                        KeyCode::Right => {
                            let field_length = match edit_field {
                                0 => edit_details.title.len(),
                                1 => edit_details.year.len(),
                                2 => edit_details.watched.len(),
                                3 => edit_details.length.len(),
                                4 => edit_details.series.len(),
                                5 => edit_details.season.len(),
                                6 => edit_details.episode_number.len(),
                                _ => 0,
                            };
                            if edit_cursor_pos < field_length {
                                edit_cursor_pos += 1;
                            }
                            redraw = true;
                        }
                        KeyCode::Home => {
                            edit_cursor_pos = 0;
                            redraw = true;
                        }
                        KeyCode::End => {
                            let field_length = match edit_field {
                                0 => edit_details.title.len(),
                                1 => edit_details.year.len(),
                                2 => edit_details.watched.len(),
                                3 => edit_details.length.len(),
                                4 => edit_details.series.len(),
                                5 => edit_details.season.len(),
                                6 => edit_details.episode_number.len(),
                                _ => 0,
                            };
                            edit_cursor_pos = field_length;
                            redraw = true;
                        }
                        KeyCode::Backspace => {
                            // removes the character BEFORE the edit_cursor_pos as long as edit_cursor_pos is > 0, otherwise it does nothing
                            if edit_cursor_pos > 0 {
                                match edit_field {
                                    0 => { edit_details.title.remove(edit_cursor_pos - 1); }
                                    1 => { edit_details.year.remove(edit_cursor_pos - 1); }
                                    2 => { edit_details.watched.remove(edit_cursor_pos - 1); }
                                    3 => { edit_details.length.remove(edit_cursor_pos - 1); }
                                    4 => { edit_details.series.remove(edit_cursor_pos - 1); }
                                    5 => { edit_details.season.remove(edit_cursor_pos - 1); }
                                    6 => { edit_details.episode_number.remove(edit_cursor_pos - 1); }
                                    _ => {}
                                }
                                edit_cursor_pos -= 1;
                                redraw = true;
                            }
                        }
                        KeyCode::Delete => {
                            // removes the character AT the edit_cursor_pos as long as edit_cursor_pos is < the length of the field, otherwise it does nothing
                            let field_length = match edit_field {
                                0 => edit_details.title.len(),
                                1 => edit_details.year.len(),
                                2 => edit_details.watched.len(),
                                3 => edit_details.length.len(),
                                4 => edit_details.series.len(),
                                5 => edit_details.season.len(),
                                6 => edit_details.episode_number.len(),
                                _ => 0,
                            };
                            if edit_cursor_pos < field_length {
                                match edit_field {
                                    0 => { edit_details.title.remove(edit_cursor_pos); }
                                    1 => { edit_details.year.remove(edit_cursor_pos); }
                                    2 => { edit_details.watched.remove(edit_cursor_pos); }
                                    3 => { edit_details.length.remove(edit_cursor_pos); }
                                    4 => { edit_details.series.remove(edit_cursor_pos); }
                                    5 => { edit_details.season.remove(edit_cursor_pos); }
                                    6 => { edit_details.episode_number.remove(edit_cursor_pos); }
                                    _ => {}
                                }
                                redraw = true;
                            }
                        }
                        KeyCode::Esc => {
                            edit_mode = false;
                            edit_cursor_pos = 0;
                            redraw = true;
                        }
                        KeyCode::Char(c) => {
                            match edit_field {
                                0 => edit_details.title.insert(edit_cursor_pos, c),
                                1 => edit_details.year.insert(edit_cursor_pos, c),
                                2 => edit_details.watched.insert(edit_cursor_pos, c),
                                3 => edit_details.length.insert(edit_cursor_pos, c),
                                4 => edit_details.series.insert(edit_cursor_pos, c),
                                5 => edit_details.season.insert(edit_cursor_pos, c),
                                6 => edit_details.episode_number.insert(edit_cursor_pos, c),
                                _ => {}
                            }
                            edit_cursor_pos += 1;
                            redraw = true;
                        }
                        _ => {}
                    }
                } else {
                    match code {
                        KeyCode::Char('l') if modifiers.contains(event::KeyModifiers::CONTROL) => {
                            if entries.len() == 0{
                                entry_mode = true;
                                entry_path.clear();
                                redraw = true;
                            }
                        }
                        KeyCode::Char('e') if modifiers.contains(event::KeyModifiers::CONTROL) => {
                            // Enter edit mode
                            edit_mode = true;
                            edit_details = database::get_entry_details(&filtered_entries[current_item]).expect("Failed to get entry details");
                            edit_field = 0;
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
                        KeyCode::PageUp => {
                            display::page_up(&mut current_item, &mut first_entry)?;
                            redraw = true;
                        }
                        KeyCode::PageDown => {
                            display::page_down(&mut current_item, &mut first_entry)?;
                            redraw = true;
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