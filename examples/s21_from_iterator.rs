// Demonstrates the FromIterator trait.
//
// `FromIterator` is what makes `.collect()` work.
// You can implement it for your own types to enable collecting
// iterators into custom collections.
//
// Run: cargo run --example s21_from_iterator

use std::iter::FromIterator;

/// A stack data structure that can be built from an iterator.
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Implement FromIterator to enable .collect::<Stack<T>>()
impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut stack = Stack::new();
        for item in iter {
            stack.push(item);
        }
        stack
    }
}

/// Also implement IntoIterator for consuming iteration.
impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::iter::Rev<std::vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter().rev() // stack order: LIFO
    }
}

fn main() {
    // --- 1. Collect into our Stack ---
    let stack: Stack<i32> = (1..=5).collect();
    println!("stack: {stack:?}");
    println!("peek: {:?}", stack.peek());

    // --- 2. Iterate over the stack ---
    for item in stack {
        print!("{item} "); // 5 4 3 2 1 (LIFO)
    }
    println!();

    // --- 3. Collect from filtered iterator ---
    let even_stack: Stack<i32> = (1..=10).filter(|n| n % 2 == 0).collect();
    println!("even stack: {even_stack:?}");

    // --- 4. Standard FromIterator examples ---
    let v: Vec<i32> = (1..=3).collect();
    let s: String = vec!['h', 'e', 'l', 'l', 'o'].into_iter().collect();
    println!("Vec: {v:?}");
    println!("String: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_stack() {
        let s: Stack<i32> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn test_stack_lifo() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
    }

    #[test]
    fn test_into_iter() {
        let s: Stack<i32> = vec![1, 2, 3].into_iter().collect();
        let v: Vec<i32> = s.into_iter().collect();
        assert_eq!(v, vec![3, 2, 1]); // LIFO
    }
}
