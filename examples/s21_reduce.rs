// Demonstrates the reduce method.
//
// `.reduce(|acc, item| ...)` is like fold but uses the first element
// as the initial value. Returns `Option` (None if empty).
//
// Run: cargo run --example s21_reduce

fn main() {
    println!("=== The reduce Method ===");

    // --- 1. Sum with reduce ---
    let sum = (1..=5).reduce(|acc, n| acc + n);
    println!("sum: {sum:?}"); // Some(15)

    // --- 2. Max with reduce ---
    let max = vec![3, 1, 4, 1, 5, 9]
        .into_iter()
        .reduce(|a, b| if a > b { a } else { b });
    println!("max: {max:?}"); // Some(9)

    // --- 3. Concatenate strings ---
    let joined = vec!["hello", "world", "rust"].into_iter().reduce(|acc, s| {
        // Note: we need to return the same type (&str)
        // This is tricky with &str, so let's use String:
        Box::leak(format!("{acc} {s}").into_boxed_str())
    });
    println!("joined: {joined:?}");

    // Better approach with owned strings:
    let joined = vec!["hello".to_string(), "world".to_string()]
        .into_iter()
        .reduce(|acc, s| format!("{acc} {s}"));
    println!("joined: {joined:?}");

    // --- 4. Empty iterator ---
    let empty: Vec<i32> = vec![];
    let result = empty.into_iter().reduce(|a, b| a + b);
    println!("empty reduce: {result:?}"); // None

    // --- 5. reduce vs fold ---
    // fold: always has initial value, never returns None
    let fold_result = (1..=5).fold(0, |a, b| a + b);
    // reduce: uses first element, returns Option
    let reduce_result = (1..=5).reduce(|a, b| a + b);
    println!("fold: {fold_result}, reduce: {reduce_result:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reduce_sum() {
        let sum = vec![1, 2, 3].into_iter().reduce(|a, b| a + b);
        assert_eq!(sum, Some(6));
    }

    #[test]
    fn test_reduce_empty() {
        let result: Option<i32> = vec![].into_iter().reduce(|a, b| a + b);
        assert_eq!(result, None);
    }

    #[test]
    fn test_reduce_single() {
        let result = vec![42].into_iter().reduce(|a, b| a + b);
        assert_eq!(result, Some(42));
    }
}
