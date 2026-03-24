// Demonstrates the .get() method on Vec in detail.
//
// `.get()` is the safe alternative to direct indexing:
// - `v[i]` panics on out-of-bounds.
// - `v.get(i)` returns `Option<&T>` — no panic.
//
// Run: cargo run --example s13_get_method

fn main() {
    let scores = vec![85, 92, 78, 95, 88];

    // --- 1. get with valid index ---
    let second = scores.get(1);
    println!("get(1): {second:?}"); // Some(&92)

    // --- 2. get with invalid index ---
    let out = scores.get(100);
    println!("get(100): {out:?}"); // None

    // --- 3. Unwrapping safely ---
    let val = scores.get(2).unwrap_or(&0);
    println!("get(2) or 0: {val}");

    let missing = scores.get(99).unwrap_or(&0);
    println!("get(99) or 0: {missing}");

    // --- 4. get with ranges ---
    let slice = scores.get(1..4);
    println!("get(1..4): {slice:?}"); // Some([92, 78, 95])

    let bad_range = scores.get(3..100);
    println!("get(3..100): {bad_range:?}"); // None

    // --- 5. Pattern matching ---
    match scores.get(0) {
        Some(&val) => println!("First score: {val}"),
        None => println!("No first score"),
    }

    // --- 6. get_mut: mutable access ---
    let mut grades = vec![70, 80, 90];
    if let Some(first) = grades.get_mut(0) {
        *first += 10; // boost the first grade
    }
    println!("after get_mut boost: {grades:?}");

    // --- 7. Practical: safe lookup function ---
    fn describe_score(scores: &[i32], index: usize) -> String {
        match scores.get(index) {
            Some(&s) if s >= 90 => format!("Score {s}: Excellent"),
            Some(&s) if s >= 80 => format!("Score {s}: Good"),
            Some(&s) => format!("Score {s}: Needs improvement"),
            None => format!("No score at index {index}"),
        }
    }

    for i in 0..7 {
        println!("  {}", describe_score(&scores, i));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_valid() {
        let v = vec![1, 2, 3];
        assert_eq!(v.get(0), Some(&1));
    }

    #[test]
    fn test_get_invalid() {
        let v = vec![1, 2, 3];
        assert_eq!(v.get(10), None);
    }

    #[test]
    fn test_get_range() {
        let v = vec![10, 20, 30, 40];
        assert_eq!(v.get(1..3), Some(&[20, 30][..]));
    }

    #[test]
    fn test_get_mut() {
        let mut v = vec![1, 2, 3];
        if let Some(val) = v.get_mut(1) {
            *val = 99;
        }
        assert_eq!(v, [1, 99, 3]);
    }
}
