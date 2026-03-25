// Demonstrates HashSet — a collection of unique values.
//
// `HashSet<T>` is like a `HashMap<T, ()>` — keys only, no values.
//
// Run: cargo run --example s16_hashset

use std::collections::HashSet;

fn main() {
    // --- 1. Creating a HashSet ---
    let mut fruits = HashSet::new();
    fruits.insert("apple");
    fruits.insert("banana");
    fruits.insert("cherry");
    println!("fruits: {fruits:?}");

    // --- 2. Duplicates are ignored ---
    let added = fruits.insert("apple"); // already exists
    println!("added duplicate: {added}"); // false
    println!("still 3: {}", fruits.len());

    // --- 3. From array ---
    let nums = HashSet::from([1, 2, 3, 4, 5]);
    println!("nums: {nums:?}");

    // --- 4. From iterator ---
    let words = vec!["hello", "world", "hello"];
    let unique: HashSet<_> = words.into_iter().collect();
    println!("unique words: {unique:?}"); // 2 elements

    // --- 5. contains ---
    println!("has banana: {}", fruits.contains("banana"));
    println!("has grape: {}", fruits.contains("grape"));

    // --- 6. remove ---
    fruits.remove("banana");
    println!("after remove: {fruits:?}");

    // --- 7. Iteration ---
    for fruit in &fruits {
        println!("  {fruit}");
    }

    // --- 8. Practical: deduplication ---
    let data = vec![5, 3, 1, 3, 5, 7, 1, 9];
    let unique: HashSet<_> = data.iter().collect();
    println!("unique numbers: {unique:?}");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn test_no_duplicates() {
        let mut s = HashSet::new();
        assert!(s.insert(1));
        assert!(!s.insert(1)); // duplicate returns false
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn test_contains() {
        let s = HashSet::from([1, 2, 3]);
        assert!(s.contains(&2));
        assert!(!s.contains(&4));
    }

    #[test]
    fn test_remove() {
        let mut s = HashSet::from([1, 2, 3]);
        assert!(s.remove(&2));
        assert!(!s.contains(&2));
    }
}
