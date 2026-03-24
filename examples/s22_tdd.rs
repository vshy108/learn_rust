// Demonstrates Test-Driven Development (TDD) in Rust.
//
// TDD cycle:
// 1. Write a failing test (Red)
// 2. Write minimal code to pass (Green)
// 3. Refactor while keeping tests green (Refactor)
//
// Example: Build a FizzBuzz implementation using TDD.
//
// Run: cargo test --example s22_tdd

/// Convert a number to its FizzBuzz string.
///
/// # Arguments
///
/// * `n` - The number to convert
///
/// # Returns
///
/// - "FizzBuzz" if divisible by 3 and 5
/// - "Fizz" if divisible by 3
/// - "Buzz" if divisible by 5
/// - The number as a string otherwise
///
/// # Examples
///
/// ```
/// fn fizzbuzz(n: u32) -> String {
///     match (n % 3, n % 5) {
///         (0, 0) => "FizzBuzz".to_string(),
///         (0, _) => "Fizz".to_string(),
///         (_, 0) => "Buzz".to_string(),
///         _ => n.to_string(),
///     }
/// }
/// assert_eq!(fizzbuzz(15), "FizzBuzz");
/// ```
fn fizzbuzz(n: u32) -> String {
    // Step 2 (Green): minimal implementation
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

/// Generate FizzBuzz for a range.
fn fizzbuzz_range(start: u32, end: u32) -> Vec<String> {
    (start..=end).map(fizzbuzz).collect()
}

fn main() {
    println!("FizzBuzz 1-20:");
    for s in fizzbuzz_range(1, 20) {
        print!("{s} ");
    }
    println!();
}

// Step 1 (Red): Write tests first, then implement
#[cfg(test)]
mod tests {
    use super::*;

    // TDD Step 1: Regular numbers
    #[test]
    fn test_regular_numbers() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(2), "2");
        assert_eq!(fizzbuzz(4), "4");
    }

    // TDD Step 2: Fizz (multiples of 3)
    #[test]
    fn test_fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(6), "Fizz");
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    // TDD Step 3: Buzz (multiples of 5)
    #[test]
    fn test_buzz() {
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(20), "Buzz");
    }

    // TDD Step 4: FizzBuzz (multiples of both)
    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(30), "FizzBuzz");
        assert_eq!(fizzbuzz(45), "FizzBuzz");
    }

    // TDD Step 5: Range
    #[test]
    fn test_range() {
        let result = fizzbuzz_range(1, 5);
        assert_eq!(result, vec!["1", "2", "Fizz", "4", "Buzz"]);
    }
}
