// Demonstrates the Copy trait — types that are duplicated on assignment.
//
// In Rust, `Copy` means "bitwise copy on assignment." The original value
// remains valid after the copy. Only simple, stack-only types implement Copy.
//
// Types that implement Copy: i32, f64, bool, char, tuples of Copy types,
// arrays of Copy types, &T (shared references).
//
// Types that do NOT: String, Vec, Box (they own heap data).
//
// Run: cargo run --example s06_copy_trait

fn main() {
    // --- 1. Copy on assignment ---
    let a = 42;
    let b = a; // bits copied; `a` still valid
    println!("a = {a}, b = {b}");

    // --- 2. Move on assignment (non-Copy type) ---
    let s1 = String::from("hello");
    let _s2 = s1; // s1 is MOVED, not copied
    // println!("{s1}"); // compile error: value used after move

    // --- 3. Which types are Copy? ---
    // All scalar types: i8..i128, u8..u128, f32, f64, bool, char.
    // Tuples of Copy types: (i32, f64) is Copy. (i32, String) is NOT.
    // Fixed arrays of Copy types: [i32; 5] is Copy.
    // Shared references: &T is Copy (it's just a pointer).
    let tuple = (1, 2.0, true);
    let tuple2 = tuple;
    println!("tuple = {tuple:?}, tuple2 = {tuple2:?}");

    // --- 4. Copy is implicit ---
    // You don't call `.copy()`. It happens automatically on assignment,
    // function calls, and pattern matching. This is different from `.clone()`
    // which is explicit.

    // --- 5. Implementing Copy on your own types ---
    // You can derive Copy + Clone for structs with only Copy fields.
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    let p = Point { x: 1.0, y: 2.0 };
    let p2 = p; // copied
    println!("p = {p:?}, p2 = {p2:?}");

    // --- 6. Cannot derive Copy if any field is non-Copy ---
    // This would NOT compile:
    // #[derive(Copy, Clone)]
    // struct Named { name: String }
    // Because String is not Copy (it owns heap data).

    // --- 7. Copy requires Clone ---
    // Copy is a sub-trait of Clone: `trait Copy: Clone {}`.
    // Every Copy type is also Clone, but not vice versa.
    // Clone provides `.clone()` — an explicit deep-ish copy.
    // Copy provides implicit bitwise copy on assignment.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_i32_is_copy() {
        let a = 42;
        let b = a;
        assert_eq!(a, b); // `a` still valid
    }

    #[test]
    fn test_tuple_of_copy_is_copy() {
        let t = (1, 2.0, true);
        let t2 = t;
        assert_eq!(t, t2);
    }

    #[test]
    fn test_array_of_copy_is_copy() {
        let a = [1, 2, 3];
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_custom_copy_struct() {
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Point {
            x: f64,
            y: f64,
        }
        let p = Point { x: 1.0, y: 2.0 };
        let p2 = p;
        assert_eq!(p, p2);
    }
}
