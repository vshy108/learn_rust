// Demonstrates how Rust uses scope to determine ownership and automatic cleanup.
//
// In Rust, values are dropped deterministically when they go out of scope.
// No GC needed — the compiler inserts `drop()` calls at scope boundaries.
//
// Run: cargo run --example s06_scope_and_ownership

fn main() {
    // --- 1. Scope determines lifetime ---
    {
        let s = String::from("scoped");
        println!("inside scope: {s}");
    } // `s` is dropped here — memory freed immediately
    // println!("{s}"); // error: `s` not found in this scope

    // --- 2. Move semantics ---
    // For heap types, assignment *moves* ownership. The original is invalidated.
    let s1 = String::from("hello");
    let s2 = s1; // `s1` moved to `s2`
    // println!("{s1}"); // error: value used after move
    println!("s2 = {s2}");

    // Why? If both s1 and s2 pointed to the same heap data, dropping both
    // would cause a double-free bug. Rust prevents this with moves.

    // --- 3. Stack types Copy, not move ---
    let a = 5;
    let b = a; // copy, not move
    println!("a = {a}, b = {b}"); // both valid

    // --- 4. Ownership transfer to functions ---
    let name = String::from("Alice");
    takes_ownership(name);
    // println!("{name}"); // error: name was moved into the function

    let num = 42;
    makes_copy(num);
    println!("num after makes_copy: {num}"); // fine — i32 is Copy

    // --- 5. Returning ownership ---
    let s3 = gives_ownership();
    println!("s3 = {s3}");

    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);
    println!("s5 = {s5}");
    // s4 is no longer valid; s5 owns the data now.

    // --- 6. Nested scopes ---
    let outer = String::from("outer");
    {
        let inner = String::from("inner");
        println!("{outer} and {inner}");
    } // `inner` dropped here
    println!("{outer}"); // `outer` still valid
}

fn takes_ownership(s: String) {
    println!("takes_ownership: {s}");
} // `s` dropped here

fn makes_copy(n: i32) {
    println!("makes_copy: {n}");
}

fn gives_ownership() -> String {
    String::from("gift")
}

fn takes_and_gives_back(s: String) -> String {
    s // ownership returned to the caller
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gives_ownership() {
        let s = gives_ownership();
        assert_eq!(s, "gift");
    }

    #[test]
    fn test_takes_and_gives_back() {
        let s = String::from("roundtrip");
        let s2 = takes_and_gives_back(s);
        assert_eq!(s2, "roundtrip");
    }

    #[test]
    fn test_copy_semantics() {
        let a = 10;
        let b = a;
        assert_eq!(a, b);
    }
}
