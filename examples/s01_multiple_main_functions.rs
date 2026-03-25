#![allow(dead_code, unused_imports, unused_parens)]
// Section 1, Topic 3: Multiple main functions
//
// Each **binary crate** has exactly ONE `fn main()`.
//
// However, a Cargo project can have multiple binaries:
//   - `src/main.rs`          → the default binary (run with `cargo run`)
//   - `src/bin/foo.rs`       → extra binary (run with `cargo run --bin foo`)
//   - `examples/foo.rs`      → example binary (run with `cargo run --example foo`)
//
// Each of these files has its own `fn main()`. They do NOT conflict because
// Cargo compiles each as a separate binary crate.
//
// KEY INSIGHT: "one main per binary" is the rule. Multiple binaries = multiple
// mains, but they live in separate files and compile independently.

/// Describe where multiple main functions can live in a Cargo project.
///
/// # Returns
///
/// A vector of (location, run_command) tuples.
fn binary_locations() -> Vec<(&'static str, &'static str)> {
    vec![
        ("src/main.rs", "cargo run"),
        ("src/bin/<name>.rs", "cargo run --bin <name>"),
        ("examples/<name>.rs", "cargo run --example <name>"),
    ]
}

/// Check if a filename would be a valid Rust binary entry point.
///
/// # Arguments
///
/// * `path` - A file path string to check.
///
/// # Returns
///
/// `true` if the path ends with `.rs` and could contain a `fn main()`.
fn is_rust_source(path: &str) -> bool {
    path.ends_with(".rs")
}

fn main() {
    println!("=== Where can fn main() live in a Cargo project? ===\n");

    for (location, command) in binary_locations() {
        println!("  {location}");
        println!("    Run with: {command}\n");
    }

    println!("Each file above compiles into its own binary.");
    println!("They each have their own fn main() — no conflict.");
    println!("\nThis file itself is an example binary:");
    println!("  cargo run --example s01_multiple_main_functions");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_locations_returns_three_entries() {
        let locations = binary_locations();
        assert_eq!(locations.len(), 3);
    }

    #[test]
    fn test_all_locations_are_rust_files() {
        for (location, _) in binary_locations() {
            assert!(is_rust_source(location), "{location} should end with .rs");
        }
    }

    #[test]
    fn test_is_rust_source_positive() {
        assert!(is_rust_source("main.rs"));
        assert!(is_rust_source("src/bin/server.rs"));
    }

    #[test]
    fn test_is_rust_source_negative() {
        assert!(!is_rust_source("main.py"));
        assert!(!is_rust_source("Cargo.toml"));
    }
}
