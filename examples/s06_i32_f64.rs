// Demonstrates that i32 and f64 live on the stack.
//
// In Rust, fixed-size types (i32, f64, bool, char, arrays of known size)
// are allocated on the **stack**. They implement the `Copy` trait, meaning
// assignment copies bits rather than moving ownership.
//
// In Python, ALL objects live on the heap — even `int`. Rust is different.
//
// Run: cargo run --example s06_i32_f64

fn main() {
    // --- 1. Stack allocation ---
    // `a` is an i32 (4 bytes) on the stack. `b` is a *copy* of those 4 bytes.
    let a: i32 = 42;
    let b = a; // bitwise copy — `a` is still valid
    println!("a = {a}, b = {b}"); // both usable

    // Compare: String (heap-allocated) would *move* instead of copy.
    // `let s1 = String::from("hi"); let s2 = s1;` — s1 is now invalid.

    // --- 2. f64 is also stack-allocated ---
    let x: f64 = 3.14;
    let y = x; // copy
    println!("x = {x}, y = {y}");

    // --- 3. Size on the stack ---
    println!("size of i32: {} bytes", std::mem::size_of::<i32>());
    println!("size of f64: {} bytes", std::mem::size_of::<f64>());
    println!("size of bool: {} byte", std::mem::size_of::<bool>());
    println!("size of char: {} bytes", std::mem::size_of::<char>());

    // --- 4. Why stack matters ---
    // Stack allocation is extremely fast (just move the stack pointer).
    // No allocator overhead, no garbage collector, deterministic cleanup.
    // When a variable goes out of scope, its stack memory is reclaimed instantly.

    // --- 5. Passing to functions copies the value ---
    let n = 100;
    print_value(n);
    println!("n after print_value: {n}"); // still valid — was copied

    // --- 6. Arrays of fixed size are also stack-allocated ---
    let arr = [1, 2, 3, 4, 5]; // [i32; 5] = 20 bytes on the stack
    let arr2 = arr; // copies all 20 bytes
    println!("arr = {arr:?}, arr2 = {arr2:?}");
}

fn print_value(val: i32) {
    println!("print_value received: {val}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_i32_copy() {
        let a = 42;
        let b = a;
        assert_eq!(a, b); // `a` still valid after copy
    }

    #[test]
    fn test_f64_copy() {
        let x: f64 = 3.14;
        let y = x;
        assert!((x - y).abs() < f64::EPSILON);
    }

    #[test]
    fn test_array_copy() {
        let a = [1, 2, 3];
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_sizes() {
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::size_of::<f64>(), 8);
    }
}
