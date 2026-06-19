// Demonstrates the replace_all method.
//
// `re.replace(text, replacement)` replaces the first match.
// `re.replace_all(text, replacement)` replaces all matches.
// Use `$1`, `$2` etc. to refer to capture groups in replacements.
//
// Run: cargo run --example s25_replace_all

use regex::Regex;

fn main() {
    println!("=== replace_all ===");

    // --- 1. Simple replacement ---
    let re = Regex::new(r"\d+").expect("invalid");
    let result = re.replace_all("I have 3 cats and 2 dogs", "N");
    println!("replaced: {result}");

    // --- 2. Replace with capture groups ---
    // can name the group with ?<xxx>  and later represent by $xxx
    let re = Regex::new(r"(\w+)@(\w+)\.(\w+)").expect("invalid");
    let result = re.replace_all("alice@example.com bob@test.org", "$1 at $2 dot $3");
    println!("emails: {result}");

    // --- 3. Reformat dates ---
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").expect("invalid");
    let result = re.replace_all("dates: 2024-03-15 and 2024-12-25", "$2/$3/$1");
    println!("reformatted: {result}");

    // --- 4. Replace first only ---
    let re = Regex::new(r"\bfoo\b").expect("invalid");
    let result = re.replace("foo bar foo baz foo", "FOO");
    println!("first only: {result}");

    // --- 5. Sanitize input (remove special chars) ---
    let re = Regex::new(r"[^a-zA-Z0-9\s]").expect("invalid");
    let sanitized = re.replace_all("Hello! @World #Rust <script>", "");
    println!("sanitized: '{sanitized}'");

    // --- 6. Replace with closure ---
    let re = Regex::new(r"\d+").expect("invalid");
    let result = re.replace_all("prices: 10, 20, 30", |caps: &regex::Captures| {
        let n: i32 = caps[0].parse().unwrap();
        format!("${}", n * 2) // double the price
    });
    println!("doubled: {result}");
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_replace_all() {
        let re = Regex::new(r"\d+").unwrap();
        let result = re.replace_all("a1b2c3", "X");
        assert_eq!(result, "aXbXcX");
    }

    #[test]
    fn test_replace_first() {
        let re = Regex::new(r"\d+").unwrap();
        let result = re.replace("a1b2c3", "X");
        assert_eq!(result, "aXb2c3");
    }

    #[test]
    fn test_replace_with_groups() {
        let re = Regex::new(r"(\w+)-(\w+)").unwrap();
        let result = re.replace("foo-bar", "$2-$1");
        assert_eq!(result, "bar-foo");
    }

    #[test]
    fn test_replace_with_named_groups() {
        let re = Regex::new(r"(?<first>\w+)-(?<second>\w+)").unwrap();
        let result = re.replace("foo-bar", "$second-$first");
        assert_eq!(result, "bar-foo");
    }
}
