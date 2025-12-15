use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, size, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, stdout, Write};

pub fn initialize_terminal() -> io::Result<()> {
    // Request terminal resize before entering alternate screen
    // Target size: 30 rows x 110 columns
    request_terminal_resize(30, 110)?;
    
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

pub fn get_terminal_size() -> io::Result<(usize, usize)> {
    let (cols, rows) = size()?;
    Ok((cols as usize, rows as usize))
}

pub fn clear_screen() -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;
    Ok(())
}

pub fn clear_line(row: usize) -> io::Result<()> {
    move_cursor(0, row)?;
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::CurrentLine))?;
    Ok(())
}

pub fn hide_cursor() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide)?;
    Ok(())
}

pub fn show_cursor() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(cursor::Show)?;
    Ok(())
}

pub fn move_cursor(col: usize, row: usize) -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, cursor::MoveTo(col as u16, row as u16))?;
    Ok(())
}

pub fn print_at(col: usize, row: usize, text: &dyn std::fmt::Display) -> io::Result<()> {
    move_cursor(col, row)?;
    print!("{}", text);
    flush_stdout()?;
    Ok(())
}

pub fn flush_stdout() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.flush()?;
    Ok(())
}

/// Request terminal resize to specified dimensions if current size is smaller
/// Uses ANSI escape sequence that may not work on all terminals - fails silently
pub fn request_terminal_resize(target_rows: u16, target_cols: u16) -> io::Result<()> {
    // Get current terminal size
    let (current_cols, current_rows) = size()?;
    
    // Only resize if current terminal is smaller than target in either dimension
    if current_rows < target_rows || current_cols < target_cols {
        // Use the larger of current or target for each dimension
        let new_rows = current_rows.max(target_rows);
        let new_cols = current_cols.max(target_cols);
        
        // Send ANSI escape sequence to request terminal resize
        // Format: \x1b[8;{rows};{cols}t
        print!("\x1b[8;{};{}t", new_rows, new_cols);
        flush_stdout()?;
    }
    
    Ok(())
}
