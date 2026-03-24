// Demonstrates how ownership works with function parameters.
//
// When you pass a value to a function:
// - **Copy types** (i32, f64, bool, char, &T) are copied — caller keeps original.
// - **Non-Copy types** (String, Vec, etc.) are MOVED — caller loses access.
//
// This is the same as assignment: `let y = x;` — copy or move depending on type.
//
// To avoid losing access, you can:
// 1. Pass a reference (`&T` or `&mut T`) — borrow, don't transfer ownership.
// 2. Return the value back from the function.
// 3. Clone before passing if you need to keep a copy.
//
// Run: cargo run --example s06_ownership_fn_params

/// Takes ownership of a String — the caller cannot use it after this call.
///
/// # Arguments
///
/// * `s` - A String (owned)
fn takes_ownership(s: String) {
    println!("I own: {s}");
} // s is dropped here — heap memory freed

/// Receives a copy of an i32 — the caller's value is unaffected.
///
/// # Arguments
///
/// * `x` - An i32 (Copy type)
fn makes_copy(x: i32) {
    println!("I got a copy: {x}");
} // x goes out of scope, nothing special happens with Copy types

/// Borrows a String — the caller retains ownership.
///
/// # Arguments
///
/// * `s` - A shared reference to a String
fn borrows(s: &String) {
    println!("I borrowed: {s}");
} // reference goes out of scope, but borrower never owned it

fn main() {
    // --- 1. Move: String passed to function ---
    let s1 = String::from("moved");
    takes_ownership(s1);
    // println!("{s1}"); // error: value used after move

    // --- 2. Copy: i32 passed to function ---
    let x: i32 = 42;
    makes_copy(x);
    println!("x still valid: {x}"); // OK — i32 is Copy

    // --- 3. Borrow: avoid move with reference ---
    let s2 = String::from("borrowed");
    borrows(&s2);
    println!("s2 still valid: {s2}"); // OK — we only lent a reference

    // --- 4. Clone: keep a copy and also pass ---
    let s3 = String::from("cloned");
    takes_ownership(s3.clone()); // clone is passed, s3 stays
    println!("s3 still valid: {s3}");

    // --- 5. Multiple Copy-type arguments ---
    let a: f64 = 3.14;
    let b: bool = true;
    let c: char = 'R';
    fn all_copy(a: f64, b: bool, c: char) {
        println!("f64={a}, bool={b}, char={c}");
    }
    all_copy(a, b, c);
    // All still valid — f64, bool, char are Copy.
    println!("a={a}, b={b}, c={c}");

    // --- 6. Pattern: return value to "give back" ownership ---
    let s4 = String::from("round trip");
    let s4 = takes_and_gives_back(s4);
    println!("s4 returned: {s4}");
}

/// Takes ownership and returns it — transferring ownership back to caller.
///
/// # Arguments
///
/// * `s` - A String (owned)
///
/// # Returns
///
/// The same String, ownership transferred back
fn takes_and_gives_back(s: String) -> String {
    println!("I have {s} temporarily");
    s // return ownership
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_takes_and_gives_back() {
        let s = String::from("test");
        let s = takes_and_gives_back(s);
        assert_eq!(s, "test");
    }

    #[test]
    fn test_copy_type_not_moved() {
        let x: i32 = 10;
        makes_copy(x);
        assert_eq!(x, 10); // still valid
    }

    #[test]
    fn test_borrow_keeps_original() {
        let s = String::from("kept");
        borrows(&s);
        assert_eq!(s, "kept");
    }
}
