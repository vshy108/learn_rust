// Demonstrates modules in Rust.
//
// Modules organize code into namespaces. They control:
// - **Organization**: group related code together
// - **Privacy**: items are private by default
// - **Path**: how you refer to items (like file paths)
//
// Syntax:
//   mod name {
//       // items: functions, structs, enums, constants, traits, modules
//   }
//
// Run: cargo run --example s14_modules

// --- Define modules ---
mod math {
    /// Adds two numbers.
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Multiplies two numbers.
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    // This function is PRIVATE — only accessible within `math`.
    fn _internal_helper() -> i32 {
        42
    }
}

mod greetings {
    /// Returns a greeting string.
    pub fn hello(name: &str) -> String {
        format!("Hello, {name}!")
    }

    /// Nested module.
    pub mod formal {
        /// Returns a formal greeting.
        pub fn greet(name: &str) -> String {
            format!("Good day, {name}. How do you do?")
        }
    }
}

fn main() {
    // --- 1. Access module items with :: ---
    let sum = math::add(3, 4);
    let product = math::multiply(5, 6);
    println!("add: {sum}, multiply: {product}");

    // --- 2. Private items are not accessible ---
    // math::_internal_helper(); // ERROR: function is private

    // --- 3. Nested modules ---
    println!("{}", greetings::hello("Rust"));
    println!("{}", greetings::formal::greet("Sir Ferris"));

    // --- 4. use keyword to bring into scope ---
    use math::add;
    println!("via use: {}", add(10, 20));

    use greetings::formal::greet;
    println!("via use: {}", greet("Ms. Crab"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(math::add(2, 3), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(math::multiply(4, 5), 20);
    }

    #[test]
    fn test_hello() {
        assert_eq!(greetings::hello("World"), "Hello, World!");
    }

    #[test]
    fn test_formal() {
        let g = greetings::formal::greet("Bob");
        assert!(g.contains("Bob"));
    }
}
