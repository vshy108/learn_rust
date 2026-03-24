// Demonstrates the remove method on HashMap.
//
// `remove(key)` returns `Option<V>`:
// - `Some(value)` if the key existed (removes it).
// - `None` if the key wasn't found.
//
// Run: cargo run --example s16_remove

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::from([("Alice", 95), ("Bob", 87), ("Charlie", 92)]);
    println!("before: {scores:?}");

    // --- 1. remove returns the value ---
    let removed = scores.remove("Bob");
    println!("removed Bob: {removed:?}"); // Some(87)
    println!("after: {scores:?}");

    // --- 2. remove non-existent key returns None ---
    let missing = scores.remove("Dave");
    println!("removed Dave: {missing:?}"); // None

    // --- 3. remove_entry returns (key, value) ---
    let entry = scores.remove_entry("Alice");
    println!("remove_entry Alice: {entry:?}"); // Some(("Alice", 95))

    // --- 4. retain: keep only matching entries ---
    let mut numbers = HashMap::from([("one", 1), ("two", 2), ("three", 3), ("four", 4)]);
    numbers.retain(|_, v| *v % 2 == 0); // keep even values
    println!("after retain (even): {numbers:?}");

    // --- 5. clear: remove all entries ---
    numbers.clear();
    println!("after clear: len={}", numbers.len());

    // --- 6. drain: remove all and iterate ---
    let mut map = HashMap::from([("a", 1), ("b", 2)]);
    for (k, v) in map.drain() {
        println!("  drained: {k}={v}");
    }
    println!("after drain: {map:?}"); // empty
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_remove_existing() {
        let mut m = HashMap::from([("a", 1)]);
        assert_eq!(m.remove("a"), Some(1));
        assert!(m.is_empty());
    }

    #[test]
    fn test_remove_missing() {
        let mut m: HashMap<&str, i32> = HashMap::new();
        assert_eq!(m.remove("x"), None);
    }

    #[test]
    fn test_retain() {
        let mut m = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
        m.retain(|_, v| *v > 1);
        assert_eq!(m.len(), 2);
        assert!(!m.contains_key("a"));
    }
}
