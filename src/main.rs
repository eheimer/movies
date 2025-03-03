mod config;
mod display;
mod database;
mod util;
mod dto;
mod terminal;
mod handlers;
mod episode_field;

use std::panic;
use config::{Config, read_config};
use database::{get_entries, initialize_database};
use dto::EpisodeDetail;
use display::draw_screen;
use episode_field::EpisodeField;
use terminal::{initialize_terminal, restore_terminal};
use util::{Entry, Mode};
use std::io;
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::Duration;
use crossterm::event::{self, Event, KeyEvent};

fn main_loop(mut entries: Vec<Entry>, config: Config) -> io::Result<()> {
    let mut current_item = 0;
    let mut redraw = true;
    let mut search: String = String::new();
    let mut filtered_entries: Vec<Entry> = entries.clone();
    let mut playing_file: Option<String> = None;
    let mut mode = Mode::Browse;
    let mut entry_path = String::new();
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
    let mut selected_entry_id: Option<i32>= None;

    // Create a channel to communicate between the thread and the main loop
    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();

    
    //if entries is empty, we will automatically load the config path
    // set entry_path to the config value, then change mode to Entry
    if entries.is_empty() {
            entry_path = config.path.clone();
            mode = Mode::Entry;
    }

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
            // filtered_entries.sort_by(|a, b| {
            //     let name_a = match a {
            //         Entry::Series { name, .. } => name,
            //         Entry::Episode { episode_number, .. } => &pad_string_as_number(episode_number,2),
            //     };
            //     let name_b = match b {
            //         Entry::Series { name, .. } => name,
            //         Entry::Episode { episode_number, .. } => &pad_string_as_number(episode_number,2),
            //     };
            //     name_a.cmp(name_b)
            // });

            // Ensure current_item is within bounds
            if current_item >= filtered_entries.len() {
                current_item = if filtered_entries.is_empty() { 0 } else { filtered_entries.len() - 1 };
            }

            //if we're in Browse mode, we need to populate edit_details before calling draw_screen
            if let Mode::Browse = mode {
                if !filtered_entries.is_empty() {
                    if let Entry::Episode { id, .. } = &filtered_entries[current_item] {
                        selected_entry_id = Some(*id);
                        if let Some(id) = selected_entry_id {
                            edit_details = database::get_episode_detail(id).expect("Failed to get entry details");
                        }
                    } else {
                        selected_entry_id = None;
                    }
                }
            }

            draw_screen(&filtered_entries, current_item, &mut first_entry, &search, &config, &mode, &entry_path, &edit_details, edit_field, edit_cursor_pos, &series, &mut series_selection, &new_series)?;
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
                match mode {
                    Mode::Entry => {
                        handlers::handle_entry_mode(code, &mut entry_path, &mut entries, &mut filtered_entries, &mut mode, &mut redraw, &config);
                    }
                    Mode::Edit => {
                        handlers::handle_edit_mode(code, modifiers, current_item, &mut filtered_entries, &mut edit_details, &mut entries, &mut mode, &mut edit_field, &mut edit_cursor_pos, &mut redraw);
                    }
                    Mode::Browse => {
                        if !handlers::handle_browse_mode(code, modifiers, &mut current_item, &mut first_entry, &mut filtered_entries, &mut entries, &mut search, &mut playing_file, &mut mode, &mut edit_details, &mut redraw, &config, &tx)? {
                            break Ok(());
                        }
                    }
                    Mode::SeriesSelect => {
                        if let Some(id) = selected_entry_id {
                            handlers::handle_series_select_mode(code, &mut series_selection, &mut mode, &mut redraw, &mut series, id, &mut edit_details, &mut entries, &mut filtered_entries);
                        } else {
                            // selected entry is a series, change mode back to browse
                            mode = Mode::Browse;
                            redraw = true;
                        }
                    }
                    Mode::SeriesCreate => {
                        if let Some(id) = selected_entry_id {
                            handlers::handle_series_create_mode(code, modifiers, &mut mode, &mut redraw, &mut new_series, &mut edit_cursor_pos, &mut series, id, &mut edit_details, &mut entries, &mut filtered_entries);
                        } else {
                            // selected entry is a series, change mode back to browse
                            mode = Mode::Browse;
                            redraw = true;
                        }
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

    let config_path = "config.json";
    let config = read_config(config_path);

    initialize_database("videos.db").expect("Failed to initialize database");

    let entries = get_entries().expect("Failed to get entries");

    initialize_terminal()?;
    let result = main_loop(entries, config);
    restore_terminal()?;
    result
}