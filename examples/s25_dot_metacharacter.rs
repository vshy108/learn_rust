// Demonstrates the dot metacharacter in regex.
//
// `.` matches any single character except newline.
// With `(?s)` flag, dot also matches newline.
//
// Run: cargo run --example s25_dot_metacharacter

use regex::Regex;

fn main() {
    println!("=== The Dot Metacharacter ===");

    // --- 1. Dot matches any char ---
    let re = Regex::new(r"h.t").expect("invalid");
    let words = ["hat", "hot", "hit", "hut", "ht", "heat"];
    for w in &words {
        println!("  '{w}' matches h.t? {}", re.is_match(w));
    }

    // --- 2. Dot with quantifiers ---
    let re = Regex::new(r"a.+z").expect("invalid");
    println!("'abcz' matches a.+z: {}", re.is_match("abcz"));
    println!("'az' matches a.+z: {}", re.is_match("az"));

    // --- 3. Dot doesn't match newline by default ---
    let re = Regex::new(r"a.b").expect("invalid");
    println!("'a\\nb' matches a.b: {}", re.is_match("a\nb"));

    // --- 4. With dotall flag (?s) ---
    let re = Regex::new(r"(?s)a.b").expect("invalid");
    println!("'a\\nb' with (?s): {}", re.is_match("a\nb"));

    // --- 5. Escape the dot ---
    let re = Regex::new(r"\.com").expect("invalid");
    println!(
        "'example.com' matches \\.com: {}",
        re.is_match("example.com")
    );
    println!(
        "'exampleXcom' matches \\.com: {}",
        re.is_match("exampleXcom")
    );
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_dot_any_char() {
        let re = Regex::new(r"h.t").unwrap();
        assert!(re.is_match("hat"));
        assert!(re.is_match("h9t"));
        assert!(!re.is_match("ht")); // no char between h and t
    }

    #[test]
    fn test_escaped_dot() {
        let re = Regex::new(r"\.").unwrap();
        assert!(re.is_match("a.b"));
        assert!(!re.is_match("axb"));
    }
}
