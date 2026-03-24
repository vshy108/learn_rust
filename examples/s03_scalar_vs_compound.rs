// Demonstrates scalar vs compound types in Rust.
//
// Rust's type system divides primitive types into two categories:
//   - Scalar: represents a single value
//   - Compound: groups multiple values into one type
//
// Run: cargo run --example s03_scalar_vs_compound

fn main() {
    // =============================================
    //  SCALAR TYPES (single value)
    // =============================================

    // --- 1. Integers ---
    // Signed: i8, i16, i32, i64, i128, isize
    // Unsigned: u8, u16, u32, u64, u128, usize
    let integer: i32 = -42;
    println!("Integer (scalar): {integer}");

    // --- 2. Floating-point ---
    // f32 (32-bit), f64 (64-bit, default)
    let float: f64 = 3.14;
    println!("Float (scalar):   {float}");

    // --- 3. Boolean ---
    // bool: true or false (1 byte)
    let boolean: bool = true;
    println!("Bool (scalar):    {boolean}");

    // --- 4. Character ---
    // char: Unicode scalar value (4 bytes)
    let character: char = 'R';
    println!("Char (scalar):    '{character}'");

    println!();

    // =============================================
    //  COMPOUND TYPES (group of values)
    // =============================================

    // --- 5. Tuple ---
    // Fixed-length, ordered, mixed types.
    // Access by index (`.0`, `.1`) or destructuring.
    let tuple: (i32, f64, &str) = (42, 3.14, "hello");
    let (a, b, c) = tuple;
    println!("Tuple (compound): ({a}, {b}, \"{c}\")");
    println!("  tuple.0 = {}", tuple.0);

    // Unit tuple `()` — the "void" type in Rust.
    let unit: () = ();
    println!("  Unit tuple: {unit:?}");

    // --- 6. Array ---
    // Fixed-length, same type, stack-allocated.
    // Syntax: [T; N] where T is the type and N is the length.
    let array: [i32; 4] = [10, 20, 30, 40];
    println!("Array (compound): {array:?}");
    println!("  array[0] = {}, len = {}", array[0], array.len());

    println!();

    // =============================================
    //  KEY DIFFERENCES
    // =============================================
    println!("Scalar types:   integers, floats, bool, char");
    println!("Compound types: tuples, arrays");
    println!();
    println!("Key differences:");
    println!("  Scalar:   Copy by default, single value");
    println!("  Compound: behavior depends on contents");
    println!("  Tuple:    fixed size, mixed types, max 12 elements for traits");
    println!("  Array:    fixed size, same type, bounds-checked at runtime");

    // --- Compound types implement Copy if all fields are Copy ---
    let arr1 = [1, 2, 3]; // [i32; 3] implements Copy (since i32: Copy)
    let arr2 = arr1; // copy, not move
    println!();
    println!("arr1 = {arr1:?}"); // still valid
    println!("arr2 = {arr2:?}");

    // --- Gotcha: arrays are NOT the same as vectors ---
    // Arrays: [T; N], fixed size, stack-allocated, known at compile time
    // Vectors: Vec<T>, growable, heap-allocated, known at runtime
    // Python's `list` is most similar to Rust's `Vec<T>`.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scalar_sizes() {
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::size_of::<f64>(), 8);
        assert_eq!(std::mem::size_of::<bool>(), 1);
        assert_eq!(std::mem::size_of::<char>(), 4);
    }

    #[test]
    fn test_tuple_access() {
        let t: (i32, f64, bool) = (1, 2.0, true);
        assert_eq!(t.0, 1);
        assert!((t.1 - 2.0).abs() < f64::EPSILON);
        assert!(t.2);
    }

    #[test]
    fn test_array_copy() {
        let a = [1, 2, 3];
        let b = a; // copy
        assert_eq!(a, b); // both valid
    }

    #[test]
    fn test_unit_type() {
        let u: () = ();
        assert_eq!(std::mem::size_of_val(&u), 0);
    }
}
