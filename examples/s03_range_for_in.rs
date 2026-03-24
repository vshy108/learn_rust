// Demonstrates ranges and for-in loops in Rust.
//
// Rust has several range types in `std::ops`:
//   `a..b`    — exclusive end (like Python's `range(a, b)`)
//   `a..=b`   — inclusive end (like Python's `range(a, b+1)`)
//   `a..`     — from a to infinity
//   `..b`     — from start to b (exclusive)
//   `..=b`    — from start to b (inclusive)
//   `..`      — full range
//
// Run: cargo run --example s03_range_for_in

fn main() {
    // --- 1. Exclusive range (a..b) ---
    print!("0..5: ");
    for i in 0..5 {
        print!("{i} ");
    }
    println!(); // 0 1 2 3 4

    // --- 2. Inclusive range (a..=b) ---
    print!("0..=5: ");
    for i in 0..=5 {
        print!("{i} ");
    }
    println!(); // 0 1 2 3 4 5

    // --- 3. Reverse range with .rev() ---
    // `5..0` produces an empty range (start > end doesn't work).
    // Use `.rev()` for counting down.
    print!("(0..5).rev(): ");
    for i in (0..5).rev() {
        print!("{i} ");
    }
    println!(); // 4 3 2 1 0

    // --- 4. Step with .step_by() ---
    print!("(0..10).step_by(2): ");
    for i in (0..10).step_by(2) {
        print!("{i} ");
    }
    println!(); // 0 2 4 6 8

    // --- 5. Range as array slice index ---
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[1..4]; // elements at index 1, 2, 3
    println!("arr[1..4] = {slice:?}"); // [20, 30, 40]

    let slice2 = &arr[2..]; // from index 2 to end
    println!("arr[2..] = {slice2:?}"); // [30, 40, 50]

    let slice3 = &arr[..3]; // from start to index 3 (exclusive)
    println!("arr[..3] = {slice3:?}"); // [10, 20, 30]

    // --- 6. contains() on ranges ---
    let range = 1..10;
    println!();
    println!("(1..10).contains(&5) = {}", range.contains(&5)); // true
    println!("(1..10).contains(&10) = {}", (1..10).contains(&10)); // false (exclusive)
    println!("(1..=10).contains(&10) = {}", (1..=10).contains(&10)); // true (inclusive)

    // --- 7. Char ranges ---
    print!("'a'..='f': ");
    for ch in 'a'..='f' {
        print!("{ch} ");
    }
    println!(); // a b c d e f

    // --- 8. for-in with iterators ---
    // `for x in collection` works with anything that implements `IntoIterator`.
    let v = vec![10, 20, 30];
    print!("vec: ");
    for val in &v {
        print!("{val} ");
    }
    println!();

    // --- 9. Enumerate ---
    // Like Python's `enumerate()`, gives (index, value) pairs.
    for (i, val) in v.iter().enumerate() {
        println!("  v[{i}] = {val}");
    }

    // --- Gotcha: ranges are lazy ---
    // `0..1_000_000` doesn't allocate a million numbers.
    // It produces values one at a time (like Python's `range()`).
    // This means they're memory-efficient for large ranges.

    // --- Gotcha: empty ranges ---
    // `5..3` is empty (start > end for exclusive range).
    let count = (5..3).count();
    println!();
    println!("(5..3).count() = {count}"); // 0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_exclusive_range() {
        let v: Vec<i32> = (0..5).collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_inclusive_range() {
        let v: Vec<i32> = (0..=5).collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_rev() {
        let v: Vec<i32> = (0..3).rev().collect();
        assert_eq!(v, vec![2, 1, 0]);
    }

    #[test]
    fn test_step_by() {
        let v: Vec<i32> = (0..10).step_by(3).collect();
        assert_eq!(v, vec![0, 3, 6, 9]);
    }

    #[test]
    fn test_contains() {
        assert!((1..10).contains(&5));
        assert!(!(1..10).contains(&10));
        assert!((1..=10).contains(&10));
    }

    #[test]
    fn test_empty_range() {
        assert_eq!((5..3).count(), 0);
    }

    #[test]
    fn test_char_range() {
        let chars: Vec<char> = ('a'..='d').collect();
        assert_eq!(chars, vec!['a', 'b', 'c', 'd']);
    }
}
