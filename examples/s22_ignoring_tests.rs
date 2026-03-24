// Demonstrates ignoring tests with #[ignore].
//
// `#[ignore]` skips a test by default.
// Run ignored tests with: `cargo test -- --ignored`
// Run ALL tests: `cargo test -- --include-ignored`
//
// Run: cargo test --example s22_ignoring_tests
// Run ignored: cargo test --example s22_ignoring_tests -- --ignored

fn fast_computation() -> i32 {
    (1..=100).sum()
}

fn slow_computation() -> i32 {
    // Simulate expensive work
    (1..=1000).sum()
}

fn main() {
    println!("fast: {}", fast_computation());
    println!("slow: {}", slow_computation());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast() {
        assert_eq!(fast_computation(), 5050);
    }

    #[test]
    #[ignore = "slow test, run with --ignored"]
    fn test_slow() {
        assert_eq!(slow_computation(), 500_500);
    }

    #[test]
    #[ignore = "requires network"]
    fn test_needs_network() {
        // Would connect to an API in real code
        assert!(true);
    }
}
