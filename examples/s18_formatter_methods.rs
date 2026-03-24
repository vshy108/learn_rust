// Demonstrates Formatter methods for controlling output.
//
// The `Formatter` provides methods for width, padding, precision,
// alignment, and alternate formatting (`#`).
//
// Run: cargo run --example s18_formatter_methods

use std::fmt;

struct Money {
    cents: i64,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dollars = self.cents / 100;
        let cents = (self.cents % 100).unsigned_abs();

        // Respect width and fill from format spec:
        let formatted = format!("${dollars}.{cents:02}");

        // Use the pad method to handle width/fill/align:
        f.pad(&formatted)
    }
}

// Alternate display with #
struct HexValue(u32);

impl fmt::Display for HexValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // {:#} — verbose form
            write!(f, "0x{:08X} ({})", self.0, self.0)
        } else {
            // {} — short form
            write!(f, "0x{:X}", self.0)
        }
    }
}

// Precision support
struct Measurement {
    value: f64,
    unit: String,
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Respect precision if specified:
        if let Some(precision) = f.precision() {
            write!(f, "{:.prec$} {}", self.value, self.unit, prec = precision)
        } else {
            write!(f, "{} {}", self.value, self.unit)
        }
    }
}

fn main() {
    let price = Money { cents: 1999 };
    println!("Price: {price}");
    println!("Padded: {:>15}", price); // right-align
    println!("Padded: {:<15}", price); // left-align
    println!("Padded: {:^15}", price); // center
    println!("Filled: {:*>15}", price); // fill with *

    println!();
    let hex = HexValue(255);
    println!("Short:    {hex}");
    println!("Verbose:  {hex:#}");

    println!();
    let m = Measurement {
        value: 3.14159265,
        unit: "kg".into(),
    };
    println!("Default:  {m}");
    println!("2 digits: {m:.2}");
    println!("4 digits: {m:.4}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_display() {
        assert_eq!(Money { cents: 1999 }.to_string(), "$19.99");
        assert_eq!(Money { cents: 100 }.to_string(), "$1.00");
        assert_eq!(Money { cents: 5 }.to_string(), "$0.05");
    }

    #[test]
    fn test_hex_alternate() {
        let h = HexValue(255);
        assert_eq!(format!("{h}"), "0xFF");
        assert!(format!("{h:#}").contains("0x000000FF"));
    }

    #[test]
    fn test_measurement_precision() {
        let m = Measurement {
            value: 3.14159,
            unit: "m".into(),
        };
        assert_eq!(format!("{m:.2}"), "3.14 m");
    }
}
