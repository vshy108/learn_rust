// Demonstrates Rust's prelude — automatically imported items.
//
// The **prelude** is a set of types, traits, and functions that Rust
// imports into every module automatically. You don't need `use` for them.
//
// Key prelude items:
// - `Option`, `Some`, `None`
// - `Result`, `Ok`, `Err`
// - `Vec`, `String`, `Box`
// - `Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, etc.
// - `Iterator`, `IntoIterator`
// - `ToString`, `From`, `Into`
// - `println!`, `format!`, `vec!`
//
// Reference: https://doc.rust-lang.org/std/prelude/
//
// Run: cargo run --example s12_prelude

fn main() {
    // --- 1. Types from the prelude ---
    let s: String = String::from("prelude"); // no `use std::string::String`
    let v: Vec<i32> = vec![1, 2, 3]; // no `use std::vec::Vec`
    let b: Box<i32> = Box::new(42); // no `use std::boxed::Box`
    println!("String: {s}, Vec: {v:?}, Box: {b}");

    // --- 2. Enum variants from the prelude ---
    let opt: Option<i32> = Some(10); // Some/None are in prelude
    let res: Result<i32, &str> = Ok(42); // Ok/Err are in prelude
    println!("opt: {opt:?}, res: {res:?}");

    // --- 3. Traits from the prelude ---
    // Clone, Copy, PartialEq, Debug, etc. are usable without `use`.
    #[derive(Debug, Clone, PartialEq, Default)]
    struct Point {
        x: f64,
        y: f64,
    }

    let p = Point::default(); // Default is in the prelude
    let p2 = p.clone(); // Clone is in the prelude
    assert_eq!(p, p2); // PartialEq is in the prelude
    println!("{p:?}"); // Debug is in the prelude

    // --- 4. Conversion traits ---
    // From and Into are in the prelude.
    let n: i64 = i64::from(42_i32); // From<i32> for i64
    let s: String = String::from("hello"); // From<&str> for String
    println!("n: {n}, s: {s}");

    // --- 5. What's NOT in the prelude ---
    // You DO need `use` for:
    // - HashMap, HashSet (use std::collections::HashMap)
    // - File, BufReader (use std::fs::File, std::io::BufReader)
    // - specific formatting traits (use std::fmt::Write)
    // - most of std::io

    // --- 6. Edition-specific preludes ---
    // Rust 2021+ prelude also includes:
    // - TryFrom, TryInto (conversion that can fail)
    // - FromIterator
    let n: i8 = 42_i32.try_into().unwrap(); // TryInto in 2021+ prelude
    println!("try_into: {n}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prelude_types() {
        // These all work without any `use` statements:
        let _s = String::new();
        let _v: Vec<i32> = Vec::new();
        let _b = Box::new(0);
        let _o: Option<i32> = None;
        let _r: Result<(), &str> = Ok(());
    }

    #[test]
    fn test_default_trait() {
        let x: i32 = Default::default();
        assert_eq!(x, 0);
        let s: String = Default::default();
        assert!(s.is_empty());
    }
}
