// Demonstrates nested functions in Rust.
//
// Functions can be defined inside other functions.
// They can't capture variables from the enclosing scope (unlike closures).
//
// Run: cargo run --example s20_nested_functions

fn main() {
    println!("=== Nested Functions ===");

    // Define a function inside main:
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    // Use them:
    println!("add(3, 4) = {}", add(3, 4));
    println!("multiply(3, 4) = {}", multiply(3, 4));

    // Nested functions can call each other:
    fn compute(x: i32) -> i32 {
        fn square(n: i32) -> i32 {
            n * n
        }
        fn double(n: i32) -> i32 {
            n * 2
        }
        square(x) + double(x)
    }
    println!("compute(5) = {}", compute(5)); // 25 + 10 = 35

    // CANNOT capture enclosing variables:
    let x = 10;
    // fn bad() -> i32 { x + 1 } // ERROR: can't capture `x`
    // For that, use closures:
    let good = || x + 1;
    println!("closure: {}", good());

    println!();
    println!("Nested functions:");
    println!("  - Defined inside other functions");
    println!("  - Cannot capture enclosing variables");
    println!("  - Useful for local helper logic");
    println!("  - For variable capture, use closures instead");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_nested_fn() {
        fn triple(n: i32) -> i32 {
            n * 3
        }
        assert_eq!(triple(4), 12);
    }
}
