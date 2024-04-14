# Numeron

Numeron is a command-line game implemented in Rust. The game generates a random number and the player has to guess it.
Original rules is from [a Japanese TV show](https://www.fujitv.co.jp/b_hp/numer0n/) ([Wikipedia(Japanese)](https://ja.wikipedia.org/wiki/Numer0n))

## Game Rules

1. The game generates a random number. You can decide the number of digits freely.
2. The player guesses the number. You can get information about `hit` and `blow`.
    - `hit` means the count of numbers that are in the same place as in the correct answer.
    - `blow` means the count of numbers that are in a different place from the correct answer, but the number is included in the correct answer.
3. The player continues to guess until they guess the correct number.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust (edition 2021)

### Installing

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
The game will generate a random number. Your task is to guess this number. After each guess, the game will give you a hint.
## Thanks

- [Rust](https://www.rust-lang.org/) - The programming language used
- [clap](https://crates.io/crates/clap) - Command Line Argument Parser for Rust
- [colored](https://crates.io/crates/colored) - For colorizing terminal output
- [inquire](https://crates.io/crates/inquire) - Rust library for building interactive prompts
- [rand](https://crates.io/crates/rand) - Rust library for random number generation
