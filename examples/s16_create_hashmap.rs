// Demonstrates creating a HashMap.
//
// `HashMap<K, V>` stores key-value pairs with O(1) average lookup.
// Like Python's `dict`, but keys must implement `Eq` and `Hash`.
//
// Not in the prelude — must `use std::collections::HashMap`.
//
// Run: cargo run --example s16_create_hashmap

use std::collections::HashMap;

fn main() {
    // --- 1. HashMap::new() ---
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Alice".to_string(), 95);
    scores.insert("Bob".to_string(), 87);
    println!("scores: {scores:?}");

    // --- 2. Type inference ---
    let mut map = HashMap::new(); // types inferred from insert
    map.insert("key", 42);
    println!("map: {map:?}");

    // --- 3. From arrays of tuples ---
    let colors = HashMap::from([
        ("red", "#FF0000"),
        ("green", "#00FF00"),
        ("blue", "#0000FF"),
    ]);
    println!("colors: {colors:?}");

    // --- 4. From iterator of tuples ---
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    let people: HashMap<_, _> = names.into_iter().zip(ages).collect();
    println!("people: {people:?}");

    // --- 5. with_capacity ---
    let _map: HashMap<String, i32> = HashMap::with_capacity(100);

    // --- 6. Key types ---
    // Any type implementing Eq + Hash can be a key.
    // Common keys: String, &str, integers, tuples of hashable types.
    let mut coord_map: HashMap<(i32, i32), &str> = HashMap::new();
    coord_map.insert((0, 0), "origin");
    coord_map.insert((1, 0), "east");
    println!("coord_map: {coord_map:?}");

    // --- 7. len and is_empty ---
    println!("len: {}", scores.len());
    println!("is_empty: {}", scores.is_empty());
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_insert_get() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        assert_eq!(m.get("a"), Some(&1));
    }

    #[test]
    fn test_from_array() {
        let m = HashMap::from([("x", 1), ("y", 2)]);
        assert_eq!(m.len(), 2);
    }

    #[test]
    fn test_collect() {
        let keys = vec!["a", "b"];
        let vals = vec![1, 2];
        let m: HashMap<_, _> = keys.into_iter().zip(vals).collect();
        assert_eq!(m.get("b"), Some(&2));
    }
}
