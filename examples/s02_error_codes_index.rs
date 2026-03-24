// Demonstrates Rust's error codes index and how to navigate it.
//
// Rust maintains a comprehensive index of all compiler error codes at:
//   https://doc.rust-lang.org/error_codes/error-index.html
//
// Error codes follow the pattern E0NNN (e.g., E0384, E0308).
// Each entry explains the error, shows example code, and suggests fixes.
//
// This file shows a few categories of errors and their codes so you
// know what to search for when you encounter them.
//
// Run: cargo run --example s02_error_codes_index

fn main() {
    // --- Error code categories ---
    // Rust error codes are roughly grouped by topic:
    //
    // E0001-E0099:  Syntax and parsing errors
    // E0100-E0199:  Type system errors
    // E0200-E0299:  Trait and impl errors
    // E0300-E0399:  Pattern matching and binding errors
    // E0400-E0499:  Lifetime and borrowing errors
    // E0500-E0599:  Closure and move errors
    // E0600-E0699:  Type inference and coercion errors
    // E0700-E0799:  Async and generator errors
    //
    // Note: these ranges are approximate — Rust adds new codes over time.

    // --- Common errors you'll see as a beginner ---
    println!("Common Rust error codes:");
    println!();

    // Variable and binding errors
    println!("Variables:");
    println!("  E0384 - Cannot assign twice to immutable variable");
    println!("  E0381 - Used binding isn't initialized");
    println!("  E0425 - Cannot find value in this scope");
    println!();

    // Type errors
    println!("Types:");
    println!("  E0308 - Mismatched types");
    println!("  E0277 - Trait bound not satisfied");
    println!("  E0599 - Method not found for type");
    println!();

    // Ownership and borrowing errors
    println!("Ownership:");
    println!("  E0382 - Use of moved value");
    println!("  E0502 - Cannot borrow as mutable (already borrowed)");
    println!("  E0505 - Cannot move out of borrowed content");
    println!("  E0106 - Missing lifetime specifier");
    println!();

    // --- How to look up an error ---
    // Method 1: Terminal
    //   rustc --explain E0384
    //
    // Method 2: Web
    //   https://doc.rust-lang.org/error_codes/E0384.html
    //
    // Method 3: IDE (rust-analyzer shows clickable error codes)
    //
    // Method 4: Search engine
    //   "rust E0384" will usually return the official docs first.

    println!("Look up any error:");
    println!("  rustc --explain E0384");
    println!("  https://doc.rust-lang.org/error_codes/E0384.html");

    // Gotcha: Not all error codes have entries in the index.
    // Some are internal compiler errors or deprecated. If `rustc --explain`
    // says "no extended information", check the Rust GitHub issues.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_error_code_format() {
        // Error codes always follow the pattern E + 4 digits.
        let code = "E0384";
        assert!(code.starts_with('E'));
        assert_eq!(code.len(), 5);
        assert!(code[1..].chars().all(|c| c.is_ascii_digit()));
    }
}
