// Demonstrates PartialEq for enums.
//
// For enums, PartialEq compares variants AND their contained data.
// Derive works for most cases; manual impl for custom logic.
//
// Run: cargo run --example s18_partial_eq_enum

/// Derived PartialEq — variants and data must match.
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

/// Derived with data variants.
#[derive(Debug, PartialEq)]
#[allow(dead_code)] // Triangle shown for completeness
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

/// Manual PartialEq — custom logic.
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Suspended(String), // reason
}

impl Status {
    /// Returns the suspension reason, if any.
    fn reason(&self) -> Option<&str> {
        match self {
            Status::Suspended(r) => Some(r),
            _ => None,
        }
    }
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        // Suspended is equal regardless of reason:
        matches!(
            (self, other),
            (Status::Active, Status::Active)
                | (Status::Inactive, Status::Inactive)
                | (Status::Suspended(_), Status::Suspended(_))
        )
    }
}

fn main() {
    // --- Derived enum comparison ---
    assert_eq!(Direction::North, Direction::North);
    assert_ne!(Direction::North, Direction::South);
    println!("North == North: true");
    println!("North == South: false");

    println!();

    // --- With data ---
    let s1 = Shape::Circle(5.0);
    let s2 = Shape::Circle(5.0);
    let s3 = Shape::Circle(10.0);
    println!("Circle(5) == Circle(5): {}", s1 == s2); // true
    println!("Circle(5) == Circle(10): {}", s1 == s3); // false

    let r = Shape::Rectangle(3.0, 4.0);
    println!("Circle(5) == Rect(3,4): {}", s1 == r); // false

    println!();

    // --- Custom: Suspended ignores reason ---
    let a = Status::Suspended("spam".into());
    let b = Status::Suspended("fraud".into());
    println!("Suspended(spam) == Suspended(fraud): {}", a == b); // true!
    println!("Reason for a: {:?}", a.reason());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_eq() {
        assert_eq!(Direction::East, Direction::East);
        assert_ne!(Direction::East, Direction::West);
    }

    #[test]
    fn test_shape_eq() {
        assert_eq!(Shape::Circle(1.0), Shape::Circle(1.0));
        assert_ne!(Shape::Circle(1.0), Shape::Circle(2.0));
        assert_ne!(Shape::Circle(1.0), Shape::Rectangle(1.0, 1.0));
    }

    #[test]
    fn test_custom_status_eq() {
        assert_eq!(Status::Active, Status::Active);
        assert_ne!(Status::Active, Status::Inactive);
        // Different reasons are equal:
        assert_eq!(Status::Suspended("a".into()), Status::Suspended("b".into()));
    }
}
