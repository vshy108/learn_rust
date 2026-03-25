// Demonstrates the fold method.
//
// `.fold(init, |acc, item| ...)` reduces an iterator to a single value,
// with an explicit initial value.
//
// Run: cargo run --example s21_fold

fn main() {
    println!("=== The fold Method ===");

    // --- 1. Sum with fold ---
    let sum = (1..=5).fold(0, |acc, n| acc + n);
    println!("sum: {sum}"); // 15

    // --- 2. Product with fold ---
    let product = (1..=5).fold(1, |acc, n| acc * n);
    println!("product: {product}"); // 120

    // --- 3. Build a string ---
    let csv = vec!["a", "b", "c"]
        .iter()
        .fold(String::new(), |mut acc, &item| {
            if !acc.is_empty() {
                acc.push(',');
            }
            acc.push_str(item);
            acc
        });
    println!("csv: {csv}");

    // --- 4. Count occurrences ---
    let text = "hello world";
    let vowel_count = text.chars().fold(0, |count, c| {
        if "aeiou".contains(c) {
            count + 1
        } else {
            count
        }
    });
    println!("vowels: {vowel_count}");

    // --- 5. Max with fold ---
    let max = vec![3, 1, 4, 1, 5, 9]
        .iter()
        .fold(i32::MIN, |max, &n| if n > max { n } else { max });
    println!("max: {max}");

    // --- 6. Fold into a HashMap ---
    let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
    let counts = words
        .iter()
        .fold(std::collections::HashMap::new(), |mut map, &word| {
            *map.entry(word).or_insert(0) += 1;
            map
        });
    println!("counts: {counts:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fold_sum() {
        let sum = (1..=10).fold(0, |a, b| a + b);
        assert_eq!(sum, 55);
    }

    #[test]
    fn test_fold_product() {
        let p = (1..=5).fold(1, |a, b| a * b);
        assert_eq!(p, 120);
    }

    #[test]
    fn test_fold_string() {
        let s = vec!["a", "b"].iter().fold(String::new(), |mut a, &b| {
            a.push_str(b);
            a
        });
        assert_eq!(s, "ab");
    }
}
