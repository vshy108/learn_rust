// Intro to the rand crate for generating random values.
//
// `rand` provides random number generation.
// Key functions:
// - `rand::random::<T>()` — generate a random value of type T
// - `rand::random_range(start..end)` — random in range
//
// Run: cargo run --example s23_intro_rand

use rand::Rng;

fn main() {
    println!("=== Intro to rand Crate ===");

    // --- 1. Random number in a range ---
    let n: i32 = rand::random_range(1..=100);
    println!("random 1..=100: {n}");

    // --- 2. Random bool ---
    let coin: bool = rand::random();
    println!("coin flip: {}", if coin { "heads" } else { "tails" });

    // --- 3. Random float ---
    let f: f64 = rand::random();
    println!("random f64 [0, 1): {f:.4}");

    // --- 4. Using a thread_rng for multiple values ---
    let mut rng = rand::rng();
    for i in 1..=5 {
        let val: u32 = rng.random_range(1..=6);
        println!("  die roll {i}: {val}");
    }

    // --- 5. Random character ---
    let ch: char = rand::random();
    println!("random char: {ch:?}");

    // --- 6. Fill an array with random values ---
    let mut buf = [0u8; 8];
    rand::fill(&mut buf);
    println!("random bytes: {buf:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_random_range() {
        for _ in 0..100 {
            let n: i32 = rand::random_range(1..=10);
            assert!((1..=10).contains(&n));
        }
    }

    #[test]
    fn test_random_float() {
        for _ in 0..100 {
            let f: f64 = rand::random();
            assert!((0.0..1.0).contains(&f));
        }
    }
}
