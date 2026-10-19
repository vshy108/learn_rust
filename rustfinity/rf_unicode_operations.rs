//! # Rustfinity: Unicode Operations
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/unicode-operations
//!
//! When working with strings in Rust, understanding the difference between bytes, characters, and grapheme clusters is essential for writing correct text-processing code. Many programming bugs stem from treating strings as simple arrays of characters, when in reality UTF-8 strings have a much richer structure.
//!
//! ## Bytes vs Characters vs Grapheme Clusters
//!
//! In Rust, a `String` is a sequence of UTF-8 encoded bytes. There are three ways to view string data:
//!
//! 1. **Bytes** (`.bytes()`) - The raw UTF-8 bytes. ASCII
//!    characters are 1 byte, but many Unicode characters
//!    take 2-4 bytes.
//! 2. **Characters** (`.chars()`) - Unicode scalar values.
//!    Most characters are single `char`s, but some displayed
//!    characters (like emojis with modifiers) are multiple
//!    `char`s.
//! 3. **Grapheme Clusters** - What humans perceive as a
//!    single "character". A family emoji like "👨‍👩‍👧" is
//!    multiple Unicode code points combined.
//!
//! ```rust
//! let text = "Hello";
//! assert_eq!(text.len(), 5);        // 5 bytes
//! assert_eq!(text.chars().count(), 5); // 5 characters
//!
//! // Family emoji (ZWJ sequence)
//! let emoji = "👨‍👩‍👧";
//! // 18 bytes!
//! assert_eq!(emoji.len(), 18);
//! // 5 Unicode scalars
//! assert_eq!(emoji.chars().count(), 5);
//! // But visually it's 1 "character"
//! // (grapheme cluster)
//! ```
//!
//! ## Safe String Slicing
//!
//! String slicing in Rust must occur at valid UTF-8 boundaries. Slicing in the middle of a multi-byte character causes a panic:
//!
//! ```rust
//! // Russian "Hello"
//! let text = "Здравствуйте";
//! // text[0..1] would panic! 'З' is 2 bytes
//! // OK - takes full first character
//! let slice = &text[0..2];
//! assert_eq!(slice, "З");
//! ```
//!
//! To safely extract substrings, use `.chars()` with indices or the `.char_indices()` method:
//!
//! ```rust
//! let text = "Здравствуйте";
//! let chars: Vec<char> = text.chars().collect();
//! let first_two: String = chars[..2].iter().collect();
//! assert_eq!(first_two, "Зд");
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions to demonstrate Unicode-aware string handling:
//!
//! 1. `char_count(s: &str) -> usize` - Count the number of
//!    Unicode characters (not bytes)
//! 2. `byte_count(s: &str) -> usize` - Count the number of
//!    bytes in the UTF-8 encoding
//! 3. `safe_substring(s: &str, start: usize, end: usize)
//! -> Option<String>` - Extract a substring by character
//!    indices (not byte indices). Return `None` if indices
//!    are out of bounds.
//! 4. `char_at(s: &str, index: usize) -> Option<char>` -
//!    Get the character at a specific index (by character
//!    position, not byte position)
//! 5. `is_single_char(s: &str) -> bool` - Check if a string
//!    contains exactly one Unicode character
//!
//! ## Examples
//!
//! ```rust
//! // char_count
//! assert_eq!(char_count("Hello"), 5);
//! assert_eq!(char_count("Привет"), 6);  // Russian "Hello"
//! assert_eq!(char_count("你好"), 2);     // Chinese "Hello"
//! assert_eq!(char_count("🎉"), 1);       // Single emoji
//!
//! // byte_count
//! // ASCII: 1 byte each
//! assert_eq!(byte_count("Hello"), 5);
//! // Cyrillic: 2 bytes each
//! assert_eq!(byte_count("Привет"), 12);
//! // Chinese: 3 bytes each
//! assert_eq!(byte_count("你好"), 6);
//! // Emoji: 4 bytes
//! assert_eq!(byte_count("🎉"), 4);
//!
//! // safe_substring
//! assert_eq!(
//!     safe_substring("Hello", 0, 3),
//!     Some("Hel".to_string())
//! );
//! assert_eq!(
//!     safe_substring("Привет", 0, 2),
//!     Some("Пр".to_string())
//! );
//! assert_eq!(
//!     safe_substring("Hello", 0, 10),
//!     None
//! );  // Out of bounds
//! assert_eq!(
//!     safe_substring("Hello", 3, 2),
//!     None
//! );   // Invalid range
//!
//! // char_at
//! assert_eq!(char_at("Hello", 0), Some('H'));
//! assert_eq!(char_at("Привет", 2), Some('и'));
//! assert_eq!(char_at("Hello", 10), None);
//!
//! // is_single_char
//! assert_eq!(is_single_char("a"), true);
//! assert_eq!(is_single_char("好"), true);
//! assert_eq!(is_single_char("🎉"), true);
//! assert_eq!(is_single_char("ab"), false);
//! assert_eq!(is_single_char(""), false);
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `.chars().count()` to count Unicode characters,
//!   not `.len()` which counts bytes
//! - Use `.len()` or `.bytes().count()` for byte count
//! - For `safe_substring`, collect characters into a
//!   `Vec<char>` first, then validate indices before
//!   extracting
//! - Use `.chars().nth(index)` to get a character at a
//!   specific position
//! - For `is_single_char`, check that the char count equals
//!   exactly 1
//!
//! </details>
//!
//! Run: `cargo run --example rf_unicode_operations`
//! Test: `cargo test --example rf_unicode_operations`
//! Filter: `cargo test --example rf_unicode_operations <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

/// Count the number of Unicode characters in a string.
///
/// This counts Unicode scalar values, not bytes or grapheme clusters.
pub fn char_count(s: &str) -> usize {
    // TODO: Use .chars() to iterate over Unicode characters
    unimplemented!()
}

/// Count the number of bytes in the UTF-8 encoding of a string.
pub fn byte_count(s: &str) -> usize {
    // TODO: Return the byte length of the string
    unimplemented!()
}

/// Extract a substring by character indices (not byte indices).
///
/// Returns `None` if indices are out of bounds or if start > end.
pub fn safe_substring(s: &str, start: usize, end: usize) -> Option<String> {
    // TODO: Implement safe substring extraction
    unimplemented!()
}

/// Get the character at a specific index (by character position, not byte position).
///
/// Returns `None` if the index is out of bounds.
pub fn char_at(s: &str, index: usize) -> Option<char> {
    // TODO: Get the character at the given position
    unimplemented!()
}

/// Check if a string contains exactly one Unicode character.
pub fn is_single_char(s: &str) -> bool {
    // TODO: Check if the character count equals 1
    unimplemented!()
}

pub fn main() {
    // Example usage
    let examples = ["Hello", "Привет", "你好", "🎉"];

    for text in &examples {
        println!(
            "\"{}\" - chars: {}, bytes: {}",
            text,
            char_count(text),
            byte_count(text)
        );
    }

    // Safe substring demonstration
    let text = "Hello, 世界!";
    println!("\nSubstring [0..5]: {:?}", safe_substring(text, 0, 5));
    println!("Char at index 7: {:?}", char_at(text, 7));
    println!("Is '好' single char? {}", is_single_char("好"));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ============================================
    // Tests for char_count
    // ============================================

    #[test]
    fn char_count_ascii() {
        assert_eq!(char_count("Hello"), 5);
        assert_eq!(char_count(""), 0);
        assert_eq!(char_count("a"), 1);
    }

    #[test]
    fn char_count_cyrillic() {
        assert_eq!(char_count("Привет"), 6);
        assert_eq!(char_count("Здравствуйте"), 12);
    }

    #[test]
    fn char_count_chinese() {
        assert_eq!(char_count("你好"), 2);
        assert_eq!(char_count("世界"), 2);
    }

    #[test]
    fn char_count_emojis() {
        assert_eq!(char_count("🎉"), 1);
        assert_eq!(char_count("🎉🎊🎁"), 3);
    }

    #[test]
    fn char_count_mixed() {
        assert_eq!(char_count("Hello, 世界!"), 10);
        assert_eq!(char_count("Rust 🦀"), 6);
    }

    // ============================================
    // Tests for byte_count
    // ============================================

    #[test]
    fn byte_count_ascii() {
        assert_eq!(byte_count("Hello"), 5);
        assert_eq!(byte_count(""), 0);
        assert_eq!(byte_count("a"), 1);
    }

    #[test]
    fn byte_count_cyrillic() {
        // Cyrillic characters are 2 bytes each in UTF-8
        assert_eq!(byte_count("Привет"), 12);
    }

    #[test]
    fn byte_count_chinese() {
        // Chinese characters are 3 bytes each in UTF-8
        assert_eq!(byte_count("你好"), 6);
        assert_eq!(byte_count("世界"), 6);
    }

    #[test]
    fn byte_count_emojis() {
        // Most emojis are 4 bytes in UTF-8
        assert_eq!(byte_count("🎉"), 4);
        assert_eq!(byte_count("🎉🎊🎁"), 12);
    }

    #[test]
    fn byte_count_mixed() {
        // "Hello, " = 7 bytes, "世界" = 6 bytes, "!" = 1 byte
        assert_eq!(byte_count("Hello, 世界!"), 14);
    }

    // ============================================
    // Tests for safe_substring
    // ============================================

    #[test]
    fn safe_substring_ascii() {
        assert_eq!(safe_substring("Hello", 0, 3), Some("Hel".to_string()));
        assert_eq!(safe_substring("Hello", 1, 4), Some("ell".to_string()));
        assert_eq!(safe_substring("Hello", 0, 5), Some("Hello".to_string()));
    }

    #[test]
    fn safe_substring_cyrillic() {
        assert_eq!(safe_substring("Привет", 0, 2), Some("Пр".to_string()));
        assert_eq!(safe_substring("Привет", 2, 5), Some("иве".to_string()));
    }

    #[test]
    fn safe_substring_chinese() {
        assert_eq!(safe_substring("你好世界", 0, 2), Some("你好".to_string()));
        assert_eq!(safe_substring("你好世界", 2, 4), Some("世界".to_string()));
    }

    #[test]
    fn safe_substring_empty() {
        assert_eq!(safe_substring("Hello", 2, 2), Some("".to_string()));
        assert_eq!(safe_substring("", 0, 0), Some("".to_string()));
    }

    #[test]
    fn safe_substring_out_of_bounds() {
        assert_eq!(safe_substring("Hello", 0, 10), None);
        assert_eq!(safe_substring("Hello", 6, 8), None);
    }

    #[test]
    fn safe_substring_invalid_range() {
        assert_eq!(safe_substring("Hello", 3, 2), None);
        assert_eq!(safe_substring("Hello", 5, 3), None);
    }

    #[test]
    fn safe_substring_emojis() {
        assert_eq!(safe_substring("🎉🎊🎁", 0, 2), Some("🎉🎊".to_string()));
        assert_eq!(safe_substring("🎉🎊🎁", 1, 3), Some("🎊🎁".to_string()));
    }

    // ============================================
    // Tests for char_at
    // ============================================

    #[test]
    fn char_at_ascii() {
        assert_eq!(char_at("Hello", 0), Some('H'));
        assert_eq!(char_at("Hello", 1), Some('e'));
        assert_eq!(char_at("Hello", 4), Some('o'));
    }

    #[test]
    fn char_at_cyrillic() {
        assert_eq!(char_at("Привет", 0), Some('П'));
        assert_eq!(char_at("Привет", 2), Some('и'));
        assert_eq!(char_at("Привет", 5), Some('т'));
    }

    #[test]
    fn char_at_chinese() {
        assert_eq!(char_at("你好世界", 0), Some('你'));
        assert_eq!(char_at("你好世界", 1), Some('好'));
        assert_eq!(char_at("你好世界", 3), Some('界'));
    }

    #[test]
    fn char_at_emoji() {
        assert_eq!(char_at("🎉🎊🎁", 0), Some('🎉'));
        assert_eq!(char_at("🎉🎊🎁", 1), Some('🎊'));
        assert_eq!(char_at("🎉🎊🎁", 2), Some('🎁'));
    }

    #[test]
    fn char_at_out_of_bounds() {
        assert_eq!(char_at("Hello", 5), None);
        assert_eq!(char_at("Hello", 100), None);
        assert_eq!(char_at("", 0), None);
    }

    #[test]
    fn char_at_mixed() {
        let text = "Hello, 世界!";
        assert_eq!(char_at(text, 0), Some('H'));
        assert_eq!(char_at(text, 7), Some('世'));
        assert_eq!(char_at(text, 8), Some('界'));
        assert_eq!(char_at(text, 9), Some('!'));
    }

    // ============================================
    // Tests for is_single_char
    // ============================================

    #[test]
    fn is_single_char_ascii() {
        assert!(is_single_char("a"));
        assert!(is_single_char("Z"));
        assert!(is_single_char("!"));
    }

    #[test]
    fn is_single_char_cyrillic() {
        assert!(is_single_char("П"));
        assert!(is_single_char("й"));
    }

    #[test]
    fn is_single_char_chinese() {
        assert!(is_single_char("你"));
        assert!(is_single_char("好"));
    }

    #[test]
    fn is_single_char_emoji() {
        assert!(is_single_char("🎉"));
        assert!(is_single_char("🦀"));
    }

    #[test]
    fn is_single_char_false_cases() {
        assert!(!is_single_char(""));
        assert!(!is_single_char("ab"));
        assert!(!is_single_char("Hello"));
        assert!(!is_single_char("你好"));
        assert!(!is_single_char("🎉🎊"));
    }

    // ============================================
    // Combined/Edge case tests
    // ============================================

    #[test]
    fn test_whitespace_characters() {
        assert_eq!(char_count(" \t\n"), 3);
        assert_eq!(byte_count(" \t\n"), 3);
        assert!(is_single_char(" "));
        assert!(is_single_char("\t"));
    }

    #[test]
    fn test_special_unicode() {
        // Zero-width joiner character (used in emoji sequences)
        let zwj = "\u{200D}";
        assert!(is_single_char(zwj));
        assert_eq!(char_count(zwj), 1);
        assert_eq!(byte_count(zwj), 3);
    }

    #[test]
    fn test_combining_characters() {
        // 'é' can be represented as single char or as 'e' + combining accent
        let single = "é"; // U+00E9
        let combined = "e\u{0301}"; // 'e' + combining acute accent

        assert_eq!(char_count(single), 1);
        assert_eq!(char_count(combined), 2); // Two Unicode scalars

        assert!(is_single_char(single));
        assert!(!is_single_char(combined)); // Two chars, not one
    }

    #[test]
    fn test_full_char_byte_comparison() {
        // Table of different scripts
        let tests = [
            ("Hello", 5, 5),     // ASCII: 1 byte per char
            ("Привет", 6, 12),   // Cyrillic: 2 bytes per char
            ("你好世界", 4, 12), // Chinese: 3 bytes per char
            ("🎉🎊🎁🎄", 4, 16), // Emojis: 4 bytes per char
        ];

        for (text, expected_chars, expected_bytes) in tests {
            assert_eq!(
                char_count(text),
                expected_chars,
                "char_count failed for \"{}\"",
                text
            );
            assert_eq!(
                byte_count(text),
                expected_bytes,
                "byte_count failed for \"{}\"",
                text
            );
        }
    }
}
