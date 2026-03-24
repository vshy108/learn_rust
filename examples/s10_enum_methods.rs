// Demonstrates defining methods on enums.
//
// Enums can have `impl` blocks with methods, just like structs.
// Methods use `match` on `self` to handle each variant.
//
// Run: cargo run --example s10_enum_methods

use std::fmt;

#[derive(Debug, Clone)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    /// Returns the next light in the cycle.
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }

    /// Returns how many seconds this light lasts.
    fn duration_secs(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }

    /// Checks if vehicles should stop.
    fn should_stop(&self) -> bool {
        match self {
            TrafficLight::Red | TrafficLight::Yellow => true,
            TrafficLight::Green => false,
        }
    }
}

impl fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TrafficLight::Red => "RED",
            TrafficLight::Yellow => "YELLOW",
            TrafficLight::Green => "GREEN",
        };
        write!(f, "{name}")
    }
}

fn main() {
    let mut light = TrafficLight::Red;

    // --- 1. Using enum methods ---
    for _ in 0..6 {
        println!(
            "{light} ({}s) — stop: {}",
            light.duration_secs(),
            light.should_stop()
        );
        light = light.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let r = TrafficLight::Red;
        let g = r.next();
        let y = g.next();
        let r2 = y.next();
        assert!(matches!(g, TrafficLight::Green));
        assert!(matches!(y, TrafficLight::Yellow));
        assert!(matches!(r2, TrafficLight::Red));
    }

    #[test]
    fn test_should_stop() {
        assert!(TrafficLight::Red.should_stop());
        assert!(TrafficLight::Yellow.should_stop());
        assert!(!TrafficLight::Green.should_stop());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", TrafficLight::Red), "RED");
        assert_eq!(format!("{}", TrafficLight::Green), "GREEN");
    }
}
