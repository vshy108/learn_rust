// Demonstrates that Vec and String are smart pointers.
//
// `Vec<T>` and `String` own heap-allocated data, implement `Deref`,
// and automatically free memory when dropped — just like Box.
//
// Run: cargo run --example s26_vectors_smart_pointers

fn main() {
    println!("=== Vectors are Smart Pointers ===");

    // --- 1. Vec<T> owns heap data ---
    let v = vec![1, 2, 3, 4, 5];
    println!("vec ptr: {:?}", v.as_ptr());
    println!("vec len: {}, capacity: {}", v.len(), v.capacity());
    // Vec stores: pointer + length + capacity on the stack (24 bytes)
    println!(
        "Vec<i32> stack size: {} bytes",
        std::mem::size_of::<Vec<i32>>()
    );

    // --- 2. Vec derefs to &[T] ---
    // Deref coercion: &Vec<i32> -> &[i32]
    fn sum_slice(s: &[i32]) -> i32 {
        s.iter().sum()
    }
    println!("sum: {}", sum_slice(&v));

    // --- 3. String owns heap data ---
    let s = String::from("hello");
    println!("String ptr: {:?}", s.as_ptr());
    println!("String len: {}, capacity: {}", s.len(), s.capacity());
    println!("String stack size: {} bytes", std::mem::size_of::<String>());

    // --- 4. String derefs to &str ---
    fn greet(name: &str) {
        println!("Hello, {name}!");
    }
    greet(&s); // Deref coercion: &String -> &str

    // --- 5. Drop behavior ---
    {
        let temp = vec![1, 2, 3];
        println!("temp exists: {temp:?}");
        // temp's heap memory is freed here
    }
    println!("temp has been dropped");

    // --- 6. Clone vs move ---
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone(); // deep copy
    let v3 = v2; // move (v2 no longer valid)
    println!("cloned and moved: {v3:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vec_deref() {
        let v = vec![1, 2, 3];
        let s: &[i32] = &v; // Deref coercion
        assert_eq!(s, &[1, 2, 3]);
    }

    #[test]
    fn test_string_deref() {
        let s = String::from("hello");
        let r: &str = &s; // Deref coercion
        assert_eq!(r, "hello");
    }

    #[test]
    fn test_vec_stack_size() {
        // Vec is 3 * usize: pointer + len + capacity
        assert_eq!(
            std::mem::size_of::<Vec<i32>>(),
            3 * std::mem::size_of::<usize>()
        );
    }
}
