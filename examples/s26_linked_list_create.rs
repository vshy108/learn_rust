// Demonstrates creating and using a linked list with more operations.
//
// Extends the linked list with iteration, Display, and From<Vec<T>>.
//
// Run: cargo run --example s26_linked_list_create

use std::fmt;

/// A singly linked list.
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

/// A node in the linked list.
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// Creates a new empty list.
    fn new() -> Self {
        LinkedList { head: None }
    }

    /// Pushes to the front (O(1)).
    fn push_front(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    /// Pushes to the back (O(n) — must traverse to end).
    fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });
        // Walk to the last node
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(new_node);
    }

    /// Returns an iterator over references.
    fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.as_deref(),
        }
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    /// Creates a linked list from a Vec (preserves order).
    fn from(vec: Vec<T>) -> Self {
        let mut list = LinkedList::new();
        // Push in reverse so order is preserved
        for item in vec.into_iter().rev() {
            list.push_front(item);
        }
        list
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        let mut current = &self.head;
        while let Some(node) = current {
            if !first {
                write!(f, " -> ")?;
            }
            write!(f, "{}", node.value)?;
            first = false;
            current = &node.next;
        }
        write!(f, " -> Nil")
    }
}

/// An iterator over linked list references.
struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}

fn main() {
    println!("=== Creating a LinkedList ===");

    // --- 1. Build with push_front ---
    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    println!("push_front: {list}");

    // --- 2. Build with push_back ---
    let mut list2 = LinkedList::new();
    list2.push_back(1);
    list2.push_back(2);
    list2.push_back(3);
    println!("push_back:  {list2}");

    // --- 3. From Vec ---
    let list3 = LinkedList::from(vec![10, 20, 30]);
    println!("from vec:   {list3}");

    // --- 4. Iterate ---
    let sum: i32 = list.iter().sum();
    println!("sum: {sum}");

    let doubled: Vec<i32> = list.iter().map(|x| x * 2).collect();
    println!("doubled: {doubled:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back_order() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let vals: Vec<&i32> = list.iter().collect();
        assert_eq!(vals, vec![&1, &2, &3]);
    }

    #[test]
    fn test_from_vec() {
        let list = LinkedList::from(vec![1, 2, 3]);
        let vals: Vec<&i32> = list.iter().collect();
        assert_eq!(vals, vec![&1, &2, &3]);
    }

    #[test]
    fn test_display() {
        let list = LinkedList::from(vec![1, 2]);
        assert_eq!(format!("{list}"), "1 -> 2 -> Nil");
    }
}
