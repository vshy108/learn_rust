// Demonstrates counting number of matches.
//
// Use `find_iter().count()` or `captures_iter().count()`.
//
// Run: cargo run --example s25_num_matches

use regex::Regex;

fn main() {
    println!("=== Number of Matches ===");

    let text = "The quick brown fox jumps over the lazy dog. The fox is fast.";

    // --- 1. Count word occurrences ---
    let re = Regex::new(r"(?i)\bthe\b").expect("invalid");
    println!("'the' appears {} times", re.find_iter(text).count());

    // --- 2. Count digits ---
    let data = "scores: 95, 87, 92, 78, 100";
    let re = Regex::new(r"\d+").expect("invalid");
    println!("digit groups: {}", re.find_iter(data).count());

    // --- 3. Count sentences (ending with . ! ?) ---
    let re = Regex::new(r"[.!?]").expect("invalid");
    println!("sentence endings: {}", re.find_iter(text).count());

    // --- 4. Count email-like patterns ---
    let emails = "Contact alice@test.com or bob@test.com. CC: charlie@test.com";
    let re = Regex::new(r"\b[\w.]+@[\w.]+\.\w+\b").expect("invalid");
    let matches: Vec<&str> = re.find_iter(emails).map(|m| m.as_str()).collect();
    println!("emails found: {} -> {matches:?}", matches.len());

    // --- 5. Count vowels ---
    let re = Regex::new(r"[aeiouAEIOU]").expect("invalid");
    println!("vowels in text: {}", re.find_iter(text).count());
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_count() {
        let re = Regex::new(r"\d+").unwrap();
        assert_eq!(re.find_iter("a1b2c3").count(), 3);
    }

    #[test]
    fn test_count_zero() {
        let re = Regex::new(r"\d+").unwrap();
        assert_eq!(re.find_iter("no digits").count(), 0);
    }
}
