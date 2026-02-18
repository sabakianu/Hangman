# Hangman Game

A simple Hangman game implementation written in Rust.

## About

This is a command-line Hangman game where you try to guess a word by suggesting letters. You have a limited number of attempts to guess the word before you lose.

## Features

- Interactive command-line gameplay
- Random word selection
- Attempts counter
- Letter guessing with input validation
- Win/lose conditions

## Requirements

- Rust 1.56 or later (edition 2021)
- Cargo

## Installation

1. Clone the repository:
```bash
git clone https://github.com/sabakianu/Hangman.git
cd Hangman
```

2. Build the project:
```bash
cargo build --release
```

## How to Play

Run the game:
```bash
cargo run
```

Or run the compiled binary directly:
```bash
./target/release/hangman
```

### Game Rules

1. The game will select a random word
2. You need to guess the word letter by letter
3. You have a limited number of incorrect attempts
4. Each correct letter guess reveals that letter in the word
5. If you guess all letters before running out of attempts, you win!
6. If you run out of attempts before guessing the word, you lose

## Dependencies

- `rand = "0.8"` - Used for random word selection

## Project Structure

```
Hangman/
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file
├── README.md           # This file
├── .gitignore          # Git ignore rules
└── src/
    └── main.rs         # Main game implementation
```
