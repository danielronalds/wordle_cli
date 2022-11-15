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
        // Option 1: Box with square edges and lowercase letters
        Print("┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐\n"),
        Print("│ w ││ o ││ r ││ d ││ l ││ e │\n"),
        Print("└───┘└───┘└───┘└───┘└───┘└───┘\n"),
        // Option 2: Box with square edges and uppercase letters
        Print("┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐\n"),
        Print("│ W ││ O ││ R ││ D ││ L ││ E │\n"),
        Print("└───┘└───┘└───┘└───┘└───┘└───┘\n"),
        // Option 3: Box with round edges and lowercase letters
        Print("╭───╮╭───╮╭───╮╭───╮╭───╮╭───╮\n"),
        Print("│ w ││ o ││ r ││ d ││ l ││ e │\n"),
        Print("╰───╯╰───╯╰───╯╰───╯╰───╯╰───╯\n"),
        // Option 4: Box with round edges and uppercase letters
        Print("╭───╮╭───╮╭───╮╭───╮╭───╮╭───╮\n"),
        Print("│ W ││ O ││ R ││ D ││ L ││ E │\n"),
        Print("╰───╯╰───╯╰───╯╰───╯╰───╯╰───╯\n"),
    )?;

    Ok(())
}
