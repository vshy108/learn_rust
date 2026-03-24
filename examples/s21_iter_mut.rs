// Demonstrates the iter_mut() method.
//
// `.iter_mut()` yields `&mut T` — mutable references to each element.
// You can modify elements in place.
//
// Run: cargo run --example s21_iter_mut

fn main() {
    println!("=== The iter_mut Method ===");

    // --- 1. Modify elements in place ---
    let mut nums = vec![1, 2, 3, 4, 5];
    println!("before: {nums:?}");
    for n in nums.iter_mut() {
        *n *= 2; // double each element
    }
    println!("after:  {nums:?}");

    // --- 2. Conditional modification ---
    let mut words = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
    for word in words.iter_mut() {
        if word.len() <= 4 {
            *word = word.to_uppercase();
        }
    }
    println!("words: {words:?}");

    // --- 3. Using for_each ---
    let mut scores = vec![85, 92, 78, 95, 88];
    scores.iter_mut().for_each(|s| {
        if *s < 80 {
            *s = 80; // minimum score
        }
    });
    println!("adjusted scores: {scores:?}");

    // --- 4. iter_mut with enumerate ---
    let mut matrix = vec![0; 5];
    for (i, val) in matrix.iter_mut().enumerate() {
        *val = (i + 1) * 10;
    }
    println!("matrix: {matrix:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_iter_mut_double() {
        let mut v = vec![1, 2, 3];
        for n in v.iter_mut() {
            *n *= 2;
        }
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_iter_mut_enumerate() {
        let mut v = vec![0, 0, 0];
        for (i, val) in v.iter_mut().enumerate() {
            *val = i as i32;
        }
        assert_eq!(v, vec![0, 1, 2]);
    }
}
