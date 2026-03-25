// Demonstrates `self` as immutable and mutable references in methods.
//
// Method receivers:
// | Receiver    | Type        | Can read? | Can modify? | Consumes? |
// |-------------|-------------|-----------|-------------|-----------|
// | `&self`     | `&Self`     | Yes       | No          | No        |
// | `&mut self` | `&mut Self` | Yes       | Yes         | No        |
// | `self`      | `Self`      | Yes       | Yes (local) | Yes       |
//
// Run: cargo run --example s09_self_ref

#[derive(Debug)]
struct Stack<T> {
    elements: Vec<T>,
}

impl<T: std::fmt::Debug> Stack<T> {
    /// Creates an empty stack.
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    /// Returns the number of elements (&self — read-only).
    fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns true if the stack is empty (&self — read-only).
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Peeks at the top element without removing (&self — read-only).
    ///
    /// # Returns
    ///
    /// Reference to the top element, or None if empty
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    /// Pushes an element onto the stack (&mut self — modifies).
    ///
    /// # Arguments
    ///
    /// * `item` - The item to push
    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    /// Pops the top element (&mut self — modifies).
    ///
    /// # Returns
    ///
    /// The top element, or None if empty
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Consumes the stack and returns all elements as a Vec (self — ownership).
    ///
    /// # Returns
    ///
    /// The underlying Vec
    fn into_vec(self) -> Vec<T> {
        self.elements
    }
}

fn main() {
    let mut stack = Stack::new();

    // --- 1. &mut self methods: modify the stack ---
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Stack: {stack:?}");

    // --- 2. &self methods: read without modifying ---
    println!("len: {}", stack.len());
    println!("is_empty: {}", stack.is_empty());
    println!("peek: {:?}", stack.peek());

    // --- 3. &mut self: pop ---
    let top = stack.pop();
    println!("Popped: {top:?}, stack: {stack:?}");

    // --- 4. self: consume the stack ---
    let vec = stack.into_vec();
    println!("As vec: {vec:?}");
    // println!("{stack:?}"); // error: stack was moved into into_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
    }

    #[test]
    fn test_into_vec() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.into_vec(), vec![1, 2]);
    }
}
