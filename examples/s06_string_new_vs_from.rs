// Demonstrates String::new() vs String::from().
//
// Both produce a `String` (heap-allocated, growable UTF-8 text), but
// they differ in how they start:
//
// - `String::new()`  — creates an empty String, capacity 0.
// - `String::from(s)` — creates a String pre-filled with contents of `s`.
//
// Internally, `String` is a wrapper around `Vec<u8>` that guarantees valid
// UTF-8. It owns its data on the heap, unlike `&str` which is a borrowed
// view into text stored elsewhere (often in the binary's read-only data).
//
// Run: cargo run --example s06_string_new_vs_from

fn main() {
    // --- 1. String::new() ---
    let mut s1 = String::new();
    println!("s1: \"{s1}\", len: {}, cap: {}", s1.len(), s1.capacity());
    // len 0, capacity 0 — no heap allocation yet!

    s1.push_str("hello");
    println!(
        "s1 after push_str: \"{s1}\", len: {}, cap: {}",
        s1.len(),
        s1.capacity()
    );
    // Now the allocator gave us some capacity (at least 5).

    // --- 2. String::from() ---
    let s2 = String::from("hello");
    println!("s2: \"{s2}\", len: {}, cap: {}", s2.len(), s2.capacity());
    // Pre-filled with "hello", capacity >= 5.

    // --- 3. Equivalent: .to_string() on &str ---
    // `"hello".to_string()` does the same thing as String::from("hello").
    let s3 = "hello".to_string();
    assert_eq!(s2, s3);

    // --- 4. String::with_capacity() ---
    // If you know how much text you'll store, pre-allocate to avoid
    // re-allocations as the String grows.
    let mut s4 = String::with_capacity(100);
    println!("s4 cap before push: {}", s4.capacity()); // >= 100
    s4.push_str("preallocated");
    println!("s4: \"{s4}\", cap after push: {}", s4.capacity()); // still >= 100

    // --- 5. Key insight: ownership ---
    // `String` owns its heap buffer. When it goes out of scope, the buffer
    // is freed. `&str` only borrows — no deallocation happens.
    let owned: String = String::from("I own this");
    let borrowed: &str = &owned; // borrow as &str via Deref coercion
    println!("owned: {owned}, borrowed: {borrowed}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_is_empty() {
        let s = String::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_from_matches_content() {
        let s = String::from("rust");
        assert_eq!(s, "rust");
        assert_eq!(s.len(), 4);
    }

    #[test]
    fn test_to_string_equals_from() {
        assert_eq!("abc".to_string(), String::from("abc"));
    }

    #[test]
    fn test_with_capacity_reserves() {
        let s = String::with_capacity(50);
        assert!(s.capacity() >= 50);
        assert!(s.is_empty());
    }

    #[test]
    fn test_push_str_grows() {
        let mut s = String::new();
        s.push_str("grow");
        assert_eq!(s, "grow");
    }
}
