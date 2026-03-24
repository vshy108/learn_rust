// Demonstrates compiler directives (attributes) in Rust.
//
// Rust uses *attributes* — metadata attached to items — instead of
// preprocessor directives like C's `#define` / `#ifdef`.
//
// Syntax:
//   `#[attr]`  — outer attribute (applies to the NEXT item)
//   `#![attr]` — inner attribute (applies to the ENCLOSING item)
//
// Run: cargo run --example s02_compiler_directives

// --- Inner attribute: applies to this entire file/crate ---
// `#![allow(...)]` suppresses specific warnings for the whole file.
#![allow(dead_code)]

// --- 1. `#[allow(...)]` / `#[warn(...)]` / `#[deny(...)]` ---
// Control the severity of compiler lints.
// `allow`: silence the warning entirely
// `warn`: show a warning (default for most lints)
// `deny`: treat as a compile error

#[allow(unused_variables)]
fn demo_allow() {
    let x = 42; // no warning about unused `x`
}

// --- 2. `#[cfg(...)]` — conditional compilation ---
// Similar to C's `#ifdef`, but type-checked and more powerful.
// The item is only compiled when the condition is true.

#[cfg(target_os = "macos")]
fn platform_message() -> &'static str {
    "Running on macOS"
}

#[cfg(target_os = "linux")]
fn platform_message() -> &'static str {
    "Running on Linux"
}

#[cfg(target_os = "windows")]
fn platform_message() -> &'static str {
    "Running on Windows"
}

// `cfg!()` macro returns a bool at runtime (the item is always compiled).
fn check_debug_mode() -> bool {
    cfg!(debug_assertions)
}

// --- 3. `#[derive(...)]` — auto-implement traits ---
// The compiler generates trait implementations automatically.
// This is one of the most-used attributes in Rust.
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

// --- 4. `#[test]` and `#[cfg(test)]` ---
// `#[test]` marks a function as a test case.
// `#[cfg(test)]` conditionally compiles a module only during testing.
// See the `tests` module at the bottom for examples.

// --- 5. `#[inline]`, `#[cold]`, `#[must_use]` ---
// Performance and correctness hints.

/// Compute the square of a number.
///
/// # Arguments
///
/// * `x` - The number to square
///
/// # Returns
///
/// The square of `x`
#[inline]
fn square(x: i32) -> i32 {
    x * x
}

/// A function that's rarely called (helps branch prediction).
#[cold]
fn handle_error(msg: &str) {
    eprintln!("Error: {msg}");
}

/// Returns a value that should not be ignored.
///
/// # Returns
///
/// Always returns `true`
#[must_use = "this result should be checked"]
fn important_check() -> bool {
    true
}

// --- 6. `#[deprecated]` ---
/// Old API that should no longer be used.
///
/// # Returns
///
/// Always returns 0
#[deprecated(since = "0.2.0", note = "use `new_function` instead")]
fn old_function() -> i32 {
    0
}

fn new_function() -> i32 {
    42
}

fn main() {
    // allow
    demo_allow();

    // cfg — platform detection
    println!("{}", platform_message());
    println!(
        "Debug mode: {}",
        if check_debug_mode() { "yes" } else { "no" }
    );

    // derive
    let p = Point { x: 1.0, y: 2.0 };
    let p2 = p.clone();
    println!("Point: {p:?}");
    println!("Points equal: {}", p == p2);

    // inline
    println!("square(5) = {}", square(5));

    // must_use — ignoring this would produce a warning
    let _check = important_check();

    // deprecated — using this produces a warning (suppressed by allow)
    #[allow(deprecated)]
    let _old = old_function();
    let _new = new_function();
    println!("new_function() = {_new}");

    // cold
    if false {
        handle_error("this won't run");
    }

    // --- Common attributes summary ---
    println!();
    println!("Common attributes:");
    println!("  #[derive(...)]:     auto-implement traits");
    println!("  #[cfg(...)]:        conditional compilation");
    println!("  #[test]:            mark test function");
    println!("  #[allow(...)]:      suppress warnings");
    println!("  #[deny(...)]:       treat warning as error");
    println!("  #[deprecated]:      mark item as deprecated");
    println!("  #[must_use]:        warn if return value ignored");
    println!("  #[inline]:          suggest inlining");
    println!("  #[doc = \"...\"]]:    documentation (/// syntax)");

    // --- Gotcha: inner vs outer attributes ---
    // `#![...]` (with `!`) applies to the *parent* (file, module, crate).
    // `#[...]`  (without `!`) applies to the *next* item.
    // Mixing them up is a common beginner mistake.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(0), 0);
        assert_eq!(square(3), 9);
        assert_eq!(square(-4), 16);
    }

    #[test]
    fn test_cfg_debug() {
        // In test builds, debug_assertions is typically true.
        assert!(check_debug_mode());
    }

    #[test]
    fn test_derive_clone_eq() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_must_use() {
        // Using the result satisfies `#[must_use]`.
        assert!(important_check());
    }

    #[test]
    fn test_new_function() {
        assert_eq!(new_function(), 42);
    }
}
