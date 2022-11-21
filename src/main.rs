use clap::Parser;

use wordle_cli;
use wordle_cli::Errors;

/// Play wordle in your terminal!
#[derive(Parser, Debug)]
struct Args {
    /// The text file containg the word list to use
    file_path: String,
}

fn main() {
    let args = Args::parse();

    match wordle_cli::play(args.file_path) {
        Ok(_) => (),
        Err(err) => {
            match err {
                Errors::NoWordsInFile => println!("No words were found in the file you selected!"),
                Errors::FailedToOpenFile => println!("Could not open the file you selected!"),
            }
            std::process::exit(1);
        }
    };
}
