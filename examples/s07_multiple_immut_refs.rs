// Demonstrates multiple immutable references.
//
// Rust allows any number of immutable (&T) references to the same data
// at the same time. This is safe because none of them can modify the data.
//
// Key rules:
// - Multiple `&T` — OK (many readers, no writers).
// - One `&mut T` — OK (one writer, no readers).
// - `&T` + `&mut T` simultaneously — FORBIDDEN.
//
// Run: cargo run --example s07_multiple_immut_refs

fn main() {
    // --- 1. Multiple immutable refs to the same value ---
    let s = String::from("shared data");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("r1={r1}, r2={r2}, r3={r3}"); // all valid simultaneously

    // --- 2. Passing multiple refs to different functions ---
    let data = vec![10, 20, 30, 40, 50];
    let sum = compute_sum(&data);
    let avg = compute_avg(&data);
    let max = find_max(&data);
    println!("sum={sum}, avg={avg:.1}, max={max:?}");

    // --- 3. Multiple refs in a collection ---
    let a = String::from("alpha");
    let b = String::from("beta");
    let c = String::from("gamma");
    let refs: Vec<&String> = vec![&a, &b, &c];
    for r in &refs {
        println!("ref: {r}");
    }
    // a, b, c all still usable:
    println!("Originals: {a}, {b}, {c}");

    // --- 4. Immutable refs can be freely copied ---
    // &T implements Copy, so passing a ref doesn't move it.
    let r = &data;
    let r_copy = r; // copy of the ref
    assert_eq!(r.len(), r_copy.len());

    // --- 5. Lifetime overlap is fine for &T ---
    let x: i32 = 42;
    let ra = &x;
    let rb = &x;
    // ra and rb overlap — both live until their last use.
    assert_eq!(*ra + *rb, 84);
}

/// Computes the sum of a slice.
///
/// # Arguments
///
/// * `data` - Shared reference to a vector of i32
///
/// # Returns
///
/// The sum
fn compute_sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

/// Computes the average of a slice.
///
/// # Arguments
///
/// * `data` - Shared reference to a vector of i32
///
/// # Returns
///
/// The average as f64
fn compute_avg(data: &[i32]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    compute_sum(data) as f64 / data.len() as f64
}

/// Finds the maximum value in a slice.
///
/// # Arguments
///
/// * `data` - Shared reference to a vector of i32
///
/// # Returns
///
/// The max value, or None if empty
fn find_max(data: &[i32]) -> Option<i32> {
    data.iter().copied().max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_sum() {
        assert_eq!(compute_sum(&[1, 2, 3]), 6);
        assert_eq!(compute_sum(&[]), 0);
    }

    #[test]
    fn test_compute_avg() {
        let avg = compute_avg(&[10, 20, 30]);
        assert!((avg - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_compute_avg_empty() {
        assert!((compute_avg(&[]) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[3, 1, 4, 1, 5]), Some(5));
        assert_eq!(find_max(&[]), None);
    }

    #[test]
    fn test_multiple_refs_coexist() {
        let v = vec![1, 2, 3];
        let r1 = &v;
        let r2 = &v;
        assert_eq!(r1, r2);
    }
}
