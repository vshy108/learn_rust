// Demonstrates the cloned method.
//
// `.cloned()` converts an `Iterator<Item = &T>` to `Iterator<Item = T>`
// by cloning each element. Useful to get owned values from references.
//
// Run: cargo run --example s21_cloned

fn main() {
    println!("=== The cloned Method ===");

    // --- 1. Basic cloned ---
    let v = vec![1, 2, 3];
    let iter_refs = v.iter(); // yields &i32
    let owned: Vec<i32> = iter_refs.cloned().collect(); // yields i32
    println!("owned: {owned:?}");

    // --- 2. cloned vs copied ---
    // For Copy types, .copied() is preferred (no allocation):
    let copied: Vec<i32> = v.iter().copied().collect();
    println!("copied: {copied:?}");

    // For Clone-only types (like String), must use .cloned():
    let names = vec!["Alice".to_string(), "Bob".to_string()];
    let owned_names: Vec<String> = names.iter().cloned().collect();
    println!("cloned strings: {owned_names:?}");
    println!("originals still here: {names:?}");

    // --- 3. cloned in filter chains ---
    let nums = vec![1, 2, 3, 4, 5];
    let even_owned: Vec<i32> = nums.iter().filter(|&&n| n % 2 == 0).cloned().collect();
    println!("even (owned): {even_owned:?}");

    // --- 4. Without cloned — you get references ---
    let even_refs: Vec<&i32> = nums.iter().filter(|&&n| n % 2 == 0).collect();
    println!("even (refs): {even_refs:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cloned() {
        let v = vec![1, 2, 3];
        let owned: Vec<i32> = v.iter().cloned().collect();
        assert_eq!(owned, v);
    }

    #[test]
    fn test_copied() {
        let v = vec![1, 2, 3];
        let owned: Vec<i32> = v.iter().copied().collect();
        assert_eq!(owned, v);
    }

    #[test]
    fn test_cloned_strings() {
        let v = vec!["a".to_string()];
        let cloned: Vec<String> = v.iter().cloned().collect();
        assert_eq!(cloned[0], "a");
        assert_eq!(v.len(), 1); // original unchanged
    }
}
