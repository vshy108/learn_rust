// Demonstrates implementing PartialEq between DIFFERENT types.
//
// You can compare `TypeA == TypeB` by implementing
// `impl PartialEq<TypeB> for TypeA`.
//
// Run: cargo run --example s18_equality_diff_types

/// Meters and Centimeters — different types, comparable.
#[derive(Debug, Clone)]
struct Meters(f64);

#[derive(Debug, Clone)]
struct Centimeters(f64);

// Meters == Centimeters
impl PartialEq<Centimeters> for Meters {
    fn eq(&self, other: &Centimeters) -> bool {
        (self.0 * 100.0 - other.0).abs() < f64::EPSILON
    }
}

// Centimeters == Meters (reverse direction)
impl PartialEq<Meters> for Centimeters {
    fn eq(&self, other: &Meters) -> bool {
        (self.0 - other.0 * 100.0).abs() < f64::EPSILON
    }
}

// Self-comparison too:
impl PartialEq for Meters {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < f64::EPSILON
    }
}

impl PartialEq for Centimeters {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < f64::EPSILON
    }
}

/// String wrapper that compares with &str.
#[derive(Debug, Clone)]
struct Name(String);

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn main() {
    let m = Meters(1.5);
    let cm = Centimeters(150.0);

    println!("1.5m == 150cm: {}", m == cm); // true
    println!("150cm == 1.5m: {}", cm == m); // true

    let m2 = Meters(2.0);
    println!("1.5m == 2.0m: {}", m == m2); // false

    println!();
    let name = Name("Rust".into());
    println!("Name == &str: {}", name == "Rust"); // true
    println!("Name == &str: {}", name == "Go"); // false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_type_eq() {
        assert_eq!(Meters(1.0), Centimeters(100.0));
        assert_eq!(Centimeters(200.0), Meters(2.0));
    }

    #[test]
    fn test_cross_type_ne() {
        assert_ne!(Meters(1.0), Centimeters(99.0));
    }

    #[test]
    fn test_name_vs_str() {
        let n = Name("hello".into());
        assert!(n == "hello");
        assert!(n != "world");
    }
}
