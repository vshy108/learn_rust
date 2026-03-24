// Demonstrates how for loops work with iterators.
//
// `for x in collection` desugars to:
//   let mut iter = collection.into_iter();
//   while let Some(x) = iter.next() { ... }
//
// Run: cargo run --example s21_for_loop_iterator

fn main() {
    println!("=== for Loop with Iterator ===");

    // --- 1. Sugar ---
    let v = vec![1, 2, 3];
    for n in &v {
        print!("{n} ");
    }
    println!();

    // --- 2. Desugared equivalent ---
    let mut iter = (&v).into_iter();
    while let Some(n) = iter.next() {
        print!("{n} ");
    }
    println!();

    // --- 3. for with range ---
    for i in 0..5 {
        print!("{i} ");
    }
    println!();

    // --- 4. for with enumerate ---
    let fruits = ["apple", "banana", "cherry"];
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  {i}: {fruit}");
    }

    // --- 5. for with pattern matching ---
    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    for (num, name) in &pairs {
        println!("  {num} = {name}");
    }

    // --- 6. break and continue work normally ---
    for n in 0..100 {
        if n % 2 != 0 {
            continue;
        }
        if n > 10 {
            break;
        }
        print!("{n} ");
    }
    println!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_for_desugars() {
        let v = vec![1, 2, 3];
        let mut sum1 = 0;
        for &n in &v {
            sum1 += n;
        }

        let mut sum2 = 0;
        let mut iter = v.iter();
        while let Some(&n) = iter.next() {
            sum2 += n;
        }
        assert_eq!(sum1, sum2);
    }
}
