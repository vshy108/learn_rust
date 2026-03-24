// Demonstrates creating structs inside a function and returning them.
//
// Functions can construct and return structs. Ownership of the struct
// transfers to the caller — no special syntax needed.
//
// Run: cargo run --example s09_create_in_function

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

/// Creates a square (a rectangle with equal width and height).
///
/// # Arguments
///
/// * `size` - The side length
///
/// # Returns
///
/// A Rectangle where width == height
fn square(size: f64) -> Rectangle {
    Rectangle {
        width: size,
        height: size,
    }
}

/// Creates a rectangle from width and height, ensuring both are positive.
///
/// # Arguments
///
/// * `width` - Rectangle width (must be > 0)
/// * `height` - Rectangle height (must be > 0)
///
/// # Returns
///
/// Ok(Rectangle) or Err with a message
///
/// # Errors
///
/// Returns an error string if width or height is not positive
fn try_create(width: f64, height: f64) -> Result<Rectangle, String> {
    if width <= 0.0 {
        return Err(format!("width must be positive, got {width}"));
    }
    if height <= 0.0 {
        return Err(format!("height must be positive, got {height}"));
    }
    Ok(Rectangle { width, height })
}

fn main() {
    // --- 1. Simple factory function ---
    let sq = square(5.0);
    println!("Square: {sq:?}");

    // --- 2. Fallible construction ---
    match try_create(10.0, 20.0) {
        Ok(r) => println!("Created: {r:?}"),
        Err(e) => println!("Error: {e}"),
    }

    match try_create(-1.0, 5.0) {
        Ok(r) => println!("Created: {r:?}"),
        Err(e) => println!("Error: {e}"),
    }

    // --- 3. Creating structs conditionally ---
    let rects: Vec<Rectangle> = [1.0, 2.0, 3.0].iter().map(|&s| square(s)).collect();
    for r in &rects {
        println!("  {r:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        let s = square(4.0);
        assert!((s.width - 4.0).abs() < f64::EPSILON);
        assert!((s.height - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_try_create_ok() {
        let r = try_create(3.0, 4.0).unwrap();
        assert!((r.width - 3.0).abs() < f64::EPSILON);
        assert!((r.height - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_try_create_err_width() {
        assert!(try_create(-1.0, 5.0).is_err());
    }

    #[test]
    fn test_try_create_err_height() {
        assert!(try_create(5.0, 0.0).is_err());
    }
}
