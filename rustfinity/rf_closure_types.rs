//! # Rustfinity: Closure Types
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/closure-types
//!
//! Closures in Rust are categorized based on how they interact with the environment and the variables they capture. The three main types are `Fn`, `FnMut`, and `FnOnce`. Understanding and using these types effectively is key to mastering closures in Rust.
//!
//! ## Closure Type Definitions
//!
//! - **Fn**: A closure that borrows its environment immutably. It is useful for read-only operations.
//! - **FnMut**: A closure that mutably borrows its environment, allowing it to modify variables.
//! - **FnOnce**: A closure that takes ownership of its captured variables, consuming them in the process.
//!
//! ## Your Task
//!
//! Implement the following closures and their respective behaviors:
//!
//! - `calculate_total`: An `Fn` closure that calculates the total price of an item, including tax (`price + price * tax_rate`).
//! - `apply_discount`: An `FnMut` closure that mutates the cart total by subtracting a given discount.
//! - `checkout_cart`: An `FnOnce` closure that consumes the cart's details (a `String`) and returns a confirmation message.
//!
//! ## Hints
//!
//! <details>
//! <summary>Click here to reveal hints</summary>
//!
//! - Use `impl Fn`, `impl FnMut`, or `impl FnOnce` to define the types of the closures. e.g.
//!   ```rust
//!     pub fn return_closure() -> impl Fn(f64, f64) -> f64 {
//!         //
//!     }
//!   ```
//!
//! </details>
//!
//! Run: `cargo run --example rf_closure_types`
//! Test: `cargo test --example rf_closure_types`
//! Filter: `cargo test --example rf_closure_types <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

// 1. Based on the `main` function below,
// Find out the types of the closures and define them
pub fn create_typed_closures() -> (
    impl Fn(f64, f64) -> f64,
    impl FnMut(&mut f64, f64),
    impl Fn(String) -> String,
) {
    // 2. Implement calculate_total closure here
    let calculate_total = |_: f64, _: f64| -> f64 { todo!("calculate a total") };

    // 3. Implement apply_discount closure here
    let apply_discount = |_: &mut f64, _: f64| todo!("apply a discount");

    // 4. Implement checkout_cart closure here
    let checkout_cart = |_: String| -> String { todo!("check out a cart") };

    (calculate_total, apply_discount, checkout_cart)
}

// Example usage
pub fn main() {
    let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

    // Example tests
    assert_eq!(calculate_total(100.0, 0.2), 120.0);

    let mut total_price = 120.0;
    apply_discount(&mut total_price, 20.0);
    assert_eq!(total_price, 100.0);

    let cart_details = String::from("Items: Apple, Banana, Orange");
    assert_eq!(
        checkout_cart(cart_details),
        "Checkout complete: Items: Apple, Banana, Orange"
    );
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_calculate_total() {
        let (calculate_total, _, _) = create_typed_closures();
        assert_eq!(calculate_total(100.0, 0.2), 120.0);
        assert_eq!(calculate_total(50.0, 0.1), 55.0);
        assert_eq!(calculate_total(0.0, 0.3), 0.0);
        assert_eq!(calculate_total(200.0, 0.15), 230.0);
    }

    #[test]
    fn test_apply_discount() {
        let (_, mut apply_discount, _) = create_typed_closures();
        let mut total_price = 120.0;

        // Apply a discount and check the result
        apply_discount(&mut total_price, 20.0);
        assert_eq!(total_price, 100.0);

        // Apply a larger discount
        apply_discount(&mut total_price, 30.0);
        assert_eq!(total_price, 70.0);

        // Apply a zero discount
        apply_discount(&mut total_price, 0.0);
        assert_eq!(total_price, 70.0);

        // Ensure that applying a discount larger than the total works correctly
        apply_discount(&mut total_price, 100.0);
        assert_eq!(total_price, -30.0);
    }

    #[test]
    fn test_checkout_cart() {
        let (_, _, checkout_cart) = create_typed_closures();

        // Standard cart checkout
        let cart_details = String::from("Items: Apple, Banana, Orange");

        let checkout_result = checkout_cart(cart_details);
        for item in ["Apple", "Banana", "Orange"].iter() {
            assert!(
                checkout_result.contains(item),
                "Receipt should contain item: {}",
                item
            );
        }
    }

    #[test]
    fn test_combined_behavior() {
        let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

        // Simulate a real-world use case
        let item_price = 150.0;
        let tax_rate = 0.2;
        let mut total_price = calculate_total(item_price, tax_rate);

        assert_eq!(total_price, 180.0); // Price after tax

        apply_discount(&mut total_price, 30.0); // Apply a discount
        assert_eq!(total_price, 150.0);

        let cart_details = String::from("Items: Laptop, Mouse");
        let receipt = checkout_cart(cart_details);

        for item in ["Laptop", "Mouse"].iter() {
            assert!(
                receipt.contains(item),
                "Receipt should contain item: {}",
                item
            );
        }
    }

}
