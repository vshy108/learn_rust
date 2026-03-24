// Demonstrates generics in enum definitions.
//
// The two most important generic enums in Rust are:
//
//   enum Option<T> { Some(T), None }
//   enum Result<T, E> { Ok(T), Err(E) }
//
// You can define your own generic enums too.
//
// Run: cargo run --example s11_generics_enums

/// A binary tree node that can hold any type.
#[derive(Debug, PartialEq)]
enum Tree<T> {
    Leaf(T),
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl<T: std::fmt::Display + std::ops::Add<Output = T> + Copy> Tree<T> {
    /// Counts all nodes (leaves + internal).
    fn count(&self) -> usize {
        match self {
            Tree::Leaf(_) => 1,
            Tree::Node { left, right, .. } => 1 + left.count() + right.count(),
        }
    }

    /// Sums all values in the tree.
    fn sum(&self) -> T {
        match self {
            Tree::Leaf(v) => *v,
            Tree::Node { value, left, right } => *value + left.sum() + right.sum(),
        }
    }
}

/// A computation result — like Result but with an extra "pending" state.
#[derive(Debug, PartialEq)]
enum AsyncResult<T, E> {
    Ready(T),
    Error(E),
    Pending,
}

impl<T, E: std::fmt::Display> AsyncResult<T, E> {
    /// Checks if the result is ready.
    fn is_ready(&self) -> bool {
        matches!(self, AsyncResult::Ready(_))
    }

    /// Unwraps the ready value, panicking otherwise.
    fn unwrap(self) -> T {
        match self {
            AsyncResult::Ready(v) => v,
            AsyncResult::Error(e) => panic!("called unwrap on Error: {e}"),
            AsyncResult::Pending => panic!("called unwrap on Pending"),
        }
    }
}

fn main() {
    // --- 1. Tree<i32> ---
    let tree = Tree::Node {
        value: 1,
        left: Box::new(Tree::Leaf(2)),
        right: Box::new(Tree::Node {
            value: 3,
            left: Box::new(Tree::Leaf(4)),
            right: Box::new(Tree::Leaf(5)),
        }),
    };
    println!("Tree: {tree:?}");
    println!("Count: {}", tree.count());
    println!("Sum: {}", tree.sum());

    // --- 2. AsyncResult ---
    let r1: AsyncResult<String, &str> = AsyncResult::Ready("done".to_string());
    let r2: AsyncResult<String, &str> = AsyncResult::Pending;
    let r3: AsyncResult<String, &str> = AsyncResult::Error("timeout");

    println!("r1 ready? {}", r1.is_ready());
    println!("r2 ready? {}", r2.is_ready());
    println!("r3 ready? {}", r3.is_ready());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf() {
        let t = Tree::Leaf(42);
        assert_eq!(t.count(), 1);
        assert_eq!(t.sum(), 42);
    }

    #[test]
    fn test_tree_count() {
        let t = Tree::Node {
            value: 1,
            left: Box::new(Tree::Leaf(2)),
            right: Box::new(Tree::Leaf(3)),
        };
        assert_eq!(t.count(), 3);
    }

    #[test]
    fn test_tree_sum() {
        let t = Tree::Node {
            value: 10,
            left: Box::new(Tree::Leaf(20)),
            right: Box::new(Tree::Leaf(30)),
        };
        assert_eq!(t.sum(), 60);
    }

    #[test]
    fn test_async_result_ready() {
        let r: AsyncResult<i32, String> = AsyncResult::Ready(42);
        assert!(r.is_ready());
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn test_async_result_pending() {
        let r: AsyncResult<i32, String> = AsyncResult::Pending;
        assert!(!r.is_ready());
    }

    #[test]
    #[should_panic(expected = "called unwrap on Pending")]
    fn test_async_result_unwrap_pending() {
        let r: AsyncResult<i32, String> = AsyncResult::Pending;
        r.unwrap();
    }
}
