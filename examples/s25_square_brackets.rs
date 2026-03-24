// Demonstrates square brackets (character classes) in regex.
//
// `[abc]` matches a, b, or c. `[a-z]` matches any lowercase letter.
// `[^abc]` matches anything except a, b, or c.
//
// Run: cargo run --example s25_square_brackets

use regex::Regex;

fn main() {
    println!("=== Square Brackets (Character Classes) ===");

    // --- 1. Simple character class ---
    let re = Regex::new(r"[aeiou]").expect("invalid");
    let vowels: Vec<&str> = re.find_iter("hello world").map(|m| m.as_str()).collect();
    println!("vowels: {vowels:?}");

    // --- 2. Range ---
    let re = Regex::new(r"[a-z]+").expect("invalid");
    let lower: Vec<&str> = re
        .find_iter("Hello World 123")
        .map(|m| m.as_str())
        .collect();
    println!("lowercase: {lower:?}");

    // --- 3. Multiple ranges ---
    let re = Regex::new(r"[a-zA-Z0-9]+").expect("invalid");
    let alnum: Vec<&str> = re
        .find_iter("hello-world_42!")
        .map(|m| m.as_str())
        .collect();
    println!("alphanumeric: {alnum:?}");

    // --- 4. Negation ---
    let re = Regex::new(r"[^a-zA-Z]+").expect("invalid");
    let non_alpha: Vec<&str> = re
        .find_iter("hello, world! 42")
        .map(|m| m.as_str())
        .collect();
    println!("non-alpha: {non_alpha:?}");

    // --- 5. Special chars in brackets ---
    let re = Regex::new(r"[.!?]+").expect("invalid");
    let punct: Vec<&str> = re
        .find_iter("Hello! How? Fine.")
        .map(|m| m.as_str())
        .collect();
    println!("punctuation: {punct:?}");

    // --- 6. Hex color pattern ---
    let re = Regex::new(r"#[0-9a-fA-F]{6}\b").expect("invalid");
    let colors: Vec<&str> = re
        .find_iter("colors: #FF0000, #00ff00, #0000FF")
        .map(|m| m.as_str())
        .collect();
    println!("hex colors: {colors:?}");
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_vowels() {
        let re = Regex::new(r"[aeiou]").unwrap();
        assert_eq!(re.find_iter("hello").count(), 2); // e, o
    }

    #[test]
    fn test_negation() {
        let re = Regex::new(r"[^0-9]+").unwrap();
        let v: Vec<&str> = re.find_iter("abc123def").map(|m| m.as_str()).collect();
        assert_eq!(v, vec!["abc", "def"]);
    }

    #[test]
    fn test_hex_color() {
        let re = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
        assert!(re.is_match("#FF00AB"));
        assert!(!re.is_match("#GG00AB"));
    }
}
