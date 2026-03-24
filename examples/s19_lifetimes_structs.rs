// Demonstrates lifetimes in structs.
//
// If a struct holds a reference, it needs a lifetime parameter.
// The struct cannot outlive the data it references.
//
// Run: cargo run --example s19_lifetimes_structs

/// A struct holding a reference — needs lifetime `'a`.
/// The struct is valid only as long as the referenced data.
#[derive(Debug)]
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn new(text: &'a str) -> Self {
        Self { text }
    }

    /// Method returning a reference — elision rule 3 applies.
    fn first_line(&self) -> &str {
        self.text.lines().next().unwrap_or("")
    }

    /// Method using the struct's lifetime explicitly.
    fn full_text(&self) -> &'a str {
        self.text
    }
}

/// Struct with multiple references — can have different lifetimes.
#[derive(Debug)]
struct Pair<'a, 'b> {
    key: &'a str,
    value: &'b str,
}

impl<'a, 'b> Pair<'a, 'b> {
    fn key(&self) -> &'a str {
        self.key
    }

    fn value(&self) -> &'b str {
        self.value
    }
}

/// Generic struct with lifetime.
#[derive(Debug)]
struct HighlightedSlice<'a, T: 'a> {
    data: &'a [T],
    label: &'a str,
}

fn main() {
    // --- 1. Basic struct with reference ---
    let novel = String::from("Call me Ishmael.\nSome years ago...");
    let excerpt = Excerpt::new(&novel);
    println!("excerpt: {excerpt:?}");
    println!("first line: {}", excerpt.first_line());
    println!("full: {}", excerpt.full_text());

    // --- 2. Struct with different lifetimes ---
    let k = String::from("language");
    let v = String::from("Rust");
    let p = Pair { key: &k, value: &v };
    println!("pair: {} = {}", p.key(), p.value());

    // --- 3. Generic with lifetime ---
    let nums = vec![10, 20, 30];
    let slice = HighlightedSlice {
        data: &nums,
        label: "numbers",
    };
    println!("{}: {:?}", slice.label, slice.data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excerpt() {
        let text = String::from("line1\nline2");
        let e = Excerpt::new(&text);
        assert_eq!(e.first_line(), "line1");
    }

    #[test]
    fn test_pair() {
        let p = Pair {
            key: "k",
            value: "v",
        };
        assert_eq!(p.key(), "k");
        assert_eq!(p.value(), "v");
    }

    #[test]
    fn test_highlighted_slice() {
        let data = vec![1, 2, 3];
        let hs = HighlightedSlice {
            data: &data,
            label: "test",
        };
        assert_eq!(hs.data.len(), 3);
    }
}
