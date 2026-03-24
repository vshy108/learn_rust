// Demonstrates the different string types in Rust.
//
// Rust has four main "string" types that often confuse newcomers:
//
// | Type       | Owned? | Mutable? | Where data lives            |
// |------------|--------|----------|-----------------------------|
// | `String`   | Yes    | Yes      | Heap                        |
// | `&String`  | No     | No       | Reference to heap String    |
// | `str`      | —      | —        | Unsized type (rarely used)  |
// | `&str`     | No     | No       | Slice into String or static |
//
// **String** = `Vec<u8>` (guaranteed UTF-8). Owns its buffer on the heap.
// **&str** = fat pointer: (pointer to bytes, length). A view/slice into text.
// **str** = the unsized type behind &str. You almost never use it directly.
// **&String** = reference to a String. Coerces to &str via Deref.
//
// Best practice: accept `&str` in function parameters (most flexible).
//
// Run: cargo run --example s06_string_types

/// Accepts `&str` — works with both String (via deref) and &str directly.
///
/// # Arguments
///
/// * `text` - A string slice
///
/// # Returns
///
/// Whether the text is empty
fn is_blank(text: &str) -> bool {
    text.trim().is_empty()
}

fn main() {
    // --- 1. String: heap-owned, growable ---
    let mut owned = String::from("hello");
    owned.push_str(" world");
    println!("String: {owned}");

    // --- 2. &str: borrowed view into string data ---
    // String literals are &str pointing to data baked into the binary.
    let literal: &str = "I live in the binary";
    println!("&str literal: {literal}");

    // You can also get a &str by slicing a String:
    let slice: &str = &owned[0..5];
    println!("&str slice: {slice}");

    // --- 3. &String: reference to String ---
    let ref_to_string: &String = &owned;
    println!("&String: {ref_to_string}");
    // &String auto-coerces to &str via Deref, so this works:
    println!("is_blank(&String): {}", is_blank(ref_to_string));

    // --- 4. &str from literal ---
    println!("is_blank(literal): {}", is_blank(literal));

    // --- 5. &str from String (borrow) ---
    // Two equivalent ways:
    let as_str: &str = owned.as_str(); // explicit
    let deref_str: &str = &owned; // deref coercion
    println!("as_str: {as_str}, deref: {deref_str}");

    // --- 6. Why prefer &str in function params? ---
    // - String literals are &str (no conversion needed).
    // - &String coerces to &str automatically.
    // - String slices are &str.
    // Using &str means your function works with ALL of the above.

    // --- 7. str (unsized) ---
    // `str` is an unsized type (size not known at compile time).
    // You can't create a variable of type `str` directly.
    // Always use it behind a pointer: &str, Box<str>, Rc<str>, etc.
    let boxed: Box<str> = "boxed str".into();
    println!("Box<str>: {boxed}");

    // --- 8. Static lifetime ---
    // String literals have type `&'static str`: they live for the
    // entire program because they're embedded in the binary.
    let forever: &'static str = "I live forever";
    println!("{forever}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_blank_empty() {
        assert!(is_blank(""));
        assert!(is_blank("   "));
    }

    #[test]
    fn test_is_blank_non_empty() {
        assert!(!is_blank("hi"));
        assert!(!is_blank("  hi  "));
    }

    #[test]
    fn test_string_to_str() {
        let s = String::from("test");
        let r: &str = &s;
        assert_eq!(r, "test");
    }

    #[test]
    fn test_slice_range() {
        let s = String::from("hello");
        let slice = &s[1..4];
        assert_eq!(slice, "ell");
    }

    #[test]
    fn test_box_str() {
        let b: Box<str> = "boxed".into();
        assert_eq!(&*b, "boxed");
    }
}
