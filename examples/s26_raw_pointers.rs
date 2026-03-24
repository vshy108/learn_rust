// Demonstrates raw pointers in Rust.
//
// Raw pointers (`*const T` and `*mut T`) are pointers without
// Rust's safety guarantees. Creating them is safe, but
// dereferencing them requires `unsafe`.
//
// **Gotcha for Python devs:** Unlike Python references which are
// always safe, raw pointers can dangle, alias mutably, or be null.
// Rust's borrow checker does NOT track raw pointers.
//
// Run: cargo run --example s26_raw_pointers

fn main() {
    println!("=== Raw Pointers ===");

    // --- 1. Creating raw pointers from references (safe) ---
    let x = 42;
    let ptr_const: *const i32 = &x; // immutable raw pointer
    println!("const ptr: {ptr_const:?}");

    let mut y = 100;
    let ptr_mut: *mut i32 = &mut y; // mutable raw pointer
    println!("mut ptr: {ptr_mut:?}");

    // --- 2. Creating raw pointers from addresses ---
    // This is safe to create but dangerous to dereference
    let addr = 0x1234usize;
    let _ptr: *const i32 = addr as *const i32;

    // --- 3. Null pointers ---
    let null_ptr: *const i32 = std::ptr::null();
    println!("null? {}", null_ptr.is_null());

    // --- 4. Multiple raw pointers to same data (allowed!) ---
    // Unlike references, we can have both *const and *mut to same data
    let mut val = 10;
    let p1: *const i32 = &val;
    let p2: *mut i32 = &mut val;
    // Both exist simultaneously -- this would be illegal with references
    println!("p1: {p1:?}, p2: {p2:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_raw_pointer_from_reference() {
        let x = 42;
        let ptr: *const i32 = &x;
        // Dereferencing requires unsafe
        // SAFETY: ptr was created from a valid reference to x
        unsafe {
            assert_eq!(*ptr, 42);
        }
    }

    #[test]
    fn test_null_pointer() {
        let ptr: *const i32 = std::ptr::null();
        assert!(ptr.is_null());
    }
}
