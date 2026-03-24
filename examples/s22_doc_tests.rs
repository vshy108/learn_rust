// Demonstrates documentation tests.
//
// Code examples in `///` doc comments are automatically run as tests
// by `cargo test`. This ensures documentation stays correct.
//
// Run: cargo test --example s22_doc_tests

/// Add two numbers.
///
/// # Examples
///
/// ```
/// // In a doc test, you'd write: `use your_crate::add;`
/// // Since this is an example file, we define add locally.
/// fn add(a: i32, b: i32) -> i32 { a + b }
/// assert_eq!(add(2, 3), 5);
/// assert_eq!(add(-1, 1), 0);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Check if a string is empty or whitespace-only.
///
/// # Examples
///
/// ```
/// fn is_blank(s: &str) -> bool { s.trim().is_empty() }
/// assert!(is_blank(""));
/// assert!(is_blank("   "));
/// assert!(!is_blank("hello"));
/// ```
fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

fn main() {
    println!("=== Documentation Tests ===");
    println!("add(2, 3) = {}", add(2, 3));
    println!("is_blank(\"\") = {}", is_blank(""));
    println!();
    println!("Doc tests are written inside /// comments.");
    println!("They run with `cargo test` automatically.");
    println!();
    println!("Useful doc test annotations:");
    println!("  /// ```no_run    — compiles but doesn't run");
    println!("  /// ```ignore    — neither compiles nor runs");
    println!("  /// ```compile_fail — must fail to compile");
    println!("  /// # hidden_line — hidden from docs, still runs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_is_blank() {
        assert!(is_blank(""));
        assert!(is_blank("  "));
        assert!(!is_blank("x"));
    }
}
