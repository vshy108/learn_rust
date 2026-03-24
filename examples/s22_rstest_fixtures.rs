// Demonstrates fixtures with the rstest crate.
//
// `rstest` provides:
// - Fixtures: reusable test setup via function injection
// - Parametrized tests: run a test with multiple inputs
//
// Add to Cargo.toml: `rstest = "0.23"` under [dev-dependencies]
//
// Run: cargo test --example s22_rstest_fixtures

/// A shopping cart.
#[derive(Debug)]
struct Cart {
    items: Vec<(String, f64)>,
}

impl Cart {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add an item with a price.
    fn add(&mut self, name: &str, price: f64) {
        self.items.push((name.to_string(), price));
    }

    /// Calculate total price.
    fn total(&self) -> f64 {
        self.items.iter().map(|(_, p)| p).sum()
    }

    /// Apply a discount (percentage, 0-100).
    fn apply_discount(&self, percent: f64) -> f64 {
        self.total() * (1.0 - percent / 100.0)
    }

    /// Number of items.
    fn item_count(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut cart = Cart::new();
    cart.add("Book", 15.99);
    cart.add("Pen", 2.50);
    println!("Cart: {cart:?}");
    println!("Total: ${:.2}", cart.total());
    println!("With 10% off: ${:.2}", cart.apply_discount(10.0));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // A fixture: creates a pre-populated cart
    #[rstest]
    fn test_empty_cart() {
        let cart = Cart::new();
        assert_eq!(cart.item_count(), 0);
        assert_eq!(cart.total(), 0.0);
    }

    // Parametrized test: test multiple discount values
    #[rstest]
    #[case(0.0, 100.0)] // 0% discount on $100
    #[case(10.0, 90.0)] // 10% discount
    #[case(50.0, 50.0)] // 50% discount
    #[case(100.0, 0.0)] // 100% discount
    fn test_discount(#[case] percent: f64, #[case] expected: f64) {
        let mut cart = Cart::new();
        cart.add("Item", 100.0);
        let result = cart.apply_discount(percent);
        assert!(
            (result - expected).abs() < f64::EPSILON,
            "discount {percent}%: expected {expected}, got {result}"
        );
    }

    // Parametrized test with multiple items
    #[rstest]
    #[case(&[10.0, 20.0], 30.0)]
    #[case(&[5.0, 5.0, 5.0], 15.0)]
    #[case(&[], 0.0)]
    fn test_total(#[case] prices: &[f64], #[case] expected: f64) {
        let mut cart = Cart::new();
        for (i, &price) in prices.iter().enumerate() {
            cart.add(&format!("item{i}"), price);
        }
        assert_eq!(cart.total(), expected);
    }
}
