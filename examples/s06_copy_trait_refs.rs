// Demonstrates how Copy trait interacts with references.
//
// Key insight: **All references are Copy**, regardless of whether the
// referenced type is Copy.
//
// - `&T` is Copy (shared reference — just a pointer, cheap to copy).
// - `&mut T` is NOT Copy (exclusive reference — copying would create
//   two mutable references, violating borrowing rules).
//
// This means you can pass `&T` around freely without worrying about moves.
//
// Run: cargo run --example s06_copy_trait_refs

fn main() {
    // --- 1. Shared references are Copy ---
    let s = String::from("hello"); // String is NOT Copy
    let r1: &String = &s; // r1 is a shared reference
    let r2 = r1; // COPY of the reference (not the String!)
    println!("r1 = {r1}, r2 = {r2}"); // both valid — refs are Copy

    // --- 2. The underlying value is NOT copied ---
    // r1 and r2 both point to the same String on the heap.
    // No heap allocation happened when we "copied" the reference.
    assert!(std::ptr::eq(r1, r2)); // same address!

    // --- 3. Works with non-Copy types ---
    let v = vec![1, 2, 3]; // Vec is not Copy
    let ref1 = &v;
    let ref2 = ref1; // Copy the reference
    println!("ref1 = {ref1:?}, ref2 = {ref2:?}");

    // --- 4. Mutable references are NOT Copy ---
    // They implement neither Copy nor Clone (by design).
    let mut x: i32 = 42;
    let mr = &mut x;
    // let mr2 = mr; // This MOVES mr, not copies it!
    // println!("{mr}"); // error: mr was moved
    *mr += 1; // use mr before it could be moved
    println!("x = {x}");

    // --- 5. Practical implication: functions taking &T ---
    let data = String::from("shared data");
    let r = &data;
    print_twice(r); // r is copied into the function — still valid after
    println!("r still valid: {r}");

    // --- 6. Copy reference to Copy type ---
    let num: i32 = 10;
    let r3: &i32 = &num;
    let r4 = r3; // Copy of &i32
    assert_eq!(*r3, *r4);
    println!("*r3 = {}, *r4 = {}", *r3, *r4);
}

/// Prints a reference twice. The reference is Copy, so calling this
/// doesn't move the caller's reference.
fn print_twice(s: &String) {
    println!("First:  {s}");
    println!("Second: {s}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shared_ref_is_copy() {
        let s = String::from("test");
        let r1 = &s;
        let r2 = r1; // copy
        assert_eq!(r1, r2);
        assert!(std::ptr::eq(r1, r2));
    }

    #[test]
    fn test_ref_to_non_copy_is_copy() {
        let v = vec![1, 2, 3];
        let r1 = &v;
        let r2 = r1;
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_ref_to_copy_type() {
        let x: i32 = 42;
        let r1 = &x;
        let r2 = r1;
        assert_eq!(*r1, *r2);
        assert_eq!(*r1, 42);
    }
}
