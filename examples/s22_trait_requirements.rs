// Demonstrates trait requirements for types used in testing.
//
// `assert_eq!` requires Debug + PartialEq on the types it compares.
// Without these traits, your types can't be used in assert_eq!/assert_ne!.
//
// Run: cargo test --example s22_trait_requirements

/// A Point type with all needed traits for testing.
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// Without Debug: can't use in assert_eq! (won't show value on failure)
// Without PartialEq: can't compare with ==

/// Demonstrate Display vs Debug for testing.
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 4.0);
    println!("distance from {a} to {b} = {}", a.distance_to(&b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_equality() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(1.0, 2.0);
        assert_eq!(a, b); // requires PartialEq + Debug
    }

    #[test]
    fn test_point_inequality() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(3.0, 4.0);
        assert_ne!(a, b);
    }

    #[test]
    fn test_distance() {
        let origin = Point::new(0.0, 0.0);
        let p = Point::new(3.0, 4.0);
        let dist = origin.distance_to(&p);
        // Floating point: use approximate comparison
        assert!((dist - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_clone() {
        let a = Point::new(1.0, 2.0);
        let b = a.clone();
        assert_eq!(a, b); // clone produces equal value
    }
}
