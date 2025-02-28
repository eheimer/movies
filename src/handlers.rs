use crossterm::event::{self, KeyCode};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread;
use walkdir::WalkDir;
use std::io;

use crate::database;
use crate::display;
use crate::dto::EpisodeDetail;
use crate::dto::Series;
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
        KeyCode::F(2) => {
            // we can only be here if the current entry is an Episode
            let episode_id = match &filtered_entries[current_item] {
                Entry::Episode { id, .. } => *id,
                _ => 0,
            };
            let _ = database::update_episode_detail(episode_id, edit_details);
            // Reload entries based on whether the episode is part of a series or not
            if let Some(series) = &edit_details.series {
                *entries = database::get_entries_for_series(series.id).expect("Failed to get entries for series");
            } else {
                *entries = database::get_entries().expect("Failed to get entries");
            }
            *filtered_entries = entries.clone();
            *mode = Mode::Browse;
            *edit_cursor_pos = 0;
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
            if *edit_field == 6 {
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
            if *edit_field == 6 {
                *edit_field = 7;
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
        KeyCode::Char('+') if *edit_field == 7 || *edit_field == 8 => {
            match *edit_field {
                //7 => edit_details.season += 1,
                8 => {
                    if let Ok(mut episode_number) = edit_details.episode_number.parse::<i32>() {
                        episode_number += 1;
                        edit_details.episode_number = episode_number.to_string();
                    } else {
                        edit_details.episode_number = "0".to_string();
                    }
                }
                _ => {}
            }
            *redraw = true;
        }
        KeyCode::Char('-') if *edit_field == 7 || *edit_field == 8 => {
            match *edit_field {
                // 7 => {
                //     if edit_details.season > 0 {
                //         edit_details.season -= 1;
                //     }
                // }
                8 => {
                    if let Ok(mut episode_number) = edit_details.episode_number.parse::<i32>() {
                        if episode_number > 0 {
                            episode_number -= 1;
                            edit_details.episode_number = episode_number.to_string();
                        }
                    } else {
                        edit_details.episode_number = "0".to_string();
                    }
                }
                _ => {}
            }
            *redraw = true;
        }
        KeyCode::Char(c) => {
            let mut allow_edit = true;
            match *edit_field {
                2 => edit_details.title.insert(*edit_cursor_pos, c),
                3 => edit_details.year.insert(*edit_cursor_pos, c),
                4 => edit_details.watched.insert(*edit_cursor_pos, c),
                5 => edit_details.length.insert(*edit_cursor_pos, c),
                //8 => edit_details.episode_number.insert(*edit_cursor_pos, c),
                _ => { allow_edit = false;}
            }
            if allow_edit {
                *edit_cursor_pos += 1;
                *redraw = true;
            }
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
        KeyCode::F(2) => {
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
        KeyCode::F(3) => {
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
        KeyCode::F(4) =>{
            // enter series select mode
            *mode = Mode::SeriesSelect;
            *redraw = true;
        }
        KeyCode::F(5) =>{
            // reload the entries back to default
            *entries = database::get_entries().expect("Failed to get entries");
            *filtered_entries = entries.clone();
            *redraw = true;
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
            let selected = *current_item;
            let selected_entry = &filtered_entries[selected].clone();
            match selected_entry {
                Entry::Series { id, .. } => {
                    // If a series is selected, reload the entries with the series filter
                    *entries = database::get_entries_for_series(*id).expect("Failed to get entries for series");
                    *filtered_entries = entries.clone();
                    *mode = Mode::Browse;
                    *redraw = true;
                }
                Entry::Episode { location, .. } => {
                    // If an episode is selected, play the video
                    if playing_file.is_none() { // only play one video at a time
                        let mut player_process = Some(run_video_player(&config, Path::new(location))?);
                        *playing_file = Some(location.to_string());

                        // Spawn a thread to wait for the process to finish
                        let tx = tx.clone();
                        thread::spawn(move || {
                            if let Some(mut process) = player_process.take() {
                                process.wait().ok();
                                tx.send(()).ok();
                            }
                        });
                    }
                }
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
    series_selection: &mut Option<usize>,
    mode: &mut Mode,
    redraw: &mut bool,
    series: &mut Vec<Series>,
    episode_id: i32,
    episode_detail: &mut EpisodeDetail,
    entries: &mut Vec<Entry>,
    filtered_entries: &mut Vec<Entry>,
) {
    match code {
        KeyCode::Up => {
            *series_selection =
                series_selection.map(|s| s.saturating_sub(1)).or(Some(0));
            *redraw = true;
        }
        KeyCode::Down => {
            *series_selection =
                series_selection.map(|s| s.saturating_add(1)).or(Some(0));
            *redraw = true;
        }
        KeyCode::Enter => {
            // save the series id to the episode, then return to browse mode
            let series_id = series[series_selection.unwrap()].id;
            *episode_detail = database::assign_series(series_id, episode_id).expect("Failed to assign series");
            // Reload entries from the database
            *entries = database::get_entries().expect("Failed to get entries");
            *filtered_entries = entries.clone();
            *mode = Mode::Browse;
            *redraw = true;
        }
        KeyCode::Char('+') => {
            // Create a new series
            *series_selection = None;
            *mode = Mode::SeriesCreate;
            *redraw = true;
        }
        KeyCode::Esc => {
            *series_selection = None;
            // Return to browse mode
            *mode = Mode::Browse;
            *redraw = true;
        }
        _ => {}
    }
}

pub fn handle_series_create_mode(
    code: KeyCode,
    modifiers: event::KeyModifiers,
    mode: &mut Mode,
    redraw: &mut bool,
    new_series: &mut String,
    edit_cursor_pos: &mut usize,
    series: &mut Vec<Series>,
    episode_id: i32,
    episode_detail: &mut EpisodeDetail,
    entries: &mut Vec<Entry>,
    filtered_entries: &mut Vec<Entry>,
) {
    match code {
        KeyCode::Enter => {
            // save the new series to the database
            *episode_detail = database::create_series_and_assign(new_series, episode_id ).expect("Failed to create series");
            // reload the series list
            *series = database::get_all_series().expect("Failed to get series");
            // Reload entries from the database
            *entries = database::get_entries().expect("Failed to get entries");
            *filtered_entries = entries.clone();
            *mode = Mode::Browse;
            *redraw = true;
        }
        KeyCode::Esc => {
            // Return to series select mode
            *new_series = String::new();
            *edit_cursor_pos = 0;
            *mode = Mode::SeriesSelect;
            *redraw = true;
        }
        KeyCode::Left if modifiers.contains(event::KeyModifiers::CONTROL) => {
            // jump back in the current field by words (separated by spaces)
            if *edit_cursor_pos > 0 {
                let mut i = *edit_cursor_pos - 1;
                while i > 0 && new_series.chars().nth(i - 1).unwrap() == ' ' {
                    i -= 1;
                }
                while i > 0 && new_series.chars().nth(i - 1).unwrap() != ' ' {
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
            if *edit_cursor_pos < new_series.len() {
                let mut i = *edit_cursor_pos;
                while i < new_series.len() && new_series.chars().nth(i).unwrap() != ' ' {
                    i += 1;
                }
                while i < new_series.len() && new_series.chars().nth(i).unwrap() == ' ' {
                    i += 1;
                }
                *edit_cursor_pos = i;
                *redraw = true;
            }
        }
        KeyCode::Right => {
            if *edit_cursor_pos < new_series.len() {
                *edit_cursor_pos += 1;
            }
            *redraw = true;
        }
        KeyCode::Home => {
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::End => {
            *edit_cursor_pos = new_series.len();
            *redraw = true;
        }
        KeyCode::Backspace => {
            // removes the character BEFORE the edit_cursor_pos as long as edit_cursor_pos is > 0, otherwise it does nothing
            if *edit_cursor_pos > 0 {
                new_series.remove(*edit_cursor_pos - 1);
                *edit_cursor_pos -= 1;
                *redraw = true;
            }
        }
        KeyCode::Delete => {
            // removes the character AT the edit_cursor_pos as long as edit_cursor_pos is < the length of the field, otherwise it does nothing
            if *edit_cursor_pos < new_series.len() {
                new_series.remove(*edit_cursor_pos);
                *redraw = true;
            }
        }
        KeyCode::Char(c) => {
            new_series.insert(*edit_cursor_pos, c);
            *edit_cursor_pos += 1;
            *redraw = true;
        }
        _ => {}
    }
}
