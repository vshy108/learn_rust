// Demonstrates the `drop` function and Drop trait.
//
// Rust automatically calls `Drop::drop()` when a value goes out of scope.
// You can also call `std::mem::drop(value)` to drop a value early.
//
// **Key rule**: You CANNOT call `.drop()` directly on a value — Rust forbids
// this to prevent double-free bugs. Use `std::mem::drop()` instead, which
// takes ownership and lets the normal Drop mechanism handle cleanup.
//
// Run: cargo run --example s06_drop_function

fn main() {
    // --- 1. Automatic drop at end of scope ---
    {
        let s = String::from("auto-dropped");
        println!("Inside scope: {s}");
    } // `s` is dropped here — String's Drop impl frees the heap buffer

    // --- 2. Custom Drop implementation ---
    struct Resource {
        name: String,
    }

    impl Drop for Resource {
        fn drop(&mut self) {
            println!("  Dropping Resource: {}", self.name);
        }
    }

    println!("Creating resources...");
    let _r1 = Resource {
        name: "first".into(),
    };
    let _r2 = Resource {
        name: "second".into(),
    };
    println!("Resources created");
    // Drop order is REVERSE of creation order (like a stack LIFO):
    // r2 dropped first, then r1.

    // --- 3. Early drop with std::mem::drop ---
    let r3 = Resource {
        name: "early".into(),
    };
    println!("Before early drop");
    drop(r3); // `r3` is moved into `drop()`, which drops it immediately
    println!("After early drop");
    // r3 is no longer valid here.

    // --- 4. Why you can't call .drop() directly ---
    // let r4 = Resource { name: "test".into() };
    // r4.drop(); // error[E0040]: explicit use of destructor method
    // Reason: Rust would still call drop again at scope end → double free.
    // `std::mem::drop()` takes ownership, preventing the double call.

    // --- 5. drop() is just an empty function ---
    // Its signature is: `pub fn drop<T>(_x: T) {}`
    // It takes ownership of `_x`, then does nothing. The Drop trait runs
    // when `_x` goes out of scope (i.e., at the closing brace of `drop`).
    // Beautifully simple.

    println!("End of main — _r1 and _r2 drop now:");
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    thread_local! {
        static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    }

    struct Tracker {
        id: &'static str,
    }

    impl Drop for Tracker {
        fn drop(&mut self) {
            LOG.with(|log| log.borrow_mut().push(self.id.to_string()));
        }
    }

    fn clear_log() {
        LOG.with(|log| log.borrow_mut().clear());
    }

    fn get_log() -> Vec<String> {
        LOG.with(|log| log.borrow().clone())
    }

    #[test]
    fn test_drop_order_is_reverse() {
        clear_log();
        {
            let _a = Tracker { id: "a" };
            let _b = Tracker { id: "b" };
            let _c = Tracker { id: "c" };
        }
        assert_eq!(get_log(), vec!["c", "b", "a"]);
    }

    #[test]
    fn test_early_drop() {
        clear_log();
        {
            let t = Tracker { id: "early" };
            drop(t);
            // At this point, "early" should already be in the log.
            assert_eq!(get_log(), vec!["early"]);
        }
    }
}
