// Intro to testing in Rust.
//
// Rust has built-in testing support via `#[test]` attribute and `cargo test`.
// Tests are functions that panic on failure.
// Test functions go inside a `#[cfg(test)]` module.
//
// Run: cargo test --example s22_intro_testing

/// Add two numbers.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiply two numbers.
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    println!("=== Intro to Testing ===");
    println!("add(2, 3) = {}", add(2, 3));
    println!("multiply(4, 5) = {}", multiply(4, 5));
    println!("Run `cargo test --example s22_intro_testing` to see tests.");
}

#[cfg(test)]
mod tests {
    // Import everything from the parent module
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
    }
}
