use clap::Parser;

use wordle_cli::Errors;

#[derive(Parser, Debug)]
/// Play wordle in your terminal!
#[clap(author, version, about)]
pub struct Args {
    /// The text file containg the word list to use
    file_path: String,

    #[arg(long)]
    /// Whether to show the word to guess or not
    show_word: bool,
}

fn main() {
    let args = Args::parse();

    match wordle_cli::play(args.file_path, args.show_word) {
        Ok(_) => (),
        Err(err) => match err {
            Errors::NoWordsInFile => println!("No words were found in the file you selected!"),
            Errors::FailedToOpenFile => println!("Could not open the file you selected!"),
        },
    };
}
