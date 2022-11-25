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
    pub fn new(word: String, right_word: &str, words: &[String]) -> Result<Word, BuildErrors> {
        // Checks if the word is too short or too long, returning the appropriate error if it is
        match word.len().cmp(&5) {
            Ordering::Greater => return Err(BuildErrors::TooLongOfWord),
            Ordering::Less => return Err(BuildErrors::TooShortOfWord),
            _ => (),
        };

        if !words.contains(&word) {
            return Err(BuildErrors::NonValidWord);
        }

        let right_word_letters: Vec<char> = right_word.chars().collect();

        let mut letters: Vec<Letter> = Vec::new();

        let mut word_array: [Option<char>; 5] = [None; 5];
        let mut right_word_array: [Option<char>; 5] = [None; 5];
        let mut letter_array: [Option<Letter>; 5] = [None, None, None, None, None];

        // Collecting the submitted word into an array
        for i in 0..5 {
            word_array[i] = word.chars().nth(i);
        }
        for i in 0..5 {
            right_word_array[i] = right_word.chars().nth(i);
        }

        // Adding the correct letters and setting them to none in the base word array
        for i in 0..5 {
            if word_array[i] == Some(right_word_letters[i]) {
                letter_array[i] = Some(Letter::new(
                    right_word_letters[i],
                    LetterState::RightLetterRightPlace,
                ));

                word_array[i] = None;
                right_word_array[i] = None;
            }
        }

        // Adding the right word wrong place letters
        for i in 0..5 {
            match word_array[i] {
                Some(letter) => {
                    if right_word_array.contains(&Some(letter)) {
                        letter_array[i] = Some(Letter::new(
                            letter.clone(),
                            LetterState::RightLetterWrongPlace,
                        ));
                        word_array[i] = None;
                        let index_to_remove = right_word_array
                            .iter()
                            .position(|&r| r == Some(letter))
                            .unwrap();
                        right_word_array[index_to_remove] = None;
                    }
                }
                None => continue,
            }
        }

        // Adding the rest of the letters
        for i in 0..5 {
            match word_array[i] {
                Some(letter) => {
                    letter_array[i] = Some(Letter::new(letter, LetterState::WrongLetterWrongPlace));
                }
                None => continue,
            };
        }

        for letter in letter_array {
            letters.push(letter.unwrap());
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

            display.push('\n');
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
