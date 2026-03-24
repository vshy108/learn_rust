// Demonstrates manual iteration using the Iterator trait's next() method.
//
// An iterator is any type implementing Iterator with a `next()` method
// that returns `Option<Item>` — `Some(value)` or `None` when done.
//
// Run: cargo run --example s21_manual_iteration

// Clippy suggests for-loops and arrays instead of vec! — but this example
// intentionally demonstrates manual .next() calls on iterators.

fn main() {
    println!("=== Manual Iteration ===");

    // --- 1. Manual next() calls ---
    let v = vec![10, 20, 30];
    let mut iter = v.iter(); // creates an iterator

    // Call next() manually:
    println!("{:?}", iter.next()); // Some(&10)
    println!("{:?}", iter.next()); // Some(&20)
    println!("{:?}", iter.next()); // Some(&30)
    println!("{:?}", iter.next()); // None — exhausted

    // --- 2. Manual while let loop ---
    let v = vec!["a", "b", "c"];
    let mut iter = v.iter();
    while let Some(item) = iter.next() {
        print!("{item} ");
    }
    println!();

    // --- 3. Range is also an iterator ---
    let mut range = 1..=5;
    while let Some(n) = range.next() {
        print!("{n} ");
    }
    println!();

    // --- 4. Consuming the iterator with a loop ---
    let text = "hello";
    let mut chars = text.chars();
    loop {
        match chars.next() {
            Some(c) => print!("{c} "),
            None => break,
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_manual_next() {
        let v = vec![1, 2, 3];
        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}
