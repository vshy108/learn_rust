// Demonstrates the RefCell type and interior mutability with borrowing.
//
// `RefCell<T>` provides interior mutability for non-Copy types.
// It enforces borrow rules at RUNTIME instead of compile time:
// - Multiple shared borrows (&T) — OK
// - One mutable borrow (&mut T) — OK
// - Shared + mutable at same time — PANICS at runtime
//
// **Gotcha:** RefCell is NOT thread-safe (`!Sync`). Use `Mutex<T>` or
// `RwLock<T>` for multi-threaded interior mutability.
//
// Run: cargo run --example s27_refcell

use std::cell::RefCell;

fn main() {
    println!("=== The RefCell Type ===");

    // --- 1. Basic usage ---
    let data = RefCell::new(vec![1, 2, 3]);
    // borrow() returns Ref<T> (like &T)
    println!("data: {:?}", data.borrow());
    // borrow_mut() returns RefMut<T> (like &mut T)
    data.borrow_mut().push(4);
    println!("after push: {:?}", data.borrow());

    // --- 2. Interior mutability in immutable struct ---
    struct Logger {
        messages: RefCell<Vec<String>>,
    }

    impl Logger {
        fn new() -> Self {
            Logger {
                messages: RefCell::new(Vec::new()),
            }
        }

        /// Logs a message. Note: &self, not &mut self!
        fn log(&self, msg: &str) {
            self.messages.borrow_mut().push(msg.to_string());
        }

        /// Returns all logged messages.
        fn get_messages(&self) -> Vec<String> {
            self.messages.borrow().clone()
        }
    }

    let logger = Logger::new(); // not mut!
    logger.log("started");
    logger.log("processing");
    logger.log("done");
    println!("logs: {:?}", logger.get_messages());

    // --- 3. Multiple shared borrows are fine ---
    let cell = RefCell::new(42);
    let r1 = cell.borrow();
    let r2 = cell.borrow();
    println!("shared borrows: {}, {}", *r1, *r2);
    drop(r1);
    drop(r2);

    // --- 4. try_borrow to avoid panics ---
    let cell = RefCell::new(10);
    let _mut_ref = cell.borrow_mut();
    match cell.try_borrow() {
        Ok(val) => println!("got: {val}"),
        Err(e) => println!("borrow failed (expected): {e}"),
    }
    drop(_mut_ref);

    // --- 5. Combining Rc + RefCell for shared mutable data ---
    use std::rc::Rc;

    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);

    clone1.borrow_mut().push(4);
    clone2.borrow_mut().push(5);
    println!("shared data: {:?}", shared.borrow());

    // --- 6. RefCell for mock objects in testing ---
    println!("(see tests for mock pattern)");
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    #[test]
    fn test_borrow_mut() {
        let cell = RefCell::new(vec![1, 2]);
        cell.borrow_mut().push(3);
        assert_eq!(*cell.borrow(), vec![1, 2, 3]);
    }

    #[test]
    fn test_multiple_shared_borrows() {
        let cell = RefCell::new(42);
        let r1 = cell.borrow();
        let r2 = cell.borrow();
        assert_eq!(*r1, *r2);
    }

    #[test]
    fn test_try_borrow_fails_when_mut_borrowed() {
        let cell = RefCell::new(0);
        let _m = cell.borrow_mut();
        assert!(cell.try_borrow().is_err());
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn test_double_mut_borrow_panics() {
        let cell = RefCell::new(0);
        let _m1 = cell.borrow_mut();
        let _m2 = cell.borrow_mut(); // panics!
    }

    /// Mock pattern using RefCell for tracking calls.
    trait Messenger {
        fn send(&self, msg: &str);
    }

    struct MockMessenger {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            MockMessenger {
                messages: RefCell::new(Vec::new()),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // &self but we need to mutate — RefCell to the rescue!
            self.messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn test_mock_messenger() {
        let mock = MockMessenger::new();
        mock.send("hello");
        mock.send("world");
        assert_eq!(mock.messages.borrow().len(), 2);
        assert_eq!(mock.messages.borrow()[0], "hello");
    }
}
