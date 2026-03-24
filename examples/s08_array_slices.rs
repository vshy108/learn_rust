// Demonstrates array slices.
//
// An array slice `&[T]` is a reference to a contiguous sequence of elements.
// Like string slices, they're a "view" without ownership.
//
// Key differences from arrays:
// - `[T; N]` — fixed-size array, size known at compile time.
// - `&[T]`   — slice, length known at runtime (fat pointer: ptr + len).
//
// Slices are the preferred way to pass arrays/vecs to functions.
//
// Run: cargo run --example s08_array_slices

/// Computes the sum of an i32 slice.
///
/// Accepts `&[i32]` — works with arrays, vecs, and other slices.
///
/// # Arguments
///
/// * `data` - A slice of i32 values
///
/// # Returns
///
/// The sum of all elements
fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

/// Returns the index of the maximum element.
///
/// # Arguments
///
/// * `data` - A non-empty slice of i32 values
///
/// # Returns
///
/// Index of the first maximum element, or None if empty
fn max_index(data: &[i32]) -> Option<usize> {
    data.iter()
        .enumerate()
        .max_by_key(|&(_, val)| *val)
        .map(|(i, _)| i)
}

fn main() {
    // --- 1. Slice from an array ---
    let arr = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[1..4]; // [20, 30, 40]
    println!("slice = {slice:?}, len = {}", slice.len());

    // --- 2. Whole array as a slice ---
    let full: &[i32] = &arr; // coercion: &[i32; 5] → &[i32]
    println!("full = {full:?}");

    // --- 3. Slice from a Vec ---
    let v = vec![100, 200, 300];
    let v_slice: &[i32] = &v[..2]; // [100, 200]
    println!("v_slice = {v_slice:?}");

    // --- 4. Functions accepting slices work with any source ---
    println!("sum(array):     {}", sum(&arr));
    println!("sum(vec):       {}", sum(&v));
    println!("sum(sub-slice): {}", sum(&arr[2..]));

    // --- 5. Slice properties ---
    let s = &arr[..];
    println!("first = {:?}", s.first()); // Some(&10)
    println!("last  = {:?}", s.last()); // Some(&50)
    println!("contains 30? {}", s.contains(&30));

    // --- 6. Iterating over a slice ---
    let numbers = [1, 2, 3, 4, 5];
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("evens = {evens:?}");

    // --- 7. Splitting slices ---
    let data = [1, 2, 0, 3, 4, 0, 5];
    let chunks: Vec<&[i32]> = data.split(|&x| x == 0).collect();
    println!("split on 0: {chunks:?}");

    // --- 8. max_index ---
    println!("max_index of arr: {:?}", max_index(&arr));
    println!("max_index of empty: {:?}", max_index(&[]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3]), 6);
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn test_sum_from_vec() {
        let v = vec![10, 20];
        assert_eq!(sum(&v), 30);
    }

    #[test]
    fn test_max_index() {
        assert_eq!(max_index(&[1, 5, 3]), Some(1));
        assert_eq!(max_index(&[10]), Some(0));
        assert_eq!(max_index(&[]), None);
    }

    #[test]
    fn test_slice_len() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(arr[1..4].len(), 3);
    }

    #[test]
    fn test_slice_contains() {
        let arr = [10, 20, 30];
        let s: &[i32] = &arr;
        assert!(s.contains(&20));
        assert!(!s.contains(&25));
    }
}
