// Demonstrates the concepts of packages and crates in Rust.
//
// **Crate**: The smallest compilation unit. Two kinds:
// - **Binary crate**: Has a `main()` function; compiles to an executable.
// - **Library crate**: No `main()`; compiles to a .rlib for other crates to use.
//
// **Package**: A bundle of one or more crates with a `Cargo.toml`.
// Rules:
// - A package can have AT MOST one library crate.
// - A package can have any number of binary crates.
// - A package must have at least one crate (library or binary).
//
// Default conventions:
// - `src/main.rs` → binary crate (root)
// - `src/lib.rs` → library crate (root)
// - `src/bin/*.rs` → additional binary crates
// - `examples/*.rs` → example binaries (this file!)
//
// Run: cargo run --example s14_packages_crates

fn main() {
    println!("=== Packages and Crates ===");
    println!();

    // This project is a PACKAGE containing:
    // - One binary crate (src/main.rs)
    // - Many example binaries (examples/*.rs)
    println!("This file is an example binary in the `learn_rust` package.");
    println!();

    // The crate ROOT is the source file that the compiler starts from:
    // - For a binary: src/main.rs (or src/bin/foo.rs)
    // - For a library: src/lib.rs
    println!("Crate roots:");
    println!("  Binary:  src/main.rs");
    println!("  Library: src/lib.rs");
    println!("  Extra bins: src/bin/*.rs");
    println!("  Examples: examples/*.rs (like this file!)");
    println!();

    // The "crate" keyword refers to the current crate root.
    println!("Key Cargo.toml fields:");
    println!("  [package] name = \"learn_rust\"");
    println!("  edition = \"2024\"");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_package_concepts() {
        // Just verifying the example compiles and runs.
        assert!(true);
    }
}
