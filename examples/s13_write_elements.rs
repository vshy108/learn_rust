// Demonstrates writing/modifying vector elements.
//
// Run: cargo run --example s13_write_elements

fn main() {
    // --- 1. Direct index assignment ---
    let mut v = vec![10, 20, 30];
    v[1] = 99;
    println!("after v[1] = 99: {v:?}");

    // --- 2. get_mut for safe mutation ---
    if let Some(val) = v.get_mut(2) {
        *val += 1;
    }
    println!("after get_mut: {v:?}");

    // --- 3. iter_mut: mutate all elements ---
    let mut nums = vec![1, 2, 3, 4, 5];
    for n in &mut nums {
        *n *= 2;
    }
    println!("doubled: {nums:?}");

    // --- 4. fill: set all elements to one value ---
    let mut v = vec![0; 5];
    v.fill(42);
    println!("filled: {v:?}");

    // --- 5. fill_with: set elements using a closure ---
    let mut v = vec![0; 5];
    let mut counter = 0;
    v.fill_with(|| {
        counter += 1;
        counter * 10
    });
    println!("fill_with: {v:?}");

    // --- 6. sort ---
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    v.sort();
    println!("sorted: {v:?}");

    // --- 7. sort_by ---
    v.sort_by(|a, b| b.cmp(a)); // reverse order
    println!("reverse sorted: {v:?}");

    // --- 8. reverse ---
    let mut v = vec![1, 2, 3];
    v.reverse();
    println!("reversed: {v:?}");

    // --- 9. swap ---
    let mut v = vec![10, 20, 30, 40];
    v.swap(0, 3); // swap first and last
    println!("after swap: {v:?}");

    // --- 10. resize ---
    let mut v = vec![1, 2, 3];
    v.resize(5, 0); // extend to len 5, filling with 0
    println!("resize bigger: {v:?}");
    v.resize(2, 0); // shrink to len 2
    println!("resize smaller: {v:?}");

    // --- 11. Practical: update by condition ---
    let mut scores = vec![85, 40, 92, 55, 78];
    for score in &mut scores {
        if *score < 60 {
            *score = 60; // minimum passing score
        }
    }
    println!("adjusted scores: {scores:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_direct_assign() {
        let mut v = vec![1, 2, 3];
        v[0] = 99;
        assert_eq!(v[0], 99);
    }

    #[test]
    fn test_sort() {
        let mut v = vec![3, 1, 2];
        v.sort();
        assert_eq!(v, [1, 2, 3]);
    }

    #[test]
    fn test_fill() {
        let mut v = vec![0; 3];
        v.fill(7);
        assert_eq!(v, [7, 7, 7]);
    }

    #[test]
    fn test_resize() {
        let mut v = vec![1, 2];
        v.resize(4, 0);
        assert_eq!(v, [1, 2, 0, 0]);
    }

    #[test]
    fn test_reverse() {
        let mut v = vec![1, 2, 3];
        v.reverse();
        assert_eq!(v, [3, 2, 1]);
    }
}
