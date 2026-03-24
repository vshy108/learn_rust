// Demonstrates capture groups in regex.
//
// Parentheses `()` create capture groups.
// `captures()` returns all groups, indexed by number.
// Named groups: `(?P<name>pattern)`.
//
// Run: cargo run --example s25_capture_groups

use regex::Regex;

fn main() {
    println!("=== Capture Groups ===");

    // --- 1. Basic capture ---
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").expect("invalid");
    let text = "Date: 2024-03-15";
    if let Some(caps) = re.captures(text) {
        println!("full: {}", &caps[0]);
        println!("year: {}", &caps[1]);
        println!("month: {}", &caps[2]);
        println!("day: {}", &caps[3]);
    }

    // --- 2. Named captures ---
    let re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").expect("invalid");
    if let Some(caps) = re.captures(text) {
        println!("year: {}", &caps["year"]);
        println!("month: {}", &caps["month"]);
    }

    // --- 3. Multiple captures with captures_iter ---
    let re = Regex::new(r"(\w+)=(\w+)").expect("invalid");
    let query = "name=alice&age=30&city=nyc";
    println!("key-value pairs:");
    for caps in re.captures_iter(query) {
        println!("  {} = {}", &caps[1], &caps[2]);
    }

    // --- 4. Optional group ---
    let re = Regex::new(r"(\w+)\s*(?::?\s*(\d+))?").expect("invalid");
    let entries = ["alice: 25", "bob", "charlie: 30"];
    for entry in &entries {
        if let Some(caps) = re.captures(entry) {
            let name = &caps[1];
            let age = caps.get(2).map_or("unknown", |m| m.as_str());
            println!("  {name}, age: {age}");
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_capture_date() {
        let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let caps = re.captures("2024-03-15").unwrap();
        assert_eq!(&caps[1], "2024");
        assert_eq!(&caps[2], "03");
        assert_eq!(&caps[3], "15");
    }

    #[test]
    fn test_named_capture() {
        let re = Regex::new(r"(?P<key>\w+)=(?P<val>\w+)").unwrap();
        let caps = re.captures("name=alice").unwrap();
        assert_eq!(&caps["key"], "name");
        assert_eq!(&caps["val"], "alice");
    }
}
