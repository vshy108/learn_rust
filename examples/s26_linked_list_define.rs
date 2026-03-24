// Demonstrates defining a linked list with a struct-based approach.
//
// A more practical linked list uses a wrapper struct with methods,
// rather than using the enum directly everywhere.
//
// Run: cargo run --example s26_linked_list_define

/// A singly linked list.
#[derive(Debug)]
struct LinkedList<T: std::fmt::Debug> {
    head: Option<Box<Node<T>>>,
}

/// A single node in the linked list.
#[derive(Debug)]
struct Node<T: std::fmt::Debug> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    /// Creates a new empty linked list.
    fn new() -> Self {
        LinkedList { head: None }
    }

    /// Returns true if the list is empty.
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns the number of elements.
    fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }

    /// Pushes a value to the front of the list.
    fn push_front(&mut self, value: T) {
        // Take the current head and make it the `next` of the new node
        let new_node = Box::new(Node {
            value,
            next: self.head.take(), // take() replaces head with None
        });
        self.head = Some(new_node);
    }

    /// Pops a value from the front. Returns None if empty.
    fn pop_front(&mut self) -> Option<T> {
        // Take the head node out
        self.head.take().map(|node| {
            self.head = node.next; // next node becomes the new head
            node.value
        })
    }
}

fn main() {
    println!("=== Defining a LinkedList ===");

    let mut list = LinkedList::new();
    println!("empty? {}", list.is_empty());

    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    println!("list: {list:?}");
    println!("len: {}", list.len());

    while let Some(val) = list.pop_front() {
        print!("{val} ");
    }
    println!();
    println!("empty after drain? {}", list.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_pop() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
