// Demonstrates T: 'static vs &'static T.
//
// These look similar but mean completely different things:
//
// **&'static T** — a reference that lives for the entire program.
//   Example: `&'static str` (a string literal).
//
// **T: 'static** — a TYPE BOUND meaning the type T contains NO
//   non-'static references. It does NOT mean the value lives forever!
//   Owned types like String, Vec, i32 all satisfy T: 'static.
//
// Think of it this way:
// - `&'static T`: "this borrow lasts forever"
// - `T: 'static`: "this type can live as long as needed (no dangling refs)"
//
// Run: cargo run --example s12_t_static

use std::fmt::Debug;

/// Accepts any T that satisfies 'static — most owned types do.
///
/// T: 'static does NOT mean T must be a static string or live forever.
/// It means T does not contain any non-'static references.
fn print_static<T: Debug + 'static>(val: T) {
    println!("static-bounded: {val:?}");
}

/// Accepts a reference with 'static lifetime.
fn print_ref_static(s: &'static str) {
    println!("static ref: {s}");
}

fn main() {
    // --- 1. T: 'static accepts owned types ---
    print_static(42_i32); // i32: 'static ✓
    print_static(String::from("owned")); // String: 'static ✓
    print_static(vec![1, 2, 3]); // Vec<i32>: 'static ✓
    // Owned types have no references, so they trivially satisfy 'static.

    // --- 2. T: 'static REJECTS types containing non-'static refs ---
    // let local = String::from("local");
    // let r: &str = &local;
    // print_static(r); // error: `local` does not live long enough
    // &str with a non-'static lifetime does NOT satisfy T: 'static.

    // --- 3. &'static str works for both ---
    let literal: &'static str = "forever";
    print_static(literal); // T = &'static str, which is 'static ✓
    print_ref_static(literal); // &'static str ✓

    // --- 4. Common misconception ---
    // People think `T: 'static` means the value must live forever.
    // Actually, it just means the TYPE can live independently
    // (no borrowed references that might dangle).
    {
        let s = String::from("short lived");
        print_static(s); // s is moved in, then dropped — NOT forever!
    }

    // --- 5. Where T: 'static matters ---
    // Thread::spawn requires F: 'static because the thread might
    // outlive the scope that created it. Closures capturing local
    // references would dangle.
    let handle = std::thread::spawn(|| {
        // This closure must be 'static — can't capture local refs.
        let x = 42;
        x
    });
    println!("thread result: {}", handle.join().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owned_is_static() {
        // These all satisfy T: 'static:
        fn assert_static<T: 'static>(_val: T) {}
        assert_static(42);
        assert_static(String::from("test"));
        assert_static(vec![1, 2, 3]);
        assert_static(true);
    }

    #[test]
    fn test_static_str_is_static() {
        fn assert_static<T: 'static>(_val: T) {}
        let s: &'static str = "hello";
        assert_static(s);
    }
}
