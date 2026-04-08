// LeetCode 94: Binary Tree Inorder Traversal [Easy]
// https://leetcode.com/problems/binary-tree-inorder-traversal/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 5.355us
//   AI solution avg: 6.847us
//   Ratio: My 1.28x faster
//
// Given the root of a binary tree, return the inorder traversal of its values.
//
// Constraints: [0, 100] nodes, -100 <= Node.val <= 100
//
// Run:  cargo run --example lc_0094_binary_tree_inorder
// Test: cargo test --example lc_0094_binary_tree_inorder
// Bench: cargo test --example lc_0094_binary_tree_inorder bench -- --nocapture

use std::cell::RefCell;
use std::rc::Rc;

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

type Node = Option<Rc<RefCell<TreeNode>>>;

/// Helper: build tree from level-order array (None = null node).
fn build_tree(vals: &[Option<i32>]) -> Node {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].expect("checked"))));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;
    while i < vals.len() {
        if let Some(node) = queue.pop_front() {
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }
    }
    Some(root)
}

/// My solution — recursive inorder traversal.
///
/// Time: O(n).
/// Space: O(n) for recursion stack.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return inorder traversal of binary tree.
        ///
        /// # Arguments
        /// * `root` - root of binary tree
        ///
        /// # Returns
        /// Values in inorder (left, root, right)
        pub fn inorder_traversal(root: Node) -> Vec<i32> {
            let mut result = Vec::new();
            Self::inorder(&root, &mut result);
            result
        }

        fn inorder(node: &Node, result: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                Self::inorder(&n.left, result);
                result.push(n.val);
                Self::inorder(&n.right, result);
            }
        }
    }
}

/// AI solution — iterative using explicit stack (Morris-like without
/// modifying tree).
///
/// Time: O(n).
/// Space: O(n) for stack.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return inorder traversal of binary tree.
        ///
        /// # Arguments
        /// * `root` - root of binary tree
        ///
        /// # Returns
        /// Values in inorder (left, root, right)
        pub fn inorder_traversal(root: Node) -> Vec<i32> {
            let mut result = Vec::new();
            let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
            let mut current = root;

            loop {
                // Push all left children
                while let Some(node) = current {
                    stack.push(Rc::clone(&node));
                    current = node.borrow().left.clone();
                }
                // Pop and visit
                if let Some(node) = stack.pop() {
                    result.push(node.borrow().val);
                    current = node.borrow().right.clone();
                } else {
                    break;
                }
            }
            result
        }
    }
}

fn main() {
    let root = build_tree(&[Some(1), None, Some(2), Some(3)]);
    let my = my_solution::Solution::inorder_traversal(root.clone());
    let ai = ai_solution::Solution::inorder_traversal(root);
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_inorder {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_example1() {
                    let root = build_tree(&[Some(1), None, Some(2), Some(3)]);
                    assert_eq!(<$solution>::inorder_traversal(root), vec![1, 3, 2],);
                }

                #[test]
                fn test_empty() {
                    assert_eq!(<$solution>::inorder_traversal(None), Vec::<i32>::new(),);
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(1)]);
                    assert_eq!(<$solution>::inorder_traversal(root), vec![1],);
                }

                #[test]
                fn test_balanced() {
                    // Tree:     2
                    //          / \
                    //         1   3
                    let root = build_tree(&[Some(2), Some(1), Some(3)]);
                    assert_eq!(<$solution>::inorder_traversal(root), vec![1, 2, 3],);
                }
            }
        };
    }

    test_inorder!(my, crate::my_solution::Solution);
    test_inorder!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=31).map(Some).collect(); // balanced tree of 31 nodes

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::inorder_traversal(
                std::hint::black_box(root),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::inorder_traversal(
                std::hint::black_box(root),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
