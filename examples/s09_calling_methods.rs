// Demonstrates calling methods from within other methods.
//
// Methods can call other methods on `self`.
//
// Run: cargo run --example s09_calling_methods

use std::fmt;

#[derive(Debug, Clone)]
struct Temperature {
    celsius: f64,
}

impl Temperature {
    /// Creates a Temperature from Celsius.
    fn from_celsius(c: f64) -> Self {
        Temperature { celsius: c }
    }

    /// Creates a Temperature from Fahrenheit.
    fn from_fahrenheit(f: f64) -> Self {
        Temperature {
            celsius: (f - 32.0) * 5.0 / 9.0,
        }
    }

    /// Returns the temperature in Fahrenheit.
    fn to_fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    /// Returns the temperature in Kelvin.
    fn to_kelvin(&self) -> f64 {
        self.celsius + 273.15
    }

    /// Checks if water would be frozen at this temperature.
    /// Calls `self.celsius` directly — a field, not a method.
    fn is_freezing(&self) -> bool {
        self.celsius <= 0.0
    }

    /// Checks if water would be boiling at standard pressure.
    fn is_boiling(&self) -> bool {
        self.celsius >= 100.0
    }

    /// Returns a human-readable description.
    /// Calls other methods: is_freezing(), is_boiling(), to_fahrenheit().
    fn describe(&self) -> String {
        let state = if self.is_boiling() {
            "boiling"
        } else if self.is_freezing() {
            "freezing"
        } else {
            "liquid"
        };
        format!(
            "{:.1} C / {:.1} F / {:.1} K (water: {state})",
            self.celsius,
            self.to_fahrenheit(),
            self.to_kelvin()
        )
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display delegates to describe(), which calls more methods.
        write!(f, "{}", self.describe())
    }
}

fn main() {
    let temps = [
        Temperature::from_celsius(-10.0),
        Temperature::from_celsius(0.0),
        Temperature::from_celsius(20.0),
        Temperature::from_celsius(100.0),
        Temperature::from_fahrenheit(72.0),
    ];

    for t in &temps {
        println!("{t}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_fahrenheit() {
        let t = Temperature::from_celsius(0.0);
        assert!((t.to_fahrenheit() - 32.0).abs() < 0.01);
    }

    #[test]
    fn test_to_kelvin() {
        let t = Temperature::from_celsius(0.0);
        assert!((t.to_kelvin() - 273.15).abs() < 0.01);
    }

    #[test]
    fn test_from_fahrenheit() {
        let t = Temperature::from_fahrenheit(212.0);
        assert!((t.celsius - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_is_freezing() {
        assert!(Temperature::from_celsius(-5.0).is_freezing());
        assert!(Temperature::from_celsius(0.0).is_freezing());
        assert!(!Temperature::from_celsius(1.0).is_freezing());
    }

    #[test]
    fn test_is_boiling() {
        assert!(Temperature::from_celsius(100.0).is_boiling());
        assert!(!Temperature::from_celsius(99.0).is_boiling());
    }

    #[test]
    fn test_describe_calls_methods() {
        let t = Temperature::from_celsius(20.0);
        let d = t.describe();
        assert!(d.contains("20.0 C"));
        assert!(d.contains("liquid"));
    }
}
