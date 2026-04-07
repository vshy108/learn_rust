// LeetCode 285: Inorder Successor in BST [Medium] (Premium)
// https://leetcode.com/problems/inorder-successor-in-bst/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 0.007us
//   AI solution avg: 0.016us
//   Ratio: My 2.41x faster
//
// Given the root of a BST and a node p, return the in-order successor
// of p (the node with the smallest key greater than p.val). Return None
// if p has no in-order successor.
//
// Example 1: root = [2,1,3], p = 1 -> 2
// Example 2: root = [5,3,6,2,4,null,null,1], p = 6 -> null
//
// Constraints:
//   - Number of nodes in [1, 10^4]
//   - -10^5 <= Node.val <= 10^5
//   - All values are unique
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0285.Inorder%20Successor%20in%20BST/README_EN.md
//
// Run:  cargo run --example lc_0285_inorder_successor_bst
// Test: cargo test --example lc_0285_inorder_successor_bst
// Bench: cargo test --example lc_0285_inorder_successor_bst bench -- --nocapture

use std::cell::RefCell;
use std::collections::VecDeque;
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
        TreeNode { val, left: None, right: None }
    }
}

/// Build a binary tree from a level-order Vec<Option<i32>>.
fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    let mut i = 1;
    while i < vals.len() {
        // Guard: if queue is exhausted but vals remain, stop
        let Some(node) = queue.pop_front() else { break };
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(v)));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;
        }
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(v)));
                node.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }
    }
    Some(root)
}

/// My solution — Iterative BST search (from doocs).
///
/// Walk down the BST. If current node's value > p, it's a potential
/// successor — record it and go left to find a smaller one. If <= p,
/// go right. After the loop, `ans` holds the smallest value > p.
///
/// Time:  O(h) where h = tree height
/// Space: O(1)
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        pub fn inorder_successor(
            root: Option<Rc<RefCell<TreeNode>>>,
            p_val: i32,
        ) -> Option<i32> {
            let mut ans: Option<i32> = None;
            let mut cur = root;
            while let Some(node) = cur {
                let node_ref = node.borrow();
                if node_ref.val > p_val {
                    ans = Some(node_ref.val);
                    cur = node_ref.left.clone(); // try to find smaller successor
                } else {
                    cur = node_ref.right.clone(); // current <= p, go right
                }
            }
            ans
        }
    }
}

/// AI solution — Recursive approach.
///
/// If root.val <= p, the successor must be in the right subtree.
/// If root.val > p, the successor is either in the left subtree
/// (if one exists there) or is root itself.
///
/// Time:  O(h)
/// Space: O(h) for recursion stack
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        pub fn inorder_successor(
            root: Option<Rc<RefCell<TreeNode>>>,
            p_val: i32,
        ) -> Option<i32> {
            let Some(node) = root else { return None };
            let node_ref = node.borrow();
            if node_ref.val <= p_val {
                // Successor must be in the right subtree
                Self::inorder_successor(node_ref.right.clone(), p_val)
            } else {
                // This node could be the successor, but check left subtree
                // for a potentially smaller successor
                Self::inorder_successor(node_ref.left.clone(), p_val)
                    .or(Some(node_ref.val))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_successor {
        ($name:ident, $vals:expr, $p:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let root = build_tree(&$vals);
                assert_eq!(
                    my_solution::Solution::inorder_successor(root.clone(), $p),
                    $expected,
                    "my_solution failed"
                );
                assert_eq!(
                    ai_solution::Solution::inorder_successor(root, $p),
                    $expected,
                    "ai_solution failed"
                );
            }
        };
    }

    // Example 1: [2,1,3], p=1 -> 2
    test_successor!(
        test_example_1,
        [Some(2), Some(1), Some(3)],
        1,
        Some(2)
    );

    // Example 2: [5,3,6,2,4,null,null,1], p=6 -> null
    test_successor!(
        test_example_2,
        [Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)],
        6,
        None
    );

    // Successor is the root
    test_successor!(
        test_successor_is_root,
        [Some(5), Some(3), Some(8)],
        3,
        Some(5)
    );

    // Successor is in right subtree
    test_successor!(
        test_successor_right,
        [Some(5), Some(3), Some(8), Some(1), Some(4)],
        3,
        Some(4)
    );

    // Leftmost node, successor is parent
    test_successor!(
        test_leftmost,
        [Some(5), Some(3), Some(8), Some(1)],
        1,
        Some(3)
    );

    // Rightmost node (max value), no successor
    test_successor!(
        test_rightmost,
        [Some(5), Some(3), Some(8)],
        8,
        None
    );

    // Single node, p is that node
    test_successor!(test_single_node, [Some(1)], 1, None);

    // Left-skewed tree
    test_successor!(
        test_left_skewed,
        [Some(5), Some(3), None, Some(1)],
        1,
        Some(3)
    );

    // Right-skewed tree: 1 -> 3 -> 5
    test_successor!(
        test_right_skewed,
        [Some(1), None, Some(3), None, Some(5)],
        1,
        Some(3)
    );

    // Large balanced tree
    test_successor!(
        test_balanced,
        [Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14)],
        6,
        Some(8)
    );

    // Successor of root
    test_successor!(
        test_root_successor,
        [Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14)],
        8,
        Some(10)
    );
}

fn main() {
    println!("LeetCode 285: Inorder Successor in BST");
    let root = build_tree(&[Some(2), Some(1), Some(3)]);
    println!(
        "my_solution: successor of 1 = {:?}",
        my_solution::Solution::inorder_successor(root.clone(), 1)
    );
    println!(
        "ai_solution: successor of 1 = {:?}",
        ai_solution::Solution::inorder_successor(root, 1)
    );
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let root = build_tree(&[
            Some(8),
            Some(4),
            Some(12),
            Some(2),
            Some(6),
            Some(10),
            Some(14),
        ]);
        let p_val = 6;
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(my_solution::Solution::inorder_successor(
                root.clone(),
                p_val,
            ));
            std::hint::black_box(ai_solution::Solution::inorder_successor(
                root.clone(),
                p_val,
            ));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::inorder_successor(
                root.clone(),
                p_val,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::inorder_successor(
                root.clone(),
                p_val,
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
