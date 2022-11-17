use std::io;
use std::io::stdout;

use wordle_cli::word::Word;

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

    for word in &words {
        word.print();
    }

    if words.len() < 6 {
        let boxes_left = 6 - words.len();

        if boxes_left > 0 {
            for _i in 0..boxes_left {
                print_blank_boxes();
            }
        }
    }

    Ok(())
}

fn print_blank_boxes() {
    execute!(
        stdout(),
        Print("╭───╮╭───╮╭───╮╭───╮╭───╮\n"),
        Print("│   ││   ││   ││   ││   │\n"),
        Print("╰───╯╰───╯╰───╯╰───╯╰───╯\n"),
    ).unwrap();
}
