// Demonstrates the String::retain method.
//
// `retain` takes an FnMut closure and keeps only characters for which
// the closure returns true. It modifies the string in place.
//
// Run: cargo run --example s20_string_retain

fn main() {
    println!("=== String::retain ===");

    // --- 1. Keep only alphabetic characters ---
    let mut s = String::from("h3ll0 w0rld!");
    s.retain(|c| c.is_alphabetic());
    println!("alphabetic only: {s}"); // "hllwrld"

    // --- 2. Remove whitespace ---
    let mut s = String::from("  hello   world  ");
    s.retain(|c| !c.is_whitespace());
    println!("no whitespace: {s}"); // "helloworld"

    // --- 3. Keep only digits ---
    let mut phone = String::from("+1 (555) 123-4567");
    phone.retain(|c| c.is_ascii_digit());
    println!("digits only: {phone}"); // "15551234567"

    // --- 4. FnMut: closure can track state ---
    let mut s = String::from("aabbccdd");
    let mut seen = std::collections::HashSet::new();
    s.retain(|c| seen.insert(c)); // insert returns false for duplicates
    println!("unique chars: {s}"); // "abcd"

    // --- 5. Keep only lowercase ---
    let mut s = String::from("Hello World RUST");
    s.retain(|c| c.is_lowercase() || c.is_whitespace());
    println!("lowercase: {s}"); // "ello orld "

    // --- 6. Remove specific characters ---
    let mut csv = String::from("a, b, c, d");
    csv.retain(|c| c != ' ');
    println!("no spaces: {csv}"); // "a,b,c,d"
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_retain_alpha() {
        let mut s = String::from("abc123");
        s.retain(|c| c.is_alphabetic());
        assert_eq!(s, "abc");
    }

    #[test]
    fn test_retain_digits() {
        let mut s = String::from("ph: 123-456");
        s.retain(|c| c.is_ascii_digit());
        assert_eq!(s, "123456");
    }

    #[test]
    fn test_retain_unique() {
        let mut s = String::from("aabbc");
        let mut seen = std::collections::HashSet::new();
        s.retain(|c| seen.insert(c));
        assert_eq!(s, "abc");
    }
}
