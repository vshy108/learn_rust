// Demonstrates the `super` keyword for referring to the parent module.
//
// `super::` moves up one level in the module hierarchy.
// Similar to `..` in file paths.
//
// Run: cargo run --example s14_super_keyword

mod parent {
    pub fn parent_fn() -> &'static str {
        "parent function"
    }

    pub const VERSION: u32 = 1;

    pub mod child {
        /// Calls the parent's function using super::
        pub fn call_parent() -> String {
            // super:: goes up one level from `child` to `parent`
            format!("child calling: {}", super::parent_fn())
        }

        /// Access parent's constant.
        pub fn version() -> u32 {
            super::VERSION
        }

        pub mod grandchild {
            /// Can chain super:: to go multiple levels up.
            pub fn call_grandparent() -> String {
                // super::super:: goes from grandchild → child → parent
                format!(
                    "grandchild: {}, v{}",
                    super::super::parent_fn(),
                    super::super::VERSION
                )
            }
        }
    }
}

// Practical example: tests use super:: to access parent items.
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    #[cfg(test)]
    mod tests {
        // super:: here refers to `calculator` module
        use super::*;

        #[test]
        fn test_add() {
            assert_eq!(add(2, 3), 5);
        }

        #[test]
        fn test_multiply() {
            assert_eq!(multiply(3, 4), 12);
        }
    }
}

fn main() {
    // --- 1. child calling parent ---
    println!("{}", parent::child::call_parent());

    // --- 2. accessing parent constant ---
    println!("version: {}", parent::child::version());

    // --- 3. grandchild using chained super ---
    println!("{}", parent::child::grandchild::call_grandparent());

    // --- 4. Summary ---
    println!();
    println!("super:: = parent module (like .. in paths)");
    println!("super::super:: = grandparent (can chain)");
    println!("Most common use: `use super::*` in test modules");
}

#[cfg(test)]
mod tests {
    use super::parent;

    #[test]
    fn test_child_calls_parent() {
        assert!(parent::child::call_parent().contains("parent function"));
    }

    #[test]
    fn test_grandchild() {
        let msg = parent::child::grandchild::call_grandparent();
        assert!(msg.contains("parent function"));
    }
}
