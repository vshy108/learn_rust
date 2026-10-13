//! # Rustfinity: Trait Bounds
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/trait-bounds
//!
//! Now that we know a bit about traits and generics, let's dive into **trait bounds**. Trait bounds allow you to specify that a generic type must implement a particular trait. This ensures that only types with certain capabilities can be used with your generic code.
//!
//! For example:
//!
//! - A generic function can be constrained to types that implement the `std::fmt::Display` trait for printing.
//! - A generic struct can be limited to types that implement traits like `PartialOrd` for comparison.
//!
//! Trait bounds make your code flexible while maintaining strong type safety.
//!
//! ## Syntax
//!
//! You can specify trait bounds in two ways:
//!
//! 1. **Inline**: Specify the trait bounds after the `impl` or `fn` keyword.
//!
//!    ```rust
//!    fn my_function<T: Trait1 + Trait2>(value: T) {
//!        // code here
//!    }
//!    ```
//!
//! 2. **Where clause**: Use a `where` clause to specify trait bounds.
//!
//!    ```rust
//!    fn my_function<T>(value: T)
//!    where
//!         T: Trait1 + Trait2,
//!    {
//!         // code here
//!    }
//!    ```
//!
//! This means that the generic type `T` must implement both `Trait1` and `Trait2`.
//!
//! ## Your Task
//!
//! 1. Define a generic function `compare_and_display` that:
//!    - Takes two parameters of the same type.
//!    - Returns the greater of the two using the `PartialOrd` trait.
//!
//! 2. Use trait bounds to ensure the function works only with types that implement both `Display` and `PartialOrd`.
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use `std::cmp::PartialOrd` to compare values.
//! - Use `std::fmt::Display` to print values with the `println!` macro.
//! - You can write trait bounds inline with `T: Trait1 + Trait2` or using a `where` clause. e.g.
//!   ```rust
//!   fn my_function<T: Trait1 + Trait2>(value: T) {
//!       // code here
//!   }
//!   ```
//!
//! </details>
//!
//! Run: `cargo run --example rf_trait_bounds`
//! Test: `cargo test --example rf_trait_bounds`
//! Filter: `cargo test --example rf_trait_bounds <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

// TODO: Define the generic function `compare_and_display` with appropriate trait bounds.
pub fn compare_and_display<T>(first: T, second: T) -> T
where
    T: std::fmt::Display + PartialOrd,
{
    unimplemented!("compare and display the greater value")
}

// Example usage
pub fn main() {
    let greater = compare_and_display(10, 20);
    println!("Greater value: {}", greater);

    let greater = compare_and_display("Apple", "Orange");
    println!("Greater value: {}", greater);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_compare_integers() {
        assert_eq!(compare_and_display(10, 20), 20);
        assert_eq!(compare_and_display(30, 15), 30);
        assert_eq!(compare_and_display(-5, -10), -5);
        assert_eq!(compare_and_display(0, 0), 0);
    }

    #[test]
    fn test_compare_floats() {
        assert_eq!(compare_and_display(3.14, 2.71), 3.14);
        assert_eq!(compare_and_display(-1.0, -2.0), -1.0);
        assert_eq!(compare_and_display(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_compare_strings() {
        assert_eq!(compare_and_display("Apple", "Orange"), "Orange");
        assert_eq!(compare_and_display("Zebra", "Yak"), "Zebra");
        assert_eq!(compare_and_display("abc", "xyz"), "xyz");
        assert_eq!(compare_and_display("", "NonEmpty"), "NonEmpty");
    }

    #[test]
    fn test_compare_characters() {
        assert_eq!(compare_and_display('a', 'b'), 'b');
        assert_eq!(compare_and_display('z', 'y'), 'z');
        assert_eq!(compare_and_display('!', '@'), '@');
    }

    #[test]
    fn test_compare_custom_struct() {
        #[derive(Debug, PartialOrd, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl std::fmt::Display for Point {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Point({}, {})", self.x, self.y)
            }
        }

        let point1 = Point { x: 1, y: 2 };
        let point2 = Point { x: 3, y: 4 };

        assert_eq!(compare_and_display(point1, point2), Point { x: 3, y: 4 });
    }

    #[test]
    fn test_compare_equal_custom_struct() {
        #[derive(Debug, PartialOrd, PartialEq)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl std::fmt::Display for Rectangle {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Rectangle({}, {})", self.width, self.height)
            }
        }

        let rect1 = Rectangle {
            width: 4,
            height: 5,
        };
        let rect2 = Rectangle {
            width: 4,
            height: 5,
        };

        assert_eq!(
            compare_and_display(rect1, rect2),
            Rectangle {
                width: 4,
                height: 5
            }
        );
    }

    #[test]
    fn test_compare_different_magnitude_custom_struct() {
        #[derive(Debug, PartialOrd, PartialEq)]
        struct Circle {
            radius: f64,
        }

        impl std::fmt::Display for Circle {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Circle({})", self.radius)
            }
        }

        let circle1 = Circle { radius: 1.5 };
        let circle2 = Circle { radius: 2.5 };

        assert_eq!(
            compare_and_display(circle1, circle2),
            Circle { radius: 2.5 }
        );
    }
}
