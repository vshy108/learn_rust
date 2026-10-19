//! # Rustfinity: Rc Reference Counting
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/rc-reference-counting
//!
//! # Rc and Reference Counting
//!
//! In Rust, ownership rules dictate that each value has a single owner, and when the owner goes out of scope, the value is dropped. But what if you need multiple parts of your program to share ownership of the same data? This is where `Rc<T>` (Reference Counted) comes in.
//!
//! `Rc<T>` is a smart pointer that enables **shared ownership** of a value. It keeps track of how many references (or "owners") exist to a value, and only deallocates the value when the last reference is dropped. This is called **reference counting**.
//!
//! `Rc<T>` is useful in scenarios like:
//!
//! - Graph data structures where nodes may have multiple incoming edges
//! - Caching systems where multiple components need access to the same data
//! - Event systems where multiple listeners observe the same source
//!
//! **Important**: `Rc<T>` is only for single-threaded scenarios. For multi-threaded code, use `Arc<T>` (Atomic Reference Counted) instead.
//!
//! ## Key Operations
//!
//! - `Rc::new(value)` - Create a new reference-counted pointer
//! - `Rc::clone(&rc)` - Create a new reference (increments count, doesn't deep clone)
//! - `Rc::strong_count(&rc)` - Get the current number of strong references
//! - `Rc::weak_count(&rc)` - Get the current number of weak references
//! - `Rc::downgrade(&rc)` - Create a `Weak<T>` reference that doesn't prevent deallocation
//! - `weak.upgrade()` - Try to convert a `Weak<T>` back to `Rc<T>` (returns `Option`)
//!
//! ## Your Task
//!
//! Implement the following types and functions to demonstrate `Rc<T>` patterns:
//!
//! ### Part 1: Basic Rc Operations
//!
//! 1. **`create_shared<T>(value: T) -> Rc<T>`** - Create a new reference-counted value
//! 2. **`clone_shared<T>(rc: &Rc<T>) -> Rc<T>`** - Clone a reference (increment count)
//! 3. **`get_strong_count<T>(rc: &Rc<T>) -> usize`** - Get the strong reference count
//! 4. **`get_value<T: Clone>(rc: &Rc<T>) -> T`** - Get a clone of the inner value
//!
//! ### Part 2: Shared Data Structure
//!
//! 5. **`SharedBuffer`** - A buffer that can be shared across multiple owners:
//!    - `new(data: Vec<u8>) -> Rc<Self>` - Create a new shared buffer
//!    - `len(&self) -> usize` - Get the buffer length
//!    - `get(&self, index: usize) -> Option<u8>` - Get a byte at index
//!    - `as_slice(&self) -> &[u8]` - Get the buffer as a slice
//!
//! ### Part 3: Weak References
//!
//! 6. **`create_weak<T>(rc: &Rc<T>) -> Weak<T>`** - Create a weak reference
//! 7. **`upgrade_weak<T>(weak: &Weak<T>) -> Option<Rc<T>>`** - Try to upgrade a weak reference
//! 8. **`get_weak_count<T>(rc: &Rc<T>) -> usize`** - Get the weak reference count
//!
//! ### Part 4: Graph Node with Rc
//!
//! 9. **`Node<T>`** - A graph node that can have multiple parents sharing child nodes:
//!    - `new(value: T) -> Rc<Self>` - Create a new node
//!    - `value(&self) -> &T` - Get reference to the value
//!    - `add_child(&mut self, child: Rc<Node<T>>)` - Add a child node
//!    - `children(&self) -> &[Rc<Node<T>>]` - Get all children
//!
//! ### Part 5: Observer Pattern
//!
//! 10. **`Observable<T>`** - A value with weak observers:
//!     - `new(value: T) -> Self` - Create observable with initial value
//!     - `get(&self) -> &T` - Get reference to value
//!     - `set(&mut self, value: T)` - Set new value
//!     - `subscribe(&mut self, observer: Rc<dyn Observer<T>>)` - Add an observer (stored as Weak)
//!     - `notify(&self)` - Notify all live observers
//!
//! 11. **`Observer<T>` trait**:
//!     - `on_update(&self, value: &T)` - Called when observable changes
//!
//! ## Examples
//!
//! ```rust
//! use rc_reference_counting::*;
//! use std::rc::Rc;
//!
//! // Basic Rc operations
//! let shared = create_shared(42);
//! assert_eq!(get_strong_count(&shared), 1);
//!
//! let cloned = clone_shared(&shared);
//! assert_eq!(get_strong_count(&shared), 2);
//! assert_eq!(get_value(&shared), 42);
//!
//! // Shared buffer
//! let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);
//! assert_eq!(buffer.len(), 5);
//! assert_eq!(buffer.get(2), Some(3));
//!
//! // Weak references
//! let strong = create_shared(String::from("hello"));
//! let weak = create_weak(&strong);
//! assert_eq!(get_weak_count(&strong), 1);
//!
//! if let Some(upgraded) = upgrade_weak(&weak) {
//!     assert_eq!(*upgraded, "hello");
//! }
//!
//! drop(strong);
//! // Value was deallocated
//! assert!(upgrade_weak(&weak).is_none());
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `Rc::new()` to wrap values for shared ownership
//! - `Rc::clone(&rc)` is cheap - it only increments a counter, doesn't copy data
//! - `Rc::downgrade()` creates a `Weak<T>` that doesn't count toward keeping the value alive
//! - `Weak::upgrade()` returns `Option<Rc<T>>` - `None` if the value was already dropped
//! - For `Node` with mutable children, you might need `RefCell` inside the node (interior mutability)
//! - Weak references are perfect for observer patterns to avoid memory leaks from circular references
//! - Remember: `Rc<T>` only works in single-threaded code!
//!
//! </details>
//!
//! Run: `cargo run --example rf_rc_reference_counting`
//! Test: `cargo test --example rf_rc_reference_counting`
//! Filter: `cargo test --example rf_rc_reference_counting <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Creates a new reference-counted value.
///
/// Wraps the given value in an `Rc<T>` for shared ownership.
pub fn create_shared<T>(value: T) -> Rc<T> {
    // TODO: Implement using Rc::new
    unimplemented!()
}

/// Clones a reference-counted pointer.
///
/// This increments the reference count without cloning the underlying data.
pub fn clone_shared<T>(rc: &Rc<T>) -> Rc<T> {
    // TODO: Implement using Rc::clone
    unimplemented!()
}

/// Returns the strong reference count of an Rc.
pub fn get_strong_count<T>(rc: &Rc<T>) -> usize {
    // TODO: Implement using Rc::strong_count
    unimplemented!()
}

/// Gets a clone of the value inside an Rc.
pub fn get_value<T: Clone>(rc: &Rc<T>) -> T {
    // TODO: Dereference the Rc and clone the value
    unimplemented!()
}

/// A buffer that can be shared across multiple owners.
#[derive(Debug, Clone)]
pub struct SharedBuffer {
    data: Vec<u8>,
}

impl SharedBuffer {
    /// Creates a new shared buffer wrapped in Rc.
    pub fn new(data: Vec<u8>) -> Rc<Self> {
        // TODO: Create a SharedBuffer and wrap it in Rc
        unimplemented!()
    }

    /// Returns the length of the buffer.
    pub fn len(&self) -> usize {
        // TODO: Return the length of the data
        unimplemented!()
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Check if the data is empty
        unimplemented!()
    }

    /// Gets a byte at the specified index.
    pub fn get(&self, index: usize) -> Option<u8> {
        // TODO: Return the byte at index, or None if out of bounds
        unimplemented!()
    }

    /// Returns the buffer data as a slice.
    pub fn as_slice(&self) -> &[u8] {
        // TODO: Return a reference to the data as a slice
        unimplemented!()
    }
}

/// Creates a weak reference from an Rc.
///
/// Weak references don't prevent the value from being deallocated.
pub fn create_weak<T>(rc: &Rc<T>) -> Weak<T> {
    // TODO: Implement using Rc::downgrade
    unimplemented!()
}

/// Attempts to upgrade a weak reference to a strong reference.
///
/// Returns `Some(Rc<T>)` if the value still exists, `None` if it was deallocated.
pub fn upgrade_weak<T>(weak: &Weak<T>) -> Option<Rc<T>> {
    // TODO: Implement using Weak::upgrade
    unimplemented!()
}

/// Returns the weak reference count of an Rc.
pub fn get_weak_count<T>(rc: &Rc<T>) -> usize {
    // TODO: Implement using Rc::weak_count
    unimplemented!()
}

/// A graph node that can have multiple parents sharing child nodes.
///
/// Uses `Rc<RefCell<...>>` pattern for interior mutability.
#[derive(Debug)]
pub struct Node<T> {
    value: T,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    /// Creates a new node wrapped in Rc.
    pub fn new(value: T) -> Rc<Self> {
        // TODO: Create a Node with empty children and wrap in Rc
        unimplemented!()
    }

    /// Returns a reference to the node's value.
    pub fn value(&self) -> &T {
        // TODO: Return a reference to the value field
        unimplemented!()
    }

    /// Adds a child node.
    pub fn add_child(&self, child: Rc<Node<T>>) {
        // TODO: Borrow the children RefCell mutably and push the child
        unimplemented!()
    }

    /// Returns a clone of the children vector.
    pub fn children(&self) -> Vec<Rc<Node<T>>> {
        // TODO: Borrow the children and clone the vector
        unimplemented!()
    }

    /// Returns the number of children.
    pub fn children_count(&self) -> usize {
        // TODO: Borrow children and return the length
        unimplemented!()
    }
}

/// Trait for objects that can observe changes to an Observable.
pub trait Observer<T> {
    /// Called when the observed value is updated.
    fn on_update(&self, value: &T);
}

/// A value that can notify observers when it changes.
///
/// Observers are stored as weak references to prevent memory leaks.
pub struct Observable<T> {
    value: T,
    observers: Vec<Weak<dyn Observer<T>>>,
}

impl<T> Observable<T> {
    /// Creates a new Observable with an initial value.
    pub fn new(value: T) -> Self {
        // TODO: Create Observable with the value and an empty observers vector
        unimplemented!()
    }

    /// Returns a reference to the current value.
    pub fn get(&self) -> &T {
        // TODO: Return a reference to the value
        unimplemented!()
    }

    /// Sets a new value.
    pub fn set(&mut self, value: T) {
        // TODO: Update the value field
        unimplemented!()
    }

    /// Subscribes an observer.
    ///
    /// The observer is stored as a weak reference to avoid memory leaks.
    pub fn subscribe(&mut self, observer: Rc<dyn Observer<T>>) {
        // TODO: Create a weak reference from the observer and add to the vector
        unimplemented!()
    }

    /// Notifies all live observers of the current value.
    ///
    /// Dead observers (whose Rc has been dropped) are automatically removed.
    pub fn notify(&mut self) {
        // TODO: Iterate through observers, upgrade each weak reference
        // If upgrade succeeds, call on_update with the current value
        // Remove observers that can't be upgraded
        unimplemented!()
    }

    /// Returns the number of live observers.
    pub fn observer_count(&self) -> usize {
        // TODO: Count observers whose weak references can be upgraded
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    // Basic Rc operations
    let shared = create_shared(42);
    println!("Value: {}", get_value(&shared));
    println!("Strong count: {}", get_strong_count(&shared));

    let cloned = clone_shared(&shared);
    println!("After clone, strong count: {}", get_strong_count(&shared));

    // Shared buffer
    let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);
    println!("Buffer length: {}", buffer.len());
    println!("Buffer[2]: {:?}", buffer.get(2));

    // Weak references
    let strong = create_shared(String::from("hello"));
    let weak = create_weak(&strong);
    println!("Weak count: {}", get_weak_count(&strong));

    if let Some(upgraded) = upgrade_weak(&weak) {
        println!("Upgraded value: {}", *upgraded);
    }

    // Node tree
    let root = Node::new("root");
    let child1 = Node::new("child1");
    let child2 = Node::new("child2");

    root.add_child(child1);
    root.add_child(child2);
    println!("Root has {} children", root.children_count());

    // Observable pattern
    use std::cell::Cell;

    struct PrintObserver;
    impl Observer<i32> for PrintObserver {
        fn on_update(&self, value: &i32) {
            println!("Value changed to: {}", value);
        }
    }

    let mut observable = Observable::new(0);
    let observer = Rc::new(PrintObserver);
    observable.subscribe(observer);

    observable.set(42);
    observable.notify();
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::cell::Cell;
    use std::rc::Rc;

    // =============================================================================
    // Basic Rc Operations Tests
    // =============================================================================

    mod create_shared_tests {
        use super::*;

        #[test]
        fn creates_shared_integer() {
            let shared = create_shared(42);
            assert_eq!(*shared, 42);
        }

        #[test]
        fn creates_shared_string() {
            let shared = create_shared(String::from("hello"));
            assert_eq!(*shared, "hello");
        }

        #[test]
        fn creates_shared_vec() {
            let shared = create_shared(vec![1, 2, 3]);
            assert_eq!(*shared, vec![1, 2, 3]);
        }

        #[test]
        fn creates_shared_zero() {
            let shared = create_shared(0);
            assert_eq!(*shared, 0);
        }

        #[test]
        fn creates_shared_negative() {
            let shared = create_shared(-100);
            assert_eq!(*shared, -100);
        }

        #[test]
        fn creates_shared_float() {
            let shared = create_shared(3.14f64);
            assert!((*shared - 3.14).abs() < f64::EPSILON);
        }

        #[test]
        fn creates_shared_tuple() {
            let shared = create_shared((1, "two", 3.0));
            assert_eq!(*shared, (1, "two", 3.0));
        }

        #[test]
        fn initial_count_is_one() {
            let shared = create_shared(42);
            assert_eq!(get_strong_count(&shared), 1);
        }
    }

    mod clone_shared_tests {
        use super::*;

        #[test]
        fn clone_increments_count() {
            let original = create_shared(42);
            let _cloned = clone_shared(&original);
            assert_eq!(get_strong_count(&original), 2);
        }

        #[test]
        fn multiple_clones() {
            let original = create_shared(42);
            let _c1 = clone_shared(&original);
            let _c2 = clone_shared(&original);
            let _c3 = clone_shared(&original);
            assert_eq!(get_strong_count(&original), 4);
        }

        #[test]
        fn clones_share_same_value() {
            let original = create_shared(42);
            let cloned = clone_shared(&original);
            assert_eq!(*original, *cloned);
        }

        #[test]
        fn clone_of_clone() {
            let original = create_shared(42);
            let c1 = clone_shared(&original);
            let _c2 = clone_shared(&c1);
            assert_eq!(get_strong_count(&original), 3);
        }

        #[test]
        fn drop_decrements_count() {
            let original = create_shared(42);
            {
                let _cloned = clone_shared(&original);
                assert_eq!(get_strong_count(&original), 2);
            }
            assert_eq!(get_strong_count(&original), 1);
        }

        #[test]
        fn clone_string() {
            let original = create_shared(String::from("hello"));
            let cloned = clone_shared(&original);
            assert_eq!(*cloned, "hello");
            assert_eq!(get_strong_count(&original), 2);
        }
    }

    mod get_strong_count_tests {
        use super::*;

        #[test]
        fn count_one() {
            let shared = create_shared(42);
            assert_eq!(get_strong_count(&shared), 1);
        }

        #[test]
        fn count_increases_with_clones() {
            let shared = create_shared(42);
            let clones: Vec<_> = (0..5).map(|_| clone_shared(&shared)).collect();
            assert_eq!(get_strong_count(&shared), 6);
            drop(clones);
            assert_eq!(get_strong_count(&shared), 1);
        }

        #[test]
        fn count_through_clone() {
            let original = create_shared(42);
            let cloned = clone_shared(&original);
            assert_eq!(get_strong_count(&cloned), 2);
        }
    }

    mod get_value_tests {
        use super::*;

        #[test]
        fn gets_integer_value() {
            let shared = create_shared(42);
            assert_eq!(get_value(&shared), 42);
        }

        #[test]
        fn gets_string_value() {
            let shared = create_shared(String::from("hello"));
            let value = get_value(&shared);
            assert_eq!(value, "hello");
        }

        #[test]
        fn gets_vec_value() {
            let shared = create_shared(vec![1, 2, 3]);
            let value = get_value(&shared);
            assert_eq!(value, vec![1, 2, 3]);
        }

        #[test]
        fn value_is_independent_clone() {
            let shared = create_shared(String::from("hello"));
            let mut value = get_value(&shared);
            value.push_str(" world");
            assert_eq!(*shared, "hello"); // Original unchanged
            assert_eq!(value, "hello world");
        }

        #[test]
        fn gets_empty_string() {
            let shared = create_shared(String::new());
            assert_eq!(get_value(&shared), "");
        }

        #[test]
        fn gets_negative() {
            let shared = create_shared(-999);
            assert_eq!(get_value(&shared), -999);
        }
    }

    // =============================================================================
    // SharedBuffer Tests
    // =============================================================================

    mod shared_buffer_tests {
        use super::*;

        #[test]
        fn new_creates_buffer() {
            let buffer = SharedBuffer::new(vec![1, 2, 3]);
            assert_eq!(buffer.len(), 3);
        }

        #[test]
        fn empty_buffer() {
            let buffer = SharedBuffer::new(vec![]);
            assert!(buffer.is_empty());
            assert_eq!(buffer.len(), 0);
        }

        #[test]
        fn get_valid_index() {
            let buffer = SharedBuffer::new(vec![10, 20, 30, 40, 50]);
            assert_eq!(buffer.get(0), Some(10));
            assert_eq!(buffer.get(2), Some(30));
            assert_eq!(buffer.get(4), Some(50));
        }

        #[test]
        fn get_invalid_index() {
            let buffer = SharedBuffer::new(vec![1, 2, 3]);
            assert_eq!(buffer.get(3), None);
            assert_eq!(buffer.get(100), None);
        }

        #[test]
        fn get_from_empty() {
            let buffer = SharedBuffer::new(vec![]);
            assert_eq!(buffer.get(0), None);
        }

        #[test]
        fn as_slice() {
            let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);
            assert_eq!(buffer.as_slice(), &[1, 2, 3, 4, 5]);
        }

        #[test]
        fn as_slice_empty() {
            let buffer = SharedBuffer::new(vec![]);
            assert_eq!(buffer.as_slice(), &[]);
        }

        #[test]
        fn shared_between_owners() {
            let buffer = SharedBuffer::new(vec![1, 2, 3]);
            let buffer2 = Rc::clone(&buffer);

            assert_eq!(buffer.len(), buffer2.len());
            assert_eq!(buffer.get(1), buffer2.get(1));
            assert_eq!(Rc::strong_count(&buffer), 2);
        }

        #[test]
        fn large_buffer() {
            let data: Vec<u8> = (0..255).collect();
            let buffer = SharedBuffer::new(data);
            assert_eq!(buffer.len(), 255);
            assert_eq!(buffer.get(100), Some(100));
        }

        #[test]
        fn is_empty_false() {
            let buffer = SharedBuffer::new(vec![1]);
            assert!(!buffer.is_empty());
        }
    }

    // =============================================================================
    // Weak Reference Tests
    // =============================================================================

    mod weak_reference_tests {
        use super::*;

        #[test]
        fn create_weak_basic() {
            let strong = create_shared(42);
            let _weak = create_weak(&strong);
            assert_eq!(get_weak_count(&strong), 1);
        }

        #[test]
        fn weak_doesnt_affect_strong_count() {
            let strong = create_shared(42);
            let _weak = create_weak(&strong);
            assert_eq!(get_strong_count(&strong), 1);
        }

        #[test]
        fn multiple_weak_refs() {
            let strong = create_shared(42);
            let _w1 = create_weak(&strong);
            let _w2 = create_weak(&strong);
            let _w3 = create_weak(&strong);
            assert_eq!(get_weak_count(&strong), 3);
        }

        #[test]
        fn upgrade_succeeds_when_alive() {
            let strong = create_shared(42);
            let weak = create_weak(&strong);

            let upgraded = upgrade_weak(&weak);
            assert!(upgraded.is_some());
            assert_eq!(*upgraded.unwrap(), 42);
        }

        #[test]
        fn upgrade_fails_when_dropped() {
            let weak = {
                let strong = create_shared(42);
                create_weak(&strong)
            };

            assert!(upgrade_weak(&weak).is_none());
        }

        #[test]
        fn upgrade_increments_count() {
            let strong = create_shared(42);
            let weak = create_weak(&strong);

            let upgraded = upgrade_weak(&weak).unwrap();
            assert_eq!(get_strong_count(&strong), 2);
            drop(upgraded);
            assert_eq!(get_strong_count(&strong), 1);
        }

        #[test]
        fn weak_count_zero_initially() {
            let strong = create_shared(42);
            assert_eq!(get_weak_count(&strong), 0);
        }

        #[test]
        fn weak_with_string() {
            let strong = create_shared(String::from("hello"));
            let weak = create_weak(&strong);

            let upgraded = upgrade_weak(&weak).unwrap();
            assert_eq!(*upgraded, "hello");
        }

        #[test]
        fn weak_survives_strong_drop() {
            let weak = {
                let strong = create_shared(42);
                create_weak(&strong)
                // strong dropped here
            };
            // weak still exists, but can't upgrade
            assert!(upgrade_weak(&weak).is_none());
        }

        #[test]
        fn multiple_upgrade_attempts() {
            let strong = create_shared(42);
            let weak = create_weak(&strong);

            assert!(upgrade_weak(&weak).is_some());
            assert!(upgrade_weak(&weak).is_some());
            assert!(upgrade_weak(&weak).is_some());
        }
    }

    // =============================================================================
    // Node Tests
    // =============================================================================

    mod node_tests {
        use super::*;

        #[test]
        fn new_node() {
            let node = Node::new(42);
            assert_eq!(*node.value(), 42);
        }

        #[test]
        fn node_with_string() {
            let node = Node::new(String::from("root"));
            assert_eq!(node.value(), "root");
        }

        #[test]
        fn empty_children() {
            let node = Node::new(1);
            assert!(node.children().is_empty());
            assert_eq!(node.children_count(), 0);
        }

        #[test]
        fn add_single_child() {
            let parent = Node::new("parent");
            let child = Node::new("child");

            parent.add_child(child);

            assert_eq!(parent.children_count(), 1);
        }

        #[test]
        fn add_multiple_children() {
            let parent = Node::new(0);
            let c1 = Node::new(1);
            let c2 = Node::new(2);
            let c3 = Node::new(3);

            parent.add_child(c1);
            parent.add_child(c2);
            parent.add_child(c3);

            assert_eq!(parent.children_count(), 3);
        }

        #[test]
        fn children_preserve_values() {
            let parent = Node::new("parent");
            let child = Node::new("child");
            parent.add_child(child);

            let children = parent.children();
            assert_eq!(*children[0].value(), "child");
        }

        #[test]
        fn shared_child() {
            let parent1 = Node::new("p1");
            let parent2 = Node::new("p2");
            let child = Node::new("shared");

            parent1.add_child(Rc::clone(&child));
            parent2.add_child(Rc::clone(&child));

            // Child is shared by both parents
            assert_eq!(Rc::strong_count(&child), 3); // original + 2 parents
        }

        #[test]
        fn nested_tree() {
            let root = Node::new(0);
            let level1 = Node::new(1);
            let level2 = Node::new(2);

            level1.add_child(level2);
            root.add_child(level1);

            assert_eq!(root.children_count(), 1);
            let children = root.children();
            assert_eq!(children[0].children_count(), 1);
        }

        #[test]
        fn node_reference_counting() {
            let node = Node::new(42);
            assert_eq!(Rc::strong_count(&node), 1);

            let cloned = Rc::clone(&node);
            assert_eq!(Rc::strong_count(&node), 2);

            drop(cloned);
            assert_eq!(Rc::strong_count(&node), 1);
        }

        #[test]
        fn children_are_clones() {
            let parent = Node::new("parent");
            let child = Node::new("child");
            parent.add_child(Rc::clone(&child));

            // Getting children returns clones
            let children1 = parent.children();
            let children2 = parent.children();

            // Same underlying node
            assert_eq!(children1[0].value(), children2[0].value());
        }
    }

    // =============================================================================
    // Observable Tests
    // =============================================================================

    struct CountingObserver {
        count: Cell<i32>,
        last_value: Cell<i32>,
    }

    impl CountingObserver {
        fn new() -> Self {
            CountingObserver {
                count: Cell::new(0),
                last_value: Cell::new(0),
            }
        }
    }

    impl Observer<i32> for CountingObserver {
        fn on_update(&self, value: &i32) {
            self.count.set(self.count.get() + 1);
            self.last_value.set(*value);
        }
    }

    struct StringObserver {
        received: Cell<bool>,
    }

    impl Observer<String> for StringObserver {
        fn on_update(&self, _value: &String) {
            self.received.set(true);
        }
    }

    mod observable_tests {
        use super::*;

        #[test]
        fn new_observable() {
            let observable = Observable::new(42);
            assert_eq!(*observable.get(), 42);
        }

        #[test]
        fn set_value() {
            let mut observable = Observable::new(1);
            observable.set(2);
            assert_eq!(*observable.get(), 2);
        }

        #[test]
        fn multiple_sets() {
            let mut observable = Observable::new(0);
            observable.set(1);
            observable.set(2);
            observable.set(3);
            assert_eq!(*observable.get(), 3);
        }

        #[test]
        fn subscribe_observer() {
            let mut observable = Observable::new(0);
            let observer = Rc::new(CountingObserver::new());

            observable.subscribe(observer.clone());
            assert_eq!(observable.observer_count(), 1);
        }

        #[test]
        fn notify_observers() {
            let mut observable = Observable::new(0);
            let observer = Rc::new(CountingObserver::new());

            observable.subscribe(observer.clone());
            observable.set(42);
            observable.notify();

            assert_eq!(observer.count.get(), 1);
            assert_eq!(observer.last_value.get(), 42);
        }

        #[test]
        fn multiple_observers() {
            let mut observable = Observable::new(0);
            let obs1 = Rc::new(CountingObserver::new());
            let obs2 = Rc::new(CountingObserver::new());

            observable.subscribe(obs1.clone());
            observable.subscribe(obs2.clone());

            observable.set(10);
            observable.notify();

            assert_eq!(obs1.last_value.get(), 10);
            assert_eq!(obs2.last_value.get(), 10);
        }

        #[test]
        fn multiple_notifications() {
            let mut observable = Observable::new(0);
            let observer = Rc::new(CountingObserver::new());

            observable.subscribe(observer.clone());

            observable.set(1);
            observable.notify();
            observable.set(2);
            observable.notify();
            observable.set(3);
            observable.notify();

            assert_eq!(observer.count.get(), 3);
            assert_eq!(observer.last_value.get(), 3);
        }

        #[test]
        fn dead_observer_removed() {
            let mut observable = Observable::new(0);

            {
                let observer = Rc::new(CountingObserver::new());
                observable.subscribe(observer.clone());
                assert_eq!(observable.observer_count(), 1);
            }
            // observer dropped

            observable.notify(); // Should clean up dead observer
            assert_eq!(observable.observer_count(), 0);
        }

        #[test]
        fn some_observers_survive() {
            let mut observable = Observable::new(0);
            let survivor = Rc::new(CountingObserver::new());

            observable.subscribe(survivor.clone());
            {
                let temp = Rc::new(CountingObserver::new());
                observable.subscribe(temp.clone());
                assert_eq!(observable.observer_count(), 2);
            }

            observable.set(5);
            observable.notify();

            assert_eq!(observable.observer_count(), 1);
            assert_eq!(survivor.last_value.get(), 5);
        }

        #[test]
        fn string_observable() {
            let mut observable = Observable::new(String::from("hello"));
            let observer = Rc::new(StringObserver {
                received: Cell::new(false),
            });

            observable.subscribe(observer.clone());
            observable.set(String::from("world"));
            observable.notify();

            assert!(observer.received.get());
        }

        #[test]
        fn no_observers() {
            let mut observable = Observable::new(42);
            observable.notify(); // Should not panic
            assert_eq!(observable.observer_count(), 0);
        }

        #[test]
        fn observer_count_accurate() {
            let mut observable = Observable::new(0);

            assert_eq!(observable.observer_count(), 0);

            let o1 = Rc::new(CountingObserver::new());
            observable.subscribe(o1.clone());
            assert_eq!(observable.observer_count(), 1);

            let o2 = Rc::new(CountingObserver::new());
            observable.subscribe(o2.clone());
            assert_eq!(observable.observer_count(), 2);

            drop(o1);
            // Count still 2 until notify cleans up
            observable.notify();
            assert_eq!(observable.observer_count(), 1);
        }
    }

    // =============================================================================
    // Integration Tests
    // =============================================================================

    mod integration_tests {
        use super::*;

        #[test]
        fn shared_buffer_workflow() {
            // Create a shared buffer
            let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);

            // Share among multiple owners
            let owner1 = Rc::clone(&buffer);
            let owner2 = Rc::clone(&buffer);

            // All see the same data
            assert_eq!(owner1.as_slice(), owner2.as_slice());
            assert_eq!(Rc::strong_count(&buffer), 3);

            // One owner drops
            drop(owner1);
            assert_eq!(Rc::strong_count(&buffer), 2);

            // Data still accessible
            assert_eq!(buffer.get(2), Some(3));
        }

        #[test]
        fn tree_structure() {
            // Build a tree
            let root = Node::new("root");
            let a = Node::new("a");
            let b = Node::new("b");
            let a1 = Node::new("a1");
            let a2 = Node::new("a2");

            a.add_child(a1);
            a.add_child(a2);
            root.add_child(a);
            root.add_child(b);

            // Verify structure
            assert_eq!(root.children_count(), 2);
            let children = root.children();
            assert_eq!(*children[0].value(), "a");
            assert_eq!(children[0].children_count(), 2);
        }

        #[test]
        fn diamond_pattern() {
            // Diamond dependency: A -> B, A -> C, B -> D, C -> D
            let d = Node::new("D");
            let b = Node::new("B");
            let c = Node::new("C");
            let a = Node::new("A");

            b.add_child(Rc::clone(&d));
            c.add_child(Rc::clone(&d));
            a.add_child(Rc::clone(&b));
            a.add_child(Rc::clone(&c));

            // D is shared by both B and C
            assert_eq!(Rc::strong_count(&d), 3); // original + B's + C's
        }

        #[test]
        fn weak_ref_observer_pattern() {
            let mut observable = Observable::new(0);
            let weak_check = {
                let observer = Rc::new(CountingObserver::new());
                observable.subscribe(observer.clone());

                // Create a weak ref to check if observer is alive
                Rc::downgrade(&observer)
            };

            // Observer was dropped, weak should fail to upgrade
            assert!(weak_check.upgrade().is_none());

            // Observable should clean up dead observer on notify
            observable.notify();
            assert_eq!(observable.observer_count(), 0);
        }

        #[test]
        fn rc_vs_weak_semantics() {
            let strong = create_shared(String::from("data"));
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
            let shared = create_shared(vec![1, 2, 3]);

            // clone_shared increments count, shares data
            let cloned = clone_shared(&shared);
            assert_eq!(get_strong_count(&shared), 2);

            // get_value creates independent copy
            let value = get_value(&shared);
            assert_eq!(get_strong_count(&shared), 2); // Still 2, no new Rc

            // Verify they're the same content
            assert_eq!(*cloned, value);
        }

        #[test]
        fn observable_lifecycle() {
            let mut observable = Observable::new(100);

            // Add observers
            let o1 = Rc::new(CountingObserver::new());
            let o2 = Rc::new(CountingObserver::new());
            let o3 = Rc::new(CountingObserver::new());

            observable.subscribe(o1.clone());
            observable.subscribe(o2.clone());
            observable.subscribe(o3.clone());

            // Notify all
            observable.set(200);
            observable.notify();
            assert_eq!(o1.last_value.get(), 200);
            assert_eq!(o2.last_value.get(), 200);
            assert_eq!(o3.last_value.get(), 200);

            // Drop one
            drop(o2);
            observable.set(300);
            observable.notify();

            // Remaining observers updated
            assert_eq!(o1.last_value.get(), 300);
            assert_eq!(o3.last_value.get(), 300);
            assert_eq!(observable.observer_count(), 2);
        }

        #[test]
        fn buffer_with_weak_reference() {
            let weak = {
                let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);
                let weak = Rc::downgrade(&buffer);

                // Buffer still alive
                assert!(weak.upgrade().is_some());
                weak
            };

            // Buffer dropped, weak can't upgrade
            assert!(weak.upgrade().is_none());
        }

        #[test]
        fn node_reference_cleanup() {
            let parent = Node::new("parent");

            {
                let child = Node::new("child");
                parent.add_child(Rc::clone(&child));
                assert_eq!(Rc::strong_count(&child), 2);
            }
            // child's original Rc dropped, but parent still holds one

            // Children still accessible through parent
            let children = parent.children();
            assert_eq!(children.len(), 1);
            assert_eq!(*children[0].value(), "child");
        }

        #[test]
        fn complex_sharing() {
            // Shared data accessed by multiple components
            let shared_data = create_shared(vec![1, 2, 3, 4, 5]);

            let processors: Vec<_> = (0..5).map(|_| clone_shared(&shared_data)).collect();

            assert_eq!(get_strong_count(&shared_data), 6);

            // All processors see the same data
            for p in &processors {
                assert_eq!(get_value(p), vec![1, 2, 3, 4, 5]);
            }

            // Drop all processors
            drop(processors);
            assert_eq!(get_strong_count(&shared_data), 1);
        }
    }
}
