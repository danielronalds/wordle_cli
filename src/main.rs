use std::io;
use std::io::stdout;

use crossterm::{cursor, execute, style::Print, terminal, Result};

fn main() -> Result<()> {
    // using the macro
    execute!(
        stdout(),
        Print("Line 1!\n"),
        Print("Line 2!\n"),
        Print("Line 3!\n"),
    )?;

    io::stdin().read_line(&mut String::new()).unwrap();

    execute!(
        stdout(),
        cursor::MoveUp(4),
        terminal::Clear(terminal::ClearType::FromCursorDown),
        Print("Line 4!\n")
    )?;

    Ok(())
}
