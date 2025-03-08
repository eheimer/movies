use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, size, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, stdout, Write};

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
