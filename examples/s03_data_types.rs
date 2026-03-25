// Demonstrates Rust's data types overview.
//
// Rust is statically typed — every value has a known type at compile time.
// Types fall into two broad categories:
//   - Scalar: single values (integers, floats, booleans, characters)
//   - Compound: group values (tuples, arrays)
//
// The compiler can usually infer types, but sometimes you need annotations.
//
// Run: cargo run --example s03_data_types

fn main() {
    // --- 1. Integer types ---
    // Signed:   i8, i16, i32, i64, i128, isize
    // Unsigned: u8, u16, u32, u64, u128, usize
    // Default:  i32 (when Rust can't infer a more specific type)
    let signed: i32 = -42;
    let unsigned: u32 = 42;
    let big: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    println!("signed={signed}, unsigned={unsigned}");
    println!("i128 max = {big}");

    // --- 2. Floating-point types ---
    // f32: 32-bit (single precision), ~7 decimal digits
    // f64: 64-bit (double precision), ~15 decimal digits (default)
    let float32: f32 = 3.14;
    let float64: f64 = 3.141592653589793;
    println!("f32={float32}, f64={float64}");

    // --- 3. Boolean type ---
    // `bool`: takes 1 byte, values are `true` or `false`
    let is_rust_fun: bool = true;
    println!("is_rust_fun = {is_rust_fun}");

    // --- 4. Character type ---
    // `char`: 4 bytes, represents a Unicode scalar value (not just ASCII)
    let letter = 'A';
    let emoji = '~'; // Using a safe ASCII char to avoid emoji
    let chinese = '\u{4e16}'; // Unicode escape for a Chinese character
    println!("letter={letter}, safe_char={emoji}, chinese={chinese}");

    // --- 5. Tuple ---
    // Fixed-size, ordered collection of values with potentially different types.
    let tup: (i32, f64, &str) = (42, 6.28, "hello");
    let (a, b, c) = tup; // destructure
    println!("tuple: a={a}, b={b}, c={c}");
    println!("tuple.0 = {}", tup.0); // index access

    // --- 6. Array ---
    // Fixed-size collection of values of the SAME type, stack-allocated.
    // Syntax: [type; length]
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {arr:?}");
    println!("arr[0] = {}, arr.len() = {}", arr[0], arr.len());

    // Repeat syntax: [value; count]
    let zeros = [0u8; 10];
    println!("zeros: {zeros:?}");

    // --- 7. Type ranges ---
    // Each integer type has MIN and MAX associated constants.
    println!();
    println!("Type ranges:");
    println!("  i8:   {} to {}", i8::MIN, i8::MAX);
    println!("  u8:   {} to {}", u8::MIN, u8::MAX);
    println!("  i16:  {} to {}", i16::MIN, i16::MAX);
    println!("  i32:  {} to {}", i32::MIN, i32::MAX);
    println!("  i64:  {} to {}", i64::MIN, i64::MAX);
    println!("  i128: {} to {}", i128::MIN, i128::MAX);
    println!("  usize: {} to {}", usize::MIN, usize::MAX);

    // --- Gotcha: no implicit type conversion ---
    // Rust NEVER implicitly converts between types.
    // `let x: i64 = 42i32;`  // error[E0308]: mismatched types
    // You must use explicit conversion: `42i32 as i64` or `.into()`
    let x: i64 = i64::from(42i32);
    println!("explicit conversion: {x}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_integer_defaults() {
        let n = 42; // defaults to i32
        assert_eq!(std::mem::size_of_val(&n), 4);
    }

    #[test]
    fn test_float_default() {
        let f = 3.14; // defaults to f64
        assert_eq!(std::mem::size_of_val(&f), 8);
    }

    #[test]
    fn test_bool_size() {
        assert_eq!(std::mem::size_of::<bool>(), 1);
    }

    #[test]
    fn test_char_size() {
        assert_eq!(std::mem::size_of::<char>(), 4);
    }

    #[test]
    fn test_tuple_destructuring() {
        let t: (i32, f64, &str) = (1, 2.0, "three");
        let (a, b, c) = t;
        assert_eq!(a, 1);
        assert!((b - 2.0).abs() < f64::EPSILON);
        assert_eq!(c, "three");
    }

    #[test]
    fn test_array_repeat() {
        let arr = [7; 3];
        assert_eq!(arr, [7, 7, 7]);
    }
}
