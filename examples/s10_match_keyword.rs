// Demonstrates the `match` keyword with enums.
//
// `match` is Rust's pattern matching construct,
// enforced to be **exhaustive**: you must handle ALL variants.
//
// If you miss a variant, the compiler gives an error.
//
// Run: cargo run --example s10_match_keyword

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    /// Returns the value in cents.
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

/// Returns the opposite direction.
fn opposite(dir: &Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn main() {
    // --- 1. Basic match ---
    let dir = Direction::North;
    match &dir {
        Direction::North => println!("Going north!"),
        Direction::South => println!("Going south!"),
        Direction::East => println!("Going east!"),
        Direction::West => println!("Going west!"),
    }

    // --- 2. match returns a value ---
    let opposite_dir = opposite(&dir);
    println!("Opposite of {dir:?} is {opposite_dir:?}");

    // --- 3. match with method ---
    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    let total: u32 = coins.iter().map(|c| c.value_in_cents()).sum();
    println!("Total: {total} cents");

    // --- 4. match is an expression ---
    let x: i32 = 3;
    let description = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other", // `_` is the catch-all pattern
    };
    println!("{x} is {description}");

    // --- 5. match with code blocks ---
    let coin = Coin::Penny;
    let value = match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        other => other.value_in_cents(),
    };
    println!("Value: {value}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_values() {
        assert_eq!(Coin::Penny.value_in_cents(), 1);
        assert_eq!(Coin::Nickel.value_in_cents(), 5);
        assert_eq!(Coin::Dime.value_in_cents(), 10);
        assert_eq!(Coin::Quarter.value_in_cents(), 25);
    }

    #[test]
    fn test_opposite() {
        assert!(matches!(opposite(&Direction::North), Direction::South));
        assert!(matches!(opposite(&Direction::South), Direction::North));
        assert!(matches!(opposite(&Direction::East), Direction::West));
        assert!(matches!(opposite(&Direction::West), Direction::East));
    }
}
