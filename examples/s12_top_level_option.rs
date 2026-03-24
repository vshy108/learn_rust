// Demonstrates top-level Option variants (Some and None without Option::).
//
// `Some` and `None` are brought into scope by the prelude, so you
// don't need to write `Option::Some(42)` — just `Some(42)`.
//
// Same goes for `Ok` and `Err` from Result.
//
// Run: cargo run --example s12_top_level_option

fn main() {
    // --- 1. You can write Some/None directly ---
    let a: Option<i32> = Some(42); // not Option::Some(42)
    let b: Option<i32> = None; // not Option::None
    println!("a = {a:?}, b = {b:?}");

    // --- 2. This works because the prelude imports them ---
    // The Rust prelude automatically includes:
    //   use std::option::Option::{self, Some, None};
    //   use std::result::Result::{self, Ok, Err};
    // So these variants are always in scope.

    // --- 3. You CAN use the full path if you want ---
    let explicit = Option::Some(99);
    let explicit_none: Option<i32> = Option::None;
    println!("explicit = {explicit:?}, none = {explicit_none:?}");

    // --- 4. Pattern matching uses top-level names ---
    match a {
        Some(n) => println!("Some({n})"),
        None => println!("None"),
    }

    // --- 5. In function signatures, Option is the type, Some/None are variants ---
    fn greet(name: Option<&str>) -> String {
        match name {
            Some(n) => format!("Hello, {n}!"),
            None => "Hello, stranger!".to_string(),
        }
    }

    println!("{}", greet(Some("Rust")));
    println!("{}", greet(None));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_top_level_some() {
        let opt = Some(42);
        assert_eq!(opt, Option::Some(42));
    }

    #[test]
    fn test_top_level_none() {
        let opt: Option<i32> = None;
        assert_eq!(opt, Option::None);
    }
}
