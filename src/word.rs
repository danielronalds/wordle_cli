use crate::letter::Letter;

/// Struct to represent a wordle guess as a word
pub struct Word {
    letters: Vec<Letter>
}

impl Word {
    /// Returns a Word Struct
    ///
    /// Parameters
    /// word:   The word the struct represents
    pub fn new(word: String) -> Word {
        let mut letters: Vec<Letter> = Vec::new();

        for letter in word.chars() {
            letters.push(Letter::new(letter));
        }

        Word { letters }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test for checking if the build function returns a Word struct with the expected letters
    fn new_struct_works() {
        let word = String::from("guess");

        let word_struct = Word::new(word.clone());

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
}
