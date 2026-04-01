// Demonstrates Non-Lexical Lifetimes (NLL).
//
// Before NLL (Rust 2018), borrows lasted until the end of the scope.
// With NLL, borrows end at the last point they're used.
//
// This is a huge ergonomic improvement — it lets you borrow, use,
// then mutate, all in the same scope.
//
// **Gotcha:** NLL only helps with compile-time borrows (`&T` / `&mut T`).
// Runtime-checked borrows (e.g. `RefCell`) still require explicit `drop()`.
//
// Run: cargo run --example s19_non_lexical_lifetimes

fn main() {
    println!("=== Non-Lexical Lifetimes ===");

    // --- 1. NLL allows this (pre-NLL did NOT) ---
    let mut v = vec![1, 2, 3];
    let first = &v[0]; // immutable borrow starts
    println!("first = {first}"); // last use of immutable borrow
    // NLL ends the borrow here ^
    v.push(4); // mutable borrow OK!
    println!("v = {v:?}");

    // --- 2. Interleaved borrows ---
    let mut s = String::from("hello");
    let r = &s;
    println!("r = {r}"); // last use of r
    s.push_str(" world"); // OK: r's borrow has ended
    println!("s = {s}");

    // --- 3. Without NLL, you'd need to use scopes ---
    let mut data = vec![10, 20, 30];
    {
        let elem = &data[1];
        println!("elem = {elem}");
    } // explicit scope end — not needed with NLL, but sometimes clearer
    data.push(40);
    println!("data = {data:?}");

    // --- 4. NLL with conditional paths ---
    let mut map = std::collections::HashMap::new();
    map.insert("key", 1);
    // NLL allows this pattern:
    if let Some(val) = map.get("key") {
        println!("found: {val}");
    }
    // Borrow from get() is done, we can mutate:
    map.insert("key2", 2);
    println!("map = {map:?}");

    // --- 5. NLL limitation: RefCell (runtime-checked borrows) ---
    // NLL is a compile-time analysis. RefCell borrows are tracked at
    // runtime, so the compiler can't end them early for you.
    // You MUST explicitly drop() the Ref/RefMut guard.
    use std::cell::RefCell;
    let data = RefCell::new(vec![1, 2, 3]);
    let borrow1 = data.borrow(); // returns Ref<Vec<i32>>
    println!("borrow1 = {borrow1:?}");
    drop(borrow1); // must drop manually -- NLL cannot help here
    data.borrow_mut().push(4); // now safe; would panic without drop above
    println!("after push = {:?}", data.borrow());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_nll_borrow_then_mutate() {
        let mut v = vec![1, 2, 3];
        let first = v[0]; // copy, not borrow (i32 is Copy)
        v.push(4);
        assert_eq!(first, 1);
        assert_eq!(v.len(), 4);
    }

    #[test]
    fn test_nll_ref_then_mutate() {
        let mut s = String::from("hi");
        let len = s.len(); // no borrow kept
        s.push('!');
        assert_eq!(len, 2);
        assert_eq!(s, "hi!");
    }

    // --- NLL limitation tests ---

    #[test]
    fn test_refcell_needs_explicit_drop() {
        use std::cell::RefCell;
        let data = RefCell::new(vec![1, 2, 3]);
        let borrow = data.borrow();
        assert_eq!(*borrow, vec![1, 2, 3]);
        drop(borrow); // without this, borrow_mut() below panics
        data.borrow_mut().push(4);
        assert_eq!(*data.borrow(), vec![1, 2, 3, 4]);
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn test_refcell_panics_without_drop() {
        use std::cell::RefCell;
        let data = RefCell::new(42);
        let _shared = data.borrow(); // Ref guard still alive
        // NLL cannot shorten runtime borrows -- this panics!
        let _exclusive = data.borrow_mut();
    }

}
