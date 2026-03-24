// Explains why an iterator variable can be immutable.
//
// `for item in &v` — the binding `item` is immutable, but iteration
// still works because `next()` is called on the iterator (which is
// mutable internally via the for loop's desugaring).
//
// Run: cargo run --example s21_why_immutable

fn main() {
    println!("=== Why Iterator Can Be Immutable ===");

    // The for loop manages the mutable iterator internally:
    let v = vec![1, 2, 3];
    for item in &v {
        // `item` is &i32 (immutable reference)
        println!("item: {item}");
        // item = &99; // ERROR: can't reassign
    }

    // Explicit desugaring shows the mutable iterator:
    let mut iter = v.iter(); // `iter` is mutable
    while let Some(item) = iter.next() {
        // `item` is still &i32 (immutable)
        println!("item: {item}");
    }

    // Key insight:
    // - The ITERATOR needs to be mutable (to track position)
    // - The ITEMS don't need to be mutable (to read values)
    // - `for` hides the mutable iterator variable

    // With iter_mut, items ARE mutable:
    let mut v = vec![1, 2, 3];
    for item in &mut v {
        // `item` is &mut i32
        *item += 10;
    }
    println!("mutated: {v:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_immutable_iteration() {
        let v = vec![1, 2, 3];
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 6);
        assert_eq!(v, vec![1, 2, 3]); // unchanged
    }
}
