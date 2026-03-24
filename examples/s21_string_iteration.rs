// Demonstrates String iteration.
//
// Strings don't implement indexing (s[0]) because UTF-8 characters
// can be multi-byte. Instead, use iterators:
// - `.chars()` → individual characters
// - `.bytes()` → raw bytes
// - `.char_indices()` → (byte_index, char)
// - `.lines()` → line iterator
//
// Run: cargo run --example s21_string_iteration

fn main() {
    let text = "Hello, Rust!";

    // --- 1. chars() ---
    print!("chars: ");
    for c in text.chars() {
        print!("{c} ");
    }
    println!();

    // --- 2. bytes() ---
    print!("bytes: ");
    for b in text.bytes() {
        print!("{b} ");
    }
    println!();

    // --- 3. char_indices() ---
    for (i, c) in text.char_indices() {
        println!("  [{i}] {c}");
    }

    // --- 4. Multi-byte characters ---
    let emoji = "Hi! 😊";
    println!("chars: {:?}", emoji.chars().collect::<Vec<_>>());
    println!("bytes: {} chars: {}", emoji.len(), emoji.chars().count());

    // --- 5. lines() ---
    let multiline = "line one\nline two\nline three";
    for (i, line) in multiline.lines().enumerate() {
        println!("  line {i}: {line}");
    }

    // --- 6. split and other string iterators ---
    let csv = "a,b,c,d";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("split: {parts:?}");

    let words: Vec<&str> = "  hello   world  ".split_whitespace().collect();
    println!("words: {words:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_chars() {
        let s = "abc";
        let v: Vec<char> = s.chars().collect();
        assert_eq!(v, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_lines() {
        let s = "a\nb\nc";
        assert_eq!(s.lines().count(), 3);
    }

    #[test]
    fn test_multibyte() {
        let s = "😊";
        assert_eq!(s.chars().count(), 1);
        assert_eq!(s.len(), 4); // 4 bytes in UTF-8
    }
}
