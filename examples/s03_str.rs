// Demonstrates the `str` type and its relationship with `String` and `&str`.
//
// In Rust, there are multiple string types:
//   - `str`     — a string *slice* type (unsized, rarely used directly)
//   - `&str`    — a *reference* to a string slice (most common for read-only)
//   - `String`  — an owned, heap-allocated, growable string
//
// Key insight: `str` is an *unsized type* — you almost never use it bare.
// You always work with it behind a reference: `&str`, `Box<str>`, `Arc<str>`.
//
// Run: cargo run --example s03_str

fn main() {
    // --- 1. String literals are `&str` ---
    // A string literal like "hello" is baked into the binary at compile time.
    // Its type is `&'static str` — a reference to a string slice with a
    // static lifetime (lives as long as the program).
    let greeting: &str = "Hello, world!";
    println!("greeting = {greeting}");

    // --- 2. `str` is unsized (dynamically sized type / DST) ---
    // `str` represents a sequence of UTF-8 bytes whose length is only
    // known at runtime. You can't put it on the stack directly:
    //   let s: str = *"hello";  // error: the size of `str` cannot be known

    // `&str` is a "fat pointer": it stores both:
    //   1. A pointer to the first byte
    //   2. The length of the slice
    println!(
        "size of &str   = {} bytes (pointer + length)",
        std::mem::size_of::<&str>()
    );
    println!(
        "size of &i32   = {} bytes (just a pointer)",
        std::mem::size_of::<&i32>()
    );

    // --- 3. `&str` from a `String` ---
    // `String` is the owned, heap-allocated version.
    // You can get a `&str` from a `String` cheaply (no copy).
    let owned = String::from("owned string");
    let slice: &str = &owned; // Deref coercion: &String -> &str
    println!("slice = {slice}");

    // --- 4. Slicing a string ---
    // You can take a sub-slice of a string. Byte indices, not char indices.
    let hello = &greeting[0..5]; // "Hello"
    println!("hello = {hello}");

    // Gotcha: slicing at a non-UTF-8 boundary panics at runtime!
    // let bad = &"cafe\u{0301}"[0..5]; // might panic

    // --- 5. Common `str` methods (called via `&str`) ---
    println!("len()           = {}", greeting.len()); // bytes, not chars
    println!("is_empty()      = {}", greeting.is_empty());
    println!("contains(\"world\") = {}", greeting.contains("world"));
    println!("starts_with(\"H\") = {}", greeting.starts_with('H'));
    println!("to_uppercase()  = {}", greeting.to_uppercase());
    println!("trim()          = {:?}", "  hi  ".trim());

    // --- 6. Iterating over characters vs bytes ---
    // `.chars()` yields `char` (Unicode scalar values).
    // `.bytes()` yields raw `u8` values.
    let emoji = "Rust\u{1F980}"; // Rust + crab emoji
    println!("chars: {:?}", emoji.chars().collect::<Vec<_>>());
    println!("bytes: {:?}", emoji.bytes().collect::<Vec<_>>());
    println!(
        "char count = {}, byte count = {}",
        emoji.chars().count(),
        emoji.len()
    );

    // Gotcha: `.len()` returns bytes, NOT characters!
    // For character count, use `.chars().count()`.

    // --- 7. When to use `&str` vs `String` ---
    // Use `&str` for: function parameters (read-only), string literals, slices.
    // Use `String` for: owned data, building strings, return values.
    // Rule of thumb: accept `&str`, return `String`.
}

/// Example function that accepts `&str` — works with both `String` and `&str`.
fn _greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_is_fat_pointer() {
        // &str is 2x the size of a regular reference (pointer + length).
        assert_eq!(
            std::mem::size_of::<&str>(),
            2 * std::mem::size_of::<usize>()
        );
    }

    #[test]
    fn test_string_to_str_slice() {
        let s = String::from("hello");
        let slice: &str = &s;
        assert_eq!(slice, "hello");
    }

    #[test]
    fn test_slicing() {
        let s = "Hello, world!";
        assert_eq!(&s[0..5], "Hello");
        assert_eq!(&s[7..12], "world");
    }

    #[test]
    fn test_len_is_bytes_not_chars() {
        let emoji = "\u{1F980}"; // crab emoji = 4 bytes
        assert_eq!(emoji.len(), 4);
        assert_eq!(emoji.chars().count(), 1);
    }

    #[test]
    fn test_greet_with_str() {
        assert_eq!(_greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn test_greet_with_string() {
        let name = String::from("Rust");
        assert_eq!(_greet(&name), "Hello, Rust!");
    }
}
