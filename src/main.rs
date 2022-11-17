use std::io::stdout;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use rand::Rng;

use wordle_cli;
use wordle_cli::word::BuildErrors;
use wordle_cli::word::Word;

use crossterm::{cursor, execute, terminal, Result};

fn main() -> Result<()> {

    let words_to_guess = lines_from_file("/home/danielronalds/Documents/rust/wordle_cli/wordle_words.txt")?;

    let word_to_guess = random_word(words_to_guess);

    println!("{}", &word_to_guess);

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

/// Reads the lines from an input file into a Vec of strings
///
/// Parameters:
/// filename:   The path of the filename
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn random_word(word_list: Vec<String>) -> String {
    let random_index: usize = rand::thread_rng().gen_range(0..word_list.len());

    word_list[random_index].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Checks if the random_word function generates two random words. This test will sometimes
    /// fail due to the small sample size
    fn random_word_generated() {
        let word_list = vec![
            String::from("three"),
            String::from("potty"),
            String::from("there"),
            String::from("panic"),
            String::from("rusty"),
            String::from("sjkdl"),
            String::from("fjslk"),
            String::from("paosd"),
            String::from("asdff"),
            String::from("asdfa"),
            String::from("vbsoc"),
            String::from("adlaf"),
            String::from("askdl"),
            String::from("fyudi"),
        ];
        let word_one = random_word(word_list.clone());
        let word_two = random_word(word_list);

        assert!(!(word_one == word_two))
    }
}
