// Demonstrates the Drop trait.
//
// `Drop` is called automatically when a value goes out of scope.
// It's Rust's equivalent of a destructor / finalizer.
//
// **Gotcha:** You cannot call `.drop()` directly (use `std::mem::drop()`
// if you need to drop early). Rust calls Drop automatically.
//
// **Python analogy:** Similar to `__del__`, but Drop is deterministic —
// it runs exactly when the value goes out of scope, not when the GC
// gets around to it.
//
// Run: cargo run --example s26_drop_trait

/// A resource that prints when created and dropped.
#[derive(Debug)]
struct Resource {
    name: String,
}

impl Resource {
    /// Creates a new resource.
    fn new(name: &str) -> Self {
        println!("  [created] {name}");
        Resource {
            name: name.to_string(),
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("  [dropped] {}", self.name);
    }
}

fn main() {
    println!("=== The Drop Trait ===");

    // --- 1. Automatic drop at scope end ---
    println!("--- scope start ---");
    {
        let _r1 = Resource::new("r1");
        let _r2 = Resource::new("r2");
        println!("  inside scope");
        // r2 dropped first (reverse order), then r1
    }
    println!("--- scope end ---");

    // --- 2. Early drop with std::mem::drop ---
    println!("--- early drop ---");
    let r3 = Resource::new("r3");
    println!("  before drop");
    drop(r3); // explicitly drop early
    println!("  after drop (r3 is gone)");

    // --- 3. Drop order in structs ---
    println!("--- struct drop ---");
    struct Pair {
        _first: Resource,
        _second: Resource,
    }
    let _pair = Pair {
        _first: Resource::new("first"),
        _second: Resource::new("second"),
    };
    println!("  pair created");
    // Fields are dropped in declaration order: first, then second

    // --- 4. Drop in vectors ---
    println!("--- vector drop ---");
    let _v = vec![
        Resource::new("v0"),
        Resource::new("v1"),
        Resource::new("v2"),
    ];
    println!("  vector created");
    // Elements dropped in order when vec is dropped
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    struct Counter<'a> {
        count: &'a Cell<u32>,
    }

    impl<'a> Drop for Counter<'a> {
        fn drop(&mut self) {
            self.count.set(self.count.get() + 1);
        }
    }

    #[test]
    fn test_drop_called() {
        let count = Cell::new(0);
        {
            let _c = Counter { count: &count };
        }
        assert_eq!(count.get(), 1);
    }

    #[test]
    fn test_early_drop() {
        let count = Cell::new(0);
        let c = Counter { count: &count };
        assert_eq!(count.get(), 0);
        drop(c);
        assert_eq!(count.get(), 1);
    }
}
