// Demonstrates that a trait must be in scope to call its methods.
//
// If you have a type that implements a trait, you can only call
// the trait's methods if the trait is imported into scope.
//
// Run: cargo run --example s18_trait_in_scope

mod formatting {
    /// A custom formatting trait.
    pub trait Fancy {
        fn fancy(&self) -> String;
    }

    impl Fancy for String {
        fn fancy(&self) -> String {
            format!("~*~ {self} ~*~")
        }
    }

    impl Fancy for i32 {
        fn fancy(&self) -> String {
            format!("#{self}#")
        }
    }
}

fn main() {
    let s = String::from("Hello");

    // --- 1. Without importing the trait —  method not available ---
    // s.fancy(); // ERROR: no method named `fancy` found

    // --- 2. Import the trait to use its methods ---
    {
        use formatting::Fancy;
        println!("{}", s.fancy());
        println!("{}", 42.fancy());
    }

    // --- 3. Common example: Write trait ---
    // use std::io::Write; // needed to call .write() on files

    // --- 4. Prelude traits are always in scope ---
    // Clone, Copy, Debug, Display variants, Iterator, etc.
    // That's why .clone(), .to_string() always work.
    let cloned = s.clone(); // Clone is in the prelude
    println!("cloned: {cloned}");

    println!();
    println!("Rule: import the trait to use its methods!");
    println!("  use formatting::Fancy;");
    println!("  use std::io::Write;");
    println!("  use std::fmt::Display;");
}

#[cfg(test)]
mod tests {
    use super::formatting::Fancy;

    #[test]
    fn test_fancy_string() {
        let s = String::from("test");
        assert!(s.fancy().contains("test"));
    }

    #[test]
    fn test_fancy_int() {
        assert!(42.fancy().contains("42"));
    }
}
