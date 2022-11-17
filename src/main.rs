use std::io;
use std::io::stdout;

use wordle_cli::word::Word;

use crossterm::{cursor, execute, style::Print, terminal, Result};

fn main() -> Result<()> {
    // using the macro
    execute!(
        stdout(),
        Print("╭───╮╭───╮╭───╮╭───╮╭───╮╭───╮\n"),
        Print("│ W ││ O ││ R ││ D ││ L ││ E │\n"),
        Print("╰───╯╰───╯╰───╯╰───╯╰───╯╰───╯\n"),
        Print("\nPress any key to start"),
    )?;

    // Waiting line
    io::stdin().read_line(&mut String::new()).unwrap();

    let word_to_guess = String::from("there");

    let word1 = Word::new(String::from("crane"), &word_to_guess).unwrap();
    let word2 = Word::new(String::from("horse"), &word_to_guess).unwrap();
    let word3 = Word::new(String::from("shore"), &word_to_guess).unwrap();
    let word4 = Word::new(String::from("there"), &word_to_guess).unwrap();

    let words = vec![word1, word2, word3, word4];

    execute!(
        stdout(),
        cursor::MoveUp(5),
        terminal::Clear(terminal::ClearType::FromCursorDown),
    )?;

    for word in words {
        word.print();
    }

    Ok(())
}
