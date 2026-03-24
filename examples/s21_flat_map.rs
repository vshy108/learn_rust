// Demonstrates the flat_map method.
//
// `flat_map` = `map` + `flatten`. The closure returns an iterator
// (or anything IntoIterator), and results are flattened.
//
// Run: cargo run --example s21_flat_map

fn main() {
    println!("=== The flat_map Method ===");

    // --- 1. Split words into characters ---
    let words = vec!["hello", "world"];
    let chars: Vec<char> = words.iter().flat_map(|w| w.chars()).collect();
    println!("chars: {chars:?}");

    // --- 2. One-to-many mapping ---
    let nums = vec![1, 2, 3];
    let expanded: Vec<i32> = nums.iter().flat_map(|&n| vec![n, n * 10]).collect();
    println!("expanded: {expanded:?}"); // [1, 10, 2, 20, 3, 30]

    // --- 3. Parse with flat_map (skip errors) ---
    let inputs = vec!["1", "abc", "3", "def", "5"];
    let parsed: Vec<i32> = inputs.iter().flat_map(|s| s.parse::<i32>()).collect();
    println!("parsed: {parsed:?}"); // [1, 3, 5]

    // --- 4. Flatten nested structure ---
    #[derive(Debug)]
    struct Department {
        members: Vec<String>,
    }
    let departments = vec![
        Department {
            members: vec!["Alice".into(), "Bob".into()],
        },
        Department {
            members: vec!["Charlie".into()],
        },
        Department {
            members: vec!["Diana".into(), "Eve".into()],
        },
    ];
    let all_members: Vec<&String> = departments.iter().flat_map(|d| &d.members).collect();
    println!("all members: {all_members:?}");

    // --- 5. Range expansion ---
    let ranges: Vec<i32> = vec![(1, 3), (5, 7)]
        .into_iter()
        .flat_map(|(start, end)| start..=end)
        .collect();
    println!("ranges: {ranges:?}"); // [1, 2, 3, 5, 6, 7]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_flat_map_chars() {
        let v: Vec<char> = vec!["ab", "cd"].iter().flat_map(|s| s.chars()).collect();
        assert_eq!(v, vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_flat_map_expand() {
        let v: Vec<i32> = vec![1, 2].iter().flat_map(|&n| vec![n, n * 10]).collect();
        assert_eq!(v, vec![1, 10, 2, 20]);
    }
}
