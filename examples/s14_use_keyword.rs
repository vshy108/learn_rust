// Demonstrates the `use` keyword for bringing items into scope.
//
// `use` creates shortcuts so you don't need full paths every time.
//
// use module::item;        // bring a specific item
// use module::{a, b};      // bring multiple items
// use module::item as alias; // rename
//
// Run: cargo run --example s14_use_keyword

mod utils {
    pub mod math {
        /// Adds two numbers.
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        /// Squares a number.
        pub fn square(n: i32) -> i32 {
            n * n
        }
    }

    pub mod strings {
        /// Capitalizes the first letter.
        pub fn capitalize(s: &str) -> String {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        }
    }
}

// --- 1. use at the top level ---
use utils::math::add;
use utils::strings::capitalize;

// --- 2. Grouped imports ---
use utils::math::{add as math_add, square};

fn main() {
    // --- 3. Use the short names ---
    println!("add(3, 4) = {}", add(3, 4));
    println!("square(5) = {}", square(5));
    println!("capitalize: {}", capitalize("hello"));

    // --- 4. Aliased import ---
    println!("math_add(1, 2) = {}", math_add(1, 2));

    // --- 5. Still can use full path if you want ---
    println!("full path: {}", utils::math::add(10, 20));

    // --- 6. use inside a function ---
    fn process() {
        use utils::math::square;
        println!("local use: square(7) = {}", square(7));
    }
    process();

    // --- 7. Standard library use ---
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("map: {map:?}");
}

#[cfg(test)]
mod tests {
    use super::utils::math::*;
    use super::utils::strings::capitalize;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_square() {
        assert_eq!(square(4), 16);
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize(""), "");
    }
}
