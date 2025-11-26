mod config;
mod database;
mod display;
mod dto;
mod episode_field;
mod handlers;
mod menu;
mod path_resolver;
mod terminal;
mod util;

use config::{read_config, Config};
use crossterm::event::{self, Event, KeyEvent};
use database::{get_entries, initialize_database};
use display::draw_screen;
use dto::EpisodeDetail;
use episode_field::EpisodeField;
use path_resolver::PathResolver;
use std::collections::HashSet;
use std::env;
use std::io;
use std::panic;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use terminal::{initialize_terminal, restore_terminal};
use util::{Entry, LastAction, Mode, ViewContext};

fn main_loop(mut entries: Vec<Entry>, config: Config, resolver: PathResolver) -> io::Result<()> {
    let mut current_item = 0;
    let mut redraw = true;
    let mut search: String = String::new();
    let mut filtered_entries: Vec<Entry> = entries.clone();
    let mut playing_file: Option<String> = None;
    let mut mode = Mode::Browse;
    let mut first_entry = 0;
    let mut edit_field = EpisodeField::Title;
    let mut edit_cursor_pos: usize = 0;
    let mut edit_details = EpisodeDetail {
        title: String::new(),
        year: String::new(),
        watched: String::new(),
        length: String::new(),
        series: None,
        season: None,
        episode_number: String::new(),
    };
    let mut series = database::get_all_series().expect("Failed to get series");
    let mut series_selection: Option<usize> = None;
    let mut new_series = String::new();
    let mut selected_entry_id: Option<usize> = None;
    let mut season_number: Option<usize> = None;
    let mut view_context = ViewContext::TopLevel;
    let mut last_action: Option<LastAction> = None;
    let mut original_edit_details: Option<EpisodeDetail> = None;
    let mut dirty_fields: HashSet<EpisodeField> = HashSet::new();
    let mut menu_selection: usize = 0;
    let mut remembered_item: usize = 0;
    let mut filter_mode: bool = false;
    let mut first_series: usize = 0;

    // Create a channel to communicate between the thread and the main loop
    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();

    //if entries is empty, we will automatically load the config path
    // set entry_path to the config value, then change mode to Entry
    // Resolve the config path to an absolute path using the PathResolver
    let mut entry_path = config.get_resolved_path(&resolver)
        .to_string_lossy()
        .to_string();
    if entries.is_empty() {
        mode = Mode::Entry;
    }

    loop {
        if redraw {
            // Split the search string into terms
            let search_terms: Vec<String> = search
                .to_lowercase()
                .split_whitespace()
                .map(String::from)
                .collect();

            // Filter entries based on the search terms (case-insensitive)
            filtered_entries = entries
                .iter()
                .filter(|entry| {
                    let name = match entry {
                        Entry::Series { name, .. } => name,
                        Entry::Episode { name, .. } => name,
                        Entry::Season { number, .. } => &format!("Season {}", number),
                    };
                    let name_lowercase = name.to_lowercase();
                    search_terms
                        .iter()
                        .all(|term| name_lowercase.contains(term))
                })
                .cloned()
                .collect();

            // Ensure current_item is within bounds
            if current_item >= filtered_entries.len() {
                current_item = if filtered_entries.is_empty() {
                    0
                } else {
                    filtered_entries.len() - 1
                };
            }

            //if we're in Browse mode, we need to populate edit_details before calling draw_screen
            if let Mode::Browse = mode {
                if !filtered_entries.is_empty() {
                    if let Entry::Episode { episode_id, .. } = &filtered_entries[current_item] {
                        selected_entry_id = Some(*episode_id);
                        if let Some(id) = selected_entry_id {
                            edit_details = database::get_episode_detail(id)
                                .expect("Failed to get entry details");
                        }
                    } else {
                        selected_entry_id = None;
                    }
                }
            }

            // Get menu items for Menu mode
            let menu_items = if let Mode::Menu = mode {
                let menu_context = menu::MenuContext {
                    selected_entry: filtered_entries.get(remembered_item).cloned(),
                    episode_detail: edit_details.clone(),
                    last_action: last_action.clone(),
                };
                menu::get_context_menu_items(&menu_context)
            } else {
                Vec::new()
            };

            draw_screen(
                &filtered_entries,
                current_item,
                &mut first_entry,
                &search,
                &config,
                &mode,
                &entry_path,
                &edit_details,
                edit_field,
                edit_cursor_pos,
                &series,
                &mut series_selection,
                &new_series,
                season_number,
                &last_action,
                &dirty_fields,
                &menu_items,
                menu_selection,
                filter_mode,
                &mut first_series,
                &view_context,
            )?;
            redraw = false;
        }

        // Check for messages from the thread
        if rx.try_recv().is_ok() {
            playing_file = None;
            redraw = true;
        }

        // Poll for events with a timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match mode {
                    Mode::Entry => {
                        handlers::handle_entry_mode(
                            code,
                            &mut entry_path,
                            &mut entries,
                            &mut filtered_entries,
                            &mut mode,
                            &mut redraw,
                            &config,
                            &resolver,
                        );
                    }
                    Mode::Edit => {
                        handlers::handle_edit_mode(
                            code,
                            modifiers,
                            current_item,
                            &mut filtered_entries,
                            &mut edit_details,
                            &mut season_number,
                            &mut entries,
                            &mut mode,
                            &mut edit_field,
                            &mut edit_cursor_pos,
                            &mut redraw,
                            &view_context,
                            &mut last_action,
                            original_edit_details.as_ref().unwrap_or(&EpisodeDetail {
                                title: String::new(),
                                year: String::new(),
                                watched: String::new(),
                                length: String::new(),
                                series: None,
                                season: None,
                                episode_number: String::new(),
                            }),
                            &mut dirty_fields,
                        );
                    }
                    Mode::Browse => {
                        if !handlers::handle_browse_mode(
                            code,
                            modifiers,
                            &mut current_item,
                            &mut first_entry,
                            &mut filtered_entries,
                            &mut entries,
                            &mut search,
                            &mut playing_file,
                            &mut mode,
                            &mut edit_details,
                            &mut season_number,
                            &mut redraw,
                            &config,
                            &resolver,
                            &tx,
                            &mut view_context,
                            &mut last_action,
                            &mut edit_field,
                            &mut edit_cursor_pos,
                            &mut original_edit_details,
                            &mut dirty_fields,
                            &mut remembered_item,
                            &mut menu_selection,
                            &mut series,
                            &mut series_selection,
                            &mut filter_mode,
                            &mut first_series,
                        )? {
                            break Ok(());
                        }
                    }
                    Mode::SeriesSelect => {
                        if let Some(id) = selected_entry_id {
                            handlers::handle_series_select_mode(
                                code,
                                &mut series_selection,
                                &mut mode,
                                &mut redraw,
                                &mut series,
                                id,
                                &mut edit_details,
                                &mut entries,
                                &mut filtered_entries,
                                &view_context,
                                &mut last_action,
                                &mut new_series,
                                &mut edit_cursor_pos,
                                &mut first_series,
                            );
                        } else {
                            // selected entry is a series, change mode back to browse
                            mode = Mode::Browse;
                            redraw = true;
                        }
                    }
                    Mode::SeriesCreate => {
                        if let Some(id) = selected_entry_id {
                            handlers::handle_series_create_mode(
                                code,
                                modifiers,
                                &mut mode,
                                &mut redraw,
                                &mut new_series,
                                &mut edit_cursor_pos,
                                &mut series,
                                id,
                                &mut edit_details,
                                &mut entries,
                                &mut filtered_entries,
                                &view_context,
                                &mut last_action,
                                &mut first_series,
                            );
                        } else {
                            // selected entry is a series, change mode back to browse
                            mode = Mode::Browse;
                            redraw = true;
                        }
                    }
                    Mode::Menu => {
                        let menu_context = menu::MenuContext {
                            selected_entry: filtered_entries.get(remembered_item).cloned(),
                            episode_detail: edit_details.clone(),
                            last_action: last_action.clone(),
                        };
                        let menu_items = menu::get_context_menu_items(&menu_context);

                        handlers::handle_menu_mode(
                            code,
                            &menu_items,
                            &mut menu_selection,
                            &mut mode,
                            &mut redraw,
                            remembered_item,
                            &mut filtered_entries,
                            &mut entries,
                            &mut edit_details,
                            &mut season_number,
                            &view_context,
                            &mut last_action,
                            &mut edit_field,
                            &mut edit_cursor_pos,
                            &mut original_edit_details,
                            &mut dirty_fields,
                            &mut series,
                            &mut series_selection,
                            &mut first_series,
                        );
                    }
                }

                // Clear dirty state when exiting EDIT mode
                if !matches!(mode, Mode::Edit) {
                    if original_edit_details.is_some() {
                        original_edit_details = None;
                        dirty_fields.clear();
                    }
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    panic::set_hook(Box::new(|info| {
        restore_terminal().ok();
        eprintln!("Application crashed: {:?}", info);
    }));

    // Get executable directory to locate config.json
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
    let config_path = exe_dir.join("config.json");
    
    let config = read_config(config_path.to_str().expect("Invalid config path"));

    // Initialize PathResolver with configurable root directory from config
    let resolver = PathResolver::new(config.root_dir.as_deref())
        .expect("Failed to initialize path resolver");

    // Use the database path from PathResolver (always in executable directory)
    let db_path = resolver.get_database_path();
    initialize_database(db_path.to_str().expect("Invalid database path"))
        .expect("Failed to initialize database");

    let entries = get_entries().expect("Failed to get entries");

    initialize_terminal()?;
    let result = main_loop(entries, config, resolver);
    restore_terminal()?;
    result
}
