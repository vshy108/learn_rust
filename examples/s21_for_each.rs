// Demonstrates the for_each method.
//
// `for_each` consumes the iterator, calling a closure on each element.
// Unlike `for`, it can't `break` or `continue`.
//
// Run: cargo run --example s21_for_each

fn main() {
    println!("=== The for_each Method ===");

    // --- 1. Basic for_each ---
    vec![1, 2, 3].iter().for_each(|n| {
        println!("  {n}");
    });

    // --- 2. for_each with side effects ---
    let mut sum = 0;
    (1..=10).for_each(|n| sum += n);
    println!("sum 1..10 = {sum}");

    // --- 3. Chained with other adaptors ---
    vec![1, 2, 3, 4, 5]
        .iter()
        .filter(|&&n| n % 2 == 0)
        .for_each(|n| println!("  even: {n}"));

    // --- 4. for_each on HashMap ---
    let map = std::collections::HashMap::from([("rust", 10), ("python", 20), ("go", 15)]);
    map.iter().for_each(|(k, v)| {
        println!("  {k}: {v}");
    });

    // --- 5. for_each with enumerate ---
    ["a", "b", "c"].iter().enumerate().for_each(|(i, val)| {
        println!("  [{i}] = {val}");
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_for_each_sum() {
        let mut total = 0;
        vec![1, 2, 3].iter().for_each(|&n| total += n);
        assert_eq!(total, 6);
    }
}
