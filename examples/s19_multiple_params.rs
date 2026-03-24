// Demonstrates lifetime annotations with multiple parameters.
//
// When a function takes multiple references and returns a reference,
// you must tell the compiler how the output lifetime relates to the inputs.
//
// Run: cargo run --example s19_multiple_params

/// Both params share lifetime 'a — result valid for shorter of the two.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

/// Result tied ONLY to `a` — `b`'s lifetime doesn't matter.
fn with_prefix<'a>(a: &'a str, _prefix: &str) -> &'a str {
    a
}

/// Different lifetimes for different parameters.
/// Returns a reference tied to the first parameter only.
fn pick_first<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str {
    a
}

/// A function where both lifetimes matter for different return paths:
fn conditional<'a, 'b>(a: &'a str, b: &'b str, use_first: bool) -> String {
    // Because we return String (owned), no lifetime constraints on return.
    // But we still need the references to be valid for the function body.
    if use_first {
        a.to_string()
    } else {
        b.to_string()
    }
}

fn main() {
    let a = String::from("alpha");
    let result;
    {
        let b = String::from("be");
        result = longest(&a, &b);
        println!("longest: {result}");
    }
    // result can't be used here if it points to b (which is dropped)

    // with_prefix: result tied only to first param
    let name = String::from("Alice");
    let prefixed = with_prefix(&name, "Ms.");
    println!("prefixed: {prefixed}");

    // pick_first: explicitly first only
    let p = pick_first("hello", "world");
    println!("picked: {p}");

    // conditional: returns owned String, no lifetime issues
    let c = conditional("A", "B", true);
    println!("conditional: {c}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abc", "ab"), "abc");
    }

    #[test]
    fn test_with_prefix() {
        assert_eq!(with_prefix("data", "prefix"), "data");
    }

    #[test]
    fn test_pick_first() {
        assert_eq!(pick_first("x", "y"), "x");
    }

    #[test]
    fn test_conditional() {
        assert_eq!(conditional("A", "B", true), "A");
        assert_eq!(conditional("A", "B", false), "B");
    }
}
