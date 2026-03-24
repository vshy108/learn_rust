// Demonstrates or logic in regex with the `|` operator.
//
// `a|b` matches either `a` or `b`.
//
// Run: cargo run --example s25_or_logic

use regex::Regex;

fn main() {
    println!("=== Or Logic ===");

    // --- 1. Simple alternation ---
    let re = Regex::new(r"cat|dog|bird").expect("invalid");
    let text = "I have a cat and a dog but no bird";
    let found: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    println!("animals: {found:?}");

    // --- 2. Alternation with grouping ---
    let re = Regex::new(r"(Mr|Mrs|Ms)\.\s\w+").expect("invalid");
    let text = "Mr. Smith and Mrs. Jones met Ms. Lee";
    let found: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    println!("titles: {found:?}");

    // --- 3. File extensions ---
    let re = Regex::new(r"\.(rs|py|js|ts)$").expect("invalid");
    let files = ["main.rs", "app.py", "index.js", "readme.md", "lib.ts"];
    for f in &files {
        println!("  {f}: {}", if re.is_match(f) { "code" } else { "other" });
    }

    // --- 4. HTTP methods ---
    let re = Regex::new(r"^(GET|POST|PUT|DELETE)\s").expect("invalid");
    let requests = ["GET /api/users", "POST /api/data", "PATCH /api/items"];
    for r in &requests {
        println!("  '{r}' valid? {}", re.is_match(r));
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_alternation() {
        let re = Regex::new(r"yes|no").unwrap();
        assert!(re.is_match("say yes"));
        assert!(re.is_match("say no"));
        assert!(!re.is_match("say maybe"));
    }

    #[test]
    fn test_file_extensions() {
        let re = Regex::new(r"\.(rs|py)$").unwrap();
        assert!(re.is_match("main.rs"));
        assert!(re.is_match("app.py"));
        assert!(!re.is_match("app.js"));
    }
}
