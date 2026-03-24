// Demonstrates creating a string slice from a String.
//
// A **string slice** (`&str`) is a reference to a contiguous sequence of
// bytes in a String (or string literal). It's a "view" into string data.
//
// Syntax: `&s[start..end]` — byte indices, NOT character indices!
//
// Gotchas:
// - Indices are BYTE offsets, not char positions. Slicing in the middle
//   of a multi-byte UTF-8 character panics at runtime.
// - The range is half-open: `[start..end)` (start inclusive, end exclusive).
//
// Run: cargo run --example s08_string_slice

fn main() {
    let s = String::from("hello world");

    // --- 1. Basic slicing ---
    let hello: &str = &s[0..5]; // bytes 0..5 → "hello"
    let world: &str = &s[6..11]; // bytes 6..11 → "world"
    println!("hello = \"{hello}\", world = \"{world}\"");

    // --- 2. Slice the whole string ---
    let full: &str = &s[..]; // same as &s[0..s.len()]
    println!("full = \"{full}\"");

    // --- 3. Relationship: &String auto-coerces to &str ---
    let s_ref: &str = &s; // Deref coercion: &String → &str
    println!("s_ref = \"{s_ref}\"");

    // --- 4. Slices borrow from the original ---
    // While a slice exists, the original String can't be mutated.
    let mut text = String::from("mutable");
    let slice = &text[0..3]; // immutable borrow
    println!("slice = \"{slice}\"");
    // text.push('!'); // error: can't mutate while slice is borrowed
    // After slice's last use (above), we can mutate:
    text.push('!');
    println!("text = \"{text}\"");

    // --- 5. String literals are slices ---
    // "hello" has type &'static str — a slice pointing into the binary.
    let literal: &str = "I am a slice";
    println!("literal = \"{literal}\"");

    // --- 6. UTF-8 caution ---
    let emoji = String::from("Rust is fun!");
    // Safe: ASCII chars are 1 byte each.
    let rust = &emoji[0..4];
    println!("rust = \"{rust}\"");
    // Danger: slicing multi-byte chars would panic:
    // let s = String::from("cafe\u{0301}"); // "café"
    // let _ = &s[4..5]; // panics: byte 4 is middle of the accent char
}

/// Extracts the first word from a string slice.
///
/// # Arguments
///
/// * `s` - A string slice
///
/// # Returns
///
/// A slice containing the first word
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    s // No space found — whole string is one word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_slice() {
        let s = String::from("hello world");
        assert_eq!(&s[0..5], "hello");
        assert_eq!(&s[6..11], "world");
    }

    #[test]
    fn test_full_slice() {
        let s = String::from("test");
        assert_eq!(&s[..], "test");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_string_literal_is_str() {
        let s: &str = "literal";
        assert_eq!(s.len(), 7);
    }
}
