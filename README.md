# Numeron

Numeron is a command-line game implemented in Rust. The game generates a random number, and the player has to guess it.
The original rules are from [a Japanese TV show](https://www.fujitv.co.jp/b_hp/numer0n/) ([Wikipedia (Japanese)](https://ja.wikipedia.org/wiki/Numer0n)).

## Game Rules

1. The game generates a random number. You can freely decide the number of digits.
2. The player guesses the number and receives feedback on `hit` and `blow`:
    - `hit`: The count of digits that are in the correct place.
    - `blow`: The count of digits that are present in the correct answer but in the wrong place.
3. The player continues guessing until they find the correct number.

## Getting Started

These instructions will help you set up and run the project on your local machine for development and testing.

### Prerequisites

- Rust (Edition 2021)

### Installation

Clone the repository:

```sh
git clone https://github.com/reyalka/numeron.git
cd numeron
```

Build the project:

```sh
cargo build
```

## Usage

Run the game:

```sh
cargo run
```

The game will generate a random number. Your task is to guess this number. After each guess, the game will provide a hint.

## Thanks

- [Rust](https://www.rust-lang.org/) - The programming language used
- [clap](https://crates.io/crates/clap) - Command-line argument parser for Rust
- [colored](https://crates.io/crates/colored) - For colorizing terminal output
- [inquire](https://crates.io/crates/inquire) - Rust library for building interactive prompts
- [rand](https://crates.io/crates/rand) - Rust library for random number generation

