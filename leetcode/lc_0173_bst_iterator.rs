// LeetCode 173: Binary Search Tree Iterator [Medium]
// https://leetcode.com/problems/binary-search-tree-iterator/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.653us
//   AI solution avg: 0.314us
//   Ratio: AI 2.08x faster
//
// Implement BSTIterator that takes a BST root and iterates in-order:
//   - next() -> returns the next smallest number
//   - has_next() -> returns whether there is a next number
// Both must run in O(1) average time, O(h) space where h is tree height.
//
// Constraints: nodes in [0, 10^5], 0 <= Node.val <= 10^6
//   At most 10^5 calls to has_next/next. next() always valid.
//
// Run:  cargo run --example lc_0173_bst_iterator
// Test: cargo test --example lc_0173_bst_iterator
// Bench: cargo test --example lc_0173_bst_iterator bench -- --nocapture

use std::cell::RefCell;
use std::rc::Rc;

/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Helper to build a BST from a sorted slice.
fn sorted_to_bst(vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() {
        return None;
    }
    let mid = vals.len() / 2;
    let mut node = TreeNode::new(vals[mid]);
    node.left = sorted_to_bst(&vals[..mid]);
    node.right = sorted_to_bst(&vals[mid + 1..]);
    Some(Rc::new(RefCell::new(node)))
}

/// My solution — flatten the BST into a sorted Vec via recursive
/// in-order traversal at construction; next() and has_next() are O(1).
///
/// Time: O(n) to construct; O(1) per next/has_next call.
/// Space: O(n) for the flattened Vec.
mod my_solution {
    use super::*;

    /// Flatten-to-vec approach: collect all values in-order up front.
    pub struct BSTIterator {
        values: Vec<i32>,
        index: usize,
    }

    impl BSTIterator {
        /// Create a new iterator by flattening the BST into a sorted vec.
        pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
            let mut values = Vec::new();
            Self::inorder(&root, &mut values);
            BSTIterator { values, index: 0 }
        }

        /// Return the next smallest number.
        pub fn next(&mut self) -> i32 {
            let val = self.values[self.index];
            self.index += 1;
            val
        }

        /// Return true if there is a next smallest number.
        pub fn has_next(&self) -> bool {
            self.index < self.values.len()
        }

        /// Recursive in-order traversal to flatten the tree.
        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
            if let Some(n) = node {
                let borrowed = n.borrow();
                Self::inorder(&borrowed.left, out);
                out.push(borrowed.val);
                Self::inorder(&borrowed.right, out);
            }
        }
    }
}

/// AI solution — maintain an explicit stack of left-spine nodes.
///
/// Push all left children initially. On next(), pop the top, then
/// push all left children of the right subtree (controlled push).
///
/// Time: O(1) amortized per call (each node pushed/popped exactly once).
/// Space: O(h) for the stack.
mod ai_solution {
    use super::*;

    pub struct BSTIterator {
        stack: Vec<Rc<RefCell<TreeNode>>>,
    }

    impl BSTIterator {
        /// Create a new iterator, pushing the left spine from root.
        pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
            let mut iter = BSTIterator { stack: Vec::new() };
            iter.push_left(root);
            iter
        }

        /// Return the next smallest number.
        pub fn next(&mut self) -> i32 {
            let node = self
                .stack
                .pop()
                .expect("next() called when has_next() is false");
            let borrowed = node.borrow();
            let val = borrowed.val;
            // Push the left spine of the right subtree
            let right = borrowed.right.clone();
            drop(borrowed); // release borrow before pushing
            self.push_left(right);
            val
        }

        /// Return true if there is a next smallest number.
        pub fn has_next(&self) -> bool {
            !self.stack.is_empty()
        }

        /// Push all left children from `node` down to the leftmost leaf.
        fn push_left(&mut self, mut node: Option<Rc<RefCell<TreeNode>>>) {
            while let Some(n) = node {
                let left = n.borrow().left.clone();
                self.stack.push(n);
                node = left;
            }
        }
    }
}

fn main() {
    let root = sorted_to_bst(&[3, 7, 9, 15, 20]);
    let mut my_it = my_solution::BSTIterator::new(root.clone());
    print!("my: ");
    while my_it.has_next() {
        print!("{} ", my_it.next());
    }
    println!();
    let mut it = ai_solution::BSTIterator::new(root);
    print!("ai: ");
    while it.has_next() {
        print!("{} ", it.next());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        let root = sorted_to_bst(&[3, 7, 9, 15, 20]);
        let mut it = my_solution::BSTIterator::new(root);
        let mut result = Vec::new();
        while it.has_next() {
            result.push(it.next());
        }
        assert_eq!(result, vec![3, 7, 9, 15, 20]);
    }

    #[test]
    fn test_ai_solution() {
        let root = sorted_to_bst(&[3, 7, 9, 15, 20]);
        let mut it = ai_solution::BSTIterator::new(root);
        let mut result = Vec::new();
        while it.has_next() {
            result.push(it.next());
        }
        assert_eq!(result, vec![3, 7, 9, 15, 20]);
    }

    #[test]
    fn test_ai_single() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(42))));
        let mut it = ai_solution::BSTIterator::new(root);
        assert!(it.has_next());
        assert_eq!(it.next(), 42);
        assert!(!it.has_next());
    }

    #[test]
    fn test_ai_empty() {
        let mut it = ai_solution::BSTIterator::new(None);
        assert!(!it.has_next());
    }

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<i32> = (0..63).collect(); // balanced BST with 63 nodes
        let root = sorted_to_bst(&vals);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root_clone = root.clone();
            let mut it = my_solution::BSTIterator::new(root_clone);
            while it.has_next() {
                std::hint::black_box(it.next());
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root_clone = root.clone();
            let mut it = ai_solution::BSTIterator::new(root_clone);
            while it.has_next() {
                std::hint::black_box(it.next());
            }
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
