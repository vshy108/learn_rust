// LeetCode 404: Sum of Left Leaves [Easy]
// https://leetcode.com/problems/sum-of-left-leaves/
// Benchmark: 100000 iterations
//   My solution (recursive DFS, is_left flag): avg 2.20us
//   AI solution (iterative BFS):               avg 4.18us
//   -> My solution is 1.90x faster
//
// Given the root of a binary tree, return the sum of all left leaves.
// A leaf is a node with no children. A left leaf is a leaf that is
// the left child of another node.
//
// Examples:
//   [3,9,20,null,null,15,7] -> 24  (left leaves: 9, 15)
//   [1] -> 0
//
// Constraints:
//   - 1 <= number of nodes <= 1000
//   - -1000 <= Node.val <= 1000
//
// Run:  cargo run --example lc_0404_sum_of_left_leaves
// Test: cargo test --example lc_0404_sum_of_left_leaves
// Bench: cargo test --example lc_0404_sum_of_left_leaves bench -- --nocapture

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
    /// Create a new leaf node.
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Helper to wrap a TreeNode in Rc<RefCell<>>.
fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

/// Helper to build a tree node with children.
fn node_with(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

/// My solution module — recursive DFS with `is_left` flag.
///
/// Pass a boolean flag indicating whether the current node is a left child.
/// If a node is a leaf AND is_left, add its value.
///
/// Time:  O(n)
/// Space: O(h) where h = tree height (recursion stack)
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the sum of all left leaves in the tree.
        ///
        /// # Arguments
        ///
        /// * `root` - Root of the binary tree
        ///
        /// # Returns
        ///
        /// Sum of left leaf values
        pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            Self::dfs(&root, false)
        }

        /// Recursive helper with is_left flag.
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    // Leaf node check: no left and no right child
                    if n.left.is_none() && n.right.is_none() {
                        return if is_left { n.val } else { 0 };
                    }
                    Self::dfs(&n.left, true) + Self::dfs(&n.right, false)
                }
            }
        }
    }
}

/// AI solution module — iterative BFS with left-child detection.
///
/// Use a queue of (node, is_left). For each dequeued node, if it's a
/// leaf and is_left, add to sum. Otherwise enqueue children.
///
/// Time:  O(n)
/// Space: O(w) where w = max tree width
mod ai_solution {
    use super::*;
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Return the sum of all left leaves in the tree.
        ///
        /// # Arguments
        ///
        /// * `root` - Root of the binary tree
        ///
        /// # Returns
        ///
        /// Sum of left leaf values
        pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let Some(root) = root else { return 0 };
            let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, bool)> = VecDeque::new();
            queue.push_back((root, false));
            let mut sum = 0;

            while let Some((node_rc, is_left)) = queue.pop_front() {
                let node = node_rc.borrow();
                let is_leaf = node.left.is_none() && node.right.is_none();

                if is_leaf && is_left {
                    sum += node.val;
                } else {
                    if let Some(left) = &node.left {
                        queue.push_back((Rc::clone(left), true));
                    }
                    if let Some(right) = &node.right {
                        queue.push_back((Rc::clone(right), false));
                    }
                }
            }
            sum
        }
    }
}

fn main() {
    // Build tree: [3, 9, 20, null, null, 15, 7]
    let tree = node_with(3, node(9), node_with(20, node(15), node(7)));

    println!("--- My Solution ---");
    let result = my_solution::Solution::sum_of_left_leaves(tree.clone());
    println!("  [3,9,20,null,null,15,7] -> {result} (expected 24)");

    println!("--- AI Solution ---");
    let result = ai_solution::Solution::sum_of_left_leaves(tree);
    println!("  [3,9,20,null,null,15,7] -> {result} (expected 24)");

    // Single node
    let single = node(1);
    println!(
        "  [1] -> {} (expected 0)",
        my_solution::Solution::sum_of_left_leaves(single)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_left_leaves {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use super::super::*;
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    let tree = node_with(3, node(9), node_with(20, node(15), node(7)));
                    assert_eq!(Solution::sum_of_left_leaves(tree), 24);
                }

                #[test]
                fn test_single_node() {
                    assert_eq!(Solution::sum_of_left_leaves(node(1)), 0);
                }

                #[test]
                fn test_empty() {
                    assert_eq!(Solution::sum_of_left_leaves(None), 0);
                }

                #[test]
                fn test_only_left_child() {
                    // root=1, left=2 (leaf)
                    let tree = node_with(1, node(2), None);
                    assert_eq!(Solution::sum_of_left_leaves(tree), 2);
                }

                #[test]
                fn test_only_right_child() {
                    // root=1, right=3 (leaf, but not left)
                    let tree = node_with(1, None, node(3));
                    assert_eq!(Solution::sum_of_left_leaves(tree), 0);
                }

                #[test]
                fn test_deep_left() {
                    // 1 -> left: 2 -> left: 3 (leaf, left)
                    let tree = node_with(1, node_with(2, node(3), None), None);
                    assert_eq!(Solution::sum_of_left_leaves(tree), 3);
                }
            }
        };
    }

    test_left_leaves!(my_tests, crate::my_solution::Solution);
    test_left_leaves!(ai_tests, crate::ai_solution::Solution);

    /// Build a complete binary tree of given depth for benchmarking.
    fn build_tree(depth: u32) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 0 {
            return None;
        }
        node_with(depth as i32, build_tree(depth - 1), build_tree(depth - 1))
    }

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let tree = build_tree(10); // 1023 nodes

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::sum_of_left_leaves(
                std::hint::black_box(tree.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::sum_of_left_leaves(
                std::hint::black_box(tree.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
