// Demonstrates the flatten method.
//
// `.flatten()` flattens one level of nesting:
// - `Iterator<Item = Vec<T>>` → `Iterator<Item = T>`
// - `Iterator<Item = Option<T>>` → `Iterator<Item = T>` (skips None)
//
// Run: cargo run --example s21_flatten

fn main() {
    println!("=== The flatten Method ===");

    // --- 1. Flatten nested Vecs ---
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("flattened: {flat:?}"); // [1, 2, 3, 4, 5]

    // --- 2. Flatten Options ---
    let options = vec![Some(1), None, Some(3), None, Some(5)];
    let values: Vec<i32> = options.into_iter().flatten().collect();
    println!("from options: {values:?}"); // [1, 3, 5]

    // --- 3. Flatten strings ---
    let words = vec!["hello", "world"];
    let chars: Vec<char> = words.iter().flat_map(|w| w.chars()).collect();
    println!("chars: {chars:?}");

    // --- 4. Flatten Results (keeps Ok values) ---
    let results: Vec<Result<i32, &str>> = vec![Ok(1), Err("bad"), Ok(3)];
    let values: Vec<i32> = results.into_iter().filter_map(|r| r.ok()).collect();
    println!("ok values: {values:?}");

    // --- 5. Flatten arrays ---
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let flat: Vec<&i32> = matrix.iter().flatten().collect();
    println!("matrix flattened: {flat:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_flatten_vecs() {
        let nested = vec![vec![1, 2], vec![3]];
        let flat: Vec<i32> = nested.into_iter().flatten().collect();
        assert_eq!(flat, vec![1, 2, 3]);
    }

    #[test]
    fn test_flatten_options() {
        let opts = vec![Some(1), None, Some(3)];
        let v: Vec<i32> = opts.into_iter().flatten().collect();
        assert_eq!(v, vec![1, 3]);
    }
}
