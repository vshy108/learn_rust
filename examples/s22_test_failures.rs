// Demonstrates test failures.
//
// When assert_eq! fails, it shows both the expected and actual values.
// Tests are independent — one failure doesn't affect others.
//
// Run: cargo test --example s22_test_failures

/// Intentionally buggy function to demonstrate test failure output.
fn buggy_add(a: i32, b: i32) -> i32 {
    a + b // this one is correct
}

/// A function that returns a greeting.
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn main() {
    println!("buggy_add(2, 3) = {}", buggy_add(2, 3));
    println!("greet(\"Rust\") = {}", greet("Rust"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_correct() {
        // This test passes:
        assert_eq!(buggy_add(2, 3), 5);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    // To see a failure, uncomment this:
    // #[test]
    // fn test_add_wrong() {
    //     // This would fail with a nice diff message:
    //     assert_eq!(buggy_add(2, 3), 6, "Expected 6 but got something else");
    // }

    #[test]
    fn test_greet_format() {
        let result = greet("Rust");
        assert!(result.contains("Rust"));
        assert!(result.starts_with("Hello"));
    }
}
