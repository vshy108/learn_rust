// Overview of smart pointers in Rust.
//
// Smart pointers own data and provide additional capabilities beyond references.
// They implement `Deref` (act like references) and `Drop` (cleanup on scope exit).
//
// Common smart pointers:
// - `Box<T>`: heap allocation, single ownership
// - `Rc<T>`: reference-counted shared ownership (single-thread)
// - `Arc<T>`: atomic reference-counted (multi-thread)
// - `Cell<T>` / `RefCell<T>`: interior mutability
//
// Run: cargo run --example s26_smart_pointers

fn main() {
    println!("=== Smart Pointers Overview ===");

    // --- 1. Box: heap allocation ---
    let boxed = Box::new(42);
    println!("Box value: {boxed}");
    // boxed is automatically freed when it goes out of scope (Drop)

    // --- 2. String is a smart pointer ---
    let s = String::from("hello");
    // String owns heap data, implements Deref<Target=str>
    print_str(&s); // Deref coercion: &String -> &str

    // --- 3. Vec is a smart pointer ---
    let v = vec![1, 2, 3];
    print_slice(&v); // Deref coercion: &Vec<i32> -> &[i32]

    // --- 4. Box for trait objects ---
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rect { w: 4.0, h: 5.0 }),
    ];
    for s in &shapes {
        println!("  area: {:.1}", s.area());
    }
}

/// Print a string slice (demonstrates deref coercion from String).
fn print_str(s: &str) {
    println!("str: {s}");
}

/// Print a slice (demonstrates deref coercion from Vec).
fn print_slice(s: &[i32]) {
    println!("slice: {s:?}");
}

trait Shape {
    /// Calculate the area of this shape.
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rect {
    w: f64,
    h: f64,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_deref() {
        let b = Box::new(42);
        assert_eq!(*b, 42); // Deref
    }

    #[test]
    fn test_trait_object_area() {
        let c: Box<dyn Shape> = Box::new(Circle { radius: 1.0 });
        let expected = std::f64::consts::PI;
        assert!((c.area() - expected).abs() < 1e-10);
    }
}
