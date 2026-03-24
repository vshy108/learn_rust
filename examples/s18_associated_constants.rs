// Demonstrates associated constants in traits.
//
// Traits can define constants that implementors must provide.
// Useful for type-level configuration.
//
// Run: cargo run --example s18_associated_constants

/// A trait with associated constants.
trait Limits {
    /// Minimum value for this type.
    const MIN: Self;
    /// Maximum value for this type.
    const MAX: Self;

    /// Returns whether `value` is within bounds.
    fn in_bounds(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Temperature(f64);

impl Limits for Temperature {
    const MIN: Self = Temperature(-273.15); // absolute zero
    const MAX: Self = Temperature(1_000_000.0); // surface of the sun-ish

    fn in_bounds(&self) -> bool {
        *self >= Self::MIN && *self <= Self::MAX
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Percentage(f64);

impl Limits for Percentage {
    const MIN: Self = Percentage(0.0);
    const MAX: Self = Percentage(100.0);

    fn in_bounds(&self) -> bool {
        *self >= Self::MIN && *self <= Self::MAX
    }
}

/// A trait with a default constant.
trait Labeled {
    const LABEL: &'static str = "Unknown";
}

struct Widget;
impl Labeled for Widget {
    const LABEL: &'static str = "Widget";
}

struct Gadget;
impl Labeled for Gadget {} // uses default

fn main() {
    let t = Temperature(25.0);
    println!("Temperature {t:?} in bounds: {}", t.in_bounds());
    println!(
        "Temperature range: {:?} to {:?}",
        Temperature::MIN,
        Temperature::MAX
    );

    let p = Percentage(150.0);
    println!("Percentage {p:?} in bounds: {}", p.in_bounds());

    println!();
    println!("Widget label: {}", Widget::LABEL);
    println!("Gadget label: {}", Gadget::LABEL);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_bounds() {
        assert!(Temperature(20.0).in_bounds());
        assert!(!Temperature(-300.0).in_bounds());
    }

    #[test]
    fn test_percentage_bounds() {
        assert!(Percentage(50.0).in_bounds());
        assert!(!Percentage(-1.0).in_bounds());
    }

    #[test]
    fn test_default_label() {
        assert_eq!(Gadget::LABEL, "Unknown");
    }
}
