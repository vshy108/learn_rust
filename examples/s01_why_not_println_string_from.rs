// Section 1, Topic 1: Why not println!(String::from("Hello, world!"))
//
// In Python, print() accepts any object and calls str() on it.
// In Rust, println!() is a *macro* that expects its first argument to be a
// **format string literal** — a &str known at compile time. It cannot be a
// String (heap-allocated) or any runtime value because the compiler parses
// the format placeholders at compile time.

/// Demonstrate the correct way to print a String value.
///
/// # Returns
///
/// A greeting string built from `String::from`.
fn build_greeting() -> String {
    // `String::from` creates a heap-allocated String from a &str literal.
    // In Python this is like: greeting = str("Hello, world!")
    String::from("Hello, world!")
}

/// Show that you can pass a &str literal directly to println!.
///
/// # Returns
///
/// The literal string slice.
fn literal_greeting() -> &'static str {
    "Hello, world!"
}

fn main() {
    // CORRECT: pass a format string literal, then the String as an argument.
    // The `{}` placeholder uses the Display trait (like Python's __str__).
    let greeting = build_greeting();
    println!("{greeting}");

    // CORRECT: pass a &str literal directly as the format string.
    println!("{}", literal_greeting());

    // WHY NOT println!(String::from("Hello, world!")) ?
    //
    // The first argument to println! MUST be a string literal (&str) so the
    // compiler can parse format placeholders like `{}` at compile time.
    // `String::from(...)` returns a `String` (heap-allocated), which is NOT
    // a literal. The compiler error would be:
    //
    //   error: format argument must be a string literal
    //
    // This is fundamentally different from Python's print(), which accepts
    // any object at runtime. Rust's approach catches format string errors at
    // compile time — no runtime format-string bugs possible.

    // ALSO CORRECT: use a variable holding &str as a format argument.
    let name = "Rust";
    println!("Hello, {name}!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_greeting_returns_expected_string() {
        assert_eq!(build_greeting(), "Hello, world!");
    }

    #[test]
    fn test_literal_greeting_returns_expected_str() {
        assert_eq!(literal_greeting(), "Hello, world!");
    }

    #[test]
    fn test_greeting_types_are_equal_in_content() {
        // String and &str can be compared directly in Rust because
        // String implements PartialEq<str> (like Python's == on str objects).
        assert_eq!(build_greeting(), literal_greeting());
    }
}
