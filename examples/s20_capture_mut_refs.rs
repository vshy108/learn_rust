// Demonstrates closures capturing mutable references.
//
// If a closure modifies a captured variable, it captures by &mut T.
// The closure itself must be declared `mut` (mutable binding).
//
// Run: cargo run --example s20_capture_mut_refs

fn main() {
    println!("=== Closures Capturing Mutable References ===");

    // --- 1. Basic mutable capture ---
    let mut count = 0;
    let mut increment = || {
        count += 1; // captures count by &mut
        count
    };
    println!("count: {}", increment());
    println!("count: {}", increment());
    println!("count: {}", increment());
    // Can't use `count` while `increment` borrows it mutably.
    // After last use of increment:
    drop(increment); // explicitly end the borrow
    println!("final count: {count}"); // now we can read it

    // --- 2. Mutating a Vec ---
    let mut items = Vec::new();
    let mut push_item = |item: &str| {
        items.push(item.to_string());
    };
    push_item("alpha");
    push_item("beta");
    push_item("gamma");
    drop(push_item);
    println!("items: {items:?}");

    // --- 3. Mutable closure in for_each ---
    let mut sum = 0;
    [1, 2, 3, 4, 5].iter().for_each(|&n| {
        sum += n;
    });
    println!("sum: {sum}");

    // --- 4. Closure that sorts (needs &mut) ---
    let mut data = vec![3, 1, 4, 1, 5];
    let mut sort_data = || data.sort();
    sort_data();
    drop(sort_data);
    println!("sorted: {data:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mut_capture() {
        let mut count = 0;
        let mut inc = || {
            count += 1;
            count
        };
        assert_eq!(inc(), 1);
        assert_eq!(inc(), 2);
    }

    #[test]
    fn test_mut_vec_capture() {
        let mut v = Vec::new();
        let mut push = |x: i32| v.push(x);
        push(1);
        push(2);
        drop(push);
        assert_eq!(v, vec![1, 2]);
    }
}
