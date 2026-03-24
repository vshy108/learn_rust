// Demonstrates the entry method on HashMap.
//
// `entry()` provides an in-place API for inserting or modifying values.
// It returns an `Entry` enum:
// - `Occupied(entry)` — key exists
// - `Vacant(entry)` — key doesn't exist
//
// Methods:
// - `or_insert(default)` — insert default if vacant
// - `or_insert_with(|| closure)` — lazy insert if vacant
// - `or_default()` — insert Default::default() if vacant
// - `and_modify(|v| ...)` — modify if occupied
// - `key()` — get the key
//
// Run: cargo run --example s16_entry

use std::collections::HashMap;

fn main() {
    // --- 1. or_insert: insert only if missing ---
    let mut map = HashMap::new();
    map.entry("a").or_insert(1);
    map.entry("a").or_insert(999); // doesn't overwrite
    println!("or_insert: {:?}", map.get("a")); // Some(1)

    // --- 2. or_insert returns a mutable reference ---
    let val = map.entry("b").or_insert(0);
    *val += 10;
    println!("b after mutation: {:?}", map.get("b")); // Some(10)

    // --- 3. Classic: word counter ---
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("word counts: {counts:?}");

    // --- 4. or_insert_with: lazy computation ---
    let mut cache: HashMap<&str, String> = HashMap::new();
    let val = cache.entry("key").or_insert_with(|| {
        println!("  computing...");
        "expensive_result".to_string()
    });
    println!("cached: {val}");

    // --- 5. or_default: uses Default trait ---
    let mut map: HashMap<&str, Vec<i32>> = HashMap::new();
    map.entry("nums").or_default().push(1);
    map.entry("nums").or_default().push(2);
    println!("or_default vec: {:?}", map.get("nums"));

    // --- 6. and_modify + or_insert ---
    let mut map = HashMap::new();
    map.entry("counter")
        .and_modify(|v| *v += 1) // if exists: increment
        .or_insert(1); // if missing: start at 1
    map.entry("counter").and_modify(|v| *v += 1).or_insert(1);
    println!("counter: {:?}", map.get("counter")); // Some(2)

    // --- 7. Grouping with entry ---
    let data = vec![
        ("fruit", "apple"),
        ("veg", "carrot"),
        ("fruit", "banana"),
        ("veg", "pea"),
    ];
    let mut groups: HashMap<&str, Vec<&str>> = HashMap::new();
    for (category, item) in data {
        groups.entry(category).or_default().push(item);
    }
    println!("groups: {groups:?}");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_or_insert() {
        let mut m = HashMap::new();
        m.entry("a").or_insert(1);
        m.entry("a").or_insert(2);
        assert_eq!(m["a"], 1);
    }

    #[test]
    fn test_word_count() {
        let mut counts = HashMap::new();
        for w in "a b a".split_whitespace() {
            *counts.entry(w).or_insert(0) += 1;
        }
        assert_eq!(counts["a"], 2);
        assert_eq!(counts["b"], 1);
    }

    #[test]
    fn test_and_modify_or_insert() {
        let mut m = HashMap::new();
        m.entry("x").and_modify(|v| *v += 1).or_insert(10);
        assert_eq!(m["x"], 10); // first time: or_insert
        m.entry("x").and_modify(|v| *v += 1).or_insert(10);
        assert_eq!(m["x"], 11); // second time: and_modify
    }
}
