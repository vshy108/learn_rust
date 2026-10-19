//! # Rustfinity: Methods On Enums
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/methods-on-enums
//!
//! Enums in Rust can have associated functions and methods, just like structs. These methods make it easier to encapsulate behavior directly within the enum.
//!
//! In this challenge, you'll model the statuses of different vehicles and implement methods to describe their behavior.
//!
//! ## Your Task
//!
//! Create an enum `VehicleStatus` with the following variants:
//!
//! 1. `Parked` — a unit variant representing a parked vehicle.
//! 2. `Driving { speed: u32 }` — a named field variant representing a vehicle driving at a certain speed.
//! 3. `BrokenDown(String)` — a tuple variant with a `String` describing the reason for the breakdown.
//!
//! Implement the following methods for `VehicleStatus`:
//!
//! - `is_operational(&self) -> bool`:
//!   - Returns `true` if the vehicle is either `Parked` or `Driving`.
//!   - Returns `false` if the vehicle is `BrokenDown`.
//!
//! - `description(&self) -> String`:
//!   - Returns `"The vehicle is parked."` for `Parked`.
//!   - Returns `"The vehicle is driving at {speed} km/h."` for `Driving { speed }`.
//!   - Returns `"The vehicle is broken down: {reason}."` for `BrokenDown(reason)`.
//!
//! ## Hints
//!
//! <details>
//! <summary>Click here to reveal hints</summary>
//!
//! - Use a `match` expression inside the methods to handle each variant.
//! - Remember to use `&self` for the methods since they should not consume the enum.
//! - Use `format!` to construct strings with dynamic values, such as `speed` and `reason`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_methods_on_enums`
//! Test: `cargo test --example rf_methods_on_enums`
//! Filter: `cargo test --example rf_methods_on_enums <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub enum VehicleStatus {
    Parked,
    Driving { speed: u32 },
    BrokenDown(String),
}

impl VehicleStatus {
    pub fn is_operational(&self) -> bool {
        todo!("check vehicle status")
    }

    pub fn description(&self) -> String {
        todo!("describe vehicle status")
    }
}

// Example use case
pub fn main() {
    let parked = VehicleStatus::Parked;
    assert!(parked.is_operational());
    assert_eq!(parked.description(), "The vehicle is parked.");

    let driving = VehicleStatus::Driving { speed: 80 };
    assert!(driving.is_operational());
    assert_eq!(driving.description(), "The vehicle is driving at 80 km/h.");

    let broken_down = VehicleStatus::BrokenDown("Flat tire".to_string());
    assert!(!broken_down.is_operational());
    assert_eq!(
        broken_down.description(),
        "The vehicle is broken down: Flat tire."
    );
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_operational() {
        let parked = VehicleStatus::Parked;
        assert!(parked.is_operational());

        let driving = VehicleStatus::Driving { speed: 80 };
        assert!(driving.is_operational());

        let broken_down = VehicleStatus::BrokenDown("Flat tire".to_string());
        assert!(!broken_down.is_operational());
    }

    #[test]
    fn test_description_parked() {
        let parked = VehicleStatus::Parked;
        assert_eq!(parked.description(), "The vehicle is parked.");
    }

    #[test]
    fn test_description_driving() {
        let driving = VehicleStatus::Driving { speed: 120 };
        assert_eq!(driving.description(), "The vehicle is driving at 120 km/h.");
    }

    #[test]
    fn test_description_broken_down() {
        let broken_down = VehicleStatus::BrokenDown("Engine failure".to_string());
        assert_eq!(
            broken_down.description(),
            "The vehicle is broken down: Engine failure."
        );
    }

    #[test]
    fn test_mixed_scenarios() {
        let parked = VehicleStatus::Parked;
        assert!(parked.is_operational());
        assert_eq!(parked.description(), "The vehicle is parked.");

        let driving = VehicleStatus::Driving { speed: 50 };
        assert!(driving.is_operational());
        assert_eq!(driving.description(), "The vehicle is driving at 50 km/h.");

        let broken_down = VehicleStatus::BrokenDown("No fuel".to_string());
        assert!(!broken_down.is_operational());
        assert_eq!(
            broken_down.description(),
            "The vehicle is broken down: No fuel."
        );
    }
}
