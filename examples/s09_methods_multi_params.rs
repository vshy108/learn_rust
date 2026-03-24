// Demonstrates methods with multiple parameters.
//
// Methods can take additional parameters after `self`/`&self`/`&mut self`.
// The first parameter is always the receiver (the struct instance).
//
// Run: cargo run --example s09_methods_multi_params

#[derive(Debug, Clone, PartialEq)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rect {
    /// Creates a new rectangle.
    fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    /// Checks if a point (px, py) is inside the rectangle.
    ///
    /// # Arguments
    ///
    /// * `px` - Point x coordinate
    /// * `py` - Point y coordinate
    ///
    /// # Returns
    ///
    /// true if the point is inside (inclusive of edges)
    fn contains_point(&self, px: f64, py: f64) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }

    /// Checks if this rectangle overlaps with another.
    ///
    /// # Arguments
    ///
    /// * `other` - Another Rectangle
    ///
    /// # Returns
    ///
    /// true if the rectangles overlap
    fn overlaps(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Moves the rectangle by (dx, dy).
    ///
    /// # Arguments
    ///
    /// * `dx` - Horizontal displacement
    /// * `dy` - Vertical displacement
    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    /// Expands (or shrinks) the rectangle by a margin on all sides.
    ///
    /// # Arguments
    ///
    /// * `margin` - Pixels to add on each side (negative shrinks)
    fn expand(&mut self, margin: f64) {
        self.x -= margin;
        self.y -= margin;
        self.width += 2.0 * margin;
        self.height += 2.0 * margin;
    }
}

fn main() {
    let mut r1 = Rect::new(0.0, 0.0, 10.0, 10.0);
    let r2 = Rect::new(5.0, 5.0, 10.0, 10.0);

    // --- 1. Method with primitive params ---
    println!("r1 contains (3, 3)? {}", r1.contains_point(3.0, 3.0));
    println!("r1 contains (15, 15)? {}", r1.contains_point(15.0, 15.0));

    // --- 2. Method with struct reference param ---
    println!("r1 overlaps r2? {}", r1.overlaps(&r2));

    // --- 3. Mutable method with params ---
    r1.translate(5.0, 5.0);
    println!("After translate: {r1:?}");

    r1.expand(2.0);
    println!("After expand: {r1:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_point_inside() {
        let r = Rect::new(0.0, 0.0, 10.0, 10.0);
        assert!(r.contains_point(5.0, 5.0));
    }

    #[test]
    fn test_contains_point_edge() {
        let r = Rect::new(0.0, 0.0, 10.0, 10.0);
        assert!(r.contains_point(0.0, 0.0));
        assert!(r.contains_point(10.0, 10.0));
    }

    #[test]
    fn test_contains_point_outside() {
        let r = Rect::new(0.0, 0.0, 10.0, 10.0);
        assert!(!r.contains_point(11.0, 5.0));
    }

    #[test]
    fn test_overlaps() {
        let r1 = Rect::new(0.0, 0.0, 10.0, 10.0);
        let r2 = Rect::new(5.0, 5.0, 10.0, 10.0);
        assert!(r1.overlaps(&r2));
    }

    #[test]
    fn test_no_overlap() {
        let r1 = Rect::new(0.0, 0.0, 5.0, 5.0);
        let r2 = Rect::new(10.0, 10.0, 5.0, 5.0);
        assert!(!r1.overlaps(&r2));
    }

    #[test]
    fn test_translate() {
        let mut r = Rect::new(0.0, 0.0, 10.0, 10.0);
        r.translate(5.0, 3.0);
        assert!((r.x - 5.0).abs() < f64::EPSILON);
        assert!((r.y - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_expand() {
        let mut r = Rect::new(5.0, 5.0, 10.0, 10.0);
        r.expand(2.0);
        assert!((r.x - 3.0).abs() < f64::EPSILON);
        assert!((r.y - 3.0).abs() < f64::EPSILON);
        assert!((r.width - 14.0).abs() < f64::EPSILON);
        assert!((r.height - 14.0).abs() < f64::EPSILON);
    }
}
