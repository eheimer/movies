use crate::components::{Component, category::{Category, CategoryType}, Browser, DetailPanel, StatusBar, ContextMenu, SeriesSelectWindow};
use crate::components::episode::Episode;
use crate::components::header::{Header, HeaderContext};
use crate::dto::{EpisodeDetail, Series};
use crate::episode_field::EpisodeField;
use crate::menu::MenuItem;
use crate::terminal::{
    get_terminal_size, hide_cursor, move_cursor, show_cursor,
};
use crate::theme::Theme;
use crate::util::{Entry, LastAction, Mode, ViewContext};


use std::collections::HashSet;
use std::convert::From;
use std::io;


const FOOTER_SIZE: usize = 1; // Reserve 1 line for status line at bottom
const COL1_WIDTH: usize = 45;
const MIN_COL2_WIDTH: usize = 20;
const DETAIL_HEIGHT: usize = 13; // Increased from 11 to accommodate progress tracking fields

/// Convert Entry objects to Browser component data
fn entries_to_browser_data(
    entries: &[Entry],
    edit_details: &EpisodeDetail,
    resolver: &crate::path_resolver::PathResolver,
) -> (Vec<Category>, Vec<Episode>) {
    let mut categories = Vec::new();
    let mut episodes = Vec::new();
    
    for entry in entries {
        match entry {
            Entry::Series { name, series_id } => {
                // Get episode counts from database
                let (total, unwatched) = crate::database::get_series_episode_counts(*series_id)
                    .unwrap_or_else(|e| {
                        crate::logger::log_warn(&format!("Failed to get episode counts for series '{}' (id: {}): {}", name, series_id, e));
                        (0, 0)
                    });
                let watched = total.saturating_sub(unwatched);
                
                // Create Category component with brackets around series name
                let category = Category::new(
                    format!("[{}]", name),
                    total,
                    watched,
                    CategoryType::Series,
                );
                categories.push(category);
            }
            Entry::Season { number, season_id } => {
                // Get episode counts from database
                let (total, unwatched) = crate::database::get_season_episode_counts(*season_id)
                    .unwrap_or_else(|e| {
                        crate::logger::log_warn(&format!("Failed to get episode counts for season {} (id: {}): {}", number, season_id, e));
                        (0, 0)
                    });
                let watched = total.saturating_sub(unwatched);
                
                // Create Category component
                let category = Category::new(
                    format!("Season {}", number),
                    total,
                    watched,
                    CategoryType::Season,
                );
                categories.push(category);
            }
            Entry::Episode { episode_id, name, location, .. } => {
                // Fetch episode details for this specific episode
                let episode_detail = crate::database::get_episode_detail(*episode_id)
                    .unwrap_or_else(|_| edit_details.clone());
                
                // Check individual conditions for combined state handling
                let absolute_path = resolver.to_absolute(std::path::Path::new(location));
                let file_exists = absolute_path.exists();
                let filename = location.rsplit('/').next().unwrap_or("");
                let is_new = episode_detail.title == filename;
                let is_watched = episode_detail.watched == "true";
                
                // Create Episode component
                let episode_component = Episode::new(
                    name.clone(),
                    is_watched,
                    file_exists,
                    is_new,
                );
                episodes.push(episode_component);
            }
        }
    }
    
    (categories, episodes)
}

fn get_sidebar_width() -> io::Result<usize> {
    let (cols, _) = get_terminal_size()?;
    let sidebar_width = cols.saturating_sub(COL1_WIDTH + 2);
    Ok(sidebar_width.max(MIN_COL2_WIDTH))
}

fn draw_detail_panel_border_to_buffer(
    writer: &mut crate::buffer::BufferWriter,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    thick: bool,
) {
    // Choose border characters based on the thickness
    let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) = if thick {
        ('╔', '╗', '╚', '╝', '═', '║')
    } else {
        ('┌', '┐', '└', '┘', '─', '│')
    };

    // Draw top border
    writer.move_to(left, top);
    writer.write_char(top_left);
    for _ in 1..width.saturating_sub(1) {
        writer.write_char(horizontal);
    }
    writer.write_char(top_right);

    // Draw side borders and clear interior
    for row in (top + 1)..(top + height.saturating_sub(1)) {
        writer.move_to(left, row);
        writer.write_char(vertical);
        // Clear the interior space
        for _ in 1..width.saturating_sub(1) {
            writer.write_char(' ');
        }
        writer.write_char(vertical);
    }

    // Draw bottom border
    writer.move_to(left, top + height.saturating_sub(1));
    writer.write_char(bottom_left);
    for _ in 1..width.saturating_sub(1) {
        writer.write_char(horizontal);
    }
    writer.write_char(bottom_right);
}











/// Main screen rendering function with double-buffer integration.
///
/// This function renders the entire UI by writing to a buffer instead of directly to the terminal.
/// The buffer layer automatically handles differential updates, writing only changed cells.
///
/// # Buffer Integration
///
/// The function follows this pattern:
/// 1. Clear desired buffer to start with empty slate
/// 2. Get BufferWriter for this frame
/// 3. Write all UI components to buffer
/// 4. Compare buffers and write only changes to terminal
///
/// # Parameters
///
/// - `buffer_manager`: The BufferManager instance (added for double-buffer support)
/// - All other parameters remain unchanged from the original implementation
///
/// # Integration Notes
///
/// - This is the ONLY function signature that changed (added one parameter)
/// - Component rendering logic remains unchanged
/// - Direct terminal writes replaced with BufferWriter calls
/// - Cursor visibility still handled via direct terminal calls
pub fn draw_screen(
    entries: &[Entry],
    current_item: usize,
    first_entry: &mut usize,
    filter: &str,
    theme: &Theme,
    mode: &Mode,
    entry_path: &String,
    edit_details: &EpisodeDetail,
    edit_field: EpisodeField,
    edit_cursor_pos: usize,
    series: &[Series],
    series_selection: &mut Option<usize>,
    new_series: &str,
    season_number: Option<usize>,
    last_action: &Option<LastAction>,
    dirty_fields: &HashSet<EpisodeField>,
    menu_items: &[MenuItem],
    menu_selection: usize,
    filter_mode: bool,
    first_series: &mut usize,
    view_context: &ViewContext,
    status_message: &str,
    resolver: &crate::path_resolver::PathResolver,
    buffer_manager: &mut crate::buffer::BufferManager,
) -> io::Result<()> {
    // Clear desired buffer to start with empty slate
    buffer_manager.clear_desired_buffer();
    
    // Get writer for this frame
    let mut writer = buffer_manager.get_writer();

    hide_cursor()?;

    //browse_series is true if the mode is browse and the current item in entries is a series
    let series_selected = matches!(mode, Mode::Browse)
        && matches!(entries.get(current_item), Some(Entry::Series { .. }));
    let season_selected = matches!(mode, Mode::Browse)
        && matches!(entries.get(current_item), Some(Entry::Season { .. }));

    //series_filter is true if the mode is browse and the current item in entries is an episode and the series field is not empty
    let _series_filter_active = matches!(mode, Mode::Browse)
        && matches!(entries.get(current_item), Some(Entry::Episode { .. }))
        && edit_details.series.is_some();

    // Calculate is_dirty from dirty_fields
    let is_dirty = !dirty_fields.is_empty();

    // Extract selected entry for header
    let selected_entry = entries.get(current_item);

    // Determine if we're in first-run state (Entry mode with no entries)
    let is_first_run = matches!(mode, Mode::Entry) && entries.is_empty();

    // Get terminal width for header
    let (terminal_width, _) = get_terminal_size()?;

    // Create HeaderContext with all required data
    let header_context = HeaderContext::new(
        mode.clone(),
        filter_mode,
        is_dirty,
        is_first_run,
        terminal_width,
        selected_entry.cloned(),
        edit_details.clone(),
        last_action.clone(),
        view_context.clone(),
        filter.to_owned(),
        filter_mode, // filter_focused is same as filter_mode for now
    );

    // Create and render Header component
    let header = Header::new(&header_context);
    let header_height = header.calculate_height();
    let header_cells = header.render(terminal_width, header_height, theme, false);

    // Write header cells to buffer
    write_cells_to_buffer(&mut writer, &header_cells, 0, 0);

    // Handle Entry mode display (both first-run and rescan scenarios)
    if let Mode::Entry = mode {
        if entries.is_empty() {
            // First-run scenario - show welcome message with detailed instructions
            writer.move_to(0, header_height);
            writer.write_str("Welcome to the video library manager!");
            writer.move_to(0, header_height + 2);
            writer.write_str("To get started, enter the full path to your video collection directory below.");
            writer.move_to(0, header_height + 4);
            writer.write_str("What happens next:");
            writer.move_to(0, header_height + 5);
            writer.write_str("  • If videos.sqlite exists in that directory, it will be used (preserving your data)");
            writer.move_to(0, header_height + 6);
            writer.write_str("  • If not, a new database will be created and your videos will be scanned");
            writer.move_to(0, header_height + 8);
            writer.write_str(&format!("Path: {}", entry_path));
        } else {
            // Rescan scenario - show simpler prompt
            writer.move_to(0, header_height + 1);
            writer.write_str("Enter the path to a directory to scan for video files.");
            writer.move_to(0, header_height + 3);
            writer.write_str(&format!("Path: {}", entry_path));
        }
    } else if !entries.is_empty() {
        let max_lines = get_max_displayed_items_with_header_height(header_height)?;

        //make sure current_item is between first_entry and first_entry + max_lines.  If it's not, adjust first_entry
        if current_item < *first_entry {
            *first_entry = current_item;
        } else if current_item >= *first_entry + max_lines as usize {
            *first_entry = current_item - max_lines as usize + 1;
        }

        // Convert entries to Browser component data
        let (categories, episodes) = entries_to_browser_data(entries, edit_details, resolver);
        
        // Create Browser component
        let mut browser = Browser::new(
            (0, header_height),  // top_left position
            COL1_WIDTH,        // width
            categories,
            episodes,
        );
        
        // Set the current selection and first visible item
        browser.set_selected_item(current_item);
        browser.first_visible_item = *first_entry;
        
        // Ensure selection is visible and bounds are correct
        browser.ensure_selection_visible(max_lines);
        
        // Update first_entry to match browser's scroll position
        *first_entry = browser.first_visible_item;
        
        // Render the browser component
        let browser_cells = browser.render(COL1_WIDTH, max_lines, theme, true);
        
        // Write browser cells to buffer
        write_cells_to_buffer(&mut writer, &browser_cells, 0, header_height);
        if !series_selected && !season_selected && !matches!(mode, Mode::Menu) {
            // Extract location from current entry
            let entry_location = match &entries[current_item] {
                Entry::Episode { location, .. } => location.clone(),
                _ => String::new(),
            };
            
            // Calculate detail panel position and dimensions
            let start_col: usize = COL1_WIDTH + 2;
            let start_row = header_height;
            let sidebar_width = get_sidebar_width()?;
            let edit_mode = matches!(mode, Mode::Edit);
            
            // Show or hide the cursor based on edit_mode
            if edit_mode {
                show_cursor()?;
            }
            
            // Draw the window border for detail panel
            draw_detail_panel_border_to_buffer(
                &mut writer,
                start_col,
                start_row,
                sidebar_width,
                DETAIL_HEIGHT,
                edit_mode,
            );
            
            // Create and render DetailPanel component
            let detail_panel = DetailPanel::new(
                mode.clone(),
                edit_details.clone(),
                edit_field,
                edit_cursor_pos,
                season_number,
                dirty_fields.clone(),
                entry_location,
            );
            
            // Calculate content area (inside the border)
            let content_width = sidebar_width.saturating_sub(2); // Subtract left and right borders
            let content_height = DETAIL_HEIGHT.saturating_sub(2); // Subtract top and bottom borders
            
            // Render the DetailPanel component
            let detail_cells = detail_panel.render(content_width, content_height, theme, false);
            
            // Write detail panel cells to buffer (inside the border)
            write_cells_to_buffer(&mut writer, &detail_cells, start_col + 1, start_row + 1);
            
            // Position cursor for Edit mode
            if edit_mode && edit_field.is_editable() {
                let edit_cursor_min = edit_field.display_name().len() + 2;
                move_cursor(
                    start_col + 1 + edit_cursor_min + edit_cursor_pos,
                    start_row + 1 + usize::from(edit_field),
                )?;
            }
        }
        if let Mode::SeriesSelect | Mode::SeriesCreate = mode {
            // Calculate window dimensions based on series count and mode
            let (window_width, window_height) = SeriesSelectWindow::calculate_dimensions(
                series.len(),
                header_height,
                mode,
            )?;
            
            // Calculate window position (centered horizontally in sidebar)
            let (window_x, window_y) = SeriesSelectWindow::calculate_horizontal_position(
                window_width,
                header_height,
            )?;
            
            // Create SeriesSelectWindow component
            let mut series_window = SeriesSelectWindow::new(
                mode.clone(),
                series.to_owned(),
                *series_selection,
                new_series.to_owned(),
                edit_cursor_pos,
                *first_series,
                window_width,
                window_height,
            );
            
            // Handle edge cases for small terminals
            let (terminal_width, terminal_height) = get_terminal_size()?;
            series_window.handle_edge_cases(
                terminal_width,
                terminal_height,
                header_height,
            )?;
            
            // Render the SeriesSelectWindow component
            let series_cells = series_window.render(window_width, window_height, theme, false);
            
            // Write series window cells to buffer
            write_cells_to_buffer(&mut writer, &series_cells, window_x, window_y);
            
            // Handle cursor positioning for SeriesCreate mode
            if let Mode::SeriesCreate = mode {
                use crate::terminal::{show_cursor, move_cursor};
                show_cursor()?;
                // Position cursor in the text input field (row 2, after the prompt)
                move_cursor(window_x + 1 + edit_cursor_pos, window_y + 2)?;
            }
        }
    }

    // Draw context menu if in Menu mode
    if let Mode::Menu = mode {
        // Create ContextMenu component
        let context_menu = ContextMenu::new(menu_items.to_vec(), menu_selection);
        
        // Calculate menu position (right-justified, at first row)
        let (terminal_width, terminal_height) = get_terminal_size()?;
        
        // Render the context menu component
        let menu_cells = context_menu.render(terminal_width, terminal_height, theme, false);
        
        // Calculate menu dimensions for positioning
        let menu_width = menu_cells.first().map(|row| row.len()).unwrap_or(0);
        
        // Position menu at top-right (right-justified, at first row)
        let start_col = terminal_width.saturating_sub(menu_width);
        let start_row = 0;
        
        // Write menu cells to buffer
        write_cells_to_buffer(&mut writer, &menu_cells, start_col, start_row);
    }

    // Draw status line at the bottom using StatusBar component
    let (terminal_width, terminal_height) = get_terminal_size()?;
    let status_row = terminal_height - 1; // Last row (0-indexed)
    
    // Create and render StatusBar component
    let status_bar = StatusBar::new(status_message.to_string());
    let status_cells = status_bar.render(terminal_width, 1, theme, false);
    
    // Write status bar to buffer
    write_cells_to_buffer(&mut writer, &status_cells, 0, status_row);
    
    // Drop the writer to release the mutable borrow
    drop(writer);
    
    // Compare buffers and write differences to terminal
    buffer_manager.render_to_terminal()?;

    // Position cursor when in filter mode or edit mode
    // This must be done AFTER all other drawing to ensure cursor is in the right place
    if filter_mode && matches!(mode, Mode::Browse) {
        show_cursor()?;
        move_cursor(8 + edit_cursor_pos, 2)?; // "filter: " is 8 chars, row 2 is filter line
    } else if matches!(mode, Mode::Edit) && !entries.is_empty() {
        // In Edit mode, reposition the cursor to the edit field
        // The cursor was already shown and positioned in the DetailPanel rendering,
        // but we need to ensure it stays visible after drawing the status line
        show_cursor()?;
        let start_col: usize = COL1_WIDTH + 2;
        let start_row = header_height;
        let edit_cursor_min = if edit_field.is_editable() {
            edit_field.display_name().len() + 2
        } else {
            0
        };
        move_cursor(
            start_col + 1 + edit_cursor_min + edit_cursor_pos,
            start_row + 1 + usize::from(edit_field),
        )?;
    }

    Ok(())
}

pub fn get_max_displayed_items_with_header_height(header_height: usize) -> io::Result<usize> {
    let (_, rows) = get_terminal_size()?;
    let max_lines = rows - header_height - FOOTER_SIZE - 1; // Adjust for header and footer lines
    Ok(max_lines)
}

/// Render the torrent search input screen
pub fn draw_torrent_search_input(
    buffer_manager: &mut crate::buffer::BufferManager,
    search_query: &str,
    theme: &Theme,
) -> io::Result<()> {
    // Clear desired buffer to start with empty slate
    buffer_manager.clear_desired_buffer();
    
    // Get writer for this frame
    let mut writer = buffer_manager.get_writer();
    
    hide_cursor()?;
    
    let (terminal_width, _) = get_terminal_size()?;
    
    // Parse theme colors
    let header_fg = string_to_color(&theme.header_fg).unwrap_or(crossterm::style::Color::Reset);
    let help_fg = string_to_color(&theme.help_fg).unwrap_or(crossterm::style::Color::Reset);
    
    // Display header: "Search Online - The Pirate Bay"
    writer.move_to(0, 0);
    writer.set_fg_color(header_fg);
    writer.set_bg_color(crossterm::style::Color::Reset);
    writer.set_bold(true);
    writer.write_str("Search Online - The Pirate Bay");
    writer.set_bold(false);
    
    // Display input field with current query
    writer.move_to(0, 2);
    writer.set_fg_color(crossterm::style::Color::Reset);
    writer.write_str("Query: ");
    writer.write_str(search_query);
    
    // Display instructions: "Enter: Search | ESC: Cancel"
    writer.move_to(0, 4);
    writer.set_fg_color(help_fg);
    writer.write_str("Enter: Search | ESC: Cancel");
    
    // Draw status line at the bottom
    let (_, terminal_height) = get_terminal_size()?;
    let status_row = terminal_height - 1;
    
    let status_bar = StatusBar::new("Enter your search query".to_string());
    let status_cells = status_bar.render(terminal_width, 1, theme, false);
    
    // Write status bar to buffer
    write_cells_to_buffer(&mut writer, &status_cells, 0, status_row);
    
    // Drop the writer to release the mutable borrow
    drop(writer);
    
    // Compare buffers and write differences to terminal
    buffer_manager.render_to_terminal()?;
    
    // Show cursor at the end of the query
    show_cursor()?;
    move_cursor(7 + search_query.len(), 2)?; // "Query: " is 7 chars, row 2
    
    Ok(())
}

/// Render the torrent search results screen
pub fn draw_torrent_search_results(
    buffer_manager: &mut crate::buffer::BufferManager,
    results: &[crate::torrent_search::TorrentResult],
    selected_index: usize,
    theme: &Theme,
) -> io::Result<()> {
    // Clear desired buffer to start with empty slate
    buffer_manager.clear_desired_buffer();
    
    // Get writer for this frame
    let mut writer = buffer_manager.get_writer();
    
    hide_cursor()?;
    
    let (terminal_width, terminal_height) = get_terminal_size()?;
    
    // Parse theme colors
    let header_fg = string_to_color(&theme.header_fg).unwrap_or(crossterm::style::Color::Reset);
    let help_fg = string_to_color(&theme.help_fg).unwrap_or(crossterm::style::Color::Reset);
    let selected_fg = string_to_color(&theme.current_fg).unwrap_or(crossterm::style::Color::Black);
    let selected_bg = string_to_color(&theme.current_bg).unwrap_or(crossterm::style::Color::White);
    let normal_fg = string_to_color(&theme.episode_fg).unwrap_or(crossterm::style::Color::Reset);
    let normal_bg = string_to_color(&theme.episode_bg).unwrap_or(crossterm::style::Color::Reset);
    
    // Display header: "Search Results (Top 5)"
    writer.move_to(0, 0);
    writer.set_fg_color(header_fg);
    writer.set_bg_color(crossterm::style::Color::Reset);
    writer.set_bold(true);
    writer.write_str("Search Results (Top 5)");
    writer.set_bold(false);
    
    // Display table header
    writer.move_to(0, 2);
    writer.set_fg_color(header_fg);
    writer.set_bold(true);
    
    // Calculate column widths
    let title_width = terminal_width.saturating_sub(40); // Reserve space for other columns
    let uploaded_width = 10;
    let size_width = 10;
    let seeds_width = 8;
    let leeches_width = 8;
    
    // Write column headers
    writer.write_str(&format!("{:<width$}", "Title", width = title_width));
    writer.write_str(&format!("{:<width$}", "Uploaded", width = uploaded_width));
    writer.write_str(&format!("{:<width$}", "Size", width = size_width));
    writer.write_str(&format!("{:>width$}", "Seeds", width = seeds_width));
    writer.write_str(&format!("{:>width$}", "Leeches", width = leeches_width));
    writer.set_bold(false);
    
    // Display results
    for (idx, result) in results.iter().enumerate() {
        let row = 3 + idx;
        writer.move_to(0, row);
        
        // Apply theme colors based on selection
        if idx == selected_index {
            writer.set_fg_color(selected_fg);
            writer.set_bg_color(selected_bg);
        } else {
            writer.set_fg_color(normal_fg);
            writer.set_bg_color(normal_bg);
        }
        
        // Truncate title if too long
        let title = if result.name.len() > title_width {
            format!("{}...", &result.name[..title_width.saturating_sub(3)])
        } else {
            result.name.clone()
        };
        
        // Write row data
        writer.write_str(&format!("{:<width$}", title, width = title_width));
        writer.write_str(&format!("{:<width$}", result.uploaded, width = uploaded_width));
        writer.write_str(&format!("{:<width$}", result.size, width = size_width));
        writer.write_str(&format!("{:>width$}", result.seeders, width = seeds_width));
        writer.write_str(&format!("{:>width$}", result.leechers, width = leeches_width));
        
        // Clear to end of line to ensure full row is highlighted
        writer.set_bg_color(crossterm::style::Color::Reset);
    }
    
    // Display instructions
    let instructions_row = 3 + results.len() + 2;
    writer.move_to(0, instructions_row);
    writer.set_fg_color(help_fg);
    writer.set_bg_color(crossterm::style::Color::Reset);
    writer.write_str("↑↓: Navigate | Enter: Download | ESC: Cancel");
    
    // Draw status line at the bottom
    let status_row = terminal_height - 1;
    
    let status_message = if results.is_empty() {
        "No results found".to_string()
    } else {
        format!("Select a torrent to download ({}/{})", selected_index + 1, results.len())
    };
    
    let status_bar = StatusBar::new(status_message);
    let status_cells = status_bar.render(terminal_width, 1, theme, false);
    
    // Write status bar to buffer
    write_cells_to_buffer(&mut writer, &status_cells, 0, status_row);
    
    // Drop the writer to release the mutable borrow
    drop(writer);
    
    // Compare buffers and write differences to terminal
    buffer_manager.render_to_terminal()?;
    
    Ok(())
}

/// Convert a color string to a Color enum
fn string_to_color(color: &str) -> Option<crossterm::style::Color> {
    match color.to_lowercase().as_str() {
        "black" => Some(crossterm::style::Color::Black),
        "red" => Some(crossterm::style::Color::Red),
        "green" => Some(crossterm::style::Color::Green),
        "yellow" => Some(crossterm::style::Color::Yellow),
        "blue" => Some(crossterm::style::Color::Blue),
        "magenta" => Some(crossterm::style::Color::Magenta),
        "cyan" => Some(crossterm::style::Color::Cyan),
        "white" => Some(crossterm::style::Color::White),
        "darkgray" | "dark_gray" => Some(crossterm::style::Color::DarkGrey),
        "reset" => Some(crossterm::style::Color::Reset),
        _ => None,
    }
}

/// Write component cells to buffer at specified position.
///
/// This is a key integration point between the component system and the buffer layer.
/// Components render to their own Cell arrays, which are then written to the buffer
/// at the appropriate screen position.
///
/// # Buffer Integration
///
/// - Converts component Cell format to buffer Cell format
/// - Handles positioning and styling
/// - Replaces direct terminal write operations
fn write_cells_to_buffer(
    writer: &mut crate::buffer::BufferWriter,
    cells: &[Vec<crate::components::Cell>],
    start_x: usize,
    start_y: usize,
) {
    for (row_index, row) in cells.iter().enumerate() {
        writer.move_to(start_x, start_y + row_index);
        for cell in row {
            // Colors are already crossterm::style::Color, no conversion needed
            writer.set_fg_color(cell.fg_color);
            writer.set_bg_color(cell.bg_color);
            writer.set_bold(cell.style.bold);
            writer.set_italic(cell.style.italic);
            writer.set_underlined(cell.style.underlined);
            writer.set_dim(cell.style.dim);
            writer.write_char(cell.character);
        }
    }
}


