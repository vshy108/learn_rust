// Demonstrates ownership interaction with immutable and mutable references.
//
// This example shows how references relate to ownership rules:
// - Ownership can be temporarily "lent" via references.
// - The owner retains control and can reclaim full access when borrows end.
// - Functions can accept references to work with data they don't own.
//
// Run: cargo run --example s07_ownership_immut_mut_refs

/// Counts words in a string slice (read-only borrow).
///
/// # Arguments
///
/// * `text` - Shared reference to a string slice
///
/// # Returns
///
/// Number of whitespace-separated words
fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Capitalizes the first character of a String (write borrow).
///
/// # Arguments
///
/// * `s` - Exclusive mutable reference to a String
fn capitalize_first(s: &mut String) {
    if let Some(first) = s.chars().next() {
        let upper = first.to_uppercase().to_string();
        // Replace the first character(s). Note: char can be multi-byte.
        let rest = &s[first.len_utf8()..];
        *s = format!("{upper}{rest}");
    }
}

fn main() {
    // --- 1. Owner creates, references borrow ---
    let mut text = String::from("hello world from rust");

    // Immutable borrow: owner still has the data, we just peek.
    let wc = word_count(&text);
    println!("Word count: {wc}");

    // Mutable borrow: owner lends exclusive write access.
    capitalize_first(&mut text);
    println!("Capitalized: {text}");

    // --- 2. Owner can use the value after borrows end ---
    // Both borrows above are finished (NLL), so owner has full access:
    text.push('!');
    println!("Final: {text}");

    // --- 3. Transferring ownership to a collection ---
    let s1 = String::from("alpha");
    let s2 = String::from("beta");
    let mut collection = vec![s1, s2]; // s1, s2 MOVED into vec
    // s1, s2 are no longer accessible.

    // We can borrow from the collection:
    for item in &collection {
        println!("item: {item}");
    }

    // Mutable borrow from collection:
    for item in &mut collection {
        capitalize_first(item);
    }
    println!("capitalized collection: {collection:?}");

    // --- 4. Ownership transfer vs borrowing decision ---
    // Use borrowing (&T / &mut T) when the caller needs to keep using the value.
    // Use ownership (T) when the function needs to store or consume the value.
    let keeper = String::from("I stay");
    borrow_it(&keeper);
    println!("keeper still here: {keeper}");

    let giver = String::from("I move");
    consume_it(giver);
    // println!("{giver}"); // error: giver was moved
}

/// Borrows a string without taking ownership.
fn borrow_it(s: &str) {
    println!("Borrowed: {s}");
}

/// Takes ownership of a string — caller can't use it afterward.
fn consume_it(s: String) {
    println!("Consumed: {s}");
} // s is dropped here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world"), 2);
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("one"), 1);
    }

    #[test]
    fn test_capitalize_first() {
        let mut s = String::from("hello");
        capitalize_first(&mut s);
        assert_eq!(s, "Hello");
    }

    #[test]
    fn test_capitalize_first_empty() {
        let mut s = String::new();
        capitalize_first(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn test_capitalize_first_unicode() {
        let mut s = String::from("umlaut");
        capitalize_first(&mut s);
        assert_eq!(s, "Umlaut");
    }

    #[test]
    fn test_owner_retains_after_borrow() {
        let s = String::from("test");
        let _ = word_count(&s);
        assert_eq!(s, "test"); // still owned
    }
}
