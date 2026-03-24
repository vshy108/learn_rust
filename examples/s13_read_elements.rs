// Demonstrates reading elements from a vector.
//
// Two main approaches:
// - `v[i]` — direct indexing; panics if out of bounds.
// - `v.get(i)` — returns Option<&T>; safe for unknown indices.
//
// Run: cargo run --example s13_read_elements

fn main() {
    let colors = vec!["red", "green", "blue", "yellow"];

    // --- 1. Direct indexing ---
    println!("colors[0] = {}", colors[0]);
    println!("colors[2] = {}", colors[2]);
    // colors[10]; // PANIC: index out of bounds!

    // --- 2. Safe access with .get() ---
    match colors.get(1) {
        Some(c) => println!("get(1) = {c}"),
        None => println!("get(1) = not found"),
    }
    println!("get(99) = {:?}", colors.get(99)); // None

    // --- 3. first and last ---
    println!("first: {:?}", colors.first());
    println!("last:  {:?}", colors.last());

    // --- 4. Slicing ---
    let slice = &colors[1..3]; // ["green", "blue"]
    println!("slice [1..3]: {slice:?}");

    // --- 5. Iterating ---
    print!("iter: ");
    for color in &colors {
        print!("{color} ");
    }
    println!();

    // --- 6. Iterating with index ---
    for (i, color) in colors.iter().enumerate() {
        println!("  [{i}] = {color}");
    }

    // --- 7. contains ---
    println!("contains \"blue\": {}", colors.contains(&"blue"));
    println!("contains \"purple\": {}", colors.contains(&"purple"));

    // --- 8. len and is_empty ---
    println!("len: {}", colors.len());
    println!("is_empty: {}", colors.is_empty());

    // --- 9. Reading with destructuring ---
    if let [first, second, ..] = colors.as_slice() {
        println!("first two: {first}, {second}");
    }

    // --- 10. windows and chunks ---
    let nums = vec![1, 2, 3, 4, 5];
    println!("windows(3):");
    for w in nums.windows(3) {
        println!("  {w:?}");
    }
    println!("chunks(2):");
    for c in nums.chunks(2) {
        println!("  {c:?}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_index() {
        let v = vec![10, 20, 30];
        assert_eq!(v[1], 20);
    }

    #[test]
    fn test_get() {
        let v = vec![10, 20, 30];
        assert_eq!(v.get(0), Some(&10));
        assert_eq!(v.get(5), None);
    }

    #[test]
    fn test_first_last() {
        let v = vec![1, 2, 3];
        assert_eq!(v.first(), Some(&1));
        assert_eq!(v.last(), Some(&3));
    }

    #[test]
    fn test_contains() {
        let v = vec!["a", "b", "c"];
        assert!(v.contains(&"b"));
        assert!(!v.contains(&"z"));
    }

    #[test]
    fn test_slice() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(&v[1..3], &[2, 3]);
    }
}
