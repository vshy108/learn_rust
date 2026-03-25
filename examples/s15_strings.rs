// Demonstrates String types in Rust.
//
// Two main string types:
// - `String` — owned, heap-allocated, growable, UTF-8 string.
// - `&str` — borrowed string slice, a view into UTF-8 data.
//
// Run: cargo run --example s15_strings

fn main() {
    // --- 1. Creating Strings ---
    let s1 = String::new(); // empty
    let s2 = String::from("hello"); // from &str
    let s3 = "world".to_string(); // to_string() on &str
    let s4 = "rust".to_owned(); // to_owned() on &str
    println!("s1: \"{s1}\", s2: \"{s2}\", s3: \"{s3}\", s4: \"{s4}\"");

    // --- 2. &str: string slices ---
    let literal: &str = "I'm a string literal"; // &'static str
    let slice: &str = &s2[0..3]; // slice of a String: "hel"
    println!("literal: {literal}");
    println!("slice: {slice}");

    // --- 3. Strings are UTF-8 ---
    let emoji = String::from("Hello 🦀!");
    println!("emoji: {emoji}");
    println!("bytes: {}", emoji.len()); // byte count (not char count!)
    println!("chars: {}", emoji.chars().count()); // character count

    // --- 4. String vs &str in function parameters ---
    fn greet(name: &str) -> String {
        // Accept &str — works with both String and &str
        format!("Hello, {name}!")
    }
    let owned = String::from("Alice");
    println!("{}", greet(&owned)); // &String auto-deref to &str
    println!("{}", greet("Bob")); // &str directly

    // --- 5. Mutating Strings ---
    let mut s = String::from("hello");
    s.push(' '); // push a char
    s.push_str("world"); // push a &str
    println!("mutated: {s}");

    // --- 6. String indexing DOES NOT work with [] ---
    // let c = s[0]; // ERROR: String cannot be indexed by integer
    // Because UTF-8 chars can be 1-4 bytes. Indexing by byte is ambiguous.
    // Use .chars(), .bytes(), or slicing instead.

    // --- 7. chars() vs bytes() ---
    let s = "Rust 🦀";
    println!("chars: {:?}", s.chars().collect::<Vec<_>>());
    println!("bytes: {:?}", s.bytes().collect::<Vec<_>>());

    // --- 8. Conversion between String and &str ---
    let owned = String::from("hello");
    let borrowed: &str = &owned; // String → &str (cheap, just a reference)
    let re_owned: String = borrowed.to_string(); // &str → String (allocates)
    println!("owned: {owned}, borrowed: {borrowed}, re_owned: {re_owned}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_string_creation() {
        assert_eq!(String::from("hello"), "hello".to_string());
    }

    #[test]
    fn test_utf8_len() {
        let s = "🦀";
        assert_eq!(s.len(), 4); // 4 bytes
        assert_eq!(s.chars().count(), 1); // 1 character
    }

    #[test]
    fn test_push() {
        let mut s = String::from("hi");
        s.push('!');
        assert_eq!(s, "hi!");
    }

    #[test]
    fn test_slice() {
        let s = String::from("hello");
        assert_eq!(&s[0..2], "he");
    }
}
