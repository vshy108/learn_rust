//! # Rustfinity: Advanced Match Patterns
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/advanced-match-patterns
//!
//! Pattern matching in Rust is a powerful feature that allows you to destructure and inspect complex data types in a concise and readable manner. By using match statements, you can handle different variants of enums, tuples, and other data structures, applying specific logic based on the shape and content of the data.
//!
//! This challenge will help you practice advanced pattern matching techniques in a real-world scenario.
//!
//! ## Your Task
//!
//! You need to implement a method `check_validity(&self)` for the `BookItem` enum that returns a `bool` indicating whether the item is valid based on the following rules:
//!
//! 1. **Book**:
//!    - The number of pages must be greater than 0.
//!    - If there is a discount:
//!      - Must be non-negative (≥ 0)
//!      - Must not exceed 50% (≤ 50)
//!
//! 2. **EBook**:
//!    - The title (a `String`) must be non-empty.
//!    - The second field in the tuple must be greater than 0.
//!
//! 3. **Collection**:
//!    - Must not be empty
//!    - **ALL** items in the collection must be valid
//!    - This means a collection containing even one invalid item is considered invalid
//!
//! 4. **OutOfPrint**:
//!    - This variant is always considered invalid.
//!
//! ### Requirements
//!
//! - Implement `check_validity(&self)` using match guards to reflect these rules.
//! - Return a boolean (`true` or `false`) based on the type of `BookItem`.
//! - Ensure that the function does not panic for any valid input.
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use `if` guards to combine multiple conditions: `if *pages > 0 && *d >= 0`
//! - Pattern match on `Option<i32>` using `Some(d)` and `None`
//! - For Collections:
//!   - Check `!items.is_empty()` first
//!   - Use `iter().all(|item| item.check_validity())` to ensure all items are valid
//! - Remember to dereference numbers when using them in guards: `*pages`, `*second`
//! - The catch-all `_ => false` arm handles any remaining invalid cases
//!
//! </details>
//!
//! Run: `cargo run --example rf_advanced_match_patterns`
//! Test: `cargo test --example rf_advanced_match_patterns`
//! Filter: `cargo test --example rf_advanced_match_patterns <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

#[derive(Debug, Clone)]
pub enum BookItem {
    Book { pages: i32, discount: Option<i32> },
    EBook(String, (i32, i32)),
    Collection(Vec<BookItem>),
    OutOfPrint,
}

impl BookItem {
    pub fn check_validity(&self) -> bool {
        todo!("check whether this book item is valid")
    }
}

// Example usage
pub fn main() {
    let book_a = BookItem::Book {
        pages: 42,
        discount: Some(100),
    };
    let ebook_b = BookItem::EBook("hello".to_string(), (1, 2));
    let collection_c = BookItem::Collection(vec![book_a.clone(), BookItem::OutOfPrint]);

    assert!(
        !book_a.check_validity(),
        "Book with discount > 50 should be invalid"
    );
    assert!(
        ebook_b.check_validity(),
        "EBook with valid title and tuple should be valid"
    );
    assert!(
        !collection_c.check_validity(),
        "Collection containing invalid items should be invalid"
    );
    assert!(
        !BookItem::OutOfPrint.check_validity(),
        "OutOfPrint should always be invalid"
    );
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_book_with_discount_valid() {
        let data = BookItem::Book {
            pages: 42,
            discount: Some(100),
        };
        assert_eq!(
            data.check_validity(),
            false,
            "Book with discount > 50 should be invalid"
        );
    }

    #[test]
    fn test_book_no_discount_valid() {
        let data = BookItem::Book {
            pages: 42,
            discount: None,
        };
        assert_eq!(
            data.check_validity(),
            true,
            "Book with valid pages and no discount should be valid"
        );
    }

    #[test]
    fn test_book_negative_pages_invalid() {
        let data = BookItem::Book {
            pages: -1,
            discount: Some(10),
        };
        assert_eq!(
            data.check_validity(),
            false,
            "Book with negative pages should be invalid"
        );
    }

    #[test]
    fn test_book_discount_out_of_range_invalid() {
        let data = BookItem::Book {
            pages: 42,
            discount: Some(60),
        };
        assert_eq!(
            data.check_validity(),
            false,
            "Book with discount > 50 should be invalid"
        );
    }

    #[test]
    fn test_book_negative_discount_invalid() {
        let data = BookItem::Book {
            pages: 42,
            discount: Some(-10),
        };
        assert_eq!(
            data.check_validity(),
            false,
            "Book with negative discount should be invalid"
        );
    }

    #[test]
    fn test_ebook_valid() {
        let data = BookItem::EBook("hello".to_string(), (1, 2));
        assert_eq!(
            data.check_validity(),
            true,
            "EBook with non-empty title and valid tuple should be valid"
        );
    }

    #[test]
    fn test_ebook_empty_name_invalid() {
        let data = BookItem::EBook("".to_string(), (0, 0));
        assert_eq!(
            data.check_validity(),
            false,
            "EBook with empty title should be invalid"
        );
    }

    #[test]
    fn test_ebook_invalid_tuple() {
        let data = BookItem::EBook("hello".to_string(), (1, 0));
        assert_eq!(
            data.check_validity(),
            false,
            "EBook with second tuple value <= 0 should be invalid"
        );
    }

    #[test]
    fn test_collection_valid() {
        let data = BookItem::Collection(vec![
            BookItem::Book {
                pages: 10,
                discount: None,
            },
            BookItem::Book {
                pages: 20,
                discount: Some(30),
            },
        ]);
        assert_eq!(
            data.check_validity(),
            true,
            "Collection with all valid items should be valid"
        );
    }

    #[test]
    fn test_collection_with_invalid() {
        let data = BookItem::Collection(vec![
            BookItem::Book {
                pages: 10,
                discount: None,
            },
            BookItem::OutOfPrint,
        ]);
        assert_eq!(
            data.check_validity(),
            false,
            "Collection with any invalid item should be invalid"
        );
    }

    #[test]
    fn test_collection_all_invalid() {
        let data = BookItem::Collection(vec![
            BookItem::Book {
                pages: -10,
                discount: None,
            },
            BookItem::OutOfPrint,
        ]);
        assert_eq!(
            data.check_validity(),
            false,
            "Collection with all invalid items should be invalid"
        );
    }

    #[test]
    fn test_collection_empty_invalid() {
        let data = BookItem::Collection(vec![]);
        assert_eq!(
            data.check_validity(),
            false,
            "Empty collection should be invalid"
        );
    }

    #[test]
    fn test_out_of_print_invalid() {
        let data = BookItem::OutOfPrint;
        assert_eq!(
            data.check_validity(),
            false,
            "OutOfPrint item should always be invalid"
        );
    }

    #[test]
    fn test_nested_collection_valid() {
        let data = BookItem::Collection(vec![
            BookItem::Collection(vec![
                BookItem::Book {
                    pages: 10,
                    discount: None,
                },
                BookItem::Book {
                    pages: 20,
                    discount: Some(30),
                },
            ]),
            BookItem::EBook("nested".to_string(), (5, 6)),
        ]);
        assert_eq!(
            data.check_validity(),
            true,
            "Nested collection with all valid items should be valid"
        );
    }

    #[test]
    fn test_nested_collection_all_invalid() {
        let data = BookItem::Collection(vec![
            BookItem::Collection(vec![
                BookItem::Book {
                    pages: -10,
                    discount: None,
                },
                BookItem::OutOfPrint,
            ]),
            BookItem::EBook("".to_string(), (0, 0)),
        ]);
        assert_eq!(
            data.check_validity(),
            false,
            "Nested collection with all invalid items should be invalid"
        );
    }
}
