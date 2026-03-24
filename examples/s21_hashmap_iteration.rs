// Demonstrates iterating over HashMaps.
//
// HashMaps provide three iterators:
// - `.iter()` → (&K, &V) pairs
// - `.keys()` → &K
// - `.values()` → &V
//
// Run: cargo run --example s21_hashmap_iteration

use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);

    // --- 1. Iterate over key-value pairs ---
    println!("All scores:");
    for (name, score) in &scores {
        println!("  {name}: {score}");
    }

    // --- 2. Iterate over keys only ---
    let names: Vec<&&str> = scores.keys().collect();
    println!("names: {names:?}");

    // --- 3. Iterate over values only ---
    let avg: f64 = scores.values().sum::<i32>() as f64 / scores.len() as f64;
    println!("average: {avg:.1}");

    // --- 4. iter_mut to modify values ---
    for (_name, score) in scores.iter_mut() {
        *score += 5; // bonus points
    }
    println!("after bonus: {scores:?}");

    // --- 5. Consuming iteration ---
    let owned: HashMap<String, i32> = vec![("x".to_string(), 1), ("y".to_string(), 2)]
        .into_iter()
        .collect();
    for (key, value) in owned {
        println!("  owned: {key}={value}");
    }

    // --- 6. Filter and collect into new HashMap ---
    let scores: HashMap<&str, i32> = [("Alice", 95), ("Bob", 60), ("Charlie", 85)].into();
    let passing: HashMap<&&str, &i32> = scores.iter().filter(|&(_, score)| *score >= 70).collect();
    println!("passing: {passing:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_iter() {
        let m: HashMap<&str, i32> = [("a", 1), ("b", 2)].into();
        let sum: i32 = m.values().sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn test_hashmap_filter() {
        let m: HashMap<&str, i32> = [("a", 10), ("b", 20), ("c", 5)].into();
        let big: Vec<_> = m.iter().filter(|&(_, v)| *v > 8).collect();
        assert_eq!(big.len(), 2);
    }
}
