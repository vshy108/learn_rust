//! # Rustfinity: Borrow Toowned
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/borrow-toowned
//!
//! # Borrow and ToOwned: Owned vs Borrowed Patterns
//!
//! The `Borrow` and `ToOwned` traits provide a powerful abstraction for working with borrowed and owned data interchangeably. Unlike `AsRef` which provides simple reference conversions, `Borrow` has a stronger semantic guarantee: the borrowed form must hash and compare the same as the owned form. This makes it essential for collection types like `HashMap` and `HashSet`.
//!
//! The `ToOwned` trait is the counterpart to `Borrow` - it allows creating owned data from borrowed data. While `Clone` creates an owned copy of `&T` to `T`, `ToOwned` generalizes this to create owned data from any borrowed form (e.g., `&str` to `String`, `&[T]` to `Vec<T>`).
//!
//! ```rust
//! pub trait Borrow<Borrowed: ?Sized> {
//!     fn borrow(&self) -> &Borrowed;
//! }
//!
//! pub trait ToOwned {
//!     type Owned: Borrow<Self>;
//!     fn to_owned(&self) -> Self::Owned;
//! }
//! ```
//!
//! ## The Cow Type: Clone-on-Write
//!
//! `Cow` (Clone on Write) is a smart pointer that can hold either borrowed or owned data. It's perfect for functions that sometimes need to modify data and sometimes don't. `Cow` defers cloning until mutation is actually needed, providing both flexibility and performance.
//!
//! ```rust
//! use std::borrow::Cow;
//!
//! fn process_name(name: &str) -> Cow<str> {
//!     if name.contains(' ') {
//!         // Needs modification - return owned
//!         Cow::Owned(name.replace(' ', "_"))
//!     } else {
//!         // No modification - return borrowed
//!         Cow::Borrowed(name)
//!     }
//! }
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions and types using `Borrow`, `ToOwned`, and `Cow`:
//!
//! ### 1. Generic Lookup Function
//!
//! Implement `lookup` that searches a `HashMap` using any type that can be borrowed as the key type:
//!
//! ```rust
//! pub fn lookup<'a, K, V, Q>(
//!     map: &'a HashMap<K, V>,
//!     key: &Q
//! ) -> Option<&'a V>
//! where
//!     K: Borrow<Q> + Eq + Hash,
//!     Q: Eq + Hash + ?Sized,
//! ```
//!
//! ### 2. To Owned String
//!
//! Implement `make_owned` that converts any borrowed string-like data to an owned `String`:
//!
//! ```rust
//! pub fn make_owned(s: &str) -> String
//! ```
//!
//! ### 3. Normalize Whitespace with Cow
//!
//! Implement `normalize_whitespace` that normalizes multiple spaces to single spaces. Use `Cow` to avoid allocation if no changes are needed:
//!
//! ```rust
//! pub fn normalize_whitespace(input: &str) -> Cow<str>
//! ```
//!
//! ### 4. Ensure Prefix with Cow
//!
//! Implement `ensure_prefix` that ensures a string starts with a given prefix. Use `Cow` to return borrowed data when the prefix already exists:
//!
//! ```rust
//! pub fn ensure_prefix<'a>(
//!     s: &'a str,
//!     prefix: &str
//! ) -> Cow<'a, str>
//! ```
//!
//! ### 5. Custom Type with Borrow
//!
//! Create a `CaseInsensitiveString` type that implements `Borrow<str>` such that lookups in collections work case-insensitively:
//!
//! ```rust
//! pub struct CaseInsensitiveString(String);
//!
//! impl CaseInsensitiveString {
//!     pub fn new(s: impl Into<String>) -> Self;
//! }
//! ```
//!
//! The type should implement `Borrow<str>`, `Hash`, and `Eq` such that "Hello" and "HELLO" are considered equal.
//!
//! ### 6. Clone on Write Vector
//!
//! Implement `append_if_missing` that appends an element to a slice only if it's not already present. Use `Cow` to avoid cloning the entire vector when the element exists:
//!
//! ```rust
//! pub fn append_if_missing<'a, T>(
//!     items: &'a [T],
//!     value: T
//! ) -> Cow<'a, [T]>
//! where
//!     T: Clone + PartialEq,
//! ```
//!
//! ### 7. Process Path with Cow
//!
//! Implement `normalize_path` that removes redundant `./` from a path. Use `Cow` for efficiency:
//!
//! ```rust
//! pub fn normalize_path(path: &str) -> Cow<str>
//! ```
//!
//! ### 8. Convert Borrowed to Owned
//!
//! Implement `to_owned_vec` that demonstrates `ToOwned` by converting a slice to an owned vector:
//!
//! ```rust
//! pub fn to_owned_vec<T: Clone>(slice: &[T]) -> Vec<T>
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use borrow_toowned::*;
//! use std::collections::HashMap;
//! use std::borrow::Cow;
//!
//! // Generic lookup with Borrow
//! let mut map: HashMap<String, i32> = HashMap::new();
//! map.insert("hello".to_string(), 42);
//! // &str key for String map
//! assert_eq!(lookup(&map, "hello"), Some(&42));
//!
//! // To owned
//! let owned = make_owned("hello");
//! assert_eq!(owned, "hello".to_string());
//!
//! // Normalize whitespace - no allocation when unchanged
//! let result = normalize_whitespace("hello world");
//! assert!(matches!(result, Cow::Borrowed(_)));
//!
//! let result = normalize_whitespace("hello   world");
//! assert!(matches!(result, Cow::Owned(_)));
//! assert_eq!(result, "hello world");
//!
//! // Ensure prefix
//! let result = ensure_prefix("hello", "hello");
//! assert!(matches!(result, Cow::Borrowed(_)));
//!
//! let result = ensure_prefix("world", "hello_");
//! assert_eq!(result, "hello_world");
//!
//! // Case insensitive string in HashMap
//! let mut map: HashMap<CaseInsensitiveString, i32> =
//!     HashMap::new();
//! map.insert(
//!     CaseInsensitiveString::new("Hello"),
//!     1
//! );
//! assert_eq!(
//!     map.get(&CaseInsensitiveString::new(
//!         "HELLO"
//!     )),
//!     Some(&1)
//! );
//!
//! // Append if missing
//! let items = [1, 2, 3];
//! let result = append_if_missing(&items, 2);
//! // Already exists
//! assert!(matches!(result, Cow::Borrowed(_)));
//!
//! let result = append_if_missing(&items, 4);
//! assert_eq!(result.as_ref(), &[1, 2, 3, 4]);
//!
//! // Normalize path
//! assert_eq!(normalize_path("./foo/./bar"), "foo/bar");
//! assert!(matches!(
//!     normalize_path("foo/bar"),
//!     Cow::Borrowed(_)
//! ));
//!
//! // To owned vec
//! let slice = &[1, 2, 3];
//! let vec = to_owned_vec(slice);
//! assert_eq!(vec, vec![1, 2, 3]);
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - For `lookup`, use `HashMap::get()` which accepts
//!   any type `Q` where `K: Borrow<Q>`
//! - `ToOwned::to_owned()` is implemented for `str`
//!   (returns `String`) and `[T]` (returns `Vec<T>`)
//! - For `Cow`, use `Cow::Borrowed(data)` when no
//!   modification is needed
//! - Use `Cow::Owned(modified_data)` when you need
//!   to return modified data
//! - For `CaseInsensitiveString`, the key insight
//!   is that `Hash` and `Eq` must be based on the
//!   lowercase form
//! - When implementing `Borrow<str>` for
//!   `CaseInsensitiveString`, note that the
//!   `borrow()` method must return `&str` - this
//!   is the internal string, not the lowercase
//!   version. The equality and hashing are what
//!   make lookups work case-insensitively.
//! - `Cow<'a, str>` implements `Deref<Target = str>`,
//!   so you can use string methods directly on it
//! - For checking if modifications are needed, do a
//!   pass to detect first, then only allocate if
//!   necessary
//!
//! </details>
//!
//! Run: `cargo run --example rf_borrow_toowned`
//! Test: `cargo test --example rf_borrow_toowned`
//! Filter: `cargo test --example rf_borrow_toowned <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Looks up a value in a HashMap using any type that can be borrowed as the key type.
///
/// # Arguments
///
/// * `map` - A reference to the HashMap to search
/// * `key` - Any type that the key type can borrow as
pub fn lookup<'a, K, V, Q>(map: &'a HashMap<K, V>, key: &Q) -> Option<&'a V>
where
    K: Borrow<Q> + Eq + Hash,
    Q: Eq + Hash + ?Sized,
{
    // TODO: Use the HashMap's get method which leverages Borrow
    unimplemented!()
}

/// Converts a borrowed string slice to an owned String using ToOwned.
///
/// # Arguments
///
/// * `s` - A string slice to convert
pub fn make_owned(s: &str) -> String {
    // TODO: Use the ToOwned trait's to_owned() method
    unimplemented!()
}

/// Normalizes multiple consecutive whitespace characters to single spaces.
///
/// Uses `Cow` to avoid allocation when no changes are needed.
///
/// # Arguments
///
/// * `input` - The string to normalize
pub fn normalize_whitespace(input: &str) -> Cow<'_, str> {
    // TODO: Check if normalization is needed
    unimplemented!()
}

/// Ensures a string starts with the given prefix.
///
/// Uses `Cow` to return borrowed data when the prefix already exists.
///
/// # Arguments
///
/// * `s` - The string to check/modify
/// * `prefix` - The prefix to ensure exists
pub fn ensure_prefix<'a>(s: &'a str, prefix: &str) -> Cow<'a, str> {
    // TODO: Check if s starts with prefix
    unimplemented!()
}

/// A case-insensitive string type for use in collections.
///
/// This type implements `Borrow<str>`, `Hash`, and `Eq` such that
/// strings differing only in case are considered equal and hash the same.
#[derive(Debug, Clone)]
pub struct CaseInsensitiveString {
    original: String,
    lowercase: String,
}

impl CaseInsensitiveString {
    /// Creates a new CaseInsensitiveString from any type convertible to String.
    pub fn new(s: impl Into<String>) -> Self {
        // TODO: Store both original and lowercase versions
        unimplemented!()
    }

    /// Returns a reference to the original string.
    pub fn as_str(&self) -> &str {
        // TODO: Return reference to original string
        unimplemented!()
    }
}

impl Borrow<str> for CaseInsensitiveString {
    fn borrow(&self) -> &str {
        // TODO: Return the lowercase version for consistent hashing/comparison
        unimplemented!()
    }
}

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO: Hash the lowercase version
        unimplemented!()
    }
}

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Compare lowercase versions
        unimplemented!()
    }
}

impl Eq for CaseInsensitiveString {}

/// Appends an element to a slice only if it's not already present.
///
/// Uses `Cow` to avoid cloning when the element already exists.
///
/// # Arguments
///
/// * `items` - The slice to potentially append to
/// * `value` - The value to append if missing
pub fn append_if_missing<'a, T>(items: &'a [T], value: T) -> Cow<'a, [T]>
where
    T: Clone + PartialEq,
{
    // TODO: Check if items contains value
    unimplemented!()
}

/// Normalizes a path by removing redundant "./" components.
///
/// Uses `Cow` for efficiency - returns borrowed data when no changes needed.
///
/// # Arguments
///
/// * `path` - The path string to normalize
pub fn normalize_path(path: &str) -> Cow<'_, str> {
    // TODO: Check if path contains "./"
    unimplemented!()
}

/// Converts a borrowed slice to an owned vector using ToOwned.
///
/// # Arguments
///
/// * `slice` - A slice to convert to an owned vector
pub fn to_owned_vec<T: Clone>(slice: &[T]) -> Vec<T> {
    // TODO: Use the ToOwned trait's to_owned() method
    unimplemented!()
}

// Example usage
pub fn main() {
    // Lookup demonstration
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("hello".to_string(), 42);
    println!("Lookup 'hello': {:?}", lookup(&map, "hello"));

    // Make owned
    let owned = make_owned("borrowed");
    println!("Made owned: {}", owned);

    // Normalize whitespace
    let normalized = normalize_whitespace("hello   world");
    println!("Normalized: '{}'", normalized);

    // Ensure prefix
    let prefixed = ensure_prefix("world", "hello_");
    println!("Prefixed: {}", prefixed);

    // Case insensitive string
    let cis = CaseInsensitiveString::new("Hello");
    println!("Case insensitive: {:?}", cis);

    // Append if missing
    let items = [1, 2, 3];
    let result = append_if_missing(&items, 4);
    println!("After append: {:?}", result.as_ref());

    // Normalize path
    let path = normalize_path("./foo/./bar");
    println!("Normalized path: {}", path);

    // To owned vec
    let vec = to_owned_vec(&[1, 2, 3]);
    println!("Owned vec: {:?}", vec);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::borrow::Cow;
    use std::collections::HashMap;

    // ==================== lookup tests ====================

    #[test]
    fn test_lookup_string_key_with_str() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("hello".to_string(), 42);
        map.insert("world".to_string(), 100);

        assert_eq!(lookup(&map, "hello"), Some(&42));
        assert_eq!(lookup(&map, "world"), Some(&100));
    }

    #[test]
    fn test_lookup_not_found() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("hello".to_string(), 42);

        assert_eq!(lookup(&map, "missing"), None);
    }

    #[test]
    fn test_lookup_empty_map() {
        let map: HashMap<String, i32> = HashMap::new();
        assert_eq!(lookup(&map, "anything"), None);
    }

    #[test]
    fn test_lookup_with_string_key() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("hello".to_string(), 42);

        let key = String::from("hello");
        assert_eq!(lookup(&map, key.as_str()), Some(&42));
    }

    #[test]
    fn test_lookup_with_empty_string() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("".to_string(), 0);

        assert_eq!(lookup(&map, ""), Some(&0));
    }

    #[test]
    fn test_lookup_unicode_keys() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("\u{1F44B}".to_string(), 1); // 👋 waving hand
        map.insert("\u{1F600}".to_string(), 2); // 😀 grinning face

        assert_eq!(lookup(&map, "\u{1F44B}"), Some(&1));
        assert_eq!(lookup(&map, "\u{1F600}"), Some(&2));
    }

    // ==================== make_owned tests ====================

    #[test]
    fn test_make_owned_simple() {
        let result = make_owned("hello");
        assert_eq!(result, String::from("hello"));
    }

    #[test]
    fn test_make_owned_empty() {
        let result = make_owned("");
        assert_eq!(result, String::new());
    }

    #[test]
    fn test_make_owned_unicode() {
        let result = make_owned("Hello, ");
        assert_eq!(result, String::from("Hello, "));
    }

    #[test]
    fn test_make_owned_whitespace() {
        let result = make_owned("  hello  ");
        assert_eq!(result, String::from("  hello  "));
    }

    #[test]
    fn test_make_owned_newlines() {
        let result = make_owned("line1\nline2\nline3");
        assert_eq!(result, String::from("line1\nline2\nline3"));
    }

    // ==================== normalize_whitespace tests ====================

    #[test]
    fn test_normalize_whitespace_no_change_needed() {
        let result = normalize_whitespace("hello world");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_normalize_whitespace_double_spaces() {
        let result = normalize_whitespace("hello  world");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_normalize_whitespace_multiple_spaces() {
        let result = normalize_whitespace("hello     world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_normalize_whitespace_tabs() {
        let result = normalize_whitespace("hello\t\tworld");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_normalize_whitespace_mixed() {
        let result = normalize_whitespace("hello \t \n world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_normalize_whitespace_empty() {
        let result = normalize_whitespace("");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "");
    }

    #[test]
    fn test_normalize_whitespace_single_word() {
        let result = normalize_whitespace("hello");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_normalize_whitespace_leading_trailing() {
        let result = normalize_whitespace("  hello  world  ");
        // This normalizes multiple spaces
        assert!(result.contains("hello world"));
    }

    #[test]
    fn test_normalize_whitespace_all_spaces() {
        let result = normalize_whitespace("     ");
        assert_eq!(result.trim(), "");
    }

    // ==================== ensure_prefix tests ====================

    #[test]
    fn test_ensure_prefix_already_has_prefix() {
        let result = ensure_prefix("hello_world", "hello_");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "hello_world");
    }

    #[test]
    fn test_ensure_prefix_needs_prefix() {
        let result = ensure_prefix("world", "hello_");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "hello_world");
    }

    #[test]
    fn test_ensure_prefix_empty_string() {
        let result = ensure_prefix("", "prefix_");
        assert_eq!(result, "prefix_");
    }

    #[test]
    fn test_ensure_prefix_empty_prefix() {
        let result = ensure_prefix("hello", "");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_ensure_prefix_partial_match() {
        let result = ensure_prefix("helloworld", "hello_");
        // "helloworld" does not start with "hello_"
        assert_eq!(result, "hello_helloworld");
    }

    #[test]
    fn test_ensure_prefix_exact_match() {
        let result = ensure_prefix("prefix", "prefix");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_ensure_prefix_unicode() {
        let result = ensure_prefix("world", "_");
        assert_eq!(result, "_world");
    }

    // ==================== CaseInsensitiveString tests ====================

    #[test]
    fn test_case_insensitive_equality() {
        let a = CaseInsensitiveString::new("Hello");
        let b = CaseInsensitiveString::new("HELLO");
        let c = CaseInsensitiveString::new("hello");

        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, c);
    }

    #[test]
    fn test_case_insensitive_not_equal() {
        let a = CaseInsensitiveString::new("Hello");
        let b = CaseInsensitiveString::new("World");

        assert_ne!(a, b);
    }

    #[test]
    fn test_case_insensitive_hash_map() {
        let mut map: HashMap<CaseInsensitiveString, i32> = HashMap::new();
        map.insert(CaseInsensitiveString::new("Hello"), 42);

        assert_eq!(map.get(&CaseInsensitiveString::new("Hello")), Some(&42));
        assert_eq!(map.get(&CaseInsensitiveString::new("HELLO")), Some(&42));
        assert_eq!(map.get(&CaseInsensitiveString::new("hello")), Some(&42));
        assert_eq!(map.get(&CaseInsensitiveString::new("HeLLo")), Some(&42));
    }

    #[test]
    fn test_case_insensitive_hash_map_overwrite() {
        let mut map: HashMap<CaseInsensitiveString, i32> = HashMap::new();
        map.insert(CaseInsensitiveString::new("hello"), 1);
        map.insert(CaseInsensitiveString::new("HELLO"), 2);

        // Should only have one entry
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&CaseInsensitiveString::new("hello")), Some(&2));
    }

    #[test]
    fn test_case_insensitive_as_str() {
        let s = CaseInsensitiveString::new("Hello World");
        assert_eq!(s.as_str(), "Hello World");
    }

    #[test]
    fn test_case_insensitive_empty() {
        let a = CaseInsensitiveString::new("");
        let b = CaseInsensitiveString::new("");
        assert_eq!(a, b);
    }

    #[test]
    fn test_case_insensitive_unicode() {
        // Note: case insensitivity for Unicode follows Rust's to_lowercase
        let a = CaseInsensitiveString::new("STRA\u{00DF}E"); // German eszett
        let b = CaseInsensitiveString::new("stra\u{00DF}e");
        assert_eq!(a, b);
    }

    // ==================== append_if_missing tests ====================

    #[test]
    fn test_append_if_missing_already_exists() {
        let items = [1, 2, 3];
        let result = append_if_missing(&items, 2);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result.as_ref(), &[1, 2, 3]);
    }

    #[test]
    fn test_append_if_missing_not_present() {
        let items = [1, 2, 3];
        let result = append_if_missing(&items, 4);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_append_if_missing_empty_slice() {
        let items: [i32; 0] = [];
        let result = append_if_missing(&items, 1);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[1]);
    }

    #[test]
    fn test_append_if_missing_first_element() {
        let items = [1, 2, 3];
        let result = append_if_missing(&items, 1);
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_append_if_missing_last_element() {
        let items = [1, 2, 3];
        let result = append_if_missing(&items, 3);
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_append_if_missing_strings() {
        let items = ["a", "b", "c"];
        let result = append_if_missing(&items, "d");
        assert_eq!(result.as_ref(), &["a", "b", "c", "d"]);

        let result = append_if_missing(&items, "b");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_append_if_missing_single_element() {
        let items = [42];
        let result = append_if_missing(&items, 42);
        assert!(matches!(result, Cow::Borrowed(_)));

        let result = append_if_missing(&items, 0);
        assert_eq!(result.as_ref(), &[42, 0]);
    }

    // ==================== normalize_path tests ====================

    #[test]
    fn test_normalize_path_no_dots() {
        let result = normalize_path("foo/bar");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "foo/bar");
    }

    #[test]
    fn test_normalize_path_leading_dot_slash() {
        let result = normalize_path("./foo/bar");
        assert_eq!(result, "foo/bar");
    }

    #[test]
    fn test_normalize_path_middle_dot_slash() {
        let result = normalize_path("foo/./bar");
        assert_eq!(result, "foo/bar");
    }

    #[test]
    fn test_normalize_path_multiple_dot_slashes() {
        let result = normalize_path("./foo/./bar/./baz");
        assert_eq!(result, "foo/bar/baz");
    }

    #[test]
    fn test_normalize_path_trailing_dot_slash() {
        let result = normalize_path("foo/bar/.");
        // The trailing . is not "./" so it might be preserved
        // But if followed by nothing, behavior depends on implementation
        assert!(result.starts_with("foo/bar"));
    }

    #[test]
    fn test_normalize_path_only_dot_slash() {
        let result = normalize_path("./");
        // Should normalize to empty or minimal path
        assert!(result.is_empty() || result == ".");
    }

    #[test]
    fn test_normalize_path_empty() {
        let result = normalize_path("");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "");
    }

    #[test]
    fn test_normalize_path_just_filename() {
        let result = normalize_path("file.txt");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "file.txt");
    }

    #[test]
    fn test_normalize_path_consecutive_dot_slashes() {
        let result = normalize_path("././foo");
        assert_eq!(result, "foo");
    }

    // ==================== to_owned_vec tests ====================

    #[test]
    fn test_to_owned_vec_integers() {
        let slice = &[1, 2, 3, 4, 5];
        let vec = to_owned_vec(slice);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_to_owned_vec_empty() {
        let slice: &[i32] = &[];
        let vec = to_owned_vec(slice);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_to_owned_vec_strings() {
        let slice = &["a".to_string(), "b".to_string()];
        let vec = to_owned_vec(slice);
        assert_eq!(vec, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_to_owned_vec_str_slices() {
        let slice = &["hello", "world"];
        let vec = to_owned_vec(slice);
        assert_eq!(vec, vec!["hello", "world"]);
    }

    #[test]
    fn test_to_owned_vec_single_element() {
        let slice = &[42];
        let vec = to_owned_vec(slice);
        assert_eq!(vec, vec![42]);
    }

    #[test]
    fn test_to_owned_vec_chars() {
        let slice = &['a', 'b', 'c'];
        let vec = to_owned_vec(slice);
        assert_eq!(vec, vec!['a', 'b', 'c']);
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_integration_lookup_with_owned_and_borrowed() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());

        // Look up with borrowed str
        assert_eq!(lookup(&map, "key1"), Some(&"value1".to_string()));

        // Look up with String converted to str
        let owned_key = String::from("key2");
        assert_eq!(lookup(&map, &owned_key[..]), Some(&"value2".to_string()));
    }

    #[test]
    fn test_integration_cow_modifications() {
        // Test chaining Cow operations
        let input = "  hello   world  ";
        let normalized = normalize_whitespace(input);

        // Further process the result
        let final_result: String = normalized.trim().to_string();
        assert_eq!(final_result, "hello world");
    }

    #[test]
    fn test_integration_ensure_prefix_chain() {
        let base = "config";
        let result = ensure_prefix(base, "app_");
        assert_eq!(result, "app_config");

        // Using the result of ensure_prefix
        let final_path = format!("{}.yaml", result);
        assert_eq!(final_path, "app_config.yaml");
    }

    #[test]
    fn test_integration_case_insensitive_word_count() {
        let mut word_count: HashMap<CaseInsensitiveString, i32> = HashMap::new();

        let words = ["Hello", "HELLO", "hello", "World", "WORLD"];
        for word in &words {
            *word_count
                .entry(CaseInsensitiveString::new(*word))
                .or_insert(0) += 1;
        }

        assert_eq!(word_count.len(), 2);
        assert_eq!(
            word_count.get(&CaseInsensitiveString::new("hello")),
            Some(&3)
        );
        assert_eq!(
            word_count.get(&CaseInsensitiveString::new("world")),
            Some(&2)
        );
    }

    #[test]
    fn test_integration_append_multiple() {
        let mut items = vec![1, 2, 3];
        let values_to_add = [2, 3, 4, 5];

        for &v in &values_to_add {
            match append_if_missing(&items, v) {
                Cow::Borrowed(_) => {
                    // Value already exists, do nothing
                }
                Cow::Owned(new_vec) => {
                    items = new_vec;
                }
            }
        }

        assert_eq!(items, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_integration_path_normalization_with_prefix() {
        let path = "./src/./lib.rs";
        let normalized = normalize_path(path);
        assert_eq!(normalized, "src/lib.rs");

        // Ensure it has a prefix
        let prefixed = ensure_prefix(&normalized, "/home/user/");
        assert_eq!(prefixed, "/home/user/src/lib.rs");
    }

    #[test]
    fn test_integration_to_owned_and_modify() {
        let original = &[1, 2, 3];
        let mut owned = to_owned_vec(original);
        owned.push(4);
        owned.push(5);

        assert_eq!(owned, vec![1, 2, 3, 4, 5]);
        assert_eq!(original, &[1, 2, 3]); // Original unchanged
    }

    #[test]
    fn test_borrow_trait_semantics() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Verify that CaseInsensitiveString properly implements Borrow
        let cis = CaseInsensitiveString::new("Hello");

        // Hash of the CaseInsensitiveString
        let mut hasher1 = DefaultHasher::new();
        cis.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        // Hash of another CaseInsensitiveString with different case
        let cis2 = CaseInsensitiveString::new("HELLO");
        let mut hasher2 = DefaultHasher::new();
        cis2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        // They should have the same hash
        assert_eq!(hash1, hash2);
    }
}
