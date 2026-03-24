// Demonstrates the ? operator in depth.
//
// The ? operator:
// 1. On `Result<T, E>`: returns `Err(e)` if Err, unwraps if Ok.
// 2. On `Option<T>`: returns `None` if None, unwraps if Some.
// 3. Performs `From` conversion on the error type automatically.
//
// Run: cargo run --example s17_try_operator

use std::num::ParseIntError;

/// Parses and doubles a string number.
///
/// # Errors
///
/// Returns `ParseIntError` if parsing fails
fn parse_double(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?; // ? on Result
    Ok(n * 2)
}

/// Chains multiple fallible operations.
///
/// # Errors
///
/// Returns error string on any failure
fn process(input: &str) -> Result<String, String> {
    let n: i32 = input.parse().map_err(|e: ParseIntError| e.to_string())?;
    let doubled = n.checked_mul(2).ok_or("overflow".to_string())?;
    Ok(format!("Result: {doubled}"))
}

/// Demonstrates ? with Option.
fn first_char_upper(s: &str) -> Option<char> {
    let c = s.chars().next()?; // ? on Option: returns None if empty
    Some(c.to_uppercase().next()?)
}

/// Multiple ? in sequence.
///
/// # Errors
///
/// Returns error string on any failure
fn add_strings(a: &str, b: &str) -> Result<i32, String> {
    let x: i32 = a.parse().map_err(|e: ParseIntError| e.to_string())?;
    let y: i32 = b.parse().map_err(|e: ParseIntError| e.to_string())?;
    x.checked_add(y).ok_or_else(|| "overflow".to_string())
}

fn main() {
    // --- 1. ? on Result ---
    println!("parse_double(\"21\"): {:?}", parse_double("21"));
    println!("parse_double(\"abc\"): {:?}", parse_double("abc"));

    // --- 2. ? with error conversion ---
    println!("process(\"10\"): {:?}", process("10"));
    println!("process(\"bad\"): {:?}", process("bad"));

    // --- 3. ? on Option ---
    println!(
        "first_char_upper(\"hello\"): {:?}",
        first_char_upper("hello")
    );
    println!("first_char_upper(\"\"): {:?}", first_char_upper(""));

    // --- 4. Chained ? ---
    println!("add_strings(\"10\", \"20\"): {:?}", add_strings("10", "20"));
    println!("add_strings(\"10\", \"x\"): {:?}", add_strings("10", "x"));

    // --- 5. ? requires compatible return type ---
    // Can only use ? in functions that return Result or Option.
    // main() returns () by default, so ? can't be used there
    // UNLESS main returns Result:
    //   fn main() -> Result<(), Box<dyn Error>> { ... }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_double_ok() {
        assert_eq!(parse_double("21"), Ok(42));
    }

    #[test]
    fn test_parse_double_err() {
        assert!(parse_double("abc").is_err());
    }

    #[test]
    fn test_first_char_upper() {
        assert_eq!(first_char_upper("hello"), Some('H'));
        assert_eq!(first_char_upper(""), None);
    }

    #[test]
    fn test_add_strings() {
        assert_eq!(add_strings("10", "20"), Ok(30));
        assert!(add_strings("x", "20").is_err());
    }
}
