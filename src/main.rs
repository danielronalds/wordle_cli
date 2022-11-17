use std::io;
use std::io::stdout;

use wordle_cli::word::Word;
use wordle_cli;

use crossterm::{cursor, execute, style::Print, terminal, Result};

fn main() -> Result<()> {
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

    wordle_cli::display_game_state(words);

    Ok(())
}

