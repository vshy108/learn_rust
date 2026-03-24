// Demonstrates generics in struct definitions.
//
// Structs can be parameterized by one or more type variables,
// allowing them to hold any type without code duplication.
//
// Run: cargo run --example s11_generics_structs

/// A 2D point with generic coordinate type.
///
/// Works with f64, f32, i32, or any other numeric type.
#[derive(Debug, Clone, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

/// A wrapper that optionally holds a value (like a simple Option).
#[derive(Debug)]
struct Container<T> {
    value: Option<T>,
    label: String,
}

impl<T: std::fmt::Debug> Container<T> {
    /// Creates a Container with a value.
    fn with_value(label: &str, value: T) -> Self {
        Container {
            value: Some(value),
            label: label.to_string(),
        }
    }

    /// Creates an empty Container.
    fn empty(label: &str) -> Self {
        Container {
            value: None,
            label: label.to_string(),
        }
    }

    /// Returns a reference to the contained value.
    fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Prints the container's state.
    fn describe(&self) {
        match &self.value {
            Some(v) => println!("{}: {:?}", self.label, v),
            None => println!("{}: <empty>", self.label),
        }
    }
}

fn main() {
    // --- 1. Point with different types ---
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 2.7 };
    println!("int:   {int_point:?}");
    println!("float: {float_point:?}");

    // --- 2. Type must be consistent within the struct ---
    // let mixed = Point { x: 1, y: 2.0 }; // error: i32 vs f64
    // For mixed types, use Point<T, U> with two type params.

    // --- 3. Container with String ---
    let c1 = Container::with_value("name", String::from("Rust"));
    c1.describe();
    println!("get: {:?}", c1.get());

    // --- 4. Container with Vec ---
    let c2 = Container::with_value("numbers", vec![1, 2, 3]);
    c2.describe();

    // --- 5. Empty container ---
    let c3: Container<i32> = Container::empty("unused");
    c3.describe();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_integer() {
        let p = Point { x: 1, y: 2 };
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
    }

    #[test]
    fn test_point_float() {
        let p = Point { x: 1.5, y: 2.5 };
        assert!((p.x - 1.5_f64).abs() < f64::EPSILON);
    }

    #[test]
    fn test_point_clone() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_container_with_value() {
        let c = Container::with_value("test", 42);
        assert_eq!(c.get(), Some(&42));
    }

    #[test]
    fn test_container_empty() {
        let c: Container<String> = Container::empty("e");
        assert_eq!(c.get(), None);
    }
}
