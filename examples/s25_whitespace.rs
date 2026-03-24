// Demonstrates whitespace matching with regex.
//
// `\s` matches whitespace (space, tab, newline, etc.), `\S` = non-whitespace.
//
// Run: cargo run --example s25_whitespace

use regex::Regex;

fn main() {
    println!("=== Whitespace ===");

    let text = "hello   world\tfoo\nbar";

    // --- 1. Match whitespace ---
    let re = Regex::new(r"\s+").expect("invalid");
    let ws: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    println!("whitespace segments: {}", ws.len());

    // --- 2. Split on whitespace (equivalent to split_whitespace) ---
    let re = Regex::new(r"\S+").expect("invalid");
    let words: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    println!("words: {words:?}");

    // --- 3. Normalize whitespace ---
    let re = Regex::new(r"\s+").expect("invalid");
    let normalized = re.replace_all("  hello   world  ", " ");
    println!("normalized: '{normalized}'");

    // --- 4. Match specific whitespace ---
    let re = Regex::new(r"\t").expect("invalid");
    println!("has tab? {}", re.is_match(text));

    let re = Regex::new(r"\n").expect("invalid");
    println!("has newline? {}", re.is_match(text));
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_split_words() {
        let re = Regex::new(r"\S+").unwrap();
        let words: Vec<&str> = re.find_iter("  a  b  c  ").map(|m| m.as_str()).collect();
        assert_eq!(words, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_normalize() {
        let re = Regex::new(r"\s+").unwrap();
        let result = re.replace_all("a  b   c", " ");
        assert_eq!(result, "a b c");
    }
}
