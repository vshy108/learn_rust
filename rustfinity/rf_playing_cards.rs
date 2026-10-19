//! # Rustfinity: Playing Cards
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/playing-cards
//!
//! Enums in Rust can combine unit, tuple and named field variants to represent a wide range of concepts. This challenge uses enums to represent playing cards in a card game.
//!
//! You will implement a `Card` enum that can represent either:
//!
//! - A face card (e.g., `King`, `Queen`, `Jack`), which is a unit variant.
//! - A numbered card with a value (e.g., `7 of Hearts`), which is a tuple variant.
//!
//! ## Your Task
//!
//! Create an enum `Card` with the following variants:
//!
//! - `King`, `Queen`, `Jack` (unit variants for face cards).
//! - `Numbered(u8, String)` representing a numbered card with its value and suit.
//!
//! Write a function `card_description` that takes a `Card` and returns a description of the card:
//!
//! - For `King`, return `"King"`.
//! - For `Queen`, return `"Queen"`.
//! - For `Jack`, return `"Jack"`.
//! - For `Numbered(value, suit)`, return `"{value} of {suit}"`, e.g., `"7 of Hearts"`.
//! - Ignore error handling for this challenge.
//!
//! ## Hints
//!
//! <details>
//! <summary>Click here to reveal hints</summary>
//!
//! - Use the `match` statement to handle each enum variant.
//! - String interpolation can be done with the `format!` macro for the `Numbered` variant.
//!
//! </details>
//!
//! Run: `cargo run --example rf_playing_cards`
//! Test: `cargo test --example rf_playing_cards`
//! Filter: `cargo test --example rf_playing_cards <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub enum Card {
    King,
    Queen,
    Jack,
    Numbered(u8, String),
}

pub fn card_description(card: &Card) -> String {
    // Your code here...
    unimplemented!()
}

fn main() {
    println!("Implement the Playing Cards challenge.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_card_description_face_cards() {
        assert_eq!(card_description(&Card::King), "King");
        assert_eq!(card_description(&Card::Queen), "Queen");
        assert_eq!(card_description(&Card::Jack), "Jack");
    }

    #[test]
    fn test_card_description_numbered_cards() {
        let card = Card::Numbered(7, "Hearts".to_string());
        assert_eq!(card_description(&card), "7 of Hearts");

        let card = Card::Numbered(10, "Diamonds".to_string());
        assert_eq!(card_description(&card), "10 of Diamonds");

        let card = Card::Numbered(1, "Clubs".to_string());
        assert_eq!(card_description(&card), "1 of Clubs");

        let card = Card::Numbered(13, "Spades".to_string());
        assert_eq!(card_description(&card), "13 of Spades");
    }

    #[test]
    fn test_card_description_edge_cases() {
        let card = Card::Numbered(0, "Hearts".to_string());
        assert_eq!(card_description(&card), "0 of Hearts"); // Valid but unusual case

        let card = Card::Numbered(100, "Jokers".to_string());
        assert_eq!(card_description(&card), "100 of Jokers"); // Out of typical card range, but still valid in enum
    }

    #[test]
    fn test_card_description_mixed_suits() {
        let card = Card::Numbered(2, "Stars".to_string());
        assert_eq!(card_description(&card), "2 of Stars");

        let card = Card::Numbered(8, "Moons".to_string());
        assert_eq!(card_description(&card), "8 of Moons");
    }

    #[test]
    fn test_card_description_custom_face_cards() {
        // Ensure the face cards are not confused with Numbered cards
        assert_ne!(card_description(&Card::King), "13 of Hearts");
        assert_ne!(card_description(&Card::Queen), "12 of Spades");
        assert_ne!(card_description(&Card::Jack), "11 of Clubs");
    }
}
