// Demonstrates nesting enums inside other enums.
//
// Enums can contain other enums as associated data, enabling
// rich hierarchical type modeling.
//
// Run: cargo run --example s10_nesting_enums

#[derive(Debug, PartialEq)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, PartialEq)]
enum Rank {
    Ace,
    Number(u8), // 2-10
    Jack,
    Queen,
    King,
}

/// A playing card — nests Suit and Rank enums.
#[derive(Debug, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

/// A hand outcome — nests Card and uses enum variants.
#[derive(Debug)]
enum HandResult {
    Win { cards: Vec<Card>, score: u32 },
    Loss { cards: Vec<Card> },
    Draw,
}

impl Card {
    /// Returns the point value of the card (Ace=11, Face=10, Number=face value).
    fn value(&self) -> u32 {
        match &self.rank {
            Rank::Ace => 11,
            Rank::Number(n) => *n as u32,
            Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
}

fn main() {
    // --- 1. Creating nested enum values ---
    let ace_of_spades = Card {
        rank: Rank::Ace,
        suit: Suit::Spades,
    };
    let ten_of_hearts = Card {
        rank: Rank::Number(10),
        suit: Suit::Hearts,
    };
    let king_of_diamonds = Card {
        rank: Rank::King,
        suit: Suit::Diamonds,
    };

    println!("{ace_of_spades:?} → {}", ace_of_spades.value());
    println!("{ten_of_hearts:?} → {}", ten_of_hearts.value());
    println!("{king_of_diamonds:?} → {}", king_of_diamonds.value());

    // --- 2. Nesting in a higher-level enum ---
    let result = HandResult::Win {
        cards: vec![ace_of_spades, king_of_diamonds],
        score: 21,
    };
    match &result {
        HandResult::Win { cards, score } => {
            println!("Won with {score} points, {cards:?}");
        }
        HandResult::Loss { cards } => {
            println!("Lost with {cards:?}");
        }
        HandResult::Draw => println!("Draw"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ace_value() {
        let c = Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
        };
        assert_eq!(c.value(), 11);
    }

    #[test]
    fn test_number_value() {
        let c = Card {
            rank: Rank::Number(7),
            suit: Suit::Hearts,
        };
        assert_eq!(c.value(), 7);
    }

    #[test]
    fn test_face_value() {
        let j = Card {
            rank: Rank::Jack,
            suit: Suit::Clubs,
        };
        let q = Card {
            rank: Rank::Queen,
            suit: Suit::Diamonds,
        };
        let k = Card {
            rank: Rank::King,
            suit: Suit::Spades,
        };
        assert_eq!(j.value(), 10);
        assert_eq!(q.value(), 10);
        assert_eq!(k.value(), 10);
    }
}
