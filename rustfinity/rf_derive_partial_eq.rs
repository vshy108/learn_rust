//! # Rustfinity: Derive Partial Eq
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/derive-partial-eq
//!
//! Enums can be useful to compare different states of a type, for example `if status == OrderStatus::Pending`. However, you can not directly compare two enums for equality.
//!
//! For this to work, you need to implement the `PartialEq` trait for the enum.
//!
//! This can be done manually, but Rust provides a convenient way to implement the `PartialEq` trait for types using the `derive` attribute. This allows you to compare instances of a type for equality without manually implementing the trait.
//!
//! In this challenge, you'll define a simple enum and use `#[derive(PartialEq)]` to automatically implement equality checks.
//!
//! ## Your Task
//!
//! 1. Define an enum `OrderStatus` with the following variants:
//!    - `Pending` — a unit variant representing an order that is not yet processed.
//!    - `Shipped` — a unit variant representing an order that has been shipped.
//!    - `Cancelled(String)` — a tuple variant with a reason for cancellation.
//!
//! 2. Use the `#[derive(PartialEq)]` attribute to derive the `PartialEq` trait for the enum.
//!
//! ### Requirements
//!
//! 1. Use the `#[derive(PartialEq)]` macro for the `OrderStatus` enum.
//!
//! ## Hints
//!
//! <details>
//!    <summary>Click here to reveal hints</summary>
//!
//! - Use the `derive` macro on the enum to automatically implement `PartialEq`.
//! - String types in Rust already implement `PartialEq`, so `Cancelled(String)` can be compared automatically.
//!
//! </details>
//!
//! Run: `cargo run --example rf_derive_partial_eq`
//! Test: `cargo test --example rf_derive_partial_eq`
//! Filter: `cargo test --example rf_derive_partial_eq <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

#[derive(Debug, PartialEq)]
pub enum OrderStatus {
    Pending,
    Shipped,
    Cancelled(String),
}

// Example use case
pub fn main() {
    let status1 = OrderStatus::Pending;
    let status2 = OrderStatus::Pending;
    assert_eq!(status1, status2);

    let cancelled1 = OrderStatus::Cancelled("Out of stock".to_string());
    let cancelled2 = OrderStatus::Cancelled("Out of stock".to_string());
    assert_eq!(cancelled1, cancelled2);

    let cancelled3 = OrderStatus::Cancelled("Customer request".to_string());
    assert_ne!(cancelled1, cancelled3);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_pending_status_equality() {
        let status1 = OrderStatus::Pending;
        let status2 = OrderStatus::Pending;

        if status1 != status2 {
            panic!("Pending statuses are not equal");
        }
    }

    #[test]
    fn test_shipped_status_equality() {
        let status1 = OrderStatus::Shipped;
        let status2 = OrderStatus::Shipped;

        if status1 != status2 {
            panic!("Shipped statuses are not equal");
        }
    }

    #[test]
    fn test_cancelled_status_equality() {
        let cancelled1 = OrderStatus::Cancelled("Out of stock".to_string());
        let cancelled2 = OrderStatus::Cancelled("Out of stock".to_string());

        if cancelled1 != cancelled2 {
            panic!("Cancelled statuses are not equal");
        }
    }

    #[test]
    fn test_cancelled_status_inequality() {
        let cancelled1 = OrderStatus::Cancelled("Out of stock".to_string());
        let cancelled2 = OrderStatus::Cancelled("Customer request".to_string());

        if cancelled1 == cancelled2 {
            panic!("Cancelled statuses are equal");
        }
    }

    #[test]
    fn test_mixed_status_inequality() {
        let pending = OrderStatus::Pending;
        let shipped = OrderStatus::Shipped;
        let cancelled = OrderStatus::Cancelled("Out of stock".to_string());

        if pending == shipped || pending == cancelled || shipped == cancelled {
            panic!("Mixed statuses are equal");
        }
    }
}
