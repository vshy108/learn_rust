// Topic 134: Submodule — rectangle.rs inside shapes/ folder.
//
// Declared in shapes/mod.rs with: `pub mod rectangle;`

/// A rectangle with width and height.
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    /// Creates a new rectangle.
    ///
    /// # Arguments
    ///
    /// * `width` - The width
    /// * `height` - The height
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width: width.abs(),
            height: height.abs(),
        }
    }

    /// Calculates the area.
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Calculates the perimeter.
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    /// Returns true if the rectangle is a square.
    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let r = Rectangle::new(3.0, 4.0);
        assert!((r.area() - 12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_perimeter() {
        let r = Rectangle::new(3.0, 4.0);
        assert!((r.perimeter() - 14.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_is_square() {
        assert!(Rectangle::new(5.0, 5.0).is_square());
        assert!(!Rectangle::new(3.0, 4.0).is_square());
    }
}
