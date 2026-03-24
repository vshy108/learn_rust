// Demonstrates take, rev, skip, and step_by methods.
//
// Run: cargo run --example s21_take_rev_skip_step_by

fn main() {
    let nums: Vec<i32> = (1..=10).collect();

    // --- 1. take(n) — first n elements ---
    let first_three: Vec<&i32> = nums.iter().take(3).collect();
    println!("take(3): {first_three:?}");

    // --- 2. skip(n) — skip first n elements ---
    let after_three: Vec<&i32> = nums.iter().skip(3).collect();
    println!("skip(3): {after_three:?}");

    // --- 3. rev — reverse ---
    let reversed: Vec<&i32> = nums.iter().rev().collect();
    println!("rev: {reversed:?}");

    // --- 4. step_by(n) — every nth element ---
    let every_other: Vec<&i32> = nums.iter().step_by(2).collect();
    println!("step_by(2): {every_other:?}");

    let every_third: Vec<&i32> = nums.iter().step_by(3).collect();
    println!("step_by(3): {every_third:?}");

    // --- 5. Combinations ---
    // Last 3 elements:
    let last_three: Vec<&i32> = nums.iter().rev().take(3).collect();
    println!("last 3 (reversed): {last_three:?}");

    // Last 3 in order:
    let last_three: Vec<&i32> = nums
        .iter()
        .rev()
        .take(3)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    println!("last 3 (in order): {last_three:?}");

    // Skip first 2, take next 3:
    let middle: Vec<&i32> = nums.iter().skip(2).take(3).collect();
    println!("skip(2).take(3): {middle:?}");

    // --- 6. take_while and skip_while ---
    #[allow(clippy::useless_vec)] // intentional: demonstrating iterator adaptors on Vec
    let data = vec![1, 2, 3, 5, 4, 3];
    let ascending: Vec<&i32> = data.iter().take_while(|&&n| n < 5).collect();
    println!("take_while < 5: {ascending:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_take() {
        let v: Vec<i32> = (1..=5).take(3).collect();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_skip() {
        let v: Vec<i32> = (1..=5).skip(2).collect();
        assert_eq!(v, vec![3, 4, 5]);
    }

    #[test]
    fn test_rev() {
        let v: Vec<i32> = (1..=3).rev().collect();
        assert_eq!(v, vec![3, 2, 1]);
    }

    #[test]
    fn test_step_by() {
        let v: Vec<i32> = (0..10).step_by(3).collect();
        assert_eq!(v, vec![0, 3, 6, 9]);
    }
}
