//! # Rustfinity: Character Counting String
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/character-counting-string
//!
//! You are given a string `s`. Your task is to count the number of characters in the string and return the **total amount of characters** in the string with the type `u32`.
//!
//! ## Example
//!
//! ```rust
//! let s = "hello";
//! let result = character_counting_string(s);
//! assert_eq!(result, 5);
//! ```
//!
//! ## Hints
//!
//! - You can use the `chars()` method to get an iterator over the characters in the string.
//! - You can use the `count()` method to count the number of elements in the iterator.
//!
//! > The `count()` method returns a `usize` which is the number of elements in the iterator. In the challenge you are asked to return a `u32`, you can use the `as` keyword to convert the `usize` to a `u32`. For example, `let count_u32 = count as u32;`
//! >
//! > Don't use `len()` : This length is in bytes, not [`char`]s or graphemes. In other words, it might not be what a human considers the length of the string.
//!
//! Run: `cargo run --example rf_character_counting_string`
//! Test: `cargo test --example rf_character_counting_string`
//! Filter: `cargo test --example rf_character_counting_string <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub fn count_characters(s: &str) -> u32 {
    // Count the number of characters in the string
    unimplemented!()
}

fn main() {
    println!("Implement the Character Counting String challenge.");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn should_count_characters() {
        assert_eq!(count_characters("hello"), 5);
        assert_eq!(count_characters("hello world"), 11);
        assert_eq!(count_characters("hello world!"), 12);
        assert_eq!(count_characters("hello world!  "), 14);
        assert_eq!(count_characters("hello world!  123"), 17);
        assert_eq!(count_characters("hello world!  123  "), 19);
        assert_eq!(count_characters("你好 世界"), 5);
        assert_eq!(count_characters("你好 🌏"), 4);
        assert_eq!(count_characters("hello 🌏"), 7);
    }

    #[test]
    fn empty_string() {
        assert_eq!(count_characters(""), 0);
    }

    #[test]
    fn single_character() {
        assert_eq!(count_characters("a"), 1);
    }

    #[test]
    fn single_word() {
        assert_eq!(count_characters("hello"), 5);
    }
}
