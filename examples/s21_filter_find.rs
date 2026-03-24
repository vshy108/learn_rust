#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates filter and find methods.
//
// - `filter(pred)` keeps only elements where pred returns true (lazy)
// - `find(pred)` returns the first matching element as Option (eager)
//
// Run: cargo run --example s21_filter_find

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // --- 1. filter ---
    let evens: Vec<&i32> = nums.iter().filter(|&&n| n % 2 == 0).collect();
    println!("evens: {evens:?}");

    // --- 2. filter with strings ---
    let words = vec!["rust", "go", "python", "c", "javascript"];
    let long: Vec<&&str> = words.iter().filter(|w| w.len() > 3).collect();
    println!("long words: {long:?}");

    // --- 3. find (returns first match) ---
    let first_even = nums.iter().find(|&&n| n % 2 == 0);
    println!("first even: {first_even:?}");

    let first_big = nums.iter().find(|&&n| n > 100);
    println!("first > 100: {first_big:?}"); // None

    // --- 4. Chaining filter + map ---
    let even_squares: Vec<i32> = nums
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .collect();
    println!("even squares: {even_squares:?}");

    // --- 5. find on structs ---
    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
    }

    let users = vec![
        User {
            name: "Alice".into(),
            age: 30,
        },
        User {
            name: "Bob".into(),
            age: 17,
        },
        User {
            name: "Charlie".into(),
            age: 25,
        },
    ];
    let minor = users.iter().find(|u| u.age < 18);
    println!("minor: {minor:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_filter() {
        let v: Vec<i32> = vec![1, 2, 3, 4]
            .into_iter()
            .filter(|n| n % 2 == 0)
            .collect();
        assert_eq!(v, vec![2, 4]);
    }

    #[test]
    fn test_find() {
        let v = vec![1, 2, 3];
        assert_eq!(v.iter().find(|&&n| n > 1), Some(&2));
        assert_eq!(v.iter().find(|&&n| n > 10), None);
    }
}
