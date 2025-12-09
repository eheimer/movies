mod components;
mod config;
mod database;
mod display;
mod dto;
mod episode_field;
mod handlers;
mod logger;
mod menu;
mod path_resolver;
mod paths;
mod scrollbar;
mod splash;
mod terminal;
mod theme;
mod util;
mod video_metadata;

use config::{read_config, save_config, Config};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use database::get_entries;
use display::draw_screen;
use dto::EpisodeDetail;
use episode_field::EpisodeField;
use path_resolver::PathResolver;
use std::collections::HashSet;
use std::io;
use std::panic;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use terminal::{initialize_terminal, restore_terminal};
use theme::Theme;
use util::{Entry, LastAction, Mode, ViewContext};
use walkdir::WalkDir;

/// Handle first-run setup flow
/// 
/// This function guides the user through initial setup when no database location is configured.
/// It prompts for a video collection directory, checks for existing database, initializes
/// the database, and performs an initial scan.
/// 
/// # Arguments
/// * `config` - Mutable reference to configuration
/// * `config_path` - Path to the config file for saving
/// 
/// # Returns
/// * `io::Result<(Vec<Entry>, PathResolver, String)>` - Loaded entries, PathResolver, and initial status message on success
fn first_run_flow(
    config: &mut Config,
    config_path: &Path,
) -> io::Result<(Vec<Entry>, PathResolver, String)> {
    let mut entry_path = String::new();
    let mut redraw = true;
    
    // Display welcome message
    println!("Welcome! Please enter the path to your video collection directory to get started.");
    println!();
    
    loop {
        if redraw {
            // Draw a simple prompt
            print!("\rVideo collection directory: {}", entry_path);
            io::Write::flush(&mut io::stdout())?;
            redraw = false;
        }
        
        // Poll for events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Enter => {
                        if entry_path.is_empty() {
                            println!("\nError: Please enter a directory path");
                            redraw = true;
                            continue;
                        }
                        
                        // Canonicalize the path
                        let path = match Path::new(&entry_path).canonicalize() {
                            Ok(p) => p,
                            Err(e) => {
                                println!("\nError: Invalid directory path: {}", e);
                                entry_path.clear();
                                redraw = true;
                                continue;
                            }
                        };
                        
                        if !path.is_dir() {
                            println!("\nError: Path is not a directory");
                            entry_path.clear();
                            redraw = true;
                            continue;
                        }
                        
                        // Check if videos.sqlite exists in the directory
                        let db_path = path.join("videos.sqlite");
                        let db_exists = db_path.exists();
                        
                        println!(); // New line after input
                        
                        if db_exists {
                            println!("Connected to existing database at {}", db_path.display());
                        } else {
                            println!("Creating new database...");
                        }
                        
                        // Initialize database
                        if let Err(e) = database::initialize_database(&db_path) {
                            println!("\nError: Failed to initialize database: {}", e);
                            
                            // Check for common error types and provide specific guidance
                            let error_str = e.to_string().to_lowercase();
                            if error_str.contains("permission") || error_str.contains("access") {
                                println!("Permission denied. Please ensure you have write permissions to this directory.");
                            } else if error_str.contains("no space") || error_str.contains("disk full") {
                                println!("Insufficient disk space. Please free up space and try again.");
                            }
                            
                            entry_path.clear();
                            redraw = true;
                            continue;
                        }
                        
                        // Update config with db_location
                        config.set_database_path(db_path.clone());
                        save_config(config, &config_path.to_path_buf());
                        
                        // Create PathResolver from database path
                        let resolver = match PathResolver::from_database_path(&db_path) {
                            Ok(r) => r,
                            Err(e) => {
                                let error_msg = e.to_string();
                                println!("\nError: Failed to create path resolver: {}", error_msg);
                                
                                match &e {
                                    path_resolver::PathResolverError::DatabaseNotFound(path) => {
                                        println!("Database not found at {}", path.display());
                                    }
                                    path_resolver::PathResolverError::InvalidDatabasePath(path) => {
                                        println!("Invalid database path: {}", path.display());
                                    }
                                    path_resolver::PathResolverError::IoError(io_err) => {
                                        println!("IO error: {}", io_err);
                                    }
                                    _ => {}
                                }
                                
                                return Err(io::Error::new(io::ErrorKind::Other, error_msg));
                            }
                        };
                        
                        // Perform initial scan
                        println!("Scanning directory for video files...");
                        let video_files: Vec<_> = WalkDir::new(&path)
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
                        
                        let mut imported_count = 0;
                        let mut skipped_count = 0;
                        
                        for video_path in &video_files {
                            let location = video_path.to_string_lossy().to_string();
                            let name = video_path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            
                            match database::import_episode_relative(&location, &name, &resolver) {
                                Ok(true) => imported_count += 1,  // Only count if actually inserted
                                Ok(false) => {},  // Already exists, don't count
                                Err(_) => skipped_count += 1,
                            }
                        }
                        
                        if db_exists {
                            println!("Connected to existing database at {}", db_path.display());
                            if imported_count > 0 {
                                println!("Found {} new videos.", imported_count);
                            }
                        } else {
                            println!("Created new database and imported {} videos", imported_count);
                        }
                        
                        if skipped_count > 0 {
                            println!("Note: {} files were skipped.", skipped_count);
                        }
                        
                        // Load entries from database
                        let entries = get_entries().expect("Failed to get entries");
                        
                        // Calculate appropriate status message based on whether DB existed and how many videos were imported
                        let status_message = if db_exists {
                            if imported_count > 0 {
                                format!("Connected to existing database. Found {} new videos", imported_count)
                            } else {
                                format!("Connected to existing database at {}", db_path.display())
                            }
                        } else {
                            format!("Created new database and imported {} videos", imported_count)
                        };
                        
                        return Ok((entries, resolver, status_message));
                    }
                    KeyCode::Esc => {
                        println!("\nSetup cancelled. Exiting...");
                        return Err(io::Error::new(io::ErrorKind::Interrupted, "Setup cancelled"));
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
            }
        }
    }
}

fn main_loop(mut entries: Vec<Entry>, mut config: Config, theme: Theme, mut resolver: Option<PathResolver>, config_path: PathBuf, mut status_message: String) -> io::Result<()> {
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

    // Entry path for manual scans (not used for first-run, which is handled separately)
    let mut entry_path = String::new();

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
                &theme,
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
                &status_message,
                resolver.as_ref().expect("PathResolver should be initialized"),
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
            let event = event::read()?;
            
            // Handle terminal resize events
            if matches!(event, Event::Resize(..)) {
                redraw = true;
                continue;
            }
            
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event
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
                            &mut config,
                            &config_path,
                            &mut resolver,
                            &mut status_message,
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
                        // If resolver is None, we need to enter Entry mode for setup
                        if resolver.is_none() {
                            mode = Mode::Entry;
                            redraw = true;
                        } else if let Some(ref res) = resolver {
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
                                res,
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
                                &mut status_message,
                            )? {
                                break Ok(());
                            }
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

                        if let Some(ref res) = resolver {
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
                                &config,
                                res,
                                &mut status_message,
                            );
                        } else {
                            // If resolver is None, exit menu and enter Entry mode
                            mode = Mode::Entry;
                            redraw = true;
                        }
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

    // Initialize application paths
    let app_paths = match paths::AppPaths::new() {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Please ensure you have write permissions to your home directory.");
            // Note: logger not initialized yet, so we can't log this error
            std::process::exit(1);
        }
    };
    
    let mut config = read_config(&app_paths.config_file);

    // Initialize logger
    // Determine log file path (custom from config or default)
    let log_file_path = if let Some(ref custom_path) = config.log_file {
        PathBuf::from(custom_path)
    } else {
        // Use default location: ~/.local/share/movies/movies.log
        let proj_dirs = directories::ProjectDirs::from("", "", "movies")
            .expect("Failed to determine application directories");
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)
            .expect("Failed to create data directory");
        data_dir.join("movies.log")
    };

    // Parse log level from config
    let log_level = config::parse_log_level(&config.log_level);
    
    // Check if the log level was invalid (will be Info if invalid)
    let was_invalid = !["error", "warn", "info", "debug"].contains(&config.log_level.to_lowercase().as_str());

    // Initialize the logger
    if let Err(e) = logger::initialize_logger(log_file_path.clone(), log_level) {
        eprintln!("Error: Failed to initialize logger: {}", e);
        eprintln!("Continuing without logging...");
        // Can't log this error since logger failed to initialize
    } else {
        // Log warning if invalid log level was provided
        if was_invalid {
            logger::log_warn(&format!(
                "Invalid log level '{}' in configuration, defaulting to 'info'",
                config.log_level
            ));
        }
        
        // Log application startup
        logger::log_info("Application started");
    }

    // Check if this is a first run (no database location configured)
    if config.is_first_run() {
        // First run - handle setup before initializing terminal
        let (entries, resolver, initial_status) = first_run_flow(&mut config, &app_paths.config_file)?;
        
        // Load theme from config directory
        let config_dir = app_paths.config_file.parent()
            .expect("Config file should have a parent directory");
        let theme_path = config_dir.join(&config.active_theme);
        logger::log_info(&format!("Loading theme from {:?}", theme_path));
        let theme = theme::load_theme(&theme_path);
        
        // Now start the main loop with the configured database
        initialize_terminal()?;
        splash::show_splash_screen()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        terminal::clear_screen()?;
        let result = main_loop(entries, config, theme, Some(resolver), app_paths.config_file.clone(), initial_status);
        restore_terminal()?;
        return result;
    }

    // Not first run - initialize database from config
    let db_path = match config.get_database_path() {
        Some(path) => path,
        None => {
            eprintln!("Error: Database location not configured");
            eprintln!("Please check your config file at: {}", app_paths.config_file.display());
            std::process::exit(1);
        }
    };

    // Check if database file exists
    if !db_path.exists() {
        eprintln!("Error: Database not found at {}", db_path.display());
        eprintln!("The database file may have been moved or deleted.");
        eprintln!("Please update your config file or delete it to run first-time setup again.");
        std::process::exit(1);
    }

    // Initialize database
    if let Err(e) = database::initialize_database(&db_path) {
        logger::log_error(&format!("Critical: Failed to initialize database at {}: {}", db_path.display(), e));
        eprintln!("Error: Failed to initialize database at {}", db_path.display());
        eprintln!("Details: {}", e);
        
        // Check for common error types and provide specific guidance
        let error_str = e.to_string().to_lowercase();
        if error_str.contains("permission") || error_str.contains("access") {
            eprintln!("This appears to be a permission error.");
            eprintln!("Please ensure you have read/write permissions to this location.");
        } else if error_str.contains("no space") || error_str.contains("disk full") {
            eprintln!("This appears to be a disk space error.");
            eprintln!("Please ensure you have sufficient disk space available.");
        } else if error_str.contains("already initialized") {
            eprintln!("Database is already initialized. This should not happen.");
        } else {
            eprintln!("Please check the error details above and ensure the path is accessible.");
        }
        
        std::process::exit(1);
    }

    // Initialize PathResolver from database location
    let resolver = match PathResolver::from_database_path(&db_path) {
        Ok(r) => r,
        Err(e) => {
            logger::log_error(&format!("Critical: Failed to initialize PathResolver from {}: {}", db_path.display(), e));
            match &e {
                path_resolver::PathResolverError::DatabaseNotFound(path) => {
                    eprintln!("Error: Database not found at {}", path.display());
                    eprintln!("The database file may have been moved or deleted.");
                }
                path_resolver::PathResolverError::InvalidDatabasePath(path) => {
                    eprintln!("Error: Invalid database path: {}", path.display());
                    eprintln!("The database path must have a valid parent directory.");
                }
                path_resolver::PathResolverError::IoError(io_err) => {
                    eprintln!("Error: Failed to access database path: {}", io_err);
                    eprintln!("Database path: {}", db_path.display());
                    
                    let error_str = io_err.to_string().to_lowercase();
                    if error_str.contains("permission") || error_str.contains("access") {
                        eprintln!("This appears to be a permission error.");
                        eprintln!("Please ensure you have read permissions to this location.");
                    }
                }
                _ => {
                    eprintln!("Error: Failed to initialize path resolver: {}", e);
                    eprintln!("Database path: {}", db_path.display());
                }
            }
            std::process::exit(1);
        }
    };

    // Load entries from database
    let entries = get_entries().expect("Failed to get entries");
    
    // Load theme from config directory
    let config_dir = app_paths.config_file.parent()
        .expect("Config file should have a parent directory");
    let theme_path = config_dir.join(&config.active_theme);
    logger::log_info(&format!("Loading theme from {:?}", theme_path));
    let theme = theme::load_theme(&theme_path);
    
    // Create empty initial status for non-first-run path
    let initial_status = String::new();

    // Start main loop
    initialize_terminal()?;
    splash::show_splash_screen()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    terminal::clear_screen()?;
    let result = main_loop(entries, config, theme, Some(resolver), app_paths.config_file, initial_status);
    restore_terminal()?;
    result
}
