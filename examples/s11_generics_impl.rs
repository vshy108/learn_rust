// Demonstrates generics in impl blocks.
//
// When implementing methods for a generic struct, you repeat the
// type parameters on the `impl` keyword:
//
//   impl<T> MyStruct<T> { ... }
//
// You can also add trait bounds to restrict which types get certain methods.
//
// Run: cargo run --example s11_generics_impl

use std::fmt;

/// A stack backed by a Vec with generic element type.
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

// --- Generic impl: available for ALL types T ---
impl<T> Stack<T> {
    /// Creates an empty stack.
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    /// Pushes an item.
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Pops the top item.
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Returns the number of items.
    fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if empty.
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// --- Constrained impl: only for types that implement Display ---
impl<T: fmt::Display> Stack<T> {
    /// Prints all elements (only available when T: Display).
    fn print_all(&self) {
        for (i, item) in self.items.iter().enumerate() {
            println!("  [{i}] {item}");
        }
    }
}

// --- Concrete impl: only for Stack<f64> ---
impl Stack<f64> {
    /// Computes the average (only available on Stack<f64>).
    fn average(&self) -> Option<f64> {
        if self.items.is_empty() {
            return None;
        }
        let sum: f64 = self.items.iter().sum();
        Some(sum / self.items.len() as f64)
    }
}

fn main() {
    // --- 1. Generic methods work for any T ---
    let mut int_stack: Stack<i32> = Stack::new();
    int_stack.push(10);
    int_stack.push(20);
    int_stack.push(30);
    println!("int_stack len: {}", int_stack.len());

    // --- 2. Display-constrained method ---
    println!("Contents:");
    int_stack.print_all();

    // --- 3. Concrete impl (f64 only) ---
    let mut float_stack = Stack::new();
    float_stack.push(1.0);
    float_stack.push(2.0);
    float_stack.push(3.0);
    println!("average: {:?}", float_stack.average());

    // This won't compile — average() is only for Stack<f64>:
    // int_stack.average(); // error: no method named `average`

    // --- 4. Pop ---
    while let Some(val) = int_stack.pop() {
        println!("Popped: {val}");
    }
    println!("Empty? {}", int_stack.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_len_and_empty() {
        let mut s: Stack<String> = Stack::new();
        assert!(s.is_empty());
        s.push("a".to_string());
        assert_eq!(s.len(), 1);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_average() {
        let mut s = Stack::new();
        s.push(10.0);
        s.push(20.0);
        s.push(30.0);
        let avg = s.average().unwrap();
        assert!((avg - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_average_empty() {
        let s: Stack<f64> = Stack::new();
        assert_eq!(s.average(), None);
    }
}
