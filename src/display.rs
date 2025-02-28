use crossterm::style::{Color, Stylize};
use std::io;
use crate::dto::{EpisodeDetail,Series};
use crate::config::Config;
use crate::util::{Entry,Mode, truncate_string};
use crate::terminal::{clear_screen,clear_line,get_terminal_size,print_at,hide_cursor,show_cursor,move_cursor};

const HEADER_SIZE: usize = 4;
const FOOTER_SIZE: usize = 0;
const COL1_WIDTH: usize = 45;
const MIN_COL2_WIDTH: usize = 20;
const DETAIL_HEIGHT: usize = 11;
const SERIES_WIDTH: usize = 40;

fn get_sidebar_width() -> io::Result<usize> {
    let (cols, _) = get_terminal_size()?;
    let sidebar_width = (cols as usize).saturating_sub(COL1_WIDTH + 2);
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

fn draw_header(mode: &Mode, filter: &String) -> io::Result<()> {
    let instructions: Vec<&str> = match mode {
        Mode::Browse => vec![
            "type to filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] play, [ESC] exit",
            "[F2] edit, [F3] toggle watched, [F4] series selection",
        ],
        Mode::Edit => vec![
            "[\u{2191}]/[\u{2193}] change field, [ESC] cancel, [F2] save changes",
        ],
        Mode::Entry => vec!["Enter a file path to scan, [ESC] cancel"],
        Mode::SeriesSelect => vec![
            "[\u{2191}]/[\u{2193}] navigate, [ENTER] select, [ESC] cancel",
            "[+] create a new series, [CTRL][-] deselect series",
        ],
        Mode::SeriesCreate => vec!["Type a series name, [ENTER] save, [ESC] cancel"],
    };
    //loop through the instructions and print them in the header
    for (i, instructions) in instructions.iter().enumerate() {
        print_at(1,i, &instructions.with(Color::Black).on(Color::White))?;
    }

    print_at(0,2, &format!("filter: {}", filter))?;
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
    edit_field: usize,
    edit_cursor_pos: usize,
    series: &Vec<Series>,
    series_selection: &mut Option<usize>,
    new_series: &String,
) -> io::Result<()> {
    clear_screen()?;
    draw_header(mode, filter)?;
    
    if entries.is_empty() {
        print_at(0,HEADER_SIZE, 
            &format!("{}", "No videos found. Adjust your filter or press CTRL-L to scan the file system".italic()))?;
        if let Mode::Entry = mode {
            print_at(0,HEADER_SIZE + 1, &format!("Enter a file path to scan: {}", entry_path))?;
        }
    } else {
        let max_lines = get_max_displayed_items()?;

        //make sure current_item is between first_entry and first_entry + max_lines.  If it's not, adjust first_entry
        if current_item < *first_entry {
            *first_entry = current_item;
        } else if current_item >= *first_entry + max_lines as usize {
            *first_entry = current_item - max_lines as usize + 1;
        }

        for (i, entry) in entries.iter().enumerate().skip(*first_entry).take(max_lines as usize) {

            let display_text = match entry {
                Entry::Series { name, .. } => format!("[{}]", truncate_string(name,COL1_WIDTH)).with(Color::Blue),
                Entry::Episode { name, .. } => truncate_string(name,COL1_WIDTH).clone().stylize(),
            };

            let mut formatted_text = format!("{}", display_text);
            if i == current_item {
                formatted_text = format!("{}", display_text.with(string_to_fg_color_or_default(&config.current_fg)).on(string_to_bg_color_or_default(&config.current_bg)));
            }
            print_at(0, i - *first_entry + HEADER_SIZE, &formatted_text)?;

        }
        draw_detail_window(&entries[current_item], &mode, edit_details, edit_field, edit_cursor_pos)?;
        if let Mode::SeriesSelect | Mode::SeriesCreate = mode {
            draw_series_window(&mode, &series, &new_series, series_selection, config)?;
        }
    }

    Ok(())
}

fn draw_detail_window(entry: &Entry, mode: &Mode, edit_details: &EpisodeDetail, edit_field: usize, edit_cursor_pos: usize) -> io::Result<()> {
    let start_col: usize = COL1_WIDTH + 2;
    let start_row = HEADER_SIZE;
    let sidebar_width = get_sidebar_width()?;
    let edit_mode = matches!(mode, Mode::Edit);

    // Show or hide the cursor based on edit_mode
    if edit_mode {
        show_cursor()?;
    } else {
        hide_cursor()?;
    }

    draw_window(start_col, start_row, sidebar_width, DETAIL_HEIGHT, edit_mode)?;

    // Extract path and filename from location
    let location = match entry {
        Entry::Episode { location, .. } => location,
        _ => "",
    };
    let path = location.rsplitn(2, '/').nth(1).unwrap_or("");
    let filename = location.rsplitn(2, '/').next().unwrap_or("");

    let detail_lines = vec![
        format!("Path: {}", path),
        format!("Filename: {}", filename),
        format!("Title: {}", edit_details.title),
        format!("Year: {}", edit_details.year),
        format!("Watched: {}", edit_details.watched),
        format!("Length: {}", edit_details.length),
        format!("Series: {}", edit_details.series.as_ref().map_or(String::new(), |s| s.name.clone())),
        format!("Season: {}", edit_details.season.as_ref().map_or(String::new(), |s| s.number.to_string())),
        format!("Ep #: {}", edit_details.episode_number),
    ];

    let mut edit_cursor_min: usize = 0;

    for (i, line) in detail_lines.iter().enumerate() {
        if edit_mode && i == edit_field {
            let field_length = match edit_field {
                2 => edit_details.title.len(),
                3 => edit_details.year.len(),
                4 => edit_details.watched.len(),
                5 => edit_details.length.len(),
                8 => edit_details.episode_number.len(),
                _ => 0,
            };
            edit_cursor_min = line.len() - field_length;
            print_at(start_col + 1, start_row + 1 + i, &format!("{}", truncate_string(line, sidebar_width - 4)))?;
        } else {
            print_at(start_col + 1, start_row + 1 + i, &format!("{}", truncate_string(line, sidebar_width - 2)))?;
        }
    }
    // Put the cursor at the end of the current edit_field line
    if edit_mode {
        move_cursor(start_col + 1 + edit_cursor_min + edit_cursor_pos, start_row + 1 + edit_field)?;
    }

    Ok(())
}

fn draw_series_window(mode: &Mode, series: &Vec<Series>, new_series: &String, series_selection: &mut Option<usize>, config: &Config) -> io::Result<()> {
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

    draw_window(series_window_start_col, start_row, series_window_width, series_window_height, matches!(mode, Mode::SeriesCreate))?;

    // write the contents
    if let Mode::SeriesCreate = mode {
        show_cursor()?;
        print_at(series_window_start_col + 1, start_row + 1, &format!("{}", "Type the series name and press [ENTER]:".with(Color::Black).on(Color::White)))?;
        print_at(series_window_start_col + 1, start_row + 2, &format!("{}", new_series))?;
    } else {
        hide_cursor()?;
        print_at(series_window_start_col + 1, start_row + 1, &format!("{}", "Choose a series or [+] to create".with(Color::Black).on(Color::White)))?;
        for (i, series) in series.iter().enumerate() {
            let display_text = format!("[{}] {}", i + 1, truncate_string(&series.name, SERIES_WIDTH));
            let formatted_text = if Some(i) == *series_selection {
                format!("{}", display_text.with(string_to_fg_color_or_default(&config.current_fg)).on(string_to_bg_color_or_default(&config.current_bg)))
            } else {
                display_text
            };
            print_at(series_window_start_col + 1, start_row + 2 + i, &formatted_text)?;
        }
    }
    Ok(())
}

fn draw_window(left: usize, top: usize, width: usize, height: usize, thick: bool) -> io::Result<()> {
    // Choose border characters based on the thickness
    let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) = if thick {
        ('╔', '╗', '╚', '╝', '═', '║')
    } else {
        ('┌', '┐', '└', '┘', '─', '│')
    };

    // Draw top border
    print_at(left,top, &top_left)?;
    for _ in 1..width - 1 {
        print!("{}", horizontal);
    }
    print!("{}", top_right);

    // Draw side borders
    for row in (top + 1)..(top + height - 1) {
        print_at(left,row,&vertical)?;
        print_at(left+width-1,row, &vertical)?;
    }

    // Draw bottom border
    print_at(left,top + height - 1, &bottom_left)?;
    for _ in 1..width - 1 {
        print!("{}", horizontal);
    }
    println!("{}", bottom_right);

    Ok(())
}

pub fn load_videos(path: &str, count: usize) -> io::Result<()> {
    clear_line(HEADER_SIZE + 1)?;
    print!("Importing {} videos from {}...", count, path);
    Ok(())
}

pub fn get_max_displayed_items() -> io::Result<usize> {
    let (_, rows) = get_terminal_size()?;
    let max_lines = (rows  - HEADER_SIZE - FOOTER_SIZE - 1) as usize; // Adjust for header and footer lines
    Ok(max_lines)
}