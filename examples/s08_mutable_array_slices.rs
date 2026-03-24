// Demonstrates mutable array slices.
//
// A mutable slice `&mut [T]` lets you modify elements in-place
// without taking ownership of the underlying array or Vec.
//
// Rules (same as mutable references):
// - Only ONE `&mut [T]` to the same data at a time.
// - No `&[T]` while `&mut [T]` exists.
// - You CAN have multiple non-overlapping mutable sub-slices
//   via `split_at_mut()`.
//
// Run: cargo run --example s08_mutable_array_slices

/// Doubles every element in the slice.
///
/// # Arguments
///
/// * `data` - Mutable slice of i32
fn double_all(data: &mut [i32]) {
    for val in data.iter_mut() {
        *val *= 2;
    }
}

/// Sorts and deduplicates a mutable slice, returning the number of unique elements.
///
/// # Arguments
///
/// * `data` - Mutable slice of i32 (will be sorted)
///
/// # Returns
///
/// A slice of the unique sorted elements
fn sort_dedup(data: &mut Vec<i32>) -> Vec<i32> {
    data.sort();
    data.dedup();
    data.clone()
}

/// Reverses a slice in place manually (to show mutable slice indexing).
///
/// # Arguments
///
/// * `data` - Mutable slice of i32
fn reverse_in_place(data: &mut [i32]) {
    let len = data.len();
    for i in 0..len / 2 {
        data.swap(i, len - 1 - i);
    }
}

fn main() {
    // --- 1. Mutating array elements through a slice ---
    let mut arr = [1, 2, 3, 4, 5];
    println!("before: {arr:?}");
    double_all(&mut arr);
    println!("after double_all: {arr:?}");

    // --- 2. Mutating Vec elements through a slice ---
    let mut v = vec![10, 20, 30];
    double_all(&mut v); // &mut Vec<i32> coerces to &mut [i32]
    println!("vec after double_all: {v:?}");

    // --- 3. Mutating a sub-slice ---
    let mut data = [1, 2, 3, 4, 5];
    double_all(&mut data[1..4]); // only doubles indices 1, 2, 3
    println!("partial double: {data:?}"); // [1, 4, 6, 8, 5]

    // --- 4. split_at_mut: two non-overlapping mutable sub-slices ---
    let mut arr = [10, 20, 30, 40, 50];
    let (left, right) = arr.split_at_mut(2);
    // left = &mut [10, 20], right = &mut [30, 40, 50]
    left[0] = 0;
    right[2] = 0;
    println!("split_at_mut: {arr:?}"); // [0, 20, 30, 40, 0]

    // --- 5. Reverse in place ---
    let mut nums = [5, 3, 8, 1, 9];
    println!("before reverse: {nums:?}");
    reverse_in_place(&mut nums);
    println!("after reverse:  {nums:?}");

    // --- 6. fill() and fill_with() ---
    let mut buf = [0i32; 5];
    buf.fill(42);
    println!("filled: {buf:?}");

    let mut counter: i32 = 0;
    buf.fill_with(|| {
        counter += 1;
        counter
    });
    println!("fill_with: {buf:?}");

    // --- 7. sort_dedup ---
    let mut values = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let unique = sort_dedup(&mut values);
    println!("unique sorted: {unique:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_all() {
        let mut arr = [1, 2, 3];
        double_all(&mut arr);
        assert_eq!(arr, [2, 4, 6]);
    }

    #[test]
    fn test_double_sub_slice() {
        let mut arr = [1, 2, 3, 4];
        double_all(&mut arr[1..3]);
        assert_eq!(arr, [1, 4, 6, 4]);
    }

    #[test]
    fn test_reverse_in_place() {
        let mut arr = [1, 2, 3, 4, 5];
        reverse_in_place(&mut arr);
        assert_eq!(arr, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_empty() {
        let mut arr: [i32; 0] = [];
        reverse_in_place(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_sort_dedup() {
        let mut v = vec![3, 1, 2, 2, 3];
        let result = sort_dedup(&mut v);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_split_at_mut() {
        let mut arr = [1, 2, 3, 4];
        let (left, right) = arr.split_at_mut(2);
        left[0] = 10;
        right[0] = 30;
        assert_eq!(arr, [10, 2, 30, 4]);
    }
}
