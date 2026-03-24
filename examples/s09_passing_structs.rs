// Demonstrates passing structs into functions.
//
// Structs follow the same ownership rules as other types:
// - Pass by value (`T`) → ownership moves to the function.
// - Pass by reference (`&T`) → function borrows, caller keeps ownership.
// - Pass by mutable reference (`&mut T`) → function can modify.
//
// Best practice: prefer `&T` for read-only, `&mut T` for modification.
// Only take ownership when the function needs to consume the value.
//
// Run: cargo run --example s09_passing_structs

#[derive(Debug, Clone)]
struct Rectangle {
    width: f64,
    height: f64,
}

/// Calculates the area (borrows — doesn't take ownership).
///
/// # Arguments
///
/// * `rect` - Shared reference to a Rectangle
///
/// # Returns
///
/// The area as f64
fn area(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}

/// Scales the rectangle in place (mutable borrow).
///
/// # Arguments
///
/// * `rect` - Mutable reference to a Rectangle
/// * `factor` - Scale factor
fn scale(rect: &mut Rectangle, factor: f64) {
    rect.width *= factor;
    rect.height *= factor;
}

/// Consumes the rectangle and returns a description string.
///
/// # Arguments
///
/// * `rect` - A Rectangle (takes ownership)
///
/// # Returns
///
/// A description string
fn describe(rect: Rectangle) -> String {
    format!("{}x{} rectangle", rect.width, rect.height)
    // rect is dropped here — caller no longer has it
}

fn main() {
    // --- 1. Borrow: read-only ---
    let r = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    println!("Area: {}", area(&r));
    println!("Still have r: {r:?}");

    // --- 2. Mutable borrow: modify in place ---
    let mut r2 = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("Before scale: {r2:?}");
    scale(&mut r2, 2.0);
    println!("After scale:  {r2:?}");

    // --- 3. Pass by value: ownership transfer ---
    let r3 = Rectangle {
        width: 7.0,
        height: 2.0,
    };
    let desc = describe(r3); // r3 is moved
    println!("{desc}");
    // println!("{r3:?}"); // error: value used after move

    // --- 4. Clone if you need to keep the original ---
    let r4 = Rectangle {
        width: 5.0,
        height: 5.0,
    };
    let desc2 = describe(r4.clone()); // clone is consumed
    println!("{desc2}");
    println!("r4 still here: {r4:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let r = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert!((area(&r) - 12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_scale() {
        let mut r = Rectangle {
            width: 2.0,
            height: 3.0,
        };
        scale(&mut r, 2.0);
        assert!((r.width - 4.0).abs() < f64::EPSILON);
        assert!((r.height - 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_describe() {
        let r = Rectangle {
            width: 10.0,
            height: 20.0,
        };
        let d = describe(r);
        assert_eq!(d, "10x20 rectangle");
    }
}
