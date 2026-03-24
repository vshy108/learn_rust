// Demonstrates documentation comments in Rust.
//
// Two types:
// - `///` — documents the item BELOW it (functions, structs, etc.)
// - `//!` — documents the enclosing item (module, crate)
//
// Doc comments use Markdown syntax and support code examples
// that are automatically tested with `cargo test`.
//
// Generate docs: `cargo doc --open`
//
// Run: cargo run --example s14_doc_comments

//! # Documentation Comments Example
//!
//! This example shows how to write documentation in Rust.
//! This `//!` comment documents the example crate itself.

/// A simple 2D point.
///
/// # Examples
///
/// ```
/// let p = Point::new(3.0, 4.0);
/// assert!((p.distance_from_origin() - 5.0).abs() < f64::EPSILON);
/// ```
#[derive(Debug, Clone, PartialEq)]
struct Point {
    /// The x-coordinate.
    x: f64,
    /// The y-coordinate.
    y: f64,
}

impl Point {
    /// Creates a new Point at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate
    /// * `y` - The y-coordinate
    ///
    /// # Returns
    ///
    /// A new `Point` instance
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculates the Euclidean distance from the origin (0, 0).
    ///
    /// Uses the formula: sqrt(x^2 + y^2)
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Point::new(3.0, 4.0);
    /// assert!((p.distance_from_origin() - 5.0).abs() < f64::EPSILON);
    /// ```
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Calculates the Manhattan distance from the origin.
    ///
    /// Manhattan distance = |x| + |y|
    fn manhattan_distance(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }
}

/// Adds two numbers together.
///
/// # Arguments
///
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
///
/// The sum of `a` and `b`
///
/// # Panics
///
/// This function does not panic.
///
/// # Examples
///
/// ```
/// assert_eq!(add(2, 3), 5);
/// assert_eq!(add(-1, 1), 0);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // --- 1. Using documented items ---
    let p = Point::new(3.0, 4.0);
    println!("Point: {p:?}");
    println!("Distance from origin: {}", p.distance_from_origin());
    println!("Manhattan distance: {}", p.manhattan_distance());

    // --- 2. add ---
    println!("add(2, 3) = {}", add(2, 3));

    // --- 3. Doc comment syntax summary ---
    println!();
    println!("Doc comment types:");
    println!("  /// - documents the next item");
    println!("  //! - documents the enclosing item");
    println!();
    println!("Common sections:");
    println!("  # Examples - code that gets tested!");
    println!("  # Arguments - parameter descriptions");
    println!("  # Returns - what the function returns");
    println!("  # Errors - when it returns Err");
    println!("  # Panics - when it might panic");
    println!();
    println!("Generate docs: cargo doc --open");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p = Point::new(3.0, 4.0);
        assert!((p.distance_from_origin() - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_manhattan() {
        let p = Point::new(-3.0, 4.0);
        assert!((p.manhattan_distance() - 7.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
