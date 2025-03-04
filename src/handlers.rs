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
use crate::episode_field::EpisodeField;
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
    season_number: &mut Option<usize>,
    entries: &mut Vec<Entry>,
    mode: &mut Mode,
    edit_field: &mut EpisodeField,
    edit_cursor_pos: &mut usize,
    redraw: &mut bool,
) {
    match code {
        KeyCode::F(2) => {
            // we can only be here if the current entry is an Episode
            let episode_id = match &filtered_entries[current_item] {
                Entry::Episode { episode_id, .. } => *episode_id,
                _ => 0,
            };
            let _ = database::update_episode_detail(episode_id, edit_details);
            // Reload entries based on whether the episode is part of a series or not
            if let Some(series) = &edit_details.series {
                // if season_number is not None, call database.create_season_and_assign
                if let Some(season_number) = season_number {
                    let _ = database::create_season_and_assign(series.id, *season_number, episode_id).expect("Failed to create season and assign");
                }
                *entries = database::get_entries_for_series(series.id).expect("Failed to get entries for series");
            } else {
                *entries = database::get_entries().expect("Failed to get entries");
            }
            // let's set edit_field back to the first field
            *edit_field = EpisodeField::Title;
            *filtered_entries = entries.clone();
            *mode = Mode::Browse;
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::Up => {
            loop {
                let mut field_value: usize = (*edit_field).into();
                field_value = if field_value == 0 { 8 } else { field_value - 1 };
                *edit_field = EpisodeField::from(field_value);
                if edit_field.is_editable() {
                    //special handling for season field
                    // if the series is not selected, then season should not be editable
                    if *edit_field == EpisodeField::Season && edit_details.series.is_none() {
                        continue;
                    }
                    //special handling for episode number field
                    // if the season is not selected, then episode number should not be editable
                    if *edit_field == EpisodeField::EpisodeNumber && season_number.is_none() {
                        continue;
                    }
                    break;
                }
            }
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::Down => {
            loop {
                let mut field_value: usize = (*edit_field).into();
                field_value = (field_value + 1) % 9;
                *edit_field = EpisodeField::from(field_value);
                if edit_field.is_editable() {
                    //special handling for season field
                    // if the series is not selected, then season should not be editable
                    if *edit_field == EpisodeField::Season && edit_details.series.is_none() {
                        continue;
                    }
                    //special handling for episode number field
                    // if the season is not selected, then episode number should not be editable
                    if *edit_field == EpisodeField::EpisodeNumber && season_number.is_none() {
                        continue;
                    }
                    break;
                }
            }
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::Left if modifiers.contains(event::KeyModifiers::CONTROL) => {
            // jump back in the current field by words (separated by spaces)
            let field = edit_field.get_field_value(edit_details);
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
            let field = edit_field.get_field_value(edit_details);
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
            let field_length = edit_field.get_field_value(&edit_details).len();
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
            let field_length = edit_field.get_field_value(&edit_details).len();
            *edit_cursor_pos = field_length;
            *redraw = true;
        }
        KeyCode::Backspace => {
            // removes the character BEFORE the edit_cursor_pos as long as edit_cursor_pos is > 0, otherwise it does nothing
            if *edit_cursor_pos > 0 {
                match *edit_field {
                    EpisodeField::Title => { edit_details.title.remove(*edit_cursor_pos - 1); }
                    EpisodeField::Year => { edit_details.year.remove(*edit_cursor_pos - 1); }
                    EpisodeField::Watched => { edit_details.watched.remove(*edit_cursor_pos - 1); }
                    EpisodeField::Length => { edit_details.length.remove(*edit_cursor_pos - 1); }
                    EpisodeField::EpisodeNumber => { edit_details.episode_number.remove(*edit_cursor_pos - 1); }
                    _ => {}
                }
                *edit_cursor_pos -= 1;
                *redraw = true;
            }
        }
        KeyCode::Delete => {
            // removes the character AT the edit_cursor_pos as long as edit_cursor_pos is < the length of the field, otherwise it does nothing
            let field_length = edit_field.get_field_value(edit_details).len();
            if *edit_cursor_pos < field_length {
                match *edit_field {
                    EpisodeField::Title => { edit_details.title.remove(*edit_cursor_pos); }
                    EpisodeField::Year => { edit_details.year.remove(*edit_cursor_pos); }
                    EpisodeField::Watched => { edit_details.watched.remove(*edit_cursor_pos); }
                    EpisodeField::Length => { edit_details.length.remove(*edit_cursor_pos); }
                    EpisodeField::EpisodeNumber => { edit_details.episode_number.remove(*edit_cursor_pos); }
                    _ => {}
                }
                *redraw = true;
            }
        }
        KeyCode::Esc => {
            *edit_field = EpisodeField::Title;
            *mode = Mode::Browse;
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::Char('+') if *edit_field == EpisodeField::EpisodeNumber => {
            if let Ok(mut episode_number) = edit_details.episode_number.parse::<i32>() {
                episode_number += 1;
                edit_details.episode_number = episode_number.to_string();
            } else {
                edit_details.episode_number = "0".to_string();
            }
            *redraw = true;
        }
        KeyCode::Char('+') if *edit_field == EpisodeField::Season => {
            // database::can_create_season returns a boolean indicating whether a season can be created
            // we need to increment seaons_number first, then pass it to the function
            // if the function returns false, we need to set season_number back to its original value
            let original_season_number = *season_number;
            if season_number.is_none() {
                *season_number = Some(0);
            } else {
                *season_number = Some(season_number.unwrap() + 1 as usize);
            }
            if !database::can_create_season(edit_details.series.as_ref().map(|s| s.id), season_number.unwrap()).unwrap_or(false) {
                *season_number = original_season_number;
            }
            
            *redraw = true;
        }
        KeyCode::Char('-') if *edit_field == EpisodeField::EpisodeNumber => {
            if let Ok(mut episode_number) = edit_details.episode_number.parse::<i32>() {
                if episode_number > 0 {
                    episode_number -= 1;
                    edit_details.episode_number = episode_number.to_string();
                }
            } else {
                edit_details.episode_number = "0".to_string();
            }
            *redraw = true;
        }
        KeyCode::Char('-') if *edit_field == EpisodeField::Season => {
            // the decrement will always be valid, so we don't need to check
            // but it must be >= 0, and if it was None, then it should be set to 0
            if season_number.is_none() {
                *season_number = Some(0);
            } else {
                *season_number = Some(season_number.unwrap().saturating_sub(1));
            }
            *redraw = true;
        }
        KeyCode::Char(c) => {
            let mut allow_edit = true;
            match *edit_field {
                EpisodeField::Title => edit_details.title.insert(*edit_cursor_pos, c),
                EpisodeField::Year => edit_details.year.insert(*edit_cursor_pos, c),
                EpisodeField::Watched => edit_details.watched.insert(*edit_cursor_pos, c),
                EpisodeField::Length => edit_details.length.insert(*edit_cursor_pos, c),
                EpisodeField::EpisodeNumber => edit_details.episode_number.insert(*edit_cursor_pos, c),
                _ => { allow_edit = false; }
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
    season_number: &mut Option<usize>,
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
                    Entry::Episode { episode_id, .. } => *episode_id,
                    _ => 0,
                };
                *edit_details = database::get_episode_detail(episode_id).expect("Failed to get entry details");
                *season_number = match &edit_details.season {
                    Some(season) => Some(season.number),
                    None => None,
                };
                *redraw = true;
            }
        }
        KeyCode::F(3) => {
            // Toggle the watched status of the selected entry
            if let Entry::Episode { .. } = filtered_entries[*current_item] {
                let episode_id = match &filtered_entries[*current_item] {
                    Entry::Episode { episode_id, .. } => *episode_id,
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
                Entry::Series { series_id, .. } => {
                    // If a series is selected, reload the entries with the series filter
                    *current_item = 0;
                    *entries = database::get_entries_for_series(*series_id).expect("Failed to get entries for series");
                    *filtered_entries = entries.clone();
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
                Entry::Season { season_id, .. } => {
                    // If a season is selected, reload the entries with the season filter
                    *current_item = 0;
                    *entries = database::get_entries_for_season(*season_id).expect("Failed to get entries for season");
                    *filtered_entries = entries.clone();
                    *redraw = true;
                }
            }
            *redraw = true;
        }
        KeyCode::Esc if matches!(filtered_entries[*current_item], Entry::Episode { .. }) && edit_details.season.is_some() => {
            //go back to the season view
            *current_item = 0;
            search.clear();
            *entries = database::get_entries_for_series(edit_details.series.as_ref().unwrap().id).expect("Failed to get entries for series");
            *filtered_entries = entries.clone();
            *redraw = true;
        }
        KeyCode::Esc if matches!(filtered_entries[*current_item], Entry::Season { .. }) || matches!(filtered_entries[*current_item], Entry::Episode { .. }) && edit_details.series.is_some() => {
            *current_item = 0;
            search.clear();
            *entries = database::get_entries().expect("Failed to get entries");
            *filtered_entries = entries.clone();
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
    episode_id: usize,
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
    episode_id: usize,
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
