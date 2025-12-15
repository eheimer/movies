use crate::components::{Component, category::{Category, CategoryType}, render_cells_at_column, Scrollbar, Browser, DetailPanel, StatusBar};
use crate::components::episode::Episode;
use crate::components::header::{Header, HeaderContext};
use crate::dto::{EpisodeDetail, Series};
use crate::episode_field::EpisodeField;
use crate::menu::MenuItem;
use crate::terminal::{
    clear_line, clear_screen, get_terminal_size, hide_cursor, move_cursor, print_at, show_cursor,
};
use crate::theme::Theme;
use crate::util::{truncate_string, Entry, LastAction, Mode, ViewContext};
use crossterm::event::KeyCode;
use crossterm::style::{Color, Stylize};
use std::collections::HashSet;
use std::convert::From;
use std::io;


const FOOTER_SIZE: usize = 1; // Reserve 1 line for status line at bottom
const COL1_WIDTH: usize = 45;
const MIN_COL2_WIDTH: usize = 20;
const DETAIL_HEIGHT: usize = 11;
const SERIES_WIDTH: usize = 40;

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



pub fn string_to_color(color: &str) -> Option<Color> {
    match color.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "darkgray" | "dark_gray" => Some(Color::DarkGrey),
        "reset" => Some(Color::Reset),
        _ => None,
    }
}

pub fn string_to_bg_color_or_default(color: &str) -> Color {
    string_to_color(color).unwrap_or(Color::White)
}

pub fn string_to_fg_color_or_default(color: &str) -> Color {
    string_to_color(color).unwrap_or(Color::Black)
}







pub fn draw_screen(
    entries: &[Entry],
    current_item: usize,
    first_entry: &mut usize,
    filter: &String,
    theme: &Theme,
    mode: &Mode,
    entry_path: &String,
    edit_details: &EpisodeDetail,
    edit_field: EpisodeField,
    edit_cursor_pos: usize,
    series: &Vec<Series>,
    series_selection: &mut Option<usize>,
    new_series: &String,
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
) -> io::Result<()> {
    clear_screen()?;

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
        filter.clone(),
        filter_mode, // filter_focused is same as filter_mode for now
    );

    // Create and render Header component
    let header = Header::new(&header_context);
    let header_height = header.calculate_height();
    let header_cells = header.render(terminal_width, header_height, theme, false);

    // Render header cells to terminal (always render all rows for fixed layout)
    for (row_index, row) in header_cells.iter().enumerate() {
        let text = cells_to_styled_string(&[row.clone()]);
        print_at(0, row_index, &text)?;
    }

    // Handle Entry mode display (both first-run and rescan scenarios)
    if let Mode::Entry = mode {
        if entries.is_empty() {
            // First-run scenario - show welcome message with detailed instructions
            print_at(
                0,
                header_height,
                &"Welcome to the video library manager!".to_string(),
            )?;
            print_at(
                0,
                header_height + 1,
                &"".to_string(),
            )?;
            print_at(
                0,
                header_height + 2,
                &"To get started, enter the full path to your video collection directory below.".to_string(),
            )?;
            print_at(
                0,
                header_height + 3,
                &"".to_string(),
            )?;
            print_at(
                0,
                header_height + 4,
                &"What happens next:".to_string(),
            )?;
            print_at(
                0,
                header_height + 5,
                &"  • If videos.sqlite exists in that directory, it will be used (preserving your data)".to_string(),
            )?;
            print_at(
                0,
                header_height + 6,
                &"  • If not, a new database will be created and your videos will be scanned".to_string(),
            )?;
            print_at(
                0,
                header_height + 7,
                &"".to_string(),
            )?;
            print_at(
                0,
                header_height + 8,
                &format!("Path: {}", entry_path),
            )?;
        } else {
            // Rescan scenario - show simpler prompt
            print_at(
                0,
                header_height + 1,
                &"Enter the path to a directory to scan for video files.".to_string(),
            )?;
            print_at(
                0,
                header_height + 2,
                &"".to_string(),
            )?;
            print_at(
                0,
                header_height + 3,
                &format!("Path: {}", entry_path),
            )?;
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
        
        // Render the browser output to the terminal
        for (row_index, row) in browser_cells.iter().enumerate() {
            if !row.is_empty() {
                // Convert Cell array to styled display text
                let text = cells_to_styled_string(&[row.clone()]);
                print_at(0, header_height + row_index, &text)?;
            }
        }
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
            
            // Draw the window border
            draw_window(
                start_col,
                start_row,
                sidebar_width,
                DETAIL_HEIGHT,
                edit_mode,
            )?;
            
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
            
            // Render the detail panel output to the terminal (inside the border)
            for (row_index, row) in detail_cells.iter().enumerate() {
                if !row.is_empty() && row_index < content_height {
                    // Convert Cell array to styled display text
                    let text = cells_to_styled_string(&[row.clone()]);
                    print_at(start_col + 1, start_row + 1 + row_index, &text)?;
                }
            }
            
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
            draw_series_window(mode, series, new_series, series_selection, theme, first_series, header_height)?;
        }
    }

    // Draw context menu if in Menu mode
    if let Mode::Menu = mode {
        draw_context_menu(menu_items, menu_selection, theme)?;
    }

    // Draw status line at the bottom using StatusBar component
    let (terminal_width, terminal_height) = get_terminal_size()?;
    let status_row = terminal_height - 1; // Last row (0-indexed)
    
    // Clear the status line
    clear_line(status_row)?;
    
    // Create and render StatusBar component
    let status_bar = StatusBar::new(status_message.to_string());
    let status_cells = status_bar.render(terminal_width, 1, theme, false);
    
    // Render the status bar to terminal
    if let Some(status_row_cells) = status_cells.first() {
        let text = cells_to_styled_string(&[status_row_cells.clone()]);
        print_at(0, status_row, &text)?;
    }

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



fn draw_series_window(
    mode: &Mode,
    series: &Vec<Series>,
    new_series: &String,
    series_selection: &mut Option<usize>,
    theme: &Theme,
    first_series: &mut usize,
    header_height: usize,
) -> io::Result<()> {
    let start_col = COL1_WIDTH + 2;
    let start_row = header_height + DETAIL_HEIGHT;
    let sidebar_width = get_sidebar_width()?;
    let series_window_width = SERIES_WIDTH + 2;

    // Calculate the available height for the terminal
    let (_, terminal_height) = get_terminal_size()?;
    let max_height = terminal_height.saturating_sub(start_row + 2); // Adjust for borders
    let mut series_window_height = (series.len() + 3).min(max_height).max(4); // Minimum height is 4

    if let Mode::SeriesCreate = mode {
        series_window_height = 4;
        *series_selection = None;
    } else {
        //if series_selection is out of bounds, make it in-bounds, if it is None, set it to 0
        if let Some(selection) = series_selection {
            if *selection >= series.len() {
                *series_selection = series.len().checked_sub(1);
            }
        } else {
            *series_selection = Some(0);
        }
    }

    let series_window_start_col = start_col + (sidebar_width.saturating_sub(series_window_width) / 2);

    draw_window(
        series_window_start_col,
        start_row,
        series_window_width,
        series_window_height,
        matches!(mode, Mode::SeriesCreate),
    )?;

    // write the contents
    if let Mode::SeriesCreate = mode {
        show_cursor()?;
        print_at(
            series_window_start_col + 1,
            start_row + 1,
            &format!(
                "{}",
                "Type the series name and press [ENTER]:"
                    .with(Color::Black)
                    .on(Color::White)
            ),
        )?;
        print_at(
            series_window_start_col + 1,
            start_row + 2,
            &new_series.to_string(),
        )?;
    } else {
        print_at(
            series_window_start_col + 1,
            start_row + 1,
            &format!(
                "{}",
                "Choose a series or [+] to create"
                    .with(Color::Black)
                    .on(Color::White)
            ),
        )?;
        
        // Calculate maximum visible series items (subtract borders and title)
        let max_visible_series = series_window_height.saturating_sub(3).max(1);
        
        // Create Scrollbar component for the series select window
        let series_scrollbar = Scrollbar::new(
            series.len(),                           // total_items
            max_visible_series,                     // visible_items
            *first_series,                          // first_visible_index
        );
        
        // Render scrollbar to get visibility information
        let series_scrollbar_cells = series_scrollbar.render(1, max_visible_series, theme, false);
        let series_scrollbar_visible = !series_scrollbar_cells.is_empty();
        
        // Calculate effective series width (reduce by 1 if scroll bar is visible)
        // Use saturating_sub to prevent underflow
        let effective_series_width = if series_scrollbar_visible {
            SERIES_WIDTH.saturating_sub(1)
        } else {
            SERIES_WIDTH
        };
        
        // Implement viewport adjustment logic
        if let Some(selection) = series_selection {
            if *selection < *first_series {
                *first_series = *selection;
            } else if *selection >= *first_series + max_visible_series {
                *first_series = *selection - max_visible_series + 1;
            }
        }
        
        // Update series rendering to use viewport
        for (i, series_item) in series.iter()
            .enumerate()
            .skip(*first_series)
            .take(max_visible_series)
        {
            let display_text = format!(
                "[{}] {}",
                i + 1,
                truncate_string(&series_item.name, effective_series_width)
            );
            let formatted_text = if Some(i) == *series_selection {
                format!(
                    "{}",
                    display_text
                        .with(string_to_fg_color_or_default(&theme.current_fg))
                        .on(string_to_bg_color_or_default(&theme.current_bg))
                )
            } else {
                display_text
            };
            // Adjust row calculation to account for skipped items
            print_at(
                series_window_start_col + 1,
                start_row + 2 + (i - *first_series),
                &formatted_text,
            )?;
        }
        
        // Render the scroll bar after rendering series items
        if series_scrollbar_visible {
            render_cells_at_column(&series_scrollbar_cells, series_window_start_col + series_window_width - 1, start_row + 2)?;
        }
    }
    Ok(())
}

fn draw_context_menu(
    menu_items: &[MenuItem],
    selected_index: usize,
    theme: &Theme,
) -> io::Result<()> {
    if menu_items.is_empty() {
        return Ok(());
    }

    // Calculate menu dimensions - need to account for label + spacing + hotkey
    let max_label_width = menu_items
        .iter()
        .map(|item| item.label.len())
        .max()
        .unwrap_or(20);
    
    let max_hotkey_width = menu_items
        .iter()
        .map(|item| format_hotkey(&item.hotkey).len())
        .max()
        .unwrap_or(5);
    
    // Width = left padding + label + spacing + hotkey + right padding
    let menu_width = 2 + max_label_width + 2 + max_hotkey_width + 2;
    let menu_height = menu_items.len() + 2; // Add 2 for top and bottom borders

    // Calculate menu position (right-justified, at first row)
    let (terminal_width, _) = get_terminal_size()?;
    let start_col = terminal_width.saturating_sub(menu_width);
    let start_row = 0;

    // Draw the menu window with double-line border
    draw_window(start_col, start_row, menu_width, menu_height, true)?;

    // Draw each menu item with left-justified label and right-justified hotkey
    for (i, item) in menu_items.iter().enumerate() {
        let hotkey_str = format_hotkey(&item.hotkey);
        // Use saturating_sub to prevent underflow when menu is very narrow
        let content_width = menu_width.saturating_sub(2); // Subtract borders
        
        // Create the display text with label left-justified and hotkey right-justified
        let spacing = content_width.saturating_sub(item.label.len() + hotkey_str.len());
        let display_text = format!("{}{}{}", item.label, " ".repeat(spacing), hotkey_str);
        
        let formatted_text = if i == selected_index {
            // Highlight the selected item
            format!(
                "{}",
                display_text
                    .with(string_to_fg_color_or_default(&theme.current_fg))
                    .on(string_to_bg_color_or_default(&theme.current_bg))
            )
        } else {
            display_text
        };

        print_at(start_col + 1, start_row + 1 + i, &formatted_text)?;
    }

    Ok(())
}

fn format_hotkey(hotkey: &Option<KeyCode>) -> String {
    match hotkey {
        Some(KeyCode::F(n)) => format!("[F{}]", n),
        Some(KeyCode::Char(c)) => format!("[{}]", c.to_uppercase()),
        Some(KeyCode::Enter) => "[ENTER]".to_string(),
        Some(KeyCode::Esc) => "[ESC]".to_string(),
        _ => "".to_string(),
    }
}

fn draw_window(
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    thick: bool,
) -> io::Result<()> {
    // Choose border characters based on the thickness
    let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) = if thick {
        ('╔', '╗', '╚', '╝', '═', '║')
    } else {
        ('┌', '┐', '└', '┘', '─', '│')
    };

    // Draw top border
    print_at(left, top, &top_left)?;
    // Use saturating_sub to prevent underflow when width is very small
    for _ in 1..width.saturating_sub(1) {
        print!("{}", horizontal);
    }
    print!("{}", top_right);

    // Draw side borders and clear interior
    // Use saturating_sub to prevent underflow when height is very small
    for row in (top + 1)..(top + height.saturating_sub(1)) {
        print_at(left, row, &vertical)?;
        // Clear the interior space
        for _ in 1..width.saturating_sub(1) {
            print!(" ");
        }
        print!("{}", vertical);
    }

    // Draw bottom border
    print_at(left, top + height.saturating_sub(1), &bottom_left)?;
    for _ in 1..width.saturating_sub(1) {
        print!("{}", horizontal);
    }
    println!("{}", bottom_right);

    Ok(())
}





pub fn get_max_displayed_items_with_header_height(header_height: usize) -> io::Result<usize> {
    let (_, rows) = get_terminal_size()?;
    let max_lines = rows - header_height - FOOTER_SIZE - 1; // Adjust for header and footer lines
    Ok(max_lines)
}







/// Convert a 2D Cell array to a String
/// 


/// Convert a 2D array of Cells to a styled string with ANSI codes
fn cells_to_styled_string(cells: &[Vec<crate::components::Cell>]) -> String {
    use crossterm::style::Stylize;
    
    cells.iter()
        .map(|row| {
            row.iter()
                .map(|cell| {
                    let mut styled = cell.character.to_string()
                        .with(cell.fg_color)
                        .on(cell.bg_color);
                    
                    // Apply text styles
                    if cell.style.bold {
                        styled = styled.bold();
                    }
                    if cell.style.italic {
                        styled = styled.italic();
                    }
                    if cell.style.underlined {
                        styled = styled.underlined();
                    }
                    if cell.style.dim {
                        styled = styled.dim();
                    }
                    
                    format!("{}", styled)
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}


