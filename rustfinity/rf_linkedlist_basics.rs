//! # Rustfinity: Linkedlist Basics
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/linkedlist-basics
//!
//! `LinkedList` is Rust's implementation of a doubly-linked list. Unlike `Vec`, which stores elements contiguously in memory, a linked list stores each element in a separate node that contains pointers to the previous and next nodes. This makes insertions and removals at both ends O(1) operations, but random access becomes O(n) since you must traverse the list to reach a specific element.
//!
//! In practice, `LinkedList` is rarely the best choice in Rust because `Vec` and `VecDeque` are more cache-friendly and often faster due to better memory locality. However, `LinkedList` shines in specific scenarios: when you need to frequently insert or remove elements in the middle of the list (if you already have a cursor at that position), when you need to split or merge lists efficiently, or when implementing certain algorithms like LRU caches where you need to move elements to the front or back without shifting other elements.
//!
//! The `LinkedList` API includes operations at both ends: `push_front()` and `push_back()` for adding elements, `pop_front()` and `pop_back()` for removing elements. It also supports efficient list manipulation through `append()` to concatenate lists and `split_off()` to divide a list at a given index.
//!
//! ```rust
//! use std::collections::LinkedList;
//!
//! let mut list = LinkedList::new();
//!
//! // Add elements at both ends
//! list.push_back(1);
//! list.push_back(2);
//! list.push_front(0);
//! // List is now: [0, 1, 2]
//!
//! // Remove from both ends
//! assert_eq!(list.pop_front(), Some(0));
//! assert_eq!(list.pop_back(), Some(2));
//! // List is now: [1]
//!
//! // Peek without removing
//! assert_eq!(list.front(), Some(&1));
//! assert_eq!(list.back(), Some(&1));
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions that demonstrate various `LinkedList` operations:
//!
//! 1. `create_list(items: &[i32]) -> LinkedList<i32>` - Create a linked list from a slice, maintaining the order of elements.
//!
//! 2. `add_front<T>(list: &mut LinkedList<T>, item: T)` - Add an element to the front of the list.
//!
//! 3. `add_back<T>(list: &mut LinkedList<T>, item: T)` - Add an element to the back of the list.
//!
//! 4. `remove_front<T>(list: &mut LinkedList<T>) -> Option<T>` - Remove and return the front element.
//!
//! 5. `remove_back<T>(list: &mut LinkedList<T>) -> Option<T>` - Remove and return the back element.
//!
//! 6. `peek_front<T>(list: &LinkedList<T>) -> Option<&T>` - Return a reference to the front element without removing it.
//!
//! 7. `peek_back<T>(list: &LinkedList<T>) -> Option<&T>` - Return a reference to the back element without removing it.
//!
//! 8. `move_to_front<T: PartialEq>(list: &mut LinkedList<T>, value: &T) -> bool` - Find an element by value and move it to the front. Returns `true` if the element was found and moved, `false` otherwise. This is a key operation for LRU cache behavior.
//!
//! 9. `concat_lists<T>(list1: LinkedList<T>, list2: LinkedList<T>) -> LinkedList<T>` - Concatenate two lists, with list2's elements appended after list1's.
//!
//! ## Examples
//!
//! ```rust
//! use std::collections::LinkedList;
//!
//! // create_list
//! let list = create_list(&[1, 2, 3]);
//! assert_eq!(list.front(), Some(&1));
//! assert_eq!(list.back(), Some(&3));
//! assert_eq!(list.len(), 3);
//!
//! // add_front / add_back
//! let mut list = LinkedList::new();
//! add_back(&mut list, 2);
//! add_front(&mut list, 1);
//! add_back(&mut list, 3);
//! // List is: [1, 2, 3]
//!
//! // remove_front / remove_back
//! let mut list = create_list(&[1, 2, 3]);
//! assert_eq!(remove_front(&mut list), Some(1));
//! assert_eq!(remove_back(&mut list), Some(3));
//! // List is: [2]
//!
//! // peek_front / peek_back
//! let list = create_list(&[10, 20, 30]);
//! assert_eq!(peek_front(&list), Some(&10));
//! assert_eq!(peek_back(&list), Some(&30));
//!
//! // move_to_front (LRU-style operation)
//! let mut list = create_list(&[1, 2, 3, 4, 5]);
//! // Move 3 to front
//! assert!(move_to_front(&mut list, &3));
//! // List is now: [3, 1, 2, 4, 5]
//! // 99 not found
//! assert!(!move_to_front(&mut list, &99));
//!
//! // concat_lists
//! let list1 = create_list(&[1, 2]);
//! let list2 = create_list(&[3, 4]);
//! let combined = concat_lists(list1, list2);
//! // combined is: [1, 2, 3, 4]
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - `LinkedList::new()` creates an empty list
//! - Use `.extend()` or iterate and push to create a list from a slice
//! - `push_front()` and `push_back()` add to the respective ends
//! - `pop_front()` and `pop_back()` remove from the respective ends
//! - `front()` and `back()` return `Option<&T>` for peeking
//! - For `move_to_front`, you'll need to remove the element and re-add it at the front
//! - Since LinkedList doesn't have random access, you may need to rebuild it to remove an element from the middle
//! - `append()` drains one list into another - be careful about ownership
//!
//! </details>
//!
//! Run: `cargo run --example rf_linkedlist_basics`
//! Test: `cargo test --example rf_linkedlist_basics`
//! Filter: `cargo test --example rf_linkedlist_basics <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::collections::LinkedList;

/// Creates a linked list from a slice, maintaining the order of elements.
pub fn create_list(items: &[i32]) -> LinkedList<i32> {
    // TODO: Create a LinkedList containing all items in order
    todo!()
}

/// Adds an element to the front of the list.
pub fn add_front<T>(list: &mut LinkedList<T>, item: T) {
    // TODO: Add item to the front of the list
    todo!()
}

/// Adds an element to the back of the list.
pub fn add_back<T>(list: &mut LinkedList<T>, item: T) {
    // TODO: Add item to the back of the list
    todo!()
}

/// Removes and returns the front element.
pub fn remove_front<T>(list: &mut LinkedList<T>) -> Option<T> {
    // TODO: Remove and return the front element
    todo!()
}

/// Removes and returns the back element.
pub fn remove_back<T>(list: &mut LinkedList<T>) -> Option<T> {
    // TODO: Remove and return the back element
    todo!()
}

/// Returns a reference to the front element without removing it.
pub fn peek_front<T>(list: &LinkedList<T>) -> Option<&T> {
    // TODO: Return a reference to the front element
    todo!()
}

/// Returns a reference to the back element without removing it.
pub fn peek_back<T>(list: &LinkedList<T>) -> Option<&T> {
    // TODO: Return a reference to the back element
    todo!()
}

/// Finds an element by value and moves it to the front.
/// Returns true if found and moved, false if not found.
pub fn move_to_front<T: PartialEq>(list: &mut LinkedList<T>, value: &T) -> bool {
    // TODO: Find the element matching value and move it to the front
    todo!()
}

/// Concatenates two lists, with list2's elements appended after list1's.
pub fn concat_lists<T>(list1: LinkedList<T>, list2: LinkedList<T>) -> LinkedList<T> {
    // TODO: Combine list1 and list2, with list2's elements after list1's
    todo!()
}

fn main() {
    // Create a list
    let list = create_list(&[1, 2, 3, 4, 5]);
    println!("Created list with {} elements", list.len());
    println!("Front: {:?}, Back: {:?}", list.front(), list.back());

    // Add elements at both ends
    let mut list = LinkedList::new();
    add_back(&mut list, 2);
    add_front(&mut list, 1);
    add_back(&mut list, 3);
    println!(
        "After adding 1 at front, 2 and 3 at back: {:?}",
        list.iter().collect::<Vec<_>>()
    );

    // Remove elements
    let mut list = create_list(&[10, 20, 30]);
    println!("Removed from front: {:?}", remove_front(&mut list));
    println!("Removed from back: {:?}", remove_back(&mut list));
    println!("Remaining: {:?}", list.iter().collect::<Vec<_>>());

    // LRU-style move to front
    let mut list = create_list(&[1, 2, 3, 4, 5]);
    println!(
        "Before move_to_front(&3): {:?}",
        list.iter().collect::<Vec<_>>()
    );
    move_to_front(&mut list, &3);
    println!(
        "After move_to_front(&3): {:?}",
        list.iter().collect::<Vec<_>>()
    );

    // Concatenate lists
    let list1 = create_list(&[1, 2]);
    let list2 = create_list(&[3, 4]);
    let combined = concat_lists(list1, list2);
    println!("Combined list: {:?}", combined.iter().collect::<Vec<_>>());
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::collections::LinkedList;

    // ==================== create_list tests ====================

    #[test]
    fn test_create_list_basic() {
        let list = create_list(&[1, 2, 3]);
        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));
    }

    #[test]
    fn test_create_list_empty() {
        let list = create_list(&[]);
        assert!(list.is_empty());
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
    }

    #[test]
    fn test_create_list_single() {
        let list = create_list(&[42]);
        assert_eq!(list.len(), 1);
        assert_eq!(list.front(), Some(&42));
        assert_eq!(list.back(), Some(&42));
    }

    #[test]
    fn test_create_list_maintains_order() {
        let list = create_list(&[5, 4, 3, 2, 1]);
        let vec: Vec<i32> = list.into_iter().collect();
        assert_eq!(vec, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_create_list_negative() {
        let list = create_list(&[-5, -1, -10, -3]);
        assert_eq!(list.front(), Some(&-5));
        assert_eq!(list.back(), Some(&-3));
    }

    #[test]
    fn test_create_list_duplicates() {
        let list = create_list(&[1, 1, 2, 2, 3, 3]);
        assert_eq!(list.len(), 6);
        let vec: Vec<i32> = list.into_iter().collect();
        assert_eq!(vec, vec![1, 1, 2, 2, 3, 3]);
    }

    // ==================== add_front tests ====================

    #[test]
    fn test_add_front_to_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        add_front(&mut list, 1);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_add_front_multiple() {
        let mut list: LinkedList<i32> = LinkedList::new();
        add_front(&mut list, 3);
        add_front(&mut list, 2);
        add_front(&mut list, 1);
        let vec: Vec<i32> = list.into_iter().collect();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_add_front_string() {
        let mut list: LinkedList<String> = LinkedList::new();
        add_front(&mut list, "world".to_string());
        add_front(&mut list, "hello".to_string());
        assert_eq!(list.front(), Some(&"hello".to_string()));
    }

    // ==================== add_back tests ====================

    #[test]
    fn test_add_back_to_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        add_back(&mut list, 1);
        assert_eq!(list.back(), Some(&1));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_add_back_multiple() {
        let mut list: LinkedList<i32> = LinkedList::new();
        add_back(&mut list, 1);
        add_back(&mut list, 2);
        add_back(&mut list, 3);
        let vec: Vec<i32> = list.into_iter().collect();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_add_back_string() {
        let mut list: LinkedList<String> = LinkedList::new();
        add_back(&mut list, "hello".to_string());
        add_back(&mut list, "world".to_string());
        assert_eq!(list.back(), Some(&"world".to_string()));
    }

    // ==================== remove_front tests ====================

    #[test]
    fn test_remove_front_basic() {
        let mut list = create_list(&[1, 2, 3]);
        assert_eq!(remove_front(&mut list), Some(1));
        assert_eq!(remove_front(&mut list), Some(2));
        assert_eq!(remove_front(&mut list), Some(3));
        assert_eq!(remove_front(&mut list), None);
    }

    #[test]
    fn test_remove_front_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(remove_front(&mut list), None);
    }

    #[test]
    fn test_remove_front_single() {
        let mut list = create_list(&[42]);
        assert_eq!(remove_front(&mut list), Some(42));
        assert!(list.is_empty());
    }

    #[test]
    fn test_remove_front_generic() {
        let mut list: LinkedList<&str> = LinkedList::new();
        add_back(&mut list, "a");
        add_back(&mut list, "b");
        assert_eq!(remove_front(&mut list), Some("a"));
        assert_eq!(remove_front(&mut list), Some("b"));
    }

    // ==================== remove_back tests ====================

    #[test]
    fn test_remove_back_basic() {
        let mut list = create_list(&[1, 2, 3]);
        assert_eq!(remove_back(&mut list), Some(3));
        assert_eq!(remove_back(&mut list), Some(2));
        assert_eq!(remove_back(&mut list), Some(1));
        assert_eq!(remove_back(&mut list), None);
    }

    #[test]
    fn test_remove_back_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(remove_back(&mut list), None);
    }

    #[test]
    fn test_remove_back_single() {
        let mut list = create_list(&[42]);
        assert_eq!(remove_back(&mut list), Some(42));
        assert!(list.is_empty());
    }

    #[test]
    fn test_remove_back_generic() {
        let mut list: LinkedList<&str> = LinkedList::new();
        add_back(&mut list, "a");
        add_back(&mut list, "b");
        assert_eq!(remove_back(&mut list), Some("b"));
        assert_eq!(remove_back(&mut list), Some("a"));
    }

    // ==================== peek_front tests ====================

    #[test]
    fn test_peek_front_basic() {
        let list = create_list(&[10, 20, 30]);
        assert_eq!(peek_front(&list), Some(&10));
        assert_eq!(list.len(), 3); // Unchanged
    }

    #[test]
    fn test_peek_front_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(peek_front(&list), None);
    }

    #[test]
    fn test_peek_front_single() {
        let list = create_list(&[42]);
        assert_eq!(peek_front(&list), Some(&42));
    }

    #[test]
    fn test_peek_front_multiple_calls() {
        let list = create_list(&[1, 2, 3]);
        assert_eq!(peek_front(&list), Some(&1));
        assert_eq!(peek_front(&list), Some(&1));
        assert_eq!(peek_front(&list), Some(&1));
        assert_eq!(list.len(), 3);
    }

    // ==================== peek_back tests ====================

    #[test]
    fn test_peek_back_basic() {
        let list = create_list(&[10, 20, 30]);
        assert_eq!(peek_back(&list), Some(&30));
        assert_eq!(list.len(), 3); // Unchanged
    }

    #[test]
    fn test_peek_back_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(peek_back(&list), None);
    }

    #[test]
    fn test_peek_back_single() {
        let list = create_list(&[42]);
        assert_eq!(peek_back(&list), Some(&42));
    }

    #[test]
    fn test_peek_back_multiple_calls() {
        let list = create_list(&[1, 2, 3]);
        assert_eq!(peek_back(&list), Some(&3));
        assert_eq!(peek_back(&list), Some(&3));
        assert_eq!(peek_back(&list), Some(&3));
        assert_eq!(list.len(), 3);
    }

    // ==================== move_to_front tests ====================

    #[test]
    fn test_move_to_front_middle_element() {
        let mut list = create_list(&[1, 2, 3, 4, 5]);
        assert!(move_to_front(&mut list, &3));
        let vec: Vec<i32> = list.into_iter().collect();
        assert_eq!(vec, vec![3, 1, 2, 4, 5]);
    }

    #[test]
    fn test_move_to_front_last_element() {
        let mut list = create_list(&[1, 2, 3, 4, 5]);
        assert!(move_to_front(&mut list, &5));
        let vec: Vec<i32> = list.into_iter().collect();
        assert_eq!(vec, vec![5, 1, 2, 3, 4]);
    }

    #[test]
    fn test_move_to_front_first_element() {
        let mut list = create_list(&[1, 2, 3, 4, 5]);
        assert!(move_to_front(&mut list, &1));
        let vec: Vec<i32> = list.into_iter().collect();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]); // Should stay the same
    }

    #[test]
    fn test_move_to_front_not_found() {
        let mut list = create_list(&[1, 2, 3, 4, 5]);
        assert!(!move_to_front(&mut list, &99));
        let vec: Vec<i32> = list.into_iter().collect();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]); // Unchanged
    }

    #[test]
    fn test_move_to_front_empty_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert!(!move_to_front(&mut list, &1));
        assert!(list.is_empty());
    }

    #[test]
    fn test_move_to_front_single_element() {
        let mut list = create_list(&[42]);
        assert!(move_to_front(&mut list, &42));
        let vec: Vec<i32> = list.into_iter().collect();
        assert_eq!(vec, vec![42]);
    }

    #[test]
    fn test_move_to_front_duplicates_moves_first() {
        let mut list = create_list(&[1, 2, 3, 2, 4]);
        assert!(move_to_front(&mut list, &2));
        let vec: Vec<i32> = list.into_iter().collect();
        // Should move the first occurrence of 2
        assert_eq!(vec, vec![2, 1, 3, 2, 4]);
    }

    #[test]
    fn test_move_to_front_string() {
        let mut list: LinkedList<String> = LinkedList::new();
        add_back(&mut list, "a".to_string());
        add_back(&mut list, "b".to_string());
        add_back(&mut list, "c".to_string());

        assert!(move_to_front(&mut list, &"b".to_string()));
        let vec: Vec<String> = list.into_iter().collect();
        assert_eq!(vec, vec!["b", "a", "c"]);
    }

    // ==================== concat_lists tests ====================

    #[test]
    fn test_concat_lists_basic() {
        let list1 = create_list(&[1, 2]);
        let list2 = create_list(&[3, 4]);
        let combined = concat_lists(list1, list2);
        let vec: Vec<i32> = combined.into_iter().collect();
        assert_eq!(vec, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_concat_lists_empty_first() {
        let list1: LinkedList<i32> = LinkedList::new();
        let list2 = create_list(&[1, 2, 3]);
        let combined = concat_lists(list1, list2);
        let vec: Vec<i32> = combined.into_iter().collect();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_concat_lists_empty_second() {
        let list1 = create_list(&[1, 2, 3]);
        let list2: LinkedList<i32> = LinkedList::new();
        let combined = concat_lists(list1, list2);
        let vec: Vec<i32> = combined.into_iter().collect();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_concat_lists_both_empty() {
        let list1: LinkedList<i32> = LinkedList::new();
        let list2: LinkedList<i32> = LinkedList::new();
        let combined = concat_lists(list1, list2);
        assert!(combined.is_empty());
    }

    #[test]
    fn test_concat_lists_single_elements() {
        let list1 = create_list(&[1]);
        let list2 = create_list(&[2]);
        let combined = concat_lists(list1, list2);
        let vec: Vec<i32> = combined.into_iter().collect();
        assert_eq!(vec, vec![1, 2]);
    }

    #[test]
    fn test_concat_lists_string() {
        let mut list1: LinkedList<String> = LinkedList::new();
        list1.push_back("hello".to_string());
        let mut list2: LinkedList<String> = LinkedList::new();
        list2.push_back("world".to_string());

        let combined = concat_lists(list1, list2);
        let vec: Vec<String> = combined.into_iter().collect();
        assert_eq!(vec, vec!["hello", "world"]);
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_integration_lru_cache_simulation() {
        // Simulate LRU cache behavior: most recently used items at front
        let mut cache = create_list(&[1, 2, 3, 4, 5]);

        // Access item 3 - move it to front
        move_to_front(&mut cache, &3);
        assert_eq!(cache.front(), Some(&3));

        // Access item 5 - move it to front
        move_to_front(&mut cache, &5);
        assert_eq!(cache.front(), Some(&5));

        // Access item 1 - move it to front
        move_to_front(&mut cache, &1);
        assert_eq!(cache.front(), Some(&1));

        // Final order should be: [1, 5, 3, 2, 4]
        let vec: Vec<i32> = cache.into_iter().collect();
        assert_eq!(vec, vec![1, 5, 3, 2, 4]);
    }

    #[test]
    fn test_integration_queue_operations() {
        // Use linked list as a queue (FIFO)
        let mut queue: LinkedList<&str> = LinkedList::new();

        // Enqueue
        add_back(&mut queue, "first");
        add_back(&mut queue, "second");
        add_back(&mut queue, "third");

        // Dequeue in FIFO order
        assert_eq!(remove_front(&mut queue), Some("first"));
        assert_eq!(remove_front(&mut queue), Some("second"));
        assert_eq!(remove_front(&mut queue), Some("third"));
        assert_eq!(remove_front(&mut queue), None);
    }

    #[test]
    fn test_integration_stack_operations() {
        // Use linked list as a stack (LIFO)
        let mut stack: LinkedList<i32> = LinkedList::new();

        // Push
        add_front(&mut stack, 1);
        add_front(&mut stack, 2);
        add_front(&mut stack, 3);

        // Pop in LIFO order
        assert_eq!(remove_front(&mut stack), Some(3));
        assert_eq!(remove_front(&mut stack), Some(2));
        assert_eq!(remove_front(&mut stack), Some(1));
        assert_eq!(remove_front(&mut stack), None);
    }

    #[test]
    fn test_integration_deque_operations() {
        // Use linked list as a double-ended queue
        let mut deque = create_list(&[2, 3, 4]);

        // Add at both ends
        add_front(&mut deque, 1);
        add_back(&mut deque, 5);

        // Check both ends
        assert_eq!(peek_front(&deque), Some(&1));
        assert_eq!(peek_back(&deque), Some(&5));

        // Remove from both ends
        assert_eq!(remove_front(&mut deque), Some(1));
        assert_eq!(remove_back(&mut deque), Some(5));

        // What remains
        let vec: Vec<i32> = deque.into_iter().collect();
        assert_eq!(vec, vec![2, 3, 4]);
    }

    #[test]
    fn test_integration_merge_multiple_lists() {
        let list1 = create_list(&[1, 2]);
        let list2 = create_list(&[3, 4]);
        let list3 = create_list(&[5, 6]);

        let merged = concat_lists(concat_lists(list1, list2), list3);
        let vec: Vec<i32> = merged.into_iter().collect();
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_integration_alternating_operations() {
        let mut list: LinkedList<i32> = LinkedList::new();

        // Interleave add and remove operations
        add_back(&mut list, 1);
        add_back(&mut list, 2);
        assert_eq!(remove_front(&mut list), Some(1));
        add_back(&mut list, 3);
        add_front(&mut list, 0);
        assert_eq!(remove_back(&mut list), Some(3));

        // Remaining: [0, 2]
        let vec: Vec<i32> = list.into_iter().collect();
        assert_eq!(vec, vec![0, 2]);
    }
}
