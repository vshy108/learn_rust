//! # Rustfinity: Closures
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/closures
//!
//! Closures in Rust are anonymous functions that can **capture variables from their environment.** They are similar to functions but have some unique properties that make them powerful and flexible. Closures are often used for short, simple operations and can be defined in a very concise way.
//!
//! ## Your Task
//!
//! Your task is to complete the implementation of the following closures:
//!
//! - `add_closure`: This closure should return the **sum of two integers**.
//! - `subtract_closure`: This closure should return the **difference between two integers**.
//! - `multiply_closure`: This closure should return the **product of two integers**.
//!
//! ### Requirements
//!
//! - Each closure should take two parameters of type `i32`.
//! - Each closure should return a result of type `i32`.
//!
//! ## Hints
//!
//! Here are some hints to help you solve the exercise:
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Use the `|a, b|` syntax to define the parameters of the closure.
//! - Since the type is already specified, it can be inferred, so you don't need to specify `a` and `b`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_closures`
//! Test: `cargo test --example rf_closures`
//! Filter: `cargo test --example rf_closures <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn create_closures() -> (
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
) {
    let add_closure = |a, b| unimplemented!("add the two closure arguments");

    let subtract_closure = |_, _| unimplemented!("subtract the closure arguments");

    let multiply_closure = |_, _| unimplemented!("multiply the closure arguments");

    (add_closure, subtract_closure, multiply_closure)
}

// Example usage
pub fn main() {
    let (add, subtract, multiply) = create_closures();

    // Example tests
    assert_eq!(add(3, 4), 7); // Expected: 7
    assert_eq!(subtract(10, 4), 6); // Expected: 6
    assert_eq!(multiply(3, 5), 15); // Expected: 15
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_add_closure() {
        let (add_closure, _, _) = create_closures();
        assert_eq!(add_closure(3, 4), 7);
        assert_eq!(add_closure(-1, 1), 0);
        assert_eq!(add_closure(0, 0), 0);
    }

    #[test]
    fn test_subtract_closure() {
        let (_, subtract_closure, _) = create_closures();
        assert_eq!(subtract_closure(10, 4), 6);
        assert_eq!(subtract_closure(4, 10), -6);
        assert_eq!(subtract_closure(0, 0), 0);
    }

    #[test]
    fn test_multiply_closure() {
        let (_, _, multiply_closure) = create_closures();
        assert_eq!(multiply_closure(3, 5), 15);
        assert_eq!(multiply_closure(-2, 3), -6);
        assert_eq!(multiply_closure(0, 10), 0);
    }
}
