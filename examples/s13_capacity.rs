// Demonstrates vector capacity — how Vec manages memory internally.
//
// A Vec has two sizes:
// - **len**: number of elements currently stored.
// - **capacity**: total space allocated (elements it CAN hold before realloc).
//
// When len == capacity and you push, Vec:
// 1. Allocates a new, larger buffer (typically 2x).
// 2. Copies all elements to the new buffer.
// 3. Frees the old buffer.
//
// This is why `Vec::with_capacity(n)` is useful when you know the size —
// it avoids expensive reallocations.
//
// Run: cargo run --example s13_capacity

fn main() {
    // --- 1. Default capacity ---
    let v: Vec<i32> = Vec::new();
    println!("new:   len={}, cap={}", v.len(), v.capacity());

    // --- 2. Watch capacity grow ---
    let mut v = Vec::new();
    println!("Empty: len={}, cap={}", v.len(), v.capacity());
    for i in 0..20 {
        v.push(i);
        println!("  push {i:2}: len={:2}, cap={:2}", v.len(), v.capacity());
    }
    // Capacity roughly doubles: 0 → 4 → 8 → 16 → 32...
    // (Exact growth strategy is an implementation detail.)

    // --- 3. with_capacity avoids reallocation ---
    let mut v = Vec::with_capacity(20);
    println!("\nwith_capacity(20): len={}, cap={}", v.len(), v.capacity());
    for i in 0..20 {
        v.push(i);
    }
    println!("after 20 pushes:   len={}, cap={}", v.len(), v.capacity());
    // No reallocation happened! Capacity stayed at 20.

    // --- 4. shrink_to_fit: release excess capacity ---
    let mut v = Vec::with_capacity(100);
    v.extend([1, 2, 3]);
    println!("\nbefore shrink: len={}, cap={}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("after shrink:  len={}, cap={}", v.len(), v.capacity());

    // --- 5. shrink_to: shrink to at least n ---
    let mut v = Vec::with_capacity(100);
    v.extend([1, 2, 3]);
    v.shrink_to(10);
    println!("shrink_to(10): len={}, cap={}", v.len(), v.capacity());

    // --- 6. reserve: ensure minimum additional capacity ---
    let mut v = vec![1, 2, 3];
    println!("\nbefore reserve: cap={}", v.capacity());
    v.reserve(100); // ensure space for 100 MORE elements
    println!("after reserve(100): cap={}", v.capacity());

    // --- 7. Why this matters: performance ---
    // BAD: many small allocations
    let mut slow = Vec::new();
    for i in 0..1000 {
        slow.push(i);
    }

    // GOOD: single allocation
    let mut fast = Vec::with_capacity(1000);
    for i in 0..1000 {
        fast.push(i);
    }
    // Both produce the same result, but `fast` avoids ~10 reallocations.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_with_capacity() {
        let v: Vec<i32> = Vec::with_capacity(50);
        assert_eq!(v.len(), 0);
        assert!(v.capacity() >= 50);
    }

    #[test]
    fn test_push_grows_capacity() {
        let mut v: Vec<i32> = Vec::new();
        assert_eq!(v.capacity(), 0);
        v.push(1);
        assert!(v.capacity() > 0);
    }

    #[test]
    fn test_shrink_to_fit() {
        let mut v = Vec::with_capacity(100);
        v.push(1);
        v.shrink_to_fit();
        assert!(v.capacity() < 100);
    }

    #[test]
    fn test_reserve() {
        let mut v = vec![1];
        v.reserve(100);
        assert!(v.capacity() >= 101);
    }
}
