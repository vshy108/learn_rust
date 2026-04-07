// LeetCode 226: Invert Binary Tree [Easy]
// https://leetcode.com/problems/invert-binary-tree/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.224us
//   AI solution avg: 1.088us
//   Ratio: AI 1.13x faster
//
// Given the root of a binary tree, invert the tree, and return its root.
// Inverting means swapping every left and right child.
//
// Constraints: [0, 100] nodes, -100 <= Node.val <= 100
//
// Run:  cargo run --example lc_0226_invert_binary_tree
// Test: cargo test --example lc_0226_invert_binary_tree
// Bench: cargo test --example lc_0226_invert_binary_tree bench -- --nocapture

use std::cell::RefCell;
use std::rc::Rc;

/// Definition for a binary tree node (shared across both solutions).
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// Convenience constructor for a leaf node.
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

/// My solution — recursive DFS.
///
/// Recursively invert left and right subtrees, then swap them.
/// Classic post-order traversal approach.
///
/// Time: O(n) — visit every node once.
/// Space: O(h) — recursion stack depth, where h is the tree height.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Invert the binary tree rooted at `root` and return the new root.
        pub fn invert_tree(
            root: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                let mut borrowed = node.borrow_mut();
                // Recursively invert each subtree
                let left = Self::invert_tree(borrowed.left.take());
                let right = Self::invert_tree(borrowed.right.take());
                // Swap: old left becomes new right and vice versa
                borrowed.left = right;
                borrowed.right = left;
                drop(borrowed); // release borrow before returning Rc
                Some(node)
            } else {
                None
            }
        }
    }
}

/// AI solution — iterative BFS with a queue.
///
/// Use a VecDeque to traverse level by level. For each node, swap its
/// left and right children, then enqueue the children.
///
/// Time: O(n) — visit every node once.
/// Space: O(w) — queue holds at most one level's width of nodes.
mod ai_solution {
    use super::*;
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Invert the binary tree rooted at `root` and return the new root.
        pub fn invert_tree(
            root: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let root_ref = root.as_ref()?; // return None if empty

            let mut queue = VecDeque::new();
            queue.push_back(Rc::clone(root_ref));

            while let Some(node) = queue.pop_front() {
                let mut borrowed = node.borrow_mut();
                // Swap left and right children.
                // We use take() + reassign instead of std::mem::swap to
                // avoid borrowing two fields of the same struct mutably.
                let left = borrowed.left.take();
                let right = borrowed.right.take();
                borrowed.left = right;
                borrowed.right = left;
                // Enqueue children (after the swap) for further processing
                if let Some(ref left) = borrowed.left {
                    queue.push_back(Rc::clone(left));
                }
                if let Some(ref right) = borrowed.right {
                    queue.push_back(Rc::clone(right));
                }
            }

            root
        }
    }
}

/// Build a tree from a level-order Vec (None = null node).
fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;
    while i < values.len() {
        if let Some(parent) = queue.pop_front() {
            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    parent.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < values.len() {
                if let Some(val) = values[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    parent.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }
    }
    Some(root)
}

/// Collect tree values in level-order (BFS) for easy comparison.
fn to_level_order(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    if let Some(r) = root {
        queue.push_back(Rc::clone(r));
    }
    while let Some(node) = queue.pop_front() {
        let borrowed = node.borrow();
        result.push(Some(borrowed.val));
        if let Some(ref left) = borrowed.left {
            queue.push_back(Rc::clone(left));
        }
        if let Some(ref right) = borrowed.right {
            queue.push_back(Rc::clone(right));
        }
    }
    result
}

fn main() {
    let tree = build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
    let inverted = my_solution::Solution::invert_tree(tree);
    println!("My: {:?}", to_level_order(&inverted));

    let tree2 = build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
    let inverted2 = ai_solution::Solution::invert_tree(tree2);
    println!("AI: {:?}", to_level_order(&inverted2));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_invert {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    let tree = build_tree(&[
                        Some(4), Some(2), Some(7),
                        Some(1), Some(3), Some(6), Some(9),
                    ]);
                    let result = <$solution>::invert_tree(tree);
                    // Inverted: [4, 7, 2, 9, 6, 3, 1]
                    assert_eq!(
                        to_level_order(&result),
                        vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)],
                    );
                }

                #[test]
                fn test_example2() {
                    let tree = build_tree(&[Some(2), Some(1), Some(3)]);
                    let result = <$solution>::invert_tree(tree);
                    assert_eq!(
                        to_level_order(&result),
                        vec![Some(2), Some(3), Some(1)],
                    );
                }

                #[test]
                fn test_empty() {
                    let result = <$solution>::invert_tree(None);
                    assert!(result.is_none());
                }

                #[test]
                fn test_single_node() {
                    let tree = build_tree(&[Some(1)]);
                    let result = <$solution>::invert_tree(tree);
                    assert_eq!(to_level_order(&result), vec![Some(1)]);
                }

                #[test]
                fn test_left_only() {
                    // Tree: 1 -> left: 2
                    let tree = build_tree(&[Some(1), Some(2), None]);
                    let result = <$solution>::invert_tree(tree);
                    // After invert, 2 should be on the right
                    let root = result.unwrap();
                    let borrowed = root.borrow();
                    assert!(borrowed.left.is_none());
                    assert_eq!(borrowed.right.as_ref().unwrap().borrow().val, 2);
                }
            }
        };
    }

    test_invert!(my, crate::my_solution::Solution);
    test_invert!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let values = vec![
            Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9),
            Some(0), Some(5), Some(8), Some(10), Some(11), Some(12), Some(13), Some(14),
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let tree = build_tree(&values);
            std::hint::black_box(crate::my_solution::Solution::invert_tree(tree));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let tree = build_tree(&values);
            std::hint::black_box(crate::ai_solution::Solution::invert_tree(tree));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
