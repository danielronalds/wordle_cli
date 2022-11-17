use std::io::Result;

use wordle_cli;

fn main() -> Result<()>  {
    wordle_cli::play()?;

    Ok(())
}
