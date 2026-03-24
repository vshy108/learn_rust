// Introduces generic lifetime annotations.
//
// When the compiler can't figure out how input/output references
// relate, you must annotate with lifetime parameters: `'a`.
//
// `'a` means "some lifetime" — the compiler fills in the actual duration.
//
// Run: cargo run --example s19_intro_generic_lifetimes

/// Without lifetimes, the compiler doesn't know which input
/// the output reference is tied to. This function needs `'a`:
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

/// The `'a` means: the returned reference is valid as long as
/// BOTH a and b are valid (the shorter of the two lifetimes).

/// One parameter — no annotation needed (elision handles it):
fn first_char(s: &str) -> &str {
    &s[..1]
}

/// Two parameters, but only one is returned — still need annotation
/// to tell the compiler which input the output relates to:
fn pick_first<'a>(a: &'a str, _b: &str) -> &'a str {
    a
}

fn main() {
    // --- 1. Both references live long enough ---
    let a = String::from("long string");
    let result;
    {
        let b = String::from("hi");
        result = longest(&a, &b);
        println!("longest: {result}");
    }
    // result can't be used here because b was dropped
    // (the lifetime is the shorter of a and b)

    // --- 2. Same scope — works fine ---
    let x = "hello";
    let y = "world!";
    let long = longest(x, y);
    println!("longest: {long}");

    // --- 3. Single parameter (elided) ---
    let ch = first_char("Rust");
    println!("first char: {ch}");

    // --- 4. Return tied to first parameter only ---
    let picked = pick_first("alpha", "beta");
    println!("picked: {picked}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abc", "ab"), "abc");
        assert_eq!(longest("x", "xy"), "xy");
        assert_eq!(longest("same", "same"), "same");
    }

    #[test]
    fn test_first_char() {
        assert_eq!(first_char("Rust"), "R");
    }

    #[test]
    fn test_pick_first() {
        assert_eq!(pick_first("a", "b"), "a");
    }
}
