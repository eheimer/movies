use crate::config::Config;
use crate::dto::{EpisodeDetail, Series};
use crate::episode_field::EpisodeField;
use crate::menu::{MenuItem, MenuContext};
use crate::terminal::{
    clear_line, clear_screen, get_terminal_size, hide_cursor, move_cursor, print_at, show_cursor,
};
use crate::util::{can_repeat_action, truncate_string, Entry, LastAction, Mode, ViewContext};
use crossterm::event::KeyCode;
use crossterm::style::{Color, Stylize};
use std::collections::HashSet;
use std::convert::From;
use std::io;

const HEADER_SIZE: usize = 5;
const FOOTER_SIZE: usize = 0;
const COL1_WIDTH: usize = 45;
const MIN_COL2_WIDTH: usize = 20;
const DETAIL_HEIGHT: usize = 11;
const SERIES_WIDTH: usize = 40;

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
        _ => None,
    }
}

pub fn string_to_bg_color_or_default(color: &str) -> Color {
    string_to_color(color).unwrap_or(Color::White)
}

pub fn string_to_fg_color_or_default(color: &str) -> Color {
    string_to_color(color).unwrap_or(Color::Black)
}

fn draw_header(
    mode: &Mode,
    filter: &String,
    series_selected: bool,
    season_selected: bool,
    series_filter_active: bool,
    last_action_display: &str,
    is_dirty: bool,
    selected_entry: Option<&Entry>,
    edit_details: &EpisodeDetail,
    filter_mode: bool,
    config: &Config,
    last_action: &Option<LastAction>,
    view_context: &ViewContext,
    is_first_run: bool,
) -> io::Result<()> {
    // Get terminal width for overflow calculation
    let (terminal_width, _) = get_terminal_size()?;
    
    // Start building the header string
    let mut header = String::new();
    
    // 1. Always start with "[F1] Menu, "
    header.push_str("[F1] Menu, ");
    
    // 2. Build hardcoded context-specific helpers based on mode/context
    let hardcoded_helpers = match mode {
        Mode::Browse => {
            // When in filter mode, show simplified menu helpers
            if filter_mode {
                "[ENTER] accept, [ESC] cancel".to_string()
            } else if series_selected {
                "[/] filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] show episodes, [ESC] exit".to_string()
            } else if season_selected {
                "[/] filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] show episodes, [ESC] back".to_string()
            } else if series_filter_active {
                "[/] filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] play, [ESC] back".to_string()
            } else {
                "[/] filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] play, [ESC] exit".to_string()
            }
        }
        Mode::Edit => {
            let mut instruction = "[\u{2191}]/[\u{2193}] change field, [ESC] cancel".to_string();
            if is_dirty {
                instruction.push_str(", [F2] save changes");
            }
            instruction
        },
        Mode::Entry => {
            // Check if we're in first-run state (no entries and no database)
            if is_first_run {
                "Welcome! Enter the path to your video collection directory, [ESC] cancel".to_string()
            } else {
                "Enter a file path to scan, [ESC] cancel".to_string()
            }
        },
        Mode::SeriesSelect => {
            "[\u{2191}]/[\u{2193}] navigate, [ENTER] select, [ESC] cancel, [+] create a new series, [CTRL][-] deselect series".to_string()
        },
        Mode::SeriesCreate => "Type a series name, [ENTER] save, [ESC] cancel".to_string(),
        Mode::Menu => {
            "[\u{2191}]/[\u{2193}] navigate, [ENTER] select, [ESC] close menu".to_string()
        },
    };
    
    // Add hardcoded helpers to header
    header.push_str(&hardcoded_helpers);
    
    // 3. Calculate remaining width for FirstLinePreferred items
    let used_width = header.len();
    let remaining_width = terminal_width.saturating_sub(used_width);
    
    // 4. Get FirstLinePreferred items (only in Browse mode, not in filter mode)
    if matches!(mode, Mode::Browse) && !filter_mode {
        let menu_context = MenuContext {
            selected_entry: selected_entry.cloned(),
            episode_detail: edit_details.clone(),
            last_action: last_action.clone(),
        };
        
        let first_line_preferred = crate::menu::get_first_line_preferred_items(&menu_context);
        
        // 5. Add FirstLinePreferred items that fit within remaining width
        let mut available_width = remaining_width;
        let mut first_item = true;
        
        for item in first_line_preferred {
            let item_width = crate::menu::calculate_menu_helper_width(&item);
            
            if item_width <= available_width {
                // Add separator before each item
                if first_item {
                    header.push_str(", ");
                    available_width = available_width.saturating_sub(2);
                    first_item = false;
                }
                
                // Format the menu item: "[hotkey] label, "
                let hotkey_str = format_hotkey(&item.hotkey);
                let item_str = format!("{} {}, ", hotkey_str, item.label);
                header.push_str(&item_str);
                
                available_width = available_width.saturating_sub(item_width);
            } else {
                // Item doesn't fit, stop adding items
                break;
            }
        }
    }
    
    // Remove trailing ", " if present
    if header.ends_with(", ") {
        header.truncate(header.len() - 2);
    }
    
    // Print single instruction line
    print_at(0, 0, &header.as_str().with(Color::Black).on(Color::White))?;

    // Print last action display at row 1
    print_at(0, 1, &last_action_display)?;

    // Print breadcrumbs at row 2 based on view context
    match view_context {
        ViewContext::Series { series_name, .. } => {
            print_at(0, 2, &format!("Browsing [{}]", series_name))?;
        }
        ViewContext::Season { series_name, season_number, .. } => {
            print_at(0, 2, &format!("Browsing [{}] -> [season {}]", series_name, season_number))?;
        }
        ViewContext::TopLevel => {
            // No breadcrumbs at top level
        }
    }

    // Show filter line when filter_mode is true OR filter string is not empty
    if filter_mode || !filter.is_empty() {
        // Apply highlight to "filter:" label when in filter mode
        let filter_label = if filter_mode {
            format!("filter:")
                .with(string_to_fg_color_or_default(&config.current_fg))
                .on(string_to_bg_color_or_default(&config.current_bg))
                .to_string()
        } else {
            "filter:".to_string()
        };
        
        print_at(0, 3, &format!("{} {}", filter_label, filter))?;
    }
    Ok(())
}

pub fn draw_screen(
    entries: &[Entry],
    current_item: usize,
    first_entry: &mut usize,
    filter: &String,
    config: &Config,
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
) -> io::Result<()> {
    clear_screen()?;

    hide_cursor()?;

    //browse_series is true if the mode is browse and the current item in entries is a series
    let series_selected = matches!(mode, Mode::Browse)
        && matches!(entries.get(current_item), Some(Entry::Series { .. }));
    let season_selected = matches!(mode, Mode::Browse)
        && matches!(entries.get(current_item), Some(Entry::Season { .. }));

    //series_filter is true if the mode is browse and the current item in entries is an episode and the series field is not empty
    let series_filter_active = matches!(mode, Mode::Browse)
        && matches!(entries.get(current_item), Some(Entry::Episode { .. }))
        && edit_details.series.is_some();

    // Calculate last_action_display string
    let last_action_display = if !entries.is_empty() {
        let selected_entry = entries.get(current_item);
        if let Some(entry) = selected_entry {
            if can_repeat_action(last_action, entry, edit_details) {
                last_action.as_ref().map(|a| a.format_display()).unwrap_or_default()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Calculate is_dirty from dirty_fields
    let is_dirty = !dirty_fields.is_empty();

    // Extract selected entry for draw_header
    let selected_entry = entries.get(current_item);

    // Determine if we're in first-run state (Entry mode with no entries)
    let is_first_run = matches!(mode, Mode::Entry) && entries.is_empty();
    
    draw_header(
        mode,
        filter,
        series_selected,
        season_selected,
        series_filter_active,
        &last_action_display,
        is_dirty,
        selected_entry,
        edit_details,
        filter_mode,
        config,
        last_action,
        view_context,
        is_first_run,
    )?;

    // Handle Entry mode display (both first-run and rescan scenarios)
    if let Mode::Entry = mode {
        if entries.is_empty() {
            // First-run scenario - show welcome message with detailed instructions
            print_at(
                0,
                HEADER_SIZE,
                &"Welcome to the video library manager!".to_string(),
            )?;
            print_at(
                0,
                HEADER_SIZE + 1,
                &"".to_string(),
            )?;
            print_at(
                0,
                HEADER_SIZE + 2,
                &"To get started, enter the full path to your video collection directory below.".to_string(),
            )?;
            print_at(
                0,
                HEADER_SIZE + 3,
                &"".to_string(),
            )?;
            print_at(
                0,
                HEADER_SIZE + 4,
                &"What happens next:".to_string(),
            )?;
            print_at(
                0,
                HEADER_SIZE + 5,
                &"  • If videos.sqlite exists in that directory, it will be used (preserving your data)".to_string(),
            )?;
            print_at(
                0,
                HEADER_SIZE + 6,
                &"  • If not, a new database will be created and your videos will be scanned".to_string(),
            )?;
            print_at(
                0,
                HEADER_SIZE + 7,
                &"".to_string(),
            )?;
            print_at(
                0,
                HEADER_SIZE + 8,
                &format!("Path: {}", entry_path),
            )?;
        } else {
            // Rescan scenario - show simpler prompt
            print_at(
                0,
                HEADER_SIZE + 1,
                &"Enter the path to a directory to scan for video files.".to_string(),
            )?;
            print_at(
                0,
                HEADER_SIZE + 2,
                &"".to_string(),
            )?;
            print_at(
                0,
                HEADER_SIZE + 3,
                &format!("Path: {}", entry_path),
            )?;
        }
    } else if !entries.is_empty() {
        let max_lines = get_max_displayed_items()?;

        //make sure current_item is between first_entry and first_entry + max_lines.  If it's not, adjust first_entry
        if current_item < *first_entry {
            *first_entry = current_item;
        } else if current_item >= *first_entry + max_lines as usize {
            *first_entry = current_item - max_lines as usize + 1;
        }

        for (i, entry) in entries
            .iter()
            .enumerate()
            .skip(*first_entry)
            .take(max_lines as usize)
        {
            let display_text = match entry {
                Entry::Series { name, .. } => {
                    format!("[{}]", truncate_string(name, COL1_WIDTH)).with(Color::Blue)
                }
                Entry::Episode { name, .. } => truncate_string(name, COL1_WIDTH).clone().stylize(),
                Entry::Season { number, .. } => format!("Season {}", number).with(Color::Blue),
            };

            let mut formatted_text = format!("{}", display_text);
            if i == current_item && !filter_mode {
                formatted_text = format!(
                    "{}",
                    display_text
                        .with(string_to_fg_color_or_default(&config.current_fg))
                        .on(string_to_bg_color_or_default(&config.current_bg))
                );
            }
            print_at(0, i - *first_entry + HEADER_SIZE, &formatted_text)?;
        }
        if !series_selected && !season_selected && !matches!(mode, Mode::Menu) {
            draw_detail_window(
                &entries[current_item],
                mode,
                edit_details,
                edit_field,
                edit_cursor_pos,
                season_number,
                dirty_fields,
                config,
            )?;
        }
        if let Mode::SeriesSelect | Mode::SeriesCreate = mode {
            draw_series_window(mode, series, new_series, series_selection, config, first_series)?;
        }
    }

    // Draw context menu if in Menu mode
    if let Mode::Menu = mode {
        draw_context_menu(menu_items, menu_selection, config)?;
    }

    // Position cursor at edit_cursor_pos when in filter mode
    // This must be done AFTER all other drawing to ensure cursor is in the right place
    if filter_mode && matches!(mode, Mode::Browse) {
        show_cursor()?;
        move_cursor(8 + edit_cursor_pos, 3)?; // "filter: " is 8 chars, row 3 is filter line
    }

    Ok(())
}

fn draw_detail_window(
    entry: &Entry,
    mode: &Mode,
    edit_details: &EpisodeDetail,
    edit_field: EpisodeField,
    edit_cursor_pos: usize,
    season_number: Option<usize>,
    dirty_fields: &HashSet<EpisodeField>,
    config: &Config,
) -> io::Result<()> {
    let start_col: usize = COL1_WIDTH + 2;
    let start_row = HEADER_SIZE;
    let sidebar_width = get_sidebar_width()?;
    let edit_mode = matches!(mode, Mode::Edit);

    // Show or hide the cursor based on edit_mode
    if edit_mode {
        show_cursor()?;
    }

    draw_window(
        start_col,
        start_row,
        sidebar_width,
        DETAIL_HEIGHT,
        edit_mode,
    )?;

    // Extract path and filename from location
    let location = match entry {
        Entry::Episode { location, .. } => location,
        _ => "",
    };
    let path = location.rsplit_once('/').map(|x| x.0).unwrap_or("");
    let filename = location.rsplit('/').next().unwrap_or("");

    let mut edit_cursor_min: usize = 0;
    if edit_mode && edit_field.is_editable() {
        edit_cursor_min = edit_field.display_name().len() + 2;
    }

    for i in 0..=8 {
        let field = EpisodeField::from(i);
        let value: String = if field == EpisodeField::Path {
            path.to_string()
        } else if field == EpisodeField::Filename {
            filename.to_string()
        } else if field == EpisodeField::Season {
            if edit_mode {
                match season_number {
                    Some(num) => num.to_string(),
                    None => String::new(),
                }
            } else if let Some(season) = &edit_details.season {
                season.number.to_string()
            } else {
                String::new()
            }
        } else {
            let field_value = field.get_field_value(edit_details);
            if field_value.is_empty() {
                String::new()
            } else {
                field_value
            }
        };

        // Apply dirty colors to field name if in edit mode and field is dirty
        let field_name_display = if edit_mode && dirty_fields.contains(&field) {
            format!("{}:", field.display_name())
                .with(string_to_fg_color_or_default(&config.dirty_fg))
                .on(string_to_bg_color_or_default(&config.dirty_bg))
                .to_string()
        } else {
            format!("{}:", field.display_name())
        };

        let line = format!("{} {}", field_name_display, value);
        
        if edit_mode && edit_field.is_editable() {
            print_at(
                start_col + 1,
                start_row + 1 + i,
                &truncate_string(&line, sidebar_width - 4).to_string(),
            )?;
        } else {
            print_at(
                start_col + 1,
                start_row + 1 + i,
                &truncate_string(&line, sidebar_width - 2).to_string(),
            )?;
        }
    }
    // Put the cursor at the end of the current edit_field line
    if edit_mode {
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
    config: &Config,
    first_series: &mut usize,
) -> io::Result<()> {
    let start_col = COL1_WIDTH + 2;
    let start_row = HEADER_SIZE + DETAIL_HEIGHT;
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

    let series_window_start_col = start_col + ((sidebar_width - series_window_width) / 2);

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
                truncate_string(&series_item.name, SERIES_WIDTH)
            );
            let formatted_text = if Some(i) == *series_selection {
                format!(
                    "{}",
                    display_text
                        .with(string_to_fg_color_or_default(&config.current_fg))
                        .on(string_to_bg_color_or_default(&config.current_bg))
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
    }
    Ok(())
}

fn draw_context_menu(
    menu_items: &[MenuItem],
    selected_index: usize,
    config: &Config,
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
        let content_width = menu_width - 2; // Subtract borders
        
        // Create the display text with label left-justified and hotkey right-justified
        let spacing = content_width.saturating_sub(item.label.len() + hotkey_str.len());
        let display_text = format!("{}{}{}", item.label, " ".repeat(spacing), hotkey_str);
        
        let formatted_text = if i == selected_index {
            // Highlight the selected item
            format!(
                "{}",
                display_text
                    .with(string_to_fg_color_or_default(&config.current_fg))
                    .on(string_to_bg_color_or_default(&config.current_bg))
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
    for _ in 1..width - 1 {
        print!("{}", horizontal);
    }
    print!("{}", top_right);

    // Draw side borders and clear interior
    for row in (top + 1)..(top + height - 1) {
        print_at(left, row, &vertical)?;
        // Clear the interior space
        for _ in 1..width - 1 {
            print!(" ");
        }
        print!("{}", vertical);
    }

    // Draw bottom border
    print_at(left, top + height - 1, &bottom_left)?;
    for _ in 1..width - 1 {
        print!("{}", horizontal);
    }
    println!("{}", bottom_right);

    Ok(())
}

pub fn load_videos(message: &str, count: usize) -> io::Result<()> {
    // Display status message on the status line (row 1)
    // This temporarily overrides the last_action_display during operations.
    // When the screen redraws after the operation completes, the last action will reappear.
    clear_line(1)?;
    if count > 0 {
        print_at(0, 1, &format!("{} ({} videos)", message, count))?;
    } else {
        print_at(0, 1, &message)?;
    }
    Ok(())
}

pub fn get_max_displayed_items() -> io::Result<usize> {
    let (_, rows) = get_terminal_size()?;
    let max_lines = rows - HEADER_SIZE - FOOTER_SIZE - 1; // Adjust for header and footer lines
    Ok(max_lines)
}
