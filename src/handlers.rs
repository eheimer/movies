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
use crate::logger;
use crate::menu::{MenuAction, MenuItem};
use crate::path_resolver::PathResolver;
use crate::player_plugin::create_player_plugin;
use crate::util::{Entry, Mode, ViewContext};
use crate::video_metadata;
use display::get_max_displayed_items_with_header_height;

pub fn handle_entry_mode(
    code: KeyCode,
    entry_path: &mut String,
    entries: &mut Vec<Entry>,
    filtered_entries: &mut Vec<Entry>,
    mode: &mut Mode,
    redraw: &mut bool,
    config: &mut Config,
    config_path: &std::path::PathBuf,
    resolver: &mut Option<PathResolver>,
    status_message: &mut String,
) {
    match code {
        KeyCode::Enter => {
            // Validate the directory exists
            let path = Path::new(&entry_path);
            if !path.exists() {
                eprintln!("Error: Directory does not exist: {}", entry_path);
                *redraw = true;
                return;
            }
            
            let canonical_path = path.canonicalize()
                .unwrap_or_else(|_| path.to_path_buf());
            
            // Check if videos.sqlite exists in that directory
            let db_path = canonical_path.join("videos.sqlite");
            let db_exists = db_path.exists();
            
            // Set status message based on whether database exists
            if db_exists {
                *status_message = format!("Connected to existing database at {}", db_path.display());
            } else {
                *status_message = "Creating new database...".to_string();
            }
            *redraw = true;
            
            // Initialize database (creates if doesn't exist, opens if exists)
            if let Err(e) = database::initialize_database(&db_path) {
                logger::log_error(&format!("Failed to initialize database at {}: {}", db_path.display(), e));
                eprintln!("\nError: Failed to initialize database: {}", e);
                
                // Check for common error types and provide specific guidance
                let error_str = e.to_string().to_lowercase();
                if error_str.contains("permission") || error_str.contains("access") {
                    eprintln!("Permission denied. Please ensure you have write permissions to this directory.");
                } else if error_str.contains("no space") || error_str.contains("disk full") {
                    eprintln!("Insufficient disk space. Please free up space and try again.");
                } else {
                    eprintln!("Please check the error details above and try again.");
                }
                
                *redraw = true;
                return;
            }
            
            // Update config with db_location and save to file
            config.set_database_path(db_path.clone());
            crate::config::save_config(config, config_path);
            
            // Create PathResolver from database path
            match PathResolver::from_database_path(&db_path) {
                Ok(new_resolver) => {
                    *resolver = Some(new_resolver);
                    
                    // Set scanning status
                    *status_message = format!("Scanning {}...", canonical_path.display());
                    *redraw = true;
                    
                    // Perform scan of the directory
                    let new_entries: Vec<_> = WalkDir::new(&canonical_path)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| e.file_type().is_file())
                        .filter(|e| {
                            e.path()
                                .extension()
                                .and_then(|ext| ext.to_str())
                                .is_some_and(|ext| {
                                    config.video_extensions.contains(&ext.to_lowercase())
                                })
                        })
                        .map(|e| e.into_path())
                        .collect();
                    
                    let mut imported_count = 0;
                    
                    for entry in &new_entries {
                        let location = entry.to_string_lossy().to_string();
                        let name = entry
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        // Use import_episode_relative with error handling for files outside root
                        if let Some(ref res) = resolver {
                            match database::import_episode_relative(&location, &name, res) {
                                Ok(true) => imported_count += 1,  // Only count if actually inserted
                                Ok(false) => {},  // Already exists, don't count
                                Err(e) => {
                                    eprintln!("Warning: Skipping file outside configured root directory: {} - {}", location, e);
                                }
                            }
                        }
                    }
                    
                    // Update status after scan
                    if db_exists {
                        if imported_count > 0 {
                            *status_message = format!("Connected to existing database. Found {} new videos", imported_count);
                        } else {
                            *status_message = format!("Connected to existing database at {}", db_path.display());
                        }
                    } else {
                        *status_message = format!("Created new database and imported {} videos", imported_count);
                    }
                    *redraw = true;

                    // Load entries and switch to Browse mode
                    *entries = database::get_entries().expect("Failed to get entries");
                    *filtered_entries = entries.clone();
                    *mode = Mode::Browse;
                    *redraw = true;
                }
                Err(e) => {
                    eprintln!("\nError: Failed to create PathResolver: {}", e);
                    
                    match e {
                        crate::path_resolver::PathResolverError::DatabaseNotFound(path) => {
                            eprintln!("Database not found at {}", path.display());
                        }
                        crate::path_resolver::PathResolverError::InvalidDatabasePath(path) => {
                            eprintln!("Invalid database path: {}", path.display());
                            eprintln!("The database path must have a valid parent directory.");
                        }
                        crate::path_resolver::PathResolverError::IoError(io_err) => {
                            eprintln!("IO error: {}", io_err);
                            
                            let error_str = io_err.to_string().to_lowercase();
                            if error_str.contains("permission") || error_str.contains("access") {
                                eprintln!("Permission denied. Please ensure you have read permissions.");
                            }
                        }
                        _ => {
                            eprintln!("Please check the error details above and try again.");
                        }
                    }
                    
                    *redraw = true;
                }
            }
        }
        KeyCode::Esc => {
            // reload entries from the database (if database is initialized)
            if resolver.is_some() {
                *entries = database::get_entries().expect("Failed to get entries");
                *filtered_entries = entries.clone();
            }
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
            logger::log_debug(&format!(
                "Edit mode: Saving episode details, dirty_fields={:?}",
                dirty_fields
            ));
            // we can only be here if the current entry is an Episode
            let episode_id = match &filtered_entries[current_item] {
                Entry::Episode { episode_id, .. } => *episode_id,
                _ => 0,
            };
            
            // Save episode details
            if let Err(e) = database::update_episode_detail(episode_id, edit_details) {
                logger::log_error(&format!("Failed to save episode details for episode {}: {}", episode_id, e));
                eprintln!("Error: Failed to save episode details: {}", e);
                return;
            }
            
            // Log metadata changes
            if !dirty_fields.is_empty() {
                let changed_fields: Vec<String> = dirty_fields.iter()
                    .map(|f| format!("{:?}", f))
                    .collect();
                logger::log_info(&format!("Saved metadata changes for episode {}: changed fields: {}", 
                    episode_id, changed_fields.join(", ")));
            }
            
            // Handle season creation if season_number is set
            if let Some(series) = &edit_details.series {
                if let Some(season_num) = season_number {
                    let season_id = database::create_season_and_assign(series.id, *season_num, episode_id)
                        .expect("Failed to create season and assign");
                    
                    // Log season assignment
                    logger::log_info(&format!("Assigned episode {} to series '{}' season {}", 
                        episode_id, series.name, season_num));
                    
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
                ViewContext::Series { series_id, .. } => {
                    database::get_entries_for_series(*series_id).unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
                ViewContext::Season { season_id, .. } => {
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
            logger::log_debug(&format!(
                "Edit mode: Canceling edit, discarding {} dirty field(s)",
                dirty_fields.len()
            ));
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
                logger::log_debug(&format!(
                    "Edit mode: Cannot increment season to {} (previous season doesn't exist), reverting to {:?}",
                    season_number.unwrap(), original_season_number
                ));
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
    filter_mode: &mut bool,
    first_series: &mut usize,
    status_message: &mut String,
) -> io::Result<bool> {
    // Check for context menu hotkeys first (F2-F5) - but not in filter mode
    // Build menu context to check if actions are available
    if !*filter_mode {
        let menu_context = crate::menu::MenuContext {
            selected_entry: filtered_entries.get(*current_item).cloned(),
            episode_detail: edit_details.clone(),
            last_action: last_action.clone(),
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
                        first_series,
                        config,
                        resolver,
                        status_message,
                    );
                    return Ok(true);
                }
            }
        }
    }
    
    match code {
        // When in filter mode, only allow filter-related keys
        KeyCode::F(1) if !*filter_mode => {
            // Open context menu
            *mode = Mode::Menu;
            *remembered_item = *current_item;
            *menu_selection = 0;
            *redraw = true;
        }
        KeyCode::Up if !*filter_mode => {
            if *current_item > 0 {
                *current_item -= 1;
                if *current_item < *first_entry {
                    *first_entry = *current_item;
                }
                *redraw = true;
            }
        }
        KeyCode::Down if !*filter_mode => {
            if *current_item < filtered_entries.len() - 1 {
                *current_item += 1;
                *redraw = true;
            }
        }
        KeyCode::Char('k') if !*filter_mode => {
            if *current_item > 0 {
                *current_item -= 1;
                if *current_item < *first_entry {
                    *first_entry = *current_item;
                }
                *redraw = true;
            }
        }
        KeyCode::Char('j') if !*filter_mode => {
            if *current_item < filtered_entries.len() - 1 {
                *current_item += 1;
                *redraw = true;
            }
        }
        KeyCode::PageUp if !*filter_mode => {
            // Use fixed header height for PageUp/PageDown
            let max_lines = get_max_displayed_items_with_header_height(4)?;
            if *current_item > *first_entry {
                *current_item = *first_entry;
            } else {
                *current_item = (*current_item).saturating_sub(max_lines);
            }
            *redraw = true;
        }
        KeyCode::PageDown if !*filter_mode => {
            // Use fixed header height for PageUp/PageDown
            let max_lines = get_max_displayed_items_with_header_height(4)?;
            if *current_item < *first_entry + max_lines - 1 {
                *current_item = *first_entry + max_lines - 1;
            } else {
                *current_item = (*current_item).saturating_add(max_lines);
            }
            *redraw = true;
        }
        KeyCode::Char('/') if !*filter_mode => {
            // Enter filter mode and set cursor to end of search string
            *filter_mode = true;
            *edit_cursor_pos = search.len();
            *redraw = true;
        }
        KeyCode::Enter if *filter_mode => {
            // Accept filter and exit filter mode
            logger::log_debug(&format!(
                "Browse mode: Accepting filter '{}', {} entries match",
                search, filtered_entries.len()
            ));
            *filter_mode = false;
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::Enter if !*filter_mode => {
            let selected = *current_item;
            let selected_entry = &filtered_entries[selected].clone();
            match selected_entry {
                Entry::Series { series_id, name } => {
                    search.clear();
                    // If a series is selected, reload the entries with the series filter
                    *current_item = 0;
                    *entries = database::get_entries_for_series(*series_id)
                        .expect("Failed to get entries for series");
                    *filtered_entries = entries.clone();
                    *view_context = ViewContext::Series { 
                        series_id: *series_id, 
                        series_name: name.clone() 
                    };
                    *redraw = true;
                }
                Entry::Episode { location, episode_id, name, .. } => {
                    // If an episode is selected, play the video
                    if playing_file.is_none() {
                        // Check if episode has length = 0 or NULL, and extract if needed
                        if edit_details.length.is_empty() || edit_details.length == "0" {
                            // Resolve relative path to absolute path for extraction
                            match database::get_episode_absolute_location(*episode_id, resolver) {
                                Ok(absolute_location) => {
                                    // Attempt to extract and update episode length (fails silently with log warning)
                                    if video_metadata::extract_and_update_episode_length(
                                        *episode_id,
                                        Path::new(&absolute_location)
                                    ).is_ok() {
                                        // Reload episode details to get updated length
                                        if let Ok(updated_details) = database::get_episode_detail(*episode_id) {
                                            *edit_details = updated_details;
                                        }
                                    }
                                }
                                Err(_e) => {
                                    // Fail silently - error already logged
                                }
                            }
                        }
                        
                        // Resolve relative path to absolute path for video playback
                        match database::get_episode_absolute_location(*episode_id, resolver) {
                            Ok(absolute_location) => {
                                // Log video playback
                                logger::log_info(&format!("Playing video: {} ({})", name, absolute_location));
                                
                                // Mark episode as unwatched when starting playback
                                if let Err(e) = database::mark_episode_unwatched(*episode_id) {
                                    logger::log_warn(&format!("Failed to mark episode {} as unwatched: {}", episode_id, e));
                                }
                                
                                // Set status message
                                *status_message = format!("Playing video: {}", name);
                                *redraw = true;
                                
                                // Create player plugin based on configured video player
                                let plugin = create_player_plugin(&config.video_player);
                                
                                // Query existing progress for resume functionality
                                let start_time = match database::get_episode_progress(*episode_id) {
                                    Ok(Some(0)) => {
                                        // Progress is explicitly 0 - start from beginning and override any watch-later file
                                        logger::log_info("Starting video from beginning (progress reset)");
                                        Some(0)
                                    }
                                    Ok(Some(progress_seconds)) if progress_seconds > 0 => {
                                        // Non-zero progress - let Celluloid handle resume from watch-later file
                                        logger::log_info(&format!("Resuming video (progress: {}s, using Celluloid's watch-later)", progress_seconds));
                                        None
                                    }
                                    Ok(Some(_)) => {
                                        // Shouldn't reach here, but handle it
                                        logger::log_info("Starting video from beginning");
                                        Some(0)
                                    }
                                    Ok(None) => {
                                        // No progress data - let Celluloid handle resume from its watch-later file
                                        logger::log_info("Starting video (no progress data)");
                                        None
                                    }
                                    Err(e) => {
                                        logger::log_warn(&format!("Failed to get progress for episode {}: {}. Starting from beginning.", episode_id, e));
                                        None
                                    }
                                };
                                
                                // Launch player using plugin
                                let (command, args) = plugin.launch_command(Path::new(&absolute_location), start_time);
                                
                                match std::process::Command::new(&command)
                                    .args(&args)
                                    .stdout(std::process::Stdio::null())
                                    .stderr(std::process::Stdio::null())
                                    .spawn()
                                {
                                    Ok(player_process) => {
                                        *playing_file = Some(location.to_string());
                                        logger::log_info(&format!("Video player launched: {} {:?}", command, args));
                                        
                                        // Spawn a thread to monitor progress and wait for player to finish
                                        let tx_clone = tx.clone();
                                        let episode_id_clone = *episode_id;
                                        let absolute_location_clone = absolute_location.clone();
                                        let watched_threshold = config.watched_threshold;
                                        let episode_duration = if !edit_details.length.is_empty() && edit_details.length != "0" {
                                            edit_details.length.parse::<u64>().unwrap_or(0)
                                        } else {
                                            0
                                        };
                                        
                                        thread::spawn(move || {
                                            use std::time::Duration;
                                            
                                            let mut player_process = player_process;
                                            let plugin = create_player_plugin(&command);
                                            
                                            // Monitor progress while player is running
                                            loop {
                                                // Check if player is still running
                                                match player_process.try_wait() {
                                                    Ok(Some(exit_status)) => {
                                                        // Player has exited
                                                        logger::log_info(&format!("Video player exited with status: {}", exit_status));
                                                        
                                                        // Give mpv a moment to write the watch-later file
                                                        thread::sleep(Duration::from_millis(500));
                                                        
                                                        // Get final position
                                                        match plugin.get_final_position(Path::new(&absolute_location_clone)) {
                                                            Ok(Some(final_position)) => {
                                                                logger::log_info(&format!("Retrieved final position: {}s", final_position));
                                                                
                                                                // Update database with final progress
                                                                if let Err(e) = crate::database::update_episode_progress(episode_id_clone, final_position) {
                                                                    logger::log_error(&format!("Failed to update progress for episode {}: {}", episode_id_clone, e));
                                                                }
                                                                
                                                                // Check if watched threshold is met
                                                                if episode_duration > 0 {
                                                                    let progress_percentage = (final_position as f64 / episode_duration as f64) * 100.0;
                                                                    let threshold = watched_threshold as f64;
                                                                    
                                                                    if progress_percentage >= threshold {
                                                                        logger::log_info(&format!(
                                                                            "Episode {} reached watched threshold ({:.1}% >= {:.1}%), marking as watched",
                                                                            episode_id_clone, progress_percentage, threshold
                                                                        ));
                                                                        
                                                                        if let Err(e) = crate::database::mark_episode_watched_with_timestamp(episode_id_clone) {
                                                                            logger::log_error(&format!("Failed to mark episode {} as watched: {}", episode_id_clone, e));
                                                                        } else {
                                                                            // Delete watch-later file so next playback starts from beginning
                                                                            if let Err(e) = plugin.delete_watch_later_file(Path::new(&absolute_location_clone)) {
                                                                                logger::log_warn(&format!("Failed to delete watch-later file: {}", e));
                                                                            }
                                                                        }
                                                                    } else {
                                                                        logger::log_info(&format!(
                                                                            "Episode {} progress: {:.1}% (threshold: {:.1}%)",
                                                                            episode_id_clone, progress_percentage, threshold
                                                                        ));
                                                                    }
                                                                }
                                                            }
                                                            Ok(None) => {
                                                                logger::log_info("No final position available from player plugin");
                                                            }
                                                            Err(e) => {
                                                                logger::log_warn(&format!("Failed to retrieve final position: {}", e));
                                                            }
                                                        }
                                                        
                                                        // Always clean up watch-later files, even if we couldn't read position
                                                        // This prevents stale/empty files from causing issues
                                                        if let Err(e) = plugin.cleanup_progress_files() {
                                                            logger::log_warn(&format!("Failed to cleanup progress files: {}", e));
                                                        }
                                                        
                                                        // Notify main thread that playback is complete
                                                        tx_clone.send(()).ok();
                                                        break;
                                                    }
                                                    Ok(None) => {
                                                        // Player is still running, just sleep and check again
                                                        // Don't read watch-later file while player is running to avoid file locking issues
                                                        thread::sleep(Duration::from_secs(10));
                                                    }
                                                    Err(e) => {
                                                        logger::log_error(&format!("Error checking player status: {}", e));
                                                        tx_clone.send(()).ok();
                                                        break;
                                                    }
                                                }
                                            }
                                        });
                                    }
                                    Err(e) => {
                                        logger::log_error(&format!("Failed to start video player for {}: {}", name, e));
                                        eprintln!("Error: Failed to start video player: {}", e);
                                        return Err(e);
                                    }
                                }
                            }
                            Err(e) => {
                                logger::log_error(&format!("Failed to resolve video path for episode {}: {}", episode_id, e));
                                eprintln!("Error resolving video path: {}", e);
                            }
                        }
                    }
                }
                Entry::Season { season_id, number } => {
                    search.clear();
                    // If a season is selected, reload the entries with the season filter
                    *current_item = 0;
                    *entries = database::get_entries_for_season(*season_id)
                        .expect("Failed to get entries for season");
                    *filtered_entries = entries.clone();
                    
                    // Get series info from current view context (we must be in a series view)
                    let series_name = match view_context {
                        ViewContext::Series { series_name, .. } => series_name.clone(),
                        _ => {
                            // Fallback: get series info from database
                            // This can happen if navigating directly to a season (e.g., after app restart)
                            let (_season, series_id_from_db) = database::get_season_by_id(*season_id)
                                .expect("Failed to get season");
                            let series = database::get_series_by_id(series_id_from_db)
                                .expect("Failed to get series");
                            series.name
                        }
                    };
                    
                    *view_context = ViewContext::Season { 
                        season_id: *season_id,
                        series_name,
                        season_number: *number
                    };
                    *redraw = true;
                }
            }
            *redraw = true;
        }
        KeyCode::Esc if *filter_mode => {
            // Cancel filter: clear search string and exit filter mode
            logger::log_debug("Browse mode: Canceling filter, clearing search");
            search.clear();
            *filter_mode = false;
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::Esc
            if !*filter_mode && !filtered_entries.is_empty() 
                && matches!(filtered_entries[*current_item], Entry::Episode { .. })
                && edit_details.season.is_some() =>
        {
            //go back to the season view
            logger::log_debug(&format!(
                "Browse mode: Navigating from season view to series view (series_id={})",
                edit_details.series.as_ref().unwrap().id
            ));
            *current_item = 0;
            search.clear();
            let series_id = edit_details.series.as_ref().unwrap().id;
            let series_name = edit_details.series.as_ref().unwrap().name.clone();
            *entries = database::get_entries_for_series(series_id)
                .expect("Failed to get entries for series");
            *filtered_entries = entries.clone();
            *view_context = ViewContext::Series { series_id, series_name };
            *redraw = true;
        }
        KeyCode::Esc
            if !*filter_mode && !filtered_entries.is_empty() 
                && (matches!(filtered_entries[*current_item], Entry::Season { .. })
                || matches!(filtered_entries[*current_item], Entry::Episode { .. })
                    && edit_details.series.is_some()) =>
        {
            logger::log_debug("Browse mode: Navigating from series/season view to top level");
            *current_item = 0;
            search.clear();
            *entries = database::get_entries().expect("Failed to get entries");
            *filtered_entries = entries.clone();
            *view_context = ViewContext::TopLevel;
            *redraw = true;
        }
        KeyCode::Esc if !*filter_mode => return Ok(false),
        KeyCode::Left if modifiers.contains(event::KeyModifiers::CONTROL) && *filter_mode => {
            // Jump back by words (separated by spaces)
            if *edit_cursor_pos > 0 {
                let mut i = *edit_cursor_pos - 1;
                while i > 0 && search.chars().nth(i - 1).unwrap() == ' ' {
                    i -= 1;
                }
                while i > 0 && search.chars().nth(i - 1).unwrap() != ' ' {
                    i -= 1;
                }
                *edit_cursor_pos = i;
                *redraw = true;
            }
        }
        KeyCode::Left if *filter_mode => {
            if *edit_cursor_pos > 0 {
                *edit_cursor_pos -= 1;
            }
            *redraw = true;
        }
        KeyCode::Right if modifiers.contains(event::KeyModifiers::CONTROL) && *filter_mode => {
            // Jump forward by words (separated by spaces)
            if *edit_cursor_pos < search.len() {
                let mut i = *edit_cursor_pos;
                while i < search.len() && search.chars().nth(i).unwrap() != ' ' {
                    i += 1;
                }
                while i < search.len() && search.chars().nth(i).unwrap() == ' ' {
                    i += 1;
                }
                *edit_cursor_pos = i;
                *redraw = true;
            }
        }
        KeyCode::Right if *filter_mode => {
            if *edit_cursor_pos < search.len() {
                *edit_cursor_pos += 1;
            }
            *redraw = true;
        }
        KeyCode::Home if *filter_mode => {
            *edit_cursor_pos = 0;
            *redraw = true;
        }
        KeyCode::End if *filter_mode => {
            *edit_cursor_pos = search.len();
            *redraw = true;
        }
        KeyCode::Backspace if *filter_mode => {
            // Remove the character BEFORE the cursor position
            if *edit_cursor_pos > 0 {
                search.remove(*edit_cursor_pos - 1);
                *edit_cursor_pos -= 1;
                *redraw = true;
            }
        }
        KeyCode::Delete if *filter_mode => {
            // Remove the character AT the cursor position
            if *edit_cursor_pos < search.len() {
                search.remove(*edit_cursor_pos);
                *redraw = true;
            }
        }
        KeyCode::Char(c) if *filter_mode => {
            // Insert character at cursor position
            search.insert(*edit_cursor_pos, c);
            *edit_cursor_pos += 1;
            *redraw = true;
        }
        _ => {}
    }
    Ok(true)
}

pub fn handle_series_select_mode(
    code: KeyCode,
    series_selection: &mut Option<usize>,
    mode: &mut Mode,
    redraw: &mut bool,
    series: &mut [Series],
    episode_id: usize,
    episode_detail: &mut EpisodeDetail,
    entries: &mut Vec<Entry>,
    filtered_entries: &mut Vec<Entry>,
    view_context: &ViewContext,
    last_action: &mut Option<crate::util::LastAction>,
    new_series: &mut String,
    edit_cursor_pos: &mut usize,
    _first_series: &mut usize,
) {
    match code {
        KeyCode::Up | KeyCode::Char('k') => {
            if !series.is_empty() {
                *series_selection = series_selection.map(|s| s.saturating_sub(1)).or(Some(0));
            } else {
                *series_selection = Some(0);
            }
            *redraw = true;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if !series.is_empty() {
                let max_index = series.len() - 1;
                *series_selection = series_selection
                    .map(|s| (s + 1).min(max_index))
                    .or(Some(0));
            } else {
                *series_selection = Some(0);
            }
            *redraw = true;
        }
        KeyCode::Enter => {
            // save the series id to the episode, then return to browse mode
            let selected_series = &series[series_selection.unwrap()];
            let series_id = selected_series.id;
            let series_name = selected_series.name.clone();
            
            *episode_detail =
                database::assign_series(series_id, episode_id).expect("Failed to assign series");
            
            // Log series assignment
            logger::log_info(&format!("Assigned episode {} to series '{}'", episode_id, series_name));
            
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
                ViewContext::Series { series_id, .. } => {
                    database::get_entries_for_series(*series_id).unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
                ViewContext::Season { season_id, .. } => {
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
    first_series: &mut usize,
) {
    match code {
        KeyCode::Enter => {
            // save the new series to the database
            *episode_detail = database::create_series_and_assign(new_series, episode_id)
                .expect("Failed to create series");

            // Log series creation
            if let Some(series) = &episode_detail.series {
                logger::log_info(&format!("Created new series '{}' and assigned episode {}", 
                    series.name, episode_id));
            }

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
                ViewContext::Series { series_id, .. } => {
                    database::get_entries_for_series(*series_id).unwrap_or_else(|_| {
                        database::get_entries().expect("Failed to get entries")
                    })
                }
                ViewContext::Season { season_id, .. } => {
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
            *first_series = 0;
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
    first_series: &mut usize,
    config: &Config,
    resolver: &PathResolver,
    status_message: &mut String,
    buffer_manager: &mut crate::buffer::BufferManager,
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
                first_series,
                config,
                resolver,
                status_message,
            );
        }
        KeyCode::Esc => {
            // Close menu and return to Browse mode
            *mode = Mode::Browse;
            // Force full redraw to clear menu artifacts from detail panel
            buffer_manager.force_full_redraw();
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
                            first_series,
                            config,
                            resolver,
                            status_message,
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
    first_series: &mut usize,
    config: &Config,
    resolver: &PathResolver,
    status_message: &mut String,
) {
    match action {
        MenuAction::Edit => {
            // Enter edit mode for the remembered episode
            if let Entry::Episode { episode_id, .. } = filtered_entries[remembered_item] {
                *mode = Mode::Edit;
                *edit_details = match database::get_episode_detail(episode_id) {
                    Ok(details) => details,
                    Err(e) => {
                        logger::log_error(&format!("Failed to get episode details for episode {}: {}", episode_id, e));
                        eprintln!("Error: Failed to load episode details: {}", e);
                        return;
                    }
                };
                *season_number = edit_details.season.as_ref().map(|season| season.number);

                // Check if episode has length = 0 or NULL, and extract if needed
                if edit_details.length.is_empty() || edit_details.length == "0" {
                    // Resolve relative path to absolute path for extraction
                    match database::get_episode_absolute_location(episode_id, resolver) {
                        Ok(absolute_location) => {
                            // Attempt to extract and update episode length (fails silently with log warning)
                            if video_metadata::extract_and_update_episode_length(
                                episode_id,
                                Path::new(&absolute_location)
                            ).is_ok() {
                                // Reload episode details to get updated length
                                if let Ok(updated_details) = database::get_episode_detail(episode_id) {
                                    *edit_details = updated_details;
                                    *season_number = edit_details.season.as_ref().map(|season| season.number);
                                }
                            }
                        }
                        Err(_e) => {
                            // Fail silently - error already logged
                        }
                    }
                }

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
            if let Entry::Episode { episode_id, location, .. } = &filtered_entries[remembered_item] {
                match database::toggle_watched_status(*episode_id) {
                    Ok(now_watched) => {
                        // Log watched status toggle
                        logger::log_info(&format!("Toggled watched status for episode {} (now: {})", episode_id, now_watched));
                        
                        // Always delete watch-later file when toggling so next playback starts from beginning
                        let absolute_location = resolver.to_absolute(Path::new(location));
                        let plugin = create_player_plugin(&config.video_player);
                        if let Err(e) = plugin.delete_watch_later_file(&absolute_location) {
                            logger::log_warn(&format!("Failed to delete watch-later file: {}", e));
                        }
                    }
                    Err(e) => {
                        logger::log_error(&format!("Failed to toggle watched status for episode {}: {}", episode_id, e));
                        eprintln!("Error: Failed to toggle watched status: {}", e);
                        return;
                    }
                }

                // Reload entries based on current view context
                *entries = match view_context {
                    ViewContext::TopLevel => database::get_entries().expect("Failed to get entries"),
                    ViewContext::Series { series_id, .. } => database::get_entries_for_series(*series_id)
                        .expect("Failed to get entries for series"),
                    ViewContext::Season { season_id, .. } => database::get_entries_for_season(*season_id)
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
                *series_selection = if series.is_empty() { None } else { Some(0) };
                *first_series = 0;
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
                        ViewContext::Series { series_id, .. } => {
                            database::get_entries_for_series(*series_id)
                                .expect("Failed to get entries for series")
                        }
                        ViewContext::Season { season_id, .. } => {
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
            // Check if db_location is None (shouldn't happen but handle gracefully)
            if config.db_location.is_none() {
                // Enter Entry mode for first-run setup
                *entries = Vec::new();
                *filtered_entries = Vec::new();
                *mode = Mode::Entry;
                *redraw = true;
            } else {
                // Get root directory from PathResolver and scan automatically
                let scan_dir = resolver.get_root_dir();
                
                // Log rescan start
                logger::log_info(&format!("Rescan started: {}", scan_dir.display()));
                
                // Set scanning status and force immediate display
                *status_message = format!("Rescanning {}...", scan_dir.display());
                
                // Force immediate terminal update to show "Rescanning..." message
                use crossterm::{cursor, terminal, ExecutableCommand};
                use std::io::{stdout, Write};
                let mut stdout = stdout();
                let _ = stdout.execute(cursor::MoveTo(0, terminal::size().unwrap_or((80, 24)).1 - 1));
                let _ = stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine));
                print!("{}", status_message);
                let _ = stdout.flush();
                
                *redraw = true;
                
                // Scan the directory for video files
                let new_entries: Vec<_> = WalkDir::new(scan_dir)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file())
                    .filter(|e| {
                        e.path()
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .is_some_and(|ext| {
                                config.video_extensions.contains(&ext.to_lowercase())
                            })
                    })
                    .map(|e| e.into_path())
                    .collect();
                
                let mut imported_count = 0;
                
                for entry in &new_entries {
                    let location = entry.to_string_lossy().to_string();
                    let name = entry
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    match database::import_episode_relative(&location, &name, resolver) {
                        Ok(true) => imported_count += 1,  // Only count if actually inserted
                        Ok(false) => {},  // Already exists, don't count
                        Err(e) => {
                            eprintln!("Warning: Skipping file: {} - {}", location, e);
                        }
                    }
                }
                
                // Update status after scan
                if imported_count > 0 {
                    *status_message = format!("Rescan complete. Found {} new videos", imported_count);
                    // Log rescan completion
                    logger::log_info(&format!("Rescan completed: imported {} new videos", imported_count));
                } else {
                    *status_message = "Rescan complete. No new videos found".to_string();
                    // Log rescan completion
                    logger::log_info("Rescan completed: no new videos found");
                }
                *redraw = true;

                // Extract missing lengths for episodes with NULL or 0 length
                match database::get_episodes_with_missing_length() {
                    Ok(episodes_to_process) => {
                        let mut extracted_count = 0;
                        let mut unsupported_extensions = std::collections::HashSet::new();
                        
                        for (episode_id, relative_location) in episodes_to_process {
                            // Convert relative path to absolute path
                            let relative_path = Path::new(&relative_location);
                            let absolute_path = resolver.to_absolute(relative_path);
                            
                            // Attempt to extract and update episode length
                            match video_metadata::extract_and_update_episode_length(episode_id, &absolute_path) {
                                Ok(_) => {
                                    extracted_count += 1;
                                }
                                Err(e) => {
                                    // Check if error is due to unsupported format
                                    let error_msg = e.to_string();
                                    if error_msg.contains("Unsupported video format:") {
                                        // Extract the extension from the error message
                                        if let Some(ext) = absolute_path.extension().and_then(|e| e.to_str()) {
                                            unsupported_extensions.insert(ext.to_lowercase());
                                        }
                                    }
                                    // Suppress warnings - don't print to stderr
                                }
                            }
                        }
                        
                        // Update status message with extraction results
                        if extracted_count > 0 || !unsupported_extensions.is_empty() {
                            let mut status_parts = vec![status_message.clone()];
                            
                            if extracted_count > 0 {
                                status_parts.push(format!("Extracted {} video lengths", extracted_count));
                            }
                            
                            if !unsupported_extensions.is_empty() {
                                let mut exts: Vec<_> = unsupported_extensions.into_iter().collect();
                                exts.sort();
                                status_parts.push(format!("{} unsupported format(s): [{}]", 
                                    exts.len(), 
                                    exts.join(", ")));
                            }
                            
                            *status_message = status_parts.join(". ");
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to query episodes with missing length: {}", e);
                    }
                }

                // Reload entries based on current view context
                *entries = match view_context {
                    ViewContext::TopLevel => database::get_entries().expect("Failed to get entries"),
                    ViewContext::Series { series_id, .. } => database::get_entries_for_series(*series_id)
                        .expect("Failed to get entries for series"),
                    ViewContext::Season { season_id, .. } => database::get_entries_for_season(*season_id)
                        .expect("Failed to get entries for season"),
                };
                *filtered_entries = entries.clone();
                *mode = Mode::Browse;
                *redraw = true;
            }
        }
        MenuAction::ClearSeriesData => {
            // Clear series, season, and episode number for the remembered episode
            if let Entry::Episode { episode_id, .. } = filtered_entries[remembered_item] {
                if let Err(e) = database::clear_series_data(episode_id) {
                    logger::log_error(&format!("Failed to clear series data for episode {}: {}", episode_id, e));
                    eprintln!("Error: Failed to clear series data: {}", e);
                    return;
                }

                // Reload entries based on current view context
                *entries = match view_context {
                    ViewContext::TopLevel => database::get_entries().expect("Failed to get entries"),
                    ViewContext::Series { series_id, .. } => database::get_entries_for_series(*series_id)
                        .expect("Failed to get entries for series"),
                    ViewContext::Season { season_id, .. } => database::get_entries_for_season(*season_id)
                        .expect("Failed to get entries for season"),
                };
                *filtered_entries = entries.clone();
                *mode = Mode::Browse;
                *redraw = true;
            }
        }
        MenuAction::UnwatchAll => {
            // Determine scope based on view_context
            match view_context {
                ViewContext::Season { season_id, .. } => {
                    database::unwatch_all_in_season(*season_id)
                        .expect("Failed to unwatch all episodes in season");
                }
                ViewContext::Series { series_id, .. } => {
                    database::unwatch_all_in_series(*series_id)
                        .expect("Failed to unwatch all episodes in series");
                }
                ViewContext::TopLevel => {
                    database::unwatch_all_standalone()
                        .expect("Failed to unwatch all standalone episodes");
                }
            }

            // Reload entries based on current view context
            *entries = match view_context {
                ViewContext::TopLevel => database::get_entries().expect("Failed to get entries"),
                ViewContext::Series { series_id, .. } => database::get_entries_for_series(*series_id)
                    .expect("Failed to get entries for series"),
                ViewContext::Season { season_id, .. } => database::get_entries_for_season(*season_id)
                    .expect("Failed to get entries for season"),
            };
            *filtered_entries = entries.clone();
            *mode = Mode::Browse;
            *redraw = true;
        }
        MenuAction::Delete => {
            // Delete the episode from the database
            if let Entry::Episode { episode_id, name, .. } = &filtered_entries[remembered_item] {
                // Delete the episode
                if let Err(e) = database::delete_episode(*episode_id) {
                    logger::log_error(&format!("Failed to delete episode {} ({}): {}", episode_id, name, e));
                    eprintln!("Error: Failed to delete episode: {}", e);
                    return;
                }
                
                // Log deletion with episode details
                logger::log_info(&format!("Deleted episode {} ({})", episode_id, name));

                // Reload entries based on current view context
                *entries = match view_context {
                    ViewContext::TopLevel => database::get_entries().expect("Failed to get entries"),
                    ViewContext::Series { series_id, .. } => database::get_entries_for_series(*series_id)
                        .expect("Failed to get entries for series"),
                    ViewContext::Season { season_id, .. } => database::get_entries_for_season(*season_id)
                        .expect("Failed to get entries for season"),
                };
                *filtered_entries = entries.clone();
                *mode = Mode::Browse;
                *redraw = true;
            }
        }
    }
}
