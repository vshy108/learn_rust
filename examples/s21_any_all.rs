// Demonstrates any and all methods.
//
// - `any(pred)` → true if ANY element satisfies pred
// - `all(pred)` → true if ALL elements satisfy pred
// Both short-circuit (stop as soon as result is known).
//
// Run: cargo run --example s21_any_all

fn main() {
    let nums = vec![2, 4, 6, 8, 10];

    // --- 1. all ---
    let all_even = nums.iter().all(|n| n % 2 == 0);
    println!("all even? {all_even}"); // true

    let all_positive = nums.iter().all(|n| *n > 0);
    println!("all positive? {all_positive}"); // true

    // --- 2. any ---
    let has_ten = nums.iter().any(|n| *n == 10);
    println!("has 10? {has_ten}"); // true

    let has_odd = nums.iter().any(|n| n % 2 != 0);
    println!("has odd? {has_odd}"); // false

    // --- 3. Short-circuit behavior ---
    let big = vec![1, 2, 3, 100, 5];
    let found = big.iter().any(|n| {
        println!("  checking {n}");
        *n > 50
    });
    println!("found > 50: {found}");
    // Notice: only checks up to 100, then stops

    // --- 4. With strings ---
    let words = vec!["hello", "world", "rust"];
    let has_long = words.iter().any(|w| w.len() > 4);
    let all_short = words.iter().all(|w| w.len() < 10);
    println!("has long word (>4)? {has_long}");
    println!("all short (<10)? {all_short}");

    // --- 5. Empty iterators ---
    let empty: Vec<i32> = vec![];
    println!("empty.all(true): {}", empty.iter().all(|_| true)); // true (vacuous truth)
    println!("empty.any(true): {}", empty.iter().any(|_| true)); // false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_any() {
        assert!(vec![1, 2, 3].iter().any(|&n| n == 2));
        assert!(!vec![1, 2, 3].iter().any(|&n| n == 4));
    }

    #[test]
    fn test_all() {
        assert!(vec![2, 4, 6].iter().all(|n| n % 2 == 0));
        assert!(!vec![2, 4, 5].iter().all(|n| n % 2 == 0));
    }

    #[test]
    fn test_empty() {
        let empty: Vec<i32> = vec![];
        assert!(empty.iter().all(|_| false)); // vacuous truth
        assert!(!empty.iter().any(|_| true));
    }
}
