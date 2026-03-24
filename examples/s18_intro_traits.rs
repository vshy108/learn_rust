// Introduces traits — Rust's version of interfaces/protocols.
//
// A **trait** defines shared behavior (method signatures) that
// types can implement. Think of it like Python's abstract base class
// or a Java/Go interface.
//
// Key idea: "What can this type DO?" rather than "What IS this type?"
//
// Run: cargo run --example s18_intro_traits

fn main() {
    println!("=== Intro to Traits ===");
    println!();
    println!("Traits define shared behavior:");
    println!("  - Like Python's abstract base classes (ABC)");
    println!("  - Like Java/TypeScript interfaces");
    println!("  - Like Go interfaces (but explicit, not structural)");
    println!();
    println!("Common built-in traits:");
    println!("  Debug   — {{:?}} formatting");
    println!("  Display — {{}} formatting");
    println!("  Clone   — deep copy");
    println!("  Copy    — bitwise copy (for simple types)");
    println!("  PartialEq — == comparison");
    println!("  Default — default value");
    println!("  Iterator — iteration protocol");
    println!();
    println!("You can define your own traits and implement them");
    println!("for your types (and even for standard library types!).");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_built_in_traits() {
        // Debug: derive gives us {:?}
        #[derive(Debug)]
        #[allow(dead_code)] // fields used only via Debug formatting
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 1, y: 2 };
        assert_eq!(format!("{p:?}"), "Point { x: 1, y: 2 }");
    }
}
