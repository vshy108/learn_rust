// Demonstrates IntoIterator in action with for loops.
//
// The `for` loop desugars to calling `.into_iter()` on the collection.
// Three ways to iterate over a Vec:
// - `for item in &v`     → iter() → &T
// - `for item in &mut v` → iter_mut() → &mut T
// - `for item in v`      → into_iter() → T (consumes)
//
// Run: cargo run --example s21_into_iterator_action

fn main() {
    println!("=== IntoIterator in Action ===");

    let names = vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Charlie".to_string(),
    ];

    // --- 1. Borrow: &names → iter() → &String ---
    println!("By reference:");
    for name in &names {
        println!("  {name}");
    }
    println!("names still valid: {names:?}");

    // --- 2. Mutable borrow: &mut names → iter_mut() → &mut String ---
    let mut scores = vec![80, 90, 75];
    println!("Before: {scores:?}");
    for score in &mut scores {
        *score += 5; // bonus points
    }
    println!("After:  {scores:?}");

    // --- 3. Consume: names → into_iter() → String ---
    println!("Consuming:");
    for name in names {
        println!("  owned: {name}");
    }
    // names is no longer available (moved)

    // --- 4. Other IntoIterator types ---
    // Ranges:
    for i in 0..3 {
        print!("{i} ");
    }
    println!();

    // Arrays:
    for x in [10, 20, 30] {
        print!("{x} ");
    }
    println!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_borrow_iteration() {
        let v = vec![1, 2, 3];
        let mut sum = 0;
        for &n in &v {
            sum += n;
        }
        assert_eq!(sum, 6);
        assert_eq!(v.len(), 3); // v still valid
    }

    #[test]
    fn test_mut_iteration() {
        let mut v = vec![1, 2, 3];
        for n in &mut v {
            *n *= 2;
        }
        assert_eq!(v, vec![2, 4, 6]);
    }
}
