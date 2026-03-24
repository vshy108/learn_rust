//! This module showcases all four Rust comment styles.
//! Inner doc comments like these appear when you run `cargo doc` and
//! navigate to this module's page.

// Demonstrates every comment style Rust supports.
//
// Rust has four kinds of comments:
//   1. Line comments        `//`
//   2. Block comments        `/* ... */`  (nestable!)
//   3. Doc comments (outer)  `///`  — documents the *next* item
//   4. Doc comments (inner)  `//!`  — documents the *enclosing* item
//
// Run: cargo run --example s01_comments
// Doc: cargo doc --document-private-items --open

// ---------------------------------------------------------------------------
// 1. Line comments — the most common kind.
//    Everything after `//` to end-of-line is ignored by the compiler.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 2. Block comments — `/* ... */`
//    Unlike C/C++, Rust block comments **nest**.
//    This lets you comment out code that already contains block comments.
// ---------------------------------------------------------------------------

/*
    This is a block comment.
    /* This nested block comment is perfectly legal in Rust. */
    Still inside the outer block comment.
*/

// ---------------------------------------------------------------------------
// 3. Outer doc comments — `///`
//    Attach to the item that *follows* them.
//    Written in Markdown. `cargo doc` renders them into HTML.
// ---------------------------------------------------------------------------

/// Add two integers and return the result.
///
/// # Arguments
///
/// * `a` - The first operand
/// * `b` - The second operand
///
/// # Examples
///
/// ```
/// // Doc-tests are compiled and run by `cargo test`!
/// assert_eq!(2 + 3, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// A simple point in 2D space.
///
/// Demonstrates that doc comments work on structs (and their fields
/// when those fields are `pub`).
#[derive(Debug, Clone, PartialEq)]
struct Point {
    /// Horizontal coordinate.
    x: f64,
    /// Vertical coordinate.
    y: f64,
}

// ---------------------------------------------------------------------------
// 4. Inner doc comments — `//!`
//    Attach to the *enclosing* item (the module or crate that contains them).
//    Typically placed at the very top of `lib.rs` or at the start of a module.
//    In an example file like this they document the implicit module.
// ---------------------------------------------------------------------------

// (Inner doc comments are shown at the crate/module level; placing them
// inside `main` is not idiomatic, so we demonstrate them at file scope.
// See the `//!` lines at the very top of this file — they must come
// before any items, or the compiler emits E0753.)

fn main() {
    // --- Line comments inside code ---
    let sum = add(3, 7); // inline comment after a statement
    println!("add(3, 7) = {sum}");

    /*
        Block comments can span
        multiple lines inside a function body too.
    */

    // --- Gotcha 1: Doc comments are syntactic sugar for #[doc = "..."] ---
    // `/// text` is equivalent to `#[doc = " text"]`.
    // This means doc comments are actual attributes, not plain comments.
    // Placing `///` where an attribute is invalid causes a compile error.

    // --- Gotcha 2: Nested block comments ---
    // In C, `/* a /* b */ c */` is a syntax error because `*/` closes the
    // first `/*`, leaving `c */` as stray tokens.
    // Rust handles nesting correctly — very handy for commenting out code.

    /* outer /* inner */ still outer — this compiles fine */

    // --- Gotcha 3: `//!` position matters ---
    // `//!` must appear at the *top* of the file (or module) before any items.
    // Placing `//!` after an `fn` item is a compile warning/error depending
    // on context. Always put `//!` before anything else in the file.

    // --- Gotcha 4: Doc-tests are real tests ---
    // Code blocks in `///` comments are compiled and executed by `cargo test`.
    // If they fail, your test suite fails. To skip execution, use
    // ```no_run``` or ```ignore```.

    let p = Point { x: 1.0, y: 2.0 };
    println!("Point: {p:?}");
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
    fn test_point_debug() {
        let p = Point { x: 1.5, y: -2.5 };
        // Verify Debug output format.
        assert_eq!(format!("{p:?}"), "Point { x: 1.5, y: -2.5 }");
    }
}
