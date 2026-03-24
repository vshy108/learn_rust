// Demonstrates creating a BST with a wrapper struct.
//
// A wrapper `Bst` struct provides a cleaner API than working
// with `Option<Box<BstNode>>` directly. Handles the empty tree case.
//
// Run: cargo run --example s26_bst_create

use std::fmt;

/// A binary search tree wrapper.
struct Bst {
    root: Option<Box<Node>>,
}

/// A single BST node.
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Bst {
    /// Creates an empty BST.
    fn new() -> Self {
        Bst { root: None }
    }

    /// Inserts a value.
    fn insert(&mut self, value: i32) {
        Self::insert_into(&mut self.root, value);
    }

    /// Recursive helper for insert.
    fn insert_into(node: &mut Option<Box<Node>>, value: i32) {
        match node {
            None => {
                *node = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
            }
            Some(n) => {
                if value < n.value {
                    Self::insert_into(&mut n.left, value);
                } else if value > n.value {
                    Self::insert_into(&mut n.right, value);
                }
            }
        }
    }

    /// Checks if the BST contains a value.
    fn contains(&self, value: i32) -> bool {
        Self::search(&self.root, value)
    }

    /// Recursive search helper.
    fn search(node: &Option<Box<Node>>, value: i32) -> bool {
        match node {
            None => false,
            Some(n) => {
                if value == n.value {
                    true
                } else if value < n.value {
                    Self::search(&n.left, value)
                } else {
                    Self::search(&n.right, value)
                }
            }
        }
    }

    /// Returns sorted values via in-order traversal.
    fn sorted(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::in_order(&self.root, &mut result);
        result
    }

    /// In-order traversal helper.
    fn in_order(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            Self::in_order(&n.left, result);
            result.push(n.value);
            Self::in_order(&n.right, result);
        }
    }

    /// Returns the minimum value, or None if empty.
    fn min(&self) -> Option<i32> {
        let mut current = &self.root;
        let mut min_val = None;
        while let Some(node) = current {
            min_val = Some(node.value);
            current = &node.left;
        }
        min_val
    }

    /// Returns the maximum value, or None if empty.
    fn max(&self) -> Option<i32> {
        let mut current = &self.root;
        let mut max_val = None;
        while let Some(node) = current {
            max_val = Some(node.value);
            current = &node.right;
        }
        max_val
    }
}

impl fmt::Display for Bst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.sorted())
    }
}

fn main() {
    println!("=== Creating a BST ===");

    let mut tree = Bst::new();
    for val in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
        tree.insert(val);
    }

    println!("sorted: {tree}");
    println!("contains 6: {}", tree.contains(6));
    println!("contains 5: {}", tree.contains(5));
    println!("min: {:?}", tree.min());
    println!("max: {:?}", tree.max());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted() {
        let mut tree = Bst::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        assert_eq!(tree.sorted(), vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_min_max() {
        let mut tree = Bst::new();
        tree.insert(5);
        tree.insert(1);
        tree.insert(9);
        assert_eq!(tree.min(), Some(1));
        assert_eq!(tree.max(), Some(9));
    }

    #[test]
    fn test_empty_tree() {
        let tree = Bst::new();
        assert!(!tree.contains(1));
        assert_eq!(tree.min(), None);
        assert_eq!(tree.max(), None);
    }
}
