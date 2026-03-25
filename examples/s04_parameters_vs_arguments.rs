// Clarifies the difference between parameters and arguments in Rust.
//
// In casual conversation, "parameter" and "argument" are often used
// interchangeably. Rust's official docs make a distinction:
//   - **Parameters** are the names in the function *definition* (formal).
//   - **Arguments** are the values passed when *calling* the function (actual).
//
// Rust enforces this strictly because every parameter must have
// a type annotation.
//
// Run: cargo run --example s04_parameters_vs_arguments

/// Add two integers.
///
/// `a` and `b` are **parameters** (defined in the signature).
/// The caller provides **arguments** (concrete values).
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Greet someone by name.
///
/// `name: &str` is a parameter. The `&str` type annotation is mandatory.
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Demonstrates that Rust passes by value (move or copy) by default.
///
/// For `Copy` types like `i32`, the argument is copied.
/// For non-`Copy` types like `String`, the argument is *moved*.
fn take_ownership(s: String) -> usize {
    s.len()
}

fn main() {
    // --- 1. Parameters vs arguments ---
    // In the call below, `3` and `7` are **arguments**.
    // They bind to `a` and `b` (the **parameters**) in `add`.
    let sum = add(3, 7); // 3, 7 = arguments
    println!("add(3, 7) = {sum}");

    // --- 2. Type annotations are mandatory ---
    // Rust requires explicit parameter types.
    // `fn add(a, b)` would be a compile error.
    let msg = greet("Rustacean");
    println!("{msg}");

    // --- 3. Pass by value ---
    // Rust doesn't have "pass by reference" by default. Everything is
    // passed by value. For `Copy` types, this means a bitwise copy.
    // For move types like `String`, the caller *loses* ownership.
    let name = String::from("Alice");
    let len = take_ownership(name);
    // `name` is now moved; using it here would be a compile error:
    //   println!("{name}"); // error: value used after move
    println!("length = {len}");

    // --- 4. To keep ownership, pass a reference ---
    // Use `&` to borrow instead of moving (covered in Section 7).
    let name2 = String::from("Bob");
    let len2 = name2.len(); // .len() borrows &self automatically
    println!("{name2} has {len2} chars");

    // --- 5. Expressions as arguments ---
    // Arguments can be any expression, not just variables.
    println!("add(1+2, 3*4) = {}", add(1 + 2, 3 * 4));

    // --- 6. Argument order matters ---
    // Rust doesn't have keyword arguments.
    // Arguments are always positional.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(100, 200), 300);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn test_take_ownership() {
        let s = String::from("hello");
        assert_eq!(take_ownership(s), 5);
    }

    #[test]
    fn test_expression_arguments() {
        assert_eq!(add(2 + 3, 4 * 5), 25);
    }
}
