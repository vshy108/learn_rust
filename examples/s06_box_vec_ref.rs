// Demonstrates Box, Vec, and references (&) — three ways to use heap/indirection.
//
// - `Box<T>`: single heap-allocated value, owned.
// - `Vec<T>`: growable heap-allocated array, owned.
// - `&T`:     borrowed reference (no ownership transfer, no allocation).
//
// Run: cargo run --example s06_box_vec_ref

fn main() {
    // --- 1. Box<T>: heap-allocate a single value ---
    let stack_val = 42;
    let boxed_val = Box::new(42);
    println!(
        "stack:  {stack_val} (size = {} bytes)",
        std::mem::size_of_val(&stack_val)
    );
    println!(
        "boxed:  {boxed_val} (pointer size = {} bytes)",
        std::mem::size_of_val(&boxed_val)
    );

    // `Box` automatically dereferences (Deref trait):
    assert_eq!(*boxed_val, 42);

    // --- 2. Vec<T>: growable heap array ---
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("vec: {v:?}, len={}, cap={}", v.len(), v.capacity());

    // Vec metadata is on the stack; elements are on the heap.
    // When Vec grows past capacity, it reallocates.

    // --- 3. & (reference): borrow without owning ---
    let s = String::from("hello");
    let r: &String = &s; // `r` borrows `s` — no heap allocation
    println!("s = {s}, r = {r}");
    // Both `s` and `r` are valid. `r` doesn't own the data.

    // --- 4. Key differences ---
    // Box: transfers ownership to the heap; exactly one owner.
    // Vec: owns a contiguous heap buffer; can grow/shrink.
    // &T:  borrows existing data; no allocation; limited by lifetime.

    // --- 5. When to use each ---
    // Box: recursive types, trait objects, large structs to avoid stack overflow.
    // Vec: dynamic collections (the most common heap type).
    // &T:  function parameters, avoiding copies, read-only access.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_box_deref() {
        let b = Box::new(99);
        assert_eq!(*b, 99);
    }

    #[test]
    fn test_vec_push() {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        assert_eq!(v, vec![1, 2]);
    }

    #[test]
    fn test_reference_does_not_move() {
        let s = String::from("hello");
        let r = &s;
        // Both `s` and `r` are valid:
        assert_eq!(r, "hello");
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_box_is_pointer_sized() {
        assert_eq!(
            std::mem::size_of::<Box<[u8; 1000]>>(),
            std::mem::size_of::<usize>()
        );
    }
}
