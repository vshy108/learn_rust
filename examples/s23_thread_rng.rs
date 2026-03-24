// Demonstrates the ThreadRng struct from the rand crate.
//
// `ThreadRng` is the default, thread-local random number generator.
// Obtained via `rand::rng()`.
// It's cryptographically secure and fast.
//
// Run: cargo run --example s23_thread_rng

use rand::Rng;

fn main() {
    println!("=== The ThreadRng struct ===");

    // Get the thread-local RNG
    let mut rng = rand::rng();

    // --- 1. Generate various types ---
    let int: i32 = rng.random_range(0..100);
    let float: f64 = rng.random_range(0.0..1.0);
    let boolean: bool = rng.random();
    println!("int: {int}, float: {float:.4}, bool: {boolean}");

    // --- 2. Generate multiple values ---
    let dice_rolls: Vec<u32> = (0..10).map(|_| rng.random_range(1..=6)).collect();
    println!("dice rolls: {dice_rolls:?}");

    // --- 3. Simulate a coin flip experiment ---
    let flips = 1000;
    let heads = (0..flips).filter(|_| rng.random::<bool>()).count();
    println!("{flips} coin flips: {heads} heads, {} tails", flips - heads);

    // --- 4. Random selection from options ---
    let options = ["rock", "paper", "scissors"];
    let choice = options[rng.random_range(0..options.len())];
    println!("random choice: {choice}");
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    #[test]
    fn test_rng_range() {
        let mut rng = rand::rng();
        for _ in 0..100 {
            let n: i32 = rng.random_range(1..=6);
            assert!((1..=6).contains(&n), "Die roll out of range: {n}");
        }
    }

    #[test]
    fn test_rng_float_range() {
        let mut rng = rand::rng();
        for _ in 0..100 {
            let f: f64 = rng.random_range(0.0..1.0);
            assert!(f >= 0.0 && f < 1.0);
        }
    }
}
