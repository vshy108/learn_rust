//! # Rustfinity: Mutable Struct References
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/mutable-struct-references
//!
//! Rust allows structs to hold mutable references to owned data, enabling them to modify the data in-place. This requires a deep understanding of lifetimes and Rust's borrowing rules to ensure memory safety.
//!
//! ## Your Task
//!
//! In this challenge, you will implement a struct named `MutableTextFinder` that holds a mutable reference to a `String`. This struct will allow for both searching and modifying the content of the `String`.
//!
//! The `MutableTextFinder` struct should provide the following functionality:
//!
//! - `new`: Creates a new instance of `MutableTextFinder` with the given content.
//! - `find_first`: Searches for the first line containing a given keyword and returns it as an immutable reference (`Option<&str>`).
//! - `replace_lines`: Replaces all lines containing a given keyword with a replacement string.
//! - `get_text`: Returns the reference to the content.
//! - Searches should be case-sensitive.
//!
//! Run: `cargo run --example rf_mutable_struct_references`
//! Test: `cargo test --example rf_mutable_struct_references`
//! Filter: `cargo test --example rf_mutable_struct_references <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub struct MutableTextFinder<'a> {
    content: &'a mut String,
}

impl<'a> MutableTextFinder<'a> {
    pub fn new(content: &'a mut String) -> Self {
        todo!("construct a finder for mutable text")
    }

    pub fn find_first(&self, keyword: &str) -> Option<&str> {
        todo!("find the first matching line for {keyword}")
    }

    pub fn replace_lines(&mut self, keyword: &str, replacement: &str) {
        todo!("replace matching lines for {keyword} with {replacement}")
    }

    pub fn get_text(&self) -> &str {
        todo!("return the current text")
    }
}

// Example usage
pub fn main() {
    let mut text = String::from("Rust is awesome\nLearning Rust\nFun with Rustaceans");
    let mut finder = MutableTextFinder::new(&mut text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is awesome")

    finder.replace_lines("Rust", "Programming in Rust");
    println!("{}", finder.get_text()); // Should print the modified text
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_text() {
        let mut text = String::from("Initial text\nAnother line");
        let finder = MutableTextFinder::new(&mut text);

        assert_eq!(finder.get_text(), "Initial text\nAnother line");
    }

    #[test]
    fn test_get_text_after_modification() {
        let mut text = String::from("Rust is awesome\nReplace this line\nFinal line");
        let mut finder = MutableTextFinder::new(&mut text);

        finder.replace_lines("Replace", "This line has been replaced");
        assert_eq!(
            finder.get_text(),
            "Rust is awesome\nThis line has been replaced\nFinal line"
        );
    }

    #[test]
    fn test_get_text_large_content() {
        let mut text = "Rust ".repeat(1000) + "\nKeyword line\n" + &"Rust ".repeat(1000);
        let mut finder = MutableTextFinder::new(&mut text);

        assert!(finder.get_text().contains("Keyword line"));
        finder.replace_lines("Keyword", "Replaced line");
        assert!(finder.get_text().contains("Replaced line"));
        assert!(!finder.get_text().contains("Keyword line"));
    }

    #[test]
    fn test_get_text_after_find_first() {
        let mut text = String::from("Line one\nLine two with keyword\nAnother line");
        let finder = MutableTextFinder::new(&mut text);

        let first = finder.find_first("keyword");
        assert_eq!(first, Some("Line two with keyword"));
        assert_eq!(
            finder.get_text(),
            "Line one\nLine two with keyword\nAnother line"
        );
    }

    #[test]
    fn test_get_text_after_multiple_replacements() {
        let mut text = String::from("Replace this line\nKeep this line\nReplace that line too");
        let mut finder = MutableTextFinder::new(&mut text);

        finder.replace_lines("Replace", "Line replaced");
        assert_eq!(
            finder.get_text(),
            "Line replaced\nKeep this line\nLine replaced"
        );

        finder.replace_lines("Keep", "Line kept");
        assert_eq!(finder.get_text(), "Line replaced\nLine kept\nLine replaced");
    }

    #[test]
    fn test_get_text_with_special_characters() {
        let mut text = String::from("Line with emoji 😊\nAnother line 🚀\nFinal 🌟");
        let mut finder = MutableTextFinder::new(&mut text);

        assert_eq!(
            finder.get_text(),
            "Line with emoji 😊\nAnother line 🚀\nFinal 🌟"
        );
        finder.replace_lines("🚀", "Rocket");
        assert_eq!(finder.get_text(), "Line with emoji 😊\nRocket\nFinal 🌟");
    }

    #[test]
    fn test_get_text_empty_string() {
        let mut text = String::new();
        let finder = MutableTextFinder::new(&mut text);

        assert_eq!(finder.get_text(), "");
    }

    #[test]
    fn test_get_text_after_no_replacement() {
        let mut text = String::from("Line one\nLine two\nLine three");
        let mut finder = MutableTextFinder::new(&mut text);

        finder.replace_lines("missing", "replacement");
        assert_eq!(finder.get_text(), "Line one\nLine two\nLine three");
    }

    #[test]
    fn test_get_text_combined_operations() {
        let mut text = String::from("Search line\nAnother line with keyword\nLast line");
        let mut finder = MutableTextFinder::new(&mut text);

        assert_eq!(
            finder.get_text(),
            "Search line\nAnother line with keyword\nLast line"
        );

        let first = finder.find_first("keyword");
        assert_eq!(first, Some("Another line with keyword"));
        assert_eq!(
            finder.get_text(),
            "Search line\nAnother line with keyword\nLast line"
        );

        finder.replace_lines("keyword", "found");
        assert_eq!(&text, "Search line\nfound\nLast line");
    }

    #[test]
    fn test_find_first_should_find_first_only() {
        let mut text = "Same line\nSame line 2\nSame line 3\nDifferent\nSame line 4".to_string();

        let finder = MutableTextFinder::new(&mut text);
        let first = finder.find_first("Same");

        assert_eq!(first, Some("Same line"));
    }
}
