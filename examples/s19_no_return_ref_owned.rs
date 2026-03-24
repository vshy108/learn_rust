// Demonstrates that functions cannot return references to owned values.
//
// If a function creates a value, it can't return a reference to it —
// the value would be dropped when the function returns, leaving a dangling reference.
//
// Run: cargo run --example s19_no_return_ref_owned

// --- This would NOT compile: ---
// fn bad() -> &str {
//     let s = String::from("hello");
//     &s  // ERROR: returns a reference to data owned by the function
// }

/// Return the owned value instead of a reference.
fn create_greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Return a reference to data that outlives the function (input parameter).
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

/// Return a reference to static data (lives forever).
fn default_name() -> &'static str {
    "World" // string literals have 'static lifetime
}

/// Another pattern: take a reference in, return part of it.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

fn main() {
    // Return owned value:
    let greeting = create_greeting("Rust");
    println!("{greeting}");

    // Return reference to input:
    let a = String::from("long string");
    let b = String::from("short");
    let result = longest(&a, &b);
    println!("longest: {result}");

    // Return static reference:
    let name = default_name();
    println!("default: {name}");

    // Return slice of input:
    let sentence = "hello world";
    let word = first_word(sentence);
    println!("first word: {word}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_greeting() {
        assert_eq!(create_greeting("Alice"), "Hello, Alice!");
    }

    #[test]
    fn test_longest() {
        assert_eq!(longest("abc", "ab"), "abc");
        assert_eq!(longest("x", "xy"), "xy");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
    }
}
