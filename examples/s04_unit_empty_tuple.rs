// Demonstrates the unit type `()` — Rust's empty tuple.
//
// In Rust, functions that don't explicitly return a value return
// `()` (the unit type). It's a real type with exactly one value: `()`.
//
// `()` is both a type AND a value.
//
// Run: cargo run --example s04_unit_empty_tuple

// We intentionally trigger these lints to demonstrate unit-type behavior.
#![allow(clippy::unused_unit, clippy::let_unit_value, unused_parens)]

/// This function returns `()` implicitly.
/// The return type is omitted, which is sugar for `-> ()`.
fn do_nothing() {
    // No return statement. The function body is a block expression
    // that evaluates to `()`.
}

/// Explicitly annotating the unit return type — identical to `do_nothing`.
fn do_nothing_explicit() -> () {
    // A trailing semicolon turns an expression into a statement,
    // and statements evaluate to `()`.
}

/// A function that prints but doesn't "return" a useful value.
///
/// Even `println!` returns `()`.
fn log_message(msg: &str) {
    println!("[LOG] {msg}");
    // The implicit return is `()`.
}

fn main() {
    // --- 1. Unit type as function return ---
    let result = do_nothing();
    println!("do_nothing() returned: {result:?}"); // => ()

    let result2 = do_nothing_explicit();
    println!("do_nothing_explicit() returned: {result2:?}"); // => ()

    // --- 2. Size of unit ---
    // `()` is a zero-sized type (ZST). It occupies 0 bytes.
    // The compiler optimizes it away entirely.
    println!("size of () = {} bytes", std::mem::size_of::<()>());
    // => 0

    // --- 3. Unit in expressions ---
    // Every block `{}` returns the value of its last expression.
    // If the last expression ends with `;`, the block returns `()`.
    let with_semicolon: () = {
        let _x = 5;
        // semicolon after `_x` makes this a statement -> returns ()
    };
    println!("block with semicolon: {with_semicolon:?}");

    let without_semicolon: i32 = {
        let x = 5;
        x + 1 // no semicolon -> this is the block's return value
    };
    println!("block without semicolon: {without_semicolon}");

    // Gotcha: accidentally adding a semicolon changes the return type!
    // `{ x + 1; }` returns `()`, not `i32`.

    // --- 4. Unit in if-else ---
    // `if` is an expression in Rust. If used as a statement (with `;`),
    // both branches must return `()`.
    let x = 5;
    if x > 3 {
        println!("big");
    } else {
        println!("small");
    }
    // Both branches return `()` — this is fine.

    // --- 5. Unit tuple vs empty tuple ---
    // `()` is called the "unit type" or "empty tuple."
    // A single-element tuple needs a trailing comma: `(42,)`.
    // Without the comma, `(42)` is just parenthesized `42`.
    let unit: () = ();
    let single_element_tuple: (i32,) = (42,);
    let just_parens: i32 = (42);
    println!("unit = {unit:?}");
    println!("single_element_tuple = {single_element_tuple:?}");
    println!("just_parens = {just_parens}");

    // --- 6. Unit in collections (rare but valid) ---
    // `HashSet<()>` or `Vec<()>` are valid. `HashSet<()>` is sometimes
    // used as a set of "present/absent" flags (though `HashSet<K>` is
    // more idiomatic).

    // --- 7. println! returns () ---
    log_message("this function returns unit");
    let print_result = println!("println! also returns unit");
    println!("println! returned: {print_result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_is_zero_sized() {
        assert_eq!(std::mem::size_of::<()>(), 0);
    }

    #[test]
    fn test_do_nothing_returns_unit() {
        assert_eq!(do_nothing(), ());
    }

    #[test]
    fn test_block_with_semicolon_returns_unit() {
        let result: () = {
            let _x = 42;
        };
        assert_eq!(result, ());
    }

    #[test]
    fn test_block_without_semicolon_returns_value() {
        let result: i32 = { 42 };
        assert_eq!(result, 42);
    }

    #[test]
    fn test_single_element_tuple_vs_parens() {
        let tuple: (i32,) = (42,);
        let scalar: i32 = (42);
        assert_eq!(tuple.0, scalar);
    }
}
