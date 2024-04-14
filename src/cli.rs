use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, default_value_t = 3)]
    pub length: usize,
}
