// Demonstrates defining methods on structs using `impl` blocks.
//
// Methods are functions defined inside an `impl` block that take
// `self` (or `&self` / `&mut self`) as their first parameter.
//
// Call them via dot notation: `instance.method()`.
//
// Run: cargo run --example s09_methods

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    /// Calculates the area of the circle.
    ///
    /// Takes `&self` — borrows the circle, doesn't consume it.
    ///
    /// # Returns
    ///
    /// The area as f64
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    /// Calculates the circumference.
    ///
    /// # Returns
    ///
    /// The circumference as f64
    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    /// Checks if this circle is larger than another.
    ///
    /// # Arguments
    ///
    /// * `other` - Another Circle to compare against
    ///
    /// # Returns
    ///
    /// true if self has a larger radius
    fn is_larger_than(&self, other: &Circle) -> bool {
        self.radius > other.radius
    }
}

fn main() {
    let c = Circle { radius: 5.0 };

    // --- 1. Calling methods with dot syntax ---
    println!("Circle: {c:?}");
    println!("Area: {:.2}", c.area());
    println!("Circumference: {:.2}", c.circumference());

    // --- 2. Method with another struct parameter ---
    let small = Circle { radius: 2.0 };
    println!("{:?} > {:?}? {}", c, small, c.is_larger_than(&small));
    println!("{:?} > {:?}? {}", small, c, small.is_larger_than(&c));

    // --- 3. Method call on reference ---
    let r = &c;
    // Rust auto-dereferences: r.area() is the same as (*r).area()
    println!("Area via ref: {:.2}", r.area());

    // --- 4. Chaining methods (when they return values) ---
    let ratio = c.area() / c.circumference();
    println!("Area/Circumference ratio: {ratio:.2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let c = Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_circumference() {
        let c = Circle { radius: 1.0 };
        let expected = 2.0 * std::f64::consts::PI;
        assert!((c.circumference() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_is_larger_than() {
        let big = Circle { radius: 10.0 };
        let small = Circle { radius: 1.0 };
        assert!(big.is_larger_than(&small));
        assert!(!small.is_larger_than(&big));
    }

    #[test]
    fn test_zero_radius() {
        let c = Circle { radius: 0.0 };
        assert!((c.area() - 0.0).abs() < f64::EPSILON);
    }
}
