// Demonstrates Box vs regular references.
//
// Key differences:
// - `&T`: borrows data, doesn't own it. The data must outlive the reference.
// - `Box<T>`: owns the data on the heap. Dropped when Box goes out of scope.
// - References have zero cost; Box has a heap allocation.
//
// **Python analogy:** Python variables are all like Box — they own a reference
// to heap data. Rust references are more like C pointers but with safety.
//
// Run: cargo run --example s26_box_vs_refs

fn main() {
    println!("=== Box vs Regular References ===");

    // --- 1. Reference borrows, Box owns ---
    let val = 42;
    let ref_val: &i32 = &val; // borrows val
    let box_val: Box<i32> = Box::new(42); // owns its own copy on heap
    println!("ref: {ref_val}, box: {box_val}");

    // --- 2. Size difference ---
    println!("&i32 size: {} bytes", std::mem::size_of::<&i32>()); // 8 (pointer)
    println!("Box<i32> size: {} bytes", std::mem::size_of::<Box<i32>>()); // 8 (pointer)
    // Both are pointer-sized, but Box manages heap memory

    // --- 3. Returning owned data ---
    // Can't return a reference to local data, but can return a Box
    let owned = make_boxed_string();
    println!("owned: {owned}");

    // --- 4. Box auto-derefs like a reference ---
    let b = Box::new(String::from("hello"));
    println!("len via deref: {}", b.len()); // b.len() == (*b).len()

    // --- 5. Box can be moved, references can't ---
    let b1 = Box::new(100);
    let b2 = b1; // ownership moves
    // b1 is no longer valid
    println!("moved box: {b2}");
}

/// Returns a boxed string — the caller owns the heap data.
fn make_boxed_string() -> Box<String> {
    Box::new(String::from("I live on the heap!"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_box_deref() {
        let b = Box::new(42);
        let r: &i32 = &b; // Box auto-derefs to &i32
        assert_eq!(*r, 42);
    }

    #[test]
    fn test_box_move() {
        let b1 = Box::new(String::from("hello"));
        let b2 = b1;
        assert_eq!(*b2, "hello");
    }
}
