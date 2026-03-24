#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates the filter_map method.
//
// `filter_map` combines filter + map in one step.
// The closure returns `Option<U>`:
// - `Some(value)` → keep and transform
// - `None` → skip
//
// Run: cargo run --example s21_filter_map

fn main() {
    println!("=== The filter_map Method ===");

    // --- 1. Parse strings, skip failures ---
    let strings = vec!["1", "two", "3", "four", "5"];
    let numbers: Vec<i32> = strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok()) // ok() converts Result to Option
        .collect();
    println!("parsed: {numbers:?}"); // [1, 3, 5]

    // --- 2. Equivalent without filter_map (messier) ---
    let numbers2: Vec<i32> = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect();
    println!("equivalent: {numbers2:?}");

    // --- 3. Extract data from Option fields ---
    #[derive(Debug)]
    struct User {
        name: String,
        email: Option<String>,
    }
    let users = vec![
        User {
            name: "Alice".into(),
            email: Some("alice@example.com".into()),
        },
        User {
            name: "Bob".into(),
            email: None,
        },
        User {
            name: "Charlie".into(),
            email: Some("charlie@example.com".into()),
        },
    ];
    let emails: Vec<&str> = users.iter().filter_map(|u| u.email.as_deref()).collect();
    println!("emails: {emails:?}");

    // --- 4. Transform and filter in one step ---
    let values: Vec<i32> = (0..10)
        .filter_map(|n| if n % 3 == 0 { Some(n * n) } else { None })
        .collect();
    println!("squares of multiples of 3: {values:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_filter_map_parse() {
        let v: Vec<i32> = vec!["1", "x", "3"]
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();
        assert_eq!(v, vec![1, 3]);
    }

    #[test]
    fn test_filter_map_option() {
        let v: Vec<i32> = vec![Some(1), None, Some(3)].into_iter().flatten().collect();
        assert_eq!(v, vec![1, 3]);
    }
}
