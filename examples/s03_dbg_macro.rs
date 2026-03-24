// Demonstrates the `dbg!` macro in Rust.
//
// `dbg!` prints the file, line number, expression, and value to stderr.
// It then RETURNS the value (unlike `println!` which returns `()`).
// This makes it perfect for quick debugging without restructuring code.
//
// Python equivalent: print(f"{expr=}") (PEP 572-style) or `breakpoint()`.
//
// Run: cargo run --example s03_dbg_macro

fn main() {
    // --- 1. Basic usage ---
    // `dbg!` prints to stderr, not stdout.
    let x = 42;
    dbg!(x);
    // Output: [examples/s03_dbg_macro.rs:14:5] x = 42

    // --- 2. dbg! returns the value ---
    // You can chain it inline without a separate `let`.
    let doubled = dbg!(x * 2);
    println!("doubled = {doubled}");
    // stderr: [examples/s03_dbg_macro.rs:19:20] x * 2 = 84
    // stdout: doubled = 84

    // --- 3. Use in expressions ---
    // Useful for debugging intermediate values in a chain.
    let result = dbg!(dbg!(2) + dbg!(3));
    println!("result = {result}");
    // stderr shows: 2 = 2, 3 = 3, 2 + 3 = 5

    // --- 4. With complex expressions ---
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = dbg!(v.iter().sum());
    println!("sum = {sum}");

    // --- 5. Multiple arguments ---
    let a = 1;
    let b = 2;
    dbg!(a, b, a + b);
    // Prints each on a separate line

    // --- 6. dbg! uses Debug, not Display ---
    // The value must implement the `Debug` trait.
    // `dbg!` uses `{:#?}` (pretty Debug) internally.
    let tuple = (1, "hello", [3, 4, 5]);
    dbg!(&tuple);

    // --- 7. dbg! moves ownership ---
    let s = String::from("hello");
    let s = dbg!(s); // takes ownership and returns it
    // Without reassigning: `dbg!(s)` would consume s
    println!("s is still valid: {s}");

    // --- 8. dbg! with no arguments ---
    dbg!(); // just prints file:line

    // --- Key differences from println! ---
    println!();
    println!("dbg! vs println!:");
    println!("  dbg!:     writes to STDERR, returns value, shows file:line");
    println!("  println!: writes to STDOUT, returns (), no file:line");
    println!();
    println!("  dbg! is for debugging — remove before committing!");
    println!("  println! is for program output — stays in production.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dbg_returns_value() {
        let x = dbg!(42);
        assert_eq!(x, 42);
    }

    #[test]
    fn test_dbg_in_expression() {
        let result = dbg!(2 + 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_dbg_with_string() {
        let s = String::from("test");
        let s = dbg!(s); // takes and returns ownership
        assert_eq!(s, "test");
    }
}
