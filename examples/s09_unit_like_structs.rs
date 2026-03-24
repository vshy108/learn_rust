// Demonstrates unit-like structs (structs with no fields).
//
// A unit-like struct has no fields — it's just a type name.
// Syntax: `struct Marker;`
//
// Uses:
// - Type-level markers (e.g., state machines, phantom types).
// - Implementing traits on a zero-size type.
// - Sentinel or capability tokens.
//
// Size: A unit struct has size 0 (zero-sized type / ZST).
// It takes no memory at runtime but exists at the type level.
//
// Run: cargo run --example s09_unit_like_structs

use std::fmt;

/// A marker indicating production environment.
struct Production;

/// A marker indicating development environment.
struct Development;

/// Trait for environment-specific behavior.
trait Environment {
    /// Returns the environment name.
    fn name(&self) -> &'static str;
    /// Returns the log level for this environment.
    fn log_level(&self) -> &'static str;
}

impl Environment for Production {
    fn name(&self) -> &'static str {
        "production"
    }
    fn log_level(&self) -> &'static str {
        "warn"
    }
}

impl Environment for Development {
    fn name(&self) -> &'static str {
        "development"
    }
    fn log_level(&self) -> &'static str {
        "debug"
    }
}

impl fmt::Debug for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Production")
    }
}

impl fmt::Debug for Development {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Development")
    }
}

/// Logs a message based on the environment.
fn log_startup(env: &dyn Environment) {
    println!(
        "[{}] Starting app (log level: {})",
        env.name(),
        env.log_level()
    );
}

fn main() {
    // --- 1. Creating unit struct instances ---
    let _prod = Production; // no fields, no parentheses, no braces
    let _dev = Development;

    // --- 2. Using as trait objects ---
    log_startup(&Production);
    log_startup(&Development);

    // --- 3. Zero-sized types ---
    println!(
        "Size of Production: {} bytes",
        std::mem::size_of::<Production>()
    );
    println!(
        "Size of Development: {} bytes",
        std::mem::size_of::<Development>()
    );

    // --- 4. Can be stored in collections (though unusual) ---
    let envs: Vec<Box<dyn Environment>> = vec![Box::new(Production), Box::new(Development)];
    for env in &envs {
        println!("  env: {} → log: {}", env.name(), env.log_level());
    }

    // --- 5. Pattern matching on unit structs ---
    let marker = Production;
    match marker {
        Production => println!("Matched Production!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_sized() {
        assert_eq!(std::mem::size_of::<Production>(), 0);
        assert_eq!(std::mem::size_of::<Development>(), 0);
    }

    #[test]
    fn test_production_env() {
        let env = Production;
        assert_eq!(env.name(), "production");
        assert_eq!(env.log_level(), "warn");
    }

    #[test]
    fn test_development_env() {
        let env = Development;
        assert_eq!(env.name(), "development");
        assert_eq!(env.log_level(), "debug");
    }
}
