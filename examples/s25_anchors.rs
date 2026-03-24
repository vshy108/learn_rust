// Demonstrates anchors in regex.
//
// `^` matches start of string (or line with (?m)).
// `$` matches end of string (or line with (?m)).
//
// Run: cargo run --example s25_anchors

use regex::Regex;

fn main() {
    println!("=== Anchors ===");

    // --- 1. Start anchor ^ ---
    let re = Regex::new(r"^Hello").expect("invalid");
    println!(
        "'Hello world' starts with Hello: {}",
        re.is_match("Hello world")
    );
    println!(
        "'Say Hello' starts with Hello: {}",
        re.is_match("Say Hello")
    );

    // --- 2. End anchor $ ---
    let re = Regex::new(r"world$").expect("invalid");
    println!(
        "'hello world' ends with world: {}",
        re.is_match("hello world")
    );
    println!(
        "'world peace' ends with world: {}",
        re.is_match("world peace")
    );

    // --- 3. Both anchors (exact match) ---
    let re = Regex::new(r"^\d{3}-\d{4}$").expect("invalid");
    let tests = ["555-1234", "555-12345", "55-1234", "abc"];
    for t in &tests {
        println!("  '{t}' is phone format? {}", re.is_match(t));
    }

    // --- 4. Multiline mode ---
    let text = "first line\nsecond line\nthird line";
    let re = Regex::new(r"(?m)^second").expect("invalid");
    println!("multiline ^second: {}", re.is_match(text));

    let re = Regex::new(r"(?m)line$").expect("invalid");
    let count = re.find_iter(text).count();
    println!("lines ending with 'line': {count}");
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_start_anchor() {
        let re = Regex::new(r"^abc").unwrap();
        assert!(re.is_match("abcdef"));
        assert!(!re.is_match("xabc"));
    }

    #[test]
    fn test_end_anchor() {
        let re = Regex::new(r"xyz$").unwrap();
        assert!(re.is_match("abcxyz"));
        assert!(!re.is_match("xyzabc"));
    }

    #[test]
    fn test_exact_match() {
        let re = Regex::new(r"^hello$").unwrap();
        assert!(re.is_match("hello"));
        assert!(!re.is_match("hello world"));
    }
}
