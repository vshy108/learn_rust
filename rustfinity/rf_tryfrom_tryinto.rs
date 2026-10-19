//! # Rustfinity: Tryfrom Tryinto
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/tryfrom-tryinto
//!
//! # TryFrom and TryInto: Fallible Conversions
//!
//! In the previous challenge, you learned about `From` and `Into` traits for infallible conversions. But what happens when a conversion might fail? For example, converting a large `i64` to a `u8` could overflow, or parsing a string into a number might fail if the string contains invalid characters.
//!
//! Rust provides `TryFrom` and `TryInto` traits in the standard library for **fallible conversions** - conversions that might not succeed. These traits return a `Result` type, allowing you to handle conversion failures gracefully instead of panicking.
//!
//! ```rust
//! pub trait TryFrom<T>: Sized {
//!     type Error;
//!     fn try_from(value: T) -> Result<Self, Self::Error>;
//! }
//!
//! pub trait TryInto<T>: Sized {
//!     type Error;
//!     fn try_into(self) -> Result<T, Self::Error>;
//! }
//! ```
//!
//! Just like `From` and `Into`, implementing `TryFrom` automatically gives you `TryInto` for free (due to a blanket implementation in the standard library).
//!
//! ## Your Task
//!
//! Implement the following types and their `TryFrom` implementations:
//!
//! **1. `PositiveNumber`**
//!
//! A wrapper around `i32` that only accepts positive numbers (greater than 0).
//!
//! - Implement `TryFrom<i32>` for `PositiveNumber`
//! - Return `Err("number must be positive")` if the value is 0 or negative
//! - The inner value should be accessible via the `.0` field
//!
//! **2. `Percentage`**
//!
//! A wrapper around `u8` that only accepts values from 0 to 100 (inclusive).
//!
//! - Implement `TryFrom<i32>` for `Percentage`
//! - Return `Err("percentage must be between 0 and 100")` if out of range
//! - The inner value should be accessible via the `.0` field
//!
//! **3. `NonEmptyString`**
//!
//! A wrapper around `String` that ensures the string is not empty.
//!
//! - Implement `TryFrom<String>` for `NonEmptyString`
//! - Implement `TryFrom<&str>` for `NonEmptyString`
//! - Return `Err("string cannot be empty")` if the string is empty
//! - The inner value should be accessible via the `.0` field
//!
//! **4. `EvenNumber`**
//!
//! A wrapper around `i32` that only accepts even numbers.
//!
//! - Implement `TryFrom<i32>` for `EvenNumber`
//! - Return `Err("number must be even")` if the number is odd
//! - The inner value should be accessible via the `.0` field
//!
//! **5. `AsciiChar`**
//!
//! A wrapper around `char` that only accepts ASCII characters.
//!
//! - Implement `TryFrom<char>` for `AsciiChar`
//! - Implement `TryFrom<u8>` for `AsciiChar` (convert byte to char, then validate)
//! - Return `Err("character must be ASCII")` if the character is not ASCII
//! - The inner value should be accessible via the `.0` field
//!
//! ## Examples
//!
//! ```rust
//! use tryfrom_tryinto::*;
//!
//! // PositiveNumber
//! let pos: Result<PositiveNumber, _> = 42.try_into();
//! assert!(pos.is_ok());
//! assert_eq!(pos.unwrap().0, 42);
//!
//! let neg: Result<PositiveNumber, _> = (-5).try_into();
//! assert!(neg.is_err());
//!
//! // Percentage
//! let valid = Percentage::try_from(75);
//! assert!(valid.is_ok());
//!
//! let invalid = Percentage::try_from(150);
//! assert!(invalid.is_err());
//!
//! // NonEmptyString
//! let s = NonEmptyString::try_from("hello");
//! assert!(s.is_ok());
//!
//! let empty = NonEmptyString::try_from("");
//! assert!(empty.is_err());
//!
//! // EvenNumber
//! let even = EvenNumber::try_from(4);
//! assert!(even.is_ok());
//!
//! let odd = EvenNumber::try_from(3);
//! assert!(odd.is_err());
//!
//! // AsciiChar
//! let ascii = AsciiChar::try_from('A');
//! assert!(ascii.is_ok());
//!
//! let non_ascii = AsciiChar::try_from('');
//! assert!(non_ascii.is_err());
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - The `TryFrom` trait requires an associated `Error` type. For simple cases, `&'static str` works well.
//! - Remember that implementing `TryFrom<T>` for your type automatically provides `TryInto<YourType>` for `T`.
//! - Use the `?` operator or pattern matching to handle results elegantly.
//! - For `AsciiChar::try_from(u8)`, you can convert a `u8` to a `char` using `char::from(byte)` or `byte as char`.
//! - The `char::is_ascii()` method checks if a character is ASCII.
//! - Zero is neither positive nor negative, so `PositiveNumber::try_from(0)` should fail.
//! - Even numbers include negative even numbers and zero: -4, -2, 0, 2, 4, ...
//!
//! </details>
//!
//! Run: `cargo run --example rf_tryfrom_tryinto`
//! Test: `cargo test --example rf_tryfrom_tryinto`
//! Filter: `cargo test --example rf_tryfrom_tryinto <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

/// A wrapper around `i32` that only accepts positive numbers (greater than 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositiveNumber(pub i32);

impl TryFrom<i32> for PositiveNumber {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        todo!("validate positive number {value}")
    }
}

/// A wrapper around `u8` that only accepts values from 0 to 100 (inclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Percentage(pub u8);

impl TryFrom<i32> for Percentage {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        todo!("validate percentage {value}")
    }
}

/// A wrapper around `String` that ensures the string is not empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyString(pub String);

impl TryFrom<String> for NonEmptyString {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!("validate non-empty string {value}")
    }
}

impl TryFrom<&str> for NonEmptyString {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!("validate non-empty string {value}")
    }
}

/// A wrapper around `i32` that only accepts even numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvenNumber(pub i32);

impl TryFrom<i32> for EvenNumber {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        todo!("validate even number {value}")
    }
}

/// A wrapper around `char` that only accepts ASCII characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsciiChar(pub char);

impl TryFrom<char> for AsciiChar {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        todo!("validate ASCII character {value}")
    }
}

impl TryFrom<u8> for AsciiChar {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        todo!("convert ASCII byte {value}")
    }
}

// Example usage
pub fn main() {
    // PositiveNumber
    let pos: Result<PositiveNumber, _> = 42.try_into();
    println!("PositiveNumber from 42: {:?}", pos);

    let neg: Result<PositiveNumber, _> = (-5).try_into();
    println!("PositiveNumber from -5: {:?}", neg);

    // Percentage
    let valid = Percentage::try_from(75);
    println!("Percentage from 75: {:?}", valid);

    let invalid = Percentage::try_from(150);
    println!("Percentage from 150: {:?}", invalid);

    // NonEmptyString
    let s = NonEmptyString::try_from("hello");
    println!("NonEmptyString from 'hello': {:?}", s);

    let empty = NonEmptyString::try_from("");
    println!("NonEmptyString from '': {:?}", empty);

    // EvenNumber
    let even = EvenNumber::try_from(4);
    println!("EvenNumber from 4: {:?}", even);

    let odd = EvenNumber::try_from(3);
    println!("EvenNumber from 3: {:?}", odd);

    // AsciiChar
    let ascii = AsciiChar::try_from('A');
    println!("AsciiChar from 'A': {:?}", ascii);

    let non_ascii = AsciiChar::try_from('ñ');
    println!("AsciiChar from 'ñ': {:?}", non_ascii);

    let from_byte = AsciiChar::try_from(65u8);
    println!("AsciiChar from 65u8: {:?}", from_byte);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ============================================================================
    // PositiveNumber tests
    // ============================================================================

    #[test]
    fn positive_number_from_positive() {
        let result = PositiveNumber::try_from(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 42);
    }

    #[test]
    fn positive_number_from_one() {
        let result = PositiveNumber::try_from(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 1);
    }

    #[test]
    fn positive_number_from_large() {
        let result = PositiveNumber::try_from(i32::MAX);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, i32::MAX);
    }

    #[test]
    fn positive_number_from_zero_fails() {
        let result = PositiveNumber::try_from(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "number must be positive");
    }

    #[test]
    fn positive_number_from_negative_fails() {
        let result = PositiveNumber::try_from(-5);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "number must be positive");
    }

    #[test]
    fn positive_number_from_min_fails() {
        let result = PositiveNumber::try_from(i32::MIN);
        assert!(result.is_err());
    }

    #[test]
    fn positive_number_try_into() {
        let result: Result<PositiveNumber, _> = 100.try_into();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 100);
    }

    // ============================================================================
    // Percentage tests
    // ============================================================================

    #[test]
    fn percentage_from_valid() {
        let result = Percentage::try_from(75);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 75);
    }

    #[test]
    fn percentage_from_zero() {
        let result = Percentage::try_from(0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 0);
    }

    #[test]
    fn percentage_from_hundred() {
        let result = Percentage::try_from(100);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 100);
    }

    #[test]
    fn percentage_from_fifty() {
        let result = Percentage::try_from(50);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 50);
    }

    #[test]
    fn percentage_over_hundred_fails() {
        let result = Percentage::try_from(101);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "percentage must be between 0 and 100");
    }

    #[test]
    fn percentage_large_value_fails() {
        let result = Percentage::try_from(150);
        assert!(result.is_err());
    }

    #[test]
    fn percentage_negative_fails() {
        let result = Percentage::try_from(-1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "percentage must be between 0 and 100");
    }

    #[test]
    fn percentage_large_negative_fails() {
        let result = Percentage::try_from(-100);
        assert!(result.is_err());
    }

    #[test]
    fn percentage_try_into() {
        let result: Result<Percentage, _> = 50.try_into();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 50);
    }

    // ============================================================================
    // NonEmptyString tests
    // ============================================================================

    #[test]
    fn non_empty_string_from_str() {
        let result = NonEmptyString::try_from("hello");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, "hello");
    }

    #[test]
    fn non_empty_string_from_string() {
        let result = NonEmptyString::try_from(String::from("world"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, "world");
    }

    #[test]
    fn non_empty_string_from_single_char() {
        let result = NonEmptyString::try_from("a");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, "a");
    }

    #[test]
    fn non_empty_string_from_whitespace() {
        let result = NonEmptyString::try_from("   ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, "   ");
    }

    #[test]
    fn non_empty_string_from_unicode() {
        let result = NonEmptyString::try_from("日本語");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, "日本語");
    }

    #[test]
    fn non_empty_string_empty_str_fails() {
        let result = NonEmptyString::try_from("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "string cannot be empty");
    }

    #[test]
    fn non_empty_string_empty_string_fails() {
        let result = NonEmptyString::try_from(String::new());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "string cannot be empty");
    }

    #[test]
    fn non_empty_string_try_into_from_str() {
        let result: Result<NonEmptyString, _> = "test".try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn non_empty_string_try_into_from_string() {
        let s = String::from("test");
        let result: Result<NonEmptyString, _> = s.try_into();
        assert!(result.is_ok());
    }

    // ============================================================================
    // EvenNumber tests
    // ============================================================================

    #[test]
    fn even_number_from_positive_even() {
        let result = EvenNumber::try_from(4);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 4);
    }

    #[test]
    fn even_number_from_zero() {
        let result = EvenNumber::try_from(0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 0);
    }

    #[test]
    fn even_number_from_negative_even() {
        let result = EvenNumber::try_from(-6);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, -6);
    }

    #[test]
    fn even_number_from_large_even() {
        let result = EvenNumber::try_from(1000000);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 1000000);
    }

    #[test]
    fn even_number_from_two() {
        let result = EvenNumber::try_from(2);
        assert!(result.is_ok());
    }

    #[test]
    fn even_number_from_negative_two() {
        let result = EvenNumber::try_from(-2);
        assert!(result.is_ok());
    }

    #[test]
    fn even_number_odd_fails() {
        let result = EvenNumber::try_from(3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "number must be even");
    }

    #[test]
    fn even_number_one_fails() {
        let result = EvenNumber::try_from(1);
        assert!(result.is_err());
    }

    #[test]
    fn even_number_negative_odd_fails() {
        let result = EvenNumber::try_from(-5);
        assert!(result.is_err());
    }

    #[test]
    fn even_number_try_into() {
        let result: Result<EvenNumber, _> = 8.try_into();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 8);
    }

    // ============================================================================
    // AsciiChar tests
    // ============================================================================

    #[test]
    fn ascii_char_from_uppercase() {
        let result = AsciiChar::try_from('A');
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 'A');
    }

    #[test]
    fn ascii_char_from_lowercase() {
        let result = AsciiChar::try_from('z');
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 'z');
    }

    #[test]
    fn ascii_char_from_digit() {
        let result = AsciiChar::try_from('5');
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, '5');
    }

    #[test]
    fn ascii_char_from_space() {
        let result = AsciiChar::try_from(' ');
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, ' ');
    }

    #[test]
    fn ascii_char_from_newline() {
        let result = AsciiChar::try_from('\n');
        assert!(result.is_ok());
    }

    #[test]
    fn ascii_char_from_special() {
        let result = AsciiChar::try_from('@');
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, '@');
    }

    #[test]
    fn ascii_char_from_null() {
        let result = AsciiChar::try_from('\0');
        assert!(result.is_ok());
    }

    #[test]
    fn ascii_char_non_ascii_fails() {
        let result = AsciiChar::try_from('ñ');
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "character must be ASCII");
    }

    #[test]
    fn ascii_char_emoji_fails() {
        let result = AsciiChar::try_from('😀');
        assert!(result.is_err());
    }

    #[test]
    fn ascii_char_chinese_fails() {
        let result = AsciiChar::try_from('中');
        assert!(result.is_err());
    }

    #[test]
    fn ascii_char_cyrillic_fails() {
        let result = AsciiChar::try_from('Д');
        assert!(result.is_err());
    }

    #[test]
    fn ascii_char_from_byte() {
        let result = AsciiChar::try_from(65u8);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 'A');
    }

    #[test]
    fn ascii_char_from_byte_zero() {
        let result = AsciiChar::try_from(0u8);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, '\0');
    }

    #[test]
    fn ascii_char_from_byte_max_ascii() {
        let result = AsciiChar::try_from(127u8);
        assert!(result.is_ok());
    }

    #[test]
    fn ascii_char_from_byte_extended() {
        // u8 values 0-127 are ASCII, 128-255 are extended ASCII (not ASCII)
        let result = AsciiChar::try_from(128u8);
        assert!(result.is_err());
    }

    #[test]
    fn ascii_char_from_byte_255() {
        let result = AsciiChar::try_from(255u8);
        assert!(result.is_err());
    }

    #[test]
    fn ascii_char_try_into_from_char() {
        let result: Result<AsciiChar, _> = 'X'.try_into();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 'X');
    }

    #[test]
    fn ascii_char_try_into_from_byte() {
        let result: Result<AsciiChar, _> = 88u8.try_into();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 'X');
    }

    // ============================================================================
    // Integration tests
    // ============================================================================

    #[test]
    fn integration_chained_conversions() {
        // Convert i32 to PositiveNumber, then access inner value
        let pos = PositiveNumber::try_from(10).unwrap();
        let even = EvenNumber::try_from(pos.0);
        assert!(even.is_ok());
        assert_eq!(even.unwrap().0, 10);
    }

    #[test]
    fn integration_error_handling() {
        fn validate_percentage(value: i32) -> Result<Percentage, &'static str> {
            Percentage::try_from(value)
        }

        assert!(validate_percentage(50).is_ok());
        assert!(validate_percentage(-10).is_err());
        assert!(validate_percentage(200).is_err());
    }

    #[test]
    fn integration_multiple_types() {
        // All succeed
        let pos = PositiveNumber::try_from(42);
        let pct = Percentage::try_from(42);
        let even = EvenNumber::try_from(42);

        assert!(pos.is_ok());
        assert!(pct.is_ok());
        assert!(even.is_ok());

        assert_eq!(pos.unwrap().0, 42);
        assert_eq!(pct.unwrap().0, 42);
        assert_eq!(even.unwrap().0, 42);
    }

    #[test]
    fn integration_conflicting_constraints() {
        // 0 is even but not positive
        let even = EvenNumber::try_from(0);
        let pos = PositiveNumber::try_from(0);

        assert!(even.is_ok());
        assert!(pos.is_err());
    }

    #[test]
    fn integration_non_empty_string_preserves_content() {
        let original = "Hello, World! 🌍";
        let nes = NonEmptyString::try_from(original).unwrap();
        assert_eq!(nes.0, original);
    }

    #[test]
    fn integration_ascii_byte_to_char_consistency() {
        // Verify byte and char conversions are consistent
        for byte in 0u8..=127 {
            let from_byte = AsciiChar::try_from(byte).unwrap();
            let from_char = AsciiChar::try_from(char::from(byte)).unwrap();
            assert_eq!(from_byte.0, from_char.0);
        }
    }

    #[test]
    fn integration_result_combinators() {
        let result = PositiveNumber::try_from(5)
            .map(|p| p.0 * 2)
            .and_then(|n| EvenNumber::try_from(n));

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 10);
    }

    #[test]
    fn integration_collecting_results() {
        let values = vec![10, 20, 30, 40];
        let percentages: Result<Vec<Percentage>, _> =
            values.into_iter().map(Percentage::try_from).collect();

        assert!(percentages.is_ok());
        let pcts = percentages.unwrap();
        assert_eq!(pcts.len(), 4);
        assert_eq!(pcts[0].0, 10);
        assert_eq!(pcts[3].0, 40);
    }

    #[test]
    fn integration_collecting_results_with_error() {
        let values = vec![10, 20, 150, 40]; // 150 is invalid
        let percentages: Result<Vec<Percentage>, _> =
            values.into_iter().map(Percentage::try_from).collect();

        assert!(percentages.is_err());
    }
}
