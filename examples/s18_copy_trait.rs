// Demonstrates implementing the Copy trait.
//
// `Copy` makes assignment a bitwise copy instead of a move.
// `Copy` requires `Clone`. Only for simple, stack-only types.
//
// Gotcha: Types with heap data (String, Vec, Box) can NEVER be Copy.
//
// Rule of thumb: if `drop` would need to free resources, it can't be Copy.
//
// Run: cargo run --example s18_copy_trait

/// Copy type — small, stack-only data.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

/// Copy type — newtype over a primitive.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Degrees(f64);

/// NOT Copy — contains String (heap data).
#[derive(Debug, Clone, PartialEq)]
struct Label {
    name: String,
}

fn main() {
    // --- Copy semantics: assignment copies, original still valid ---
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1; // Copy, not move!
    println!("p1: {p1:?}"); // p1 still valid!
    println!("p2: {p2:?}");

    // --- Move semantics: non-Copy type ---
    let l1 = Label {
        name: "hello".into(),
    };
    let _l2 = l1.clone(); // Must clone explicitly
    // let _l3 = l1; // This would MOVE l1
    // println!("{l1:?}"); // ERROR if moved

    // --- Copy in function calls ---
    let d = Degrees(90.0);
    take_degrees(d);
    println!("d still valid: {d:?}"); // Copy means no move

    // --- Copy in loops ---
    let p = Point { x: 0.0, y: 0.0 };
    for _ in 0..3 {
        use_point(p); // Copy each iteration — no move!
    }

    println!();
    println!("Copy types: i32, f64, bool, char, (tuples of Copy types)");
    println!("NOT Copy:   String, Vec, Box, any type with Drop");
}

fn take_degrees(d: Degrees) {
    println!("Got degrees: {d:?}");
}

fn use_point(p: Point) {
    println!("Using point: {p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_semantics() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = p1; // Copy
        assert_eq!(p1, p2); // Both valid
    }

    #[test]
    fn test_copy_in_function() {
        let d = Degrees(45.0);
        take_degrees(d);
        assert_eq!(d, Degrees(45.0)); // Still valid after passing
    }
}
