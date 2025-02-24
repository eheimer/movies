use crossterm::{
    execute,
    terminal::{self, Clear, ClearType, size},
    cursor,
    style::{Color, Stylize},
    event::{DisableMouseCapture, EnableMouseCapture},
    ExecutableCommand,
};
use std::io::{self, stdout};
use crate::database::{get_entry_details, EntryDetails};
use crate::config::Config;
use crate::util::Entry;

const HEADER_SIZE: u16 = 3;
const FOOTER_SIZE: u16 = 0;
const COL1_WIDTH: usize = 45;
const MIN_COL2_WIDTH: usize = 20;
const COL2_HEIGHT: usize = 9;

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

pub fn initialize_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    stdout.execute(EnableMouseCapture)?;
    stdout.execute(cursor::Hide)?;
    Ok(())
}

pub fn restore_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    stdout.execute(DisableMouseCapture)?;
    stdout.execute(cursor::Show)?;
    Ok(())
}

fn get_terminal_size() -> io::Result<(u16, u16)> {
    let (cols, rows) = size()?;
    Ok((cols, rows))
}

pub fn draw_screen(
    entries: &[Entry],
    current_item: usize,
    first_entry: &mut usize,
    filter: &String,
    config: &Config,
    entry_mode: bool,
    entry_path: &String,
    edit_mode: bool,
    edit_details: &EntryDetails,
    edit_field: usize,
    edit_cursor_pos: usize,
) -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    println!("Use the arrow keys to navigate, 'Enter' play video, 'Esc' to exit, type to search");
    execute!(stdout, cursor::MoveTo(0, 1))?;
    println!("Filter: {}", filter);

    if entries.is_empty() {
        execute!(stdout, cursor::MoveTo(0, HEADER_SIZE))?;
        println!("{}", "No videos found, press CTRL-L to load videos from the file system".italic());
        if entry_mode {
          execute!(stdout, cursor::MoveTo(0, HEADER_SIZE + 1))?;
          println!("Enter a file path to scan: {}", entry_path);
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
            execute!(stdout, cursor::MoveTo(0, (i - *first_entry) as u16 + HEADER_SIZE))?;
            let display_text = match entry {
                Entry::Series { name, .. } => format!("[{}]", truncate_string(name,COL1_WIDTH)).with(Color::Blue),
                Entry::Episode { name, .. } => truncate_string(name,COL1_WIDTH).clone().stylize(),
            };

            if i == current_item {
                let styled_entry = display_text.with(string_to_fg_color_or_default(&config.current_fg)).on(string_to_bg_color_or_default(&config.current_bg));
                println!("{}", styled_entry);
            } else {
                println!("{}", display_text);
            }
        }
        draw_sidebar(&entries[current_item], edit_mode, edit_details, edit_field, edit_cursor_pos)?;
    }

    Ok(())
}

fn draw_sidebar(entry: &Entry, edit_mode: bool, edit_details: &EntryDetails, edit_field: usize, edit_cursor_pos: usize) -> io::Result<()> {
    let mut stdout = stdout();
    let start_col: u16 = COL1_WIDTH as u16 + 2;
    let start_row = HEADER_SIZE;
    let sidebar_width = get_sidebar_width()?;

    // Show or hide the cursor based on edit_mode
    if edit_mode {
        execute!(stdout, cursor::Show)?;
    } else {
        execute!(stdout, cursor::Hide)?;
    }

    // Draw top border
    execute!(stdout, cursor::MoveTo(start_col, start_row))?;
    print!("+");
    for _ in 1..sidebar_width - 1 {
        print!("-");
    }
    println!("+");

    // Draw side borders
    for row in (start_row + 1)..(start_row + COL2_HEIGHT as u16 - 1) {
        execute!(stdout, cursor::MoveTo(start_col, row))?;
        print!("|");
        execute!(stdout, cursor::MoveTo(start_col + sidebar_width as u16 - 1, row))?;
        println!("|");
    }

    // Draw bottom border
    execute!(stdout, cursor::MoveTo(start_col, start_row + COL2_HEIGHT as u16 - 1))?;
    print!("+");
    for _ in 1..sidebar_width - 1 {
        print!("-");
    }
    println!("+");

    // Display details inside the sidebar
    let details: EntryDetails = if edit_mode { edit_details.clone() } else { get_entry_details(entry).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))? };
    let detail_lines = vec![
        format!("Title: {}", details.title),
        format!("Year: {}", details.year),
        format!("Watched: {}", details.watched),
        format!("Length: {}", details.length),
        format!("Series: {}", details.series),
        format!("Season: {}", details.season),
        format!("Ep #: {}", details.episode_number),
    ];

    let mut edit_cursor_min: usize = 0;

    for (i, line) in detail_lines.iter().enumerate() {
        execute!(stdout, cursor::MoveTo(start_col + 1, start_row + 1 + i as u16))?;
        if edit_mode && i == edit_field {
            let field_length = match edit_field {
                0 => details.title.len(),
                1 => details.year.len(),
                2 => details.watched.len(),
                3 => details.length.len(),
                4 => details.series.len(),
                5 => details.season.len(),
                6 => details.episode_number.len(),
                _ => 0,
            };
            edit_cursor_min = line.len() - field_length;
            print!("{}", truncate_string(line, sidebar_width - 4));
        } else {
            println!("{}", truncate_string(line, sidebar_width - 2));
        }
    }
    //put the cursor at the end of the current edit_field line
    if edit_mode {
        execute!(stdout, cursor::MoveTo(start_col + 1 + edit_cursor_min as u16 + edit_cursor_pos as u16, start_row + 1 + edit_field as u16))?;
    }

    Ok(())
}
            
fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() > max_length {
        format!("{}...", &s[..max_length - 3])
    } else {
        s.to_string()
    }
}

pub fn load_videos(path: &str, count: usize) -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, cursor::MoveTo(0, HEADER_SIZE + 1), Clear(ClearType::CurrentLine))?;
    println!("Importing {} videos from {}...", count, path);
    Ok(())
}

fn get_max_displayed_items() -> io::Result<usize> {
    let (_, rows) = get_terminal_size()?;
    let max_lines = (rows  - HEADER_SIZE - FOOTER_SIZE - 1) as usize; // Adjust for header and footer lines
    Ok(max_lines)
}

pub fn page_up(current_item: &mut usize, first_entry: &mut usize) -> io::Result<()> {
    let max_lines = get_max_displayed_items()?;
    if *current_item > *first_entry {
        *current_item = *first_entry;
    } else {
        *current_item = (*current_item).saturating_sub(max_lines);
    }
    Ok(())
}

pub fn page_down(current_item: &mut usize, first_entry: &mut usize) -> io::Result<()> {
    let max_lines = get_max_displayed_items()?;
    if *current_item < *first_entry + max_lines - 1 {
        *current_item = *first_entry + max_lines - 1;
    } else {
        *current_item = (*current_item).saturating_add(max_lines);
    }
    Ok(())
}