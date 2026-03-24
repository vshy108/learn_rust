// Demonstrates the dereference operator (*) and the `Deref` trait.
//
// `*` follows a reference (or smart pointer) to the value it points to.
//
// The `Deref` trait lets you customize `*` for your own types. When you
// implement `Deref`, Rust can automatically coerce `&YourType` into
// `&Target` — this is called **deref coercion**. It makes smart pointers
// feel like regular references.
//
// Key deref coercions:
// - `&String`   → `&str`    (because String implements Deref<Target = str>)
// - `&Vec<T>`   → `&[T]`    (because Vec<T> implements Deref<Target = [T]>)
// - `&Box<T>`   → `&T`      (because Box<T> implements Deref<Target = T>)
//
// Run: cargo run --example s06_dereference_deref_trait

use std::ops::Deref;

/// A simple wrapper type to demonstrate implementing Deref.
///
/// `MyBox<T>` wraps a value of type T. We implement Deref so `*MyBox<T>`
/// gives us `&T`, and `&MyBox<T>` coerces to `&T`.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

// Implementing Deref so *MyBox<T> works.
impl<T> Deref for MyBox<T> {
    // This is the "target type" that dereferencing produces.
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// Takes a &str — demonstrates deref coercion from &String or &MyBox<String>.
fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // --- 1. Basic dereference with & ---
    let x: i32 = 5;
    let y: &i32 = &x;
    assert_eq!(5, *y); // * follows the reference to get the value
    println!("x = {x}, *y = {}", *y);

    // --- 2. Dereference with Box ---
    let boxed = Box::new(42);
    assert_eq!(42, *boxed); // Box implements Deref
    println!("*boxed = {}", *boxed);

    // --- 3. Deref coercion: &String → &str ---
    let name = String::from("Rust");
    greet(&name); // &String auto-coerces to &str via Deref
    // This is why you can pass &String to functions expecting &str!

    // --- 4. Custom Deref with MyBox ---
    let my_val = MyBox::new(String::from("world"));
    greet(&my_val);
    // Coercion chain: &MyBox<String> → &String → &str
    // Rust applies Deref repeatedly until types match.

    // --- 5. Deref coercion: &Vec<T> → &[T] ---
    let nums = vec![1, 2, 3];
    let slice: &[i32] = &nums; // Vec<i32> derefs to &[i32]
    println!("slice = {slice:?}");

    // --- 6. DerefMut for mutable dereferencing ---
    // If a type implements DerefMut, `*my_val = ...` works through the
    // mutable dereference. Box and Vec both implement DerefMut.
    let mut boxed_str = Box::new(String::from("mutable"));
    boxed_str.push_str("!"); // DerefMut: &mut Box<String> → &mut String
    println!("boxed_str = {boxed_str}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_deref() {
        let x: i32 = 10;
        let r = &x;
        assert_eq!(*r, 10);
    }

    #[test]
    fn test_box_deref() {
        let b = Box::new(99);
        assert_eq!(*b, 99);
    }

    #[test]
    fn test_mybox_deref() {
        let mb = MyBox::new(42);
        assert_eq!(*mb, 42);
    }

    #[test]
    fn test_deref_coercion_string() {
        let s = String::from("test");
        // &String coerces to &str
        let r: &str = &s;
        assert_eq!(r, "test");
    }

    #[test]
    fn test_deref_coercion_vec() {
        let v = vec![1, 2, 3];
        let s: &[i32] = &v;
        assert_eq!(s, &[1, 2, 3]);
    }
}
