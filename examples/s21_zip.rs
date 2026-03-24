// Demonstrates the zip method.
//
// `.zip(other)` pairs elements from two iterators into tuples.
// Stops when either iterator is exhausted.
//
// Run: cargo run --example s21_zip

fn main() {
    println!("=== The zip Method ===");

    // --- 1. Basic zip ---
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];
    let paired: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("paired: {paired:?}");

    // --- 2. Zip stops at shorter iterator ---
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![10, 20, 30];
    let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("zipped (truncated): {zipped:?}"); // only 3 pairs

    // --- 3. Zip with map ---
    let sums: Vec<i32> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
    println!("sums: {sums:?}");

    // --- 4. Zip with range ---
    let indexed: Vec<_> = (0..).zip(names.iter()).collect();
    println!("indexed: {indexed:?}");

    // --- 5. Zip into HashMap ---
    let keys = vec!["x", "y", "z"];
    let vals = vec![1, 2, 3];
    let map: std::collections::HashMap<_, _> = keys.iter().zip(vals.iter()).collect();
    println!("map: {map:?}");

    // --- 6. Unzip (reverse of zip) ---
    let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    let (nums, chars): (Vec<i32>, Vec<char>) = pairs.into_iter().unzip();
    println!("nums: {nums:?}, chars: {chars:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_zip() {
        let a = vec![1, 2, 3];
        let b = vec!['a', 'b', 'c'];
        let z: Vec<_> = a.iter().zip(b.iter()).collect();
        assert_eq!(z, vec![(&1, &'a'), (&2, &'b'), (&3, &'c')]);
    }

    #[test]
    fn test_zip_unequal_length() {
        let a = vec![1, 2, 3];
        let b = vec![10, 20];
        let z: Vec<_> = a.iter().zip(b.iter()).collect();
        assert_eq!(z.len(), 2);
    }
}
