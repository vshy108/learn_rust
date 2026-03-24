// Demonstrates the partition method.
//
// `.partition(pred)` splits an iterator into two collections:
// items where pred is true, and items where pred is false.
//
// Run: cargo run --example s21_partition

fn main() {
    println!("=== The partition Method ===");

    // --- 1. Split even/odd ---
    let nums: Vec<i32> = (1..=10).collect();
    let (evens, odds): (Vec<i32>, Vec<i32>) = nums.iter().partition(|&&n| n % 2 == 0);
    println!("evens: {evens:?}");
    println!("odds:  {odds:?}");

    // --- 2. Split pass/fail ---
    let scores = vec![85, 92, 45, 78, 55, 96, 30];
    let (pass, fail): (Vec<&i32>, Vec<&i32>) = scores.iter().partition(|&&s| s >= 60);
    println!("pass: {pass:?}");
    println!("fail: {fail:?}");

    // --- 3. Split strings by length ---
    let words = vec!["hi", "hello", "hey", "greetings", "yo"];
    let (short, long): (Vec<&&str>, Vec<&&str>) = words.iter().partition(|w| w.len() <= 3);
    println!("short: {short:?}");
    println!("long:  {long:?}");

    // --- 4. Partition with owned values ---
    let items = vec![1, -2, 3, -4, 5];
    let (positive, negative): (Vec<i32>, Vec<i32>) = items.into_iter().partition(|&n| n > 0);
    println!("positive: {positive:?}");
    println!("negative: {negative:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_partition() {
        let (evens, odds): (Vec<i32>, Vec<i32>) = (1..=6).partition(|n| n % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }
}
