//! # Rustfinity: Debug Display Derive
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/debug-display-derive
//!
//! In Rust, the `Debug` and `Display` traits control how types are formatted as strings. `Debug` is intended for programmer-facing output (useful for debugging and logging), while `Display` is for user-facing output. Understanding when and how to use each is essential for writing clear, professional Rust code.
//!
//! The `Debug` trait can almost always be derived automatically using `#[derive(Debug)]`, which generates a representation showing the type name and all field values. This is incredibly useful during development - you can print any `Debug` type using `{:?}` (compact) or `{:#?}` (pretty-printed with indentation).
//!
//! The `Display` trait, however, must be implemented manually because there's no single "correct" way to display a type to users. You implement `Display` by writing a `fmt` method that uses format macros to build the output string. Once you implement `Display`, you automatically get a `to_string()` method for free.
//!
//! ## The Debug and Display Traits
//!
//! ```rust
//! use std::fmt;
//!
//! // Debug - can be derived
//! #[derive(Debug)]
//! struct Point {
//!     x: i32,
//!     y: i32,
//! }
//!
//! // Display - must be implemented manually
//! impl fmt::Display for Point {
//!     fn fmt(
//!         &self,
//!         f: &mut fmt::Formatter<'_>
//!     ) -> fmt::Result {
//!         write!(f, "({}, {})", self.x, self.y)
//!     }
//! }
//!
//! let p = Point { x: 3, y: 4 };
//! println!("{:?}", p);   // Debug: Point { x: 3, y: 4 }
//! println!("{}", p);     // Display: (3, 4)
//! ```
//!
//! ## Format Specifiers
//!
//! - `{}` - Uses `Display` trait
//! - `{:?}` - Uses `Debug` trait (compact)
//! - `{:#?}` - Uses `Debug` trait (pretty-printed)
//!
//! ## Your Task
//!
//! Implement the following types demonstrating `Debug` and `Display`:
//!
//! ### 1. Coordinate Struct
//!
//! Create a `Coordinate` struct with `x` and `y` fields (both `f64`).
//!
//! - Derive `Debug`
//! - Implement `Display` to format as `"(x, y)"` (e.g., `"(3.5, -2.0)"`)
//!
//! ### 2. Color Enum
//!
//! Create a `Color` enum with variants:
//!
//! - `Red`, `Green`, `Blue` (unit variants)
//! - `Rgb(u8, u8, u8)` (RGB values)
//! - `Hex(String)` (hex string like "#FF5733")
//!
//! - Derive `Debug`
//! - Implement `Display`:
//!   - Unit variants display as their name: `"Red"`, `"Green"`, `"Blue"`
//!   - `Rgb` displays as `"rgb(r, g, b)"`
//!     (e.g., `"rgb(255, 128, 0)"`)
//!   - `Hex` displays as the hex string (e.g., `"#FF5733"`)
//!
//! ### 3. Temperature Struct
//!
//! Create a `Temperature` struct that holds a value (`f64`) and a unit (`TemperatureUnit` enum).
//!
//! The `TemperatureUnit` enum should have variants: `Celsius`, `Fahrenheit`, `Kelvin`.
//!
//! - Derive `Debug` for both
//! - Implement `Display` for `Temperature` to format
//!   as `"value°C"`, `"value°F"`, or `"valueK"`
//!   (e.g., `"25.5°C"`, `"98.6°F"`, `"300K"`)
//!
//! ### 4. LogLevel Enum
//!
//! Create a `LogLevel` enum with variants: `Error`, `Warning`, `Info`, `Debug`.
//!
//! - Derive `Debug`
//! - Implement `Display` to show as uppercase strings:
//!   `"ERROR"`, `"WARNING"`, `"INFO"`, `"DEBUG"`
//!
//! ### 5. LogMessage Struct
//!
//! Create a `LogMessage` struct with:
//!
//! - `level`: `LogLevel`
//! - `message`: `String`
//!
//! - Derive `Debug`
//! - Implement `Display` to format as
//!   `"[LEVEL] message"` (e.g.,
//!   `"[ERROR] Connection failed"`)
//!
//! ### 6. Utility Functions
//!
//! Implement these functions to work with Debug and Display:
//!
//! - `debug_string<T: std::fmt::Debug>(
//!   value: &T
//! ) -> String`
//!   - Returns the debug representation of any `Debug` type
//! - `display_string<T: std::fmt::Display>(
//!   value: &T
//! ) -> String`
//!   - Returns the display representation of any
//!     `Display` type
//! - `pretty_debug<T: std::fmt::Debug>(
//!   value: &T
//! ) -> String`
//!   - Returns the pretty-printed debug
//!     representation (with `{:#?}`)
//!
//! ## Examples
//!
//! ```rust
//! // Coordinate
//! let coord = Coordinate { x: 3.5, y: -2.0 };
//! assert_eq!(format!("{}", coord), "(3.5, -2.0)");
//! assert_eq!(
//!     format!("{:?}", coord),
//!     "Coordinate { x: 3.5, y: -2.0 }"
//! );
//!
//! // Color
//! let red = Color::Red;
//! let custom = Color::Rgb(255, 128, 0);
//! let hex = Color::Hex(String::from("#FF5733"));
//! assert_eq!(format!("{}", red), "Red");
//! assert_eq!(format!("{}", custom), "rgb(255, 128, 0)");
//! assert_eq!(format!("{}", hex), "#FF5733");
//!
//! // Temperature
//! let temp = Temperature {
//!     value: 25.5,
//!     unit: TemperatureUnit::Celsius
//! };
//! assert_eq!(format!("{}", temp), "25.5°C");
//!
//! // LogMessage
//! let log = LogMessage {
//!     level: LogLevel::Error,
//!     message: String::from("Connection failed"),
//! };
//! assert_eq!(
//!     format!("{}", log),
//!     "[ERROR] Connection failed"
//! );
//!
//! // Utility functions
//! assert_eq!(
//!     debug_string(&coord),
//!     "Coordinate { x: 3.5, y: -2.0 }"
//! );
//! assert_eq!(display_string(&coord), "(3.5, -2.0)");
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `#[derive(Debug)]` on structs and enums for
//!   automatic Debug implementation
//! - Implement `Display` using
//!   `impl std::fmt::Display for TypeName {
//!   fn fmt(...)
//! }`
//! - Use `write!(f, "format string", args)` inside
//!   the `fmt` method
//! - For enums, use `match self { ... }` to handle
//!   each variant
//! - The degree symbol `°` is a valid UTF-8
//!   character you can include directly in strings
//! - `format!("{:?}", value)` returns the Debug
//!   string
//! - `format!("{:#?}", value)` returns the pretty-
//!   printed Debug string
//! - `format!("{}", value)` returns the Display
//!   string
//! - You can also use `value.to_string()` for
//!   Display types
//!
//! </details>
//!
//! Run: `cargo run --example rf_debug_display_derive`
//! Test: `cargo test --example rf_debug_display_derive`
//! Filter: `cargo test --example rf_debug_display_derive <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::fmt;

/// A 2D coordinate with x and y values.
///
/// TODO: Derive Debug trait
/// TODO: Implement Display to format as "(x, y)"
#[derive(Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("format Coordinate for display")
    }
}
// The output should be "(x, y)" format, e.g., "(3.5, -2.0)"

/// A color representation supporting named colors, RGB, and hex values.
///
/// TODO: Derive Debug trait
/// TODO: Implement Display for each variant
#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Hex(String),
}

impl fmt::Display for Color {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("format Color for display")
    }
}
// - Red, Green, Blue -> "Red", "Green", "Blue"
// - Rgb(r, g, b) -> "rgb(r, g, b)"
// - Hex(s) -> the hex string itself

/// Temperature units.
///
/// TODO: Derive Debug trait
#[derive(Debug)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

/// A temperature value with its unit.
///
/// TODO: Derive Debug trait
/// TODO: Implement Display to format as "value°C", "value°F", or "valueK"
#[derive(Debug)]
pub struct Temperature {
    pub value: f64,
    pub unit: TemperatureUnit,
}

impl fmt::Display for Temperature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("format Temperature for display")
    }
}
// - Celsius: "25.5°C"
// - Fahrenheit: "98.6°F"
// - Kelvin: "300K" (no degree symbol for Kelvin)

/// Log severity levels.
///
/// TODO: Derive Debug trait
/// TODO: Implement Display to show as uppercase: "ERROR", "WARNING", "INFO", "DEBUG"
#[derive(Debug)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("format LogLevel for display")
    }
}
// Output should be uppercase: "ERROR", "WARNING", "INFO", "DEBUG"

/// A log message with a severity level and message text.
///
/// TODO: Derive Debug trait
/// TODO: Implement Display to format as "[LEVEL] message"
#[derive(Debug)]
pub struct LogMessage {
    pub level: LogLevel,
    pub message: String,
}

impl fmt::Display for LogMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("format LogMessage for display")
    }
}
// Output should be "[LEVEL] message", e.g., "[ERROR] Connection failed"

/// Returns the debug representation of any Debug type.
///
/// Use format!("{:?}", value) to get the debug string.
pub fn debug_string<T: fmt::Debug>(value: &T) -> String {
    // TODO
    todo!()
}

/// Returns the display representation of any Display type.
///
/// Use format!("{}", value) to get the display string.
pub fn display_string<T: fmt::Display>(value: &T) -> String {
    // TODO
    todo!()
}

/// Returns the pretty-printed debug representation of any Debug type.
///
/// Use format!("{:#?}", value) to get the pretty-printed debug string.
pub fn pretty_debug<T: fmt::Debug>(value: &T) -> String {
    // TODO
    todo!()
}

// Example usage
pub fn main() {
    // Coordinate examples
    let coord = Coordinate { x: 3.5, y: -2.0 };
    println!("Coordinate Display: {}", coord);
    println!("Coordinate Debug: {:?}", coord);

    // Color examples
    let red = Color::Red;
    let custom = Color::Rgb(255, 128, 0);
    let hex = Color::Hex(String::from("#FF5733"));
    println!("Red: {}", red);
    println!("Custom RGB: {}", custom);
    println!("Hex color: {}", hex);

    // Temperature examples
    let temp = Temperature {
        value: 25.5,
        unit: TemperatureUnit::Celsius,
    };
    println!("Temperature: {}", temp);

    // LogMessage examples
    let log = LogMessage {
        level: LogLevel::Error,
        message: String::from("Connection failed"),
    };
    println!("Log: {}", log);

    // Utility function examples
    println!("Debug string: {}", debug_string(&coord));
    println!("Display string: {}", display_string(&coord));
    println!("Pretty debug: {}", pretty_debug(&coord));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ============================================================================
    // Coordinate Tests
    // ============================================================================

    mod coordinate_tests {
        use super::*;

        #[test]
        fn display_positive_values() {
            let coord = Coordinate { x: 3.5, y: 2.5 };
            assert_eq!(format!("{}", coord), "(3.5, 2.5)");
        }

        #[test]
        fn display_negative_values() {
            let coord = Coordinate { x: -3.5, y: -2.5 };
            assert_eq!(format!("{}", coord), "(-3.5, -2.5)");
        }

        #[test]
        fn display_mixed_values() {
            let coord = Coordinate { x: 3.5, y: -2.5 };
            assert_eq!(format!("{}", coord), "(3.5, -2.5)");
        }

        #[test]
        fn display_zero_values() {
            let coord = Coordinate { x: 0.0, y: 0.0 };
            // Rust's default f64 formatting may display "0" instead of "0.0"
            let output = format!("{}", coord);
            assert!(output.starts_with("(") && output.ends_with(")"));
            assert!(output.contains("0"));
        }

        #[test]
        fn display_integer_like() {
            let coord = Coordinate { x: 1.5, y: 2.5 };
            assert_eq!(format!("{}", coord), "(1.5, 2.5)");
        }

        #[test]
        fn debug_format() {
            let coord = Coordinate { x: 3.5, y: -2.0 };
            assert_eq!(format!("{:?}", coord), "Coordinate { x: 3.5, y: -2.0 }");
        }

        #[test]
        fn debug_contains_type_name() {
            let coord = Coordinate { x: 1.0, y: 2.0 };
            let debug_str = format!("{:?}", coord);
            assert!(debug_str.contains("Coordinate"));
        }

        #[test]
        fn pretty_debug_format() {
            let coord = Coordinate { x: 3.5, y: -2.0 };
            let pretty = format!("{:#?}", coord);
            assert!(pretty.contains("Coordinate"));
            assert!(pretty.contains("x:"));
            assert!(pretty.contains("y:"));
        }

        #[test]
        fn to_string_uses_display() {
            let coord = Coordinate { x: 1.5, y: 2.5 };
            assert_eq!(coord.to_string(), "(1.5, 2.5)");
        }

        #[test]
        fn large_values() {
            let coord = Coordinate {
                x: 1000000.5,
                y: -999999.9,
            };
            assert_eq!(format!("{}", coord), "(1000000.5, -999999.9)");
        }
    }

    // ============================================================================
    // Color Tests
    // ============================================================================

    mod color_tests {
        use super::*;

        #[test]
        fn display_red() {
            assert_eq!(format!("{}", Color::Red), "Red");
        }

        #[test]
        fn display_green() {
            assert_eq!(format!("{}", Color::Green), "Green");
        }

        #[test]
        fn display_blue() {
            assert_eq!(format!("{}", Color::Blue), "Blue");
        }

        #[test]
        fn display_rgb_basic() {
            let color = Color::Rgb(255, 128, 0);
            assert_eq!(format!("{}", color), "rgb(255, 128, 0)");
        }

        #[test]
        fn display_rgb_zeros() {
            let color = Color::Rgb(0, 0, 0);
            assert_eq!(format!("{}", color), "rgb(0, 0, 0)");
        }

        #[test]
        fn display_rgb_max() {
            let color = Color::Rgb(255, 255, 255);
            assert_eq!(format!("{}", color), "rgb(255, 255, 255)");
        }

        #[test]
        fn display_hex() {
            let color = Color::Hex(String::from("#FF5733"));
            assert_eq!(format!("{}", color), "#FF5733");
        }

        #[test]
        fn display_hex_lowercase() {
            let color = Color::Hex(String::from("#ff5733"));
            assert_eq!(format!("{}", color), "#ff5733");
        }

        #[test]
        fn display_hex_short() {
            let color = Color::Hex(String::from("#FFF"));
            assert_eq!(format!("{}", color), "#FFF");
        }

        #[test]
        fn debug_red() {
            assert_eq!(format!("{:?}", Color::Red), "Red");
        }

        #[test]
        fn debug_rgb() {
            let color = Color::Rgb(100, 150, 200);
            let debug_str = format!("{:?}", color);
            assert!(debug_str.contains("Rgb"));
            assert!(debug_str.contains("100"));
        }

        #[test]
        fn debug_hex() {
            let color = Color::Hex(String::from("#ABC"));
            let debug_str = format!("{:?}", color);
            assert!(debug_str.contains("Hex"));
            assert!(debug_str.contains("#ABC"));
        }

        #[test]
        fn to_string_uses_display() {
            assert_eq!(Color::Red.to_string(), "Red");
            assert_eq!(Color::Rgb(1, 2, 3).to_string(), "rgb(1, 2, 3)");
        }

        #[test]
        fn pretty_debug_rgb() {
            let color = Color::Rgb(10, 20, 30);
            let pretty = format!("{:#?}", color);
            assert!(pretty.contains("Rgb"));
        }
    }

    // ============================================================================
    // Temperature Tests
    // ============================================================================

    mod temperature_tests {
        use super::*;

        #[test]
        fn display_celsius_positive() {
            let temp = Temperature {
                value: 25.5,
                unit: TemperatureUnit::Celsius,
            };
            assert_eq!(format!("{}", temp), "25.5°C");
        }

        #[test]
        fn display_celsius_negative() {
            let temp = Temperature {
                value: -10.0,
                unit: TemperatureUnit::Celsius,
            };
            assert_eq!(format!("{}", temp), "-10°C");
        }

        #[test]
        fn display_celsius_zero() {
            let temp = Temperature {
                value: 0.0,
                unit: TemperatureUnit::Celsius,
            };
            assert_eq!(format!("{}", temp), "0°C");
        }

        #[test]
        fn display_fahrenheit_positive() {
            let temp = Temperature {
                value: 98.6,
                unit: TemperatureUnit::Fahrenheit,
            };
            assert_eq!(format!("{}", temp), "98.6°F");
        }

        #[test]
        fn display_fahrenheit_freezing() {
            let temp = Temperature {
                value: 32.0,
                unit: TemperatureUnit::Fahrenheit,
            };
            assert_eq!(format!("{}", temp), "32°F");
        }

        #[test]
        fn display_kelvin() {
            let temp = Temperature {
                value: 300.0,
                unit: TemperatureUnit::Kelvin,
            };
            assert_eq!(format!("{}", temp), "300K");
        }

        #[test]
        fn display_kelvin_absolute_zero() {
            let temp = Temperature {
                value: 0.0,
                unit: TemperatureUnit::Kelvin,
            };
            assert_eq!(format!("{}", temp), "0K");
        }

        #[test]
        fn display_kelvin_decimal() {
            let temp = Temperature {
                value: 273.15,
                unit: TemperatureUnit::Kelvin,
            };
            assert_eq!(format!("{}", temp), "273.15K");
        }

        #[test]
        fn debug_celsius() {
            let temp = Temperature {
                value: 25.0,
                unit: TemperatureUnit::Celsius,
            };
            let debug_str = format!("{:?}", temp);
            assert!(debug_str.contains("Temperature"));
            assert!(debug_str.contains("Celsius"));
        }

        #[test]
        fn debug_fahrenheit() {
            let temp = Temperature {
                value: 100.0,
                unit: TemperatureUnit::Fahrenheit,
            };
            let debug_str = format!("{:?}", temp);
            assert!(debug_str.contains("Fahrenheit"));
        }

        #[test]
        fn debug_kelvin() {
            let temp = Temperature {
                value: 300.0,
                unit: TemperatureUnit::Kelvin,
            };
            let debug_str = format!("{:?}", temp);
            assert!(debug_str.contains("Kelvin"));
        }

        #[test]
        fn to_string_uses_display() {
            let temp = Temperature {
                value: 20.0,
                unit: TemperatureUnit::Celsius,
            };
            assert_eq!(temp.to_string(), "20°C");
        }

        #[test]
        fn temperature_unit_debug() {
            assert_eq!(format!("{:?}", TemperatureUnit::Celsius), "Celsius");
            assert_eq!(format!("{:?}", TemperatureUnit::Fahrenheit), "Fahrenheit");
            assert_eq!(format!("{:?}", TemperatureUnit::Kelvin), "Kelvin");
        }
    }

    // ============================================================================
    // LogLevel Tests
    // ============================================================================

    mod log_level_tests {
        use super::*;

        #[test]
        fn display_error() {
            assert_eq!(format!("{}", LogLevel::Error), "ERROR");
        }

        #[test]
        fn display_warning() {
            assert_eq!(format!("{}", LogLevel::Warning), "WARNING");
        }

        #[test]
        fn display_info() {
            assert_eq!(format!("{}", LogLevel::Info), "INFO");
        }

        #[test]
        fn display_debug() {
            assert_eq!(format!("{}", LogLevel::Debug), "DEBUG");
        }

        #[test]
        fn debug_error() {
            assert_eq!(format!("{:?}", LogLevel::Error), "Error");
        }

        #[test]
        fn debug_warning() {
            assert_eq!(format!("{:?}", LogLevel::Warning), "Warning");
        }

        #[test]
        fn debug_info() {
            assert_eq!(format!("{:?}", LogLevel::Info), "Info");
        }

        #[test]
        fn debug_debug_level() {
            assert_eq!(format!("{:?}", LogLevel::Debug), "Debug");
        }

        #[test]
        fn to_string_uses_display() {
            assert_eq!(LogLevel::Error.to_string(), "ERROR");
            assert_eq!(LogLevel::Warning.to_string(), "WARNING");
        }

        #[test]
        fn display_is_uppercase() {
            // Verify all display outputs are uppercase
            for level in [
                LogLevel::Error,
                LogLevel::Warning,
                LogLevel::Info,
                LogLevel::Debug,
            ] {
                let display = format!("{}", level);
                assert_eq!(display, display.to_uppercase());
            }
        }
    }

    // ============================================================================
    // LogMessage Tests
    // ============================================================================

    mod log_message_tests {
        use super::*;

        #[test]
        fn display_error_message() {
            let log = LogMessage {
                level: LogLevel::Error,
                message: String::from("Connection failed"),
            };
            assert_eq!(format!("{}", log), "[ERROR] Connection failed");
        }

        #[test]
        fn display_warning_message() {
            let log = LogMessage {
                level: LogLevel::Warning,
                message: String::from("Low memory"),
            };
            assert_eq!(format!("{}", log), "[WARNING] Low memory");
        }

        #[test]
        fn display_info_message() {
            let log = LogMessage {
                level: LogLevel::Info,
                message: String::from("Server started"),
            };
            assert_eq!(format!("{}", log), "[INFO] Server started");
        }

        #[test]
        fn display_debug_message() {
            let log = LogMessage {
                level: LogLevel::Debug,
                message: String::from("Variable x = 42"),
            };
            assert_eq!(format!("{}", log), "[DEBUG] Variable x = 42");
        }

        #[test]
        fn display_empty_message() {
            let log = LogMessage {
                level: LogLevel::Info,
                message: String::new(),
            };
            assert_eq!(format!("{}", log), "[INFO] ");
        }

        #[test]
        fn display_long_message() {
            let log = LogMessage {
                level: LogLevel::Error,
                message: String::from(
                    "This is a very long error message that contains lots of information about what went wrong",
                ),
            };
            assert!(format!("{}", log).starts_with("[ERROR] This is a very long"));
        }

        #[test]
        fn debug_format() {
            let log = LogMessage {
                level: LogLevel::Error,
                message: String::from("Test"),
            };
            let debug_str = format!("{:?}", log);
            assert!(debug_str.contains("LogMessage"));
            assert!(debug_str.contains("level"));
            assert!(debug_str.contains("message"));
        }

        #[test]
        fn pretty_debug_format() {
            let log = LogMessage {
                level: LogLevel::Warning,
                message: String::from("Test"),
            };
            let pretty = format!("{:#?}", log);
            assert!(pretty.contains("LogMessage"));
            assert!(pretty.contains("level:"));
            assert!(pretty.contains("message:"));
        }

        #[test]
        fn to_string_uses_display() {
            let log = LogMessage {
                level: LogLevel::Info,
                message: String::from("Hello"),
            };
            assert_eq!(log.to_string(), "[INFO] Hello");
        }

        #[test]
        fn message_with_special_characters() {
            let log = LogMessage {
                level: LogLevel::Error,
                message: String::from("Error: file 'test.txt' not found!"),
            };
            assert_eq!(
                format!("{}", log),
                "[ERROR] Error: file 'test.txt' not found!"
            );
        }
    }

    // ============================================================================
    // Utility Function Tests
    // ============================================================================

    mod debug_string_tests {
        use super::*;

        #[test]
        fn debug_string_coordinate() {
            let coord = Coordinate { x: 1.0, y: 2.0 };
            assert_eq!(debug_string(&coord), "Coordinate { x: 1.0, y: 2.0 }");
        }

        #[test]
        fn debug_string_color_red() {
            assert_eq!(debug_string(&Color::Red), "Red");
        }

        #[test]
        fn debug_string_color_rgb() {
            let color = Color::Rgb(100, 150, 200);
            let result = debug_string(&color);
            assert!(result.contains("Rgb"));
        }

        #[test]
        fn debug_string_temperature() {
            let temp = Temperature {
                value: 25.0,
                unit: TemperatureUnit::Celsius,
            };
            let result = debug_string(&temp);
            assert!(result.contains("Temperature"));
            assert!(result.contains("Celsius"));
        }

        #[test]
        fn debug_string_log_level() {
            assert_eq!(debug_string(&LogLevel::Error), "Error");
        }

        #[test]
        fn debug_string_log_message() {
            let log = LogMessage {
                level: LogLevel::Info,
                message: String::from("Test"),
            };
            let result = debug_string(&log);
            assert!(result.contains("LogMessage"));
        }

        #[test]
        fn debug_string_primitive() {
            assert_eq!(debug_string(&42), "42");
            assert_eq!(debug_string(&"hello"), "\"hello\"");
        }

        #[test]
        fn debug_string_vec() {
            let v = vec![1, 2, 3];
            assert_eq!(debug_string(&v), "[1, 2, 3]");
        }
    }

    mod display_string_tests {
        use super::*;

        #[test]
        fn display_string_coordinate() {
            let coord = Coordinate { x: 1.5, y: 2.5 };
            assert_eq!(display_string(&coord), "(1.5, 2.5)");
        }

        #[test]
        fn display_string_color_red() {
            assert_eq!(display_string(&Color::Red), "Red");
        }

        #[test]
        fn display_string_color_rgb() {
            let color = Color::Rgb(100, 150, 200);
            assert_eq!(display_string(&color), "rgb(100, 150, 200)");
        }

        #[test]
        fn display_string_temperature() {
            let temp = Temperature {
                value: 25.0,
                unit: TemperatureUnit::Celsius,
            };
            assert_eq!(display_string(&temp), "25°C");
        }

        #[test]
        fn display_string_log_level() {
            assert_eq!(display_string(&LogLevel::Error), "ERROR");
        }

        #[test]
        fn display_string_log_message() {
            let log = LogMessage {
                level: LogLevel::Warning,
                message: String::from("Test"),
            };
            assert_eq!(display_string(&log), "[WARNING] Test");
        }

        #[test]
        fn display_string_primitive() {
            assert_eq!(display_string(&42), "42");
            assert_eq!(display_string(&"hello"), "hello");
        }

        #[test]
        fn display_string_matches_to_string() {
            let coord = Coordinate { x: 5.0, y: 10.0 };
            assert_eq!(display_string(&coord), coord.to_string());
        }
    }

    mod pretty_debug_tests {
        use super::*;

        #[test]
        fn pretty_debug_coordinate() {
            let coord = Coordinate { x: 1.0, y: 2.0 };
            let result = pretty_debug(&coord);
            assert!(result.contains("Coordinate"));
            assert!(result.contains("x:"));
            assert!(result.contains("y:"));
        }

        #[test]
        fn pretty_debug_color_rgb() {
            let color = Color::Rgb(100, 150, 200);
            let result = pretty_debug(&color);
            assert!(result.contains("Rgb"));
        }

        #[test]
        fn pretty_debug_temperature() {
            let temp = Temperature {
                value: 25.0,
                unit: TemperatureUnit::Celsius,
            };
            let result = pretty_debug(&temp);
            assert!(result.contains("Temperature"));
        }

        #[test]
        fn pretty_debug_log_message() {
            let log = LogMessage {
                level: LogLevel::Error,
                message: String::from("Test error"),
            };
            let result = pretty_debug(&log);
            assert!(result.contains("LogMessage"));
            assert!(result.contains("level:"));
            assert!(result.contains("message:"));
        }

        #[test]
        fn pretty_debug_vec() {
            let v = vec![1, 2, 3, 4, 5];
            let result = pretty_debug(&v);
            assert!(result.contains("1"));
            assert!(result.contains("5"));
        }

        #[test]
        fn pretty_debug_nested_structure() {
            let logs = vec![
                LogMessage {
                    level: LogLevel::Info,
                    message: String::from("Start"),
                },
                LogMessage {
                    level: LogLevel::Error,
                    message: String::from("Failed"),
                },
            ];
            let result = pretty_debug(&logs);
            assert!(result.contains("LogMessage"));
            assert!(result.contains("Start"));
            assert!(result.contains("Failed"));
        }
    }

    // ============================================================================
    // Integration Tests
    // ============================================================================

    mod integration_tests {
        use super::*;

        #[test]
        fn debug_vs_display_coordinate() {
            let coord = Coordinate { x: 3.5, y: -2.5 };
            let debug_out = format!("{:?}", coord);
            let display_out = format!("{}", coord);

            // Debug includes type name and field names
            assert!(debug_out.contains("Coordinate"));
            assert!(debug_out.contains("x:"));

            // Display is more concise
            assert_eq!(display_out, "(3.5, -2.5)");
            assert!(!display_out.contains("Coordinate"));
        }

        #[test]
        fn debug_vs_display_color() {
            let color = Color::Rgb(255, 0, 128);
            let debug_out = format!("{:?}", color);
            let display_out = format!("{}", color);

            // Debug shows variant name
            assert!(debug_out.contains("Rgb"));

            // Display shows user-friendly format
            assert_eq!(display_out, "rgb(255, 0, 128)");
        }

        #[test]
        fn utility_functions_consistency() {
            let coord = Coordinate { x: 1.0, y: 2.0 };

            assert_eq!(debug_string(&coord), format!("{:?}", coord));
            assert_eq!(display_string(&coord), format!("{}", coord));
            assert_eq!(pretty_debug(&coord), format!("{:#?}", coord));
        }

        #[test]
        fn log_system_workflow() {
            let logs = vec![
                LogMessage {
                    level: LogLevel::Info,
                    message: String::from("Application started"),
                },
                LogMessage {
                    level: LogLevel::Warning,
                    message: String::from("Memory usage high"),
                },
                LogMessage {
                    level: LogLevel::Error,
                    message: String::from("Database connection lost"),
                },
            ];

            let output: Vec<String> = logs.iter().map(|log| log.to_string()).collect();

            assert_eq!(output[0], "[INFO] Application started");
            assert_eq!(output[1], "[WARNING] Memory usage high");
            assert_eq!(output[2], "[ERROR] Database connection lost");
        }

        #[test]
        fn temperature_formatting_consistency() {
            let temps = vec![
                Temperature {
                    value: 0.0,
                    unit: TemperatureUnit::Celsius,
                },
                Temperature {
                    value: 32.0,
                    unit: TemperatureUnit::Fahrenheit,
                },
                Temperature {
                    value: 273.15,
                    unit: TemperatureUnit::Kelvin,
                },
            ];

            // All represent roughly the same temperature (freezing point)
            assert_eq!(temps[0].to_string(), "0°C");
            assert_eq!(temps[1].to_string(), "32°F");
            assert_eq!(temps[2].to_string(), "273.15K");
        }

        #[test]
        fn color_palette_display() {
            let palette = vec![
                Color::Red,
                Color::Green,
                Color::Blue,
                Color::Rgb(255, 165, 0),             // Orange
                Color::Hex(String::from("#800080")), // Purple
            ];

            let names: Vec<String> = palette.iter().map(|c| c.to_string()).collect();

            assert_eq!(
                names,
                vec!["Red", "Green", "Blue", "rgb(255, 165, 0)", "#800080"]
            );
        }

        #[test]
        fn debug_for_troubleshooting() {
            // Simulate debugging a problematic coordinate
            let coord = Coordinate {
                x: f64::NAN,
                y: f64::INFINITY,
            };
            let debug_out = format!("{:?}", coord);

            assert!(debug_out.contains("NaN"));
            assert!(debug_out.contains("inf"));
        }

        #[test]
        fn combined_display_output() {
            let coord = Coordinate { x: 10.5, y: 20.5 };
            let temp = Temperature {
                value: 25.5,
                unit: TemperatureUnit::Celsius,
            };
            let color = Color::Rgb(255, 128, 64);

            let combined = format!("Point {} at {} with color {}", coord, temp, color);
            assert_eq!(
                combined,
                "Point (10.5, 20.5) at 25.5°C with color rgb(255, 128, 64)"
            );
        }

        #[test]
        fn generic_debug_function_with_various_types() {
            // Test debug_string works with various types
            assert!(debug_string(&Coordinate { x: 0.0, y: 0.0 }).contains("Coordinate"));
            assert!(debug_string(&Color::Red).contains("Red"));
            assert!(debug_string(&LogLevel::Error).contains("Error"));
            assert!(debug_string(&vec![1, 2, 3]).contains("[1, 2, 3]"));
            assert!(debug_string(&Some(42)).contains("Some"));
        }

        #[test]
        fn generic_display_function_with_various_types() {
            // Test display_string works with various types
            assert_eq!(display_string(&Coordinate { x: 1.5, y: 2.5 }), "(1.5, 2.5)");
            assert_eq!(display_string(&Color::Green), "Green");
            assert_eq!(display_string(&LogLevel::Warning), "WARNING");
            assert_eq!(display_string(&42i32), "42");
            assert_eq!(display_string(&String::from("test")), "test");
        }
    }
}
