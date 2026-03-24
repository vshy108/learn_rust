// Demonstrates find and find_iter methods.
//
// `find` returns the first match, `find_iter` returns all matches.
//
// Run: cargo run --example s25_find_find_iter

use regex::Regex;

fn main() {
    println!("=== find and find_iter ===");

    let re = Regex::new(r"\b\w{5}\b").expect("invalid regex");
    let text = "The quick brown fox jumps over the lazy dogs today";

    // --- 1. find (first match) ---
    if let Some(m) = re.find(text) {
        println!("first 5-letter word: '{}'", m.as_str());
    }

    // --- 2. find_iter (all matches) ---
    println!("all 5-letter words:");
    for m in re.find_iter(text) {
        println!("  '{}' at {}..{}", m.as_str(), m.start(), m.end());
    }

    // --- 3. Collect matches ---
    let words: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    println!("collected: {words:?}");

    // --- 4. Count matches ---
    let count = re.find_iter(text).count();
    println!("count of 5-letter words: {count}");

    // --- 5. Find numbers ---
    let re = Regex::new(r"\d+").expect("invalid");
    let data = "score: 95, time: 130, bonus: 25";
    let numbers: Vec<&str> = re.find_iter(data).map(|m| m.as_str()).collect();
    println!("numbers: {numbers:?}");
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_find_iter() {
        let re = Regex::new(r"\d+").unwrap();
        let matches: Vec<&str> = re.find_iter("a1b2c3").map(|m| m.as_str()).collect();
        assert_eq!(matches, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_find_none() {
        let re = Regex::new(r"\d+").unwrap();
        assert!(re.find("no numbers").is_none());
    }
}
