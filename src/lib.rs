pub mod letter;
pub mod word;

use word::Word;

use std::io::stdout;
use crossterm::{execute, style::Print};

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
    ).unwrap();
}

