//! # Rustfinity: Arc Thread Safety
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/arc-thread-safety
//!
//! # Arc and Thread Safety
//!
//! While `Rc<T>` provides shared ownership for single-threaded code, it cannot be safely shared across threads because its reference count operations are not atomic. For multi-threaded scenarios, Rust provides `Arc<T>` (Atomically Reference Counted), which uses atomic operations to safely manage reference counts across thread boundaries.
//!
//! `Arc<T>` works similarly to `Rc<T>` but is thread-safe. The "atomic" part means that incrementing and decrementing the reference count is done using atomic CPU operations, which are guaranteed to complete without interference from other threads. This makes `Arc<T>` slightly slower than `Rc<T>` due to the synchronization overhead, so you should only use `Arc<T>` when you actually need to share data across threads.
//!
//! Common use cases for `Arc<T>` include:
//!
//! - Sharing read-only configuration across worker threads
//! - Sharing immutable data structures in parallel computations
//! - Combined with `Mutex<T>` or `RwLock<T>` for shared mutable state
//! - Thread pools that share access to resources
//!
//! ## Key Differences from Rc
//!
//! | Feature       | `Rc<T>`          | `Arc<T>`                |
//! | ------------- | ---------------- | ----------------------- |
//! | Thread-safe   | No               | Yes                     |
//! | Performance   | Faster           | Slightly slower         |
//! | Marker traits | `!Send`, `!Sync` | `Send + Sync` (if T is) |
//! | Use case      | Single-threaded  | Multi-threaded          |
//!
//! ## Key Operations
//!
//! - `Arc::new(value)` - Create a new atomic reference-counted pointer
//! - `Arc::clone(&arc)` - Create a new reference (increments count atomically)
//! - `Arc::strong_count(&arc)` - Get the current number of strong references
//! - `Arc::weak_count(&arc)` - Get the current number of weak references
//! - `Arc::downgrade(&arc)` - Create a `Weak<T>` reference
//! - `weak.upgrade()` - Try to convert a `Weak<T>` back to `Arc<T>`
//!
//! ## Your Task
//!
//! Implement the following types and functions to demonstrate `Arc<T>` patterns:
//!
//! ### Part 1: Basic Arc Operations
//!
//! 1. **`create_arc<T>(value: T) -> Arc<T>`** - Create a new atomic reference-counted value
//! 2. **`clone_arc<T>(arc: &Arc<T>) -> Arc<T>`** - Clone a reference (increment count)
//! 3. **`get_strong_count<T>(arc: &Arc<T>) -> usize`** - Get the strong reference count
//! 4. **`get_value<T: Clone>(arc: &Arc<T>) -> T`** - Get a clone of the inner value
//!
//! ### Part 2: Thread-Safe Shared Data
//!
//! 5. **`SharedConfig`** - A thread-safe configuration that can be shared across threads:
//!    - `new(
//!  app_name: String,
//!  max_connections: usize,
//!  debug_mode: bool
//! ) -> Arc<Self>`
//!    - `app_name(&self) -> &str`
//!    - `max_connections(&self) -> usize`
//!    - `debug_mode(&self) -> bool`
//!
//! ### Part 3: Weak References
//!
//! 6. **`create_weak<T>(arc: &Arc<T>) -> Weak<T>`** - Create a weak reference
//! 7. **`upgrade_weak<T>(weak: &Weak<T>) -> Option<Arc<T>>`** - Try to upgrade a weak reference
//! 8. **`get_weak_count<T>(arc: &Arc<T>) -> usize`** - Get the weak reference count
//!
//! ### Part 4: Thread-Safe Counter
//!
//! 9. **`AtomicCounter`** - A thread-safe counter using `Arc<AtomicUsize>`:
//!    - `new() -> Self` - Create with initial value 0
//!    - `new_with_value(value: usize) -> Self` - Create with specific value
//!    - `get(&self) -> usize` - Get current value
//!    - `increment(&self) -> usize` - Increment and return previous value
//!    - `decrement(&self) -> usize` - Decrement and return previous value
//!    - `add(&self, val: usize) -> usize` - Add and return previous value
//!    - `clone_counter(&self) -> Self` - Create another handle to the same counter
//!
//! ### Part 5: Shared Data with Mutex
//!
//! 10. **`SharedVec<T>`** - A thread-safe vector using `Arc<Mutex<Vec<T>>>`:
//!     - `new() -> Self` - Create empty shared vec
//!     - `push(&self, value: T)` - Add an element
//!     - `pop(&self) -> Option<T>` - Remove and return last element
//!     - `len(&self) -> usize` - Get current length
//!     - `is_empty(&self) -> bool` - Check if empty
//!     - `get(&self, index: usize) -> Option<T>` where T: Clone - Get element at index
//!     - `clone_vec(&self) -> Self` - Create another handle to the same vec
//!
//! ## Examples
//!
//! ```rust
//! use arc_thread_safety::*;
//! use std::sync::Arc;
//! use std::thread;
//!
//! // Basic Arc operations
//! let shared = create_arc(42);
//! assert_eq!(get_strong_count(&shared), 1);
//!
//! let cloned = clone_arc(&shared);
//! assert_eq!(get_strong_count(&shared), 2);
//!
//! // Sharing across threads
//! let data = create_arc(vec![1, 2, 3]);
//! let data_clone = clone_arc(&data);
//!
//! let handle = thread::spawn(move || {
//!     // data_clone is accessible in this thread
//!     assert_eq!(data_clone.len(), 3);
//! });
//! handle.join().unwrap();
//!
//! // Shared configuration
//! let config = SharedConfig::new(
//!     "MyApp".to_string(),
//!     100,
//!     true
//! );
//! let config_clone = Arc::clone(&config);
//!
//! thread::spawn(move || {
//!     assert_eq!(config_clone.app_name(), "MyApp");
//! }).join().unwrap();
//!
//! // Atomic counter shared across threads
//! let counter = AtomicCounter::new();
//! let counter_clone = counter.clone_counter();
//!
//! let handle = thread::spawn(move || {
//!     for _ in 0..1000 {
//!         counter_clone.increment();
//!     }
//! });
//! for _ in 0..1000 {
//!     counter.increment();
//! }
//! handle.join().unwrap();
//! assert_eq!(counter.get(), 2000);
//!
//! // Thread-safe vector
//! let vec = SharedVec::new();
//! vec.push(1);
//! vec.push(2);
//! assert_eq!(vec.len(), 2);
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `Arc::new()` to wrap values for thread-safe shared ownership
//! - `Arc::clone(&arc)` is cheap - it only atomically increments a counter
//! - For mutable shared state, combine `Arc` with `Mutex`: `Arc<Mutex<T>>`
//! - `AtomicUsize` provides lock-free atomic operations like `fetch_add` and `fetch_sub`
//! - Use `Ordering::SeqCst` for atomic operations if unsure about memory ordering
//! - When working with `Mutex`, use `.lock().unwrap()` to acquire the lock (panics if poisoned)
//! - `Weak<T>` works the same way with `Arc` as it does with `Rc`
//! - Remember: `Arc<T>` is `Send + Sync` only if `T` is `Send + Sync`
//!
//! </details>
//!
//! Run: `cargo run --example rf_arc_thread_safety`
//! Test: `cargo test --example rf_arc_thread_safety`
//! Filter: `cargo test --example rf_arc_thread_safety <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

/// Creates a new atomic reference-counted value.
///
/// Wraps the given value in an `Arc<T>` for thread-safe shared ownership.
pub fn create_arc<T>(value: T) -> Arc<T> {
    // TODO: Wrap the value in an Arc
    unimplemented!()
}

/// Clones an atomic reference-counted pointer.
///
/// This atomically increments the reference count without cloning the underlying data.
pub fn clone_arc<T>(arc: &Arc<T>) -> Arc<T> {
    // TODO: Clone the Arc (hint: use Arc::clone)
    unimplemented!()
}

/// Returns the strong reference count of an Arc.
pub fn get_strong_count<T>(arc: &Arc<T>) -> usize {
    // TODO: Return the strong reference count
    unimplemented!()
}

/// Gets a clone of the value inside an Arc.
pub fn get_value<T: Clone>(arc: &Arc<T>) -> T {
    // TODO: Clone and return the inner value
    unimplemented!()
}

/// A thread-safe configuration that can be shared across threads.
#[derive(Debug, Clone)]
pub struct SharedConfig {
    app_name: String,
    max_connections: usize,
    debug_mode: bool,
}

impl SharedConfig {
    /// Creates a new shared configuration wrapped in Arc.
    pub fn new(app_name: String, max_connections: usize, debug_mode: bool) -> Arc<Self> {
        // TODO: Create and wrap SharedConfig in an Arc
        unimplemented!()
    }

    /// Returns the application name.
    pub fn app_name(&self) -> &str {
        // TODO: Return the app name
        unimplemented!()
    }

    /// Returns the maximum number of connections.
    pub fn max_connections(&self) -> usize {
        // TODO: Return max connections
        unimplemented!()
    }

    /// Returns whether debug mode is enabled.
    pub fn debug_mode(&self) -> bool {
        // TODO: Return debug mode
        unimplemented!()
    }
}

/// Creates a weak reference from an Arc.
pub fn create_weak<T>(arc: &Arc<T>) -> Weak<T> {
    // TODO: Create a weak reference using Arc::downgrade
    unimplemented!()
}

/// Attempts to upgrade a weak reference to a strong reference.
pub fn upgrade_weak<T>(weak: &Weak<T>) -> Option<Arc<T>> {
    // TODO: Try to upgrade the weak reference
    unimplemented!()
}

/// Returns the weak reference count of an Arc.
pub fn get_weak_count<T>(arc: &Arc<T>) -> usize {
    // TODO: Return the weak reference count
    unimplemented!()
}

/// A thread-safe counter using atomic operations.
#[derive(Debug)]
pub struct AtomicCounter {
    value: Arc<AtomicUsize>,
}

impl AtomicCounter {
    /// Creates a new counter with initial value 0.
    pub fn new() -> Self {
        // TODO: Create a new AtomicCounter with value 0
        unimplemented!()
    }

    /// Creates a new counter with a specific initial value.
    pub fn new_with_value(value: usize) -> Self {
        // TODO: Create a new AtomicCounter with the given value
        unimplemented!()
    }

    /// Gets the current value of the counter.
    pub fn get(&self) -> usize {
        // TODO: Load and return the current value
        unimplemented!()
    }

    /// Increments the counter by 1 and returns the previous value.
    pub fn increment(&self) -> usize {
        // TODO: Atomically increment and return previous value
        unimplemented!()
    }

    /// Decrements the counter by 1 and returns the previous value.
    pub fn decrement(&self) -> usize {
        // TODO: Atomically decrement and return previous value
        unimplemented!()
    }

    /// Adds a value to the counter and returns the previous value.
    pub fn add(&self, val: usize) -> usize {
        // TODO: Atomically add and return previous value
        unimplemented!()
    }

    /// Creates another handle to the same counter.
    pub fn clone_counter(&self) -> Self {
        // TODO: Clone the Arc to share the same AtomicUsize
        unimplemented!()
    }
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// A thread-safe vector using Arc and Mutex.
#[derive(Debug)]
pub struct SharedVec<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T> SharedVec<T> {
    /// Creates a new empty shared vector.
    pub fn new() -> Self {
        // TODO: Create a new SharedVec with an empty Vec inside Arc<Mutex<>>
        unimplemented!()
    }

    /// Adds an element to the end of the vector.
    pub fn push(&self, value: T) {
        // TODO: Lock the mutex and push the value
        unimplemented!()
    }

    /// Removes and returns the last element, or None if empty.
    pub fn pop(&self) -> Option<T> {
        // TODO: Lock the mutex and pop from the vector
        unimplemented!()
    }

    /// Returns the current number of elements.
    pub fn len(&self) -> usize {
        // TODO: Lock the mutex and return the length
        unimplemented!()
    }

    /// Returns true if the vector is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Lock the mutex and check if empty
        unimplemented!()
    }

    /// Creates another handle to the same vector.
    pub fn clone_vec(&self) -> Self {
        // TODO: Clone the Arc to share the same Mutex<Vec<T>>
        unimplemented!()
    }
}

impl<T: Clone> SharedVec<T> {
    /// Gets a clone of the element at the specified index.
    pub fn get(&self, index: usize) -> Option<T> {
        // TODO: Lock the mutex, get the element, and clone it
        unimplemented!()
    }
}

impl<T> Default for SharedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Example usage
pub fn main() {
    // Basic Arc operations
    let shared = create_arc(42);
    println!("Created arc with value: {}", *shared);
    println!("Strong count: {}", get_strong_count(&shared));

    let cloned = clone_arc(&shared);
    println!("After clone, strong count: {}", get_strong_count(&shared));

    // Shared configuration
    let config = SharedConfig::new("MyApp".to_string(), 100, true);
    println!(
        "App: {}, Max connections: {}",
        config.app_name(),
        config.max_connections()
    );

    // Atomic counter
    let counter = AtomicCounter::new();
    counter.increment();
    counter.increment();
    println!("Counter value: {}", counter.get());

    // Shared vector
    let vec: SharedVec<i32> = SharedVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vec length: {}", vec.len());
    println!("Element at index 1: {:?}", vec.get(1));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::sync::Arc;
    use std::thread;

    // =============================================================================
    // Basic Arc Operations Tests
    // =============================================================================

    mod create_arc_tests {
        use super::*;

        #[test]
        fn creates_arc_integer() {
            let shared = create_arc(42);
            assert_eq!(*shared, 42);
        }

        #[test]
        fn creates_arc_string() {
            let shared = create_arc(String::from("hello"));
            assert_eq!(*shared, "hello");
        }

        #[test]
        fn creates_arc_vec() {
            let shared = create_arc(vec![1, 2, 3]);
            assert_eq!(*shared, vec![1, 2, 3]);
        }

        #[test]
        fn creates_arc_zero() {
            let shared = create_arc(0);
            assert_eq!(*shared, 0);
        }

        #[test]
        fn creates_arc_negative() {
            let shared = create_arc(-100);
            assert_eq!(*shared, -100);
        }

        #[test]
        fn creates_arc_float() {
            let shared = create_arc(3.14f64);
            assert!((*shared - 3.14).abs() < f64::EPSILON);
        }

        #[test]
        fn creates_arc_tuple() {
            let shared = create_arc((1, "two", 3.0));
            assert_eq!(*shared, (1, "two", 3.0));
        }

        #[test]
        fn initial_count_is_one() {
            let shared = create_arc(42);
            assert_eq!(get_strong_count(&shared), 1);
        }
    }

    mod clone_arc_tests {
        use super::*;

        #[test]
        fn clone_increments_count() {
            let original = create_arc(42);
            let _cloned = clone_arc(&original);
            assert_eq!(get_strong_count(&original), 2);
        }

        #[test]
        fn multiple_clones() {
            let original = create_arc(42);
            let _c1 = clone_arc(&original);
            let _c2 = clone_arc(&original);
            let _c3 = clone_arc(&original);
            assert_eq!(get_strong_count(&original), 4);
        }

        #[test]
        fn clones_share_same_value() {
            let original = create_arc(42);
            let cloned = clone_arc(&original);
            assert_eq!(*original, *cloned);
        }

        #[test]
        fn clone_of_clone() {
            let original = create_arc(42);
            let c1 = clone_arc(&original);
            let _c2 = clone_arc(&c1);
            assert_eq!(get_strong_count(&original), 3);
        }

        #[test]
        fn drop_decrements_count() {
            let original = create_arc(42);
            {
                let _cloned = clone_arc(&original);
                assert_eq!(get_strong_count(&original), 2);
            }
            assert_eq!(get_strong_count(&original), 1);
        }

        #[test]
        fn clone_string() {
            let original = create_arc(String::from("hello"));
            let cloned = clone_arc(&original);
            assert_eq!(*cloned, "hello");
            assert_eq!(get_strong_count(&original), 2);
        }
    }

    mod get_strong_count_tests {
        use super::*;

        #[test]
        fn count_one() {
            let shared = create_arc(42);
            assert_eq!(get_strong_count(&shared), 1);
        }

        #[test]
        fn count_increases_with_clones() {
            let shared = create_arc(42);
            let clones: Vec<_> = (0..5).map(|_| clone_arc(&shared)).collect();
            assert_eq!(get_strong_count(&shared), 6);
            drop(clones);
            assert_eq!(get_strong_count(&shared), 1);
        }

        #[test]
        fn count_through_clone() {
            let original = create_arc(42);
            let cloned = clone_arc(&original);
            assert_eq!(get_strong_count(&cloned), 2);
        }
    }

    mod get_value_tests {
        use super::*;

        #[test]
        fn gets_integer_value() {
            let shared = create_arc(42);
            assert_eq!(get_value(&shared), 42);
        }

        #[test]
        fn gets_string_value() {
            let shared = create_arc(String::from("hello"));
            let value = get_value(&shared);
            assert_eq!(value, "hello");
        }

        #[test]
        fn gets_vec_value() {
            let shared = create_arc(vec![1, 2, 3]);
            let value = get_value(&shared);
            assert_eq!(value, vec![1, 2, 3]);
        }

        #[test]
        fn value_is_independent_clone() {
            let shared = create_arc(String::from("hello"));
            let mut value = get_value(&shared);
            value.push_str(" world");
            assert_eq!(*shared, "hello"); // Original unchanged
            assert_eq!(value, "hello world");
        }

        #[test]
        fn gets_empty_string() {
            let shared = create_arc(String::new());
            assert_eq!(get_value(&shared), "");
        }

        #[test]
        fn gets_negative() {
            let shared = create_arc(-999);
            assert_eq!(get_value(&shared), -999);
        }
    }

    // =============================================================================
    // SharedConfig Tests
    // =============================================================================

    mod shared_config_tests {
        use super::*;

        #[test]
        fn new_creates_config() {
            let config = SharedConfig::new("TestApp".to_string(), 100, true);
            assert_eq!(config.app_name(), "TestApp");
        }

        #[test]
        fn app_name_returns_correct_value() {
            let config = SharedConfig::new("MyApp".to_string(), 50, false);
            assert_eq!(config.app_name(), "MyApp");
        }

        #[test]
        fn max_connections_returns_correct_value() {
            let config = SharedConfig::new("App".to_string(), 200, false);
            assert_eq!(config.max_connections(), 200);
        }

        #[test]
        fn debug_mode_true() {
            let config = SharedConfig::new("App".to_string(), 100, true);
            assert!(config.debug_mode());
        }

        #[test]
        fn debug_mode_false() {
            let config = SharedConfig::new("App".to_string(), 100, false);
            assert!(!config.debug_mode());
        }

        #[test]
        fn empty_app_name() {
            let config = SharedConfig::new(String::new(), 10, false);
            assert_eq!(config.app_name(), "");
        }

        #[test]
        fn zero_connections() {
            let config = SharedConfig::new("App".to_string(), 0, false);
            assert_eq!(config.max_connections(), 0);
        }

        #[test]
        fn large_connections() {
            let config = SharedConfig::new("App".to_string(), 1_000_000, false);
            assert_eq!(config.max_connections(), 1_000_000);
        }

        #[test]
        fn shared_between_clones() {
            let config = SharedConfig::new("Shared".to_string(), 42, true);
            let config2 = Arc::clone(&config);

            assert_eq!(config.app_name(), config2.app_name());
            assert_eq!(config.max_connections(), config2.max_connections());
            assert_eq!(config.debug_mode(), config2.debug_mode());
            assert_eq!(Arc::strong_count(&config), 2);
        }

        #[test]
        fn config_is_arc() {
            let config = SharedConfig::new("Test".to_string(), 10, false);
            assert_eq!(Arc::strong_count(&config), 1);
        }
    }

    // =============================================================================
    // Weak Reference Tests
    // =============================================================================

    mod weak_reference_tests {
        use super::*;

        #[test]
        fn create_weak_basic() {
            let strong = create_arc(42);
            let _weak = create_weak(&strong);
            assert_eq!(get_weak_count(&strong), 1);
        }

        #[test]
        fn weak_doesnt_affect_strong_count() {
            let strong = create_arc(42);
            let _weak = create_weak(&strong);
            assert_eq!(get_strong_count(&strong), 1);
        }

        #[test]
        fn multiple_weak_refs() {
            let strong = create_arc(42);
            let _w1 = create_weak(&strong);
            let _w2 = create_weak(&strong);
            let _w3 = create_weak(&strong);
            assert_eq!(get_weak_count(&strong), 3);
        }

        #[test]
        fn upgrade_succeeds_when_alive() {
            let strong = create_arc(42);
            let weak = create_weak(&strong);

            let upgraded = upgrade_weak(&weak);
            assert!(upgraded.is_some());
            assert_eq!(*upgraded.unwrap(), 42);
        }

        #[test]
        fn upgrade_fails_when_dropped() {
            let weak = {
                let strong = create_arc(42);
                create_weak(&strong)
            };

            assert!(upgrade_weak(&weak).is_none());
        }

        #[test]
        fn upgrade_increments_count() {
            let strong = create_arc(42);
            let weak = create_weak(&strong);

            let upgraded = upgrade_weak(&weak).unwrap();
            assert_eq!(get_strong_count(&strong), 2);
            drop(upgraded);
            assert_eq!(get_strong_count(&strong), 1);
        }

        #[test]
        fn weak_count_zero_initially() {
            let strong = create_arc(42);
            assert_eq!(get_weak_count(&strong), 0);
        }

        #[test]
        fn weak_with_string() {
            let strong = create_arc(String::from("hello"));
            let weak = create_weak(&strong);

            let upgraded = upgrade_weak(&weak).unwrap();
            assert_eq!(*upgraded, "hello");
        }

        #[test]
        fn weak_survives_strong_drop() {
            let weak = {
                let strong = create_arc(42);
                create_weak(&strong)
            };
            assert!(upgrade_weak(&weak).is_none());
        }

        #[test]
        fn multiple_upgrade_attempts() {
            let strong = create_arc(42);
            let weak = create_weak(&strong);

            assert!(upgrade_weak(&weak).is_some());
            assert!(upgrade_weak(&weak).is_some());
            assert!(upgrade_weak(&weak).is_some());
        }
    }

    // =============================================================================
    // AtomicCounter Tests
    // =============================================================================

    mod atomic_counter_tests {
        use super::*;

        #[test]
        fn new_starts_at_zero() {
            let counter = AtomicCounter::new();
            assert_eq!(counter.get(), 0);
        }

        #[test]
        fn new_with_value() {
            let counter = AtomicCounter::new_with_value(100);
            assert_eq!(counter.get(), 100);
        }

        #[test]
        fn increment_returns_previous() {
            let counter = AtomicCounter::new();
            assert_eq!(counter.increment(), 0);
            assert_eq!(counter.increment(), 1);
            assert_eq!(counter.increment(), 2);
        }

        #[test]
        fn increment_increases_value() {
            let counter = AtomicCounter::new();
            counter.increment();
            counter.increment();
            counter.increment();
            assert_eq!(counter.get(), 3);
        }

        #[test]
        fn decrement_returns_previous() {
            let counter = AtomicCounter::new_with_value(5);
            assert_eq!(counter.decrement(), 5);
            assert_eq!(counter.decrement(), 4);
            assert_eq!(counter.decrement(), 3);
        }

        #[test]
        fn decrement_decreases_value() {
            let counter = AtomicCounter::new_with_value(10);
            counter.decrement();
            counter.decrement();
            counter.decrement();
            assert_eq!(counter.get(), 7);
        }

        #[test]
        fn add_returns_previous() {
            let counter = AtomicCounter::new();
            assert_eq!(counter.add(10), 0);
            assert_eq!(counter.add(5), 10);
        }

        #[test]
        fn add_increases_value() {
            let counter = AtomicCounter::new();
            counter.add(10);
            counter.add(20);
            assert_eq!(counter.get(), 30);
        }

        #[test]
        fn add_zero() {
            let counter = AtomicCounter::new_with_value(42);
            counter.add(0);
            assert_eq!(counter.get(), 42);
        }

        #[test]
        fn clone_counter_shares_value() {
            let counter1 = AtomicCounter::new();
            let counter2 = counter1.clone_counter();

            counter1.increment();
            assert_eq!(counter2.get(), 1);

            counter2.increment();
            assert_eq!(counter1.get(), 2);
        }

        #[test]
        fn default_creates_zero() {
            let counter = AtomicCounter::default();
            assert_eq!(counter.get(), 0);
        }

        #[test]
        fn large_value() {
            let counter = AtomicCounter::new_with_value(usize::MAX - 10);
            assert_eq!(counter.get(), usize::MAX - 10);
        }
    }

    // =============================================================================
    // SharedVec Tests
    // =============================================================================

    mod shared_vec_tests {
        use super::*;

        #[test]
        fn new_is_empty() {
            let vec: SharedVec<i32> = SharedVec::new();
            assert!(vec.is_empty());
            assert_eq!(vec.len(), 0);
        }

        #[test]
        fn push_adds_elements() {
            let vec = SharedVec::new();
            vec.push(1);
            vec.push(2);
            vec.push(3);
            assert_eq!(vec.len(), 3);
        }

        #[test]
        fn pop_removes_last() {
            let vec = SharedVec::new();
            vec.push(1);
            vec.push(2);
            vec.push(3);
            assert_eq!(vec.pop(), Some(3));
            assert_eq!(vec.pop(), Some(2));
            assert_eq!(vec.pop(), Some(1));
        }

        #[test]
        fn pop_empty_returns_none() {
            let vec: SharedVec<i32> = SharedVec::new();
            assert_eq!(vec.pop(), None);
        }

        #[test]
        fn get_valid_index() {
            let vec = SharedVec::new();
            vec.push(10);
            vec.push(20);
            vec.push(30);
            assert_eq!(vec.get(0), Some(10));
            assert_eq!(vec.get(1), Some(20));
            assert_eq!(vec.get(2), Some(30));
        }

        #[test]
        fn get_invalid_index() {
            let vec = SharedVec::new();
            vec.push(1);
            assert_eq!(vec.get(1), None);
            assert_eq!(vec.get(100), None);
        }

        #[test]
        fn get_from_empty() {
            let vec: SharedVec<i32> = SharedVec::new();
            assert_eq!(vec.get(0), None);
        }

        #[test]
        fn is_empty_false_after_push() {
            let vec = SharedVec::new();
            vec.push(1);
            assert!(!vec.is_empty());
        }

        #[test]
        fn is_empty_true_after_pop_all() {
            let vec = SharedVec::new();
            vec.push(1);
            vec.pop();
            assert!(vec.is_empty());
        }

        #[test]
        fn clone_vec_shares_data() {
            let vec1 = SharedVec::new();
            let vec2 = vec1.clone_vec();

            vec1.push(42);
            assert_eq!(vec2.len(), 1);
            assert_eq!(vec2.get(0), Some(42));
        }

        #[test]
        fn clone_vec_both_can_modify() {
            let vec1 = SharedVec::new();
            let vec2 = vec1.clone_vec();

            vec1.push(1);
            vec2.push(2);
            vec1.push(3);

            assert_eq!(vec1.len(), 3);
            assert_eq!(vec2.len(), 3);
        }

        #[test]
        fn with_strings() {
            let vec = SharedVec::new();
            vec.push(String::from("hello"));
            vec.push(String::from("world"));

            assert_eq!(vec.get(0), Some(String::from("hello")));
            assert_eq!(vec.get(1), Some(String::from("world")));
        }

        #[test]
        fn default_is_empty() {
            let vec: SharedVec<i32> = SharedVec::default();
            assert!(vec.is_empty());
        }
    }

    // =============================================================================
    // Thread Safety Tests
    // =============================================================================

    mod thread_safety_tests {
        use super::*;

        #[test]
        fn arc_can_be_sent_to_thread() {
            let shared = create_arc(vec![1, 2, 3]);
            let shared_clone = clone_arc(&shared);

            let handle = thread::spawn(move || {
                assert_eq!(shared_clone.len(), 3);
                assert_eq!(shared_clone[0], 1);
            });

            handle.join().unwrap();
        }

        #[test]
        fn config_can_be_shared_across_threads() {
            let config = SharedConfig::new("ThreadTest".to_string(), 100, true);
            let config_clone = Arc::clone(&config);

            let handle = thread::spawn(move || {
                assert_eq!(config_clone.app_name(), "ThreadTest");
                assert_eq!(config_clone.max_connections(), 100);
                assert!(config_clone.debug_mode());
            });

            handle.join().unwrap();
        }

        #[test]
        fn counter_works_across_threads() {
            let counter = AtomicCounter::new();
            let counter_clone = counter.clone_counter();

            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    counter_clone.increment();
                }
            });

            for _ in 0..100 {
                counter.increment();
            }

            handle.join().unwrap();
            assert_eq!(counter.get(), 200);
        }

        #[test]
        fn counter_multiple_threads() {
            let counter = AtomicCounter::new();
            let num_threads = 4;
            let increments_per_thread = 100;

            let handles: Vec<_> = (0..num_threads)
                .map(|_| {
                    let counter_clone = counter.clone_counter();
                    thread::spawn(move || {
                        for _ in 0..increments_per_thread {
                            counter_clone.increment();
                        }
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }

            assert_eq!(counter.get(), num_threads * increments_per_thread);
        }

        #[test]
        fn shared_vec_works_across_threads() {
            let vec = SharedVec::new();
            let vec_clone = vec.clone_vec();

            let handle = thread::spawn(move || {
                for i in 0..50 {
                    vec_clone.push(i);
                }
            });

            for i in 50..100 {
                vec.push(i);
            }

            handle.join().unwrap();
            assert_eq!(vec.len(), 100);
        }

        #[test]
        fn multiple_readers() {
            let shared = create_arc(String::from("shared data"));
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let shared_clone = clone_arc(&shared);
                    thread::spawn(move || {
                        assert_eq!(*shared_clone, "shared data");
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        }

        #[test]
        fn strong_count_across_threads() {
            let shared = create_arc(42);

            // Clone first, then spawn threads to avoid race conditions
            let clones: Vec<_> = (0..4).map(|_| clone_arc(&shared)).collect();

            // Main thread + 4 clones = 5
            assert_eq!(get_strong_count(&shared), 5);

            let handles: Vec<_> = clones
                .into_iter()
                .map(|clone| {
                    thread::spawn(move || {
                        // Each thread has a clone
                        assert!(*clone == 42);
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }

            // Back to 1 after threads complete
            assert_eq!(get_strong_count(&shared), 1);
        }
    }

    // =============================================================================
    // Integration Tests
    // =============================================================================

    mod integration_tests {
        use super::*;

        #[test]
        fn arc_vs_weak_semantics() {
            let strong = create_arc(String::from("data"));
            let weak = create_weak(&strong);

            // Both can access
            assert_eq!(get_value(&strong), "data");
            assert_eq!(*upgrade_weak(&weak).unwrap(), "data");

            // Strong keeps data alive
            assert_eq!(get_strong_count(&strong), 1);
            assert_eq!(get_weak_count(&strong), 1);

            // Drop strong
            drop(strong);

            // Weak can't access anymore
            assert!(upgrade_weak(&weak).is_none());
        }

        #[test]
        fn clone_vs_get_value() {
            let shared = create_arc(vec![1, 2, 3]);

            // clone_arc increments count, shares data
            let cloned = clone_arc(&shared);
            assert_eq!(get_strong_count(&shared), 2);

            // get_value creates independent copy
            let value = get_value(&shared);
            assert_eq!(get_strong_count(&shared), 2); // Still 2, no new Arc

            // Verify they're the same content
            assert_eq!(*cloned, value);
        }

        #[test]
        fn config_workflow() {
            let config = SharedConfig::new("Production".to_string(), 1000, false);

            // Share with multiple workers
            let workers: Vec<_> = (0..4)
                .map(|_| {
                    let cfg = Arc::clone(&config);
                    thread::spawn(move || {
                        // Each worker can access config
                        assert_eq!(cfg.app_name(), "Production");
                        assert_eq!(cfg.max_connections(), 1000);
                        assert!(!cfg.debug_mode());
                    })
                })
                .collect();

            for w in workers {
                w.join().unwrap();
            }
        }

        #[test]
        fn counter_add_and_increment() {
            let counter = AtomicCounter::new_with_value(100);

            counter.add(50); // 100 + 50 = 150
            counter.increment(); // 150 + 1 = 151
            counter.increment(); // 151 + 1 = 152
            counter.decrement(); // 152 - 1 = 151

            assert_eq!(counter.get(), 151);
        }

        #[test]
        fn shared_vec_push_pop_workflow() {
            let vec = SharedVec::new();

            // Push some values
            for i in 0..10 {
                vec.push(i);
            }
            assert_eq!(vec.len(), 10);

            // Pop half
            for _ in 0..5 {
                vec.pop();
            }
            assert_eq!(vec.len(), 5);

            // Check remaining values
            for i in 0..5 {
                assert_eq!(vec.get(i), Some(i));
            }
        }

        #[test]
        fn multiple_handles_to_counter() {
            let c1 = AtomicCounter::new();
            let c2 = c1.clone_counter();
            let c3 = c1.clone_counter();
            let c4 = c2.clone_counter();

            c1.increment();
            c2.increment();
            c3.increment();
            c4.increment();

            assert_eq!(c1.get(), 4);
            assert_eq!(c2.get(), 4);
            assert_eq!(c3.get(), 4);
            assert_eq!(c4.get(), 4);
        }

        #[test]
        fn weak_reference_with_threads() {
            let strong = create_arc(42);
            let weak = create_weak(&strong);

            let handle = thread::spawn(move || {
                // Weak reference can be upgraded in another thread
                let upgraded = upgrade_weak(&weak);
                assert!(upgraded.is_some());
                assert_eq!(*upgraded.unwrap(), 42);
            });

            handle.join().unwrap();
        }

        #[test]
        fn complex_sharing_pattern() {
            let data = create_arc(vec![1, 2, 3, 4, 5]);
            let counter = AtomicCounter::new();

            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let data_clone = clone_arc(&data);
                    let counter_clone = counter.clone_counter();
                    thread::spawn(move || {
                        let sum: i32 = data_clone.iter().sum();
                        counter_clone.add(sum as usize);
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }

            // Sum of [1,2,3,4,5] = 15, times 4 threads = 60
            assert_eq!(counter.get(), 60);
        }

        #[test]
        fn shared_vec_with_strings() {
            let vec = SharedVec::new();
            vec.push(String::from("hello"));
            vec.push(String::from("world"));

            let vec2 = vec.clone_vec();
            vec2.push(String::from("!"));

            assert_eq!(vec.len(), 3);
            assert_eq!(vec.get(2), Some(String::from("!")));
        }

        #[test]
        fn producer_consumer_pattern() {
            let vec = SharedVec::new();
            let counter = AtomicCounter::new();

            let producer_vec = vec.clone_vec();
            let producer = thread::spawn(move || {
                for i in 0..20 {
                    producer_vec.push(i);
                }
            });

            producer.join().unwrap();

            let consumer_vec = vec.clone_vec();
            let consumer_counter = counter.clone_counter();
            let consumer = thread::spawn(move || {
                while let Some(_) = consumer_vec.pop() {
                    consumer_counter.increment();
                }
            });

            consumer.join().unwrap();
            assert_eq!(counter.get(), 20);
            assert!(vec.is_empty());
        }
    }
}
