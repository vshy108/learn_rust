// Demonstrates searching for digits with regex.
//
// `\d` matches a digit [0-9], `\D` matches a non-digit.
// Quantifiers: `+` (one or more), `*` (zero or more), `{n}` (exactly n).
//
// Run: cargo run --example s25_searching_digits

use regex::Regex;

fn main() {
    println!("=== Searching for Digits ===");

    let text = "Order #1234 shipped on 2024-03-15, tracking: AB5678CD";

    // --- 1. Any sequence of digits ---
    let re = Regex::new(r"\d+").expect("invalid");
    let matches: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    println!("digit sequences: {matches:?}");

    // --- 2. Exactly 4 digits ---
    let re = Regex::new(r"\b\d{4}\b").expect("invalid");
    let exact4: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    println!("exactly 4 digits: {exact4:?}");

    // --- 3. Date pattern ---
    let re = Regex::new(r"\d{4}-\d{2}-\d{2}").expect("invalid");
    if let Some(m) = re.find(text) {
        println!("date found: {}", m.as_str());
    }

    // --- 4. Non-digits ---
    let re = Regex::new(r"\D+").expect("invalid");
    let non_digits: Vec<&str> = re.find_iter("abc123def456").map(|m| m.as_str()).collect();
    println!("non-digits: {non_digits:?}");

    // --- 5. Optional digit ---
    let re = Regex::new(r"v\d?\.").expect("invalid");
    let texts = ["v1.0", "v.0", "v12.0"];
    for t in &texts {
        println!("  '{t}' matches: {}", re.is_match(t));
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_digit_sequences() {
        let re = Regex::new(r"\d+").unwrap();
        let v: Vec<&str> = re.find_iter("a1b23c456").map(|m| m.as_str()).collect();
        assert_eq!(v, vec!["1", "23", "456"]);
    }

    #[test]
    fn test_date_pattern() {
        let re = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
        assert!(re.is_match("date: 2024-03-15"));
        assert!(!re.is_match("date: 24-3-15"));
    }
}
