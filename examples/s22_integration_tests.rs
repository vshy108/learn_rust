// Demonstrates integration tests.
//
// Integration tests go in a `tests/` directory at the project root.
// They test the public API as an external consumer would.
//
// Since examples can't have separate integration test files,
// this file demonstrates the concept inline.
//
// Run: cargo test --example s22_integration_tests

/// Public API: a calculator module
mod calculator {
    /// Add two numbers.
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Multiply two numbers.
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    /// Evaluate a simple expression: a * b + c
    pub fn evaluate(a: i32, b: i32, c: i32) -> i32 {
        add(multiply(a, b), c)
    }
}

fn main() {
    println!("=== Integration Tests Concept ===");
    println!("2 + 3 = {}", calculator::add(2, 3));
    println!("2 * 3 = {}", calculator::multiply(2, 3));
    println!("2 * 3 + 1 = {}", calculator::evaluate(2, 3, 1));
    println!();
    println!("In a real project, integration tests go in tests/ directory:");
    println!("  tests/");
    println!("    calculator_test.rs");
    println!("    another_test.rs");
    println!();
    println!("They import your crate: `use your_crate::add;`");
    println!("Each file in tests/ is compiled as a separate crate.");
}

// Unit tests (in the same file, test internal details):
#[cfg(test)]
mod unit_tests {
    use super::calculator;

    #[test]
    fn test_add() {
        assert_eq!(calculator::add(1, 2), 3);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(calculator::multiply(3, 4), 12);
    }
}

// Integration-style tests (test the public API as a whole):
#[cfg(test)]
mod integration_tests {
    use super::calculator;

    #[test]
    fn test_evaluate_workflow() {
        // Test a complete workflow through the public API
        let result = calculator::evaluate(2, 3, 1);
        assert_eq!(result, 7); // 2 * 3 + 1
    }

    #[test]
    fn test_complex_calculation() {
        let a = calculator::multiply(2, 3); // 6
        let b = calculator::multiply(4, 5); // 20
        let total = calculator::add(a, b); // 26
        assert_eq!(total, 26);
    }
}
