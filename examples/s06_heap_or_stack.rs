#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates how to determine whether a value lives on the stack or heap.
//
// Rule of thumb:
//   - Fixed-size types (i32, f64, bool, [T; N], tuples) → stack.
//   - Dynamically-sized or growable types (String, Vec, Box) → heap data
//     with a stack-resident "handle" (pointer + metadata).
//
// Run: cargo run --example s06_heap_or_stack

fn main() {
    // --- 1. Definitely stack ---
    let _a: i32 = 42; // 4 bytes, fixed
    let _b: f64 = 3.14; // 8 bytes, fixed
    let _c: bool = true; // 1 byte, fixed
    let _d: char = 'R'; // 4 bytes, fixed
    let _e: [i32; 3] = [1, 2, 3]; // 12 bytes, fixed
    let _f: (i32, f64) = (1, 2.0); // 12 bytes (with padding), fixed
    println!("All fixed-size types: stack only");

    // --- 2. Stack handle + heap data ---
    let _s: String = String::from("hello"); // 24-byte handle on stack, data on heap
    let _v: Vec<i32> = vec![1, 2, 3]; // 24-byte handle on stack, data on heap
    let _b: Box<i32> = Box::new(42); // 8-byte pointer on stack, i32 on heap
    println!("String, Vec, Box: stack handle + heap data");

    // --- 3. &str (string slice reference) ---
    // A string literal `&str` points to data in the binary's read-only section,
    // NOT the heap. The reference itself (ptr + len) is on the stack.
    let _lit: &str = "literal"; // stack ptr -> static binary data
    println!("&str literal: stack reference -> binary");

    // A `&str` from a `String` points to heap data:
    let owned = String::from("owned");
    let _slice: &str = &owned; // stack ptr -> heap data
    println!("&str from String: stack reference -> heap");

    // --- 4. References (&T) are always on the stack ---
    let x = 42;
    let _r: &i32 = &x; // _r is a pointer on the stack, pointing to x on the stack
    println!("References are stack-sized pointers");

    // --- 5. Structs and enums ---
    // Structs are stack-allocated if all fields are stack-sized.
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    let _p = Point { x: 1.0, y: 2.0 }; // 16 bytes on the stack
    println!("Point: {} bytes, stack", std::mem::size_of::<Point>());

    // But if a struct contains a String, its String data is on the heap:
    #[derive(Debug)]
    struct Person {
        name: String, // handle on stack, chars on heap
        age: u32,     // stack
    }
    let _person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!(
        "Person struct: {} bytes on stack",
        std::mem::size_of::<Person>()
    );
    println!("  (but name data is on the heap)");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fixed_types_sizes() {
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::size_of::<f64>(), 8);
        assert_eq!(std::mem::size_of::<bool>(), 1);
        assert_eq!(std::mem::size_of::<char>(), 4);
    }

    #[test]
    fn test_string_handle_size() {
        // String handle = ptr + len + cap = 3 * usize
        assert_eq!(
            std::mem::size_of::<String>(),
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
    fn test_reference_is_pointer_sized() {
        assert_eq!(std::mem::size_of::<&i32>(), std::mem::size_of::<usize>());
    }
}
