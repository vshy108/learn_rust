// Demonstrates the iter() method.
//
// `.iter()` borrows the collection, yielding `&T` references.
// The collection remains usable after iteration.
//
// Run: cargo run --example s21_iter

fn main() {
    println!("=== The iter Method ===");

    let names = vec!["Alice", "Bob", "Charlie"];

    // iter() yields &T:
    for name in names.iter() {
        println!("  {name}"); // name is &&str here
    }
    println!("names still here: {names:?}");

    // Chaining with iterator adaptors:
    let lengths: Vec<usize> = names.iter().map(|n| n.len()).collect();
    println!("lengths: {lengths:?}");

    // Filtering:
    let long_names: Vec<&&str> = names.iter().filter(|n| n.len() > 3).collect();
    println!("long names: {long_names:?}");

    // With numbers:
    let nums = vec![1, 2, 3, 4, 5];
    let sum: i32 = nums.iter().sum();
    println!("sum: {sum}");
    println!("nums unchanged: {nums:?}");

    // iter() on arrays:
    let arr = [10, 20, 30];
    let doubled: Vec<i32> = arr.iter().map(|&x| x * 2).collect();
    println!("doubled: {doubled:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_iter_borrows() {
        let v = vec![1, 2, 3];
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 6);
        assert_eq!(v.len(), 3); // still usable
    }

    #[test]
    fn test_iter_map() {
        let v = vec![1, 2, 3];
        let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }
}
