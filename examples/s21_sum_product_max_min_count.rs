// Demonstrates sum, product, max, min, and count.
//
// These are "consuming adaptors" — they consume the iterator
// and return a single value.
//
// Run: cargo run --example s21_sum_product_max_min_count

fn main() {
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // --- 1. sum ---
    let sum: i32 = nums.iter().sum();
    println!("sum: {sum}");

    // --- 2. product ---
    let product: i32 = vec![1, 2, 3, 4, 5].iter().product();
    println!("product: {product}"); // 120

    // --- 3. max and min ---
    let max = nums.iter().max();
    let min = nums.iter().min();
    println!("max: {max:?}, min: {min:?}");

    // --- 4. count ---
    let count = nums.iter().count();
    println!("count: {count}");

    // --- 5. Chaining with filter ---
    let even_count = nums.iter().filter(|&&n| n % 2 == 0).count();
    let even_sum: i32 = nums.iter().filter(|&&n| n % 2 == 0).sum();
    println!("even count: {even_count}, even sum: {even_sum}");

    // --- 6. max_by and min_by for custom comparison ---
    let words = vec!["short", "medium", "extraordinarily"];
    let longest = words.iter().max_by_key(|w| w.len());
    let shortest = words.iter().min_by_key(|w| w.len());
    println!("longest: {longest:?}, shortest: {shortest:?}");

    // --- 7. Floats need max_by (no Ord for f64) ---
    let floats = vec![1.5, 0.3, 2.7, 1.1];
    let max_f = floats.iter().copied().reduce(f64::max);
    println!("max float: {max_f:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum() {
        let sum: i32 = vec![1, 2, 3].iter().sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_product() {
        let p: i32 = vec![2, 3, 4].iter().product();
        assert_eq!(p, 24);
    }

    #[test]
    fn test_max_min() {
        let v = vec![5, 1, 3];
        assert_eq!(v.iter().max(), Some(&5));
        assert_eq!(v.iter().min(), Some(&1));
    }

    #[test]
    fn test_count() {
        assert_eq!((0..10).count(), 10);
    }
}
