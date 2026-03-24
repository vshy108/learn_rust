// Demonstrates sort and sort_by_key.
//
// Sorting methods on Vec (not iterators, but commonly combined with them).
//
// Run: cargo run --example s21_sort

fn main() {
    // --- 1. sort (requires Ord) ---
    let mut nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    nums.sort();
    println!("sorted: {nums:?}");

    // --- 2. sort_unstable (faster, doesn't preserve equal-element order) ---
    let mut nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    nums.sort_unstable();
    println!("sort_unstable: {nums:?}");

    // --- 3. sort descending ---
    let mut nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    nums.sort_by(|a, b| b.cmp(a));
    println!("descending: {nums:?}");

    // --- 4. sort_by_key ---
    let mut words = vec!["banana", "apple", "cherry", "date"];
    words.sort_by_key(|w| w.len());
    println!("by length: {words:?}");

    // --- 5. Sort structs ---
    #[derive(Debug)]
    struct Student {
        name: String,
        gpa: f64,
    }

    let mut students = vec![
        Student {
            name: "Alice".into(),
            gpa: 3.8,
        },
        Student {
            name: "Bob".into(),
            gpa: 3.5,
        },
        Student {
            name: "Charlie".into(),
            gpa: 3.9,
        },
    ];

    // Sort by GPA descending:
    students.sort_by(|a, b| b.gpa.partial_cmp(&a.gpa).unwrap());
    for s in &students {
        println!("  {} ({:.1})", s.name, s.gpa);
    }

    // --- 6. sorted iterator (collects into new Vec) ---
    let v = vec![5, 2, 8, 1];
    let mut sorted = v.clone();
    sorted.sort();
    println!("original: {v:?}");
    println!("sorted copy: {sorted:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sort() {
        let mut v = vec![3, 1, 2];
        v.sort();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_sort_by_key() {
        let mut v = vec!["cc", "a", "bb"];
        v.sort_by_key(|s| s.len());
        assert_eq!(v, vec!["a", "cc", "bb"]); // stable sort: cc before bb
    }

    #[test]
    fn test_sort_descending() {
        let mut v = vec![1, 3, 2];
        v.sort_by(|a, b| b.cmp(a));
        assert_eq!(v, vec![3, 2, 1]);
    }
}
