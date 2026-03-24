// Demonstrates raw pointers with unsafe code.
//
// Dereferencing raw pointers requires an `unsafe` block.
// You must guarantee safety invariants yourself.
//
// **Gotcha:** `unsafe` doesn't disable the borrow checker—it only
// allows specific operations: deref raw pointers, call unsafe fns,
// access mutable statics, implement unsafe traits, access union fields.
//
// Run: cargo run --example s26_raw_pointers_unsafe

fn main() {
    println!("=== Raw Pointers and Unsafe ===");

    // --- 1. Dereferencing a raw pointer ---
    let x = 42;
    let ptr: *const i32 = &x;

    // SAFETY: ptr is derived from a valid reference to x, which is still alive
    let val = unsafe { *ptr };
    println!("dereferenced: {val}");

    // --- 2. Mutating through a raw pointer ---
    let mut y = 100;
    let ptr_mut: *mut i32 = &mut y;

    // SAFETY: ptr_mut is the only active pointer, y is alive
    unsafe {
        *ptr_mut += 10;
    }
    println!("mutated y: {y}");

    // --- 3. Calling an unsafe function ---
    // SAFETY: offset 0 from a valid pointer is always valid
    let arr = [10, 20, 30];
    let ptr = arr.as_ptr();
    unsafe {
        println!("arr[0]: {}", *ptr);
        println!("arr[1]: {}", *ptr.add(1));
        println!("arr[2]: {}", *ptr.add(2));
    }

    // --- 4. Converting between pointer types ---
    let data: u32 = 0x04030201;
    let byte_ptr: *const u8 = &data as *const u32 as *const u8;
    // SAFETY: reading individual bytes of a u32 is valid
    unsafe {
        // Little-endian: first byte is least significant
        println!("first byte: 0x{:02X}", *byte_ptr);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deref_raw_pointer() {
        let x = 99;
        let ptr: *const i32 = &x;
        // SAFETY: ptr derived from valid reference
        let val = unsafe { *ptr };
        assert_eq!(val, 99);
    }

    #[test]
    fn test_mutate_through_ptr() {
        let mut x = 5;
        let ptr: *mut i32 = &mut x;
        // SAFETY: ptr is only active pointer to x
        unsafe { *ptr = 10 };
        assert_eq!(x, 10);
    }

    #[test]
    fn test_pointer_arithmetic() {
        let arr = [1, 2, 3, 4, 5];
        let ptr = arr.as_ptr();
        // SAFETY: offset within bounds of arr
        unsafe {
            assert_eq!(*ptr.add(0), 1);
            assert_eq!(*ptr.add(4), 5);
        }
    }
}
