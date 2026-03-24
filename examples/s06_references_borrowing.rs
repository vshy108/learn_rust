// Demonstrates references and borrowing in Rust.
//
// A **reference** lets you access a value without taking ownership.
// This is called **borrowing** — you borrow the value, then give it back.
//
// Two kinds of references:
// - `&T`     — shared/immutable reference: can read, cannot modify.
// - `&mut T` — exclusive/mutable reference: can read AND modify.
//
// Borrowing rules (enforced at compile time):
// 1. You can have EITHER one &mut OR any number of &T — not both.
// 2. References must always be valid (no dangling references).
//
// Run: cargo run --example s06_references_borrowing

/// Calculate the length of a string WITHOUT taking ownership.
///
/// # Arguments
///
/// * `s` - A shared reference to a String
///
/// # Returns
///
/// The length in bytes
fn calculate_length(s: &String) -> usize {
    s.len()
    // `s` goes out of scope but because it's a reference (not owned),
    // the original String is NOT dropped.
}

/// Append " world" to a string through a mutable reference.
///
/// # Arguments
///
/// * `s` - An exclusive mutable reference to a String
fn add_world(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    // --- 1. Immutable reference (&T) ---
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // borrow s1
    println!("Length of \"{s1}\" is {len}"); // s1 still valid!

    // --- 2. Mutable reference (&mut T) ---
    let mut s2 = String::from("hello");
    add_world(&mut s2); // exclusive mutable borrow
    println!("s2 = {s2}"); // hello world

    // --- 3. Multiple immutable refs are fine ---
    let s3 = String::from("shared");
    let r1 = &s3;
    let r2 = &s3;
    println!("r1 = {r1}, r2 = {r2}"); // OK: multiple shared borrows

    // --- 4. Can't mix &T and &mut T ---
    // let mut s4 = String::from("conflict");
    // let r3 = &s4;
    // let r4 = &mut s4; // error: can't borrow as mutable while immut ref exists
    // println!("{r3}");

    // --- 5. Non-Lexical Lifetimes (NLL) ---
    // Rust is smart: a borrow ends at its LAST USE, not at end of scope.
    let mut s5 = String::from("nll");
    let r5 = &s5;
    println!("r5 = {r5}"); // last use of r5
    // r5's borrow is now over — we can take a &mut:
    let r6 = &mut s5;
    r6.push_str(" rocks");
    println!("s5 = {s5}");

    // --- 6. References are like pointers but safe ---
    let x: i32 = 10;
    let r = &x;
    println!("r points to: {r}, value: {}", *r); // * dereferences
    // Unlike C pointers, Rust references are always valid and non-null.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("test");
        assert_eq!(calculate_length(&s), 4);
        // s is still valid after borrowing
        assert_eq!(s, "test");
    }

    #[test]
    fn test_add_world() {
        let mut s = String::from("hello");
        add_world(&mut s);
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_multiple_immutable_refs() {
        let s = String::from("data");
        let r1 = &s;
        let r2 = &s;
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_deref_reads_value() {
        let x: i32 = 42;
        let r = &x;
        assert_eq!(*r, 42);
    }
}
