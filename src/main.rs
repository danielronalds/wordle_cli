use clap::Parser;

use wordle_cli;

/// Play wordle in your terminal!
#[derive(Parser, Debug)]
struct Args {
    /// The text file containg the word list to use
    file_path: String, 
}

fn main() -> Result<(), wordle_cli::Errors>  {
    let args = Args::parse();

    wordle_cli::play(args.file_path)?;

    Ok(())
}
