// Demonstrates running a subset of tests.
//
// `cargo test test_name` — run tests matching the name
// `cargo test module::` — run tests in a specific module
//
// Run: cargo test --example s22_subset_tests
// Run: cargo test --example s22_subset_tests -- test_add
// Run: cargo test --example s22_subset_tests -- math
// Run: cargo test --example s22_subset_tests -- string

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

fn main() {
    println!("add(1, 2) = {}", add(1, 2));
    println!("multiply(3, 4) = {}", multiply(3, 4));
    println!("capitalize(\"hello\") = {}", capitalize("hello"));
    println!();
    println!("Try running subsets:");
    println!("  cargo test --example s22_subset_tests -- test_add");
    println!("  cargo test --example s22_subset_tests -- math");
    println!("  cargo test --example s22_subset_tests -- string");
}

#[cfg(test)]
mod math_tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -2), -3);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
    }
}

#[cfg(test)]
mod string_tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
    }

    #[test]
    fn test_capitalize_empty() {
        assert_eq!(capitalize(""), "");
    }
}
