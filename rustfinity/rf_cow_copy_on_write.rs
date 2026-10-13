//! # Rustfinity: Cow Copy On Write
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/cow-copy-on-write
//!
//! # Cow: Copy-on-Write Smart Pointer
//!
//! `Cow` (Clone-on-Write) is a smart pointer that provides efficient handling of borrowed vs. owned data. It's defined as:
//!
//! ```rust
//! pub enum Cow<'a, B: ?Sized + ToOwned> {
//!     Borrowed(&'a B),
//!     Owned(<B as ToOwned>::Owned),
//! }
//! ```
//!
//! The key insight is that `Cow` allows you to work with data that might be borrowed or owned, and only clones when mutation is actually needed. This is a powerful optimization pattern when:
//!
//! - You often receive data that doesn't need modification
//! - Cloning is expensive (large strings, vectors)
//! - You want a unified API that handles both borrowed and owned data
//!
//! ## Key Operations
//!
//! - `Cow::Borrowed(data)` - Wrap borrowed data
//! - `Cow::Owned(data)` - Wrap owned data
//! - `cow.to_mut()` - Get a mutable reference, cloning if borrowed
//! - `cow.into_owned()` - Convert to owned, cloning if borrowed
//! - `Cow::from(data)` - Create from &str, String, &[`T`], Vec`<T>`, etc.
//!
//! ## When to Use Cow
//!
//! 1. **Functions returning conditionally modified data**: Return borrowed when unchanged, owned when modified
//! 2. **APIs accepting both borrowed and owned**: Single function handles &str and String
//! 3. **Caching/memoization**: Store results that might be borrowed or computed
//! 4. **Configuration defaults**: Use default (borrowed) unless overridden (owned)
//!
//! ## Your Task
//!
//! Implement the following functions demonstrating `Cow` patterns:
//!
//! ### Part 1: Basic Cow Usage
//!
//! 1. **`maybe_uppercase`** - Convert to uppercase
//!    only if any lowercase letters exist
//!    - Returns `Cow::Borrowed` if already all
//!      uppercase (or empty)
//!    - Returns `Cow::Owned` if conversion was needed
//!
//! 2. **`ensure_suffix`** - Ensure string ends with suffix
//!    - Returns `Cow::Borrowed` if suffix already present
//!    - Returns `Cow::Owned` with suffix appended otherwise
//!
//! 3. **`trim_and_lowercase`** - Trim whitespace and
//!    convert to lowercase
//!    - Returns `Cow::Borrowed` only if string is
//!      already trimmed and lowercase
//!    - Returns `Cow::Owned` otherwise
//!
//! ### Part 2: Cow with Collections
//!
//! 4. **`remove_zeros`** - Remove all zeros from a slice
//!    - Returns `Cow::Borrowed` if no zeros exist
//!    - Returns `Cow::Owned` with zeros filtered out
//!
//! 5. **`deduplicate_sorted`** - Remove consecutive duplicates from sorted slice
//!    - Returns `Cow::Borrowed` if no duplicates
//!    - Returns `Cow::Owned` with duplicates removed
//!
//! 6. **`clamp_values`** - Clamp all values to a range [min, max]
//!    - Returns `Cow::Borrowed` if all values already in range
//!    - Returns `Cow::Owned` with clamped values
//!
//! ### Part 3: Modifying Cow with to_mut()
//!
//! 7. **`ensure_capacity`** - Ensure string has at
//!    least N characters, padding with char if needed
//!    - Uses `to_mut()` to modify only when necessary
//!    - Returns the modified/unmodified Cow
//!
//! 8. **`modify_if_needed`** - Apply a transformation
//!    function only if predicate returns true
//!    - Generic over the transformation
//!    - Uses `to_mut()` for efficient in-place
//!      modification when already owned
//!
//! ### Part 4: Cow in Data Structures
//!
//! 9. **`TextProcessor`** - A struct demonstrating
//!    Cow in a real-world scenario
//!    - Stores text as `Cow<'a, str>`
//!    - `new(text: &'a str)` - Create with borrowed
//!      text
//!    - `from_owned(text: String)` - Create with
//!      owned text
//!    - `process(&mut self)` - Apply processing
//!      (trim + normalize whitespace)
//!    - `into_string(self)` - Convert to String
//!    - `is_borrowed(&self)` - Check if currently
//!      borrowing
//!
//! ## Examples
//!
//! ```rust
//! use cow_copy_on_write::*;
//! use std::borrow::Cow;
//!
//! // Basic string operations
//! let result = maybe_uppercase("HELLO");
//! // Already uppercase
//! assert!(matches!(result, Cow::Borrowed(_)));
//!
//! let result = maybe_uppercase("Hello");
//! assert_eq!(result, "HELLO"); // Needed conversion
//!
//! // Suffix handling
//! let result = ensure_suffix("file.txt", ".txt");
//! // Already has suffix
//! assert!(matches!(result, Cow::Borrowed(_)));
//!
//! let result = ensure_suffix("file", ".txt");
//! assert_eq!(result, "file.txt");
//!
//! // Collection operations
//! let data = [1, 2, 3, 4];
//! let result = remove_zeros(&data);
//! assert!(matches!(result, Cow::Borrowed(_))); // No zeros
//!
//! let data = [0, 1, 0, 2];
//! let result = remove_zeros(&data);
//! assert_eq!(result.as_ref(), &[1, 2]);
//!
//! // Using to_mut() for efficient modification
//! let cow: Cow<str> = Cow::Borrowed("hi");
//! let result = ensure_capacity(cow, 5, '!');
//! assert_eq!(result, "hi!!!");
//!
//! // TextProcessor
//! let mut processor = TextProcessor::new("  hello   world  ");
//! assert!(processor.is_borrowed());
//! processor.process();
//! // Now owned after processing
//! assert!(!processor.is_borrowed());
//! assert_eq!(processor.into_string(), "hello world");
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `matches!(cow, Cow::Borrowed(_))` to check if borrowed
//! - `to_mut()` returns `&mut B::Owned` and clones if currently borrowed
//! - `into_owned()` consumes the Cow and returns
//!   owned data
//! - For strings: `Cow<'a, str>` with
//!   `Cow::Borrowed(&str)` and `Cow::Owned(String)`
//! - For slices: `Cow<'a, [T]>` with
//!   `Cow::Borrowed(&[T])` and `Cow::Owned(Vec<T>)`
//! - Check conditions BEFORE creating Cow to
//!   decide borrowed vs owned
//! - The `Cow::from()` trait creates
//!   `Cow::Borrowed` from references and
//!   `Cow::Owned` from owned types
//! - When returning Cow from a function, the
//!   lifetime ties to the input reference
//!
//! </details>
//!
//! Run: `cargo run --example rf_cow_copy_on_write`
//! Test: `cargo test --example rf_cow_copy_on_write`
//! Filter: `cargo test --example rf_cow_copy_on_write <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::borrow::Cow;

// =============================================================================
// Part 1: Basic Cow Usage
// =============================================================================

/// Converts a string to uppercase only if it contains lowercase letters.
///
/// Returns `Cow::Borrowed` if the string is already all uppercase (or empty),
/// `Cow::Owned` if conversion was needed.
pub fn maybe_uppercase(s: &str) -> Cow<'_, str> {
    // TODO: Check if the string contains any lowercase letters
    // If yes, return Cow::Owned with the uppercase version
    // If no, return Cow::Borrowed with the original string
    unimplemented!()
}

/// Ensures a string ends with the given suffix.
///
/// Returns `Cow::Borrowed` if the suffix is already present,
/// `Cow::Owned` with suffix appended otherwise.
pub fn ensure_suffix<'a>(s: &'a str, suffix: &str) -> Cow<'a, str> {
    // TODO: Check if string already ends with suffix
    // If yes, return Cow::Borrowed
    // If no, return Cow::Owned with suffix appended
    unimplemented!()
}

/// Trims whitespace and converts to lowercase.
///
/// Returns `Cow::Borrowed` only if the string is already trimmed and lowercase,
/// `Cow::Owned` otherwise.
pub fn trim_and_lowercase(s: &str) -> Cow<'_, str> {
    // TODO: Check if the string needs trimming or lowercasing
    // Only return Cow::Borrowed if absolutely no change is needed
    // Otherwise return Cow::Owned with trimmed and lowercased string
    unimplemented!()
}

// =============================================================================
// Part 2: Cow with Collections
// =============================================================================

/// Removes all zeros from a slice.
///
/// Returns `Cow::Borrowed` if no zeros exist,
/// `Cow::Owned` with zeros filtered out.
pub fn remove_zeros(values: &[i32]) -> Cow<'_, [i32]> {
    // TODO: Check if the slice contains any zeros
    // If no zeros, return Cow::Borrowed
    // If has zeros, return Cow::Owned with zeros filtered out
    unimplemented!()
}

/// Removes consecutive duplicates from a sorted slice.
///
/// Returns `Cow::Borrowed` if no consecutive duplicates exist,
/// `Cow::Owned` with duplicates removed.
pub fn deduplicate_sorted<T: Clone + PartialEq>(values: &[T]) -> Cow<'_, [T]> {
    // TODO: Check if there are any consecutive duplicates
    // If no duplicates, return Cow::Borrowed
    // If has duplicates, return Cow::Owned with duplicates removed
    unimplemented!()
}

/// Clamps all values to a range [min, max].
///
/// Returns `Cow::Borrowed` if all values are already in range,
/// `Cow::Owned` with clamped values.
pub fn clamp_values(values: &[i32], min: i32, max: i32) -> Cow<'_, [i32]> {
    // TODO: Check if any values are outside the range
    // If all in range, return Cow::Borrowed
    // If some out of range, return Cow::Owned with clamped values
    unimplemented!()
}

// =============================================================================
// Part 3: Modifying Cow with to_mut()
// =============================================================================

/// Ensures a string has at least `min_len` characters, padding with `pad_char` if needed.
///
/// Uses `to_mut()` to modify efficiently.
pub fn ensure_capacity<'a>(mut s: Cow<'a, str>, min_len: usize, pad_char: char) -> Cow<'a, str> {
    // TODO: Check current character count
    // If less than min_len, use s.to_mut() to get mutable access and push pad_char enough times
    // Return the (potentially modified) Cow
    unimplemented!()
}

/// Applies a transformation function only if the predicate returns true.
///
/// Uses `to_mut()` for efficient in-place modification when already owned.
pub fn modify_if_needed<'a, P, T>(mut s: Cow<'a, str>, predicate: P, transform: T) -> Cow<'a, str>
where
    P: FnOnce(&str) -> bool,
    T: Fn(char) -> char,
{
    // TODO: Check if predicate returns true for the string
    // If yes, transform all characters and use to_mut() to update
    // Return the (potentially modified) Cow
    unimplemented!()
}

// =============================================================================
// Part 4: Cow in Data Structures
// =============================================================================

/// A text processor that demonstrates Cow usage in a struct.
#[derive(Debug, Clone)]
pub struct TextProcessor<'a> {
    text: Cow<'a, str>,
}

impl<'a> TextProcessor<'a> {
    /// Creates a new TextProcessor with borrowed text.
    pub fn new(text: &'a str) -> Self {
        // TODO: Create TextProcessor with Cow::Borrowed
        unimplemented!()
    }

    /// Creates a new TextProcessor with owned text.
    pub fn from_owned(text: String) -> Self {
        // TODO: Create TextProcessor with Cow::Owned
        unimplemented!()
    }

    /// Processes the text by trimming and normalizing whitespace.
    ///
    /// Multiple consecutive whitespace characters are collapsed to single spaces.
    pub fn process(&mut self) {
        // TODO: Trim the text and collapse multiple whitespace to single spaces
        // Only modify if needed
        // Update self.text to Cow::Owned if changes were made
        unimplemented!()
    }

    /// Returns a reference to the current text.
    pub fn as_str(&self) -> &str {
        // TODO: Return a reference to the inner text
        unimplemented!()
    }

    /// Converts the processor into a String.
    pub fn into_string(self) -> String {
        // TODO: Use into_owned() to convert to String
        unimplemented!()
    }

    /// Checks if the text is currently borrowed.
    pub fn is_borrowed(&self) -> bool {
        // TODO: Return true if self.text is Cow::Borrowed
        // Hint: use matches! macro
        unimplemented!()
    }

    /// Appends text to the processor.
    ///
    /// This will convert to owned if currently borrowed.
    pub fn append(&mut self, text: &str) {
        // TODO: Use to_mut() to get mutable access and append
        unimplemented!()
    }

    /// Gets the length of the text.
    pub fn len(&self) -> usize {
        // TODO: Return the byte length of the text
        unimplemented!()
    }

    /// Checks if the text is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Return true if text is empty
        unimplemented!()
    }
}

impl Default for TextProcessor<'_> {
    fn default() -> Self {
        // TODO: Return a TextProcessor with an empty owned String
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    // Part 1: Basic Cow usage
    let result = maybe_uppercase("HELLO");
    println!("maybe_uppercase(HELLO): {:?}", result);

    let result = maybe_uppercase("Hello");
    println!("maybe_uppercase(Hello): {:?}", result);

    let result = ensure_suffix("file.txt", ".txt");
    println!("ensure_suffix already has: {:?}", result);

    let result = ensure_suffix("file", ".txt");
    println!("ensure_suffix needs: {:?}", result);

    // Part 2: Collections
    let data = [1, 2, 3, 4];
    let result = remove_zeros(&data);
    println!("remove_zeros (no zeros): {:?}", result);

    let data = [0, 1, 0, 2];
    let result = remove_zeros(&data);
    println!("remove_zeros (has zeros): {:?}", result);

    // Part 3: to_mut()
    let cow: Cow<str> = Cow::Borrowed("hi");
    let result = ensure_capacity(cow, 5, '!');
    println!("ensure_capacity: {:?}", result);

    // Part 4: TextProcessor
    let mut processor = TextProcessor::new("  hello   world  ");
    println!("Is borrowed: {}", processor.is_borrowed());
    processor.process();
    println!("After process: {}", processor.as_str());
    println!("Is borrowed: {}", processor.is_borrowed());
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::borrow::Cow;

    // =============================================================================
    // Part 1: Basic Cow Usage - maybe_uppercase
    // =============================================================================

    #[test]
    fn maybe_uppercase_already_uppercase() {
        let result = maybe_uppercase("HELLO");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn maybe_uppercase_all_lowercase() {
        let result = maybe_uppercase("hello");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn maybe_uppercase_mixed_case() {
        let result = maybe_uppercase("HeLLo");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn maybe_uppercase_empty() {
        let result = maybe_uppercase("");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "");
    }

    #[test]
    fn maybe_uppercase_numbers_only() {
        let result = maybe_uppercase("12345");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "12345");
    }

    #[test]
    fn maybe_uppercase_special_chars() {
        let result = maybe_uppercase("!@#$%");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "!@#$%");
    }

    #[test]
    fn maybe_uppercase_unicode() {
        let result = maybe_uppercase("ÜBER");
        assert!(matches!(result, Cow::Borrowed(_)));

        let result = maybe_uppercase("über");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "ÜBER");
    }

    #[test]
    fn maybe_uppercase_single_lowercase() {
        let result = maybe_uppercase("HELLo");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "HELLO");
    }

    // =============================================================================
    // Part 1: Basic Cow Usage - ensure_suffix
    // =============================================================================

    #[test]
    fn ensure_suffix_already_present() {
        let result = ensure_suffix("file.txt", ".txt");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "file.txt");
    }

    #[test]
    fn ensure_suffix_needs_suffix() {
        let result = ensure_suffix("file", ".txt");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "file.txt");
    }

    #[test]
    fn ensure_suffix_empty_string() {
        let result = ensure_suffix("", ".txt");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, ".txt");
    }

    #[test]
    fn ensure_suffix_empty_suffix() {
        let result = ensure_suffix("file.txt", "");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "file.txt");
    }

    #[test]
    fn ensure_suffix_partial_match() {
        let result = ensure_suffix("file.tx", ".txt");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "file.tx.txt");
    }

    #[test]
    fn ensure_suffix_longer_suffix() {
        let result = ensure_suffix("txt", "file.txt");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "txtfile.txt");
    }

    #[test]
    fn ensure_suffix_multiple_occurrences() {
        let result = ensure_suffix(".txt.txt", ".txt");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn ensure_suffix_slash() {
        let result = ensure_suffix("/path/to/dir", "/");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "/path/to/dir/");

        let result = ensure_suffix("/path/to/dir/", "/");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    // =============================================================================
    // Part 1: Basic Cow Usage - trim_and_lowercase
    // =============================================================================

    #[test]
    fn trim_and_lowercase_no_change_needed() {
        let result = trim_and_lowercase("hello");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "hello");
    }

    #[test]
    fn trim_and_lowercase_needs_trimming() {
        let result = trim_and_lowercase("  hello  ");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "hello");
    }

    #[test]
    fn trim_and_lowercase_needs_lowercasing() {
        let result = trim_and_lowercase("HELLO");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "hello");
    }

    #[test]
    fn trim_and_lowercase_needs_both() {
        let result = trim_and_lowercase("  HELLO  ");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "hello");
    }

    #[test]
    fn trim_and_lowercase_empty() {
        let result = trim_and_lowercase("");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "");
    }

    #[test]
    fn trim_and_lowercase_whitespace_only() {
        let result = trim_and_lowercase("   ");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "");
    }

    #[test]
    fn trim_and_lowercase_leading_whitespace() {
        let result = trim_and_lowercase("  hello");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "hello");
    }

    #[test]
    fn trim_and_lowercase_trailing_whitespace() {
        let result = trim_and_lowercase("hello  ");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "hello");
    }

    #[test]
    fn trim_and_lowercase_unicode() {
        let result = trim_and_lowercase("  ÜBER  ");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "über");
    }

    #[test]
    fn trim_and_lowercase_mixed_content() {
        let result = trim_and_lowercase("HeLLo WoRLD");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "hello world");
    }

    // =============================================================================
    // Part 2: Cow with Collections - remove_zeros
    // =============================================================================

    #[test]
    fn remove_zeros_no_zeros() {
        let data = [1, 2, 3, 4, 5];
        let result = remove_zeros(&data);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result.as_ref(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn remove_zeros_has_zeros() {
        let data = [0, 1, 0, 2, 0];
        let result = remove_zeros(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[1, 2]);
    }

    #[test]
    fn remove_zeros_all_zeros() {
        let data = [0, 0, 0];
        let result = remove_zeros(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert!(result.is_empty());
    }

    #[test]
    fn remove_zeros_empty() {
        let data: [i32; 0] = [];
        let result = remove_zeros(&data);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert!(result.is_empty());
    }

    #[test]
    fn remove_zeros_single_zero() {
        let data = [0];
        let result = remove_zeros(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert!(result.is_empty());
    }

    #[test]
    fn remove_zeros_single_nonzero() {
        let data = [42];
        let result = remove_zeros(&data);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result.as_ref(), &[42]);
    }

    #[test]
    fn remove_zeros_negative_values() {
        let data = [-1, 0, 1, -2, 0, 2];
        let result = remove_zeros(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[-1, 1, -2, 2]);
    }

    #[test]
    fn remove_zeros_leading_zero() {
        let data = [0, 1, 2, 3];
        let result = remove_zeros(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[1, 2, 3]);
    }

    #[test]
    fn remove_zeros_trailing_zero() {
        let data = [1, 2, 3, 0];
        let result = remove_zeros(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[1, 2, 3]);
    }

    // =============================================================================
    // Part 2: Cow with Collections - deduplicate_sorted
    // =============================================================================

    #[test]
    fn deduplicate_sorted_no_duplicates() {
        let data = [1, 2, 3, 4, 5];
        let result = deduplicate_sorted(&data);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result.as_ref(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn deduplicate_sorted_has_duplicates() {
        let data = [1, 1, 2, 2, 3];
        let result = deduplicate_sorted(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[1, 2, 3]);
    }

    #[test]
    fn deduplicate_sorted_all_same() {
        let data = [5, 5, 5, 5, 5];
        let result = deduplicate_sorted(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[5]);
    }

    #[test]
    fn deduplicate_sorted_empty() {
        let data: [i32; 0] = [];
        let result = deduplicate_sorted(&data);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert!(result.is_empty());
    }

    #[test]
    fn deduplicate_sorted_single() {
        let data = [42];
        let result = deduplicate_sorted(&data);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result.as_ref(), &[42]);
    }

    #[test]
    fn deduplicate_sorted_two_same() {
        let data = [1, 1];
        let result = deduplicate_sorted(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[1]);
    }

    #[test]
    fn deduplicate_sorted_two_different() {
        let data = [1, 2];
        let result = deduplicate_sorted(&data);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result.as_ref(), &[1, 2]);
    }

    #[test]
    fn deduplicate_sorted_strings() {
        let data = ["a", "a", "b", "c", "c"];
        let result = deduplicate_sorted(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &["a", "b", "c"]);
    }

    #[test]
    fn deduplicate_sorted_multiple_runs() {
        let data = [1, 1, 1, 2, 2, 3, 3, 3, 3];
        let result = deduplicate_sorted(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[1, 2, 3]);
    }

    // =============================================================================
    // Part 2: Cow with Collections - clamp_values
    // =============================================================================

    #[test]
    fn clamp_values_all_in_range() {
        let data = [5, 6, 7, 8];
        let result = clamp_values(&data, 0, 10);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result.as_ref(), &[5, 6, 7, 8]);
    }

    #[test]
    fn clamp_values_some_out_of_range() {
        let data = [-5, 5, 15];
        let result = clamp_values(&data, 0, 10);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[0, 5, 10]);
    }

    #[test]
    fn clamp_values_all_below() {
        let data = [-10, -5, -1];
        let result = clamp_values(&data, 0, 10);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[0, 0, 0]);
    }

    #[test]
    fn clamp_values_all_above() {
        let data = [15, 20, 100];
        let result = clamp_values(&data, 0, 10);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[10, 10, 10]);
    }

    #[test]
    fn clamp_values_empty() {
        let data: [i32; 0] = [];
        let result = clamp_values(&data, 0, 10);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert!(result.is_empty());
    }

    #[test]
    fn clamp_values_at_boundaries() {
        let data = [0, 5, 10];
        let result = clamp_values(&data, 0, 10);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result.as_ref(), &[0, 5, 10]);
    }

    #[test]
    fn clamp_values_negative_range() {
        let data = [-15, -5, 5];
        let result = clamp_values(&data, -10, 0);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[-10, -5, 0]);
    }

    #[test]
    fn clamp_values_single_value_range() {
        let data = [0, 5, 10];
        let result = clamp_values(&data, 5, 5);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[5, 5, 5]);
    }

    // =============================================================================
    // Part 3: Modifying Cow - ensure_capacity
    // =============================================================================

    #[test]
    fn ensure_capacity_already_long() {
        let cow: Cow<str> = Cow::Borrowed("hello");
        let result = ensure_capacity(cow, 3, '!');
        assert_eq!(result, "hello");
    }

    #[test]
    fn ensure_capacity_needs_padding() {
        let cow: Cow<str> = Cow::Borrowed("hi");
        let result = ensure_capacity(cow, 5, '!');
        assert_eq!(result, "hi!!!");
    }

    #[test]
    fn ensure_capacity_exact_length() {
        let cow: Cow<str> = Cow::Borrowed("hello");
        let result = ensure_capacity(cow, 5, '!');
        assert_eq!(result, "hello");
    }

    #[test]
    fn ensure_capacity_empty_string() {
        let cow: Cow<str> = Cow::Borrowed("");
        let result = ensure_capacity(cow, 3, 'x');
        assert_eq!(result, "xxx");
    }

    #[test]
    fn ensure_capacity_zero_min() {
        let cow: Cow<str> = Cow::Borrowed("hello");
        let result = ensure_capacity(cow, 0, '!');
        assert_eq!(result, "hello");
    }

    #[test]
    fn ensure_capacity_owned_input() {
        let cow: Cow<str> = Cow::Owned(String::from("hi"));
        let result = ensure_capacity(cow, 4, '.');
        assert_eq!(result, "hi..");
    }

    #[test]
    fn ensure_capacity_unicode_padding() {
        let cow: Cow<str> = Cow::Borrowed("hi");
        let result = ensure_capacity(cow, 5, '★');
        assert_eq!(result, "hi★★★");
    }

    #[test]
    fn ensure_capacity_unicode_string() {
        let cow: Cow<str> = Cow::Borrowed("日本");
        let result = ensure_capacity(cow, 4, '語');
        assert_eq!(result, "日本語語");
    }

    // =============================================================================
    // Part 3: Modifying Cow - modify_if_needed
    // =============================================================================

    #[test]
    fn modify_if_needed_predicate_false() {
        let cow: Cow<str> = Cow::Borrowed("hello");
        let result = modify_if_needed(cow, |s| s.contains('X'), |c| c.to_ascii_uppercase());
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "hello");
    }

    #[test]
    fn modify_if_needed_predicate_true() {
        let cow: Cow<str> = Cow::Borrowed("hello");
        let result = modify_if_needed(cow, |s| s.contains('e'), |c| c.to_ascii_uppercase());
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn modify_if_needed_empty_string() {
        let cow: Cow<str> = Cow::Borrowed("");
        let result = modify_if_needed(cow, |_| true, |c| c.to_ascii_uppercase());
        assert_eq!(result, "");
    }

    #[test]
    fn modify_if_needed_owned_input() {
        let cow: Cow<str> = Cow::Owned(String::from("hello"));
        let result = modify_if_needed(cow, |s| s.len() > 3, |c| c.to_ascii_uppercase());
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn modify_if_needed_custom_transform() {
        let cow: Cow<str> = Cow::Borrowed("abc");
        let result = modify_if_needed(
            cow,
            |_| true,
            |c| {
                if c == 'a' { 'x' } else { c }
            },
        );
        assert_eq!(result, "xbc");
    }

    #[test]
    fn modify_if_needed_length_check() {
        let cow: Cow<str> = Cow::Borrowed("hi");
        let result = modify_if_needed(cow, |s| s.len() >= 5, |c| c.to_ascii_uppercase());
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "hi");
    }

    // =============================================================================
    // Part 4: TextProcessor
    // =============================================================================

    #[test]
    fn text_processor_new_borrowed() {
        let processor = TextProcessor::new("hello");
        assert!(processor.is_borrowed());
        assert_eq!(processor.as_str(), "hello");
    }

    #[test]
    fn text_processor_from_owned() {
        let processor = TextProcessor::from_owned(String::from("hello"));
        assert!(!processor.is_borrowed());
        assert_eq!(processor.as_str(), "hello");
    }

    #[test]
    fn text_processor_process_no_change() {
        let mut processor = TextProcessor::new("hello");
        processor.process();
        assert_eq!(processor.as_str(), "hello");
    }

    #[test]
    fn text_processor_process_trim() {
        let mut processor = TextProcessor::new("  hello  ");
        assert!(processor.is_borrowed());
        processor.process();
        assert!(!processor.is_borrowed());
        assert_eq!(processor.as_str(), "hello");
    }

    #[test]
    fn text_processor_process_normalize_whitespace() {
        let mut processor = TextProcessor::new("hello   world");
        processor.process();
        assert_eq!(processor.as_str(), "hello world");
    }

    #[test]
    fn text_processor_process_both() {
        let mut processor = TextProcessor::new("  hello   world  ");
        processor.process();
        assert_eq!(processor.as_str(), "hello world");
    }

    #[test]
    fn text_processor_into_string_borrowed() {
        let processor = TextProcessor::new("hello");
        let s = processor.into_string();
        assert_eq!(s, "hello");
    }

    #[test]
    fn text_processor_into_string_owned() {
        let processor = TextProcessor::from_owned(String::from("hello"));
        let s = processor.into_string();
        assert_eq!(s, "hello");
    }

    #[test]
    fn text_processor_append() {
        let mut processor = TextProcessor::new("hello");
        assert!(processor.is_borrowed());
        processor.append(" world");
        assert!(!processor.is_borrowed());
        assert_eq!(processor.as_str(), "hello world");
    }

    #[test]
    fn text_processor_len() {
        let processor = TextProcessor::new("hello");
        assert_eq!(processor.len(), 5);
    }

    #[test]
    fn text_processor_is_empty() {
        let processor = TextProcessor::new("");
        assert!(processor.is_empty());

        let processor = TextProcessor::new("hello");
        assert!(!processor.is_empty());
    }

    #[test]
    fn text_processor_default() {
        let processor = TextProcessor::default();
        assert!(processor.is_empty());
        assert!(!processor.is_borrowed());
    }

    #[test]
    fn text_processor_clone() {
        let processor = TextProcessor::new("hello");
        let cloned = processor.clone();
        assert_eq!(cloned.as_str(), "hello");
    }

    #[test]
    fn text_processor_unicode() {
        let mut processor = TextProcessor::new("  日本語   テスト  ");
        processor.process();
        assert_eq!(processor.as_str(), "日本語 テスト");
    }

    #[test]
    fn text_processor_tabs_and_newlines() {
        let mut processor = TextProcessor::new("hello\t\tworld\n\ntest");
        processor.process();
        assert_eq!(processor.as_str(), "hello world test");
    }

    #[test]
    fn text_processor_empty_after_trim() {
        let mut processor = TextProcessor::new("   ");
        processor.process();
        assert_eq!(processor.as_str(), "");
    }

    #[test]
    fn text_processor_multiple_appends() {
        let mut processor = TextProcessor::new("a");
        processor.append("b");
        processor.append("c");
        assert_eq!(processor.as_str(), "abc");
    }

    // =============================================================================
    // Integration Tests
    // =============================================================================

    #[test]
    fn integration_string_pipeline() {
        // Process a string through multiple Cow operations
        let input = "  HELLO WORLD  ";
        let result = trim_and_lowercase(input);
        assert_eq!(result, "hello world");

        let result = ensure_suffix(&result, "!");
        assert_eq!(result, "hello world!");
    }

    #[test]
    fn integration_collection_pipeline() {
        // Process a collection through multiple operations
        let data = [0, 1, 1, 0, 2, 2, 0, 3];

        // First remove zeros
        let no_zeros = remove_zeros(&data);
        assert_eq!(no_zeros.as_ref(), &[1, 1, 2, 2, 3]);

        // Then deduplicate (data is sorted after removing zeros in this case)
        let unique = deduplicate_sorted(&no_zeros);
        assert_eq!(unique.as_ref(), &[1, 2, 3]);
    }

    #[test]
    fn integration_text_processor_workflow() {
        let mut processor = TextProcessor::new("  HELLO   WORLD  ");
        assert!(processor.is_borrowed());

        processor.process();
        assert!(!processor.is_borrowed());
        assert_eq!(processor.as_str(), "HELLO WORLD");

        processor.append("!");
        assert_eq!(processor.as_str(), "HELLO WORLD!");

        let final_text = processor.into_string();
        assert_eq!(final_text, "HELLO WORLD!");
    }

    #[test]
    fn integration_conditional_modifications() {
        // Test a pattern where we conditionally modify based on content
        let inputs = ["HELLO", "hello", "HeLLo", "12345"];

        for input in &inputs {
            let result = maybe_uppercase(input);
            assert!(result.chars().all(|c| !c.is_lowercase()));
        }
    }

    #[test]
    fn integration_chained_ensure_suffix() {
        let filename = "document";

        // Chain multiple suffix operations
        let with_txt = ensure_suffix(filename, ".txt");
        assert_eq!(with_txt, "document.txt");

        // If we try to add it again, it should be borrowed
        let result = ensure_suffix(&with_txt, ".txt");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn integration_clamp_and_deduplicate() {
        let data = [-100, 50, 50, 200, 200, 300];

        // First clamp to range
        let clamped = clamp_values(&data, 0, 100);
        assert_eq!(clamped.as_ref(), &[0, 50, 50, 100, 100, 100]);

        // Then deduplicate (already sorted after clamping this data)
        let unique = deduplicate_sorted(&clamped);
        assert_eq!(unique.as_ref(), &[0, 50, 100]);
    }

    #[test]
    fn integration_modify_and_ensure_capacity() {
        let cow: Cow<str> = Cow::Borrowed("hello");

        // First uppercase if contains 'e'
        let result = modify_if_needed(cow, |s| s.contains('e'), |c| c.to_ascii_uppercase());
        assert_eq!(result, "HELLO");

        // Then ensure minimum length
        let result = ensure_capacity(result, 8, '!');
        assert_eq!(result, "HELLO!!!");
    }

    #[test]
    fn integration_borrowed_vs_owned_tracking() {
        // Verify we can track when conversions happen
        let borrowed: Cow<str> = Cow::Borrowed("test");
        assert!(matches!(borrowed, Cow::Borrowed(_)));

        let owned: Cow<str> = Cow::Owned(String::from("test"));
        assert!(matches!(owned, Cow::Owned(_)));

        // Operations that don't need changes should stay borrowed
        let result = maybe_uppercase("TEST");
        assert!(matches!(result, Cow::Borrowed(_)));

        // Operations that need changes should become owned
        let result = maybe_uppercase("test");
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn integration_empty_input_handling() {
        // Test all functions with empty input
        assert!(matches!(maybe_uppercase(""), Cow::Borrowed(_)));
        assert!(matches!(ensure_suffix("", ".txt"), Cow::Owned(_)));
        assert!(matches!(trim_and_lowercase(""), Cow::Borrowed(_)));
        assert!(matches!(remove_zeros(&[]), Cow::Borrowed(_)));
        assert!(matches!(deduplicate_sorted::<i32>(&[]), Cow::Borrowed(_)));
        assert!(matches!(clamp_values(&[], 0, 10), Cow::Borrowed(_)));
    }

    #[test]
    fn integration_processor_reuse() {
        let mut processor = TextProcessor::new("hello");
        processor.append(" world");
        processor.process();

        // After processing, we can still use it
        assert_eq!(processor.len(), 11);
        assert!(!processor.is_empty());

        processor.append("!");
        assert_eq!(processor.into_string(), "hello world!");
    }
}
