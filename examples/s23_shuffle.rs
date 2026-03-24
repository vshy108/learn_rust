// Demonstrates randomizing vector elements with shuffle.
//
// `slice.shuffle(&mut rng)` randomly reorders elements in place.
//
// Run: cargo run --example s23_shuffle

use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};

fn main() {
    println!("=== Shuffle ===");

    let mut rng = rand::rng();

    // --- 1. Shuffle a vector ---
    let mut cards: Vec<u32> = (1..=10).collect();
    println!("before: {cards:?}");
    cards.shuffle(&mut rng);
    println!("after:  {cards:?}");

    // --- 2. Shuffle strings ---
    let mut names = vec!["Alice", "Bob", "Charlie", "Diana", "Eve"];
    names.shuffle(&mut rng);
    println!("shuffled names: {names:?}");

    // --- 3. Choose a random element ---
    let fruits = vec!["apple", "banana", "cherry", "date"];
    let pick = fruits.choose(&mut rng);
    println!("random fruit: {pick:?}");

    // --- 4. Choose multiple without replacement ---
    let nums: Vec<i32> = (1..=50).collect();
    let lottery: Vec<&i32> = nums.choose_multiple(&mut rng, 6).collect();
    println!("lottery numbers: {lottery:?}");

    // --- 5. Simulate a card deck shuffle ---
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = [
        "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
    ];
    let mut deck: Vec<String> = suits
        .iter()
        .flat_map(|s| ranks.iter().map(move |r| format!("{r} of {s}")))
        .collect();
    deck.shuffle(&mut rng);

    println!("top 5 cards:");
    for card in deck.iter().take(5) {
        println!("  {card}");
    }

    // --- 6. Weighted random selection ---
    let items = ["common", "uncommon", "rare"];
    let weights = [70, 25, 5];
    let mut counts = [0u32; 3];
    for _ in 0..1000 {
        let roll: u32 = rng.random_range(0..100);
        let idx = if roll < weights[0] {
            0
        } else if roll < weights[0] + weights[1] {
            1
        } else {
            2
        };
        counts[idx] += 1;
    }
    println!("Weighted selection (1000 trials):");
    for (item, count) in items.iter().zip(counts.iter()) {
        println!("  {item}: {count}");
    }
}

#[cfg(test)]
mod tests {
    use rand::seq::{IndexedRandom, SliceRandom};

    #[test]
    fn test_shuffle_preserves_elements() {
        let mut rng = rand::rng();
        let mut v: Vec<i32> = (1..=10).collect();
        v.shuffle(&mut rng);
        v.sort();
        assert_eq!(v, (1..=10).collect::<Vec<i32>>());
    }

    #[test]
    fn test_choose() {
        let mut rng = rand::rng();
        let v = vec![1, 2, 3];
        let pick = v.choose(&mut rng).unwrap();
        assert!(v.contains(pick));
    }
}
