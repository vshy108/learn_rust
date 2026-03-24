// Demonstrates module ambiguity.
//
// Rust doesn't allow both `src/foo.rs` and `src/foo/mod.rs` to exist.
// You must choose ONE module layout style per module:
//
// Style 1 (modern): `src/foo.rs` + `src/foo/bar.rs`
// Style 2 (legacy): `src/foo/mod.rs` + `src/foo/bar.rs`
//
// Mixing them for the SAME module triggers a compile error.
//
// In a single-file example, we demonstrate the concept with inline modules.
//
// Run: cargo run --example s14_module_ambiguity

// You cannot declare the same module name twice in the same scope:
// mod foo { }
// mod foo { }  // ERROR: the name `foo` is defined multiple times

// But you CAN have same-named items in DIFFERENT modules:
mod animals {
    pub fn name() -> &'static str {
        "Dog"
    }
}

mod robots {
    pub fn name() -> &'static str {
        "R2D2"
    }
}

fn main() {
    println!("=== Module Ambiguity ===");
    println!();

    // Same function name `name()` — no ambiguity because they're in different modules
    println!("animal: {}", animals::name());
    println!("robot: {}", robots::name());

    // --- Key rules ---
    println!();
    println!("Rules for file-based modules:");
    println!("  1. Can't have both foo.rs AND foo/mod.rs");
    println!("  2. Modern style: foo.rs + foo/bar.rs");
    println!("  3. Legacy style: foo/mod.rs + foo/bar.rs");
    println!("  4. Pick one style per module (Rust 2018+)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_name_different_modules() {
        assert_eq!(animals::name(), "Dog");
        assert_eq!(robots::name(), "R2D2");
    }
}
