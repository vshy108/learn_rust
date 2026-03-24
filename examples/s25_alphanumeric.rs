// Demonstrates alphanumeric character matching with regex.
//
// `\w` = word character [a-zA-Z0-9_], `\W` = non-word character.
//
// Run: cargo run --example s25_alphanumeric

use regex::Regex;

fn main() {
    println!("=== Alphanumeric Characters ===");

    let text = "user_name: john123, email: john@example.com";

    // --- 1. Word characters ---
    let re = Regex::new(r"\w+").expect("invalid");
    let words: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    println!("word chars: {words:?}");

    // --- 2. Identifier pattern ---
    let re = Regex::new(r"[a-zA-Z_]\w*").expect("invalid");
    let identifiers: Vec<&str> = re
        .find_iter("foo bar_baz 123bad _ok")
        .map(|m| m.as_str())
        .collect();
    println!("identifiers: {identifiers:?}");

    // --- 3. Only letters ---
    let re = Regex::new(r"[a-zA-Z]+").expect("invalid");
    let letters: Vec<&str> = re.find_iter("abc123def").map(|m| m.as_str()).collect();
    println!("letters only: {letters:?}");

    // --- 4. Non-word chars ---
    let re = Regex::new(r"\W+").expect("invalid");
    let non_words: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    println!("non-word: {non_words:?}");

    // --- 5. Hex pattern ---
    let re = Regex::new(r"[0-9a-fA-F]+").expect("invalid");
    let hex_vals: Vec<&str> = re
        .find_iter("color: #FF00AB, code: 0x1a2b")
        .map(|m| m.as_str())
        .collect();
    println!("hex values: {hex_vals:?}");
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_word_chars() {
        let re = Regex::new(r"\w+").unwrap();
        let v: Vec<&str> = re.find_iter("hello world").map(|m| m.as_str()).collect();
        assert_eq!(v, vec!["hello", "world"]);
    }

    #[test]
    fn test_letters_only() {
        let re = Regex::new(r"[a-zA-Z]+").unwrap();
        let v: Vec<&str> = re.find_iter("abc123def").map(|m| m.as_str()).collect();
        assert_eq!(v, vec!["abc", "def"]);
    }
}
