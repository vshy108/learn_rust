// Demonstrates &'static str vs &str.
//
// **&str** — a string slice with some lifetime 'a: `&'a str`.
//   It borrows text and is valid as long as the source lives.
//
// **&'static str** — a string slice that lives for the ENTIRE program.
//   - All string literals (`"hello"`) are `&'static str`.
//   - The data is embedded in the program binary at compile time.
//
// Key differences:
// | Type         | Lifetime       | Source                         |
// |-------------|----------------|--------------------------------|
// | `&str`      | Any lifetime   | Borrowed from String, slice, etc.|
// | `&'static str` | Entire program | Literal in the binary, leaked memory |
//
// Run: cargo run --example s12_static_str

/// Returns a static string — the returned reference is valid forever.
fn language() -> &'static str {
    "Rust" // string literal: &'static str
}

/// Returns a borrowed str from the input — lifetime depends on the input.
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    // --- 1. String literals are &'static str ---
    let s: &'static str = "I live forever";
    println!("{s}");

    // --- 2. Returning &'static str from a function ---
    let lang = language();
    println!("Language: {lang}");

    // --- 3. &str borrowed from a String ---
    let owned = String::from("hello world");
    let word: &str = first_word(&owned); // lifetime tied to `owned`
    println!("First word: {word}");
    // `word` must not outlive `owned`

    // --- 4. Static str can be stored anywhere ---
    let names: Vec<&'static str> = vec!["Alice", "Bob", "Charlie"];
    println!("names: {names:?}");

    // --- 5. You can "leak" a String to get &'static str ---
    // WARNING: This leaks memory! Only do this for truly permanent data.
    let owned = String::from("leaked");
    let leaked: &'static str = Box::leak(owned.into_boxed_str());
    println!("leaked: {leaked}");

    // --- 6. Error messages are often &'static str ---
    fn validate(n: i32) -> Result<i32, &'static str> {
        if n > 0 {
            Ok(n)
        } else {
            Err("must be positive") // &'static str error
        }
    }
    println!("validate(5): {:?}", validate(5));
    println!("validate(-1): {:?}", validate(-1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_is_static() {
        let s: &'static str = language();
        assert_eq!(s, "Rust");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_static_in_vec() {
        let v: Vec<&'static str> = vec!["a", "b"];
        assert_eq!(v.len(), 2);
    }
}
