// Demonstrates the `self` keyword in module paths.
//
// `self` refers to the current module. It's used in:
// - `use self::item` — explicitly refer to something in the current module
// - `use module::{self, item}` — import the module AND specific items
//
// Run: cargo run --example s14_self_keyword

mod shapes {
    pub fn area_circle(r: f64) -> f64 {
        std::f64::consts::PI * r * r
    }

    pub fn area_square(side: f64) -> f64 {
        side * side
    }

    pub mod three_d {
        pub fn volume_cube(side: f64) -> f64 {
            side * side * side
        }

        pub fn volume_sphere(r: f64) -> f64 {
            (4.0 / 3.0) * std::f64::consts::PI * r * r * r
        }
    }
}

fn main() {
    // --- 1. use module::{self, items} ---
    // Import the module itself AND specific items from it.
    use shapes::{self, area_circle};

    // Now we can use both:
    println!("circle area: {:.2}", area_circle(5.0)); // direct name
    println!("square area: {:.2}", shapes::area_square(4.0)); // via module

    // --- 2. use self:: in a module context ---
    // `self::` refers to the current module (useful for disambiguation).
    mod my_module {
        pub fn helper() -> &'static str {
            "helper"
        }
        pub fn run() -> String {
            // `self::helper()` explicitly means "helper in THIS module"
            format!("running {}", self::helper())
        }
    }
    println!("{}", my_module::run());

    // --- 3. use three_d::{self, ...} ---
    use shapes::three_d::{self, volume_cube};
    println!("cube volume: {:.2}", volume_cube(3.0));
    println!("sphere volume: {:.2}", three_d::volume_sphere(3.0));
}

#[cfg(test)]
mod tests {
    use super::shapes;

    #[test]
    fn test_area_circle() {
        let area = shapes::area_circle(1.0);
        assert!((area - std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_volume_cube() {
        assert!((shapes::three_d::volume_cube(2.0) - 8.0).abs() < f64::EPSILON);
    }
}
