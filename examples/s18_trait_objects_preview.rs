// A preview of trait objects — dynamic dispatch with `dyn Trait`.
//
// Generics use static dispatch (resolved at compile time).
// Trait objects use dynamic dispatch (resolved at runtime via vtable).
//
// Run: cargo run --example s18_trait_objects_preview

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64,
}
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn name(&self) -> &str {
        "Circle"
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn name(&self) -> &str {
        "Rectangle"
    }
}

/// Static dispatch: ONE type per call (monomorphized).
fn print_area_static(shape: &impl Shape) {
    println!("{}: area = {:.2}", shape.name(), shape.area());
}

/// Dynamic dispatch: ANY Shape at runtime.
fn print_area_dynamic(shape: &dyn Shape) {
    println!("{}: area = {:.2}", shape.name(), shape.area());
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    // Static dispatch — works but can't mix types in a collection:
    print_area_static(&c);
    print_area_static(&r);

    // Dynamic dispatch — can mix types:
    let shapes: Vec<&dyn Shape> = vec![&c, &r];
    for shape in &shapes {
        print_area_dynamic(*shape);
    }

    // Or with owned trait objects:
    let owned: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle {
            width: 2.0,
            height: 3.0,
        }),
    ];
    for shape in &owned {
        println!("  {} area: {:.2}", shape.name(), shape.area());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_dispatch() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
        ];
        assert!((shapes[0].area() - std::f64::consts::PI).abs() < 0.01);
        assert!((shapes[1].area() - 6.0).abs() < f64::EPSILON);
    }
}
