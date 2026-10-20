//! # Rustfinity: Number Formatting
//!
//! Challenge: https://github.com/rustfinity/rustfinity/tree/main/challenges/number-formatting
//!
//! Implement these formatting functions:
//!
//! - `format_padded`: zero-pad an `i32` to a minimum width; keep a negative sign before the zeros.
//! - `format_aligned`: right-align an `i32` in a minimum-width field, padding with spaces.
//! - `format_binary` and `format_binary_prefixed`: binary output without and with `0b`.
//! - `format_hex_lower` and `format_hex_upper_prefixed`: lowercase hex without a prefix and
//!   uppercase hex with a `0x` prefix.
//! - `format_octal`: octal output without a prefix.
//! - `format_float_precision`: fixed decimal output with exactly the requested precision.
//! - `format_scientific`: lowercase scientific notation.
//! - `format_currency`: dollars with two decimal places and a leading minus sign for negatives.
//!
//! Run: `cargo run --example rf_number_formatting`
//! Test this challenge: `cargo test --example rf_number_formatting`
//! Filter its tests: `cargo test --example rf_number_formatting format_padded`

#![allow(unused)]

/// Formats an integer with leading zeros to the specified width.
///
/// The number is padded with leading zeros to reach the specified width.
/// If the number is wider than the specified width, it is not truncated.
/// Negative numbers include the minus sign before the zeros.
pub fn format_padded(num: i32, width: usize) -> String {
    let mut result: String = num.to_string();
    // len() returns the number of bytes, not necessarily the number of characters.
    // For plain ASCII text, byte count and character count are the same.
    // For Unicode text, they may differ.
    let len = result.len();
    // usize - usize is negative, cannot use len - width that overflow
    // width.saturating_sub(len) should be ok, when negative it will become zero
    if len >= width {
        return result;
    }
    // rust no trinary expression
    // result.starts_with('-') as usize;
    let insert_index = if num.is_negative() { 1 } else { 0 };
    // insert_str not return result, 2nd argument is &str
    // as_str() from String to &str
    result.insert_str(insert_index, "0".repeat(width - len).as_str());

    result
}

/// Formats a number with right alignment in a field of the given width.
///
/// The number is right-aligned and padded with spaces on the left.
/// If the number is wider than the specified width, it is not truncated.
pub fn format_aligned(num: i32, width: usize) -> String {
    let mut result: String = num.to_string();
    let len = result.len();
    if len >= width {
        return result;
    }
    let padding_spaces = " ".repeat(width - len);
    result.insert_str(0, &padding_spaces);

    result
}

// String cannot throw Err, but Result can, first is Ok result type, second is Err type
pub fn format_base_less_than_ten(num: u32, base: u32) -> Result<String, &'static str> {
    if base == 0 {
        // Err need return
        return Err("base must not be zero");
    }

    // TODO: Reject base 1; this algorithm would not reduce the quotient and can loop forever.
    // if return in if, then better use if instead of else if for idiomatic guard clauses
    if base >= 10 {
        return Err("base must be less than ten");
    }
    if num == 0 {
        return Ok(String::from("0"));
    } else if num < base {
        return Ok(num.to_string());
    }
    let mut q = num / base;
    let mut r = num % base;
    let mut result = String::new();
    while q > 0 {
        // TODO: Append digits in reverse order, then reverse once to avoid shifting each iteration.
        result.insert_str(0, &(r.to_string()));
        // need to get remainder 1st
        r = q % base;
        q /= base;
    }

    if base == 2 {
        // the last remainder always 1
        result.insert_str(0, "1");
    } else {
        result.insert_str(0, &(r.to_string()));
    }

    Ok(result)
}

/// Formats an unsigned integer as binary without prefix.
///
/// Returns the binary representation as a string without the "0b" prefix.
pub fn format_binary(num: u32) -> String {
    match format_base_less_than_ten(num, 2) {
        Ok(result) => result,
        // TODO: Avoid hiding conversion errors by returning an empty string.
        Err(err_message) => String::from(""),
    }
}

/// Formats an unsigned integer as binary with "0b" prefix.
///
/// Returns the binary representation as a string with the "0b" prefix.
pub fn format_binary_prefixed(num: u32) -> String {
    // String + &String to String + &str
    String::from("0b") + &format_binary(num)
}

fn decimal_to_hex_lower(num: u32) -> String {
    match num {
        0..=9 => num.to_string(),
        10 => String::from("a"),
        11 => String::from("b"),
        12 => String::from("c"),
        13 => String::from("d"),
        14 => String::from("e"),
        15 => String::from("f"),
        // TODO: Make the valid digit range explicit instead of silently returning an empty string.
        16.. => String::new(),
    }
}

/// Formats an unsigned integer as lowercase hexadecimal without prefix.
///
/// Returns the hexadecimal representation as a string without the "0x" prefix.
/// Letters a-f are lowercase.
pub fn format_hex_lower(num: u32) -> String {
    const HEXADECIMAL_LENGTH: u32 = 16;

    if num == 0 {
        return String::from("0");
    }

    let mut q = num / HEXADECIMAL_LENGTH;
    let mut r = num % HEXADECIMAL_LENGTH;
    let mut result = String::new();

    while q > 0 {
        // TODO: Append digits in reverse order, then reverse once to avoid shifting each iteration.
        result.insert_str(0, &decimal_to_hex_lower(r));
        // need to get remainder 1st
        r = q % HEXADECIMAL_LENGTH;
        q /= HEXADECIMAL_LENGTH;
    }

    result.insert_str(0, &decimal_to_hex_lower(r));

    result
}

fn decimal_to_hex_upper(num: u32) -> String {
    match num {
        0..=9 => num.to_string(),
        10 => String::from("A"),
        11 => String::from("B"),
        12 => String::from("C"),
        13 => String::from("D"),
        14 => String::from("E"),
        15 => String::from("F"),
        // TODO: Make the valid digit range explicit instead of silently returning an empty string.
        16.. => String::new(),
    }
}

/// Formats an unsigned integer as uppercase hexadecimal with "0x" prefix.
///
/// Returns the hexadecimal representation as a string with the "0x" prefix.
/// Letters A-F are uppercase.
pub fn format_hex_upper_prefixed(num: u32) -> String {
    const HEXADECIMAL_LENGTH: u32 = 16;

    if num == 0 {
        return String::from("0x0");
    }

    let mut q = num / HEXADECIMAL_LENGTH;
    let mut r = num % HEXADECIMAL_LENGTH;
    let mut result = String::new();

    while q > 0 {
        // TODO: Append digits in reverse order, then reverse once to avoid shifting each iteration.
        result.insert_str(0, &decimal_to_hex_upper(r));
        // need to get remainder 1st
        r = q % HEXADECIMAL_LENGTH;
        q /= HEXADECIMAL_LENGTH;
    }

    result.insert_str(0, &decimal_to_hex_upper(r));

    String::from("0x") + &result
}

/// Formats an unsigned integer as octal without a prefix.
pub fn format_octal(num: u32) -> String {
    // Learn: the `o` format specifier selects base 8.
    match format_base_less_than_ten(num, 8) {
        Ok(result) => result,
        // TODO: Avoid hiding conversion errors by returning an empty string.
        Err(err_message) => String::from(""),
    }
}

/// Formats a floating-point number with exactly the requested number of decimal places.
pub fn format_float_precision(num: f64, precision: usize) -> String {
    // Learn: precision controls digits after the decimal point and rounds when necessary.
    // round up to certain decimal places, precision can be zero, padding with zeroes if not enough precision
    // precision is number of digits after decimal point, decimal point is always .?
    if !num.is_finite() {
        return num.to_string();
    }

    let num_string = num.to_string();
    match num_string.split_once('.') {
        Some((integer_string, decimal_places_string)) => {
            let decimal_places_len = decimal_places_string.len();
            if decimal_places_len == precision {
                return num_string;
            } else if decimal_places_len < precision {
                let mut filled = num_string.to_owned();
                // generate padding 0 string with .repeat()
                filled.push_str(&"0".repeat(precision - decimal_places_len));

                return filled;
            } else {
                // if decimal_places_len > precision
                let mut decimal_places_chars: Vec<char> = decimal_places_string.chars().collect();
                // make vec of char with specified size with .truncate()
                decimal_places_chars.truncate(precision + 1);
                let last_removed_char: char = decimal_places_chars.pop().unwrap();

                // // &str to String with .to_owned()
                // let mut might_need_plus_one_string = integer_string.to_owned();
                // // String + char with .push()
                // might_need_plus_one_string.push('.');
                // // String + &str with .push_str()
                // might_need_plus_one_string
                //     // collect need turbofish if Rust does not know target data type
                //     .push_str(&decimal_places_chars.iter().collect::<String>());

                // the suffix dot still needed to add last decimal places 3. + 7 becomes 3.7
                let mut might_need_plus_one_string = integer_string.to_owned()
                    + "."
                    + &decimal_places_chars.iter().collect::<String>();

                let last_is_at_least_five = last_removed_char
                    // char to u32 with .to_digit(10)
                    .to_digit(10)
                    // if Some then proceed checking with .is_some_and()
                    .is_some_and(|digit| digit >= 5);
                if last_is_at_least_five {
                    // add 1 to right most number
                    let factor = 10_f64.powi(precision as i32);
                    // .round() to round up
                    might_need_plus_one_string.push(last_removed_char);
                    // TODO: Round the kept decimal digits with a carry instead of parsing and rounding through f64.
                    let might_need_zero_padding =
                        ((might_need_plus_one_string.parse::<f64>().unwrap() * factor).round()
                            / factor);
                    // 2 => 2.00, 1.9 => 1.90
                    format_float_precision(might_need_zero_padding, precision)
                } else {
                    // The last character is a dot.
                    // 3.1 => 3 not 3.
                    if might_need_plus_one_string.ends_with('.') {
                        might_need_plus_one_string.pop();
                    }

                    return might_need_plus_one_string;
                }
            }
        }
        None => {
            if precision == 0 {
                return num_string;
            } else {
                return num_string + "." + &("0".repeat(precision));
            }
        }
    }
}

/// Formats a floating-point number in lowercase scientific notation.
pub fn format_scientific(num: f64) -> String {
    // Learn: scientific notation expresses a value as a coefficient times a power of ten.
    // TODO: format using scientific notation.
    todo!("format {num} in scientific notation")
}

/// Formats a dollar amount with two decimal places and a leading minus sign when negative.
pub fn format_currency(amount: f64) -> String {
    // Learn: place the sign before `$`; normalize the magnitude so -0.0 displays as $0.00.
    // TODO: format the amount as currency.
    todo!("format {amount} as currency")
}

// Example usage. The starter functions must be implemented before running this example.
fn main() {
    // Padding with zeros
    println!("format_padded(42, 5) = {}", format_padded(42, 5));

    // Right alignment
    println!("format_aligned(42, 5) = '{}'", format_aligned(42, 5));

    // Binary formatting
    println!("format_binary(10) = {}", format_binary(10));
    println!(
        "format_binary_prefixed(10) = {}",
        format_binary_prefixed(10)
    );

    // Hexadecimal formatting
    println!("format_hex_lower(255) = {}", format_hex_lower(255));
    println!(
        "format_hex_upper_prefixed(255) = {}",
        format_hex_upper_prefixed(255)
    );

    // Octal formatting
    println!("format_octal(8) = {}", format_octal(8));

    // Float precision
    println!(
        "format_float_precision(3.14159, 2) = {}",
        format_float_precision(3.14159, 2)
    );

    // Scientific notation
    println!("format_scientific(1234.5) = {}", format_scientific(1234.5));

    // Currency
    println!("format_currency(19.99) = {}", format_currency(19.99));
    println!("format_currency(-5.5) = {}", format_currency(-5.5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_padded_cases() {
        assert_eq!(format_padded(42, 5), "00042");
        assert_eq!(format_padded(-7, 4), "-007");
        assert_eq!(format_padded(12345, 3), "12345");
        assert_eq!(format_padded(0, 3), "000");
        assert_eq!(format_padded(42, 0), "42");
    }

    #[test]
    fn format_aligned_cases() {
        assert_eq!(format_aligned(42, 5), "   42");
        assert_eq!(format_aligned(-7, 5), "   -7");
        assert_eq!(format_aligned(12345, 3), "12345");
        assert_eq!(format_aligned(0, 3), "  0");
        assert_eq!(format_aligned(42, 0), "42");
    }

    #[test]
    fn format_binary_cases() {
        assert_eq!(format_binary(0), "0");
        assert_eq!(format_binary(1), "1");
        assert_eq!(format_binary(10), "1010");
        assert_eq!(format_binary(255), "11111111");
        assert_eq!(format_binary(256), "100000000");
    }

    #[test]
    fn format_binary_prefixed_cases() {
        assert_eq!(format_binary_prefixed(0), "0b0");
        assert_eq!(format_binary_prefixed(10), "0b1010");
        assert_eq!(format_binary_prefixed(255), "0b11111111");
    }

    #[test]
    fn format_hex_lower_cases() {
        assert_eq!(format_hex_lower(0), "0");
        assert_eq!(format_hex_lower(16), "10");
        assert_eq!(format_hex_lower(171), "ab");
        assert_eq!(format_hex_lower(3_735_928_559), "deadbeef");
    }

    #[test]
    fn format_hex_upper_prefixed_cases() {
        assert_eq!(format_hex_upper_prefixed(0), "0x0");
        assert_eq!(format_hex_upper_prefixed(255), "0xFF");
        assert_eq!(format_hex_upper_prefixed(171), "0xAB");
        assert_eq!(format_hex_upper_prefixed(3_735_928_559), "0xDEADBEEF");
    }

    #[test]
    fn format_octal_cases() {
        assert_eq!(format_octal(0), "0");
        assert_eq!(format_octal(8), "10");
        assert_eq!(format_octal(63), "77");
        assert_eq!(format_octal(493), "755");
        assert_eq!(format_octal(420), "644");
    }

    #[test]
    fn format_float_precision_cases() {
        assert_eq!(format_float_precision(3.14159, 2), "3.14");
        assert_eq!(format_float_precision(2.5, 4), "2.5000");
        assert_eq!(format_float_precision(-1.5, 1), "-1.5");
        assert_eq!(format_float_precision(0.0, 3), "0.000");
        assert_eq!(format_float_precision(1.999, 2), "2.00");
        assert_eq!(format_float_precision(1.899, 2), "1.90");
        assert_eq!(format_float_precision(3.7, 0), "4");
        assert_eq!(format_float_precision(3.1, 0), "3");

        // Carry across multiple 9s.
        assert_eq!(format_float_precision(9.999, 2), "10.00");
        assert_eq!(format_float_precision(-1.999, 2), "-2.00");

        // Precision zero.
        assert_eq!(format_float_precision(3.4, 0), "3");
        assert_eq!(format_float_precision(-3.5, 0), "-4");

        // Keep trailing zeros after rounding.
        assert_eq!(format_float_precision(1.995, 2), "2.00");

        // Small value rounding up.
        assert_eq!(format_float_precision(0.0009, 3), "0.001");

        // Non-finite values.
        assert_eq!(format_float_precision(f64::INFINITY, 2), "inf");
        assert_eq!(format_float_precision(f64::NEG_INFINITY, 2), "-inf");
        assert_eq!(format_float_precision(f64::NAN, 2), "NaN");
    }

    #[test]
    fn format_scientific_cases() {
        assert_eq!(format_scientific(1234.5), "1.2345e3");
        assert_eq!(format_scientific(0.00123), "1.23e-3");
        assert_eq!(format_scientific(1.0), "1e0");
        assert_eq!(format_scientific(-5000.0), "-5e3");
        assert_eq!(format_scientific(0.0), "0e0");
    }

    #[test]
    fn format_currency_cases() {
        assert_eq!(format_currency(19.99), "$19.99");
        assert_eq!(format_currency(-5.5), "-$5.50");
        assert_eq!(format_currency(1000.0), "$1000.00");
        assert_eq!(format_currency(9.999), "$10.00");
        assert_eq!(format_currency(-0.99), "-$0.99");
        assert_eq!(format_currency(-0.0), "$0.00");
    }

    #[test]
    fn integration_format_table_columns() {
        let values = [1, 23, 456, 7890];
        let formatted: Vec<String> = values
            .iter()
            .map(|&value| format_aligned(value, 6))
            .collect();
        assert_eq!(formatted, ["     1", "    23", "   456", "  7890"]);
    }

    #[test]
    fn integration_format_same_integer_in_multiple_bases() {
        assert_eq!(format_binary(42), "101010");
        assert_eq!(format_hex_lower(42), "2a");
        assert_eq!(format_octal(42), "52");
    }

    #[test]
    fn integration_format_currency_total() {
        let total = 29.99 * (1.0 + 0.08);
        assert_eq!(format_currency(total), "$32.39");
    }
}
