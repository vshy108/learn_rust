// Demonstrates the size of `char` in Rust.
//
// A Rust `char` is 4 bytes (32 bits) — it represents a Unicode scalar value.
// This is NOT the same as a byte (u8) or a C `char` (1 byte).
//
// A Unicode scalar value is any Unicode code point except surrogates
// (U+0000 to U+D7FF and U+E000 to U+10FFFF).
//
// Run: cargo run --example s03_char_size

fn main() {
    // --- 1. char is 4 bytes ---
    println!("size_of::<char>() = {} bytes", std::mem::size_of::<char>()); // 4

    // --- 2. char holds any Unicode scalar value ---
    let ascii = 'A';
    let accented = '\u{00E9}'; // e with acute accent
    let chinese = '\u{4e16}'; // Chinese character
    let math = '\u{03C0}'; // Greek pi
    println!("ASCII:    '{ascii}' = U+{:04X}", ascii as u32);
    println!("Accented: '{accented}' = U+{:04X}", accented as u32);
    println!("Chinese:  '{chinese}' = U+{:04X}", chinese as u32);
    println!("Math:     '{math}' = U+{:04X}", math as u32);

    // --- 3. char vs u8 ---
    // `u8` is a byte (0-255). `char` is a Unicode scalar (0-0x10FFFF).
    // String bytes (UTF-8) can be 1-4 bytes per character.
    let ch = 'A';
    println!();
    println!("'A' as char: {} bytes", std::mem::size_of_val(&ch));
    println!("'A' in UTF-8: {} byte(s)", ch.len_utf8());
    println!("Chinese char in UTF-8: {} bytes", chinese.len_utf8());

    // --- 4. char methods ---
    println!();
    println!("'A'.is_alphabetic() = {}", 'A'.is_alphabetic());
    println!("'5'.is_numeric()    = {}", '5'.is_numeric());
    println!("'A'.is_uppercase()  = {}", 'A'.is_uppercase());
    println!("'a'.is_lowercase()  = {}", 'a'.is_lowercase());
    println!("' '.is_whitespace() = {}", ' '.is_whitespace());
    println!("'A'.to_lowercase(): {:?}", 'A'.to_lowercase().next());

    // --- 5. char is NOT indexable in strings ---
    // Rust strings are UTF-8 encoded. You can't index by byte position
    // to get a char because multi-byte characters would break.
    let s = "hello";
    // s[0]  // error: String/str cannot be indexed by integer
    // Use .chars() iterator instead:
    let first = s.chars().next();
    println!();
    println!("first char of \"{s}\": {first:?}");

    // --- 6. char range ---
    println!();
    println!("char range: U+0000 to U+10FFFF (excluding surrogates)");
    println!("Max valid: '\\u{{10FFFF}}' = U+10FFFF");
    println!(
        "Total Unicode scalars: {}",
        0x10FFFF_u32 - 0xD800 + 0xE000 - 0xDFFF
    );

    // --- Gotcha: "character" != "grapheme cluster" ---
    // Some visible characters are composed of multiple `char` values.
    // For example, some emojis or accented characters use combining marks.
    // For proper text segmentation, use the `unicode-segmentation` crate.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_char_size() {
        assert_eq!(std::mem::size_of::<char>(), 4);
    }

    #[test]
    fn test_char_is_unicode() {
        assert!('\u{4e16}'.is_alphabetic()); // Chinese character
        assert!('5'.is_numeric());
        assert!(' '.is_whitespace());
    }

    #[test]
    fn test_len_utf8() {
        assert_eq!('A'.len_utf8(), 1); // ASCII: 1 byte
        assert_eq!('\u{00E9}'.len_utf8(), 2); // accented: 2 bytes
        assert_eq!('\u{4e16}'.len_utf8(), 3); // Chinese: 3 bytes
    }

    #[test]
    fn test_char_to_digit() {
        assert_eq!('7'.to_digit(10), Some(7));
        assert_eq!('a'.to_digit(16), Some(10));
        assert_eq!('z'.to_digit(10), None);
    }

    #[test]
    fn test_ascii_value() {
        assert_eq!('A' as u32, 65);
        assert_eq!('z' as u32, 122);
    }
}
