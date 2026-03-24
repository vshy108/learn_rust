// Demonstrates `while let` — looping with pattern matching.
//
// `while let` continues looping as long as a pattern matches:
//
//   while let Some(val) = iterator.next() { ... }
//
// It's syntactic sugar for:
//   loop {
//       match expr {
//           pattern => { ... },
//           _ => break,
//       }
//   }
//
// Run: cargo run --example s12_while_let

fn main() {
    // --- 1. while let with Vec::pop ---
    let mut stack = vec![1, 2, 3, 4, 5];
    println!("Popping from stack:");
    while let Some(top) = stack.pop() {
        println!("  {top}");
    }
    println!("Stack is now empty: {stack:?}");

    // --- 2. while let with an iterator ---
    // Clippy suggests `for word in iter` — which IS the idiomatic way.
    // We use while let here intentionally to show the pattern.

    let data = vec!["hello", "world", "rust"];
    let mut iter = data.iter();
    while let Some(word) = iter.next() {
        println!("  word: {word}");
    }
    // NOTE: `for` loops are usually preferred for iterators.
    // `while let` is more useful for non-iterator patterns.

    // --- 3. while let with Option mutations ---
    let mut count = Some(3_i32);
    while let Some(n) = count {
        if n > 0 {
            println!("  countdown: {n}");
            count = Some(n - 1);
        } else {
            println!("  done!");
            count = None;
        }
    }

    // --- 4. while let with nested pattern ---
    let pairs = vec![Some((1, "one")), Some((2, "two")), None, Some((3, "three"))];
    let mut iter = pairs.into_iter();
    while let Some(Some((n, name))) = iter.next() {
        println!("  {n}: {name}");
    }
    // Loop stops at the first None (the pattern doesn't match).

    // --- 5. Practical: reading from a channel-like structure ---
    struct Queue {
        items: Vec<String>,
    }
    impl Queue {
        fn dequeue(&mut self) -> Option<String> {
            if self.items.is_empty() {
                None
            } else {
                Some(self.items.remove(0))
            }
        }
    }

    let mut q = Queue {
        items: vec!["task1".into(), "task2".into(), "task3".into()],
    };
    println!("Processing queue:");
    while let Some(task) = q.dequeue() {
        println!("  Processing: {task}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_while_let_pop() {
        let mut v = vec![1, 2, 3];
        let mut result = Vec::new();
        while let Some(val) = v.pop() {
            result.push(val);
        }
        assert_eq!(result, vec![3, 2, 1]); // LIFO order
    }

    #[test]
    fn test_while_let_stops_at_none() {
        let data = vec![Some(1), Some(2), None, Some(3)];
        let mut iter = data.into_iter();
        let mut collected = Vec::new();
        while let Some(Some(n)) = iter.next() {
            collected.push(n);
        }
        // Stops at None — doesn't reach Some(3)
        assert_eq!(collected, vec![1, 2]);
    }
}
