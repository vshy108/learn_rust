// Demonstrates the enumerate method.
//
// `.enumerate()` wraps each item with its index: `(index, item)`.
//
// Run: cargo run --example s21_enumerate

fn main() {
    let fruits = vec!["apple", "banana", "cherry", "date"];

    // --- 1. Basic enumerate ---
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  {i}: {fruit}");
    }

    // --- 2. find index ---
    let pos = fruits.iter().enumerate().find(|&(_, f)| *f == "cherry");
    println!("cherry at: {pos:?}");

    // --- 3. Enumerate with map ---
    let numbered: Vec<String> = fruits
        .iter()
        .enumerate()
        .map(|(i, f)| format!("{}. {}", i + 1, f))
        .collect();
    println!("numbered: {numbered:?}");

    // --- 4. Enumerate starts at 0 ---
    let first_even_pos = vec![1, 3, 4, 7, 8]
        .iter()
        .enumerate()
        .find(|&(_, n)| n % 2 == 0)
        .map(|(i, _)| i);
    println!("first even position: {first_even_pos:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_enumerate() {
        let v = vec!["a", "b", "c"];
        let pairs: Vec<(usize, &&str)> = v.iter().enumerate().collect();
        assert_eq!(pairs[0], (0, &"a"));
        assert_eq!(pairs[2], (2, &"c"));
    }

    #[test]
    fn test_find_index() {
        let v = vec![10, 20, 30];
        let idx = v
            .iter()
            .enumerate()
            .find(|&(_, n)| *n == 20)
            .map(|(i, _)| i);
        assert_eq!(idx, Some(1));
    }
}
