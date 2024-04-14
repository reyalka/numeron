mod numeron;
mod cli;

use std::error::Error;
use clap::Parser;
use numeron::Numeron;
use cli::Cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let numeron = Numeron::new(cli.length);
    numeron.run()?;

    Ok(())
}
