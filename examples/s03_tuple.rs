#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates tuples in Rust.
//
// A tuple is a fixed-size, ordered collection of values of potentially
// different types. Tuples are one of Rust's two compound types.
//
// Rust tuples:
//   - Are 0-indexed with `.0`, `.1` syntax (not `[0]`, `[1]`)
//   - Have a maximum useful size (traits implemented up to 12)
//   - Are stack-allocated and have a fixed compile-time type
//
// Run: cargo run --example s03_tuple

fn main() {
    // --- 1. Basic tuple ---
    let tup: (i32, f64, &str) = (42, 3.14, "hello");
    println!("tup = {tup:?}");

    // --- 2. Access by index ---
    // Use `.0`, `.1`, `.2` syntax (not square brackets).
    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);
    println!("tup.2 = {}", tup.2);

    // Gotcha: indices must be literals, not variables.
    // let i = 0; tup.i; // error: no field `i` on type `(i32, f64, &str)`

    // --- 3. Destructuring ---
    let (x, y, z) = tup;
    println!("destructured: x={x}, y={y}, z={z}");

    // Ignore values with `_`:
    let (first, _, last) = tup;
    println!("first={first}, last={last}");

    // --- 4. Mutable tuples ---
    let mut pair = (1, 2);
    pair.0 = 10;
    pair.1 = 20;
    println!("mutated pair: {pair:?}");

    // --- 5. Unit tuple ---
    // `()` is the "unit" type — Rust's equivalent of "void" or "nothing".
    // Functions that don't return a value implicitly return `()`.
    let unit: () = ();
    println!("unit = {unit:?}");
    println!("size of (): {}", std::mem::size_of::<()>()); // 0 bytes

    // --- 6. Single-element tuple ---
    // Use a trailing comma to distinguish from a parenthesized expression.
    let single = (42,); // tuple with one element
    let not_tuple = (42); // just an integer in parentheses
    println!("single = {single:?}"); // (42,)
    println!("not_tuple = {not_tuple}"); // 42

    // --- 7. Returning tuples from functions ---
    // Tuples are great for returning multiple values.
    let (min, max) = min_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("min={min}, max={max}");

    // --- 8. Nested tuples ---
    let nested = ((1, 2), (3, 4));
    println!("nested.0.1 = {}", nested.0.1);
    println!("nested.1.0 = {}", nested.1.0);

    // --- 9. Tuple comparison ---
    // Tuples implement PartialOrd: compared element by element.
    let a = (1, 2, 3);
    let b = (1, 2, 4);
    println!();
    println!("(1,2,3) < (1,2,4)? {}", a < b); // true

    // --- Gotcha: trait implementations limited to 12 elements ---
    // Standard library traits (Debug, Clone, PartialEq, etc.) are only
    // implemented for tuples up to 12 elements. Larger tuples exist but
    // lack these trait implementations.
}

/// Find the minimum and maximum values in a slice.
///
/// # Arguments
///
/// * `values` - A non-empty slice of i32 values
///
/// # Returns
///
/// A tuple of (min, max)
///
/// # Panics
///
/// Panics if the slice is empty
fn min_max(values: &[i32]) -> (i32, i32) {
    let mut min = values[0];
    let mut max = values[0];
    for &v in &values[1..] {
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
    }
    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destructuring() {
        let t = (1, 2, 3);
        let (a, b, c) = t;
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
    }

    #[test]
    fn test_unit_size() {
        assert_eq!(std::mem::size_of::<()>(), 0);
    }

    #[test]
    fn test_single_element_tuple() {
        let t = (42,);
        assert_eq!(t.0, 42);
    }

    #[test]
    fn test_min_max() {
        assert_eq!(min_max(&[3, 1, 4, 1, 5]), (1, 5));
        assert_eq!(min_max(&[42]), (42, 42));
    }

    #[test]
    fn test_tuple_comparison() {
        assert!((1, 2) < (1, 3));
        assert!((1, 2) < (2, 0));
        assert_eq!((1, 2), (1, 2));
    }
}
