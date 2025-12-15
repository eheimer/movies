//! ASCII splash screen module for the movies application
use crossterm::{
    cursor,
    event::{self, Event},
    execute,
    terminal,
};
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// ASCII art representation of "movies" in script font style
const ASCII_ART: &str = r#"                                                           ██                         ██████
     ████                                                  ██                       ███   ██
  █ ██████              ██████        ███        █                     ███        ████    ██
██████ ███ ████        ████ ███       ███       ███       ███       ███████       ███     ██
██████ ████████       ███ █████       ████     █████     ████      ████  ██       ████            ████
█████  █████████     ████  ███████████████     ██  ██████████      ███████       ███████      ██████
█████  ████ ███     █████    ███ ██    ███    ███        ████     ████          ███  ███████████
████   ███  ███    ██████    ██        ███    ██         ████    █████        ███      █████
████   ███  ███    ██ ███    ██        ████   ██         ████   ██████       ███     ████████
████        ███  ███   ███████          ███  ██           ███  ███  ████ █████     ███    ████
███         ██████       ███             ██████           ███ ███     █████       ███     ███
             ███                           ██               ███                   ██     ███
                                                                                  █████████
                -- written by Eric Heimerman (with a little help from Kiro)         ███"#;

/// Press any key prompt
const PRESS_KEY_PROMPT: &str = "Press any key";

/// Returns the ASCII art for the splash screen
pub fn get_ascii_art() -> &'static str {
    ASCII_ART
}

/// Renders the splash screen with ASCII art and press key prompt
pub fn render_splash(
    ascii_art: &str,
    press_key_prompt: &str,
    terminal_width: u16,
    terminal_height: u16,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    
    // Calculate vertical position (20% from top)
    let vertical_offset = (terminal_height as f32 * 0.2) as u16;
    
    // Split ASCII art into lines
    let art_lines: Vec<&str> = ascii_art.lines().collect();
    
    // Find the maximum display width to center the art as a block
    // Use unicode_width to properly handle UTF-8 characters like █
    let max_art_width = art_lines.iter().map(|line| line.width()).max().unwrap_or(0);
    
    // Calculate left padding to center the entire ASCII art block
    let art_left_padding = if max_art_width < terminal_width as usize {
        (terminal_width as usize - max_art_width) / 2
    } else {
        0
    };
    
    // Render ASCII art lines with consistent left padding
    for (i, line) in art_lines.iter().enumerate() {
        let row = vertical_offset + i as u16;
        let padded_line = format!("{}{}", " ".repeat(art_left_padding), line);
        execute!(
            stdout,
            cursor::MoveTo(0, row),
            crossterm::style::Print(&padded_line)
        )?;
    }
    
    // Position "Press any key" prompt 5 lines below ASCII art, centered
    let prompt_row = vertical_offset + art_lines.len() as u16 + 5;
    let prompt_len = press_key_prompt.len();
    let prompt_padding = if prompt_len < terminal_width as usize {
        (terminal_width as usize - prompt_len) / 2
    } else {
        0
    };
    
    let centered_prompt = format!("{}{}", " ".repeat(prompt_padding), press_key_prompt);
    execute!(
        stdout,
        cursor::MoveTo(0, prompt_row),
        crossterm::style::Print(&centered_prompt)
    )?;
    
    stdout.flush()?;
    Ok(())
}

/// Displays the splash screen and waits for user input
pub fn show_splash_screen() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    
    // Clear terminal and hide cursor
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide
    )?;
    
    // Get terminal dimensions with fallback to default (80x24)
    let (terminal_width, terminal_height) = terminal::size().unwrap_or((80, 24));
    
    // Render the splash screen
    render_splash(
        get_ascii_art(),
        PRESS_KEY_PROMPT,
        terminal_width,
        terminal_height,
    )?;
    
    // Wait for any key press
    loop {
        if let Event::Key(_) = event::read()? {
            break;
        }
    }
    
    Ok(())
}
