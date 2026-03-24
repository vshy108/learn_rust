// Demonstrates immutable and mutable reference parameters.
//
// Functions can borrow values instead of taking ownership:
// - `&T`     — immutable (shared) reference: read-only access.
// - `&mut T` — mutable (exclusive) reference: read and write access.
//
// Choosing between them:
// - Use `&T` when the function only needs to READ the data.
// - Use `&mut T` when the function needs to MODIFY the data.
// - Take ownership (`T`) only when the function needs to store or consume it.
//
// Run: cargo run --example s07_immut_mut_ref_params

/// Summarizes a list of numbers (read-only access).
///
/// # Arguments
///
/// * `nums` - Shared reference to a vector of f64
///
/// # Returns
///
/// A formatted summary string
fn summarize(nums: &Vec<f64>) -> String {
    if nums.is_empty() {
        return "empty list".to_string();
    }
    let sum: f64 = nums.iter().sum();
    let avg = sum / nums.len() as f64;
    format!("count={}, sum={sum:.2}, avg={avg:.2}", nums.len())
}

/// Normalizes a vector so all values sum to 1.0 (requires write access).
///
/// # Arguments
///
/// * `nums` - Exclusive mutable reference to a vector of f64
fn normalize(nums: &mut Vec<f64>) {
    let sum: f64 = nums.iter().sum();
    if sum == 0.0 {
        return;
    }
    for val in nums.iter_mut() {
        *val /= sum;
    }
}

/// Removes all negative values from a vector.
///
/// # Arguments
///
/// * `nums` - Exclusive mutable reference to a vector of f64
fn remove_negatives(nums: &mut Vec<f64>) {
    nums.retain(|&x| x >= 0.0);
}

fn main() {
    // --- 1. Immutable reference: read-only ---
    let data = vec![10.0, 20.0, 30.0, 40.0];
    let summary = summarize(&data);
    println!("Summary: {summary}");
    println!("data still usable: {data:?}"); // not moved

    // --- 2. Mutable reference: modify in place ---
    let mut values = vec![2.0, 3.0, 5.0];
    println!("Before normalize: {values:?}");
    normalize(&mut values);
    println!("After normalize:  {values:?}");

    // --- 3. Mutable reference: structural modification ---
    let mut mixed = vec![1.0, -2.0, 3.0, -4.0, 5.0];
    println!("Before remove_negatives: {mixed:?}");
    remove_negatives(&mut mixed);
    println!("After remove_negatives:  {mixed:?}");

    // --- 4. You can re-borrow after each call ---
    // The mutable borrow ends when normalize() returns,
    // so we can borrow immutably right after.
    let mut nums = vec![1.0, 2.0, 3.0];
    normalize(&mut nums); // mutable borrow (temporary)
    let s = summarize(&nums); // immutable borrow (OK — mut borrow is done)
    println!("Normalized summary: {s}");

    // --- 5. Pattern: chain immutable reads ---
    let a = vec![1.0, 2.0];
    let b = vec![3.0, 4.0];
    // Multiple immutable borrows can coexist:
    println!("a: {}, b: {}", summarize(&a), summarize(&b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarize() {
        let v = vec![10.0, 20.0, 30.0];
        let s = summarize(&v);
        assert!(s.contains("count=3"));
        assert!(s.contains("sum=60.00"));
        assert!(s.contains("avg=20.00"));
    }

    #[test]
    fn test_summarize_empty() {
        let v: Vec<f64> = vec![];
        assert_eq!(summarize(&v), "empty list");
    }

    #[test]
    fn test_normalize() {
        let mut v = vec![2.0, 3.0, 5.0];
        normalize(&mut v);
        let sum: f64 = v.iter().sum();
        assert!((sum - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_normalize_zeros() {
        let mut v = vec![0.0, 0.0];
        normalize(&mut v); // should not panic
        assert_eq!(v, vec![0.0, 0.0]);
    }

    #[test]
    fn test_remove_negatives() {
        let mut v = vec![1.0, -2.0, 3.0, -4.0];
        remove_negatives(&mut v);
        assert_eq!(v, vec![1.0, 3.0]);
    }
}
