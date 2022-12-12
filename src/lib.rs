pub mod letter;
pub mod word;

use word::BuildErrors;
use word::Word;

use rand::Rng;

use std::fs::File;
use std::io::stdout;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crossterm::{cursor, execute, style::Print, terminal};

use colored::Colorize;

const MAX_GUESSES: usize = 6;

/// Enum for possible errors
#[derive(Debug)]
pub enum Errors {
    FailedToOpenFile,
    NoWordsInFile,
}

/// Plays the game
///
/// Parameters
/// wordfile:    The path to the word file to choose the random word from
/// show_word:   Whether to print the word to guess
pub fn play(wordfile: String, show_word: bool) -> Result<(), Errors> {
    // Getting the list of words to choose from out of the file supplied
    let words_to_guess = lines_from_file(wordfile)?;

    // Choosing a random word
    let word_to_guess = random_word(&words_to_guess);

    // Printing out the word for testing purposes
    if show_word {
        println!("{}", &word_to_guess.bold());
    }

    // Instantiating a vec to store the players guesses
    let mut guesses: Vec<Word> = Vec::new();

    let mut game_over = false;

    while !game_over {
        // Clearing the screen from the cursor down. We don't move the cursor here as if we do it
        // here the program would wipe lines above the cli game being player
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::FromCursorDown)
        )
        .unwrap(); // This is probably unsafe...

        // Displaying the word grid to the console
        display_game_state(&guesses);

        // Prompt for the user to type their guess
        execute!(stdout(), Print("> "),).unwrap();

        // Getting the users input from the player and trimming it
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read the line");
        let guess = guess.trim().to_string();

        // If the player guessed the word, then the game is over
        if guess == word_to_guess {
            game_over = true;
        }

        // Creating a Word struct to store the players guess
        let guessed_word = Word::new(guess, &word_to_guess, &words_to_guess);

        // Adding the gussed word to the guesses vec if it was created successfully, otherwise
        // printing an appropriate error message for the player
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

                // Pausing for the player to be able to read the message
                io::stdin()
                    .read_line(&mut String::new())
                    .expect("Could not read the line");
                // Moves the cursor back the lines we just created
                execute!(stdout(), cursor::MoveUp(2)).unwrap();
            }
        }

        // Move the cursor back to the saved position in prep for clearing the screen
        execute!(stdout(), cursor::MoveUp(19)).unwrap();

        // If the player has had more than the max guesses then the game is also over
        if guesses.len() >= MAX_GUESSES {
            game_over = true;
        }

        // If the game is over, then the loop ends and the user wont see the word grid with their
        // correct guess, so we print it here
        if game_over {
            execute!(
                stdout(),
                terminal::Clear(terminal::ClearType::FromCursorDown)
            )
            .unwrap();

            display_game_state(&guesses);

            println!("The word was {}", word_to_guess.bold());
        }
    }

    Ok(())
}

/// Prints the current word grid to the console
///
/// Parameters
/// guesses:    The Vec containg the players guesses
pub fn display_game_state(guesses: &Vec<Word>) {
    for word in guesses {
        word.print();
    }

    if guesses.len() < MAX_GUESSES {
        let boxes_left = MAX_GUESSES - guesses.len();

        if boxes_left > 0 {
            for _i in 0..boxes_left {
                print_blank_boxes();
            }
        }
    }
}

/// Prints a blank word to the console, used so that there is always a 5x6 grid on the screen
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
fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>, Errors> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err(Errors::FailedToOpenFile),
    };

    let mut lines: Vec<String> = Vec::new();

    for line in BufReader::new(file).lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(_) => continue,
        }
    }

    if lines.is_empty() {
        return Err(Errors::NoWordsInFile);
    }

    Ok(lines)
}

/// Chooses a random word
///
/// Parameters
/// word_list:  The vec of String to choose a word from
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
