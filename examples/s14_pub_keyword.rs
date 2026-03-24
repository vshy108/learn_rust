// Demonstrates the `pub` keyword for controlling visibility.
//
// By default, everything in Rust is PRIVATE:
// - Functions, structs, enums, constants, modules — all private.
// - `pub` makes them accessible from outside the module.
//
// Visibility levels:
// - `pub` — visible to all
// - `pub(crate)` — visible within the current crate only
// - `pub(super)` — visible to the parent module
// - `pub(in path)` — visible within a specific path
// - (no modifier) — private to the current module
//
// Run: cargo run --example s14_pub_keyword

mod outer {
    // Public function
    pub fn public_fn() -> &'static str {
        "I'm public!"
    }

    // Private function — only accessible from within `outer`
    fn _private_fn() -> &'static str {
        "I'm private!"
    }

    // Public struct with mixed field visibility
    pub struct Config {
        pub name: String,       // public field
        pub(super) debug: bool, // visible to parent (outer's parent = root)
        _secret: String,        // private field
    }

    impl Config {
        /// Creates a new Config. Only way to set private fields.
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                debug: false,
                _secret: "hidden".to_string(),
            }
        }
    }

    // Submodule
    pub mod inner {
        /// Accessible from outside because both `inner` and `greet` are pub.
        pub fn greet() -> String {
            // Can call private fns from parent via super::
            format!("Inner says: {}", super::_private_fn())
        }

        // pub(super) — only visible within outer, not outside.
        pub(super) fn _semi_private() -> &'static str {
            "visible to outer only"
        }
    }
}

fn main() {
    // --- 1. Public function ---
    println!("{}", outer::public_fn());

    // --- 2. Private function is NOT accessible ---
    // outer::_private_fn(); // ERROR: function is private

    // --- 3. Public struct, mixed fields ---
    let config = outer::Config::new("production");
    println!("name: {}", config.name); // pub field OK
    println!("debug: {}", config.debug); // pub(super) OK from root
    // println!("{}", config._secret); // ERROR: private field

    // --- 4. Nested module public fn ---
    println!("{}", outer::inner::greet());

    // --- 5. pub(super) not accessible from here ---
    // outer::inner::_semi_private(); // ERROR: restricted visibility
}

#[cfg(test)]
mod tests {
    use super::outer;

    #[test]
    fn test_public_fn() {
        assert_eq!(outer::public_fn(), "I'm public!");
    }

    #[test]
    fn test_config_creation() {
        let c = outer::Config::new("test");
        assert_eq!(c.name, "test");
    }

    #[test]
    fn test_inner_greet() {
        let g = outer::inner::greet();
        assert!(g.contains("private"));
    }
}
