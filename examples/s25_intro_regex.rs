// Intro to the regex crate for regular expressions in Rust.
//
// The `regex` crate provides compiled regex patterns for searching,
// matching, and replacing text.
//
// Run: cargo run --example s25_intro_regex

use regex::Regex;

fn main() {
    println!("=== Intro to regex Crate ===");

    // --- 1. Create and use a regex ---
    let re = Regex::new(r"\d+").expect("invalid regex");
    let text = "I have 42 apples and 7 oranges";

    // is_match: does the pattern exist?
    println!("has numbers? {}", re.is_match(text));

    // find: first match
    if let Some(m) = re.find(text) {
        println!(
            "first number: '{}' at {}..{}",
            m.as_str(),
            m.start(),
            m.end()
        );
    }

    // --- 2. Case-insensitive ---
    let re = Regex::new(r"(?i)rust").expect("invalid");
    println!("matches 'RUST'? {}", re.is_match("I love RUST"));

    // --- 3. Anchored pattern ---
    let re = Regex::new(r"^hello").expect("invalid");
    println!("starts with hello? {}", re.is_match("hello world"));
    println!("starts with hello? {}", re.is_match("say hello"));
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_is_match() {
        let re = Regex::new(r"\d+").unwrap();
        assert!(re.is_match("abc123"));
        assert!(!re.is_match("no digits"));
    }

    #[test]
    fn test_find() {
        let re = Regex::new(r"\d+").unwrap();
        let m = re.find("abc42def").unwrap();
        assert_eq!(m.as_str(), "42");
    }
}
