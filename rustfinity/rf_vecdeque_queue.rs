//! # Rustfinity: Vecdeque Queue
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/vecdeque-queue
//!
//! `VecDeque<T>` is Rust's implementation of a double-ended queue (deque) backed by a growable ring buffer. Unlike `Vec`, which only provides efficient operations at the back, `VecDeque` allows O(1) amortized insertion and removal at both ends. This makes it ideal for implementing queues, task schedulers, and sliding window algorithms.
//!
//! The `VecDeque` type is found in `std::collections` and provides methods like `push_front()`, `push_back()`, `pop_front()`, and `pop_back()` for double-ended operations. It also supports indexed access, iteration, and rotation operations.
//!
//! One key advantage of `VecDeque` over `Vec` is when you need FIFO (first-in-first-out) queue behavior. With `Vec`, removing from the front requires shifting all elements (O(n)), but `VecDeque` does this in O(1) time. This makes `VecDeque` the go-to choice for task queues, breadth-first search, and any algorithm that needs to efficiently add and remove from both ends.
//!
//! ```rust
//! use std::collections::VecDeque;
//!
//! let mut queue: VecDeque<&str> = VecDeque::new();
//!
//! // Add to the back (like a queue)
//! queue.push_back("first");
//! queue.push_back("second");
//! queue.push_back("third");
//!
//! // Remove from the front (FIFO order)
//! assert_eq!(queue.pop_front(), Some("first"));
//! assert_eq!(queue.pop_front(), Some("second"));
//!
//! // Can also add to front and remove from back
//! queue.push_front("new_first");
//! assert_eq!(queue.pop_back(), Some("third"));
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions for working with `VecDeque`:
//!
//! 1. `create_queue(items: &[T]) -> VecDeque<T>` - Create a VecDeque from a slice, preserving order (first element of slice is front of queue)
//!
//! 2. `enqueue(queue: &mut VecDeque<T>, item: T)` - Add an item to the back of the queue
//!
//! 3. `dequeue(queue: &mut VecDeque<T>) -> Option<T>` - Remove and return the item from the front of the queue
//!
//! 4. `peek_front(queue: &VecDeque<T>) -> Option<&T>` - Return a reference to the front item without removing it
//!
//! 5. `peek_back(queue: &VecDeque<T>) -> Option<&T>` - Return a reference to the back item without removing it
//!
//! 6. `rotate_left(queue: &mut VecDeque<T>, n: usize)` - Rotate the queue so the front n elements move to the back
//!
//! 7. `rotate_right(queue: &mut VecDeque<T>, n: usize)` - Rotate the queue so the back n elements move to the front
//!
//! ## Examples
//!
//! ```rust
//! use std::collections::VecDeque;
//!
//! // create_queue
//! let queue = create_queue(&[1, 2, 3]);
//! assert_eq!(queue.front(), Some(&1));
//! assert_eq!(queue.back(), Some(&3));
//!
//! // enqueue and dequeue (FIFO behavior)
//! let mut queue = create_queue(&["first", "second"]);
//! enqueue(&mut queue, "third");
//! assert_eq!(
//!     dequeue(&mut queue),
//!     Some("first")
//! );
//! assert_eq!(
//!     dequeue(&mut queue),
//!     Some("second")
//! );
//! assert_eq!(
//!     dequeue(&mut queue),
//!     Some("third")
//! );
//!
//! // peek_front and peek_back
//! let queue = create_queue(&[10, 20, 30]);
//! assert_eq!(peek_front(&queue), Some(&10));
//! assert_eq!(peek_back(&queue), Some(&30));
//!
//! // rotate_left - front elements move to back
//! let mut queue = create_queue(&[1, 2, 3, 4, 5]);
//! rotate_left(&mut queue, 2);
//! // Now: [3, 4, 5, 1, 2]
//! assert_eq!(queue.front(), Some(&3));
//!
//! // rotate_right - back elements move to front
//! let mut queue = create_queue(&[1, 2, 3, 4, 5]);
//! rotate_right(&mut queue, 2);
//! // Now: [4, 5, 1, 2, 3]
//! assert_eq!(queue.front(), Some(&4));
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `create_queue`, you can use `.iter().cloned().collect()` to collect from a slice into a VecDeque
//! - For `enqueue`, use the `.push_back()` method
//! - For `dequeue`, use the `.pop_front()` method which returns `Option<T>`
//! - For `peek_front` and `peek_back`, use `.front()` and `.back()` methods
//! - For `rotate_left`, use the `.rotate_left()` method on VecDeque - but be careful with n larger than the queue length!
//! - For `rotate_right`, use the `.rotate_right()` method - also handle edge cases with n
//! - When n is 0 or the queue is empty, rotation should be a no-op
//! - The Clone bound on T is needed for `create_queue` to clone elements from the slice
//!
//! </details>
//!
//! Run: `cargo run --example rf_vecdeque_queue`
//! Test: `cargo test --example rf_vecdeque_queue`
//! Filter: `cargo test --example rf_vecdeque_queue <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::collections::VecDeque;

/// Creates a VecDeque from a slice, preserving order.
///
/// The first element of the slice becomes the front of the queue.
pub fn create_queue<T: Clone>(items: &[T]) -> VecDeque<T> {
    // TODO: Iterate over items and collect into a VecDeque
    unimplemented!()
}

/// Adds an item to the back of the queue.
pub fn enqueue<T>(queue: &mut VecDeque<T>, item: T) {
    // TODO: Use push_back to add the item
    unimplemented!()
}

/// Removes and returns the item from the front of the queue.
///
/// Returns `Some(item)` if the queue is not empty, `None` otherwise.
pub fn dequeue<T>(queue: &mut VecDeque<T>) -> Option<T> {
    // TODO: Use pop_front to remove and return the front item
    unimplemented!()
}

/// Returns a reference to the front item without removing it.
pub fn peek_front<T>(queue: &VecDeque<T>) -> Option<&T> {
    // TODO: Use the front() method
    unimplemented!()
}

/// Returns a reference to the back item without removing it.
pub fn peek_back<T>(queue: &VecDeque<T>) -> Option<&T> {
    // TODO: Use the back() method
    unimplemented!()
}

/// Rotates the queue left so the front n elements move to the back.
///
/// After rotation, the element at index n becomes the new front.
/// Handle edge cases: empty queue, n == 0, n >= len.
pub fn rotate_left<T>(queue: &mut VecDeque<T>, n: usize) {
    // TODO: Handle edge cases, then use rotate_left method
    // Remember: n might be larger than queue length!
    unimplemented!()
}

/// Rotates the queue right so the back n elements move to the front.
///
/// After rotation, the element at index (len - n) becomes the new front.
/// Handle edge cases: empty queue, n == 0, n >= len.
pub fn rotate_right<T>(queue: &mut VecDeque<T>, n: usize) {
    // TODO: Handle edge cases, then use rotate_right method
    // Remember: n might be larger than queue length!
    unimplemented!()
}

// Example usage
pub fn main() {
    // Create a task queue
    let mut tasks = create_queue(&["task1", "task2", "task3"]);
    println!("Initial queue: {:?}", tasks);

    // Add a new task
    enqueue(&mut tasks, "task4");
    println!("After enqueue: {:?}", tasks);

    // Process tasks (FIFO order)
    while let Some(task) = dequeue(&mut tasks) {
        println!("Processing: {}", task);
    }

    // Demonstrate rotation
    let mut numbers = create_queue(&[1, 2, 3, 4, 5]);
    println!("\nBefore rotation: {:?}", numbers);

    rotate_left(&mut numbers, 2);
    println!("After rotate_left(2): {:?}", numbers);

    rotate_right(&mut numbers, 2);
    println!("After rotate_right(2): {:?}", numbers);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::collections::VecDeque;
    // ============================================================================
    // Tests for create_queue
    // ============================================================================

    #[test]
    fn test_create_queue_basic() {
        let queue = create_queue(&[1, 2, 3]);
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.front(), Some(&1));
        assert_eq!(queue.back(), Some(&3));
    }

    #[test]
    fn test_create_queue_empty() {
        let queue: VecDeque<i32> = create_queue(&[]);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_create_queue_single() {
        let queue = create_queue(&["only"]);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.front(), Some(&"only"));
        assert_eq!(queue.back(), Some(&"only"));
    }

    #[test]
    fn test_create_queue_strings() {
        let queue = create_queue(&["first".to_string(), "second".to_string()]);
        assert_eq!(queue.front(), Some(&"first".to_string()));
        assert_eq!(queue.back(), Some(&"second".to_string()));
    }

    #[test]
    fn test_create_queue_preserves_order() {
        let items: Vec<i32> = (0..10).collect();
        let queue = create_queue(&items);
        for (i, item) in queue.iter().enumerate() {
            assert_eq!(*item, i as i32);
        }
    }

    // ============================================================================
    // Tests for enqueue
    // ============================================================================

    #[test]
    fn test_enqueue_basic() {
        let mut queue = create_queue(&[1, 2]);
        enqueue(&mut queue, 3);
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.back(), Some(&3));
    }

    #[test]
    fn test_enqueue_to_empty() {
        let mut queue: VecDeque<i32> = VecDeque::new();
        enqueue(&mut queue, 42);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.front(), Some(&42));
        assert_eq!(queue.back(), Some(&42));
    }

    #[test]
    fn test_enqueue_multiple() {
        let mut queue: VecDeque<char> = VecDeque::new();
        enqueue(&mut queue, 'a');
        enqueue(&mut queue, 'b');
        enqueue(&mut queue, 'c');
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.front(), Some(&'a'));
        assert_eq!(queue.back(), Some(&'c'));
    }

    #[test]
    fn test_enqueue_strings() {
        let mut queue = create_queue(&["first".to_string()]);
        enqueue(&mut queue, "second".to_string());
        assert_eq!(queue.back(), Some(&"second".to_string()));
    }

    // ============================================================================
    // Tests for dequeue
    // ============================================================================

    #[test]
    fn test_dequeue_basic() {
        let mut queue = create_queue(&[1, 2, 3]);
        assert_eq!(dequeue(&mut queue), Some(1));
        assert_eq!(dequeue(&mut queue), Some(2));
        assert_eq!(dequeue(&mut queue), Some(3));
        assert_eq!(dequeue(&mut queue), None);
    }

    #[test]
    fn test_dequeue_empty() {
        let mut queue: VecDeque<i32> = VecDeque::new();
        assert_eq!(dequeue(&mut queue), None);
    }

    #[test]
    fn test_dequeue_single() {
        let mut queue = create_queue(&["only"]);
        assert_eq!(dequeue(&mut queue), Some("only"));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_dequeue_fifo_order() {
        let mut queue = create_queue(&["first", "second", "third"]);
        enqueue(&mut queue, "fourth");

        assert_eq!(dequeue(&mut queue), Some("first"));
        assert_eq!(dequeue(&mut queue), Some("second"));
        assert_eq!(dequeue(&mut queue), Some("third"));
        assert_eq!(dequeue(&mut queue), Some("fourth"));
    }

    #[test]
    fn test_dequeue_interleaved() {
        let mut queue: VecDeque<i32> = VecDeque::new();
        enqueue(&mut queue, 1);
        enqueue(&mut queue, 2);
        assert_eq!(dequeue(&mut queue), Some(1));
        enqueue(&mut queue, 3);
        assert_eq!(dequeue(&mut queue), Some(2));
        assert_eq!(dequeue(&mut queue), Some(3));
    }

    // ============================================================================
    // Tests for peek_front
    // ============================================================================

    #[test]
    fn test_peek_front_basic() {
        let queue = create_queue(&[10, 20, 30]);
        assert_eq!(peek_front(&queue), Some(&10));
        assert_eq!(queue.len(), 3); // Still has all elements
    }

    #[test]
    fn test_peek_front_empty() {
        let queue: VecDeque<i32> = VecDeque::new();
        assert_eq!(peek_front(&queue), None);
    }

    #[test]
    fn test_peek_front_single() {
        let queue = create_queue(&[42]);
        assert_eq!(peek_front(&queue), Some(&42));
    }

    #[test]
    fn test_peek_front_after_dequeue() {
        let mut queue = create_queue(&[1, 2, 3]);
        dequeue(&mut queue);
        assert_eq!(peek_front(&queue), Some(&2));
    }

    #[test]
    fn test_peek_front_does_not_modify() {
        let queue = create_queue(&["first", "second"]);
        let _ = peek_front(&queue);
        let _ = peek_front(&queue);
        let _ = peek_front(&queue);
        assert_eq!(queue.len(), 2);
        assert_eq!(peek_front(&queue), Some(&"first"));
    }

    // ============================================================================
    // Tests for peek_back
    // ============================================================================

    #[test]
    fn test_peek_back_basic() {
        let queue = create_queue(&[10, 20, 30]);
        assert_eq!(peek_back(&queue), Some(&30));
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_peek_back_empty() {
        let queue: VecDeque<i32> = VecDeque::new();
        assert_eq!(peek_back(&queue), None);
    }

    #[test]
    fn test_peek_back_single() {
        let queue = create_queue(&[42]);
        assert_eq!(peek_back(&queue), Some(&42));
    }

    #[test]
    fn test_peek_back_after_enqueue() {
        let mut queue = create_queue(&[1, 2]);
        enqueue(&mut queue, 3);
        assert_eq!(peek_back(&queue), Some(&3));
    }

    #[test]
    fn test_peek_front_and_back_same_single() {
        let queue = create_queue(&[99]);
        assert_eq!(peek_front(&queue), peek_back(&queue));
        assert_eq!(peek_front(&queue), Some(&99));
    }

    // ============================================================================
    // Tests for rotate_left
    // ============================================================================

    #[test]
    fn test_rotate_left_basic() {
        let mut queue = create_queue(&[1, 2, 3, 4, 5]);
        rotate_left(&mut queue, 2);
        // [1, 2, 3, 4, 5] -> [3, 4, 5, 1, 2]
        let expected: VecDeque<i32> = VecDeque::from([3, 4, 5, 1, 2]);
        assert_eq!(queue, expected);
    }

    #[test]
    fn test_rotate_left_zero() {
        let mut queue = create_queue(&[1, 2, 3]);
        rotate_left(&mut queue, 0);
        let expected: VecDeque<i32> = VecDeque::from([1, 2, 3]);
        assert_eq!(queue, expected);
    }

    #[test]
    fn test_rotate_left_empty() {
        let mut queue: VecDeque<i32> = VecDeque::new();
        rotate_left(&mut queue, 5); // Should not panic
        assert!(queue.is_empty());
    }

    #[test]
    fn test_rotate_left_single() {
        let mut queue = create_queue(&[42]);
        rotate_left(&mut queue, 1);
        assert_eq!(queue.front(), Some(&42));
    }

    #[test]
    fn test_rotate_left_full_rotation() {
        let mut queue = create_queue(&[1, 2, 3]);
        rotate_left(&mut queue, 3); // Full rotation = no change
        let expected: VecDeque<i32> = VecDeque::from([1, 2, 3]);
        assert_eq!(queue, expected);
    }

    #[test]
    fn test_rotate_left_more_than_len() {
        let mut queue = create_queue(&[1, 2, 3, 4]);
        rotate_left(&mut queue, 6); // 6 % 4 = 2
        // [1, 2, 3, 4] -> [3, 4, 1, 2]
        let expected: VecDeque<i32> = VecDeque::from([3, 4, 1, 2]);
        assert_eq!(queue, expected);
    }

    #[test]
    fn test_rotate_left_one() {
        let mut queue = create_queue(&[1, 2, 3, 4]);
        rotate_left(&mut queue, 1);
        let expected: VecDeque<i32> = VecDeque::from([2, 3, 4, 1]);
        assert_eq!(queue, expected);
    }

    // ============================================================================
    // Tests for rotate_right
    // ============================================================================

    #[test]
    fn test_rotate_right_basic() {
        let mut queue = create_queue(&[1, 2, 3, 4, 5]);
        rotate_right(&mut queue, 2);
        // [1, 2, 3, 4, 5] -> [4, 5, 1, 2, 3]
        let expected: VecDeque<i32> = VecDeque::from([4, 5, 1, 2, 3]);
        assert_eq!(queue, expected);
    }

    #[test]
    fn test_rotate_right_zero() {
        let mut queue = create_queue(&[1, 2, 3]);
        rotate_right(&mut queue, 0);
        let expected: VecDeque<i32> = VecDeque::from([1, 2, 3]);
        assert_eq!(queue, expected);
    }

    #[test]
    fn test_rotate_right_empty() {
        let mut queue: VecDeque<i32> = VecDeque::new();
        rotate_right(&mut queue, 5); // Should not panic
        assert!(queue.is_empty());
    }

    #[test]
    fn test_rotate_right_single() {
        let mut queue = create_queue(&[42]);
        rotate_right(&mut queue, 1);
        assert_eq!(queue.front(), Some(&42));
    }

    #[test]
    fn test_rotate_right_full_rotation() {
        let mut queue = create_queue(&[1, 2, 3]);
        rotate_right(&mut queue, 3); // Full rotation = no change
        let expected: VecDeque<i32> = VecDeque::from([1, 2, 3]);
        assert_eq!(queue, expected);
    }

    #[test]
    fn test_rotate_right_more_than_len() {
        let mut queue = create_queue(&[1, 2, 3, 4]);
        rotate_right(&mut queue, 6); // 6 % 4 = 2
        // [1, 2, 3, 4] -> [3, 4, 1, 2]
        let expected: VecDeque<i32> = VecDeque::from([3, 4, 1, 2]);
        assert_eq!(queue, expected);
    }

    #[test]
    fn test_rotate_right_one() {
        let mut queue = create_queue(&[1, 2, 3, 4]);
        rotate_right(&mut queue, 1);
        let expected: VecDeque<i32> = VecDeque::from([4, 1, 2, 3]);
        assert_eq!(queue, expected);
    }

    // ============================================================================
    // Tests for rotation inverses
    // ============================================================================

    #[test]
    fn test_rotate_left_then_right_inverse() {
        let mut queue = create_queue(&[1, 2, 3, 4, 5]);
        rotate_left(&mut queue, 2);
        rotate_right(&mut queue, 2);
        // Should be back to original
        let expected: VecDeque<i32> = VecDeque::from([1, 2, 3, 4, 5]);
        assert_eq!(queue, expected);
    }

    #[test]
    fn test_rotate_right_then_left_inverse() {
        let mut queue = create_queue(&[1, 2, 3, 4, 5]);
        rotate_right(&mut queue, 3);
        rotate_left(&mut queue, 3);
        // Should be back to original
        let expected: VecDeque<i32> = VecDeque::from([1, 2, 3, 4, 5]);
        assert_eq!(queue, expected);
    }

    // ============================================================================
    // Integration tests
    // ============================================================================

    #[test]
    fn test_task_queue_workflow() {
        let mut tasks: VecDeque<&str> = VecDeque::new();

        // Add tasks
        enqueue(&mut tasks, "download");
        enqueue(&mut tasks, "process");
        enqueue(&mut tasks, "upload");

        // Check front task without removing
        assert_eq!(peek_front(&tasks), Some(&"download"));

        // Process tasks in order
        assert_eq!(dequeue(&mut tasks), Some("download"));
        assert_eq!(dequeue(&mut tasks), Some("process"));

        // Add more tasks
        enqueue(&mut tasks, "cleanup");

        // Continue processing
        assert_eq!(dequeue(&mut tasks), Some("upload"));
        assert_eq!(dequeue(&mut tasks), Some("cleanup"));
        assert_eq!(dequeue(&mut tasks), None);
    }

    #[test]
    fn test_sliding_window_simulation() {
        // Simulate a sliding window of size 3
        let mut window = create_queue(&[1, 2, 3]);

        // Slide: remove front, add to back
        dequeue(&mut window);
        enqueue(&mut window, 4);
        assert_eq!(peek_front(&window), Some(&2));
        assert_eq!(peek_back(&window), Some(&4));

        dequeue(&mut window);
        enqueue(&mut window, 5);
        assert_eq!(peek_front(&window), Some(&3));
        assert_eq!(peek_back(&window), Some(&5));
    }

    #[test]
    fn test_round_robin_scheduling() {
        let mut processes = create_queue(&["P1", "P2", "P3"]);

        // Round robin: move front to back
        rotate_left(&mut processes, 1);
        assert_eq!(peek_front(&processes), Some(&"P2"));

        rotate_left(&mut processes, 1);
        assert_eq!(peek_front(&processes), Some(&"P3"));

        rotate_left(&mut processes, 1);
        assert_eq!(peek_front(&processes), Some(&"P1"));
    }

    #[test]
    fn test_with_complex_type() {
        #[derive(Debug, Clone, PartialEq)]
        struct Task {
            id: u32,
            name: String,
        }

        let tasks = vec![
            Task {
                id: 1,
                name: "first".to_string(),
            },
            Task {
                id: 2,
                name: "second".to_string(),
            },
        ];
        let mut queue = create_queue(&tasks);

        enqueue(
            &mut queue,
            Task {
                id: 3,
                name: "third".to_string(),
            },
        );

        assert_eq!(peek_front(&queue).unwrap().id, 1);
        assert_eq!(peek_back(&queue).unwrap().id, 3);

        let dequeued = dequeue(&mut queue).unwrap();
        assert_eq!(dequeued.name, "first");
    }

    #[test]
    fn test_large_queue() {
        let items: Vec<i32> = (0..1000).collect();
        let mut queue = create_queue(&items);

        assert_eq!(queue.len(), 1000);
        assert_eq!(peek_front(&queue), Some(&0));
        assert_eq!(peek_back(&queue), Some(&999));

        // Dequeue first 100
        for i in 0..100 {
            assert_eq!(dequeue(&mut queue), Some(i));
        }

        assert_eq!(queue.len(), 900);
        assert_eq!(peek_front(&queue), Some(&100));

        // Rotate
        rotate_left(&mut queue, 50);
        assert_eq!(peek_front(&queue), Some(&150));
    }
}
