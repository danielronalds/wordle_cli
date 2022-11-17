use std::cmp::Ordering;
use std::io::stdout;

use crossterm::{execute, style::Print};

use crate::letter::Letter;
use crate::letter::LetterState;

/// Enum for possible errors
#[derive(Debug)]
pub enum BuildErrors {
    NonAlphabeticCharcter,
    TooShortOfWord,
    TooLongOfWord,
    NonValidWord,
}

/// Struct to represent a wordle guess as a word
pub struct Word {
    letters: Vec<Letter>,
}

impl Word {
    /// Returns a Word Struct
    ///
    /// Parameters
    /// word:         The word the struct represents
    /// right_word:   The word being guessed
    /// words:        The wordlist the player is guessing from. Userd to check if the guess is a 
    ///               valid word
    pub fn new(word: String, right_word: &String, words: &Vec<String>) -> Result<Word, BuildErrors> {
        // Checks if the word is too short or too long, returning the appropriate error if it is
        match word.len().cmp(&5) {
            Ordering::Greater => return Err(BuildErrors::TooLongOfWord),
            Ordering::Less => return Err(BuildErrors::TooShortOfWord),
            _ => (),
        };

        let mut letters: Vec<Letter> = Vec::new();

        let right_word_letters: Vec<char> = right_word.chars().collect();
        let word_letters: Vec<char> = word.chars().collect();

        for i in 0..5 {
            // Catching any non alphabetic charcters and returning the correct error
            if !word_letters[i].is_alphabetic() {
                return Err(BuildErrors::NonAlphabeticCharcter);
            }
            // Choosing the right letter state for each letter
            if right_word_letters[i] == word_letters[i] {
                letters.push(Letter::new(
                    word_letters[i],
                    LetterState::RightLetterRightPlace,
                ));
            } else if right_word_letters.contains(&word_letters[i]) {
                letters.push(Letter::new(
                    word_letters[i],
                    LetterState::RightLetterWrongPlace,
                ));
            } else {
                letters.push(Letter::new(
                    word_letters[i],
                    LetterState::WrongLetterWrongPlace,
                ));
            }
        }

        Ok(Word { letters })
    }

    /// Prints the word to the console
    pub fn print(&self) {
        let mut display = String::new();

        for i in 0..3 {
            for letter in &self.letters {
                display.push_str(&letter.get_row(i).unwrap());
            }

            display.push_str("\n");
        }

        execute! {
            stdout(),
            Print(display)
        }
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Checks if the constructor returns a Word struct with the expected letters from the input
    /// word
    fn constructor_works() {
        // Wordlist to guess from
        let word_list = vec![
            String::from("crane"),
            String::from("guess"),
            String::from("juice"),
            String::from("spree"),
        ];

        let word = String::from("guess");
        let right_word = String::from("guess");

        let word_struct = Word::new(word.clone(), &right_word, &word_list).unwrap();

        let mut correct_word = false;

        for i in 0..word_struct.letters.len() {
            if word_struct.letters[i].letter() != &word.chars().nth(i).unwrap() {
                correct_word = false;
                break;
            }
            correct_word = true;
        }

        assert!(correct_word);
    }

    #[test]
    /// Checks if the constructor catches if the user has inputted numbers and returns the correct
    /// error
    fn constructor_error_on_numbers() {
        // Wordlist to guess from
        let word_list = vec![
            String::from("crane"),
            String::from("guess"),
            String::from("juice"),
            String::from("spree"),
        ];
        let word_struct = Word::new(String::from("w0rds"), &String::from("never"), &word_list);

        let correct_error;

        match word_struct {
            Ok(_) => correct_error = false,
            Err(err) => match err {
                BuildErrors::NonAlphabeticCharcter => correct_error = true,
                _ => correct_error = false,
            },
        };

        assert!(correct_error)
    }

    #[test]
    /// Checks if the constructor returns the correct error if the word passed has chacters that
    /// are not in the alphabet
    fn constructor_error_on_non_alphabetic_chars() {
        let word_list = vec![
            String::from("crane"),
            String::from("guess"),
            String::from("juice"),
            String::from("spree"),
        ];

        let word_struct = Word::new(String::from("w0rds"), &String::from("never"), &word_list);

        let correct_error;

        match word_struct {
            Ok(_) => correct_error = false,
            Err(err) => match err {
                BuildErrors::NonAlphabeticCharcter => correct_error = true,
                _ => correct_error = false,
            },
        };

        assert!(correct_error)
    }

    #[test]
    /// Checks if the constructor catches if the user has inputted too many words and returns the
    /// correct error
    fn constructor_error_on_long_word() {
        // Wordlist to guess from
        let word_list = vec![
            String::from("crane"),
            String::from("guess"),
            String::from("juice"),
            String::from("spree"),
        ];
        let word_struct = Word::new(String::from("spread"), &String::from("never"), &word_list);

        let correct_error;

        match word_struct {
            Ok(_) => correct_error = false,
            Err(err) => match err {
                BuildErrors::TooLongOfWord => correct_error = true,
                _ => correct_error = false,
            },
        };

        assert!(correct_error)
    }

    #[test]
    /// Checks if the constructor catches if the user has inputted too many words and returns the
    /// correct error
    fn constructor_error_on_short_word() {
        // Wordlist to guess from
        let word_list = vec![
            String::from("crane"),
            String::from("guess"),
            String::from("juice"),
            String::from("spree"),
        ];
        let word_struct = Word::new(String::from("tool"), &String::from("never"), &word_list);

        let correct_error;

        match word_struct {
            Ok(_) => correct_error = false,
            Err(err) => match err {
                BuildErrors::TooShortOfWord => correct_error = true,
                _ => correct_error = false,
            },
        };

        assert!(correct_error)
    }

    #[test]
    /// Checks if the constructor recognises words that are not valid guesses(Not in the word list)
    fn constructor_error_on_invalid_guess() {
        // Wordlist to guess from
        let word_list = vec![
            String::from("crane"),
            String::from("guess"),
            String::from("juice"),
            String::from("spree"),
        ];

        let word_struct = Word::new(String::from("plead"), &String::from("never"), &word_list);

        let correct_error;

        match word_struct {
            Ok(_) => correct_error = false,
            Err(err) => match err {
                BuildErrors::NonValidWord => correct_error = true,
                _ => correct_error = false,
            },
        };

        assert!(correct_error);
    }
}
