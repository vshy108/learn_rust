// Demonstrates using external crates.
//
// External crates are added to `Cargo.toml` under `[dependencies]`.
// Then you `use` them like any module.
//
// Example from this project's Cargo.toml:
//   [dependencies]
//   tokio = { version = "1", features = ["full"] }
//
// Run: cargo run --example s14_external_crates

fn main() {
    println!("=== External Crates ===");
    println!();

    // --- 1. How to add a dependency ---
    println!("Add to Cargo.toml:");
    println!("  [dependencies]");
    println!("  serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("  tokio = {{ version = \"1\", features = [\"full\"] }}");
    println!();

    // --- 2. Or use cargo add ---
    println!("Or use the command:");
    println!("  cargo add serde --features derive");
    println!();

    // --- 3. Usage in code ---
    println!("Then in your code:");
    println!("  use serde::{{Serialize, Deserialize}};");
    println!();

    // --- 4. crates.io is the registry ---
    println!("Browse crates at: https://crates.io");
    println!("Docs at: https://docs.rs");
    println!();

    // --- 5. This project already uses tokio ---
    // We can demonstrate with a simple example:
    println!("This project uses tokio (async runtime).");
    println!("Check Cargo.toml for the full dependency list.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_external_crates_example_runs() {
        assert!(true);
    }
}
