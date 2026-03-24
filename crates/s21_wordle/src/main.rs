// Wordle game — a CLI implementation showcasing iterators.
//
// Run: cargo run -p s21_wordle

use s21_wordle::{evaluate_guess, is_valid_guess, pick_word};
use std::io::{self, Write};

/// Maximum number of guesses allowed.
const MAX_GUESSES: usize = 6;

fn main() {
    // Pick a word based on a simple seed (process ID for variety)
    let seed = std::process::id() as usize;
    let answer = pick_word(seed);

    println!("=== WORDLE ===");
    println!("Guess the 5-letter word. You have {MAX_GUESSES} attempts.");
    println!();

    let mut guesses: Vec<String> = Vec::with_capacity(MAX_GUESSES);

    for attempt in 1..=MAX_GUESSES {
        // Read guess from stdin
        let guess = loop {
            print!("Guess {attempt}/{MAX_GUESSES}: ");
            io::stdout().flush().expect("failed to flush stdout");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let trimmed = input.trim().to_lowercase();

            if is_valid_guess(&trimmed) {
                break trimmed;
            }
            println!("Please enter exactly 5 lowercase letters.");
        };

        let result = evaluate_guess(answer, &guess);
        println!("         {result}");

        guesses.push(guess.clone());

        if result.is_winner() {
            println!();
            println!("You won in {attempt} guess(es)!");
            print_summary(&guesses, answer);
            return;
        }
    }

    println!();
    println!("Out of guesses! The word was: {answer}");
    print_summary(&guesses, answer);
}

/// Print a summary of all guesses using iterators.
fn print_summary(guesses: &[String], answer: &str) {
    println!();
    println!("--- Summary ---");
    guesses
        .iter()
        .enumerate()
        .map(|(i, g)| (i + 1, evaluate_guess(answer, g)))
        .for_each(|(num, result)| {
            println!("  {num}. {result}");
        });
}
