// Demonstrates overwriting values in a HashMap.
//
// Inserting with an existing key replaces the old value.
//
// Run: cargo run --example s16_overwrite

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // --- 1. Insert replaces existing value ---
    scores.insert("Alice", 90);
    println!("initial: {:?}", scores.get("Alice"));
    scores.insert("Alice", 95); // overwrites!
    println!("after overwrite: {:?}", scores.get("Alice"));

    // --- 2. insert returns the old value ---
    let old = scores.insert("Alice", 100);
    println!("old value: {old:?}"); // Some(95)
    println!("new value: {:?}", scores.get("Alice")); // Some(&100)

    // --- 3. First insert returns None (no old value) ---
    let first = scores.insert("Bob", 80);
    println!("first insert: {first:?}"); // None

    // --- 4. Conditional insert: only if key doesn't exist ---
    // Use entry().or_insert():
    scores.entry("Bob").or_insert(999); // Bob exists, not overwritten
    scores.entry("Charlie").or_insert(88); // Charlie doesn't exist, inserted
    println!("Bob: {:?}", scores.get("Bob")); // 80 (unchanged)
    println!("Charlie: {:?}", scores.get("Charlie")); // 88

    // --- 5. Update based on old value ---
    let mut counts = HashMap::new();
    let words = vec!["hello", "world", "hello", "rust", "hello"];
    for word in words {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("word counts: {counts:?}");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_overwrite() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("a", 2);
        assert_eq!(m.get("a"), Some(&2));
    }

    #[test]
    fn test_insert_returns_old() {
        let mut m = HashMap::new();
        assert_eq!(m.insert("a", 1), None);
        assert_eq!(m.insert("a", 2), Some(1));
    }

    #[test]
    fn test_or_insert_no_overwrite() {
        let mut m = HashMap::from([("a", 1)]);
        m.entry("a").or_insert(99);
        assert_eq!(m["a"], 1);
    }
}
