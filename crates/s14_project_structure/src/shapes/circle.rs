// Topic 134: Submodule — circle.rs inside shapes/ folder.
//
// Declared in shapes/mod.rs with: `pub mod circle;`

/// A circle with a given radius.
#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    /// Creates a new circle with the given radius.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radius (must be non-negative)
    pub fn new(radius: f64) -> Self {
        Self {
            radius: radius.abs(),
        }
    }

    /// Calculates the area.
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    /// Calculates the circumference.
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    /// Returns the radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let c = Circle::new(1.0);
        assert!((c.area() - std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_circumference() {
        let c = Circle::new(1.0);
        assert!((c.circumference() - 2.0 * std::f64::consts::PI).abs() < 0.001);
    }
}
