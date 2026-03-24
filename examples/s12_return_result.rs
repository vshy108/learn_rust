// Demonstrates returning Result from a function.
//
// The `?` operator is key: it propagates errors automatically.
// `expr?` means "if Err, return the error; if Ok, unwrap the value."
//
// Run: cargo run --example s12_return_result

use std::num::ParseIntError;

/// Parses a string as i32, doubles it, and converts to String.
///
/// # Arguments
///
/// * `s` - A string containing an integer
///
/// # Returns
///
/// Ok with the doubled value as String, or Err with parse error
///
/// # Errors
///
/// Returns `ParseIntError` if the input is not a valid integer
fn double_parse(s: &str) -> Result<String, ParseIntError> {
    let n: i32 = s.parse()?; // ? returns early on Err
    Ok(format!("{}", n * 2))
}

/// Calculates the sum of string numbers.
///
/// # Arguments
///
/// * `nums` - Slice of string slices containing integers
///
/// # Returns
///
/// Ok(sum) or Err on first parse failure
///
/// # Errors
///
/// Returns `ParseIntError` if any string is not a valid integer
fn sum_strings(nums: &[&str]) -> Result<i32, ParseIntError> {
    let mut total: i32 = 0;
    for s in nums {
        let n: i32 = s.parse()?;
        total += n;
    }
    Ok(total)
}

/// Alternative: collect Results into a single Result.
fn sum_strings_iter(nums: &[&str]) -> Result<i32, ParseIntError> {
    let parsed: Result<Vec<i32>, _> = nums.iter().map(|s| s.parse::<i32>()).collect();
    Ok(parsed?.iter().sum())
}

fn main() {
    // --- 1. Basic usage ---
    println!("double_parse(\"21\"): {:?}", double_parse("21"));
    println!("double_parse(\"abc\"): {:?}", double_parse("abc"));

    // --- 2. Chaining with ? ---
    println!("sum_strings: {:?}", sum_strings(&["10", "20", "30"]));
    println!("sum_strings: {:?}", sum_strings(&["10", "oops", "30"]));

    // --- 3. Iterator version ---
    println!("sum_iter: {:?}", sum_strings_iter(&["1", "2", "3"]));

    // --- 4. Handling the result at the call site ---
    match double_parse("42") {
        Ok(s) => println!("Success: {s}"),
        Err(e) => println!("Failed: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_parse_ok() {
        assert_eq!(double_parse("21"), Ok("42".to_string()));
    }

    #[test]
    fn test_double_parse_err() {
        assert!(double_parse("abc").is_err());
    }

    #[test]
    fn test_sum_strings_ok() {
        assert_eq!(sum_strings(&["1", "2", "3"]), Ok(6));
    }

    #[test]
    fn test_sum_strings_err() {
        assert!(sum_strings(&["1", "bad", "3"]).is_err());
    }

    #[test]
    fn test_sum_strings_iter() {
        assert_eq!(sum_strings_iter(&["10", "20"]), Ok(30));
    }
}
