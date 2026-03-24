// Demonstrates the Cell type and interior mutability.
//
// `Cell<T>` allows mutation through a shared reference (&T).
// This is called "interior mutability" — the outer container is
// immutable, but the inner value can change.
//
// **Key constraint:** Cell works only with `Copy` types because
// it gives/takes values by copy (`.get()` / `.set()`).
//
// **Python analogy:** In Python, everything is mutable. In Rust,
// shared references (&T) are normally immutable. Cell lets you
// opt out of that restriction for simple Copy types.
//
// **Gotcha:** Cell is NOT thread-safe (`!Sync`). Use `AtomicXxx` or
// `Mutex` for shared mutation across threads.
//
// Run: cargo run --example s27_cell

use std::cell::Cell;

fn main() {
    println!("=== The Cell Type ===");

    // --- 1. Basic Cell usage ---
    let c = Cell::new(42);
    println!("initial: {}", c.get());
    c.set(100); // mutate through shared reference!
    println!("after set: {}", c.get());

    // --- 2. Cell in an immutable struct ---
    // Normally you can't mutate fields of an immutable struct.
    // Cell lets you opt specific fields into mutability.
    struct Counter {
        value: Cell<u32>,
        name: String,
    }

    let counter = Counter {
        value: Cell::new(0),
        name: "hits".to_string(),
    };
    // counter is not mut, but we can still update value!
    counter.value.set(counter.value.get() + 1);
    counter.value.set(counter.value.get() + 1);
    println!("{}: {}", counter.name, counter.value.get());

    // --- 3. Cell with shared references ---
    let cell = Cell::new(10);
    let r1 = &cell;
    let r2 = &cell;
    // Both shared refs can mutate!
    r1.set(20);
    r2.set(30);
    println!("after shared mutation: {}", cell.get());

    // --- 4. Practical use: caching in immutable API ---
    struct Fibonacci {
        cache: Cell<Option<Vec<u64>>>,
    }

    impl Fibonacci {
        fn new() -> Self {
            Fibonacci {
                cache: Cell::new(None),
            }
        }

        /// Computes the first n Fibonacci numbers (caches result).
        fn compute(&self, n: usize) -> Vec<u64> {
            // Take the cached value out
            let cached = self.cache.take();
            if let Some(v) = cached {
                if v.len() >= n {
                    let result = v[..n].to_vec();
                    self.cache.set(Some(v));
                    return result;
                }
            }
            // Compute fresh
            let mut fibs = Vec::with_capacity(n);
            for i in 0..n {
                let val = if i < 2 {
                    i as u64
                } else {
                    fibs[i - 1] + fibs[i - 2]
                };
                fibs.push(val);
            }
            let result = fibs.clone();
            self.cache.set(Some(fibs));
            result
        }
    }

    let fib = Fibonacci::new();
    println!("fib(10): {:?}", fib.compute(10));
    println!("fib(5): {:?}", fib.compute(5)); // uses cache

    // --- 5. Cell::replace and Cell::take ---
    let c = Cell::new(String::from("hello"));
    let old = c.replace(String::from("world")); // works for non-Copy too!
    println!("old: {old}, new: {:?}", c.into_inner());
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    #[test]
    fn test_basic_cell() {
        let c = Cell::new(0);
        c.set(42);
        assert_eq!(c.get(), 42);
    }

    #[test]
    fn test_shared_mutation() {
        let c = Cell::new(0);
        let r1 = &c;
        let r2 = &c;
        r1.set(10);
        r2.set(20);
        assert_eq!(c.get(), 20);
    }

    #[test]
    fn test_replace() {
        let c = Cell::new(1);
        let old = c.replace(2);
        assert_eq!(old, 1);
        assert_eq!(c.get(), 2);
    }
}
