use std::io;
use std::io::stdout;

use wordle_cli;
use wordle_cli::word::BuildErrors;
use wordle_cli::word::Word;

use crossterm::{cursor, execute, style::Print, terminal, Result};

fn main() -> Result<()> {
    let word_to_guess = String::from("there");

    let mut guesses: Vec<Word> = Vec::new();

    let mut game_over = false;

    while !game_over {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::FromCursorDown)
        )
        .unwrap();

        wordle_cli::display_game_state(&guesses);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read the line");

        let guess = guess.trim().to_string();

        if guess == word_to_guess {
            game_over = true;
        }

        let guessed_word = Word::new(guess, &word_to_guess);

        match guessed_word {
            Ok(word) => guesses.push(word),
            Err(err) => {
                match err {
                    BuildErrors::TooLongOfWord => println!("Words cannot be longer than 5 letters"),
                    BuildErrors::TooShortOfWord => {
                        println!("Words cannot be shorter than 5 letters")
                    }
                    BuildErrors::NonAlphabeticCharcter => {
                        println!("Words can only contain alphabetic characters!")
                    }
                }

                io::stdin()
                    .read_line(&mut String::new())
                    .expect("Could not read the line");
                execute!(stdout(), cursor::MoveUp(2))?;
            }
        }

        execute!(stdout(), cursor::MoveUp(19))?;

        if game_over {
            execute!(
                stdout(),
                terminal::Clear(terminal::ClearType::FromCursorDown)
            )
            .unwrap();

            wordle_cli::display_game_state(&guesses);
        }
    }

    Ok(())
}
