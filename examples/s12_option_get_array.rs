// Demonstrates Option with array/slice `.get()` method.
//
// Instead of panicking on out-of-bounds access like `arr[i]`,
// `.get(i)` returns `Option<&T>`:
// - `Some(&value)` if the index is valid
// - `None` if out of bounds
//
// This is safer than direct indexing for user-controlled indices.
//
// Run: cargo run --example s12_option_get_array

fn main() {
    let fruits = ["apple", "banana", "cherry"];

    // --- 1. Safe access with .get() ---
    let first = fruits.get(0);
    let fourth = fruits.get(3);
    println!("get(0) = {first:?}"); // Some("apple")
    println!("get(3) = {fourth:?}"); // None

    // --- 2. Compare with direct indexing ---
    let direct = fruits[0]; // OK
    println!("direct: {direct}");
    // fruits[10]; // PANIC: index out of bounds!

    // --- 3. Pattern matching on .get() ---
    match fruits.get(1) {
        Some(fruit) => println!("Found: {fruit}"),
        None => println!("Not found"),
    }

    // --- 4. Vec::get() works the same way ---
    let numbers = vec![10, 20, 30, 40];
    for i in 0..6 {
        match numbers.get(i) {
            Some(n) => println!("  [{i}] = {n}"),
            None => println!("  [{i}] = out of bounds"),
        }
    }

    // --- 5. get() with ranges returns an Option<&[T]> ---
    let slice = numbers.get(1..3); // Some(&[20, 30])
    println!("get(1..3) = {slice:?}");
    let bad = numbers.get(2..10); // None
    println!("get(2..10) = {bad:?}");

    // --- 6. Practical: safely find an element ---
    let names = vec!["Alice", "Bob", "Charlie"];
    let index: usize = 5;
    let name = names.get(index).unwrap_or(&"Unknown");
    println!("Name at {index}: {name}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_valid() {
        let arr = [1, 2, 3];
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(2), Some(&3));
    }

    #[test]
    fn test_get_out_of_bounds() {
        let arr = [1, 2, 3];
        assert_eq!(arr.get(5), None);
    }

    #[test]
    fn test_get_range() {
        let v = vec![10, 20, 30, 40];
        assert_eq!(v.get(1..3), Some(&[20, 30][..]));
        assert_eq!(v.get(3..10), None);
    }

    #[test]
    fn test_unwrap_or() {
        let v = vec![1, 2, 3];
        assert_eq!(*v.get(0).unwrap_or(&0), 1);
        assert_eq!(*v.get(10).unwrap_or(&0), 0);
    }
}
