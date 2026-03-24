// Demonstrates how lifetimes relate to the referents (the values being referenced).
//
// The lifetime `'a` doesn't change how long data lives — it tells the compiler
// about the RELATIONSHIP between references and the data they point to.
//
// Run: cargo run --example s19_lifetimes_referents

/// The returned reference is tied to the input's lifetime.
/// The referent (original data) must outlive the reference.
fn get_first<'a>(items: &'a [i32]) -> &'a i32 {
    &items[0]
}

/// The returned reference is tied to both inputs.
/// The result is valid for the SHORTER of the two lifetimes.
fn pick_longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

/// Struct holding a reference — it can't outlive the referent.
#[derive(Debug)]
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    /// Returns a reference tied to the same lifetime as the struct's data.
    fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}

fn main() {
    // --- 1. Slice reference tied to the vec ---
    let nums = vec![10, 20, 30];
    let first = get_first(&nums);
    println!("first: {first}");
    // first is valid as long as nums exists

    // --- 2. String references ---
    let s1 = String::from("Rust programming");
    let result;
    {
        let s2 = String::from("Go");
        result = pick_longer(&s1, &s2);
        // result is valid here because both s1 and s2 are alive
        println!("longer: {result}");
    }
    // result would be invalid here if assigned from the shorter-lived s2

    // --- 3. Struct with reference ---
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt { text: &novel };
    println!("excerpt: {excerpt:?}");
    println!("first word: {}", excerpt.first_word());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first() {
        let v = vec![1, 2, 3];
        assert_eq!(*get_first(&v), 1);
    }

    #[test]
    fn test_pick_longer() {
        assert_eq!(pick_longer("abc", "ab"), "abc");
    }

    #[test]
    fn test_excerpt() {
        let text = String::from("hello world");
        let e = Excerpt { text: &text };
        assert_eq!(e.first_word(), "hello");
    }
}
