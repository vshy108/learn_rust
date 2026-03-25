// Demonstrates stack vs heap memory in Rust.
//
// Stack: fast, fixed-size, LIFO, auto-cleaned when scope ends.
// Heap: slower, dynamic-size, explicitly allocated, cleaned by Drop trait.
//
// Rust lets you choose stack vs heap — and defaults to stack when possible
// for performance.
//
// Run: cargo run --example s06_stack_vs_heap

fn main() {
    // --- 1. Stack allocation (default for fixed-size types) ---
    let x: i32 = 10; // 4 bytes on the stack
    let y: f64 = 3.14; // 8 bytes on the stack
    let arr = [1, 2, 3]; // 12 bytes on the stack
    println!("stack: x={x}, y={y}, arr={arr:?}");

    // --- 2. Heap allocation (String, Vec, Box) ---
    // `String` stores metadata on the stack (pointer, len, capacity)
    // and the actual character data on the heap.
    let s = String::from("hello");
    println!("heap: s={s}");
    println!(
        "  stack part: {} bytes (ptr + len + cap)",
        std::mem::size_of::<String>()
    );
    println!("  heap part:  {} bytes (actual data)", s.len());

    // --- 3. Vec is similar to String ---
    let v = vec![1, 2, 3, 4, 5];
    println!(
        "vec metadata size: {} bytes",
        std::mem::size_of::<Vec<i32>>()
    );
    println!("vec data: {} elements, {} bytes", v.len(), v.len() * 4);

    // --- 4. Why two regions? ---
    // Stack: size must be known at compile time. Very fast (pointer bump).
    // Heap: size can be determined at runtime. Slower (allocator call).
    //
    // Stack frame layout (conceptual):
    //   |  x: i32 (4 bytes)     |
    //   |  y: f64 (8 bytes)     |
    //   |  s: {ptr, len, cap}   | -> points to heap: "hello"
    //   |  v: {ptr, len, cap}   | -> points to heap: [1,2,3,4,5]

    // --- 5. Automatic cleanup ---
    // Stack: cleaned when function returns (stack pointer moves back).
    // Heap: cleaned when the owner is dropped (Rust calls `Drop::drop`).
    {
        let temp = String::from("temporary");
        println!("inside scope: {temp}");
    } // `temp` is dropped here — heap memory freed automatically

    // --- 6. Box: explicit heap allocation ---
    // `Box<T>` puts a value on the heap. Useful for large values or
    // recursive types (which need indirection).
    let boxed = Box::new(42);
    println!("boxed value: {boxed}");
    println!(
        "Box<i32> stack size: {} bytes (just a pointer)",
        std::mem::size_of::<Box<i32>>()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_string_metadata_size() {
        // String = pointer + len + capacity = 3 * usize = 24 bytes on 64-bit.
        assert_eq!(
            std::mem::size_of::<String>(),
            3 * std::mem::size_of::<usize>()
        );
    }

    #[test]
    fn test_vec_metadata_size() {
        // Vec<i32> has the same layout as String: ptr + len + cap.
        assert_eq!(
            std::mem::size_of::<Vec<i32>>(),
            3 * std::mem::size_of::<usize>()
        );
    }

    #[test]
    fn test_box_is_pointer_sized() {
        assert_eq!(
            std::mem::size_of::<Box<i32>>(),
            std::mem::size_of::<usize>()
        );
    }

    #[test]
    fn test_stack_types_are_small() {
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::size_of::<f64>(), 8);
        assert_eq!(std::mem::size_of::<[i32; 3]>(), 12);
    }
}
