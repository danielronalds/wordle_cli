use crate::letter::Letter;
use std::cmp::Ordering;

/// Enum for possible errors
#[derive(Debug)]
pub enum BuildErrors {
    NonAlphabeticCharcter,
    TooLongOfWord,
    TooShortOfWord,
}

/// Struct to represent a wordle guess as a word
pub struct Word {
    letters: Vec<Letter>,
}

impl Word {
    /// Returns a Word Struct
    ///
    /// Parameters
    /// word:   The word the struct represents
    pub fn new(word: String) -> Result<Word, BuildErrors> {
        // Checks if the word is too short or too long, returning the appropriate error if it is
        match word.len().cmp(&5) {
            Ordering::Greater => return Err(BuildErrors::TooLongOfWord),
            Ordering::Less => return Err(BuildErrors::TooShortOfWord),
            _ => (),
        };

        let mut letters: Vec<Letter> = Vec::new();

        for letter in word.chars() {
            // If the char is not alphabetic, return the appropriate error
            if !letter.is_alphabetic() {
                return Err(BuildErrors::NonAlphabeticCharcter);
            }

            letters.push(Letter::new(letter));
        }

        Ok(Word { letters })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Checks if the constructor returns a Word struct with the expected letters from the input
    /// word
    fn constructor_works() {
        let word = String::from("guess");

        let word_struct = Word::new(word.clone()).unwrap();

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
        let word_struct = Word::new(String::from("w0rds"));

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
        let word_struct = Word::new(String::from("w0rds"));

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
        let word_struct = Word::new(String::from("spread"));

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
        let word_struct = Word::new(String::from("tool"));

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
}
