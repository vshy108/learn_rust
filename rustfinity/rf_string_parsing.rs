//! # Rustfinity: String Parsing
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/string-parsing
//!
//! Rust makes it easy to parse strings into other types using the `.parse()` method and the `FromStr` trait. This is a fundamental skill for handling user input, reading configuration files, or processing text data.
//!
//! ## The `parse()` Method
//!
//! The `.parse()` method attempts to convert a string slice into another type. It returns a `Result` because parsing can fail if the string doesn't represent a valid value of the target type:
//!
//! ```rust
//! let num: i32 = "42".parse().unwrap();
//! assert_eq!(num, 42);
//!
//! // Or with turbofish syntax:
//! let num = "42".parse::<i32>().unwrap();
//! ```
//!
//! ## The `FromStr` Trait
//!
//! When you call `.parse()`, Rust uses the `FromStr` trait behind the scenes. You can implement this trait for your own types to make them parseable from strings:
//!
//! ```rust
//! use std::str::FromStr;
//!
//! struct Point {
//!     x: i32,
//!     y: i32,
//! }
//!
//! impl FromStr for Point {
//!     type Err = String;
//!
//!     fn from_str(s: &str) -> Result<Self, Self::Err> {
//!         // Parse "x,y" format
//!         let parts: Vec<&str> = s.split(',').collect();
//!         if parts.len() != 2 {
//!             return Err("Expected format: x,y".to_string());
//!         }
//!         let x = parts[0]
//!             .trim()
//!             .parse()
//!             .map_err(|_| "Invalid x")?;
//!         let y = parts[1]
//!             .trim()
//!             .parse()
//!             .map_err(|_| "Invalid y")?;
//!         Ok(Point { x, y })
//!     }
//! }
//! ```
//!
//! ## Your Task
//!
//! Implement the following:
//!
//! 1. `parse_int(s: &str) -> Result<i32, String>` - Parse a string into an i32, returning a descriptive error message on failure
//! 2. `parse_bool(s: &str) -> Result<bool, String>` - Parse common boolean representations: "true", "false", "1", "0", "yes", "no" (case-insensitive)
//! 3. `parse_key_value(s: &str) -> Result<(String, String), String>` - Parse a "key=value" string into a tuple
//! 4. Implement `FromStr` for a `Color` struct that parses "r,g,b" format (e.g., "255,128,0")
//! 5. `parse_list<T: FromStr>(s: &str, delimiter: char) -> Result<Vec<T>, String>` - Parse a delimited list of values into a Vec
//!
//! ## Examples
//!
//! ```rust
//! // parse_int: string to integer
//! assert_eq!(parse_int("42"), Ok(42));
//! assert_eq!(parse_int("-17"), Ok(-17));
//! assert!(parse_int("not a number").is_err());
//!
//! // parse_bool: flexible boolean parsing
//! assert_eq!(parse_bool("true"), Ok(true));
//! assert_eq!(parse_bool("YES"), Ok(true));
//! assert_eq!(parse_bool("0"), Ok(false));
//! assert!(parse_bool("maybe").is_err());
//!
//! // parse_key_value: extract key and value
//! assert_eq!(
//!     parse_key_value("name=Alice"),
//!     Ok(("name".to_string(), "Alice".to_string()))
//! );
//! assert_eq!(
//!     parse_key_value("count=42"),
//!     Ok(("count".to_string(), "42".to_string()))
//! );
//! assert!(parse_key_value("no_equals_sign").is_err());
//!
//! // Color with FromStr
//! let color: Color = "255,128,0".parse().unwrap();
//! assert_eq!(color.r, 255);
//! assert_eq!(color.g, 128);
//! assert_eq!(color.b, 0);
//!
//! // parse_list: generic list parsing
//! assert_eq!(
//!     parse_list::<i32>("1,2,3", ','),
//!     Ok(vec![1, 2, 3])
//! );
//! assert_eq!(
//!     parse_list::<f64>("1.5;2.5;3.5", ';'),
//!     Ok(vec![1.5, 2.5, 3.5])
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `parse_int`, use `.parse::<i32>()` and `.map_err()` to convert the error
//! - For `parse_bool`, match on `.to_lowercase().as_str()` for case-insensitive comparison
//! - For `parse_key_value`, use `.splitn(2, '=')` to split at only the first `=`
//! - For the `Color` struct, implement `FromStr` with `type Err = String`
//! - For `parse_list`, use `.split()`, then `.map()` with `.parse()`, and handle the Results with `.collect::<Result<Vec<T>, _>>()`
//! - The `?` operator is your friend for propagating errors
//!
//! </details>
//!
//! Run: `cargo run --example rf_string_parsing`
//! Test: `cargo test --example rf_string_parsing`
//! Filter: `cargo test --example rf_string_parsing <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::str::FromStr;

/// Parse a string into an i32, returning a descriptive error message on failure.
pub fn parse_int(s: &str) -> Result<i32, String> {
    // TODO: Parse the string as an i32
    unimplemented!()
}

/// Parse common boolean representations (case-insensitive).
/// Accepts: "true", "false", "1", "0", "yes", "no"
pub fn parse_bool(s: &str) -> Result<bool, String> {
    // TODO: Match on the lowercase version of the string
    unimplemented!()
}

/// Parse a "key=value" string into a tuple.
pub fn parse_key_value(s: &str) -> Result<(String, String), String> {
    // TODO: Split the string at '=' and return (key, value)
    unimplemented!()
}

/// A color represented by red, green, and blue components.
#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// TODO: Implement FromStr for Color
// The format is "r,g,b" (e.g., "255,128,0")
impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Parse the "r,g,b" format
        unimplemented!()
    }
}

/// Parse a delimited list of values into a Vec.
pub fn parse_list<T: FromStr>(s: &str, delimiter: char) -> Result<Vec<T>, String> {
    // TODO: Split by delimiter, parse each part, collect into Result<Vec<T>, String>
    unimplemented!()
}

pub fn main() {
    // Demonstrate parse_int
    println!("Parsing integers:");
    println!("  '42' -> {:?}", parse_int("42"));
    println!("  '-17' -> {:?}", parse_int("-17"));
    println!("  'abc' -> {:?}", parse_int("abc"));

    // Demonstrate parse_bool
    println!("\nParsing booleans:");
    println!("  'true' -> {:?}", parse_bool("true"));
    println!("  'YES' -> {:?}", parse_bool("YES"));
    println!("  '0' -> {:?}", parse_bool("0"));
    println!("  'maybe' -> {:?}", parse_bool("maybe"));

    // Demonstrate parse_key_value
    println!("\nParsing key=value pairs:");
    println!("  'name=Alice' -> {:?}", parse_key_value("name=Alice"));
    println!("  'count=42' -> {:?}", parse_key_value("count=42"));
    println!("  'invalid' -> {:?}", parse_key_value("invalid"));

    // Demonstrate Color parsing
    println!("\nParsing colors:");
    let color: Result<Color, _> = "255,128,0".parse();
    println!("  '255,128,0' -> {:?}", color);

    // Demonstrate parse_list
    println!("\nParsing lists:");
    println!("  '1,2,3' as i32 -> {:?}", parse_list::<i32>("1,2,3", ','));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ============ parse_int tests ============

    #[test]
    fn parse_int_positive() {
        assert_eq!(parse_int("42"), Ok(42));
    }

    #[test]
    fn parse_int_negative() {
        assert_eq!(parse_int("-17"), Ok(-17));
    }

    #[test]
    fn parse_int_zero() {
        assert_eq!(parse_int("0"), Ok(0));
    }

    #[test]
    fn parse_int_with_whitespace() {
        assert_eq!(parse_int("  123  "), Ok(123));
    }

    #[test]
    fn parse_int_max() {
        assert_eq!(parse_int("2147483647"), Ok(i32::MAX));
    }

    #[test]
    fn parse_int_min() {
        assert_eq!(parse_int("-2147483648"), Ok(i32::MIN));
    }

    #[test]
    fn parse_int_invalid() {
        assert!(parse_int("not a number").is_err());
    }

    #[test]
    fn parse_int_empty() {
        assert!(parse_int("").is_err());
    }

    #[test]
    fn parse_int_float() {
        assert!(parse_int("3.14").is_err());
    }

    // ============ parse_bool tests ============

    #[test]
    fn parse_bool_true() {
        assert_eq!(parse_bool("true"), Ok(true));
    }

    #[test]
    fn parse_bool_false() {
        assert_eq!(parse_bool("false"), Ok(false));
    }

    #[test]
    fn parse_bool_yes() {
        assert_eq!(parse_bool("yes"), Ok(true));
    }

    #[test]
    fn parse_bool_no() {
        assert_eq!(parse_bool("no"), Ok(false));
    }

    #[test]
    fn parse_bool_one() {
        assert_eq!(parse_bool("1"), Ok(true));
    }

    #[test]
    fn parse_bool_zero() {
        assert_eq!(parse_bool("0"), Ok(false));
    }

    #[test]
    fn parse_bool_case_insensitive() {
        assert_eq!(parse_bool("TRUE"), Ok(true));
        assert_eq!(parse_bool("FALSE"), Ok(false));
        assert_eq!(parse_bool("Yes"), Ok(true));
        assert_eq!(parse_bool("NO"), Ok(false));
    }

    #[test]
    fn parse_bool_with_whitespace() {
        assert_eq!(parse_bool("  true  "), Ok(true));
    }

    #[test]
    fn parse_bool_invalid() {
        assert!(parse_bool("maybe").is_err());
        assert!(parse_bool("2").is_err());
        assert!(parse_bool("").is_err());
    }

    // ============ parse_key_value tests ============

    #[test]
    fn parse_key_value_simple() {
        assert_eq!(
            parse_key_value("name=Alice"),
            Ok(("name".to_string(), "Alice".to_string()))
        );
    }

    #[test]
    fn parse_key_value_number() {
        assert_eq!(
            parse_key_value("count=42"),
            Ok(("count".to_string(), "42".to_string()))
        );
    }

    #[test]
    fn parse_key_value_empty_value() {
        assert_eq!(
            parse_key_value("key="),
            Ok(("key".to_string(), "".to_string()))
        );
    }

    #[test]
    fn parse_key_value_with_spaces() {
        assert_eq!(
            parse_key_value(" key = value "),
            Ok(("key".to_string(), "value".to_string()))
        );
    }

    #[test]
    fn parse_key_value_multiple_equals() {
        // Should only split at the first '='
        assert_eq!(
            parse_key_value("equation=x=y+z"),
            Ok(("equation".to_string(), "x=y+z".to_string()))
        );
    }

    #[test]
    fn parse_key_value_no_equals() {
        assert!(parse_key_value("no_equals_sign").is_err());
    }

    #[test]
    fn parse_key_value_url_value() {
        assert_eq!(
            parse_key_value("url=https://example.com?foo=bar"),
            Ok(("url".to_string(), "https://example.com?foo=bar".to_string()))
        );
    }

    // ============ Color FromStr tests ============

    #[test]
    fn color_parse_basic() {
        let color: Color = "255,128,0".parse().unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn color_parse_black() {
        let color: Color = "0,0,0".parse().unwrap();
        assert_eq!(color, Color { r: 0, g: 0, b: 0 });
    }

    #[test]
    fn color_parse_white() {
        let color: Color = "255,255,255".parse().unwrap();
        assert_eq!(
            color,
            Color {
                r: 255,
                g: 255,
                b: 255
            }
        );
    }

    #[test]
    fn color_parse_with_spaces() {
        let color: Color = " 100 , 150 , 200 ".parse().unwrap();
        assert_eq!(
            color,
            Color {
                r: 100,
                g: 150,
                b: 200
            }
        );
    }

    #[test]
    fn color_parse_invalid_format() {
        let result: Result<Color, _> = "255,128".parse();
        assert!(result.is_err());
    }

    #[test]
    fn color_parse_too_many_values() {
        let result: Result<Color, _> = "255,128,0,255".parse();
        assert!(result.is_err());
    }

    #[test]
    fn color_parse_out_of_range() {
        let result: Result<Color, _> = "256,0,0".parse();
        assert!(result.is_err());
    }

    #[test]
    fn color_parse_negative() {
        let result: Result<Color, _> = "-1,0,0".parse();
        assert!(result.is_err());
    }

    #[test]
    fn color_parse_non_numeric() {
        let result: Result<Color, _> = "red,green,blue".parse();
        assert!(result.is_err());
    }

    // ============ parse_list tests ============

    #[test]
    fn parse_list_integers() {
        assert_eq!(parse_list::<i32>("1,2,3", ','), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn parse_list_floats() {
        assert_eq!(
            parse_list::<f64>("1.5;2.5;3.5", ';'),
            Ok(vec![1.5, 2.5, 3.5])
        );
    }

    #[test]
    fn parse_list_single_element() {
        assert_eq!(parse_list::<i32>("42", ','), Ok(vec![42]));
    }

    #[test]
    fn parse_list_with_spaces() {
        assert_eq!(parse_list::<i32>(" 1 , 2 , 3 ", ','), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn parse_list_different_delimiter() {
        assert_eq!(parse_list::<i32>("1|2|3", '|'), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn parse_list_negative_numbers() {
        assert_eq!(parse_list::<i32>("-1,-2,-3", ','), Ok(vec![-1, -2, -3]));
    }

    #[test]
    fn parse_list_invalid_element() {
        assert!(parse_list::<i32>("1,two,3", ',').is_err());
    }

    #[test]
    fn parse_list_empty_element() {
        assert!(parse_list::<i32>("1,,3", ',').is_err());
    }

    #[test]
    fn parse_list_unsigned() {
        assert_eq!(parse_list::<u32>("10,20,30", ','), Ok(vec![10, 20, 30]));
    }
}
