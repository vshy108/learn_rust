// Demonstrates Deref and DerefMut traits.
//
// `Deref` lets you customize `*` (dereference). It also enables
// "deref coercion" — automatic conversion like &Box<T> -> &T.
// `DerefMut` does the same for mutable references.
//
// **Gotcha:** Deref should only be implemented for smart pointer types.
// Using it for general type conversions is an anti-pattern.
//
// Run: cargo run --example s26_deref_derefmut

use std::ops::{Deref, DerefMut};

/// A simple wrapper that demonstrates Deref.
/// Think of it like a "named Box" with a label.
#[derive(Debug)]
struct Wrapper<T> {
    label: String,
    inner: T,
}

impl<T> Wrapper<T> {
    /// Creates a new Wrapper with a label and inner value.
    fn new(label: &str, inner: T) -> Self {
        Wrapper {
            label: label.to_string(),
            inner,
        }
    }

    /// Returns the label.
    fn label(&self) -> &str {
        &self.label
    }
}

// Deref: allows *wrapper to access inner, and &Wrapper<T> -> &T coercion
impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

// DerefMut: allows *wrapper = ... and &mut Wrapper<T> -> &mut T coercion
impl<T> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

fn main() {
    println!("=== Deref and DerefMut ===");

    // --- 1. Deref with * ---
    let w = Wrapper::new("my_int", 42);
    println!("label: {}", w.label());
    println!("deref: {}", *w); // calls w.deref() -> &42

    // --- 2. Deref coercion ---
    let w = Wrapper::new("greeting", String::from("hello"));
    // &Wrapper<String> -> &String -> &str (two deref coercions!)
    print_str(&w);

    // --- 3. DerefMut ---
    let mut w = Wrapper::new("nums", vec![1, 2, 3]);
    w.push(4); // DerefMut coercion: calls Vec::push
    println!("after push: {:?}", *w);

    // --- 4. Box already implements Deref ---
    let b = Box::new(String::from("boxed"));
    print_str(&b); // &Box<String> -> &String -> &str

    // --- 5. Deref coercion chain ---
    let w = Wrapper::new("boxed_str", Box::new(String::from("deep")));
    // &Wrapper<Box<String>> -> &Box<String> -> &String -> &str
    print_str(&w);
}

/// Takes a &str — deref coercion allows passing &String, &Box<String>, etc.
fn print_str(s: &str) {
    println!("  str: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let w = Wrapper::new("test", 42);
        assert_eq!(*w, 42);
    }

    #[test]
    fn test_deref_mut() {
        let mut w = Wrapper::new("test", vec![1]);
        w.push(2);
        assert_eq!(*w, vec![1, 2]);
    }

    #[test]
    fn test_deref_coercion() {
        let w = Wrapper::new("test", String::from("hello"));
        let s: &str = &w; // deref coercion
        assert_eq!(s, "hello");
    }
}
