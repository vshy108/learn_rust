// LeetCode 235: Lowest Common Ancestor of a Binary Search Tree [Medium]
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.142us
//   AI solution avg: 0.945us
//   Ratio: AI 1.21x faster
//
// Given a binary search tree (BST), find the lowest common ancestor (LCA)
// node of two given nodes in the BST.
//
// The LCA is the lowest node that has both p and q as descendants
// (a node can be a descendant of itself).
//
// Constraints: [2, 10^5] nodes, -10^9 <= Node.val <= 10^9,
// All values are unique, p != q, p and q exist in the BST.
//
// Run:  cargo run --example lc_0235_lowest_common_ancestor_bst
// Test: cargo test --example lc_0235_lowest_common_ancestor_bst
// Bench: cargo test --example lc_0235_lowest_common_ancestor_bst bench -- --nocapture

use std::cell::RefCell;
use std::rc::Rc;

/// Binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// Create a leaf node.
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

/// My solution — recursive BST property exploitation.
///
/// If both p and q are smaller than root, LCA is in the left subtree.
/// If both are larger, LCA is in the right subtree.
/// Otherwise, root is the LCA (split point).
///
/// Time: O(h) where h is the tree height.
/// Space: O(h) for the recursion stack.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the value of the LCA of nodes with values `p` and `q`.
        pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> i32 {
            let node = root.expect("root must be non-null");
            let val = node.borrow().val;

            if p < val && q < val {
                // Both targets are in the left subtree
                Self::lowest_common_ancestor(node.borrow().left.clone(), p, q)
            } else if p > val && q > val {
                // Both targets are in the right subtree
                Self::lowest_common_ancestor(node.borrow().right.clone(), p, q)
            } else {
                // Split point: one on each side, or one equals root
                val
            }
        }
    }
}

/// AI solution — iterative approach.
///
/// Same logic as recursive, but with a loop. Avoids recursion overhead.
///
/// Time: O(h).
/// Space: O(1).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the value of the LCA of nodes with values `p` and `q`.
        pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> i32 {
            let mut current = root;

            loop {
                let node = current.expect("should find LCA before null");
                let val = node.borrow().val;

                if p < val && q < val {
                    current = node.borrow().left.clone();
                } else if p > val && q > val {
                    current = node.borrow().right.clone();
                } else {
                    return val;
                }
            }
        }
    }
}

/// Build a BST from a level-order Vec (None = null node).
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

fn main() {
    // BST: [6,2,8,0,4,7,9,null,null,3,5]
    let tree = build_tree(&[
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ]);

    let my1 = my_solution::Solution::lowest_common_ancestor(tree.clone(), 2, 8);
    let ai1 = ai_solution::Solution::lowest_common_ancestor(tree.clone(), 2, 8);
    println!("LCA(2,8): my={my1}, ai={ai1} (expected 6)");

    let my2 = my_solution::Solution::lowest_common_ancestor(tree.clone(), 2, 4);
    let ai2 = ai_solution::Solution::lowest_common_ancestor(tree, 2, 4);
    println!("LCA(2,4): my={my2}, ai={ai2} (expected 2)");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> Option<Rc<RefCell<TreeNode>>> {
        build_tree(&[
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ])
    }

    macro_rules! test_lca {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::lowest_common_ancestor(sample_tree(), 2, 8), 6,);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::lowest_common_ancestor(sample_tree(), 2, 4), 2,);
                }

                #[test]
                fn test_same_node() {
                    assert_eq!(<$solution>::lowest_common_ancestor(sample_tree(), 3, 3), 3,);
                }

                #[test]
                fn test_leaf_nodes() {
                    assert_eq!(<$solution>::lowest_common_ancestor(sample_tree(), 3, 5), 4,);
                }

                #[test]
                fn test_root_is_lca() {
                    assert_eq!(<$solution>::lowest_common_ancestor(sample_tree(), 0, 9), 6,);
                }

                #[test]
                fn test_simple_tree() {
                    let tree = build_tree(&[Some(2), Some(1)]);
                    assert_eq!(<$solution>::lowest_common_ancestor(tree, 2, 1), 2,);
                }
            }
        };
    }

    test_lca!(my, crate::my_solution::Solution);
    test_lca!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let tree_vals = vec![
            Some(20),
            Some(10),
            Some(30),
            Some(5),
            Some(15),
            Some(25),
            Some(35),
            Some(3),
            Some(7),
            Some(12),
            Some(18),
            Some(22),
            Some(28),
            Some(32),
            Some(38),
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let tree = build_tree(&tree_vals);
            std::hint::black_box(crate::my_solution::Solution::lowest_common_ancestor(
                tree, 3, 18,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let tree = build_tree(&tree_vals);
            std::hint::black_box(crate::ai_solution::Solution::lowest_common_ancestor(
                tree, 3, 18,
            ));
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
