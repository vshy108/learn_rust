// Demonstrates exhausting an iterator.
//
// Once an iterator returns None, it's exhausted — subsequent
// calls to next() should also return None.
//
// Run: cargo run --example s21_exhausting_iterator

fn main() {
    println!("=== Exhausting the Iterator ===");

    // --- 1. Exhaust with next() ---
    let v = vec![1, 2];
    let mut iter = v.iter();
    println!("{:?}", iter.next()); // Some(&1)
    println!("{:?}", iter.next()); // Some(&2)
    println!("{:?}", iter.next()); // None
    println!("{:?}", iter.next()); // None (stays None)

    // --- 2. Exhaust with for loop ---
    let v = vec!["a", "b", "c"];
    for item in &v {
        print!("{item} ");
    }
    println!();
    // The for loop consumed the iterator completely

    // --- 3. Consuming adaptors exhaust the iterator ---
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter().sum(); // consumes the iterator
    println!("sum: {sum}");

    let count = v.iter().count(); // also consumes
    println!("count: {count}");

    // --- 4. Partial consumption ---
    let v = vec![10, 20, 30, 40, 50];
    let mut iter = v.iter();
    let first_two: Vec<_> = iter.by_ref().take(2).collect();
    println!("first two: {first_two:?}");
    // Iterator still has remaining items:
    let rest: Vec<_> = iter.collect();
    println!("rest: {rest:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_exhaustion() {
        let v = vec![1];
        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None); // stays None
    }

    #[test]
    fn test_partial_consumption() {
        let v = vec![1, 2, 3, 4];
        let mut iter = v.iter();
        let first: Vec<_> = iter.by_ref().take(2).collect();
        let rest: Vec<_> = iter.collect();
        assert_eq!(first, vec![&1, &2]);
        assert_eq!(rest, vec![&3, &4]);
    }
}
