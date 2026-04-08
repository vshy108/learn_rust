// LeetCode 230: Kth Smallest Element in a BST [Medium]
// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.304us
//   AI solution avg: 1.039us
//   Ratio: AI 1.25x faster
//
// Given the root of a binary search tree, and an integer k, return the
// kth smallest value (1-indexed) of all the values of the nodes in the
// tree.
//
// Constraints: 1 <= k <= n <= 10^4, 0 <= Node.val <= 10^4
//
// Run:  cargo run --example lc_0230_kth_smallest_element_in_bst
// Test: cargo test --example lc_0230_kth_smallest_element_in_bst
// Bench: cargo test --example lc_0230_kth_smallest_element_in_bst bench -- --nocapture

use std::cell::RefCell;
use std::rc::Rc;

/// Binary tree node definition.
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

/// My solution — recursive in-order traversal collecting into a Vec.
///
/// In-order traversal of a BST yields sorted values. Collect them all,
/// then index at k-1. Simple but allocates the full list.
///
/// Time: O(n) — visit every node.
/// Space: O(n) — store all values plus O(h) recursion stack.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the kth smallest element in the BST.
        pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
            let mut vals = Vec::new();
            Self::inorder(&root, &mut vals);
            // k is 1-indexed, so kth smallest is at index k-1
            vals[(k - 1) as usize]
        }

        /// Collect all node values in in-order (sorted) sequence.
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

/// AI solution — iterative in-order with early exit.
///
/// Use an explicit stack for in-order traversal. Decrement k each time
/// we "visit" a node. Stop as soon as k reaches 0 — no need to traverse
/// the entire tree.
///
/// Time: O(h + k) — push left spine (h), then pop k nodes.
/// Space: O(h) — stack never exceeds tree height.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the kth smallest element in the BST.
        pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
            let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
            let mut current = root;
            let mut remaining = k;

            loop {
                // Push all left children onto the stack (leftmost path)
                while let Some(node) = current {
                    stack.push(Rc::clone(&node));
                    let left = node.borrow().left.clone();
                    current = left;
                }

                // Pop the next in-order node
                let node = stack.pop().expect("k <= n guarantees stack is non-empty");
                remaining -= 1;

                if remaining == 0 {
                    return node.borrow().val;
                }

                // Move to the right subtree
                current = node.borrow().right.clone();
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
    // Example 1: [3,1,4,null,2], k=1 => 1
    let tree1 = build_tree(&[Some(3), Some(1), Some(4), None, Some(2)]);
    let my1 = my_solution::Solution::kth_smallest(tree1.clone(), 1);
    let ai1 = ai_solution::Solution::kth_smallest(tree1, 1);
    println!("Example 1: my={my1}, ai={ai1} (expected 1)");

    // Example 2: [5,3,6,2,4,null,null,1], k=3 => 3
    let tree2 = build_tree(&[
        Some(5),
        Some(3),
        Some(6),
        Some(2),
        Some(4),
        None,
        None,
        Some(1),
    ]);
    let my2 = my_solution::Solution::kth_smallest(tree2.clone(), 3);
    let ai2 = ai_solution::Solution::kth_smallest(tree2, 3);
    println!("Example 2: my={my2}, ai={ai2} (expected 3)");
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_kth {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    let tree = build_tree(&[Some(3), Some(1), Some(4), None, Some(2)]);
                    assert_eq!(<$solution>::kth_smallest(tree, 1), 1);
                }

                #[test]
                fn test_example2() {
                    let tree = build_tree(&[
                        Some(5),
                        Some(3),
                        Some(6),
                        Some(2),
                        Some(4),
                        None,
                        None,
                        Some(1),
                    ]);
                    assert_eq!(<$solution>::kth_smallest(tree, 3), 3);
                }

                #[test]
                fn test_single_node() {
                    let tree = build_tree(&[Some(42)]);
                    assert_eq!(<$solution>::kth_smallest(tree, 1), 42);
                }

                #[test]
                fn test_k_equals_n() {
                    // Largest element (rightmost)
                    let tree = build_tree(&[Some(3), Some(1), Some(4), None, Some(2)]);
                    assert_eq!(<$solution>::kth_smallest(tree, 4), 4);
                }

                #[test]
                fn test_left_skewed() {
                    // Tree: 5 -> 4 -> 3 -> 2 -> 1
                    let tree = build_tree(&[
                        Some(5),
                        Some(4),
                        None,
                        Some(3),
                        None,
                        Some(2),
                        None,
                        Some(1),
                    ]);
                    assert_eq!(<$solution>::kth_smallest(tree, 2), 2);
                }
            }
        };
    }

    test_kth!(my, crate::my_solution::Solution);
    test_kth!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a balanced BST with 15 nodes
        let tree_vals = vec![
            Some(8),
            Some(4),
            Some(12),
            Some(2),
            Some(6),
            Some(10),
            Some(14),
            Some(1),
            Some(3),
            Some(5),
            Some(7),
            Some(9),
            Some(11),
            Some(13),
            Some(15),
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let tree = build_tree(&tree_vals);
            std::hint::black_box(crate::my_solution::Solution::kth_smallest(tree, 7));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let tree = build_tree(&tree_vals);
            std::hint::black_box(crate::ai_solution::Solution::kth_smallest(tree, 7));
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
