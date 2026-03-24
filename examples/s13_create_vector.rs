// Demonstrates creating vectors in Rust.
//
// `Vec<T>` is a growable, heap-allocated array. Think of it as
// Python's `list`, but every element must be the SAME type.
//
// Two main ways to create:
// - `Vec::new()` — empty vector
// - `vec![...]` — macro with initial values
//
// Run: cargo run --example s13_create_vector

fn main() {
    // --- 1. Vec::new() — empty vector ---
    // Must annotate the type because Rust can't infer from an empty vec.
    let v1: Vec<i32> = Vec::new();
    println!("v1: {v1:?}");

    // --- 2. vec! macro — creates with initial elements ---
    let v2 = vec![1, 2, 3, 4, 5]; // type inferred as Vec<i32>
    println!("v2: {v2:?}");

    // --- 3. vec! with repeated values ---
    let zeros = vec![0; 5]; // [0, 0, 0, 0, 0]
    println!("zeros: {zeros:?}");

    // --- 4. Vec::with_capacity — preallocate ---
    // Like Python's list but you hint at the size upfront.
    // This avoids reallocations as the vector grows.
    let mut v3 = Vec::with_capacity(10);
    println!("v3 len: {}, capacity: {}", v3.len(), v3.capacity());
    v3.push(42);
    println!("v3 after push: len={}, cap={}", v3.len(), v3.capacity());

    // --- 5. From an iterator ---
    let v4: Vec<i32> = (1..=5).collect();
    println!("v4 from range: {v4:?}");

    // --- 6. From other collections ---
    let arr = [10, 20, 30];
    let v5 = arr.to_vec();
    println!("v5 from array: {v5:?}");

    let v6 = Vec::from([100, 200]);
    println!("v6 from array: {v6:?}");

    // --- 7. Vec of Strings ---
    let names: Vec<String> = vec!["Alice".into(), "Bob".into()];
    println!("names: {names:?}");

    // --- 8. Empty vec inferred from usage ---
    let mut v7 = Vec::new(); // type inferred when we push
    v7.push(3.14_f64);
    println!("v7: {v7:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_empty() {
        let v: Vec<i32> = Vec::new();
        assert!(v.is_empty());
    }

    #[test]
    fn test_vec_macro() {
        let v = vec![1, 2, 3];
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], 1);
    }

    #[test]
    fn test_vec_repeat() {
        let v = vec![0; 4];
        assert_eq!(v, [0, 0, 0, 0]);
    }

    #[test]
    fn test_with_capacity() {
        let v: Vec<i32> = Vec::with_capacity(10);
        assert_eq!(v.len(), 0);
        assert!(v.capacity() >= 10);
    }

    #[test]
    fn test_from_range() {
        let v: Vec<i32> = (1..=3).collect();
        assert_eq!(v, [1, 2, 3]);
    }
}
