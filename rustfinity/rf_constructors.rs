//! # Rustfinity: Constructors
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/constructors
//!
//! Unlike other programming languages, Rust doesn't have a built-in `constructor` keyword to construct instances of a struct. Instead, you can directly create an instance of a struct simply by calling the struct's name and passing the required values.
//!
//! ```rust
//! struct Post {
//!     pub content: String,
//!     pub user: String,
//!     pub views: u32
//! }
//!
//! fn main(){
//!     let new_blog_post = Post {
//!         content: "Some content".to_string(),
//!         user: "Jon Doe".to_string(),
//!         views: 0
//!     };
//! }
//! ```
//!
//! However, this pattern is discouraged, in real-life scenarios, we might not want to expose all fields with `pub` or some fields will have default values that it would be repetitive to provide the value for every new instance, instead.
//!
//! In Rust we have a **convention**, constructors are implemented as associated functions named `new`. They encapsulate the initialization of the struct, ensuring that instances are always created in a valid state.
//!
//! For example, for the `Post` struct above, we can have a default value for the `views` to always be `0` for new instances.
//!
//! ```rust
//! impl Post {
//!     pub fn new(content: String, user: String) -> Post {
//!         Post {
//!             content,
//!             user,
//!             views: 0
//!         }
//!     }
//! }
//!
//! fn main(){
//!     let new_blog_post = Post::new(
//!         "Some content".to_string(),
//!         "Jon Doe".to_string()
//!     );
//! }
//! ```
//!
//! ## Your Task
//!
//! In this challenge, you will implement a constructor for a struct that represents a **Book**. A `Book` will have the following fields:
//!
//! - `title`: A string that represents the book's title.
//! - `author`: A string that represents the book's author.
//! - `year`: An integer that represents the year the book was published.
//! - `likes`: An integer that represents the number of likes the book has received. Default value is `0`.
//!
//! The constructor function, `Book::new`, should take three parameters (`title`, `author`, and `year`) and return a fully initialized `Book`.
//!
//! ## Requirements
//!
//! 1. Implement a constructor function `Book::new` that:
//!    - Takes three arguments: `title: &str`, `author: &str`, and `year` (integer).
//!    - Returns a `Book` instance with the specified values and default `likes` value of `0`.
//!
//! 2. Remember to use `pub` for fields (required for testing).
//!
//! ## Example Test
//!
//! ```rust
//! let book = Book::new(
//!     "ASOIAF",
//!     "George R. R. Martin",
//!     1996
//! );
//!
//! assert_eq!(book.title, "ASOIAF");
//! assert_eq!(book.author, "George R. R. Martin");
//! assert_eq!(book.year, 1996);
//! assert_eq!(book.likes, 0);
//! ```
//!
//! Run: `cargo run --example rf_constructors`
//! Test: `cargo test --example rf_constructors`
//! Filter: `cargo test --example rf_constructors <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub struct Book {
    pub title: String,
    pub author: String,
    pub year: i32,
    pub likes: u32,
}

impl Book {
    pub fn new(title: &str, author: &str, year: i32) -> Self {
        todo!("construct a book titled {title} by {author}, published in {year}")
    }
}

fn main() {
    println!("Implement the Constructors challenge.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_constructor_initialization() {
        let book = Book::new(
            "The Rust Programming Language",
            "Steve Klabnik and Carol Nichols",
            2019,
        );

        assert_eq!(book.title, "The Rust Programming Language");
        assert_eq!(book.author, "Steve Klabnik and Carol Nichols");
        assert_eq!(book.year, 2019);
        assert_eq!(book.likes, 0);
    }

    #[test]
    fn test_constructor_with_different_data() {
        let book = Book::new("Eloquent JavaScript", "Marijn Haverbeke", 2018);

        assert_eq!(book.title, "Eloquent JavaScript");
        assert_eq!(book.author, "Marijn Haverbeke");
        assert_eq!(book.year, 2018);
        assert_eq!(book.likes, 0);
    }

    #[test]
    fn test_constructor_handles_edge_case_year() {
        let book = Book::new("Old Book", "Unknown Author", -500);

        assert_eq!(book.title, "Old Book");
        assert_eq!(book.author, "Unknown Author");
        assert_eq!(book.year, -500);
        assert_eq!(book.likes, 0);
    }

    #[test]
    fn test_constructor_with_empty_title_and_author() {
        let book = Book::new("", "", 2023);

        assert_eq!(book.title, "");
        assert_eq!(book.author, "");
        assert_eq!(book.year, 2023);
        assert_eq!(book.likes, 0);
    }

    #[test]
    fn test_constructor_long_title_and_author() {
        let long_title = "A Very Long and Detailed Title for a Book That Goes On and On and On";
        let long_author =
            "A Very Long and Detailed Name of an Author Who Has an Incredibly Lengthy Name";
        let book = Book::new(long_title, long_author, 2024);

        assert_eq!(book.title, long_title);
        assert_eq!(book.author, long_author);
        assert_eq!(book.year, 2024);
        assert_eq!(book.likes, 0);
    }

    #[test]
    fn test_constructor_special_characters_in_title_and_author() {
        let special_title = "Rust ♥ Programming!";
        let special_author = "John \"Rustacean\" Doe";
        let book = Book::new(special_title, special_author, 2020);

        assert_eq!(book.title, special_title);
        assert_eq!(book.author, special_author);
        assert_eq!(book.year, 2020);
        assert_eq!(book.likes, 0);
    }

    #[test]
    fn test_constructor_unicode_characters_in_title_and_author() {
        let unicode_title = "Программирование на Rust";
        let unicode_author = "Иван Иванович";
        let book = Book::new(unicode_title, unicode_author, 2021);

        assert_eq!(book.title, unicode_title);
        assert_eq!(book.author, unicode_author);
        assert_eq!(book.year, 2021);
        assert_eq!(book.likes, 0);
    }

    #[test]
    fn test_constructor_negative_year() {
        let book = Book::new("Ancient Wisdom", "Unknown Sage", -3000);

        assert_eq!(book.title, "Ancient Wisdom");
        assert_eq!(book.author, "Unknown Sage");
        assert_eq!(book.year, -3000);
        assert_eq!(book.likes, 0);
    }

    #[test]
    fn test_constructor_zero_year() {
        let book = Book::new("The Dawn of Civilization", "Prehistoric Scholar", 0);

        assert_eq!(book.title, "The Dawn of Civilization");
        assert_eq!(book.author, "Prehistoric Scholar");
        assert_eq!(book.year, 0);
        assert_eq!(book.likes, 0);
    }
}
