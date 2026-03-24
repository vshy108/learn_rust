// Demonstrates array declaration in Rust.
//
// Arrays in Rust are fixed-size, stack-allocated collections of the same type.
// Syntax: `[T; N]` — T is the element type, N is the compile-time length.
//
// Unlike Python lists, Rust arrays:
//   - Have a fixed size known at compile time
//   - Are stack-allocated (no heap overhead)
//   - Are bounds-checked at runtime (panics on out-of-bounds access)
//
// Run: cargo run --example s03_array_declaration

fn main() {
    // --- 1. Basic declaration ---
    let arr = [1, 2, 3, 4, 5]; // type inferred as [i32; 5]
    println!("arr = {arr:?}");

    // Explicit type annotation:
    let arr2: [f64; 3] = [1.1, 2.2, 3.3];
    println!("arr2 = {arr2:?}");

    // --- 2. Repeat initialization ---
    // [value; count] creates an array filled with `value`.
    let zeros = [0u8; 10];
    println!("zeros = {zeros:?}");

    let all_true = [true; 5];
    println!("all_true = {all_true:?}");

    // --- 3. Accessing elements ---
    println!();
    println!("arr[0] = {}", arr[0]); // 1
    println!("arr[4] = {}", arr[4]); // 5
    println!("arr.len() = {}", arr.len());

    // Out-of-bounds PANICS at runtime:
    // arr[5]; // thread 'main' panicked at 'index out of bounds'

    // --- 4. Iterating ---
    print!("for loop: ");
    for val in &arr {
        print!("{val} ");
    }
    println!();

    // With index:
    print!("enumerate: ");
    for (i, val) in arr.iter().enumerate() {
        print!("[{i}]={val} ");
    }
    println!();

    // --- 5. Mutable arrays ---
    let mut mutable = [10, 20, 30];
    mutable[1] = 99;
    println!("mutable after change: {mutable:?}");

    // --- 6. Array slicing ---
    // `&arr[start..end]` creates a slice (a view, not a copy).
    let slice = &arr[1..4];
    println!("slice [1..4] = {slice:?}"); // [2, 3, 4]

    // --- 7. Array methods ---
    println!();
    println!("contains 3? {}", arr.contains(&3));
    println!("starts_with [1,2]? {}", arr.starts_with(&[1, 2]));

    // .map() transforms each element (Rust 1.55+)
    let doubled = arr.map(|x| x * 2);
    println!("doubled = {doubled:?}");

    // --- 8. Multidimensional arrays ---
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!();
    println!("matrix = {matrix:?}");
    println!("matrix[1][2] = {}", matrix[1][2]); // 6

    // --- 9. Arrays implement Copy if elements do ---
    let a = [1, 2, 3];
    let b = a; // copies, not moves
    println!("a = {a:?}, b = {b:?}"); // both valid

    // --- Gotcha: length is part of the type ---
    // [i32; 3] and [i32; 5] are DIFFERENT types.
    // You can't assign one to the other.
    // fn take_array(a: [i32; 3]) {} // only accepts exactly 3 elements
    // For variable-length, use slices (&[T]) or Vec<T>.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_array_len() {
        let a = [1, 2, 3, 4, 5];
        assert_eq!(a.len(), 5);
    }

    #[test]
    fn test_repeat_init() {
        let a = [7; 4];
        assert_eq!(a, [7, 7, 7, 7]);
    }

    #[test]
    fn test_slice() {
        let a = [10, 20, 30, 40, 50];
        assert_eq!(&a[1..4], &[20, 30, 40]);
    }

    #[test]
    fn test_map() {
        let a = [1, 2, 3];
        let b = a.map(|x| x * 10);
        assert_eq!(b, [10, 20, 30]);
    }

    #[test]
    fn test_multidimensional() {
        let m = [[1, 2], [3, 4]];
        assert_eq!(m[0][0], 1);
        assert_eq!(m[1][1], 4);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds_panics() {
        let a = [1, 2, 3];
        // `black_box` hides the value from the compiler so it can't
        // const-fold the index and trigger `unconditional_panic`.
        let idx = std::hint::black_box(3);
        let _val = a[idx]; // panics at runtime
    }
}
