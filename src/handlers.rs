use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread;
use walkdir::WalkDir;
use std::io;

use crate::database;
use crate::display;
use crate::dto::EpisodeDetail;
use display::get_max_displayed_items;
use crate::util::{Entry, Mode, run_video_player};
use crate::config::Config;

pub fn handle_entry_mode(
    code: KeyCode,
    entry_path: &mut String,
    entries: &mut Vec<Entry>,
    filtered_entries: &mut Vec<Entry>,
    mode: &mut Mode,
    redraw: &mut bool,
    config: &Config,
) {
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
            display::load_videos(&entry_path, new_entries.len()).expect("Failed to load videos");
            for entry in &new_entries {
                let location = entry.to_string_lossy().to_string();
                let name = entry.file_name().unwrap_or_default().to_string_lossy().to_string();

                database::import_episode(&location, &name).expect("Failed to import episode");
            }

            // Reload entries from the database
            *entries = database::get_entries().expect("Failed to get entries");
            *filtered_entries = entries.clone();
            *mode = Mode::Browse;
            *redraw = true;
        }
        KeyCode::Esc => {
            *mode = Mode::Browse;
            *redraw = true;
        }
        KeyCode::Backspace => {
            entry_path.pop();
            *redraw = true;
        }
        KeyCode::Char(c) => {
            entry_path.push(c);
            *redraw = true;
        }
        _ => (),
    }
}

pub fn handle_edit_mode(
    code: KeyCode,
    modifiers: event::KeyModifiers,
    current_item: usize,
    filtered_entries: &mut Vec<Entry>,
    edit_details: &mut EpisodeDetail,
    entries: &mut Vec<Entry>,
    mode: &mut Mode,
    edit_field: &mut usize,
    edit_cursor_pos: &mut usize,
    redraw: &mut bool,
) {
    match code {
        KeyCode::Char('e') if modifiers.contains(event::KeyModifiers::CONTROL) => {
            // we can only be here if the current entry is an Episode
            let episode_id = match &filtered_entries[current_item] {
                Entry::Episode { id, .. } => *id,
                _ => 0,
            };
            let _ = database::update_episode_detail(episode_id, edit_details);
            *entries = database::get_entries().expect("Failed to get entries");
            *filtered_entries = entries.clone();
            *mode = Mode::Browse;
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::Char('r') if modifiers.contains(event::KeyModifiers::CONTROL) => {
            *mode = Mode::SeriesSelect;
            *redraw = true;
        }
        KeyCode::Up => {
            if *edit_field == 2 {
                *edit_field = 8;
            } else {
                *edit_field -= 1;
            }
            if *edit_field == 4 {
                *edit_field = 3;
            }
            if *edit_field == 6 || *edit_field == 7 {
                *edit_field = 5;
            }
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::Down => {
            *edit_field = (*edit_field + 1) % 9;
            if *edit_field < 2 {
                *edit_field = 2;
            }
            if *edit_field == 4 {
                *edit_field = 5;
            }
            if *edit_field == 6 || *edit_field == 7 {
                *edit_field = 8;
            }
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::Left if modifiers.contains(event::KeyModifiers::CONTROL) => {
            // jump back in the current field by words (separated by spaces)
            let field = match *edit_field {
                2 => &edit_details.title,
                3 => &edit_details.year,
                4 => &edit_details.watched,
                5 => &edit_details.length,
                8 => &edit_details.episode_number,
                _ => &String::new(),
            };
            if *edit_cursor_pos > 0 {
                let mut i = *edit_cursor_pos - 1;
                while i > 0 && field.chars().nth(i - 1).unwrap() == ' ' {
                    i -= 1;
                }
                while i > 0 && field.chars().nth(i - 1).unwrap() != ' ' {
                    i -= 1;
                }
                *edit_cursor_pos = i;
                *redraw = true;
            }
        }
        KeyCode::Left => {
            if *edit_cursor_pos > 0 {
                *edit_cursor_pos -= 1;
            }
            *redraw = true;
        }
        KeyCode::Right if modifiers.contains(event::KeyModifiers::CONTROL) => {
            // jump forward in the current field by words (separated by spaces)
            let field = match *edit_field {
                2 => &edit_details.title,
                3 => &edit_details.year,
                4 => &edit_details.watched,
                5 => &edit_details.length,
                8 => &edit_details.episode_number,
                _ => &String::new(),
            };
            if *edit_cursor_pos < field.len() {
                let mut i = *edit_cursor_pos;
                while i < field.len() && field.chars().nth(i).unwrap() != ' ' {
                    i += 1;
                }
                while i < field.len() && field.chars().nth(i).unwrap() == ' ' {
                    i += 1;
                }
                *edit_cursor_pos = i;
                *redraw = true;
            }
        }
        KeyCode::Right => {
            let field_length = match *edit_field {
                2 => edit_details.title.len(),
                3 => edit_details.year.len(),
                4 => edit_details.watched.len(),
                5 => edit_details.length.len(),
                8 => edit_details.episode_number.len(),
                _ => 0,
            };
            if *edit_cursor_pos < field_length {
                *edit_cursor_pos += 1;
            }
            *redraw = true;
        }
        KeyCode::Home => {
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::End => {
            let field_length = match *edit_field {
                2 => edit_details.title.len(),
                3 => edit_details.year.len(),
                4 => edit_details.watched.len(),
                5 => edit_details.length.len(),
                8 => edit_details.episode_number.len(),
                _ => 0,
            };
            *edit_cursor_pos = field_length;
            *redraw = true;
        }
        KeyCode::Backspace => {
            // removes the character BEFORE the edit_cursor_pos as long as edit_cursor_pos is > 0, otherwise it does nothing
            if *edit_cursor_pos > 0 {
                match *edit_field {
                    2 => { edit_details.title.remove(*edit_cursor_pos - 1); }
                    3 => { edit_details.year.remove(*edit_cursor_pos - 1); }
                    4 => { edit_details.watched.remove(*edit_cursor_pos - 1); }
                    5 => { edit_details.length.remove(*edit_cursor_pos - 1); }
                    8 => { edit_details.episode_number.remove(*edit_cursor_pos - 1); }
                    _ => {}
                }
                *edit_cursor_pos -= 1;
                *redraw = true;
            }
        }
        KeyCode::Delete => {
            // removes the character AT the edit_cursor_pos as long as edit_cursor_pos is < the length of the field, otherwise it does nothing
            let field_length = match *edit_field {
                2 => edit_details.title.len(),
                3 => edit_details.year.len(),
                4 => edit_details.watched.len(),
                5 => edit_details.length.len(),
                8 => edit_details.episode_number.len(),
                _ => 0,
            };
            if *edit_cursor_pos < field_length {
                match *edit_field {
                    2 => { edit_details.title.remove(*edit_cursor_pos); }
                    3 => { edit_details.year.remove(*edit_cursor_pos); }
                    4 => { edit_details.watched.remove(*edit_cursor_pos); }
                    5 => { edit_details.length.remove(*edit_cursor_pos); }
                    8 => { edit_details.episode_number.remove(*edit_cursor_pos); }
                    _ => {}
                }
                *redraw = true;
            }
        }
        KeyCode::Esc => {
            *mode = Mode::Browse;
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::Char(c) => {
            match *edit_field {
                2 => edit_details.title.insert(*edit_cursor_pos, c),
                3 => edit_details.year.insert(*edit_cursor_pos, c),
                4 => edit_details.watched.insert(*edit_cursor_pos, c),
                5 => edit_details.length.insert(*edit_cursor_pos, c),
                8 => edit_details.episode_number.insert(*edit_cursor_pos, c),
                _ => {}
            }
            *edit_cursor_pos += 1;
            *redraw = true;
        }
        _ => {}
    }
}

pub fn handle_browse_mode(
    code: KeyCode,
    modifiers: event::KeyModifiers,
    current_item: &mut usize,
    first_entry: &mut usize,
    filtered_entries: &mut Vec<Entry>,
    entries: &mut Vec<Entry>,
    search: &mut String,
    playing_file: &mut Option<String>,
    mode: &mut Mode,
    edit_details: &mut EpisodeDetail,
    redraw: &mut bool,
    config: &Config,
    tx: &Sender<()>,
) -> io::Result<bool> {
    match code {
        KeyCode::Char('l') if modifiers.contains(event::KeyModifiers::CONTROL) => {
            if entries.len() == 0 {
                *mode = Mode::Entry;
                search.clear();
                *redraw = true;
            }
        }
        KeyCode::Char('e') if modifiers.contains(event::KeyModifiers::CONTROL) => {
            // Enter edit mode only if the current entry is an Episode
            if let Entry::Episode { .. } = filtered_entries[*current_item] {
                *mode = Mode::Edit;
                let episode_id = match &filtered_entries[*current_item] {
                    Entry::Episode { id, .. } => *id,
                    _ => 0,
                };
                *edit_details = database::get_episode_detail(episode_id).expect("Failed to get entry details");
                *redraw = true;
            }
        }
        KeyCode::Char('w') if modifiers.contains(event::KeyModifiers::CONTROL) => {
            // Toggle the watched status of the selected entry
            if let Entry::Episode { .. } = filtered_entries[*current_item] {
                let episode_id = match &filtered_entries[*current_item] {
                    Entry::Episode { id, .. } => *id,
                    _ => 0,
                };
                database::toggle_watched_status(episode_id).expect("Failed to toggle watched status");
                *entries = database::get_entries().expect("Failed to get entries");
                *filtered_entries = entries.clone();
                *redraw = true;
            }
        }
        KeyCode::Up => {
            if *current_item > 0 {
                *current_item -= 1;
                if *current_item < *first_entry {
                    *first_entry = *current_item;
                }
                *redraw = true;
            }
        }
        KeyCode::Down => {
            if *current_item < filtered_entries.len() - 1 {
                *current_item += 1;
                *redraw = true;
            }
        }
        KeyCode::PageUp => {
            let max_lines = get_max_displayed_items()?;
            if *current_item > *first_entry {
                *current_item = *first_entry;
            } else {
                *current_item = (*current_item).saturating_sub(max_lines);
            }
            *redraw = true;
        }
        KeyCode::PageDown => {
            let max_lines = get_max_displayed_items()?;
            if *current_item < *first_entry + max_lines - 1 {
                *current_item = *first_entry + max_lines - 1;
            } else {
                *current_item = (*current_item).saturating_add(max_lines);
            }
            *redraw = true;
        }
        KeyCode::Enter => {
            if playing_file.is_none() {
                let selected = *current_item;
                let selected_entry = &filtered_entries[selected];
                let file_path = match selected_entry {
                    Entry::Episode { location, .. } => location,
                    _ => return Ok(true),
                };
                let mut player_process = Some(run_video_player(&config, Path::new(file_path))?);
                *playing_file = Some(file_path.clone());

                // Spawn a thread to wait for the process to finish
                let tx = tx.clone();
                thread::spawn(move || {
                    if let Some(mut process) = player_process.take() {
                        process.wait().ok();
                        tx.send(()).ok();
                    }
                });
            }
            *redraw = true;
        }
        KeyCode::Esc => return Ok(false),
        KeyCode::Backspace => {
            // If backspace is pressed, remove the last character from the search string
            search.pop();
            *redraw = true;
        }
        _ => {
            // If a displayable character is pressed, add it to the search string
            if let KeyCode::Char(c) = code {
                search.push(c);
                *redraw = true;
            }
        }
    }
    Ok(true)
}

pub fn handle_series_select_mode(
    code: KeyCode,
    mode: &mut Mode,
    redraw: &mut bool,
) {
    match code {
        KeyCode::Char('+') => {
            // Create a new series
            *mode = Mode::SeriesCreate;
            *redraw = true;
        }
        KeyCode::Esc => {
            // Return to video detail edit mode
            *mode = Mode::Edit;
            *redraw = true;
        }
        _ => {}
    }
}

pub fn handle_series_create_mode(
    code: KeyCode,
    mode: &mut Mode,
    redraw: &mut bool,
) {
    match code {
        KeyCode::Enter => {
            // save the new series to the database
            // reload the series list
            // return to series select mode
            *mode = Mode::SeriesSelect;
            *redraw = true;
        }
        KeyCode::Esc => {
            // Return to series select mode
            *mode = Mode::SeriesSelect;
            *redraw = true;
        }
        _ => {}
    }
}
