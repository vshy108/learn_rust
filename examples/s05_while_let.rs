// Demonstrates `while let` — a loop that runs while a pattern matches.
//
// `while let` is to `while` what `if let` is to `if`:
// it destructures a value each iteration and stops when the pattern fails.
//
// Most commonly used with `Option` (e.g., popping from a stack) or iterators.
//
// Run: cargo run --example s05_while_let

fn main() {
    // --- 1. Basic while let with a stack (Vec::pop) ---
    // `Vec::pop()` returns `Option<T>`: `Some(val)` or `None` when empty.
    let mut stack = vec![1, 2, 3, 4, 5];

    // Pop elements until the stack is empty:
    while let Some(top) = stack.pop() {
        println!("popped: {top}");
    }
    // Prints 5, 4, 3, 2, 1 (LIFO order).
    println!("stack is now empty: {stack:?}");

    // --- 2. Equivalent match-based loop (verbose) ---
    let mut stack2 = vec![10, 20, 30];
    loop {
        match stack2.pop() {
            Some(val) => println!("match-popped: {val}"),
            None => break,
        }
    }

    // --- 3. while let with iterator.next() ---
    // Iterators return `Option<Item>`: `Some(item)` or `None` when exhausted.
    // A `for` loop is usually preferred, but `while let` gives more control.
    let names = vec!["Alice", "Bob", "Charlie"];
    let mut iter = names.iter();
    while let Some(name) = iter.next() {
        println!("name: {name}");
    }

    // --- 4. while let with nested patterns ---
    let data: Vec<Option<i32>> = vec![Some(1), None, Some(3), Some(4)];
    let mut iter = data.into_iter();
    // This stops at the first `None` because `Some(Some(val))` won't match `None`.
    while let Some(Some(val)) = iter.next() {
        println!("nested: {val}");
    }
    // Only prints "nested: 1" — stops at the None.

    // --- 5. while let vs loop + match ---
    // Use `while let` when:
    //   - You only care about one pattern variant.
    //   - The loop should stop when the pattern fails.
    // Use `loop + match` when:
    //   - You need to handle multiple variants per iteration.
    //   - You want explicit control over `break` conditions.

    // --- 6. Gotcha: while let doesn't backtrack ---
    // Once the pattern fails, the loop ends. It doesn't skip non-matching
    // elements like `filter()` would.
    let nums: Vec<Option<i32>> = vec![Some(1), Some(2), None, Some(4)];
    let mut iter = nums.into_iter();
    let mut collected = Vec::new();
    while let Some(Some(n)) = iter.next() {
        collected.push(n);
    }
    // `collected` is [1, 2] — 4 is never reached because None stopped the loop.
    println!("collected = {collected:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_while_let_pop() {
        let mut stack = vec![1, 2, 3];
        let mut result = Vec::new();
        while let Some(val) = stack.pop() {
            result.push(val);
        }
        assert_eq!(result, vec![3, 2, 1]); // LIFO
    }

    #[test]
    fn test_while_let_stops_at_none() {
        let data: Vec<Option<i32>> = vec![Some(10), Some(20), None, Some(40)];
        let mut iter = data.into_iter();
        let mut collected = Vec::new();
        while let Some(Some(val)) = iter.next() {
            collected.push(val);
        }
        // Stops at None, doesn't reach Some(40).
        assert_eq!(collected, vec![10, 20]);
    }

    #[test]
    fn test_while_let_empty() {
        let mut stack: Vec<i32> = Vec::new();
        let mut count = 0;
        while let Some(_) = stack.pop() {
            count += 1;
        }
        assert_eq!(count, 0);
    }

    #[test]
    fn test_while_let_iterator() {
        let v = vec![1, 2, 3];
        let mut iter = v.iter();
        let mut sum = 0;
        while let Some(&val) = iter.next() {
            sum += val;
        }
        assert_eq!(sum, 6);
    }
}
