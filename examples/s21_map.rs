// Demonstrates the map method.
//
// `.map(closure)` transforms each element using the closure.
// It's lazy — nothing happens until you consume the iterator.
//
// Run: cargo run --example s21_map

fn main() {
    println!("=== The map Method ===");

    // --- 1. Basic map ---
    let nums = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|&n| n * 2).collect();
    println!("doubled: {doubled:?}");

    // --- 2. Transform types ---
    let strings: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
    println!("strings: {strings:?}");

    // --- 3. Map is lazy ---
    let _lazy = nums.iter().map(|n| {
        println!("  processing {n}"); // only prints when consumed
        n * 2
    });
    println!("(lazy map created, nothing printed yet)");
    let _consumed: Vec<_> = nums
        .iter()
        .map(|n| {
            print!("{n} ");
            n * 2
        })
        .collect();
    println!("(now consumed)");

    // --- 4. Chaining maps ---
    let result: Vec<String> = (1..=5)
        .map(|n| n * n) // square
        .map(|n| n + 1) // add 1
        .map(|n| format!("#{n}")) // format
        .collect();
    println!("chained: {result:?}");

    // --- 5. Map with strings ---
    let words = vec!["hello", "world", "rust"];
    let upper: Vec<String> = words.iter().map(|w| w.to_uppercase()).collect();
    println!("upper: {upper:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_map_double() {
        let v: Vec<i32> = vec![1, 2, 3].iter().map(|&n| n * 2).collect();
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_map_type_transform() {
        let v: Vec<String> = vec![1, 2].iter().map(|n| n.to_string()).collect();
        assert_eq!(v, vec!["1", "2"]);
    }
}
