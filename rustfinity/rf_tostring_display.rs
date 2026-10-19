//! # Rustfinity: Tostring Display
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/tostring-display
//!
//! The `Display` trait is Rust's standard way to format types for user-facing output. When you implement `Display`, you automatically get the `ToString` trait for free, allowing you to convert your type to a `String`. While the `Debug` trait is for developer-oriented output (often derived automatically), `Display` is meant for clean, human-readable formatting.
//!
//! The `ToString` trait provides a single method `to_string()` that converts a value to a `String`. In practice, you almost never implement `ToString` directly—instead, you implement `Display`, and the standard library provides a blanket implementation of `ToString` for any type that implements `Display`.
//!
//! ## The Display and ToString Traits
//!
//! ```rust
//! use std::fmt;
//!
//! pub trait Display {
//!     fn fmt(
//!         &self,
//!         f: &mut fmt::Formatter<'_>
//!     ) -> fmt::Result;
//! }
//!
//! pub trait ToString {
//!     fn to_string(&self) -> String;
//! }
//! ```
//!
//! The `Display::fmt` method writes formatted output to a `Formatter`. You use the `write!` macro inside this method to write your formatted content.
//!
//! ## Standard Library Examples
//!
//! ```rust
//! use std::fmt;
//!
//! // Built-in types implement Display
//! let n = 42;
//! println!("{}", n);  // Uses Display
//! let s = n.to_string();  // "42"
//!
//! // Strings implement Display trivially
//! let greeting = "Hello";
//! println!("{}", greeting);  // "Hello"
//! ```
//!
//! ## Your Task
//!
//! Implement the `Display` trait and related formatting for the following types:
//!
//! ### 1. Point
//!
//! Create a `Point` struct with `x` and `y` fields (both `i32`). Implement `Display` to format it as `(x, y)`.
//!
//! ### 2. Color
//!
//! Create a `Color` enum with variants `Red`, `Green`, `Blue`, and `Custom(u8, u8, u8)`. Implement `Display` to show:
//!
//! - Named colors as their name (e.g., "Red", "Green", "Blue")
//! - Custom colors as "RGB(r, g, b)"
//!
//! ### 3. Temperature
//!
//! Create a `Temperature` enum with variants `Celsius(f64)` and `Fahrenheit(f64)`. Implement `Display` to format as:
//!
//! - Celsius: "X°C" (e.g., "25.5°C")
//! - Fahrenheit: "X°F" (e.g., "77.0°F")
//!
//! ### 4. Money
//!
//! Create a `Money` struct with `amount` (i64, representing
//! cents) and `currency` (String) fields. Implement `Display`
//! to format as "$X.XX" for USD, "€X.XX" for EUR, or
//! "X.XX CURRENCY" for others. The amount should always show
//! exactly 2 decimal places.
//!
//! ### 5. Person
//!
//! Create a `Person` struct with `name` (String) and `age` (u32) fields. Implement `Display` to format as "Name (age X)".
//!
//! ### 6. list_to_string Function
//!
//! Write a generic function
//! `list_to_string<T: Display>(items: &[T]) -> String`
//! that formats a slice of displayable items as a
//! comma-separated list in square brackets,
//! e.g., "[1, 2, 3]".
//!
//! ### 7. format_table Function
//!
//! Write a function
//! `format_table(headers: &[&str], rows: &[Vec<String>])
//! -> String` that formats data as a simple text table with
//! headers and rows, each cell separated by " | ".
//!
//! ## Examples
//!
//! ```rust
//! // Point
//! let point = Point { x: 3, y: -4 };
//! assert_eq!(point.to_string(), "(3, -4)");
//! assert_eq!(format!("{}", point), "(3, -4)");
//!
//! // Color
//! let red = Color::Red;
//! assert_eq!(red.to_string(), "Red");
//! let custom = Color::Custom(255, 128, 0);
//! assert_eq!(custom.to_string(), "RGB(255, 128, 0)");
//!
//! // Temperature
//! let celsius = Temperature::Celsius(25.5);
//! assert_eq!(celsius.to_string(), "25.5°C");
//! let fahrenheit = Temperature::Fahrenheit(77.0);
//! assert_eq!(fahrenheit.to_string(), "77°F");
//!
//! // Money
//! let usd = Money {
//!     amount: 1234,
//!     currency: "USD".to_string()
//! };
//! assert_eq!(
//!     usd.to_string(),
//!     "$12.34"
//! );
//! let eur = Money {
//!     amount: 5000,
//!     currency: "EUR".to_string()
//! };
//! assert_eq!(
//!     eur.to_string(),
//!     "€50.00"
//! );
//! let other = Money {
//!     amount: 999,
//!     currency: "GBP".to_string()
//! };
//! assert_eq!(
//!     other.to_string(),
//!     "9.99 GBP"
//! );
//!
//! // Person
//! let person = Person {
//!     name: "Alice".to_string(),
//!     age: 30
//! };
//! assert_eq!(person.to_string(), "Alice (age 30)");
//!
//! // list_to_string
//! let numbers = vec![1, 2, 3];
//! assert_eq!(list_to_string(&numbers), "[1, 2, 3]");
//! let empty: Vec<i32> = vec![];
//! assert_eq!(list_to_string(&empty), "[]");
//!
//! // format_table
//! let headers = vec!["Name", "Age"];
//! let rows = vec![
//!     vec!["Alice".to_string(), "30".to_string()],
//!     vec!["Bob".to_string(), "25".to_string()],
//! ];
//! let table = format_table(&headers, &rows);
//! assert!(table.contains("Name | Age"));
//! assert!(table.contains("Alice | 30"));
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `write!(f, "...")` inside the `fmt` method to write
//!   formatted output
//! - The `Formatter` passed to `fmt` implements
//!   `std::fmt::Write`, so `write!` works with it
//! - For the degree symbol, you can use the Unicode
//!   character `°` (or `\u{00B0}`)
//! - To format floats with specific precision, use `{:.1}`
//!   for 1 decimal place
//! - Remember that `to_string()` is automatically available
//!   once you implement `Display`
//! - For Money, convert cents to dollars by dividing:
//!   `amount / 100` for whole part, `amount % 100` for cents
//! - Use `format!` to build intermediate strings when needed
//! - The `join` method on iterators is useful for creating
//!   comma-separated lists
//!
//! </details>
//!
//! Run: `cargo run --example rf_tostring_display`
//! Test: `cargo test --example rf_tostring_display`
//! Filter: `cargo test --example rf_tostring_display <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::fmt;

/// A point in 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// TODO: Implement the Display trait for Point
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Use write!(f, ...) to format the point
        todo!()
    }
}

/// A color represented as either a named color or custom RGB values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

// TODO: Implement the Display trait for Color
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Match on self and format each variant appropriately
        todo!()
    }
}

/// A temperature in either Celsius or Fahrenheit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

// TODO: Implement the Display trait for Temperature
impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Match on self and format each variant
        todo!()
    }
}

/// Money represented in cents with a currency code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    pub amount: i64, // Amount in cents (can be negative)
    pub currency: String,
}

// TODO: Implement the Display trait for Money
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Convert cents to dollars/cents
        todo!()
    }
}

/// A person with a name and age.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

// TODO: Implement the Display trait for Person
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Format the person
        todo!()
    }
}

/// Converts a slice of displayable items to a comma-separated string in square brackets.
///
/// Examples:
/// - [1, 2, 3] -> "[1, 2, 3]"
/// - [] -> "[]"
/// - ["hello"] -> "[hello]"
pub fn list_to_string<T: fmt::Display>(items: &[T]) -> String {
    // TODO: Iterate over items, convert each to string, and join with ", "
    todo!()
}

/// Formats data as a simple text table with headers and rows.
///
/// Each row is formatted with cells separated by " | ", and the header row
/// is followed by a separator line of dashes.
///
/// Example output:
/// ```text
/// Name | Age
/// -----+----
/// Alice | 30
/// Bob | 25
/// ```
pub fn format_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    // TODO: Format as a text table with headers and rows
    todo!()
}

// Example usage
pub fn main() {
    // Point
    let point = Point { x: 3, y: -4 };
    println!("Point: {}", point);

    // Color
    let red = Color::Red;
    let custom = Color::Custom(255, 128, 0);
    println!("Colors: {}, {}", red, custom);

    // Temperature
    let celsius = Temperature::Celsius(25.5);
    let fahrenheit = Temperature::Fahrenheit(77.0);
    println!("Temperatures: {}, {}", celsius, fahrenheit);

    // Money
    let usd = Money {
        amount: 1234,
        currency: "USD".to_string(),
    };
    let eur = Money {
        amount: 5000,
        currency: "EUR".to_string(),
    };
    println!("Money: {}, {}", usd, eur);

    // Person
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("Person: {}", person);

    // list_to_string
    let numbers = vec![1, 2, 3];
    println!("List: {}", list_to_string(&numbers));

    // format_table
    let headers = vec!["Name", "Age"];
    let rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];
    println!("Table:\n{}", format_table(&headers, &rows));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ==================== Point Tests ====================

    #[test]
    fn point_positive_coordinates() {
        let p = Point { x: 3, y: 4 };
        assert_eq!(p.to_string(), "(3, 4)");
    }

    #[test]
    fn point_negative_coordinates() {
        let p = Point { x: -3, y: -4 };
        assert_eq!(p.to_string(), "(-3, -4)");
    }

    #[test]
    fn point_mixed_coordinates() {
        let p = Point { x: 3, y: -4 };
        assert_eq!(p.to_string(), "(3, -4)");
    }

    #[test]
    fn point_zero_coordinates() {
        let p = Point { x: 0, y: 0 };
        assert_eq!(p.to_string(), "(0, 0)");
    }

    #[test]
    fn point_format_macro() {
        let p = Point { x: 10, y: 20 };
        assert_eq!(format!("{}", p), "(10, 20)");
    }

    #[test]
    fn point_large_values() {
        let p = Point {
            x: i32::MAX,
            y: i32::MIN,
        };
        assert_eq!(p.to_string(), format!("({}, {})", i32::MAX, i32::MIN));
    }

    // ==================== Color Tests ====================

    #[test]
    fn color_red() {
        let c = Color::Red;
        assert_eq!(c.to_string(), "Red");
    }

    #[test]
    fn color_green() {
        let c = Color::Green;
        assert_eq!(c.to_string(), "Green");
    }

    #[test]
    fn color_blue() {
        let c = Color::Blue;
        assert_eq!(c.to_string(), "Blue");
    }

    #[test]
    fn color_custom() {
        let c = Color::Custom(255, 128, 0);
        assert_eq!(c.to_string(), "RGB(255, 128, 0)");
    }

    #[test]
    fn color_custom_black() {
        let c = Color::Custom(0, 0, 0);
        assert_eq!(c.to_string(), "RGB(0, 0, 0)");
    }

    #[test]
    fn color_custom_white() {
        let c = Color::Custom(255, 255, 255);
        assert_eq!(c.to_string(), "RGB(255, 255, 255)");
    }

    #[test]
    fn color_format_macro() {
        let c = Color::Custom(100, 150, 200);
        assert_eq!(format!("{}", c), "RGB(100, 150, 200)");
    }

    // ==================== Temperature Tests ====================

    #[test]
    fn temperature_celsius_integer() {
        let t = Temperature::Celsius(25.0);
        assert_eq!(t.to_string(), "25°C");
    }

    #[test]
    fn temperature_celsius_decimal() {
        let t = Temperature::Celsius(25.5);
        assert_eq!(t.to_string(), "25.5°C");
    }

    #[test]
    fn temperature_celsius_negative() {
        let t = Temperature::Celsius(-10.0);
        assert_eq!(t.to_string(), "-10°C");
    }

    #[test]
    fn temperature_celsius_zero() {
        let t = Temperature::Celsius(0.0);
        assert_eq!(t.to_string(), "0°C");
    }

    #[test]
    fn temperature_fahrenheit_integer() {
        let t = Temperature::Fahrenheit(77.0);
        assert_eq!(t.to_string(), "77°F");
    }

    #[test]
    fn temperature_fahrenheit_decimal() {
        let t = Temperature::Fahrenheit(98.6);
        assert_eq!(t.to_string(), "98.6°F");
    }

    #[test]
    fn temperature_fahrenheit_negative() {
        let t = Temperature::Fahrenheit(-40.0);
        assert_eq!(t.to_string(), "-40°F");
    }

    #[test]
    fn temperature_format_macro() {
        let t = Temperature::Celsius(100.0);
        assert_eq!(format!("{}", t), "100°C");
    }

    // ==================== Money Tests ====================

    #[test]
    fn money_usd_simple() {
        let m = Money {
            amount: 1234,
            currency: "USD".to_string(),
        };
        assert_eq!(m.to_string(), "$12.34");
    }

    #[test]
    fn money_usd_whole_dollars() {
        let m = Money {
            amount: 5000,
            currency: "USD".to_string(),
        };
        assert_eq!(m.to_string(), "$50.00");
    }

    #[test]
    fn money_usd_cents_only() {
        let m = Money {
            amount: 99,
            currency: "USD".to_string(),
        };
        assert_eq!(m.to_string(), "$0.99");
    }

    #[test]
    fn money_usd_zero() {
        let m = Money {
            amount: 0,
            currency: "USD".to_string(),
        };
        assert_eq!(m.to_string(), "$0.00");
    }

    #[test]
    fn money_usd_negative() {
        let m = Money {
            amount: -1234,
            currency: "USD".to_string(),
        };
        assert_eq!(m.to_string(), "-$12.34");
    }

    #[test]
    fn money_eur_simple() {
        let m = Money {
            amount: 5000,
            currency: "EUR".to_string(),
        };
        assert_eq!(m.to_string(), "€50.00");
    }

    #[test]
    fn money_eur_negative() {
        let m = Money {
            amount: -999,
            currency: "EUR".to_string(),
        };
        assert_eq!(m.to_string(), "-€9.99");
    }

    #[test]
    fn money_other_currency() {
        let m = Money {
            amount: 999,
            currency: "GBP".to_string(),
        };
        assert_eq!(m.to_string(), "9.99 GBP");
    }

    #[test]
    fn money_other_currency_large() {
        let m = Money {
            amount: 123456,
            currency: "JPY".to_string(),
        };
        assert_eq!(m.to_string(), "1234.56 JPY");
    }

    #[test]
    fn money_other_currency_negative() {
        let m = Money {
            amount: -500,
            currency: "CAD".to_string(),
        };
        assert_eq!(m.to_string(), "-5.00 CAD");
    }

    #[test]
    fn money_format_macro() {
        let m = Money {
            amount: 4200,
            currency: "USD".to_string(),
        };
        assert_eq!(format!("{}", m), "$42.00");
    }

    // ==================== Person Tests ====================

    #[test]
    fn person_basic() {
        let p = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        assert_eq!(p.to_string(), "Alice (age 30)");
    }

    #[test]
    fn person_zero_age() {
        let p = Person {
            name: "Baby".to_string(),
            age: 0,
        };
        assert_eq!(p.to_string(), "Baby (age 0)");
    }

    #[test]
    fn person_long_name() {
        let p = Person {
            name: "Alexander Hamilton".to_string(),
            age: 47,
        };
        assert_eq!(p.to_string(), "Alexander Hamilton (age 47)");
    }

    #[test]
    fn person_old_age() {
        let p = Person {
            name: "Elder".to_string(),
            age: 120,
        };
        assert_eq!(p.to_string(), "Elder (age 120)");
    }

    #[test]
    fn person_format_macro() {
        let p = Person {
            name: "Bob".to_string(),
            age: 25,
        };
        assert_eq!(format!("{}", p), "Bob (age 25)");
    }

    // ==================== list_to_string Tests ====================

    #[test]
    fn list_to_string_integers() {
        let numbers = vec![1, 2, 3];
        assert_eq!(list_to_string(&numbers), "[1, 2, 3]");
    }

    #[test]
    fn list_to_string_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(list_to_string(&empty), "[]");
    }

    #[test]
    fn list_to_string_single() {
        let single = vec![42];
        assert_eq!(list_to_string(&single), "[42]");
    }

    #[test]
    fn list_to_string_strings() {
        let strings = vec!["hello", "world"];
        assert_eq!(list_to_string(&strings), "[hello, world]");
    }

    #[test]
    fn list_to_string_points() {
        let points = vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
        assert_eq!(list_to_string(&points), "[(1, 2), (3, 4)]");
    }

    #[test]
    fn list_to_string_colors() {
        let colors = vec![Color::Red, Color::Green, Color::Custom(128, 128, 128)];
        assert_eq!(list_to_string(&colors), "[Red, Green, RGB(128, 128, 128)]");
    }

    #[test]
    fn list_to_string_many_elements() {
        let numbers: Vec<i32> = (1..=10).collect();
        assert_eq!(list_to_string(&numbers), "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]");
    }

    // ==================== format_table Tests ====================

    #[test]
    fn format_table_basic() {
        let headers = vec!["Name", "Age"];
        let rows = vec![
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ];
        let table = format_table(&headers, &rows);
        assert!(table.contains("Name | Age"));
        assert!(table.contains("Alice | 30"));
        assert!(table.contains("Bob | 25"));
    }

    #[test]
    fn format_table_single_row() {
        let headers = vec!["Item", "Price"];
        let rows = vec![vec!["Apple".to_string(), "$1.00".to_string()]];
        let table = format_table(&headers, &rows);
        assert!(table.contains("Item | Price"));
        assert!(table.contains("Apple | $1.00"));
    }

    #[test]
    fn format_table_empty_rows() {
        let headers = vec!["Col1", "Col2"];
        let rows: Vec<Vec<String>> = vec![];
        let table = format_table(&headers, &rows);
        assert!(table.contains("Col1 | Col2"));
        // Should still have header and separator
    }

    #[test]
    fn format_table_single_column() {
        let headers = vec!["Name"];
        let rows = vec![vec!["Alice".to_string()], vec!["Bob".to_string()]];
        let table = format_table(&headers, &rows);
        assert!(table.contains("Name"));
        assert!(table.contains("Alice"));
        assert!(table.contains("Bob"));
    }

    #[test]
    fn format_table_three_columns() {
        let headers = vec!["ID", "Name", "Status"];
        let rows = vec![
            vec!["1".to_string(), "Task A".to_string(), "Done".to_string()],
            vec!["2".to_string(), "Task B".to_string(), "Pending".to_string()],
        ];
        let table = format_table(&headers, &rows);
        assert!(table.contains("ID | Name | Status"));
        assert!(table.contains("1 | Task A | Done"));
        assert!(table.contains("2 | Task B | Pending"));
    }

    #[test]
    fn format_table_has_separator() {
        let headers = vec!["A", "B"];
        let rows = vec![vec!["x".to_string(), "y".to_string()]];
        let table = format_table(&headers, &rows);
        // Check that there's a separator line with dashes
        assert!(table.contains("-"));
    }

    // ==================== Integration Tests ====================

    #[test]
    fn integration_display_in_format_string() {
        let point = Point { x: 5, y: 10 };
        let person = Person {
            name: "Alice".to_string(),
            age: 25,
        };
        let message = format!("Person {} is at {}", person, point);
        assert_eq!(message, "Person Alice (age 25) is at (5, 10)");
    }

    #[test]
    fn integration_nested_display() {
        let temp = Temperature::Celsius(20.0);
        let message = format!("Current temperature: {}", temp);
        assert_eq!(message, "Current temperature: 20°C");
    }

    #[test]
    fn integration_list_of_custom_types() {
        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
        ];
        let result = list_to_string(&people);
        assert_eq!(result, "[Alice (age 30), Bob (age 25)]");
    }

    #[test]
    fn integration_money_in_table() {
        let headers = vec!["Item", "Price"];
        let m1 = Money {
            amount: 1099,
            currency: "USD".to_string(),
        };
        let m2 = Money {
            amount: 2500,
            currency: "USD".to_string(),
        };
        let rows = vec![
            vec!["Coffee".to_string(), m1.to_string()],
            vec!["Sandwich".to_string(), m2.to_string()],
        ];
        let table = format_table(&headers, &rows);
        assert!(table.contains("Coffee | $10.99"));
        assert!(table.contains("Sandwich | $25.00"));
    }

    #[test]
    fn integration_temperatures_list() {
        let temps = vec![
            Temperature::Celsius(0.0),
            Temperature::Celsius(25.0),
            Temperature::Celsius(100.0),
        ];
        let result = list_to_string(&temps);
        assert_eq!(result, "[0°C, 25°C, 100°C]");
    }

    #[test]
    fn integration_to_string_vs_format() {
        let color = Color::Custom(255, 0, 128);
        // Both should produce the same result
        assert_eq!(color.to_string(), format!("{}", color));

        let money = Money {
            amount: 4567,
            currency: "EUR".to_string(),
        };
        assert_eq!(money.to_string(), format!("{}", money));
    }

    #[test]
    fn integration_display_trait_object() {
        // Test that Display works with trait objects
        let point = Point { x: 1, y: 2 };
        let displayable: &dyn std::fmt::Display = &point;
        assert_eq!(format!("{}", displayable), "(1, 2)");
    }
}
