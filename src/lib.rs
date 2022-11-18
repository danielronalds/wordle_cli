pub mod letter;
pub mod word;

use word::BuildErrors;
use word::Word;

use rand::Rng;
use std::fs::File;
use std::io::stdout;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crossterm::{cursor, execute, style::Print, terminal, Result};

/// Plays the game
pub fn play(file_path: String) -> Result<()> {
    let words_to_guess =
        lines_from_file(file_path)?;

    let word_to_guess = random_word(&words_to_guess);

    println!("{}", &word_to_guess);

    let mut guesses: Vec<Word> = Vec::new();

    let mut game_over = false;

    while !game_over {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::FromCursorDown)
        )
        .unwrap();

        display_game_state(&guesses);

        execute!(stdout(), Print("> "),)?;

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read the line");

        let guess = guess.trim().to_string();

        if guess == word_to_guess {
            game_over = true;
        }

        let guessed_word = Word::new(guess, &word_to_guess, &words_to_guess);

        match guessed_word {
            Ok(word) => guesses.push(word),
            Err(err) => {
                match err {
                    BuildErrors::TooLongOfWord => {
                        println!("Words cannot be longer than 5 letters!")
                    }
                    BuildErrors::TooShortOfWord => {
                        println!("Words cannot be shorter than 5 letters!")
                    }
                    BuildErrors::NonAlphabeticCharcter => {
                        println!("Words can only contain alphabetic characters!")
                    }
                    BuildErrors::NonValidWord => {
                        println!("That is not a valid guess!")
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

            display_game_state(&guesses);
        }
    }

    Ok(())
}

pub fn display_game_state(guesses: &Vec<Word>) {
    for word in guesses {
        word.print();
    }

    if guesses.len() < 6 {
        let boxes_left = 6 - guesses.len();

        if boxes_left > 0 {
            for _i in 0..boxes_left {
                print_blank_boxes();
            }
        }
    }
}

fn print_blank_boxes() {
    execute!(
        stdout(),
        Print("╭───╮╭───╮╭───╮╭───╮╭───╮\n"),
        Print("│   ││   ││   ││   ││   │\n"),
        Print("╰───╯╰───╯╰───╯╰───╯╰───╯\n"),
    )
    .unwrap();
}

/// Reads the lines from an input file into a Vec of strings
///
/// Parameters:
/// filename:   The path of the filename
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn random_word(word_list: &Vec<String>) -> String {
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
        let word_one = random_word(&word_list);
        let word_two = random_word(&word_list);

        assert!(!(word_one == word_two))
    }
}
