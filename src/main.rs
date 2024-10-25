mod numeron;
mod cli;

use std::io;
use clap::Parser;
use numeron::Numeron;
use cli::Cli;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let numeron = Numeron::new(cli.length);
    numeron.run()?;

    Ok(())
}
