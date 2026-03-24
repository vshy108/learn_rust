// Demonstrates string slices as function parameters.
//
// Best practice: accept `&str` instead of `&String` in function parameters.
//
// Why? `&str` is more flexible — it works with:
// - String literals (`"hello"`) — already `&str`
// - Owned Strings (`&my_string`) — coerced to `&str` via Deref
// - String slices (`&my_string[1..4]`) — already `&str`
//
// If you accept `&String`, callers with `&str` must convert to String first.
//
// Run: cargo run --example s08_string_slices_fn_params

/// Checks if a string contains only ASCII digits.
///
/// Accepts `&str` — works with all string-like types.
///
/// # Arguments
///
/// * `s` - A string slice to check
///
/// # Returns
///
/// true if all characters are ASCII digits (0-9)
fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

/// Truncates a string to `max_len` characters, adding "..." if truncated.
///
/// # Arguments
///
/// * `s` - Input string slice
/// * `max_len` - Maximum number of characters to keep
///
/// # Returns
///
/// Truncated string as an owned String
fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len).collect();
        format!("{truncated}...")
    }
}

/// Counts occurrences of a substring.
///
/// # Arguments
///
/// * `haystack` - The text to search in
/// * `needle` - The substring to find
///
/// # Returns
///
/// Number of non-overlapping occurrences
fn count_occurrences(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

fn main() {
    // --- 1. Pass a String (auto-coerces to &str) ---
    let owned = String::from("12345");
    println!("is_numeric(owned): {}", is_numeric(&owned));

    // --- 2. Pass a string literal (already &str) ---
    println!("is_numeric(literal): {}", is_numeric("42"));
    println!("is_numeric(\"abc\"): {}", is_numeric("abc"));

    // --- 3. Pass a slice of a String ---
    let data = String::from("abc123def");
    let substr = &data[3..6]; // "123"
    println!("is_numeric(slice \"{substr}\"): {}", is_numeric(substr));

    // --- 4. Truncate examples ---
    let long = "The quick brown fox jumps over the lazy dog";
    println!("truncate(20): {}", truncate(long, 20));
    println!("truncate(100): {}", truncate(long, 100));
    println!("truncate(String): {}", truncate(&String::from("test"), 10));

    // --- 5. Count occurrences ---
    let text = "abcabcabc";
    println!("'abc' in \"{text}\": {}", count_occurrences(text, "abc"));
    println!("'xyz' in \"{text}\": {}", count_occurrences(text, "xyz"));

    // --- 6. Combining owned and borrowed in same call ---
    let greeting = String::from("Hello Hello Hello");
    let pattern = "Hello";
    println!("count: {}", count_occurrences(&greeting, pattern));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_numeric_true() {
        assert!(is_numeric("123"));
        assert!(is_numeric("0"));
    }

    #[test]
    fn test_is_numeric_false() {
        assert!(!is_numeric("12a3"));
        assert!(!is_numeric(""));
        assert!(!is_numeric("abc"));
    }

    #[test]
    fn test_truncate_short() {
        assert_eq!(truncate("hi", 10), "hi");
    }

    #[test]
    fn test_truncate_long() {
        assert_eq!(truncate("hello world", 5), "hello...");
    }

    #[test]
    fn test_count_occurrences() {
        assert_eq!(count_occurrences("aaa", "a"), 3);
        assert_eq!(count_occurrences("abab", "ab"), 2);
        assert_eq!(count_occurrences("xyz", "a"), 0);
    }
}
