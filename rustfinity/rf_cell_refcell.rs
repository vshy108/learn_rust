//! # Rustfinity: Cell Refcell
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/cell-refcell
//!
//! # Cell and RefCell: Interior Mutability
//!
//! Rust's ownership and borrowing rules are enforced at compile time, ensuring memory safety without a garbage collector. However, there are situations where you need to mutate data even when there are immutable references to it. This pattern is called **interior mutability**.
//!
//! The standard library provides two key types for interior mutability:
//!
//! - **`Cell<T>`**: Provides interior mutability for `Copy` types. It moves values in and out rather than borrowing, making it zero-cost but limited to types that implement `Copy`.
//! - **`RefCell<T>`**: Provides interior mutability for any type by tracking borrows at runtime. It enforces borrowing rules dynamically and will panic if you violate them (e.g., two mutable borrows at once).
//!
//! ## When to Use Each
//!
//! Use **`Cell<T>`** when:
//!
//! - Working with `Copy` types (integers, booleans, small structs)
//! - You need simple get/set operations
//! - You want zero runtime overhead
//!
//! Use **`RefCell<T>`** when:
//!
//! - Working with non-`Copy` types (String, Vec, etc.)
//! - You need to borrow the inner value (get a reference)
//! - You're okay with runtime borrow checking
//!
//! ## Key Operations
//!
//! **`Cell<T>`:**
//!
//! - `Cell::new(value)` - Create a new Cell
//! - `cell.get()` - Get a copy of the value (requires `T: Copy`)
//! - `cell.set(value)` - Set a new value
//! - `cell.replace(value)` - Set value and return old value
//! - `cell.take()` - Take the value, leaving `Default::default()` (requires `T: Default`)
//!
//! **`RefCell<T>`:**
//!
//! - `RefCell::new(value)` - Create a new RefCell
//! - `refcell.borrow()` - Get immutable reference (`Ref<T>`), panics if mutably borrowed
//! - `refcell.borrow_mut()` - Get mutable reference (`RefMut<T>`), panics if any borrows exist
//! - `refcell.try_borrow()` - Returns `Result<Ref<T>, BorrowError>`
//! - `refcell.try_borrow_mut()` - Returns `Result<RefMut<T>, BorrowMutError>`
//!
//! ## Your Task
//!
//! Implement the following types and functions to demonstrate interior mutability patterns:
//!
//! ### Part 1: Cell Basics
//!
//! 1. **`Counter`** - A simple counter using Cell:
//!    - `new() -> Self` - Create counter with value 0
//!    - `get(&self) -> i32` - Get current count
//!    - `set(&self, value: i32)` - Set the count
//!    - `increment(&self)` - Increment by 1
//!    - `decrement(&self)` - Decrement by 1
//!
//! 2. **`CachedValue<T: Copy + Default>`** - A value that tracks access count:
//!    - `new(value: T) -> Self` - Create with initial value
//!    - `get(&self) -> T` - Get value (increments access count)
//!    - `set(&self, value: T)` - Set value (resets access count)
//!    - `access_count(&self) -> u32` - Get number of times value was accessed
//!
//! ### Part 2: RefCell Basics
//!
//! 3. **`SharedString`** - A string that can be modified through shared references:
//!    - `new(s: &str) -> Self` - Create with initial string
//!    - `get(&self) -> String` - Get a clone of the string
//!    - `set(&self, s: &str)` - Set a new string
//!    - `append(&self, s: &str)` - Append to the string
//!    - `len(&self) -> usize` - Get string length
//!    - `is_empty(&self) -> bool` - Check if empty
//!
//! 4. **`SharedVec<T>`** - A vector with interior mutability:
//!    - `new() -> Self` - Create empty vector
//!    - `push(&self, value: T)` - Add element
//!    - `pop(&self) -> Option<T>` - Remove and return last element
//!    - `len(&self) -> usize` - Get length
//!    - `is_empty(&self) -> bool` - Check if empty
//!    - `get(&self, index: usize) -> Option<T>` where `T: Clone` - Get element at index
//!
//! ### Part 3: Safe Borrowing with try_borrow
//!
//! 5. **`SafeCell<T>`** - A RefCell wrapper with safe borrowing:
//!    - `new(value: T) -> Self` - Create with initial value
//!    - `try_read(&self) -> Option<std::cell::Ref<T>>` - Try to borrow immutably
//!    - `try_write(&self) -> Option<std::cell::RefMut<T>>` - Try to borrow mutably
//!    - `is_borrowed(&self) -> bool` - Check if currently borrowed (any type)
//!    - `with_value<F, R>(&self, f: F) -> Option<R>` where `F: FnOnce(&T) -> R` - Apply function if not borrowed
//!    - `with_value_mut<F, R>(&self, f: F) -> Option<R>` where `F: FnOnce(&mut T) -> R` - Apply mutating function
//!
//! ### Part 4: Combining Rc and RefCell
//!
//! 6. **`SharedCounter`** - A reference-counted counter with interior mutability:
//!    - `new() -> Rc<Self>` - Create new counter wrapped in Rc
//!    - `get(&self) -> i32` - Get current value
//!    - `increment(&self)` - Increment by 1
//!    - `decrement(&self)` - Decrement by 1
//!    - `add(&self, n: i32)` - Add n to counter
//!
//! 7. **`TreeNode<T>`** - A tree node where children can be added through shared references:
//!    - `new(value: T) -> Rc<Self>` - Create leaf node
//!    - `value(&self) -> std::cell::Ref<T>` - Get reference to value
//!    - `set_value(&self, value: T)` - Set new value
//!    - `add_child(&self, child: Rc<TreeNode<T>>)` - Add a child node
//!    - `children_count(&self) -> usize` - Get number of children
//!
//! ## Examples
//!
//! ```rust
//! use cell_refcell::*;
//! use std::rc::Rc;
//!
//! // Cell-based Counter
//! let counter = Counter::new();
//! counter.increment();
//! counter.increment();
//! assert_eq!(counter.get(), 2);
//!
//! // CachedValue with access tracking
//! let cached = CachedValue::new(42);
//! assert_eq!(cached.get(), 42);
//! assert_eq!(cached.get(), 42);
//! assert_eq!(cached.access_count(), 2);
//!
//! // RefCell-based SharedString
//! let shared = SharedString::new("Hello");
//! shared.append(", World!");
//! assert_eq!(shared.get(), "Hello, World!");
//!
//! // SharedVec
//! let vec: SharedVec<i32> = SharedVec::new();
//! vec.push(1);
//! vec.push(2);
//! assert_eq!(vec.pop(), Some(2));
//! assert_eq!(vec.len(), 1);
//!
//! // SafeCell with try_borrow
//! let safe = SafeCell::new(42);
//! let result = safe.with_value(|v| *v * 2);
//! assert_eq!(result, Some(84));
//!
//! // Rc<RefCell> pattern with SharedCounter
//! let counter = SharedCounter::new();
//! let counter2 = Rc::clone(&counter);
//! counter.increment();
//! counter2.increment();
//! assert_eq!(counter.get(), 2);
//!
//! // Tree with interior mutability
//! let root = TreeNode::new("root");
//! let child1 = TreeNode::new("child1");
//! root.add_child(child1);
//! assert_eq!(root.children_count(), 1);
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - `Cell<T>` uses `.get()` and `.set()` - no borrowing, just copying values
//! - `RefCell<T>` uses `.borrow()` for `&T` and `.borrow_mut()` for `&mut T`
//! - `try_borrow()` and `try_borrow_mut()` return `Result` instead of panicking
//! - When combining `Rc<RefCell<T>>`, use `Rc::clone()` for sharing and `borrow()/borrow_mut()` for access
//! - `Ref<T>` and `RefMut<T>` implement `Deref` and `DerefMut`, so you can use them like regular references
//! - For `SafeCell::is_borrowed()`, you can try to borrow and see if it succeeds
//! - Remember that `RefCell` will panic on invalid borrows - use `try_*` methods when unsure
//! - The `TreeNode` pattern (`Rc<RefCell<Vec<Rc<TreeNode>>>>`) is common for tree structures in Rust
//!
//! </details>
//!
//! Run: `cargo run --example rf_cell_refcell`
//! Test: `cargo test --example rf_cell_refcell`
//! Filter: `cargo test --example rf_cell_refcell <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::cell::{Cell, Ref, RefCell, RefMut};
use std::rc::Rc;

// =============================================================================
// Part 1: Cell Basics
// =============================================================================

/// A simple counter using `Cell` for interior mutability.
///
/// `Cell<T>` provides interior mutability for `Copy` types with zero overhead.
/// All operations work through shared references (&self).
#[derive(Debug, Default)]
pub struct Counter {
    // TODO: Add a Cell<i32> field named `value`
}

impl Counter {
    /// Creates a new counter with initial value 0.
    pub fn new() -> Self {
        // TODO: Implement using Cell::new(0)
        unimplemented!()
    }

    /// Gets the current count.
    pub fn get(&self) -> i32 {
        // TODO: Use Cell::get() to return the value
        unimplemented!()
    }

    /// Sets the count to a specific value.
    pub fn set(&self, value: i32) {
        // TODO: Use Cell::set() to set the value
        unimplemented!()
    }

    /// Increments the count by 1.
    pub fn increment(&self) {
        // TODO: Get current value, add 1, and set it back
        unimplemented!()
    }

    /// Decrements the count by 1.
    pub fn decrement(&self) {
        // TODO: Get current value, subtract 1, and set it back
        unimplemented!()
    }
}

/// A cached value that tracks how many times it's been accessed.
///
/// Uses `Cell` for both the value and access count.
#[derive(Debug)]
pub struct CachedValue<T: Copy + Default> {
    // TODO: Add a Cell<T> field named `value`
    // TODO: Add a Cell<u32> field named `access_count`
    _marker: std::marker::PhantomData<T>,
}

impl<T: Copy + Default> CachedValue<T> {
    /// Creates a new cached value.
    pub fn new(value: T) -> Self {
        // TODO: Initialize both cells, access_count starts at 0
        unimplemented!()
    }

    /// Gets the cached value and increments the access count.
    pub fn get(&self) -> T {
        // TODO: Increment access_count, then return the value
        unimplemented!()
    }

    /// Sets a new value and resets the access count.
    pub fn set(&self, value: T) {
        // TODO: Set the new value and reset access_count to 0
        unimplemented!()
    }

    /// Returns the number of times the value was accessed.
    pub fn access_count(&self) -> u32 {
        // TODO: Return the access count
        unimplemented!()
    }
}

impl<T: Copy + Default> Default for CachedValue<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

// =============================================================================
// Part 2: RefCell Basics
// =============================================================================

/// A string that can be modified through shared references.
///
/// Uses `RefCell` for interior mutability of the String.
#[derive(Debug, Default)]
pub struct SharedString {
    // TODO: Add a RefCell<String> field named `value`
}

impl SharedString {
    /// Creates a new shared string.
    pub fn new(s: &str) -> Self {
        // TODO: Initialize with RefCell::new(s.to_string())
        unimplemented!()
    }

    /// Gets a clone of the string.
    pub fn get(&self) -> String {
        // TODO: Use borrow() to get an immutable reference, then clone
        unimplemented!()
    }

    /// Sets a new string value.
    pub fn set(&self, s: &str) {
        // TODO: Use borrow_mut() to get a mutable reference, then assign
        unimplemented!()
    }

    /// Appends a string to the current value.
    pub fn append(&self, s: &str) {
        // TODO: Use borrow_mut() and push_str()
        unimplemented!()
    }

    /// Returns the length of the string.
    pub fn len(&self) -> usize {
        // TODO: Use borrow() and len()
        unimplemented!()
    }

    /// Returns true if the string is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Use borrow() and is_empty()
        unimplemented!()
    }
}

/// A vector with interior mutability.
///
/// Uses `RefCell` to allow modification through shared references.
#[derive(Debug, Default)]
pub struct SharedVec<T> {
    // TODO: Add a RefCell<Vec<T>> field named `data`
    _marker: std::marker::PhantomData<T>,
}

impl<T> SharedVec<T> {
    /// Creates a new empty shared vector.
    pub fn new() -> Self {
        // TODO: Initialize with RefCell::new(Vec::new())
        unimplemented!()
    }

    /// Adds an element to the end of the vector.
    pub fn push(&self, value: T) {
        // TODO: Use borrow_mut() and push()
        unimplemented!()
    }

    /// Removes and returns the last element, or None if empty.
    pub fn pop(&self) -> Option<T> {
        // TODO: Use borrow_mut() and pop()
        unimplemented!()
    }

    /// Returns the current number of elements.
    pub fn len(&self) -> usize {
        // TODO: Use borrow() and len()
        unimplemented!()
    }

    /// Returns true if the vector is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Use borrow() and is_empty()
        unimplemented!()
    }
}

impl<T: Clone> SharedVec<T> {
    /// Gets a clone of the element at the specified index.
    pub fn get(&self, index: usize) -> Option<T> {
        // TODO: Use borrow(), get(index), and cloned()
        unimplemented!()
    }
}

// =============================================================================
// Part 3: Safe Borrowing with try_borrow
// =============================================================================

/// A RefCell wrapper that provides safe, non-panicking borrow operations.
///
/// Uses `try_borrow` and `try_borrow_mut` internally to avoid panics.
#[derive(Debug, Default)]
pub struct SafeCell<T> {
    // TODO: Add a RefCell<T> field named `value`
    _marker: std::marker::PhantomData<T>,
}

impl<T> SafeCell<T> {
    /// Creates a new SafeCell.
    pub fn new(value: T) -> Self {
        // TODO: Initialize with RefCell::new(value)
        unimplemented!()
    }

    /// Tries to borrow the value immutably.
    ///
    /// Returns `None` if the value is currently mutably borrowed.
    pub fn try_read(&self) -> Option<Ref<'_, T>> {
        // TODO: Use try_borrow() and convert Result to Option with .ok()
        unimplemented!()
    }

    /// Tries to borrow the value mutably.
    ///
    /// Returns `None` if the value is currently borrowed.
    pub fn try_write(&self) -> Option<RefMut<'_, T>> {
        // TODO: Use try_borrow_mut() and convert Result to Option with .ok()
        unimplemented!()
    }

    /// Checks if the value is currently borrowed.
    ///
    /// Returns `true` if there are any active borrows.
    pub fn is_borrowed(&self) -> bool {
        // TODO: Try to borrow mutably - if it fails, value is borrowed
        unimplemented!()
    }

    /// Applies a function to the value if not currently borrowed.
    ///
    /// Returns `None` if the value cannot be borrowed.
    pub fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        // TODO: Use try_read() and map() to apply the function
        unimplemented!()
    }

    /// Applies a mutating function to the value if not currently borrowed.
    ///
    /// Returns `None` if the value cannot be mutably borrowed.
    pub fn with_value_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        // TODO: Use try_write() and map() to apply the function
        unimplemented!()
    }
}

// =============================================================================
// Part 4: Combining Rc and RefCell
// =============================================================================

/// A reference-counted counter with interior mutability.
///
/// Combines `Rc` for shared ownership with `RefCell` for interior mutability.
#[derive(Debug, Default)]
pub struct SharedCounter {
    // TODO: Add a RefCell<i32> field named `value`
}

impl SharedCounter {
    /// Creates a new shared counter wrapped in Rc.
    pub fn new() -> Rc<Self> {
        // TODO: Create a new SharedCounter and wrap it in Rc
        unimplemented!()
    }

    /// Gets the current value.
    pub fn get(&self) -> i32 {
        // TODO: Use borrow() and dereference
        unimplemented!()
    }

    /// Increments the counter by 1.
    pub fn increment(&self) {
        // TODO: Use borrow_mut() and add 1
        unimplemented!()
    }

    /// Decrements the counter by 1.
    pub fn decrement(&self) {
        // TODO: Use borrow_mut() and subtract 1
        unimplemented!()
    }

    /// Adds n to the counter.
    pub fn add(&self, n: i32) {
        // TODO: Use borrow_mut() and add n
        unimplemented!()
    }
}

/// A tree node with interior mutability for children.
///
/// Combines `Rc` for shared ownership with `RefCell` for mutable children list.
#[derive(Debug)]
pub struct TreeNode<T> {
    // TODO: Add a RefCell<T> field named `value`
    // TODO: Add a RefCell<Vec<Rc<TreeNode<T>>>> field named `children`
    _marker: std::marker::PhantomData<T>,
}

impl<T> TreeNode<T> {
    /// Creates a new tree node wrapped in Rc.
    pub fn new(value: T) -> Rc<Self> {
        // TODO: Create a new TreeNode with the value and empty children, wrapped in Rc
        unimplemented!()
    }

    /// Gets a reference to the node's value.
    pub fn value(&self) -> Ref<'_, T> {
        // TODO: Use borrow() to return Ref<T>
        unimplemented!()
    }

    /// Sets a new value for the node.
    pub fn set_value(&self, value: T) {
        // TODO: Use borrow_mut() and assign new value
        unimplemented!()
    }

    /// Adds a child node.
    pub fn add_child(&self, child: Rc<TreeNode<T>>) {
        // TODO: Use borrow_mut() on children and push the child
        unimplemented!()
    }

    /// Returns the number of children.
    pub fn children_count(&self) -> usize {
        // TODO: Use borrow() on children and return len()
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    // Cell-based Counter
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("Counter: {}", counter.get());

    // CachedValue with access tracking
    let cached = CachedValue::new(42);
    println!(
        "Value: {} (accessed {} times)",
        cached.get(),
        cached.access_count()
    );

    // RefCell-based SharedString
    let shared = SharedString::new("Hello");
    shared.append(", World!");
    println!("String: {}", shared.get());

    // SharedVec
    let vec: SharedVec<i32> = SharedVec::new();
    vec.push(1);
    vec.push(2);
    println!("Vec length: {}", vec.len());

    // SafeCell with try_borrow
    let safe = SafeCell::new(42);
    if let Some(doubled) = safe.with_value(|v| *v * 2) {
        println!("Doubled: {}", doubled);
    }

    // Rc<RefCell> pattern with SharedCounter
    let counter = SharedCounter::new();
    let counter2 = Rc::clone(&counter);
    counter.increment();
    counter2.increment();
    println!("Shared counter: {}", counter.get());

    // Tree with interior mutability
    let root = TreeNode::new("root");
    let child = TreeNode::new("child");
    root.add_child(child);
    println!("Tree children: {}", root.children_count());
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::rc::Rc;

    // =============================================================================
    // Part 1: Counter Tests
    // =============================================================================

    #[test]
    fn test_counter_new() {
        let counter = Counter::new();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_counter_increment() {
        let counter = Counter::new();
        counter.increment();
        assert_eq!(counter.get(), 1);
        counter.increment();
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_counter_decrement() {
        let counter = Counter::new();
        counter.set(5);
        counter.decrement();
        assert_eq!(counter.get(), 4);
        counter.decrement();
        assert_eq!(counter.get(), 3);
    }

    #[test]
    fn test_counter_set() {
        let counter = Counter::new();
        counter.set(100);
        assert_eq!(counter.get(), 100);
        counter.set(-50);
        assert_eq!(counter.get(), -50);
    }

    #[test]
    fn test_counter_negative() {
        let counter = Counter::new();
        counter.decrement();
        assert_eq!(counter.get(), -1);
        counter.decrement();
        assert_eq!(counter.get(), -2);
    }

    #[test]
    fn test_counter_mixed_operations() {
        let counter = Counter::new();
        counter.increment();
        counter.increment();
        counter.decrement();
        counter.increment();
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_counter_default() {
        let counter = Counter::default();
        assert_eq!(counter.get(), 0);
    }

    // =============================================================================
    // Part 1: CachedValue Tests
    // =============================================================================

    #[test]
    fn test_cached_value_new() {
        let cached = CachedValue::new(42);
        assert_eq!(cached.access_count(), 0);
    }

    #[test]
    fn test_cached_value_get() {
        let cached = CachedValue::new(42);
        assert_eq!(cached.get(), 42);
        assert_eq!(cached.access_count(), 1);
    }

    #[test]
    fn test_cached_value_multiple_gets() {
        let cached = CachedValue::new(100);
        for _ in 0..5 {
            cached.get();
        }
        assert_eq!(cached.access_count(), 5);
    }

    #[test]
    fn test_cached_value_set_resets_count() {
        let cached = CachedValue::new(42);
        cached.get();
        cached.get();
        assert_eq!(cached.access_count(), 2);
        cached.set(100);
        assert_eq!(cached.access_count(), 0);
        assert_eq!(cached.get(), 100);
        assert_eq!(cached.access_count(), 1);
    }

    #[test]
    fn test_cached_value_float() {
        let cached = CachedValue::new(3.14f64);
        assert!((cached.get() - 3.14).abs() < 0.001);
        assert_eq!(cached.access_count(), 1);
    }

    #[test]
    fn test_cached_value_bool() {
        let cached = CachedValue::new(true);
        assert!(cached.get());
        cached.set(false);
        assert!(!cached.get());
    }

    #[test]
    fn test_cached_value_char() {
        let cached = CachedValue::new('a');
        assert_eq!(cached.get(), 'a');
        cached.set('z');
        assert_eq!(cached.get(), 'z');
    }

    #[test]
    fn test_cached_value_default() {
        let cached: CachedValue<i32> = CachedValue::default();
        assert_eq!(cached.get(), 0);
    }

    // =============================================================================
    // Part 2: SharedString Tests
    // =============================================================================

    #[test]
    fn test_shared_string_new() {
        let shared = SharedString::new("hello");
        assert_eq!(shared.get(), "hello");
    }

    #[test]
    fn test_shared_string_empty() {
        let shared = SharedString::new("");
        assert!(shared.is_empty());
        assert_eq!(shared.len(), 0);
    }

    #[test]
    fn test_shared_string_set() {
        let shared = SharedString::new("old");
        shared.set("new");
        assert_eq!(shared.get(), "new");
    }

    #[test]
    fn test_shared_string_append() {
        let shared = SharedString::new("Hello");
        shared.append(", World!");
        assert_eq!(shared.get(), "Hello, World!");
    }

    #[test]
    fn test_shared_string_multiple_appends() {
        let shared = SharedString::new("");
        shared.append("a");
        shared.append("b");
        shared.append("c");
        assert_eq!(shared.get(), "abc");
    }

    #[test]
    fn test_shared_string_len() {
        let shared = SharedString::new("hello");
        assert_eq!(shared.len(), 5);
        shared.append(" world");
        assert_eq!(shared.len(), 11);
    }

    #[test]
    fn test_shared_string_is_empty() {
        let shared = SharedString::new("");
        assert!(shared.is_empty());
        shared.append("x");
        assert!(!shared.is_empty());
    }

    #[test]
    fn test_shared_string_unicode() {
        let shared = SharedString::new("hello ");
        shared.append("世界");
        assert_eq!(shared.get(), "hello 世界");
    }

    #[test]
    fn test_shared_string_set_after_append() {
        let shared = SharedString::new("Hello");
        shared.append(" World");
        shared.set("Reset");
        assert_eq!(shared.get(), "Reset");
    }

    #[test]
    fn test_shared_string_default() {
        let shared = SharedString::default();
        assert!(shared.is_empty());
    }

    // =============================================================================
    // Part 2: SharedVec Tests
    // =============================================================================

    #[test]
    fn test_shared_vec_new() {
        let vec: SharedVec<i32> = SharedVec::new();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_shared_vec_push() {
        let vec = SharedVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_shared_vec_pop() {
        let vec = SharedVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_shared_vec_get() {
        let vec = SharedVec::new();
        vec.push(10);
        vec.push(20);
        vec.push(30);
        assert_eq!(vec.get(0), Some(10));
        assert_eq!(vec.get(1), Some(20));
        assert_eq!(vec.get(2), Some(30));
        assert_eq!(vec.get(3), None);
    }

    #[test]
    fn test_shared_vec_is_empty() {
        let vec: SharedVec<i32> = SharedVec::new();
        assert!(vec.is_empty());
        vec.push(1);
        assert!(!vec.is_empty());
        vec.pop();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_shared_vec_strings() {
        let vec = SharedVec::new();
        vec.push(String::from("hello"));
        vec.push(String::from("world"));
        assert_eq!(vec.get(0), Some(String::from("hello")));
        assert_eq!(vec.get(1), Some(String::from("world")));
    }

    #[test]
    fn test_shared_vec_mixed_operations() {
        let vec = SharedVec::new();
        vec.push(1);
        vec.push(2);
        vec.pop();
        vec.push(3);
        vec.push(4);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get(0), Some(1));
        assert_eq!(vec.get(1), Some(3));
        assert_eq!(vec.get(2), Some(4));
    }

    #[test]
    fn test_shared_vec_default() {
        let vec: SharedVec<i32> = SharedVec::default();
        assert!(vec.is_empty());
    }

    // =============================================================================
    // Part 3: SafeCell Tests
    // =============================================================================

    #[test]
    fn test_safe_cell_new() {
        let cell = SafeCell::new(42);
        assert_eq!(*cell.try_read().unwrap(), 42);
    }

    #[test]
    fn test_safe_cell_try_read() {
        let cell = SafeCell::new(100);
        let read = cell.try_read();
        assert!(read.is_some());
        assert_eq!(*read.unwrap(), 100);
    }

    #[test]
    fn test_safe_cell_try_write() {
        let cell = SafeCell::new(42);
        {
            let mut write = cell.try_write().unwrap();
            *write = 100;
        }
        assert_eq!(*cell.try_read().unwrap(), 100);
    }

    #[test]
    fn test_safe_cell_multiple_reads() {
        let cell = SafeCell::new(42);
        let _read1 = cell.try_read();
        let read2 = cell.try_read();
        assert!(read2.is_some());
    }

    #[test]
    fn test_safe_cell_write_fails_during_read() {
        let cell = SafeCell::new(42);
        let _read = cell.try_read().unwrap();
        let write = cell.try_write();
        assert!(write.is_none());
    }

    #[test]
    fn test_safe_cell_read_fails_during_write() {
        let cell = SafeCell::new(42);
        let _write = cell.try_write().unwrap();
        let read = cell.try_read();
        assert!(read.is_none());
    }

    #[test]
    fn test_safe_cell_is_borrowed_false() {
        let cell = SafeCell::new(42);
        assert!(!cell.is_borrowed());
    }

    #[test]
    fn test_safe_cell_is_borrowed_during_read() {
        let cell = SafeCell::new(42);
        let _read = cell.try_read().unwrap();
        assert!(cell.is_borrowed());
    }

    #[test]
    fn test_safe_cell_is_borrowed_during_write() {
        let cell = SafeCell::new(42);
        let _write = cell.try_write().unwrap();
        assert!(cell.is_borrowed());
    }

    #[test]
    fn test_safe_cell_with_value() {
        let cell = SafeCell::new(42);
        let result = cell.with_value(|v| *v * 2);
        assert_eq!(result, Some(84));
    }

    #[test]
    fn test_safe_cell_with_value_string() {
        let cell = SafeCell::new(String::from("hello"));
        let result = cell.with_value(|s| s.len());
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_safe_cell_with_value_mut() {
        let cell = SafeCell::new(42);
        let old = cell.with_value_mut(|v| {
            let old = *v;
            *v = 100;
            old
        });
        assert_eq!(old, Some(42));
        assert_eq!(*cell.try_read().unwrap(), 100);
    }

    #[test]
    fn test_safe_cell_with_value_fails_during_borrow() {
        let cell = SafeCell::new(42);
        let _write = cell.try_write().unwrap();
        let result = cell.with_value(|v| *v);
        assert!(result.is_none());
    }

    #[test]
    fn test_safe_cell_with_value_mut_fails_during_borrow() {
        let cell = SafeCell::new(42);
        let _read = cell.try_read().unwrap();
        let result = cell.with_value_mut(|v| *v = 100);
        assert!(result.is_none());
    }

    #[test]
    fn test_safe_cell_default() {
        let cell: SafeCell<i32> = SafeCell::default();
        assert_eq!(*cell.try_read().unwrap(), 0);
    }

    // =============================================================================
    // Part 4: SharedCounter Tests
    // =============================================================================

    #[test]
    fn test_shared_counter_new() {
        let counter = SharedCounter::new();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_shared_counter_increment() {
        let counter = SharedCounter::new();
        counter.increment();
        assert_eq!(counter.get(), 1);
        counter.increment();
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_shared_counter_decrement() {
        let counter = SharedCounter::new();
        counter.add(5);
        counter.decrement();
        assert_eq!(counter.get(), 4);
    }

    #[test]
    fn test_shared_counter_add() {
        let counter = SharedCounter::new();
        counter.add(100);
        assert_eq!(counter.get(), 100);
        counter.add(-50);
        assert_eq!(counter.get(), 50);
    }

    #[test]
    fn test_shared_counter_shared_ownership() {
        let counter = SharedCounter::new();
        let counter2 = Rc::clone(&counter);
        let counter3 = Rc::clone(&counter);

        counter.increment();
        counter2.increment();
        counter3.increment();

        assert_eq!(counter.get(), 3);
        assert_eq!(counter2.get(), 3);
        assert_eq!(counter3.get(), 3);
    }

    #[test]
    fn test_shared_counter_rc_count() {
        let counter = SharedCounter::new();
        assert_eq!(Rc::strong_count(&counter), 1);

        let counter2 = Rc::clone(&counter);
        assert_eq!(Rc::strong_count(&counter), 2);

        drop(counter2);
        assert_eq!(Rc::strong_count(&counter), 1);
    }

    #[test]
    fn test_shared_counter_negative() {
        let counter = SharedCounter::new();
        counter.decrement();
        counter.decrement();
        assert_eq!(counter.get(), -2);
    }

    #[test]
    fn test_shared_counter_mixed_operations() {
        let counter = SharedCounter::new();
        let c2 = Rc::clone(&counter);

        counter.add(10);
        c2.decrement();
        counter.increment();
        c2.add(-5);

        assert_eq!(counter.get(), 5);
    }

    // =============================================================================
    // Part 4: TreeNode Tests
    // =============================================================================

    #[test]
    fn test_tree_node_new() {
        let node = TreeNode::new(42);
        assert_eq!(*node.value(), 42);
        assert_eq!(node.children_count(), 0);
    }

    #[test]
    fn test_tree_node_string_value() {
        let node = TreeNode::new(String::from("root"));
        assert_eq!(*node.value(), "root");
    }

    #[test]
    fn test_tree_node_set_value() {
        let node = TreeNode::new(42);
        node.set_value(100);
        assert_eq!(*node.value(), 100);
    }

    #[test]
    fn test_tree_node_add_child() {
        let parent = TreeNode::new("parent");
        let child = TreeNode::new("child");
        parent.add_child(child);
        assert_eq!(parent.children_count(), 1);
    }

    #[test]
    fn test_tree_node_multiple_children() {
        let parent = TreeNode::new("parent");
        parent.add_child(TreeNode::new("child1"));
        parent.add_child(TreeNode::new("child2"));
        parent.add_child(TreeNode::new("child3"));
        assert_eq!(parent.children_count(), 3);
    }

    #[test]
    fn test_tree_node_shared_child() {
        let parent1 = TreeNode::new("parent1");
        let parent2 = TreeNode::new("parent2");
        let shared_child = TreeNode::new("shared");

        parent1.add_child(Rc::clone(&shared_child));
        parent2.add_child(Rc::clone(&shared_child));

        assert_eq!(parent1.children_count(), 1);
        assert_eq!(parent2.children_count(), 1);
        assert_eq!(Rc::strong_count(&shared_child), 3); // Original + 2 parents
    }

    #[test]
    fn test_tree_node_nested() {
        let root = TreeNode::new("root");
        let child = TreeNode::new("child");
        let grandchild = TreeNode::new("grandchild");

        child.add_child(grandchild);
        root.add_child(child);

        assert_eq!(root.children_count(), 1);
    }

    #[test]
    fn test_tree_node_modify_after_add() {
        let root = TreeNode::new("root");
        let child = TreeNode::new("original");
        root.add_child(Rc::clone(&child));

        child.set_value("modified");
        assert_eq!(*child.value(), "modified");
    }

    // =============================================================================
    // Integration Tests
    // =============================================================================

    #[test]
    fn test_counter_through_function() {
        fn modify_counter(c: &Counter) {
            c.increment();
            c.increment();
            c.decrement();
        }

        let counter = Counter::new();
        modify_counter(&counter);
        modify_counter(&counter);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_cached_value_in_struct() {
        struct Config {
            value: CachedValue<i32>,
        }

        let config = Config {
            value: CachedValue::new(42),
        };

        fn read_config(c: &Config) -> i32 {
            c.value.get()
        }

        assert_eq!(read_config(&config), 42);
        assert_eq!(read_config(&config), 42);
        assert_eq!(config.value.access_count(), 2);
    }

    #[test]
    fn test_shared_string_builder_pattern() {
        let builder = SharedString::new("");
        builder.append("SELECT ");
        builder.append("* ");
        builder.append("FROM ");
        builder.append("users");
        assert_eq!(builder.get(), "SELECT * FROM users");
    }

    #[test]
    fn test_shared_vec_stack_behavior() {
        let stack = SharedVec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut result = Vec::new();
        while let Some(v) = stack.pop() {
            result.push(v);
        }

        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_safe_cell_transaction_pattern() {
        let cell = SafeCell::new(100);

        // Simulate a transaction
        let result = cell.with_value_mut(|balance| {
            if *balance >= 50 {
                *balance -= 50;
                true
            } else {
                false
            }
        });

        assert_eq!(result, Some(true));
        assert_eq!(*cell.try_read().unwrap(), 50);
    }

    #[test]
    fn test_shared_counter_parallel_style() {
        let counter = SharedCounter::new();
        let handles: Vec<Rc<SharedCounter>> = (0..5).map(|_| Rc::clone(&counter)).collect();

        for handle in &handles {
            handle.increment();
        }

        assert_eq!(counter.get(), 5);
    }

    #[test]
    fn test_tree_node_file_system() {
        // Simulate a file system structure
        let root = TreeNode::new("/");
        let home = TreeNode::new("home");
        let user = TreeNode::new("user");
        let docs = TreeNode::new("documents");
        let pics = TreeNode::new("pictures");

        user.add_child(docs);
        user.add_child(pics);
        home.add_child(user);
        root.add_child(home);

        assert_eq!(root.children_count(), 1);
    }

    #[test]
    fn test_cell_vs_refcell_difference() {
        // Cell: simple get/set for Copy types
        let counter = Counter::new();
        counter.set(42);
        assert_eq!(counter.get(), 42);

        // RefCell: borrowing for non-Copy types
        let shared = SharedString::new("hello");
        shared.append(" world");
        assert_eq!(shared.get(), "hello world");
    }

    #[test]
    fn test_rc_refcell_pattern() {
        // The Rc<RefCell<T>> pattern allows multiple owners to mutate shared data
        let counter = SharedCounter::new();
        let c1 = Rc::clone(&counter);
        let c2 = Rc::clone(&counter);

        // All clones can mutate the same counter
        c1.add(10);
        c2.add(20);
        counter.increment();

        // All clones see the same value
        assert_eq!(c1.get(), 31);
        assert_eq!(c2.get(), 31);
        assert_eq!(counter.get(), 31);
    }

    #[test]
    fn test_safe_cell_concurrent_access_simulation() {
        let cell = SafeCell::new(0);

        // Simulate checking before modifying
        if !cell.is_borrowed() {
            let _ = cell.with_value_mut(|v| *v += 1);
        }

        assert_eq!(*cell.try_read().unwrap(), 1);
    }

    #[test]
    fn test_tree_modification_through_shared_ref() {
        let tree = TreeNode::new("root");
        let child = TreeNode::new("child");

        fn add_child_to_tree<T>(tree: &TreeNode<T>, child: Rc<TreeNode<T>>) {
            tree.add_child(child);
        }

        add_child_to_tree(&tree, child);
        assert_eq!(tree.children_count(), 1);
    }

    #[test]
    fn test_counter_immutable_interface() {
        // Even though we have &Counter, we can modify it
        fn use_counter(c: &Counter) {
            c.increment();
        }

        let counter = Counter::new();
        use_counter(&counter);
        use_counter(&counter);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_shared_vec_with_complex_type() {
        #[derive(Clone, PartialEq, Debug)]
        struct Item {
            id: i32,
            name: String,
        }

        let vec = SharedVec::new();
        vec.push(Item {
            id: 1,
            name: "first".to_string(),
        });
        vec.push(Item {
            id: 2,
            name: "second".to_string(),
        });

        assert_eq!(
            vec.get(0),
            Some(Item {
                id: 1,
                name: "first".to_string()
            })
        );
    }

    #[test]
    fn test_cached_value_reset_workflow() {
        let cached = CachedValue::new(42);

        // Access multiple times
        for _ in 0..10 {
            cached.get();
        }
        assert_eq!(cached.access_count(), 10);

        // Update resets count
        cached.set(100);
        assert_eq!(cached.access_count(), 0);
        assert_eq!(cached.get(), 100);
        assert_eq!(cached.access_count(), 1);
    }

    #[test]
    fn test_safe_cell_guard_lifetime() {
        let cell = SafeCell::new(vec![1, 2, 3]);

        // Guard keeps the borrow alive
        {
            let guard = cell.try_read().unwrap();
            assert_eq!(guard.len(), 3);
            // guard dropped here
        }

        // Now we can write
        let result = cell.with_value_mut(|v| {
            v.push(4);
            v.len()
        });
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_tree_diamond_pattern() {
        // Diamond pattern: two parents share the same child
        let shared_child = TreeNode::new("shared");
        let parent_a = TreeNode::new("A");
        let parent_b = TreeNode::new("B");

        parent_a.add_child(Rc::clone(&shared_child));
        parent_b.add_child(Rc::clone(&shared_child));

        // Modifying shared child is visible from both parents
        shared_child.set_value("modified");

        assert_eq!(*shared_child.value(), "modified");
        assert_eq!(Rc::strong_count(&shared_child), 3);
    }
}
