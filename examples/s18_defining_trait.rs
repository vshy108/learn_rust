// Demonstrates defining a trait.
//
// A trait declaration lists method signatures. Types that
// implement the trait must provide bodies for these methods.
//
// Run: cargo run --example s18_defining_trait

/// A trait for things that can describe themselves.
trait Describable {
    /// Returns a description string.
    fn describe(&self) -> String;

    /// Returns the category. (No body = required method.)
    fn category(&self) -> &str;
}

/// A trait for things with an area (not used here, just demonstrating syntax).
#[allow(dead_code)]
trait HasArea {
    /// Calculates the area.
    fn area(&self) -> f64;
}

// Traits with no methods are called "marker traits":
// trait Sendable {}

fn main() {
    println!("=== Defining a Trait ===");
    println!();
    println!("trait Describable {{");
    println!("    fn describe(&self) -> String;");
    println!("    fn category(&self) -> &str;");
    println!("}}");
    println!();
    println!("Key points:");
    println!("  - Methods take &self, &mut self, or self");
    println!("  - No method body = required (must implement)");
    println!("  - With body = default implementation (can override)");
    println!("  - Traits can require other traits (supertraits)");
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Dog {
        name: String,
    }

    impl Describable for Dog {
        fn describe(&self) -> String {
            format!("Dog named {}", self.name)
        }
        fn category(&self) -> &str {
            "animal"
        }
    }

    #[test]
    fn test_trait_impl() {
        let d = Dog { name: "Rex".into() };
        assert!(d.describe().contains("Rex"));
        assert_eq!(d.category(), "animal");
    }
}
