// Demonstrates concrete lifetimes for references.
//
// A reference's lifetime must not outlive the value it borrows.
// The borrow checker verifies this at compile time.
//
// Run: cargo run --example s19_concrete_lifetimes_refs

fn main() {
    println!("=== Concrete Lifetimes for References ===");

    // --- 1. Valid: reference lives within value's scope ---
    let s = String::from("hello");
    let r = &s; // r borrows s
    println!("r = {r}"); // fine: s is still alive

    // --- 2. Invalid (wouldn't compile): dangling reference ---
    // let r;
    // {
    //     let s = String::from("temp");
    //     r = &s;  // ERROR: s dropped at end of block
    // }
    // println!("{r}"); // ERROR: dangling reference

    // --- 3. Reference scope: borrow lasts until last use ---
    let mut data = vec![1, 2, 3];
    let first = &data[0];
    println!("first = {first}");
    // first is no longer used after this point, so we can mutate:
    data.push(4);
    println!("data = {data:?}");

    // --- 4. Multiple references ---
    let text = String::from("world");
    let r1 = &text;
    let r2 = &text;
    println!("{r1} {r2}"); // multiple shared refs OK
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ref_within_scope() {
        let val = 42;
        let r = &val;
        assert_eq!(*r, 42);
    }

    #[test]
    fn test_ref_ends_before_mutation() {
        let mut v = vec![1, 2];
        let _first = v[0]; // copy, not reference
        v.push(3);
        assert_eq!(v.len(), 3);
    }
}
