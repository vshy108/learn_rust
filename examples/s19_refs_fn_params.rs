// Demonstrates references as function parameters and lifetime implications.
//
// When you pass references to functions, the compiler tracks
// how long those references are valid.
//
// Run: cargo run --example s19_refs_fn_params

/// Takes a shared reference — doesn't own the data.
fn print_length(s: &str) {
    println!("'{s}' has length {}", s.len());
}

/// Takes a mutable reference — can modify borrower's data.
fn append_exclaim(s: &mut String) {
    s.push('!');
}

/// Takes two references — both must be valid for the call.
fn print_both(a: &str, b: &str) {
    println!("{a} and {b}");
}

/// Returns a reference derived from the parameter.
/// The compiler infers that the output lifetime matches the input.
fn trim_greeting(s: &str) -> &str {
    s.trim()
}

/// Multiple reference parameters with different purposes.
fn find_in(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    let s = String::from("  Hello, Rust!  ");

    // Shared reference:
    print_length(&s);

    // Trimmed reference into the original:
    let trimmed = trim_greeting(&s);
    println!("trimmed: '{trimmed}'");

    // Multiple references:
    print_both("left", "right");

    // Search:
    println!("contains 'Rust': {}", find_in(&s, "Rust"));

    // Mutable reference:
    let mut owned = String::from("Hello");
    append_exclaim(&mut owned);
    println!("after append: {owned}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_greeting() {
        assert_eq!(trim_greeting("  hi  "), "hi");
    }

    #[test]
    fn test_find_in() {
        assert!(find_in("hello world", "world"));
        assert!(!find_in("hello world", "xyz"));
    }

    #[test]
    fn test_append_exclaim() {
        let mut s = String::from("wow");
        append_exclaim(&mut s);
        assert_eq!(s, "wow!");
    }
}
