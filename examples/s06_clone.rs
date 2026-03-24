// Demonstrates `clone()` — explicit deep copying.
//
// In Rust, assignment of non-Copy types MOVES ownership. If you need a
// second independent copy, call `.clone()`. This trait method performs a
// "deep copy" — allocating new heap memory and duplicating the data.
//
// `Clone` is explicit and potentially expensive. `Copy` is implicit and cheap.
//
// Key points:
// - `.clone()` creates a new, independent value.
// - The original remains valid after `.clone()`.
// - Deriving `Clone` auto-generates it for all fields.
// - `Clone` can be implemented manually for custom deep-copy logic.
//
// Run: cargo run --example s06_clone

fn main() {
    // --- 1. String clone ---
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Deep copy: new heap allocation
    println!("s1 = {s1}, s2 = {s2}"); // Both valid!
    // Without .clone(), `let s2 = s1;` would MOVE s1.

    // --- 2. Vec clone ---
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();
    println!("v1 = {v1:?}, v2 = {v2:?}");

    // --- 3. Deriving Clone on custom struct ---
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    let alice = Person {
        name: "Alice".into(),
        age: 30,
    };
    let alice2 = alice.clone();
    println!("original: {alice:?}");
    println!("clone:    {alice2:?}");

    // --- 4. Clone vs Copy ---
    // i32 is Copy — assignment is an implicit bitwise copy, no .clone() needed.
    let x: i32 = 42;
    let y = x; // Copy, not move
    println!("x = {x}, y = {y}");

    // String is NOT Copy — assignment moves. Use clone for a duplicate.
    let a = String::from("moved?");
    let _b = a.clone(); // Clone OK
    println!("a still valid: {a}");

    // --- 5. clone() can be expensive ---
    // Cloning a large Vec copies ALL elements to a new heap allocation.
    let big = vec![0u8; 1_000_000]; // 1 MB
    let _big2 = big.clone(); // Another 1 MB allocated
    println!("Cloned a 1MB vector");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_string_clone_independent() {
        let s1 = String::from("test");
        let mut s2 = s1.clone();
        s2.push_str("_modified");
        // Original unchanged — they're independent.
        assert_eq!(s1, "test");
        assert_eq!(s2, "test_modified");
    }

    #[test]
    fn test_vec_clone_independent() {
        let v1 = vec![1, 2, 3];
        let mut v2 = v1.clone();
        v2.push(4);
        assert_eq!(v1, vec![1, 2, 3]);
        assert_eq!(v2, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_derived_clone() {
        #[derive(Debug, Clone, PartialEq)]
        struct Point {
            x: f64,
            y: f64,
        }
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }
}
