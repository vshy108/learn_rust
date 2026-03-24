// Demonstrates Deref coercion with array slices.
//
// Deref coercion automatically converts references between types when
// the source implements `Deref` targeting the destination:
//
// - `&Vec<T>` → `&[T]`   (Vec<T> implements Deref<Target = [T]>)
// - `&[T; N]` → `&[T]`   (arrays coerce to slices)
// - `&String`  → `&str`   (shown earlier in Section 6)
// - `&Box<T>`  → `&T`     (Box<T> implements Deref<Target = T>)
//
// This means functions accepting `&[T]` automatically work with
// Vec<T>, arrays, and other slice sources — no explicit conversion.
//
// Run: cargo run --example s08_deref_coercion_array

/// Prints all elements in a slice. Accepts `&[i32]`.
///
/// Thanks to Deref coercion, this works with:
/// - `&Vec<i32>` (auto-coerced to `&[i32]`)
/// - `&[i32; N]` (array reference coerced to `&[i32]`)
/// - `&[i32]` directly
///
/// # Arguments
///
/// * `data` - A slice of i32
fn print_all(data: &[i32]) {
    print!("[");
    for (i, val) in data.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{val}");
    }
    println!("]");
}

/// Returns true if all elements are positive.
///
/// # Arguments
///
/// * `data` - A slice of i32
///
/// # Returns
///
/// Whether every element is > 0
fn all_positive(data: &[i32]) -> bool {
    data.iter().all(|&x| x > 0)
}

fn main() {
    // --- 1. Vec<i32> auto-coerces to &[i32] ---
    let vec_data: Vec<i32> = vec![1, 2, 3, 4, 5];
    print!("Vec:   ");
    print_all(&vec_data); // &Vec<i32> → &[i32]

    // --- 2. Array [i32; N] coerces to &[i32] ---
    let arr_data: [i32; 3] = [10, 20, 30];
    print!("Array: ");
    print_all(&arr_data); // &[i32; 3] → &[i32]

    // --- 3. Already a slice ---
    let slice: &[i32] = &vec_data[1..4];
    print!("Slice: ");
    print_all(slice);

    // --- 4. Works in generic contexts ---
    println!("all_positive(vec): {}", all_positive(&vec_data));
    println!("all_positive(arr): {}", all_positive(&arr_data));
    println!("all_positive([-1]): {}", all_positive(&[-1]));

    // --- 5. Nested coercion: Box<[i32]> ---
    let boxed: Box<[i32]> = vec![7, 8, 9].into_boxed_slice();
    print!("Box:   ");
    print_all(&boxed); // &Box<[i32]> → &[i32]

    // --- 6. Coercion chain in method calls ---
    let v = vec![1, 2, 3];
    // v.first() works because Vec<T> derefs to [T],
    // and [T] has the .first() method.
    println!("first: {:?}", v.first());
    println!("last:  {:?}", v.last());

    // --- 7. Why this matters ---
    // Write ONE function taking &[T], and it works with Vec, arrays,
    // slices, and Box<[T]> — no function overloading needed.
    // This is a core Rust ergonomic pattern.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_positive_vec() {
        let v = vec![1, 2, 3];
        assert!(all_positive(&v));
    }

    #[test]
    fn test_all_positive_array() {
        assert!(all_positive(&[10, 20]));
        assert!(!all_positive(&[10, -1]));
    }

    #[test]
    fn test_all_positive_empty() {
        // An empty slice vacuously satisfies "all positive".
        assert!(all_positive(&[]));
    }

    #[test]
    fn test_coercion_from_boxed_slice() {
        let boxed: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
        assert!(all_positive(&boxed));
    }
}
