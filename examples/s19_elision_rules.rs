// Demonstrates lifetime elision rules.
//
// Rust has 3 rules that automatically insert lifetime annotations,
// so you usually don't need to write them.
//
// Rule 1: Each reference parameter gets its own lifetime.
// Rule 2: If exactly one input lifetime, it's assigned to all output lifetimes.
// Rule 3: If one parameter is `&self` or `&mut self`, self's lifetime is used.
//
// If the rules don't determine output lifetimes, you must annotate.
//
// Run: cargo run --example s19_elision_rules

// --- Rule 1 + 2: One input reference → output gets same lifetime ---
// Written:  fn first(s: &str) -> &str
// Expanded: fn first<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// --- Rule 1 only: Two inputs, no output reference → no annotation needed ---
// Written:  fn both(a: &str, b: &str) -> bool
fn starts_same(a: &str, b: &str) -> bool {
    a.as_bytes().first() == b.as_bytes().first()
}

// --- Rule 3: &self method → output gets self's lifetime ---
struct Config {
    name: String,
}

impl Config {
    // Written:  fn name(&self) -> &str
    // Expanded: fn name<'a>(&'a self) -> &'a str
    fn name(&self) -> &str {
        &self.name
    }
}

// --- Rules DON'T cover this case: two input refs, returning one ---
// Must annotate because compiler can't know which input the output relates to:
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    // Rule 2 in action:
    let sentence = "hello world";
    let word = first_word(sentence);
    println!("first word: {word}");

    // No output ref:
    println!("starts same: {}", starts_same("abc", "axyz"));

    // Rule 3 in action:
    let cfg = Config {
        name: "prod".into(),
    };
    println!("config: {}", cfg.name());

    // Explicit annotation required:
    let long = longest("rust", "go");
    println!("longest: {long}");

    println!();
    println!("Elision rules (in order):");
    println!("  1. Each param gets its own lifetime");
    println!("  2. Single input lifetime -> used for output");
    println!("  3. &self/&mut self lifetime -> used for output");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
    }

    #[test]
    fn test_starts_same() {
        assert!(starts_same("apple", "avocado"));
        assert!(!starts_same("apple", "banana"));
    }

    #[test]
    fn test_config_name() {
        let c = Config {
            name: "test".into(),
        };
        assert_eq!(c.name(), "test");
    }
}
