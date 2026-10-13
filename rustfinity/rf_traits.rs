//! # Rustfinity: Traits
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/traits
//!
//! In Rust, traits define shared behavior that types can implement. A trait acts as a contract, requiring implementing types to provide certain methods. This makes traits one of the most powerful features for achieving polymorphism and code reusability in Rust.
//!
//! For example, the `Display` trait allows custom formatting for types using the `{}` formatter. Similarly, you can define your own traits to describe behaviors relevant to your program.
//!
//! ## Your Task
//!
//! In this challenge, you'll implement a trait `Describable` and use it to define a common interface for objects that can provide a description. Your task is to:
//!
//! 1. Define a trait `Describable` with a single method `describe` that returns a `String`.
//! 2. Implement this trait for the struct `Person`.
//! 3. Implement this trait for the struct `Book`.
//!
//! ### Requirements
//!
//! - The `Person` struct should have fields `name: String` and `age: u8`.
//! - The `Book` struct should have fields `title: String` and `author: String`.
//! - The `describe` method for `Person` should return a string
//!   like `"Person: Alice, Age: 30"`.
//! - The `describe` method for `Book` should return a string
//!   like `"Book: Rust Programming, Author: Jane Doe"`.
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Implement a trait for a struct using
//!   `impl TraitName for StructName`. e.g.
//!   ```rust
//!   impl Describable for Person {
//!       fn describe(&self) -> String {
//!           format!(
//!               "Person: {}, Age: {}",
//!               self.name,
//!               self.age
//!           )
//!       }
//!   }
//!   ```
//! - Don't forget to use the `&self` as the parameter for
//!   the `describe` method in the trait.
//!
//! </details>
//!
//! Run: `cargo run --example rf_traits`
//! Test: `cargo test --example rf_traits`
//! Filter: `cargo test --example rf_traits <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Person {
    pub name: String,
    pub age: u8,
}

pub struct Book {
    pub title: String,
    pub author: String,
}

impl Describable for Person {
    fn describe(&self) -> String {
        todo!("describe person {}", self.name)
    }
}

impl Describable for Book {
    fn describe(&self) -> String {
        todo!("describe book {}", self.title)
    }
}

// Example usage
pub fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let book = Book {
        title: "Rust Programming".to_string(),
        author: "Jane Doe".to_string(),
    };

    println!("{}", person.describe());
    println!("{}", book.describe());
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_person_description() {
        let person = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        assert_eq!(person.describe(), "Person: Alice, Age: 30");
    }

    #[test]
    fn test_book_description() {
        let book = Book {
            title: "Rust Programming".to_string(),
            author: "Jane Doe".to_string(),
        };
        assert_eq!(book.describe(), "Book: Rust Programming, Author: Jane Doe");
    }

    #[test]
    fn test_person_with_empty_name() {
        let person = Person {
            name: "".to_string(),
            age: 25,
        };
        assert_eq!(person.describe(), "Person: , Age: 25");
    }

    #[test]
    fn test_book_with_empty_title_and_author() {
        let book = Book {
            title: "".to_string(),
            author: "".to_string(),
        };
        assert_eq!(book.describe(), "Book: , Author: ");
    }

    #[test]
    fn test_person_description_with_special_characters() {
        let person = Person {
            name: "J@hn_Doe!".to_string(),
            age: 45,
        };
        assert_eq!(person.describe(), "Person: J@hn_Doe!, Age: 45");
    }

    #[test]
    fn test_book_description_with_special_characters() {
        let book = Book {
            title: "Learn Rust: A Practical Approach!".to_string(),
            author: "Dr. Cøder".to_string(),
        };
        assert_eq!(
            book.describe(),
            "Book: Learn Rust: A Practical Approach!, Author: Dr. Cøder"
        );
    }

    #[test]
    fn test_person_age_boundary_values() {
        let young_person = Person {
            name: "Baby".to_string(),
            age: 0,
        };
        assert_eq!(young_person.describe(), "Person: Baby, Age: 0");

        let old_person = Person {
            name: "Elder".to_string(),
            age: 255, // Maximum value for `u8`
        };
        assert_eq!(old_person.describe(), "Person: Elder, Age: 255");
    }
}
