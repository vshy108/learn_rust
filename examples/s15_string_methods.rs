// Demonstrates common string methods.
//
// Run: cargo run --example s15_string_methods

fn main() {
    let s = String::from("  Hello, Rust World!  ");

    // --- 1. trim, trim_start, trim_end ---
    println!("trim: \"{}\"", s.trim());
    println!("trim_start: \"{}\"", s.trim_start());
    println!("trim_end: \"{}\"", s.trim_end());

    // --- 2. to_uppercase, to_lowercase ---
    let s = "Hello World";
    println!("upper: {}", s.to_uppercase());
    println!("lower: {}", s.to_lowercase());

    // --- 3. contains, starts_with, ends_with ---
    println!("contains 'World': {}", s.contains("World"));
    println!("starts_with 'Hello': {}", s.starts_with("Hello"));
    println!("ends_with '!': {}", s.ends_with('!'));

    // --- 4. replace, replacen ---
    let s = "foo bar foo baz foo";
    println!("replace: {}", s.replace("foo", "qux"));
    println!("replacen(2): {}", s.replacen("foo", "qux", 2));

    // --- 5. split, split_whitespace ---
    let csv = "a,b,c,d";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("split: {parts:?}");

    let words: Vec<&str> = "hello   world  rust".split_whitespace().collect();
    println!("split_whitespace: {words:?}");

    // --- 6. find, rfind ---
    let s = "hello world hello";
    println!("find 'world': {:?}", s.find("world"));
    println!("rfind 'hello': {:?}", s.rfind("hello"));

    // --- 7. len, is_empty ---
    println!("len: {}", s.len());
    println!("is_empty: {}", s.is_empty());
    println!("empty str is_empty: {}", "".is_empty());

    // --- 8. chars and char_indices ---
    let s = "Rust🦀";
    for (i, c) in s.char_indices() {
        println!("  byte {i}: '{c}'");
    }

    // --- 9. truncate (in-place, by byte index) ---
    let mut s = String::from("Hello, World!");
    s.truncate(5);
    println!("truncate: {s}");

    // --- 10. repeat ---
    println!("repeat: {}", "abc".repeat(3));

    // --- 11. Checking properties ---
    println!("is_ascii: {}", "hello".is_ascii());
    println!("is_ascii: {}", "hello🦀".is_ascii());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trim() {
        assert_eq!("  hi  ".trim(), "hi");
    }

    #[test]
    fn test_replace() {
        assert_eq!("aab".replace('a', "x"), "xxb");
    }

    #[test]
    fn test_split() {
        let v: Vec<&str> = "a,b,c".split(',').collect();
        assert_eq!(v, ["a", "b", "c"]);
    }

    #[test]
    fn test_contains() {
        assert!("hello world".contains("world"));
        assert!(!"hello world".contains("xyz"));
    }

    #[test]
    fn test_to_case() {
        assert_eq!("Hello".to_uppercase(), "HELLO");
        assert_eq!("Hello".to_lowercase(), "hello");
    }
}
