#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates why Copy and Drop cannot coexist on the same type.
//
// `Copy`: implicit bitwise copy on assignment — no custom logic.
// `Drop`: custom cleanup when a value goes out of scope.
//
// If a type implements Drop, it means "something special happens on destroy"
// (e.g., freeing heap memory, closing file handles). Bitwise copies would
// create two owners, both running Drop — causing double-free or similar bugs.
//
// Rust prevents this at compile time: `Copy` and `Drop` are mutually exclusive.
//
// Run: cargo run --example s06_copy_drop_conflict

fn main() {
    // --- 1. Copy works without Drop ---
    #[derive(Debug, Copy, Clone)]
    struct Simple {
        x: i32,
        y: i32,
    }

    let a = Simple { x: 1, y: 2 };
    let b = a; // Copy: bitwise
    println!("a = {a:?}, b = {b:?}"); // both valid

    // --- 2. Drop implies no Copy ---
    // If we implement Drop, the type can't be Copy.
    #[derive(Debug)]
    struct Resource {
        name: String,
    }

    impl Drop for Resource {
        fn drop(&mut self) {
            println!("Dropping Resource: {}", self.name);
        }
    }

    let r = Resource {
        name: "DB handle".to_string(),
    };
    let _r2 = r; // MOVE, not copy — because Resource has Drop
    // println!("{r:?}"); // error: value used after move
    println!("Resource moved to _r2");

    // --- 3. The compiler error if you try both ---
    // Uncommenting below would cause:
    //   error[E0184]: the trait `Copy` cannot be implemented for this type;
    //   the type has a destructor
    //
    // #[derive(Copy, Clone)]
    // struct Bad { x: i32 }
    // impl Drop for Bad { fn drop(&mut self) {} }

    // --- 4. Why this rule exists ---
    // Imagine if Resource were Copy:
    //   let r1 = Resource { name: "file.txt" };
    //   let r2 = r1; // bitwise copy — now r1 AND r2 have the same data
    //   // When r2 goes out of scope: Drop runs, frees the handle
    //   // When r1 goes out of scope: Drop runs AGAIN — double free!
    // Rust's rule prevents this entire class of bugs.

    // --- 5. Alternatives ---
    // If you need to duplicate a type with Drop, use `.clone()`.
    // Clone is explicit and can perform a deep copy.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_is_copy() {
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Simple {
            x: i32,
        }
        let a = Simple { x: 42 };
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_drop_runs_on_scope_exit() {
        use std::cell::RefCell;
        // Track whether drop was called using a shared counter.
        thread_local! {
            static DROPPED: RefCell<bool> = const { RefCell::new(false) };
        }

        struct Tracker;
        impl Drop for Tracker {
            fn drop(&mut self) {
                DROPPED.with(|d| *d.borrow_mut() = true);
            }
        }

        DROPPED.with(|d| *d.borrow_mut() = false);
        {
            let _t = Tracker;
        } // _t dropped here
        DROPPED.with(|d| assert!(*d.borrow()));
    }
}
