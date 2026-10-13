//! # Rustfinity: Box Heap Allocation
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/box-heap-allocation
//!
//! # Box and Heap Allocation
//!
//! In Rust, most values are stored on the stack by default. The stack is fast and efficient, but it has limitations: all values must have a known, fixed size at compile time, and the stack has limited space. When you need to store data with a size unknown at compile time, or when you have large data that you don't want to copy, you need heap allocation.
//!
//! `Box<T>` is Rust's simplest smart pointer for heap allocation. When you create a `Box<T>`, the value of type `T` is allocated on the heap, and the `Box` itself (which is just a pointer) lives on the stack. When the `Box` goes out of scope, both the pointer and the heap data are deallocated automatically thanks to Rust's ownership system.
//!
//! Common use cases for `Box<T>` include:
//!
//! - Storing large data without copying it when transferring ownership
//! - Enabling recursive types (types that contain themselves)
//! - Trait objects when you need dynamic dispatch (`Box<dyn Trait>`)
//! - Transferring ownership of data without copying it
//!
//! ## Your Task
//!
//! Implement the following types and functions that demonstrate heap allocation with `Box<T>`:
//!
//! 1. **`boxed_value<T>(value: T) -> Box<T>`** - A generic function that takes any value and returns it boxed on the heap.
//!
//! 2. **`unbox<T>(boxed: Box<T>) -> T`** - A generic function that takes a boxed value and returns the inner value, moving it out of the box.
//!
//! 3. **`List` enum** - A recursive linked list type:
//!    - `Nil` - represents the end of the list
//!    - `Cons(i32, Box<List>)` - a node containing a value and a pointer to the next node
//!
//! 4. **`List::new() -> List`** - Creates an empty list (Nil).
//!
//! 5. **`List::prepend(self, value: i32) -> List`** - Adds a value to the front of the list, returning a new list.
//!
//! 6. **`List::len(&self) -> usize`** - Returns the number of elements in the list.
//!
//! 7. **`List::sum(&self) -> i32`** - Returns the sum of all elements in the list.
//!
//! 8. **`List::to_vec(&self) -> Vec<i32>`** - Converts the list to a Vec, preserving order.
//!
//! 9. **`LargeData` struct** - A struct with a large array `data: [u8; 1000]` to demonstrate boxing large data.
//!
//! 10. **`box_large_data(data: LargeData) -> Box<LargeData>`** - Boxes a large data structure.
//!
//! 11. **`modify_boxed<T, F>(boxed: &mut Box<T>, f: F)`** - Takes a mutable reference to a boxed value and applies a function to modify the inner value. `F: FnOnce(&mut T)`.
//!
//! ## Examples
//!
//! ```rust
//! // Boxing and unboxing
//! let boxed = boxed_value(42);
//! assert_eq!(*boxed, 42);
//!
//! let value = unbox(boxed);
//! assert_eq!(value, 42);
//!
//! // Recursive list
//! let list = List::new()
//!     .prepend(3)
//!     .prepend(2)
//!     .prepend(1);
//!
//! assert_eq!(list.len(), 3);
//! assert_eq!(list.sum(), 6);
//! assert_eq!(list.to_vec(), vec![1, 2, 3]);
//!
//! // Boxing large data
//! let large = LargeData { data: [0; 1000] };
//! let boxed_large = box_large_data(large);
//! assert_eq!(boxed_large.data[0], 0);
//!
//! // Modifying boxed values
//! let mut boxed_num = Box::new(10);
//! modify_boxed(&mut boxed_num, |n| *n += 5);
//! assert_eq!(*boxed_num, 15);
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use `Box::new(value)` to create a boxed value
//! - The `*` operator dereferences a `Box` to access the inner value
//! - For recursive types like `List`, you must use `Box` to give the type a known size
//! - The `List::prepend` method should create a new `Cons` node with the current list boxed as the tail
//! - For `modify_boxed`, use `&mut **boxed` or `boxed.as_mut()` to get a mutable reference to the inner value
//! - Remember that `Box<T>` implements `Deref` and `DerefMut`, so you can often use it like a regular reference
//!
//! </details>
//!
//! Run: `cargo run --example rf_box_heap_allocation`
//! Test: `cargo test --example rf_box_heap_allocation`
//! Filter: `cargo test --example rf_box_heap_allocation <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

/// Creates a boxed value on the heap.
///
/// Takes any value and allocates it on the heap, returning a `Box<T>`.
pub fn boxed_value<T>(value: T) -> Box<T> {
    // TODO: Use Box::new to allocate the value on the heap
    unimplemented!()
}

/// Unboxes a value, returning the inner value.
///
/// Takes ownership of a `Box<T>` and returns the inner value.
pub fn unbox<T>(boxed: Box<T>) -> T {
    // TODO: Dereference the box to get the inner value
    unimplemented!()
}

/// A recursive linked list demonstrating heap allocation for recursive types.
///
/// Without `Box`, this type would have infinite size because `List` contains
/// `List` directly. Using `Box` gives it a known size (the size of a pointer).
#[derive(Debug, Clone, PartialEq)]
pub enum List {
    /// End of the list
    Nil,
    /// A node containing a value and pointer to the next node
    Cons(i32, Box<List>),
}

impl List {
    /// Creates an empty list.
    pub fn new() -> List {
        // TODO: Return an empty list (Nil)
        unimplemented!()
    }

    /// Adds a value to the front of the list.
    ///
    /// Consumes the list and returns a new list with the value prepended.
    pub fn prepend(self, value: i32) -> List {
        // TODO: Create a Cons node with the value and box the current list as the tail
        unimplemented!()
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        // TODO: Recursively count elements
        unimplemented!()
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Check if this is Nil
        unimplemented!()
    }

    /// Returns the sum of all elements in the list.
    pub fn sum(&self) -> i32 {
        // TODO: Recursively sum elements
        unimplemented!()
    }

    /// Converts the list to a Vec, preserving order.
    pub fn to_vec(&self) -> Vec<i32> {
        // TODO: Convert the list to a vector
        unimplemented!()
    }
}

impl Default for List {
    fn default() -> Self {
        List::new()
    }
}

/// A large data structure demonstrating the benefits of heap allocation.
///
/// Stack space is limited, so large arrays should be boxed to avoid stack overflow.
#[derive(Debug, Clone, PartialEq)]
pub struct LargeData {
    pub data: [u8; 1000],
}

impl LargeData {
    /// Creates new LargeData filled with zeros.
    pub fn new() -> Self {
        LargeData { data: [0; 1000] }
    }

    /// Creates new LargeData filled with the given value.
    pub fn filled(value: u8) -> Self {
        LargeData {
            data: [value; 1000],
        }
    }
}

impl Default for LargeData {
    fn default() -> Self {
        LargeData::new()
    }
}

/// Boxes a large data structure.
///
/// This is useful when you want to transfer ownership of large data without
/// copying it on the stack.
pub fn box_large_data(data: LargeData) -> Box<LargeData> {
    // TODO: Box the large data structure
    unimplemented!()
}

/// Modifies a boxed value in place using a closure.
///
/// This demonstrates that `Box<T>` implements `DerefMut`, allowing
/// mutable access to the inner value.
pub fn modify_boxed<T, F>(boxed: &mut Box<T>, f: F)
where
    F: FnOnce(&mut T),
{
    // TODO: Apply the closure to the inner value
    unimplemented!()
}

// Example usage
pub fn main() {
    // Boxing and unboxing
    let boxed = boxed_value(42);
    println!("Boxed value: {}", *boxed);

    let value = unbox(boxed);
    println!("Unboxed value: {}", value);

    // Recursive list
    let list = List::new().prepend(3).prepend(2).prepend(1);
    println!("List length: {}", list.len());
    println!("List sum: {}", list.sum());
    println!("List as vec: {:?}", list.to_vec());

    // Boxing large data
    let large = LargeData::filled(42);
    let boxed_large = box_large_data(large);
    println!("First byte of large data: {}", boxed_large.data[0]);

    // Modifying boxed values
    let mut boxed_num = Box::new(10);
    modify_boxed(&mut boxed_num, |n| *n += 5);
    println!("Modified boxed value: {}", *boxed_num);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ==================== boxed_value tests ====================

    #[test]
    fn boxed_value_integer() {
        let boxed = boxed_value(42);
        assert_eq!(*boxed, 42);
    }

    #[test]
    fn boxed_value_zero() {
        let boxed = boxed_value(0);
        assert_eq!(*boxed, 0);
    }

    #[test]
    fn boxed_value_negative() {
        let boxed = boxed_value(-100);
        assert_eq!(*boxed, -100);
    }

    #[test]
    fn boxed_value_string() {
        let boxed = boxed_value(String::from("hello"));
        assert_eq!(&*boxed, "hello");
    }

    #[test]
    fn boxed_value_empty_string() {
        let boxed = boxed_value(String::new());
        assert_eq!(&*boxed, "");
    }

    #[test]
    fn boxed_value_float() {
        let boxed: Box<f64> = boxed_value(3.14);
        assert!((*boxed - 3.14).abs() < f64::EPSILON);
    }

    #[test]
    fn boxed_value_bool() {
        let boxed_true = boxed_value(true);
        let boxed_false = boxed_value(false);
        assert!(*boxed_true);
        assert!(!*boxed_false);
    }

    #[test]
    fn boxed_value_tuple() {
        let boxed = boxed_value((1, "two", 3.0));
        assert_eq!(*boxed, (1, "two", 3.0));
    }

    #[test]
    fn boxed_value_vec() {
        let boxed = boxed_value(vec![1, 2, 3]);
        assert_eq!(*boxed, vec![1, 2, 3]);
    }

    #[test]
    fn boxed_value_option() {
        let some = boxed_value(Some(42));
        let none: Box<Option<i32>> = boxed_value(None);
        assert_eq!(*some, Some(42));
        assert_eq!(*none, None);
    }

    // ==================== unbox tests ====================

    #[test]
    fn unbox_integer() {
        let boxed = Box::new(42);
        assert_eq!(unbox(boxed), 42);
    }

    #[test]
    fn unbox_zero() {
        let boxed = Box::new(0);
        assert_eq!(unbox(boxed), 0);
    }

    #[test]
    fn unbox_string() {
        let boxed = Box::new(String::from("hello"));
        assert_eq!(unbox(boxed), "hello");
    }

    #[test]
    fn unbox_vec() {
        let boxed = Box::new(vec![1, 2, 3]);
        assert_eq!(unbox(boxed), vec![1, 2, 3]);
    }

    #[test]
    fn unbox_nested_box() {
        let inner = Box::new(42);
        let outer = Box::new(inner);
        let unboxed_outer = unbox(outer);
        let unboxed_inner = unbox(unboxed_outer);
        assert_eq!(unboxed_inner, 42);
    }

    #[test]
    fn boxed_then_unbox_roundtrip() {
        let original = 12345;
        let boxed = boxed_value(original);
        let unboxed = unbox(boxed);
        assert_eq!(unboxed, original);
    }

    // ==================== List::new tests ====================

    #[test]
    fn list_new_empty() {
        let list = List::new();
        assert!(list.is_empty());
    }

    #[test]
    fn list_new_len_zero() {
        let list = List::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn list_new_sum_zero() {
        let list = List::new();
        assert_eq!(list.sum(), 0);
    }

    #[test]
    fn list_new_to_vec_empty() {
        let list = List::new();
        assert_eq!(list.to_vec(), Vec::<i32>::new());
    }

    #[test]
    fn list_default() {
        let list = List::default();
        assert!(list.is_empty());
    }

    // ==================== List::prepend tests ====================

    #[test]
    fn list_prepend_single() {
        let list = List::new().prepend(1);
        assert_eq!(list.len(), 1);
        assert_eq!(list.to_vec(), vec![1]);
    }

    #[test]
    fn list_prepend_multiple() {
        let list = List::new().prepend(3).prepend(2).prepend(1);
        assert_eq!(list.len(), 3);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn list_prepend_zero() {
        let list = List::new().prepend(0);
        assert_eq!(list.len(), 1);
        assert_eq!(list.to_vec(), vec![0]);
    }

    #[test]
    fn list_prepend_negative() {
        let list = List::new().prepend(-3).prepend(-2).prepend(-1);
        assert_eq!(list.to_vec(), vec![-1, -2, -3]);
    }

    #[test]
    fn list_prepend_mixed() {
        let list = List::new().prepend(-1).prepend(0).prepend(1);
        assert_eq!(list.to_vec(), vec![1, 0, -1]);
    }

    #[test]
    fn list_prepend_duplicates() {
        let list = List::new().prepend(1).prepend(1).prepend(1);
        assert_eq!(list.to_vec(), vec![1, 1, 1]);
    }

    // ==================== List::len tests ====================

    #[test]
    fn list_len_zero() {
        assert_eq!(List::new().len(), 0);
    }

    #[test]
    fn list_len_one() {
        assert_eq!(List::new().prepend(1).len(), 1);
    }

    #[test]
    fn list_len_many() {
        let list = List::new()
            .prepend(1)
            .prepend(2)
            .prepend(3)
            .prepend(4)
            .prepend(5);
        assert_eq!(list.len(), 5);
    }

    // ==================== List::is_empty tests ====================

    #[test]
    fn list_is_empty_true() {
        assert!(List::new().is_empty());
    }

    #[test]
    fn list_is_empty_false() {
        assert!(!List::new().prepend(1).is_empty());
    }

    // ==================== List::sum tests ====================

    #[test]
    fn list_sum_empty() {
        assert_eq!(List::new().sum(), 0);
    }

    #[test]
    fn list_sum_single() {
        assert_eq!(List::new().prepend(42).sum(), 42);
    }

    #[test]
    fn list_sum_multiple() {
        let list = List::new().prepend(3).prepend(2).prepend(1);
        assert_eq!(list.sum(), 6);
    }

    #[test]
    fn list_sum_negative() {
        let list = List::new().prepend(-3).prepend(-2).prepend(-1);
        assert_eq!(list.sum(), -6);
    }

    #[test]
    fn list_sum_mixed() {
        let list = List::new().prepend(-5).prepend(10).prepend(-5);
        assert_eq!(list.sum(), 0);
    }

    #[test]
    fn list_sum_zeros() {
        let list = List::new().prepend(0).prepend(0).prepend(0);
        assert_eq!(list.sum(), 0);
    }

    #[test]
    fn list_sum_large() {
        let list = List::new()
            .prepend(1000)
            .prepend(2000)
            .prepend(3000)
            .prepend(4000);
        assert_eq!(list.sum(), 10000);
    }

    // ==================== List::to_vec tests ====================

    #[test]
    fn list_to_vec_empty() {
        assert_eq!(List::new().to_vec(), Vec::<i32>::new());
    }

    #[test]
    fn list_to_vec_single() {
        assert_eq!(List::new().prepend(42).to_vec(), vec![42]);
    }

    #[test]
    fn list_to_vec_preserves_order() {
        let list = List::new().prepend(3).prepend(2).prepend(1);
        // Prepending 1 last means it's at the front
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn list_to_vec_with_negatives() {
        let list = List::new().prepend(3).prepend(-2).prepend(1);
        assert_eq!(list.to_vec(), vec![1, -2, 3]);
    }

    // ==================== List clone and equality tests ====================

    #[test]
    fn list_clone() {
        let list = List::new().prepend(3).prepend(2).prepend(1);
        let cloned = list.clone();
        assert_eq!(list.to_vec(), cloned.to_vec());
    }

    #[test]
    fn list_eq() {
        let list1 = List::new().prepend(2).prepend(1);
        let list2 = List::new().prepend(2).prepend(1);
        assert_eq!(list1, list2);
    }

    #[test]
    fn list_ne() {
        let list1 = List::new().prepend(1);
        let list2 = List::new().prepend(2);
        assert_ne!(list1, list2);
    }

    // ==================== LargeData tests ====================

    #[test]
    fn large_data_new() {
        let data = LargeData::new();
        assert_eq!(data.data[0], 0);
        assert_eq!(data.data[999], 0);
    }

    #[test]
    fn large_data_filled() {
        let data = LargeData::filled(42);
        assert_eq!(data.data[0], 42);
        assert_eq!(data.data[500], 42);
        assert_eq!(data.data[999], 42);
    }

    #[test]
    fn large_data_default() {
        let data = LargeData::default();
        assert_eq!(data.data[0], 0);
    }

    #[test]
    fn large_data_size() {
        let data = LargeData::new();
        assert_eq!(data.data.len(), 1000);
    }

    // ==================== box_large_data tests ====================

    #[test]
    fn box_large_data_basic() {
        let large = LargeData::new();
        let boxed = box_large_data(large);
        assert_eq!(boxed.data[0], 0);
    }

    #[test]
    fn box_large_data_filled() {
        let large = LargeData::filled(255);
        let boxed = box_large_data(large);
        assert_eq!(boxed.data[0], 255);
        assert_eq!(boxed.data[999], 255);
    }

    #[test]
    fn box_large_data_modify_via_deref() {
        let large = LargeData::new();
        let mut boxed = box_large_data(large);
        boxed.data[0] = 100;
        assert_eq!(boxed.data[0], 100);
    }

    #[test]
    fn box_large_data_clone() {
        let large = LargeData::filled(42);
        let boxed = box_large_data(large);
        let cloned = boxed.clone();
        assert_eq!(boxed.data, cloned.data);
    }

    // ==================== modify_boxed tests ====================

    #[test]
    fn modify_boxed_integer_add() {
        let mut boxed = Box::new(10);
        modify_boxed(&mut boxed, |n| *n += 5);
        assert_eq!(*boxed, 15);
    }

    #[test]
    fn modify_boxed_integer_multiply() {
        let mut boxed = Box::new(7);
        modify_boxed(&mut boxed, |n| *n *= 3);
        assert_eq!(*boxed, 21);
    }

    #[test]
    fn modify_boxed_integer_set() {
        let mut boxed = Box::new(0);
        modify_boxed(&mut boxed, |n| *n = 100);
        assert_eq!(*boxed, 100);
    }

    #[test]
    fn modify_boxed_string_push() {
        let mut boxed = Box::new(String::from("hello"));
        modify_boxed(&mut boxed, |s| s.push_str(" world"));
        assert_eq!(&*boxed, "hello world");
    }

    #[test]
    fn modify_boxed_string_clear() {
        let mut boxed = Box::new(String::from("hello"));
        modify_boxed(&mut boxed, |s| s.clear());
        assert_eq!(&*boxed, "");
    }

    #[test]
    fn modify_boxed_vec_push() {
        let mut boxed = Box::new(vec![1, 2, 3]);
        modify_boxed(&mut boxed, |v| v.push(4));
        assert_eq!(*boxed, vec![1, 2, 3, 4]);
    }

    #[test]
    fn modify_boxed_vec_clear() {
        let mut boxed = Box::new(vec![1, 2, 3]);
        modify_boxed(&mut boxed, |v| v.clear());
        assert!(boxed.is_empty());
    }

    #[test]
    fn modify_boxed_vec_sort() {
        let mut boxed = Box::new(vec![3, 1, 2]);
        modify_boxed(&mut boxed, |v| v.sort());
        assert_eq!(*boxed, vec![1, 2, 3]);
    }

    #[test]
    fn modify_boxed_struct() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        let mut boxed = Box::new(Point { x: 0, y: 0 });
        modify_boxed(&mut boxed, |p| {
            p.x = 10;
            p.y = 20;
        });
        assert_eq!(*boxed, Point { x: 10, y: 20 });
    }

    #[test]
    fn modify_boxed_large_data() {
        let large = LargeData::new();
        let mut boxed = box_large_data(large);
        modify_boxed(&mut boxed, |d| {
            d.data[0] = 1;
            d.data[999] = 2;
        });
        assert_eq!(boxed.data[0], 1);
        assert_eq!(boxed.data[999], 2);
    }

    // ==================== Integration tests ====================

    #[test]
    fn integration_box_unbox_list() {
        let list = List::new().prepend(3).prepend(2).prepend(1);
        let boxed = boxed_value(list);
        assert_eq!(boxed.len(), 3);
        let unboxed = unbox(boxed);
        assert_eq!(unboxed.sum(), 6);
    }

    #[test]
    fn integration_nested_boxes() {
        let inner = boxed_value(42);
        let outer = boxed_value(inner);
        assert_eq!(**outer, 42);
    }

    #[test]
    fn integration_box_and_modify() {
        let mut boxed = boxed_value(vec![1, 2, 3]);
        modify_boxed(&mut boxed, |v| {
            v.push(4);
            v.push(5);
        });
        assert_eq!(*boxed, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn integration_list_workflow() {
        // Build a list
        let list = List::new()
            .prepend(5)
            .prepend(4)
            .prepend(3)
            .prepend(2)
            .prepend(1);

        // Check properties
        assert!(!list.is_empty());
        assert_eq!(list.len(), 5);
        assert_eq!(list.sum(), 15);
        assert_eq!(list.to_vec(), vec![1, 2, 3, 4, 5]);

        // Clone and verify independence
        let cloned = list.clone();
        assert_eq!(list, cloned);
    }

    #[test]
    fn integration_large_data_workflow() {
        // Create and box large data
        let large = LargeData::filled(10);
        let mut boxed = box_large_data(large);

        // Modify via closure
        modify_boxed(&mut boxed, |d| {
            for i in 0..100 {
                d.data[i] = 20;
            }
        });

        // Verify
        assert_eq!(boxed.data[0], 20);
        assert_eq!(boxed.data[99], 20);
        assert_eq!(boxed.data[100], 10);
    }

    #[test]
    fn integration_generic_types() {
        // Test boxed_value with various types
        let boxed_int = boxed_value(42i32);
        let boxed_float = boxed_value(3.14f64);
        let boxed_string = boxed_value(String::from("test"));
        let boxed_option: Box<Option<i32>> = boxed_value(Some(1));

        assert_eq!(unbox(boxed_int), 42);
        assert!((unbox(boxed_float) - 3.14).abs() < f64::EPSILON);
        assert_eq!(unbox(boxed_string), "test");
        assert_eq!(unbox(boxed_option), Some(1));
    }

    #[test]
    fn integration_recursive_list_deep() {
        // Create a longer list to test recursion
        let mut list = List::new();
        for i in (1..=10).rev() {
            list = list.prepend(i);
        }
        assert_eq!(list.len(), 10);
        assert_eq!(list.sum(), 55); // 1+2+...+10 = 55
        assert_eq!(list.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn integration_modify_multiple_times() {
        let mut boxed = Box::new(0);
        for i in 1..=5 {
            modify_boxed(&mut boxed, |n| *n += i);
        }
        assert_eq!(*boxed, 15); // 1+2+3+4+5
    }

    #[test]
    fn integration_box_of_boxes() {
        // Box containing a Box containing a value
        let inner = Box::new(42);
        let middle = Box::new(inner);
        let outer = Box::new(middle);

        assert_eq!(***outer, 42);
    }
}
