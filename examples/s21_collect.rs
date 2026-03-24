// Demonstrates the collect method.
//
// `.collect()` consumes an iterator and builds a collection.
// The target type is inferred from context or specified with turbofish `::<>`.
//
// Run: cargo run --example s21_collect

use std::collections::{HashMap, HashSet};

fn main() {
    println!("=== The collect Method ===");

    // --- 1. Collect into Vec ---
    let v: Vec<i32> = (1..=5).collect();
    println!("Vec: {v:?}");

    // --- 2. Turbofish syntax ---
    let v = (1..=5).collect::<Vec<i32>>();
    println!("turbofish: {v:?}");

    // --- 3. Collect into String ---
    let s: String = "hello"
        .chars()
        .map(|c| c.to_uppercase().next().unwrap())
        .collect();
    println!("String: {s}");

    // --- 4. Collect into HashSet ---
    let set: HashSet<i32> = vec![1, 2, 2, 3, 3, 3].into_iter().collect();
    println!("HashSet: {set:?}");

    // --- 5. Collect into HashMap ---
    let map: HashMap<&str, i32> = vec![("a", 1), ("b", 2), ("c", 3)].into_iter().collect();
    println!("HashMap: {map:?}");

    // --- 6. Collect Result<Vec> from Vec<Result> ---
    let results: Vec<Result<i32, _>> = vec!["1", "2", "3"]
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("results: {results:?}");

    // Collect Result<Vec, E>: stops at first error
    let ok: Result<Vec<i32>, _> = vec!["1", "2", "3"]
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("all ok: {ok:?}");

    let err: Result<Vec<i32>, _> = vec!["1", "abc", "3"]
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("has err: {err:?}");

    // --- 7. Unzip ---
    let (names, ages): (Vec<&str>, Vec<i32>) = vec![("Alice", 30), ("Bob", 25)].into_iter().unzip();
    println!("names: {names:?}, ages: {ages:?}");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn test_collect_vec() {
        let v: Vec<i32> = (1..4).collect();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_collect_set() {
        let s: HashSet<i32> = vec![1, 1, 2].into_iter().collect();
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_collect_result() {
        let ok: Result<Vec<i32>, _> = vec!["1", "2"].iter().map(|s| s.parse::<i32>()).collect();
        assert_eq!(ok, Ok(vec![1, 2]));

        let err: Result<Vec<i32>, _> = vec!["1", "x"].iter().map(|s| s.parse::<i32>()).collect();
        assert!(err.is_err());
    }
}
