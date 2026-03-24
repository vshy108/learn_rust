// Demonstrates the glob operator (* and **) in use statements.
//
// `use module::*` imports ALL public items from a module.
// This is called a "glob import" or "wildcard import".
//
// **Caution**: Glob imports can cause naming conflicts and
// make it unclear where items come from. Use sparingly.
//
// Acceptable uses:
// - `use super::*` in test modules
// - `use crate::prelude::*` (library preludes)
// - Quick prototyping
//
// Run: cargo run --example s14_glob_operator

mod colors {
    pub const RED: &str = "red";
    pub const GREEN: &str = "green";
    pub const BLUE: &str = "blue";

    pub fn mix(a: &str, b: &str) -> String {
        format!("{a}+{b}")
    }
}

mod shapes {
    pub const CIRCLE: &str = "circle";

    pub fn describe(shape: &str) -> String {
        format!("shape: {shape}")
    }
}

fn main() {
    // --- 1. Glob import ---
    {
        use colors::*;
        println!("{RED}, {GREEN}, {BLUE}");
        println!("mix: {}", mix(RED, BLUE));
    }

    // --- 2. Problem: naming conflicts ---
    // If both modules had a `describe` function, glob would conflict.
    use shapes::*;
    println!("{CIRCLE}");
    println!("{}", describe(CIRCLE));

    // --- 3. Selective import is usually better ---
    use colors::{RED, mix};
    println!("selective: {RED}, mix: {}", mix(RED, "yellow"));

    // --- 4. Glob in tests is standard practice ---
    // See the test module below — `use super::*` is idiomatic.
}

#[cfg(test)]
mod tests {
    // Glob import from parent is standard in tests:
    use super::colors;
    use super::shapes;

    #[test]
    fn test_colors() {
        assert_eq!(colors::RED, "red");
        assert_eq!(colors::mix("a", "b"), "a+b");
    }

    #[test]
    fn test_shapes() {
        assert_eq!(shapes::CIRCLE, "circle");
    }
}
