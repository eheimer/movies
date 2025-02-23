use crossterm::{
    execute,
    terminal::{self, ClearType, size},
    cursor,
    style::{Color, Stylize},
    event::{DisableMouseCapture, EnableMouseCapture},
    ExecutableCommand,
};
use std::io::{self, stdout};
use crate::config::Config;
use crate::util::Entry;

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

pub fn get_terminal_size() -> io::Result<(u16, u16)> {
    let (cols, rows) = size()?;
    Ok((cols, rows))
}

pub fn draw_screen(
    entries: &[Entry],
    current_item: usize,
    filter: &String,
    config: &Config,
    window_start: usize,
    playing_file: Option<&String>,
    entry_mode: bool,
    entry_path: &String,
) -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    println!("Use the arrow keys to navigate, 'Enter' play video, 'Esc' to exit, type to search");
    execute!(stdout, cursor::MoveTo(0, 2))?;
    if let Some(file) = playing_file {
        println!("Playing video: {}", file);
    }
    execute!(stdout, cursor::MoveTo(0, 3))?;
    println!("Filter: {}", filter);

    if entries.is_empty() {
        execute!(stdout, cursor::MoveTo(0, 5))?;
        println!("{}", "No videos found, press CTRL-S to scan for files".italic());
        if entry_mode {
          execute!(stdout, cursor::MoveTo(0, 6))?;
          println!("Enter a file path to scan: {}", entry_path);
        }
    } else {
        let (_, rows) = get_terminal_size()?;
        let max_lines = rows as usize - 6; // Adjust for header and footer lines

        for (i, entry) in entries.iter().enumerate().skip(window_start).take(max_lines) {
            execute!(stdout, cursor::MoveTo(0, (i - window_start) as u16 + 5))?;
            let display_text = match entry {
                Entry::Series { name, .. } => format!("[{}]", name).with(Color::Blue),
                Entry::Episode { name, .. } => name.clone().stylize(),
            };
            if i == current_item {
                let styled_entry = display_text.with(string_to_fg_color_or_default(&config.current_fg)).on(string_to_bg_color_or_default(&config.current_bg));
                println!("{}", styled_entry);
            } else {
                println!("{}", display_text);
            }
        }
    }

    Ok(())
}