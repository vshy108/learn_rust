// Demonstrates assert_eq! macro in tests.
//
// `assert_eq!(left, right)` panics if left != right,
// printing both values on failure (requires Debug trait).
//
// Run: cargo test --example s22_assert_eq

/// Calculate the average of a slice.
fn average(nums: &[f64]) -> f64 {
    if nums.is_empty() {
        return 0.0;
    }
    nums.iter().sum::<f64>() / nums.len() as f64
}

/// Return the longer of two strings.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    println!("average of [1,2,3]: {}", average(&[1.0, 2.0, 3.0]));
    println!("longer(\"hello\", \"hi\"): {}", longer("hello", "hi"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        assert_eq!(average(&[1.0, 2.0, 3.0]), 2.0);
    }

    #[test]
    fn test_average_empty() {
        assert_eq!(average(&[]), 0.0);
    }

    #[test]
    fn test_average_single() {
        assert_eq!(average(&[42.0]), 42.0);
    }

    #[test]
    fn test_longer() {
        assert_eq!(longer("hello", "hi"), "hello");
        assert_eq!(longer("a", "bb"), "bb");
    }

    #[test]
    fn test_longer_equal() {
        // When equal length, returns the first
        assert_eq!(longer("ab", "cd"), "ab");
    }
}
