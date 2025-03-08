use crate::config::Config;
use crate::dto::{EpisodeDetail, Series};
use crate::episode_field::EpisodeField;
use crate::terminal::{
    clear_line, clear_screen, get_terminal_size, hide_cursor, move_cursor, print_at, show_cursor,
};
use crate::util::{truncate_string, Entry, Mode};
use crossterm::style::{Color, Stylize};
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
) -> io::Result<()> {
    let instructions: Vec<&str> = match mode {
        Mode::Browse => {
            if series_selected {
                vec![
                    "type to filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] show episodes, [ESC] exit",
                ]
            } else if season_selected {
                vec![
                    "type to filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] show episodes, [ESC] back",
                ]
            } else if series_filter_active {
                vec![
                    "type to filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] play, [ESC] back",
                    "[F2] edit, [F3] toggle watched, [F4] assign to series",
                ]
            } else {
                vec![
                    "type to filter, [\u{2191}]/[\u{2193}] navigate, [ENTER] play, [ESC] exit",
                    "[F2] edit, [F3] toggle watched, [F4] assign to series",
                ]
            }
        }
        Mode::Edit => vec!["[\u{2191}]/[\u{2193}] change field, [ESC] cancel, [F2] save changes"],
        Mode::Entry => vec!["Enter a file path to scan, [ESC] cancel"],
        Mode::SeriesSelect => vec![
            "[\u{2191}]/[\u{2193}] navigate, [ENTER] select, [ESC] cancel",
            "[+] create a new series, [CTRL][-] deselect series",
        ],
        Mode::SeriesCreate => vec!["Type a series name, [ENTER] save, [ESC] cancel"],
    };
    //loop through the instructions and print them in the header
    for (i, instructions) in instructions.iter().enumerate() {
        print_at(1, i, &instructions.with(Color::Black).on(Color::White))?;
    }

    print_at(0, 3, &format!("filter: {}", filter))?;
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

    draw_header(
        mode,
        filter,
        series_selected,
        season_selected,
        series_filter_active,
    )?;

    if entries.is_empty() {
        print_at(
            0,
            HEADER_SIZE,
            &format!(
                "{}",
                "No videos found. Adjust your filter or press CTRL-L to scan the file system"
                    .italic()
            ),
        )?;
        if let Mode::Entry = mode {
            print_at(
                0,
                HEADER_SIZE + 1,
                &format!("Enter a file path to scan: {}", entry_path),
            )?;
        }
    } else {
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
            if i == current_item {
                formatted_text = format!(
                    "{}",
                    display_text
                        .with(string_to_fg_color_or_default(&config.current_fg))
                        .on(string_to_bg_color_or_default(&config.current_bg))
                );
            }
            print_at(0, i - *first_entry + HEADER_SIZE, &formatted_text)?;
        }
        if !series_selected && !season_selected {
            draw_detail_window(
                &entries[current_item],
                mode,
                edit_details,
                edit_field,
                edit_cursor_pos,
                season_number,
            )?;
        }
        if let Mode::SeriesSelect | Mode::SeriesCreate = mode {
            draw_series_window(mode, series, new_series, series_selection, config)?;
        }
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

    let mut detail_lines = Vec::new();

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
        detail_lines.push(format!("{}: {}", field.display_name(), value));
    }

    let mut edit_cursor_min: usize = 0;
    if edit_mode && edit_field.is_editable() {
        edit_cursor_min = edit_field.display_name().len() + 2;
    }

    for (i, line) in detail_lines.iter().enumerate() {
        if edit_mode && edit_field.is_editable() {
            print_at(
                start_col + 1,
                start_row + 1 + i,
                &truncate_string(line, sidebar_width - 4).to_string(),
            )?;
        } else {
            print_at(
                start_col + 1,
                start_row + 1 + i,
                &truncate_string(line, sidebar_width - 2).to_string(),
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
        for (i, series) in series.iter().enumerate() {
            let display_text = format!(
                "[{}] {}",
                i + 1,
                truncate_string(&series.name, SERIES_WIDTH)
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
            print_at(
                series_window_start_col + 1,
                start_row + 2 + i,
                &formatted_text,
            )?;
        }
    }
    Ok(())
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

    // Draw side borders
    for row in (top + 1)..(top + height - 1) {
        print_at(left, row, &vertical)?;
        print_at(left + width - 1, row, &vertical)?;
    }

    // Draw bottom border
    print_at(left, top + height - 1, &bottom_left)?;
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
    let max_lines = rows - HEADER_SIZE - FOOTER_SIZE - 1; // Adjust for header and footer lines
    Ok(max_lines)
}
