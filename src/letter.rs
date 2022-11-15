pub enum PrintStyle {
    Box,
    Round,
}

/// Struct to represent a Letter in a wordle guess
pub struct Letter {
    letter: char,
    rows: Vec<String>,
}

impl Letter {
    /// Returns a Letter struct
    ///
    /// Parameters
    /// letter:   The letter the struct represents
    pub fn new(letter: char) -> Letter {
        // Creating the vec to store the rows of the letters
        let mut rows: Vec<String> = Vec::new();

        rows.push(String::from("╭───╮"));
        rows.push(format!("│ {} │", letter.clone()));
        rows.push(String::from("╰───╯"));

        Letter { letter, rows }
    }

    /// Returns the char the Letter struct represents
    pub fn letter(&self) -> &char {
        &self.letter
    }

    /// Returns a string containg the corresponding row of the letter
    ///
    /// Parameters
    /// index:    The row to get(Between 0-2)
    pub fn get_row(&self, index: usize) -> Result<String, &'static str> {
        // Returning an error if the index is greater the the num of rows to prevent a panic
        if index > self.rows.len() {
            return Err("That row does not exist!");
        }

        Ok(self.rows[index].clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Test for checking if the Letter struct is generated correctly
    fn new_returns_correct_struct() {
        let letter_struct = Letter::new('a');
        
        assert!(letter_struct.letter() == &'a');
    }

    #[test]
    /// Test to see if the get_row() function returns the correct row one
    fn get_row_returns_right_row_one() {
        let letter_struct = Letter::new('w');

        let row_one = letter_struct.get_row(0);

        assert!(row_one.unwrap() == String::from("╭───╮"))
    }

    #[test]
    /// Test to see if the get_row() function returns the correct row two
    fn get_row_returns_right_row_two() {
        let letter_struct = Letter::new('w');

        let row_one = letter_struct.get_row(1);

        assert!(row_one.unwrap() == String::from("│ w │"))
    }

    #[test]
    /// Test to see if the get_row() function returns the correct row three
    fn get_row_returns_right_row_three() {
        let letter_struct = Letter::new('w');

        let row_one = letter_struct.get_row(2);

        assert!(row_one.unwrap() == String::from("╰───╯"))
    }
}
