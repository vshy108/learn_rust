// LeetCode 114: Flatten Binary Tree to Linked List [Medium]
// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 3.926us
//   AI solution avg: 1.988us
//   Ratio: AI 1.97x faster
//
// Given the root of a binary tree, flatten the tree into a "linked list"
// using the right pointers (preorder traversal order). Left should be null.
//
// Constraints: [0, 2000] nodes, -100 <= Node.val <= 100
//
// Run:  cargo run --example lc_0114_flatten_binary_tree
// Test: cargo test --example lc_0114_flatten_binary_tree
// Bench: cargo test --example lc_0114_flatten_binary_tree bench -- --nocapture

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

/// Collect right-chain values (after flattening, tree = right-only chain).
fn collect_chain(root: &Node) -> Vec<i32> {
    let mut result = Vec::new();
    let mut cur = root.clone();
    while let Some(node) = cur {
        let node = node.borrow();
        assert!(node.left.is_none(), "left must be None after flatten");
        result.push(node.val);
        cur = node.right.clone();
    }
    result
}

/// My solution — preorder collect then rebuild right-chain.
///
/// Time: O(n).
/// Space: O(n) for collecting preorder values.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Flatten tree to linked list in-place (preorder).
        pub fn flatten(root: &mut Node) {
            let mut vals = Vec::new();
            Self::preorder(root, &mut vals);
            // Build chain from back to front
            *root = None;
            for &v in vals.iter().rev() {
                let mut node = TreeNode::new(v);
                node.right = root.take();
                *root = Some(Rc::new(RefCell::new(node)));
            }
        }

        fn preorder(node: &Node, vals: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                vals.push(n.val);
                Self::preorder(&n.left, vals);
                Self::preorder(&n.right, vals);
            }
        }
    }
}

/// AI solution — Morris-like traversal: for each node with left child,
/// find the rightmost of its left subtree, connect it to right, move left to right.
///
/// Time: O(n).
/// Space: O(1) extra (in-place rewiring).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Flatten tree using iterative rewiring (O(1) space).
        pub fn flatten(root: &mut Node) {
            let mut cur = root.clone();
            while let Some(node) = cur.clone() {
                let mut node_borrow = node.borrow_mut();
                if let Some(left) = node_borrow.left.take() {
                    // Find rightmost node of left subtree
                    let mut rightmost = left.clone();
                    loop {
                        let next = rightmost.borrow().right.clone();
                        match next {
                            Some(n) => rightmost = n,
                            None => break,
                        }
                    }
                    // Connect right subtree to rightmost of left
                    rightmost.borrow_mut().right = node_borrow.right.take();
                    // Move left subtree to right
                    node_borrow.right = Some(left);
                }
                cur = node_borrow.right.clone();
            }
        }
    }
}

fn main() {
    let mut root = build_tree(&[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);
    my_solution::Solution::flatten(&mut root);
    println!("my flatten: {:?}", collect_chain(&root));

    let mut root = build_tree(&[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);
    ai_solution::Solution::flatten(&mut root);
    println!("ai flatten: {:?}", collect_chain(&root));
}

#[cfg(test)]
mod tests {
    use super::{build_tree, collect_chain};

    macro_rules! test_flatten {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{build_tree, collect_chain};

                #[test]
                fn test_example1() {
                    let mut root =
                        build_tree(&[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);
                    <$solution>::flatten(&mut root);
                    assert_eq!(collect_chain(&root), vec![1, 2, 3, 4, 5, 6]);
                }

                #[test]
                fn test_empty() {
                    let mut root = None;
                    <$solution>::flatten(&mut root);
                    assert_eq!(collect_chain(&root), Vec::<i32>::new());
                }

                #[test]
                fn test_single() {
                    let mut root = build_tree(&[Some(0)]);
                    <$solution>::flatten(&mut root);
                    assert_eq!(collect_chain(&root), vec![0]);
                }

                #[test]
                fn test_left_only() {
                    let mut root = build_tree(&[Some(1), Some(2), None, Some(3)]);
                    <$solution>::flatten(&mut root);
                    assert_eq!(collect_chain(&root), vec![1, 2, 3]);
                }

                #[test]
                fn test_right_only() {
                    let mut root = build_tree(&[Some(1), None, Some(2), None, Some(3)]);
                    <$solution>::flatten(&mut root);
                    assert_eq!(collect_chain(&root), vec![1, 2, 3]);
                }
            }
        };
    }

    test_flatten!(my, crate::my_solution::Solution);
    test_flatten!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=31).map(Some).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut root = build_tree(&vals);
            crate::my_solution::Solution::flatten(std::hint::black_box(&mut root));
            std::hint::black_box(&root);
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut root = build_tree(&vals);
            crate::ai_solution::Solution::flatten(std::hint::black_box(&mut root));
            std::hint::black_box(&root);
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
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
