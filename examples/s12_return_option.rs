// Demonstrates returning Option<T> from a function.
//
// Use Option as a return type when a function might not have a result.
// This is Rust's idiomatic way to express "nullable" return values.
//
// Run: cargo run --example s12_return_option

/// Finds the first even number in a slice.
///
/// # Arguments
///
/// * `nums` - A slice of i32
///
/// # Returns
///
/// Some(first_even) or None if no even number exists
fn first_even(nums: &[i32]) -> Option<i32> {
    nums.iter().find(|&&n| n % 2 == 0).copied()
}

/// Parses a string as a positive integer.
///
/// # Arguments
///
/// * `s` - The string to parse
///
/// # Returns
///
/// Some(n) if it's a valid positive integer, None otherwise
fn parse_positive(s: &str) -> Option<u32> {
    let n: u32 = s.parse().ok()?; // ? on Option: returns None if Err
    if n > 0 { Some(n) } else { None }
}

/// Finds the median of a sorted slice.
///
/// # Arguments
///
/// * `sorted` - A sorted slice of f64
///
/// # Returns
///
/// Some(median) or None if empty
fn median(sorted: &[f64]) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
}

fn main() {
    // --- 1. first_even ---
    println!("first_even([1,3,5,4]): {:?}", first_even(&[1, 3, 5, 4]));
    println!("first_even([1,3,5]):   {:?}", first_even(&[1, 3, 5]));

    // --- 2. parse_positive ---
    println!("parse_positive(\"42\"): {:?}", parse_positive("42"));
    println!("parse_positive(\"0\"):  {:?}", parse_positive("0"));
    println!("parse_positive(\"abc\"): {:?}", parse_positive("abc"));

    // --- 3. median ---
    println!("median([1,2,3]):   {:?}", median(&[1.0, 2.0, 3.0]));
    println!("median([1,2,3,4]): {:?}", median(&[1.0, 2.0, 3.0, 4.0]));
    println!("median([]):        {:?}", median(&[]));

    // --- 4. Chaining Option returns ---
    let input = "7";
    let result = parse_positive(input).map(|n| n * 2).filter(|&n| n < 100);
    println!("chained: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_even_found() {
        assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
    }

    #[test]
    fn test_first_even_none() {
        assert_eq!(first_even(&[1, 3, 5]), None);
    }

    #[test]
    fn test_first_even_empty() {
        assert_eq!(first_even(&[]), None);
    }

    #[test]
    fn test_parse_positive_valid() {
        assert_eq!(parse_positive("42"), Some(42));
    }

    #[test]
    fn test_parse_positive_zero() {
        assert_eq!(parse_positive("0"), None);
    }

    #[test]
    fn test_parse_positive_invalid() {
        assert_eq!(parse_positive("abc"), None);
    }

    #[test]
    fn test_median_odd() {
        assert_eq!(median(&[1.0, 2.0, 3.0]), Some(2.0));
    }

    #[test]
    fn test_median_even() {
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
    }

    #[test]
    fn test_median_empty() {
        assert_eq!(median(&[]), None);
    }
}
