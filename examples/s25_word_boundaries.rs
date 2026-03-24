// Demonstrates word boundaries with regex.
//
// `\b` matches a word boundary (between \w and \W).
// Useful for matching whole words.
//
// Run: cargo run --example s25_word_boundaries

use regex::Regex;

fn main() {
    println!("=== Word Boundaries ===");

    // --- 1. Match whole word ---
    let re = Regex::new(r"\brust\b").expect("invalid");
    let texts = ["rust is great", "rustic charm", "distrust", "Rust"];
    for t in &texts {
        println!("  '{t}' matches \\brust\\b? {}", re.is_match(t));
    }

    // --- 2. Case-insensitive word match ---
    let re = Regex::new(r"(?i)\brust\b").expect("invalid");
    println!("case-insensitive 'Rust': {}", re.is_match("I love Rust!"));

    // --- 3. Match word at start ---
    let re = Regex::new(r"\bhello\b").expect("invalid");
    println!("'hello world' matches: {}", re.is_match("hello world"));
    println!("'say hello' matches: {}", re.is_match("say hello"));
    println!("'helloworld' matches: {}", re.is_match("helloworld"));

    // --- 4. Find all standalone numbers ---
    let re = Regex::new(r"\b\d+\b").expect("invalid");
    let found: Vec<&str> = re
        .find_iter("x=42, y=100, z=7abc")
        .map(|m| m.as_str())
        .collect();
    println!("standalone numbers: {found:?}");
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_word_boundary() {
        let re = Regex::new(r"\bcat\b").unwrap();
        assert!(re.is_match("the cat sat"));
        assert!(!re.is_match("category"));
        assert!(!re.is_match("scatter"));
    }

    #[test]
    fn test_whole_word_numbers() {
        let re = Regex::new(r"\b\d+\b").unwrap();
        let v: Vec<&str> = re.find_iter("a 42 b 100").map(|m| m.as_str()).collect();
        assert_eq!(v, vec!["42", "100"]);
    }
}
