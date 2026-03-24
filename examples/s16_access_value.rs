// Demonstrates accessing values in a HashMap.
//
// Run: cargo run --example s16_access_value

use std::collections::HashMap;

fn main() {
    let scores = HashMap::from([("Alice", 95), ("Bob", 87), ("Charlie", 92)]);

    // --- 1. get() returns Option<&V> ---
    match scores.get("Alice") {
        Some(score) => println!("Alice: {score}"),
        None => println!("Alice not found"),
    }
    println!("missing: {:?}", scores.get("Dave")); // None

    // --- 2. Direct index with [] panics if missing ---
    println!("Bob: {}", scores["Bob"]);
    // scores["Dave"]; // PANIC: key not found

    // --- 3. contains_key ---
    println!("has Alice: {}", scores.contains_key("Alice"));
    println!("has Dave: {}", scores.contains_key("Dave"));

    // --- 4. Iterating ---
    print!("all scores: ");
    for (name, score) in &scores {
        print!("{name}={score} ");
    }
    println!();

    // --- 5. keys() and values() ---
    let keys: Vec<&&str> = scores.keys().collect();
    let values: Vec<&i32> = scores.values().collect();
    println!("keys: {keys:?}");
    println!("values: {values:?}");

    // --- 6. get_key_value ---
    if let Some((k, v)) = scores.get_key_value("Charlie") {
        println!("get_key_value: {k}={v}");
    }

    // --- 7. Practical: default for missing ---
    let score = scores.get("Dave").copied().unwrap_or(0);
    println!("Dave's score (default 0): {score}");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_get_existing() {
        let m = HashMap::from([("a", 1)]);
        assert_eq!(m.get("a"), Some(&1));
    }

    #[test]
    fn test_get_missing() {
        let m = HashMap::from([("a", 1)]);
        assert_eq!(m.get("b"), None);
    }

    #[test]
    fn test_contains_key() {
        let m = HashMap::from([("x", 10)]);
        assert!(m.contains_key("x"));
        assert!(!m.contains_key("y"));
    }

    #[test]
    fn test_default_for_missing() {
        let m: HashMap<&str, i32> = HashMap::new();
        let val = m.get("missing").copied().unwrap_or(42);
        assert_eq!(val, 42);
    }
}
