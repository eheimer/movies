use crossterm::event::{self, KeyCode};
use std::collections::HashSet;
use std::io;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread;
use walkdir::WalkDir;

use crate::config::Config;
use crate::database;
use crate::display;
use crate::dto::EpisodeDetail;
use crate::dto::Series;
use crate::episode_field::EpisodeField;
use crate::menu::{MenuAction, MenuItem};
use crate::path_resolver::PathResolver;
use crate::util::{run_video_player, Entry, Mode, ViewContext};
use display::get_max_displayed_items;

pub fn handle_entry_mode(
    code: KeyCode,
    entry_path: &mut String,
    entries: &mut Vec<Entry>,
    filtered_entries: &mut Vec<Entry>,
    mode: &mut Mode,
    redraw: &mut bool,
    config: &Config,
    resolver: &PathResolver,
) {
    match code {
        KeyCode::Enter => {
            // Scan the entered path for video files and insert them into the database
            let path = Path::new(&entry_path)
                .canonicalize()
                .unwrap_or_else(|_| Path::new(&entry_path).to_path_buf());
            let new_entries: Vec<_> = WalkDir::new(&path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .filter(|e| {
                    e.path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map_or(false, |ext| {
                            config.video_extensions.contains(&ext.to_lowercase())
                        })
                })
                .map(|e| e.into_path())
                .collect();
            display::load_videos(entry_path, new_entries.len()).expect("Failed to load videos");
            
            let mut imported_count = 0;
            let mut skipped_count = 0;
            
            for entry in &new_entries {
                let location = entry.to_string_lossy().to_string();
                let name = entry
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                // Use import_episode_relative with error handling for files outside root
                match database::import_episode_relative(&location, &name, resolver) {
                    Ok(_) => imported_count += 1,
                    Err(e) => {
                        eprintln!("Warning: Skipping file outside configured root directory: {} - {}", location, e);
                        skipped_count += 1;
                    }
                }
            }
            
            if skipped_count > 0 {
                eprintln!("Imported {} files, skipped {} files outside configured root directory", imported_count, skipped_count);
            }

            // Reload entries from the database
            *entries = database::get_entries().expect("Failed to get entries");
            *filtered_entries = entries.clone();
            *mode = Mode::Browse;
            *redraw = true;
        }
        KeyCode::Esc => {
            // reload entries from the database
            *entries = database::get_entries().expect("Failed to get entries");
            *filtered_entries = entries.clone();
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

fn update_dirty_state(
    field: EpisodeField,
    current_details: &EpisodeDetail,
    original_details: &EpisodeDetail,
    dirty_fields: &mut HashSet<EpisodeField>,
    season_number: &Option<usize>,
) {
    let current_value = field.get_field_value(current_details);
    let original_value = field.get_field_value(original_details);

    // Special handling for Season field
    let is_dirty = if field == EpisodeField::Season {
        let original_season = original_details.season.as_ref().map(|s| s.number);
        season_number != &original_season
    } else {
        current_value != original_value
    };

    if is_dirty {
        dirty_fields.insert(field);
    } else {
        dirty_fields.remove(&field);
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
    view_context: &ViewContext,
    last_action: &mut Option<crate::util::LastAction>,
    original_edit_details: &EpisodeDetail,
    dirty_fields: &mut HashSet<EpisodeField>,
) {
    match code {
        KeyCode::F(2) => {
            // we can only be here if the current entry is an Episode
            let episode_id = match &filtered_entries[current_item] {
                Entry::Episode { episode_id, .. } => *episode_id,
                _ => 0,
            };
            let _ = database::update_episode_detail(episode_id, edit_details);
            
            // Handle season creation if season_number is set
            if let Some(series) = &edit_details.series {
                if let Some(season_num) = season_number {
                    let season_id = database::create_season_and_assign(series.id, *season_num, episode_id)
                        .expect("Failed to create season and assign");
                    
                    // Update last_action with the season assignment
                    *last_action = Some(crate::util::LastAction::SeasonAssignment {
                        series_id: series.id,
                        series_name: series.name.clone(),
                        season_id,
                        season_number: *season_num,
                    });
                }
            }
            
            // Reload entries based on current view context
            *entries = match view_context {
                ViewContext::TopLevel => {
                    database::get_entries().unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
                ViewContext::Series { series_id } => {
                    database::get_entries_for_series(*series_id).unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
                ViewContext::Season { season_id } => {
                    database::get_entries_for_season(*season_id).unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
            };
            // Clear dirty fields when saving
            dirty_fields.clear();
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
            let field_length = edit_field.get_field_value(edit_details).len();
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
            let field_length = edit_field.get_field_value(edit_details).len();
            *edit_cursor_pos = field_length;
            *redraw = true;
        }
        KeyCode::Backspace => {
            // removes the character BEFORE the edit_cursor_pos as long as edit_cursor_pos is > 0, otherwise it does nothing
            if *edit_cursor_pos > 0 {
                match *edit_field {
                    EpisodeField::Title => {
                        edit_details.title.remove(*edit_cursor_pos - 1);
                    }
                    EpisodeField::Year => {
                        edit_details.year.remove(*edit_cursor_pos - 1);
                    }
                    EpisodeField::Watched => {
                        edit_details.watched.remove(*edit_cursor_pos - 1);
                    }
                    EpisodeField::Length => {
                        edit_details.length.remove(*edit_cursor_pos - 1);
                    }
                    EpisodeField::EpisodeNumber => {
                        edit_details.episode_number.remove(*edit_cursor_pos - 1);
                    }
                    _ => {}
                }
                *edit_cursor_pos -= 1;
                update_dirty_state(*edit_field, edit_details, original_edit_details, dirty_fields, season_number);
                *redraw = true;
            }
        }
        KeyCode::Delete => {
            // removes the character AT the edit_cursor_pos as long as edit_cursor_pos is < the length of the field, otherwise it does nothing
            let field_length = edit_field.get_field_value(edit_details).len();
            if *edit_cursor_pos < field_length {
                match *edit_field {
                    EpisodeField::Title => {
                        edit_details.title.remove(*edit_cursor_pos);
                    }
                    EpisodeField::Year => {
                        edit_details.year.remove(*edit_cursor_pos);
                    }
                    EpisodeField::Watched => {
                        edit_details.watched.remove(*edit_cursor_pos);
                    }
                    EpisodeField::Length => {
                        edit_details.length.remove(*edit_cursor_pos);
                    }
                    EpisodeField::EpisodeNumber => {
                        edit_details.episode_number.remove(*edit_cursor_pos);
                    }
                    _ => {}
                }
                update_dirty_state(*edit_field, edit_details, original_edit_details, dirty_fields, season_number);
                *redraw = true;
            }
        }
        KeyCode::Esc => {
            // Clear dirty fields when canceling
            dirty_fields.clear();
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
            update_dirty_state(*edit_field, edit_details, original_edit_details, dirty_fields, season_number);
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
                *season_number = Some(season_number.unwrap() + 1_usize);
            }
            if !database::can_create_season(
                edit_details.series.as_ref().map(|s| s.id),
                season_number.unwrap(),
            )
            .unwrap_or(false)
            {
                *season_number = original_season_number;
            }
            update_dirty_state(*edit_field, edit_details, original_edit_details, dirty_fields, season_number);
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
            update_dirty_state(*edit_field, edit_details, original_edit_details, dirty_fields, season_number);
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
            update_dirty_state(*edit_field, edit_details, original_edit_details, dirty_fields, season_number);
            *redraw = true;
        }
        KeyCode::Char(c) => {
            let mut allow_edit = true;
            match *edit_field {
                EpisodeField::Title => edit_details.title.insert(*edit_cursor_pos, c),
                EpisodeField::Year => edit_details.year.insert(*edit_cursor_pos, c),
                EpisodeField::Watched => edit_details.watched.insert(*edit_cursor_pos, c),
                EpisodeField::Length => edit_details.length.insert(*edit_cursor_pos, c),
                EpisodeField::EpisodeNumber => {
                    edit_details.episode_number.insert(*edit_cursor_pos, c)
                }
                _ => {
                    allow_edit = false;
                }
            }
            if allow_edit {
                *edit_cursor_pos += 1;
                update_dirty_state(*edit_field, edit_details, original_edit_details, dirty_fields, season_number);
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
    resolver: &PathResolver,
    tx: &Sender<()>,
    view_context: &mut ViewContext,
    last_action: &mut Option<crate::util::LastAction>,
    edit_field: &mut EpisodeField,
    edit_cursor_pos: &mut usize,
    original_edit_details: &mut Option<EpisodeDetail>,
    dirty_fields: &mut HashSet<EpisodeField>,
    remembered_item: &mut usize,
    menu_selection: &mut usize,
    series: &mut Vec<Series>,
    series_selection: &mut Option<usize>,
) -> io::Result<bool> {
    // Check for context menu hotkeys first (F2-F5)
    // Build menu context to check if actions are available
    let menu_context = crate::menu::MenuContext {
        selected_entry: filtered_entries.get(*current_item).cloned(),
        episode_detail: edit_details.clone(),
        last_action: last_action.clone(),
        view_context: view_context.clone(),
    };
    let menu_items = crate::menu::get_context_menu_items(&menu_context);
    
    // Check if the pressed key matches any available menu item hotkey
    for item in &menu_items {
        if let Some(hotkey) = &item.hotkey {
            if *hotkey == code {
                // Execute the menu action directly
                execute_menu_action(
                    &item.action,
                    mode,
                    redraw,
                    *current_item,
                    filtered_entries,
                    entries,
                    edit_details,
                    season_number,
                    view_context,
                    last_action,
                    edit_field,
                    edit_cursor_pos,
                    original_edit_details,
                    dirty_fields,
                    series,
                    series_selection,
                );
                return Ok(true);
            }
        }
    }
    
    match code {
        KeyCode::Char('l') if modifiers.contains(event::KeyModifiers::CONTROL) => {
            // clear entries and filtered entries
            *entries = Vec::new();
            *filtered_entries = Vec::new();
            *mode = Mode::Entry;
            search.clear();
            *redraw = true;
        }
        KeyCode::F(1) => {
            // Open context menu
            *mode = Mode::Menu;
            *remembered_item = *current_item;
            *menu_selection = 0;
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
                    *entries = database::get_entries_for_series(*series_id)
                        .expect("Failed to get entries for series");
                    *filtered_entries = entries.clone();
                    *view_context = ViewContext::Series { series_id: *series_id };
                    *redraw = true;
                }
                Entry::Episode { location, episode_id, .. } => {
                    // If an episode is selected, play the video
                    if playing_file.is_none() {
                        // Resolve relative path to absolute path for video playback
                        match database::get_episode_absolute_location(*episode_id, resolver) {
                            Ok(absolute_location) => {
                                // only play one video at a time
                                let mut player_process =
                                    Some(run_video_player(config, Path::new(&absolute_location))?);
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
                            Err(e) => {
                                eprintln!("Error resolving video path: {}", e);
                            }
                        }
                    }
                }
                Entry::Season { season_id, .. } => {
                    // If a season is selected, reload the entries with the season filter
                    *current_item = 0;
                    *entries = database::get_entries_for_season(*season_id)
                        .expect("Failed to get entries for season");
                    *filtered_entries = entries.clone();
                    *view_context = ViewContext::Season { season_id: *season_id };
                    *redraw = true;
                }
            }
            *redraw = true;
        }
        KeyCode::Esc
            if matches!(filtered_entries[*current_item], Entry::Episode { .. })
                && edit_details.season.is_some() =>
        {
            //go back to the season view
            *current_item = 0;
            search.clear();
            let series_id = edit_details.series.as_ref().unwrap().id;
            *entries = database::get_entries_for_series(series_id)
                .expect("Failed to get entries for series");
            *filtered_entries = entries.clone();
            *view_context = ViewContext::Series { series_id };
            *redraw = true;
        }
        KeyCode::Esc
            if matches!(filtered_entries[*current_item], Entry::Season { .. })
                || matches!(filtered_entries[*current_item], Entry::Episode { .. })
                    && edit_details.series.is_some() =>
        {
            *current_item = 0;
            search.clear();
            *entries = database::get_entries().expect("Failed to get entries");
            *filtered_entries = entries.clone();
            *view_context = ViewContext::TopLevel;
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
    view_context: &ViewContext,
    last_action: &mut Option<crate::util::LastAction>,
    new_series: &mut String,
    edit_cursor_pos: &mut usize,
) {
    match code {
        KeyCode::Up => {
            *series_selection = series_selection.map(|s| s.saturating_sub(1)).or(Some(0));
            *redraw = true;
        }
        KeyCode::Down => {
            *series_selection = series_selection.map(|s| s.saturating_add(1)).or(Some(0));
            *redraw = true;
        }
        KeyCode::Enter => {
            // save the series id to the episode, then return to browse mode
            let selected_series = &series[series_selection.unwrap()];
            let series_id = selected_series.id;
            let series_name = selected_series.name.clone();
            
            *episode_detail =
                database::assign_series(series_id, episode_id).expect("Failed to assign series");
            
            // Update last_action with the series assignment
            *last_action = Some(crate::util::LastAction::SeriesAssignment {
                series_id,
                series_name,
            });
            
            // Reload entries based on current view context
            *entries = match view_context {
                ViewContext::TopLevel => {
                    database::get_entries().unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
                ViewContext::Series { series_id } => {
                    database::get_entries_for_series(*series_id).unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
                ViewContext::Season { season_id } => {
                    database::get_entries_for_season(*season_id).unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
            };
            *filtered_entries = entries.clone();
            *mode = Mode::Browse;
            *redraw = true;
        }
        KeyCode::Char('+') => {
            // Create a new series
            *series_selection = None;
            *new_series = String::new();
            *edit_cursor_pos = 0;
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
    view_context: &ViewContext,
    last_action: &mut Option<crate::util::LastAction>,
) {
    match code {
        KeyCode::Enter => {
            // save the new series to the database
            *episode_detail = database::create_series_and_assign(new_series, episode_id)
                .expect("Failed to create series");

            // Update last_action with the series assignment
            if let Some(series) = &episode_detail.series {
                *last_action = Some(crate::util::LastAction::SeriesAssignment {
                    series_id: series.id,
                    series_name: series.name.clone(),
                });
            }

            // reload the series list
            *series = database::get_all_series().expect("Failed to get series");
            // Reload entries based on current view context
            *entries = match view_context {
                ViewContext::TopLevel => {
                    database::get_entries().unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
                ViewContext::Series { series_id } => {
                    database::get_entries_for_series(*series_id).unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
                ViewContext::Season { season_id } => {
                    database::get_entries_for_season(*season_id).unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
            };
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

pub fn handle_menu_mode(
    code: KeyCode,
    menu_items: &[MenuItem],
    menu_selection: &mut usize,
    mode: &mut Mode,
    redraw: &mut bool,
    remembered_item: usize,
    filtered_entries: &mut Vec<Entry>,
    entries: &mut Vec<Entry>,
    edit_details: &mut EpisodeDetail,
    season_number: &mut Option<usize>,
    view_context: &ViewContext,
    last_action: &mut Option<crate::util::LastAction>,
    edit_field: &mut EpisodeField,
    edit_cursor_pos: &mut usize,
    original_edit_details: &mut Option<EpisodeDetail>,
    dirty_fields: &mut HashSet<EpisodeField>,
    series: &mut Vec<Series>,
    series_selection: &mut Option<usize>,
) {
    // Handle navigation
    match code {
        KeyCode::Up => {
            if menu_items.is_empty() {
                return;
            }
            if *menu_selection == 0 {
                *menu_selection = menu_items.len() - 1;
            } else {
                *menu_selection -= 1;
            }
            *redraw = true;
        }
        KeyCode::Down => {
            if menu_items.is_empty() {
                return;
            }
            *menu_selection = (*menu_selection + 1) % menu_items.len();
            *redraw = true;
        }
        KeyCode::Enter => {
            if menu_items.is_empty() {
                return;
            }
            // Execute the selected menu item
            let selected_action = &menu_items[*menu_selection].action;
            execute_menu_action(
                selected_action,
                mode,
                redraw,
                remembered_item,
                filtered_entries,
                entries,
                edit_details,
                season_number,
                view_context,
                last_action,
                edit_field,
                edit_cursor_pos,
                original_edit_details,
                dirty_fields,
                series,
                series_selection,
            );
        }
        KeyCode::Esc => {
            // Close menu and return to Browse mode
            *mode = Mode::Browse;
            *redraw = true;
        }
        _ => {
            // Check if the key matches any hotkey
            for (index, item) in menu_items.iter().enumerate() {
                if let Some(hotkey) = &item.hotkey {
                    if *hotkey == code {
                        // Execute this menu item
                        execute_menu_action(
                            &item.action,
                            mode,
                            redraw,
                            remembered_item,
                            filtered_entries,
                            entries,
                            edit_details,
                            season_number,
                            view_context,
                            last_action,
                            edit_field,
                            edit_cursor_pos,
                            original_edit_details,
                            dirty_fields,
                            series,
                            series_selection,
                        );
                        // Update menu selection to match the executed item
                        *menu_selection = index;
                        return;
                    }
                }
            }
        }
    }
}

fn execute_menu_action(
    action: &MenuAction,
    mode: &mut Mode,
    redraw: &mut bool,
    remembered_item: usize,
    filtered_entries: &mut Vec<Entry>,
    entries: &mut Vec<Entry>,
    edit_details: &mut EpisodeDetail,
    season_number: &mut Option<usize>,
    view_context: &ViewContext,
    last_action: &mut Option<crate::util::LastAction>,
    edit_field: &mut EpisodeField,
    edit_cursor_pos: &mut usize,
    original_edit_details: &mut Option<EpisodeDetail>,
    dirty_fields: &mut HashSet<EpisodeField>,
    series: &mut Vec<Series>,
    series_selection: &mut Option<usize>,
) {
    match action {
        MenuAction::Edit => {
            // Enter edit mode for the remembered episode
            if let Entry::Episode { episode_id, .. } = filtered_entries[remembered_item] {
                *mode = Mode::Edit;
                *edit_details = database::get_episode_detail(episode_id)
                    .expect("Failed to get entry details");
                *season_number = edit_details.season.as_ref().map(|season| season.number);

                // Initialize dirty state when entering EDIT mode
                *original_edit_details = Some(edit_details.clone());
                dirty_fields.clear();

                // Auto-fill episode number if series is assigned but episode number is not
                if edit_details.series.is_some()
                    && season_number.is_some()
                    && (edit_details.episode_number.is_empty() || edit_details.episode_number == "0")
                {
                    // Calculate next available episode number
                    let next_episode = database::get_next_available_episode_number(
                        edit_details.series.as_ref().unwrap().id,
                        *season_number,
                    )
                    .unwrap_or(1);

                    // Pre-fill the episode number
                    edit_details.episode_number = next_episode.to_string();
                    dirty_fields.insert(EpisodeField::EpisodeNumber);

                    // Set cursor to episode number field
                    *edit_field = EpisodeField::EpisodeNumber;
                    *edit_cursor_pos = edit_details.episode_number.len();
                } else {
                    // Normal behavior: start at first field
                    *edit_field = EpisodeField::Title;
                    *edit_cursor_pos = 0;
                }

                *redraw = true;
            }
        }
        MenuAction::ToggleWatched => {
            // Toggle watched status for the remembered episode
            if let Entry::Episode { episode_id, .. } = filtered_entries[remembered_item] {
                database::toggle_watched_status(episode_id)
                    .expect("Failed to toggle watched status");

                // Reload entries based on current view context
                *entries = match view_context {
                    ViewContext::TopLevel => database::get_entries().expect("Failed to get entries"),
                    ViewContext::Series { series_id } => database::get_entries_for_series(*series_id)
                        .expect("Failed to get entries for series"),
                    ViewContext::Season { season_id } => database::get_entries_for_season(*season_id)
                        .expect("Failed to get entries for season"),
                };
                *filtered_entries = entries.clone();
                *mode = Mode::Browse;
                *redraw = true;
            }
        }
        MenuAction::AssignToSeries => {
            // Enter series selection mode for the remembered episode
            if let Entry::Episode { .. } = filtered_entries[remembered_item] {
                // Reload series list
                *series = database::get_all_series().expect("Failed to get series");
                *series_selection = Some(0);
                *mode = Mode::SeriesSelect;
                *redraw = true;
            }
        }
        MenuAction::RepeatAction => {
            // Repeat the last action on the remembered episode
            if let Some(action) = last_action {
                if let Entry::Episode { episode_id, .. } = filtered_entries[remembered_item] {
                    match action {
                        crate::util::LastAction::SeriesAssignment { series_id, .. } => {
                            // Assign the episode to the series
                            let _ = database::assign_series(*series_id, episode_id);
                        }
                        crate::util::LastAction::SeasonAssignment {
                            series_id,
                            season_number: season_num,
                            ..
                        } => {
                            // Assign the episode to the series and season
                            let _ = database::create_season_and_assign(
                                *series_id,
                                *season_num,
                                episode_id,
                            );
                        }
                    }

                    // Reload entries based on current view context
                    *entries = match view_context {
                        ViewContext::TopLevel => {
                            database::get_entries().expect("Failed to get entries")
                        }
                        ViewContext::Series { series_id } => {
                            database::get_entries_for_series(*series_id)
                                .expect("Failed to get entries for series")
                        }
                        ViewContext::Season { season_id } => {
                            database::get_entries_for_season(*season_id)
                                .expect("Failed to get entries for season")
                        }
                    };
                    *filtered_entries = entries.clone();
                    *mode = Mode::Browse;
                    *redraw = true;
                }
            }
        }
        MenuAction::Rescan => {
            // This action is handled in browse mode with CTRL+L
            // For consistency, we'll close the menu and let the user use CTRL+L
            // Or we could implement it here - let's implement it here for completeness
            *entries = Vec::new();
            *filtered_entries = Vec::new();
            *mode = Mode::Entry;
            *redraw = true;
        }
    }
}
