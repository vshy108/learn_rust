//! # Rustfinity: Structs And References
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/structs-and-references
//!
//! In Rust, structs can store references in addition to owned values. This allows you to create powerful and memory-efficient abstractions by reusing data rather than duplicating it. However, using references in structs requires careful attention to lifetimes to ensure that the references remain valid.
//!
//! In this challenge, you will create a struct named `TextFinder` that holds a reference to a string slice (`&str`). This struct will be used to perform search operations on the string slice by finding lines that contain a given keyword.
//!
//! ## Your Task
//!
//! - Create a struct called `TextFinder` that holds a reference to a string slice.
//! - The struct should have a constructor `new()` that takes a string slice and returns a `TextFinder` instance.
//! - The struct should have a method called `find_first` that returns the first line containing the keyword, or `None` if no match is found.
//! - The struct should have a method called `find_many` that returns a vector of all lines containing the keyword.
//! - The search functionality should be case-sensitive.
//! - Ensure you return references to the original string slice rather than creating new owned strings.
//!
//! ## Hints
//!
//! If you're stuck, you may find the following hints useful:
//!
//! <details>
//! <summary>Click here to reveal hints</summary>
//!
//! - **Lifetime Annotations**: Use a lifetime annotation to link the struct's lifetime to the lifetime of the string slice it references. e.g.
//!   ```rust
//!   pub struct TextFinder<'a> {
//!       text: &'a str,
//!   }
//!   ```
//! - **String Methods**: You can use `.lines()` on a `&str` to split it into lines and `.contains()` to check if a string contains a substring.
//! - **Iterators**: Iterators like `.find()` or `.filter()` can simplify search operations.
//!
//! </details>
//!
//! Run: `cargo run --example rf_structs_and_references`
//! Test: `cargo test --example rf_structs_and_references`
//! Filter: `cargo test --example rf_structs_and_references <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub struct TextFinder<'a> {
    text: &'a str,
}

impl<'a> TextFinder<'a> {
    pub fn new(text: &'a str) -> Self {
        todo!("create a finder for text")
    }

    pub fn find_first(&self, keyword: &str) -> Option<&'a str> {
        todo!("find the first line containing {keyword}")
    }

    pub fn find_many(&self, keyword: &str) -> Vec<&'a str> {
        todo!("find all lines containing {keyword}")
    }
}

// Example usage
pub fn main() {
    let text = "Rust is fast and memory-efficient.\nOwnership is key to Rust's safety.\nRustaceans love the borrow checker.";
    let finder = TextFinder::new(text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is fast and memory-efficient.")

    let matches = finder.find_many("Rust");
    println!("{:?}", matches); // Should print: ["Rust is fast and memory-efficient.", "Ownership is key to Rust's safety."]
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_find_first() {
        let text = "Line one\nLine two with keyword\nAnother line";
        let finder = TextFinder::new(text);

        assert_eq!(finder.find_first("keyword"), Some("Line two with keyword"));
        assert_eq!(finder.find_first("missing"), None);
        assert_eq!(finder.find_first("Line"), Some("Line one"));
    }

    #[test]
    fn test_find_many() {
        let text = "First line with keyword\nSecond line\nAnother keyword line\nFinal line";
        let finder = TextFinder::new(text);

        let results = finder.find_many("keyword");
        assert_eq!(
            results,
            vec!["First line with keyword", "Another keyword line"]
        );

        let empty_results = finder.find_many("nonexistent");
        assert!(empty_results.is_empty());
    }

    #[test]
    fn test_find_many_edge_cases() {
        let text = "keyword in first line\nkeyword\nanother keyword here\nno match here";
        let finder = TextFinder::new(text);

        let results = finder.find_many("keyword");
        assert_eq!(
            results,
            vec!["keyword in first line", "keyword", "another keyword here"]
        );
    }

    #[test]
    fn test_case_sensitivity() {
        let text = "Keyword in line one\nanother Keyword\ncase sensitivity test";
        let finder = TextFinder::new(text);

        assert_eq!(finder.find_first("Keyword"), Some("Keyword in line one"));
        assert_eq!(finder.find_first("keyword"), None);

        let results = finder.find_many("Keyword");
        assert_eq!(results, vec!["Keyword in line one", "another Keyword"]);
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let finder = TextFinder::new(text);

        assert_eq!(finder.find_first("keyword"), None);

        let results = finder.find_many("keyword");
        assert!(results.is_empty());
    }

    #[test]
    fn test_empty_keyword() {
        let text = "First line\nSecond line with keyword\nThird line";
        let finder = TextFinder::new(text);

        assert_eq!(finder.find_first(""), Some("First line"));

        let results = finder.find_many("");
        assert_eq!(
            results,
            vec!["First line", "Second line with keyword", "Third line"]
        );
    }

    #[test]
    fn test_multiple_matches_in_one_line() {
        let text = "Keyword Keyword Keyword\nAnother line";
        let finder = TextFinder::new(text);

        assert_eq!(
            finder.find_first("Keyword"),
            Some("Keyword Keyword Keyword")
        );

        let results = finder.find_many("Keyword");
        assert_eq!(results, vec!["Keyword Keyword Keyword"]);
    }

    #[test]
    fn test_no_newline_in_text() {
        let text = "Single line with Keyword";
        let finder = TextFinder::new(text);

        assert_eq!(
            finder.find_first("Keyword"),
            Some("Single line with Keyword")
        );

        let results = finder.find_many("Keyword");
        assert_eq!(results, vec!["Single line with Keyword"]);
    }

    #[test]
    fn test_lines_with_only_whitespace() {
        let text = "   \nLine with keyword\n   \nAnother keyword line\n";
        let finder = TextFinder::new(text);

        assert_eq!(finder.find_first("keyword"), Some("Line with keyword"));

        let results = finder.find_many("keyword");
        assert_eq!(results, vec!["Line with keyword", "Another keyword line"]);
    }

    #[test]
    fn test_whitespace_in_keyword() {
        let text = "Line with a space\nKeyword in a line\n";
        let finder = TextFinder::new(text);

        assert_eq!(finder.find_first("a "), Some("Line with a space"));

        let results = finder.find_many("a ");
        assert_eq!(results, vec!["Line with a space", "Keyword in a line"]);
    }
}
