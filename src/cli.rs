use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, default_value_t = 3, value_parser = clap::value_parser!(u8).range(1..=9))]
    /// The length of digit. Do not exceed 9.
    pub length: u8,
}
