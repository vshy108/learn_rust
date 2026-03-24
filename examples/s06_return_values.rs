// Demonstrates return values and ownership transfer.
//
// When a function returns a value, ownership moves to the caller.
// This is how you "give" a value to another scope.
//
// Patterns covered:
// 1. Function creates and returns a new value → ownership to caller.
// 2. Function takes and returns a value → ownership round-trips.
// 3. Returning tuples to give back multiple values.
// 4. Returning references (borrowing) to avoid moves.
//
// Run: cargo run --example s06_return_values

/// Creates a new String and returns ownership to the caller.
///
/// # Returns
///
/// A freshly allocated String
fn create_greeting() -> String {
    let s = String::from("Hello from function!");
    s // ownership moves to the caller
} // nothing is dropped here — s was moved out

/// Takes a String, uses it, and returns it to "give back" ownership.
///
/// # Arguments
///
/// * `s` - A String (owned)
///
/// # Returns
///
/// The same String with ownership returned to caller
fn process_and_return(s: String) -> String {
    println!("Processing: {s}");
    s // give ownership back
}

/// Returns ownership of a String AND its length as a tuple.
///
/// # Arguments
///
/// * `s` - A String (owned)
///
/// # Returns
///
/// Tuple of (the original String, its length)
fn with_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

/// Returns a reference to the longest of two string slices.
///
/// # Arguments
///
/// * `a` - First string slice
/// * `b` - Second string slice
///
/// # Returns
///
/// Reference to the longer slice
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    // --- 1. Function creates and returns a value ---
    let greeting = create_greeting();
    println!("{greeting}"); // we own the String now

    // --- 2. Round-trip ownership ---
    let s1 = String::from("round trip");
    let s1 = process_and_return(s1); // s1 moved in, then moved back
    println!("Still have: {s1}");

    // --- 3. Return tuple to give back multiple things ---
    let s2 = String::from("measure me");
    let (s2, len) = with_length(s2);
    println!("\"{s2}\" has length {len}");

    // --- 4. Better: use references instead of ownership round-trips ---
    // The above patterns work but are verbose. Borrowing is usually preferred.
    let s3 = String::from("just borrow");
    let len = s3.len(); // &self — no ownership transfer
    println!("\"{s3}\" has {len} bytes");

    // --- 5. Returning references with lifetimes ---
    let a = String::from("short");
    let b = String::from("longer");
    let result = longest(&a, &b);
    println!("Longest: {result}");
    // result borrows from either a or b — they must live long enough.

    // --- 6. Common pattern: builder functions ---
    let config = build_config("app", 8080);
    println!("{config}");
}

/// Builds a config string: demonstrates returning a newly created owned value.
///
/// # Arguments
///
/// * `name` - Application name
/// * `port` - Port number
///
/// # Returns
///
/// A formatted config String
fn build_config(name: &str, port: u16) -> String {
    format!("{name} running on port {port}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_greeting() {
        let s = create_greeting();
        assert_eq!(s, "Hello from function!");
    }

    #[test]
    fn test_process_and_return() {
        let s = String::from("test");
        let s = process_and_return(s);
        assert_eq!(s, "test");
    }

    #[test]
    fn test_with_length() {
        let s = String::from("abcd");
        let (s, len) = with_length(s);
        assert_eq!(s, "abcd");
        assert_eq!(len, 4);
    }

    #[test]
    fn test_longest() {
        assert_eq!(longest("short", "longer"), "longer");
        assert_eq!(longest("equal", "equal"), "equal");
    }

    #[test]
    fn test_build_config() {
        let cfg = build_config("myapp", 3000);
        assert_eq!(cfg, "myapp running on port 3000");
    }
}
