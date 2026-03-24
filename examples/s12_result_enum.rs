// Demonstrates the Result<T, E> enum.
//
// Result is Rust's standard error handling type:
//
//   enum Result<T, E> {
//       Ok(T),   // success with value of type T
//       Err(E),  // error with value of type E
//   }
//
// Unlike exceptions in Python, errors in Rust are values you return.
// The compiler ensures you handle them.
//
// Run: cargo run --example s12_result_enum

/// Divides two integers, returning an error for division by zero.
///
/// # Arguments
///
/// * `a` - Numerator
/// * `b` - Denominator
///
/// # Returns
///
/// Ok(quotient) or Err(message)
///
/// # Errors
///
/// Returns an error string when `b` is zero
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    // --- 1. Using Result ---
    let ok = divide(10, 3);
    let err = divide(10, 0);
    println!("10/3 = {ok:?}");
    println!("10/0 = {err:?}");

    // --- 2. Pattern matching ---
    match divide(20, 4) {
        Ok(result) => println!("Result: {result}"),
        Err(msg) => println!("Error: {msg}"),
    }

    // --- 3. is_ok / is_err ---
    println!("ok.is_ok(): {}", ok.is_ok());
    println!("err.is_err(): {}", err.is_err());

    // --- 4. unwrap and expect on Result ---
    let val = divide(100, 5).unwrap();
    println!("unwrap: {val}");
    let val = divide(100, 5).expect("division should work");
    println!("expect: {val}");

    // --- 5. Result with different error types ---
    fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n: i32 = s.parse()?; // ? propagates the error
        Ok(n * 2)
    }
    println!("parse_and_double(\"21\"): {:?}", parse_and_double("21"));
    println!("parse_and_double(\"abc\"): {:?}", parse_and_double("abc"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_ok() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_zero() {
        assert_eq!(divide(10, 0), Err("division by zero".to_string()));
    }

    #[test]
    fn test_is_ok() {
        assert!(divide(1, 1).is_ok());
        assert!(divide(1, 0).is_err());
    }
}
