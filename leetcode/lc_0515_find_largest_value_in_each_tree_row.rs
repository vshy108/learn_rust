// LeetCode 515: Find Largest Value in Each Tree Row [Medium]
// https://leetcode.com/problems/find-largest-value-in-each-tree-row/
//
// Given the root of a binary tree, return an array of the largest value in
// each row of the tree (0-indexed).
//
// Examples:
//   [1,3,2,5,3,null,9] -> [1,3,9]
//   [1,2,3]             -> [1,3]
//
// Constraints:
//   0 <= number of nodes <= 10^4
//   -2^31 <= Node.val <= 2^31 - 1
//
// Run:  cargo run --example lc_0515_find_largest_value_in_each_tree_row
// Test: cargo test --example lc_0515_find_largest_value_in_each_tree_row
// Bench: cargo test --example lc_0515_find_largest_value_in_each_tree_row bench -- --nocapture
//
// First benchmark (100K iterations, 2026-04-08):
//   My solution: avg 182ns
//   AI solution: avg 70ns
//   AI is 2.60x faster
//   Reason: DFS avoids VecDeque allocation overhead and has better cache
//   locality from recursive stack frames vs heap-allocated queue nodes.

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

/// Helper to build a tree from a level-order list (None = null node).
fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;
    while i < vals.len() {
        if let Some(node) = queue.pop_front() {
            // Left child
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
            }
            i += 1;
            // Right child
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
            }
            i += 1;
        }
    }
    Some(root)
}

/// BFS level-order traversal — track maximum per level using VecDeque.
///
/// Adapted from doocs/leetcode Rust solution.
///
/// Time:  O(n)
/// Space: O(n)
mod my_solution {
    use super::*;
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Return the largest value in each row of the binary tree.
        ///
        /// # Arguments
        /// * `root` - root of the binary tree
        ///
        /// # Returns
        /// Vec of the max value per level
        pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut ans = Vec::new();
            let mut q = VecDeque::new();
            if let Some(r) = root {
                q.push_back(r);
            }
            while !q.is_empty() {
                let mut level_max = i32::MIN;
                // Process all nodes at the current level
                for _ in 0..q.len() {
                    let node = q.pop_front().unwrap();
                    let node_ref = node.borrow();
                    level_max = level_max.max(node_ref.val);
                    if let Some(ref left) = node_ref.left {
                        q.push_back(Rc::clone(left));
                    }
                    if let Some(ref right) = node_ref.right {
                        q.push_back(Rc::clone(right));
                    }
                }
                ans.push(level_max);
            }
            ans
        }
    }
}

/// DFS recursive approach — maintain a result vector indexed by depth.
///
/// At each node, if depth == result length, push the value; otherwise
/// update the existing entry with the max.
///
/// Time:  O(n)
/// Space: O(h) where h is tree height (recursion stack)
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the largest value in each row of the binary tree.
        ///
        /// # Arguments
        /// * `root` - root of the binary tree
        ///
        /// # Returns
        /// Vec of the max value per level
        pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut res = Vec::new();
            Self::dfs(&root, 0, &mut res);
            res
        }

        /// DFS helper that tracks current depth and updates the result vector.
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, res: &mut Vec<i32>) {
            if let Some(n) = node {
                let n_ref = n.borrow();
                if res.len() == depth {
                    // First node at this depth — initialize the level max
                    res.push(n_ref.val);
                } else {
                    // Update the max for this level
                    res[depth] = res[depth].max(n_ref.val);
                }
                Self::dfs(&n_ref.left, depth + 1, res);
                Self::dfs(&n_ref.right, depth + 1, res);
            }
        }
    }
}

fn main() {
    let tree1 = build_tree(&[Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)]);
    let tree2 = build_tree(&[Some(1), Some(2), Some(3)]);

    println!(
        "my  [1,3,2,5,3,null,9]: {:?}",
        my_solution::Solution::largest_values(tree1.clone())
    );
    println!(
        "ai  [1,3,2,5,3,null,9]: {:?}",
        ai_solution::Solution::largest_values(tree1)
    );
    println!(
        "my  [1,2,3]: {:?}",
        my_solution::Solution::largest_values(tree2.clone())
    );
    println!(
        "ai  [1,2,3]: {:?}",
        ai_solution::Solution::largest_values(tree2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! largest_values_tests {
        ($mod:ident, $solution:path) => {
            mod $mod {
                use super::*;

                #[test]
                fn test_example1() {
                    let root =
                        build_tree(&[Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)]);
                    assert_eq!(<$solution>::largest_values(root), vec![1, 3, 9]);
                }

                #[test]
                fn test_example2() {
                    let root = build_tree(&[Some(1), Some(2), Some(3)]);
                    assert_eq!(<$solution>::largest_values(root), vec![1, 3]);
                }

                #[test]
                fn test_empty() {
                    assert_eq!(<$solution>::largest_values(None), Vec::<i32>::new());
                }

                #[test]
                fn test_single_node() {
                    let root = build_tree(&[Some(42)]);
                    assert_eq!(<$solution>::largest_values(root), vec![42]);
                }

                #[test]
                fn test_negative_values() {
                    let root = build_tree(&[Some(-1), Some(-3), Some(-2)]);
                    assert_eq!(<$solution>::largest_values(root), vec![-1, -2]);
                }

                #[test]
                fn test_left_skewed() {
                    let root = build_tree(&[Some(1), Some(2), None, Some(3)]);
                    assert_eq!(<$solution>::largest_values(root), vec![1, 2, 3]);
                }
            }
        };
    }

    largest_values_tests!(my_tests, my_solution::Solution);
    largest_values_tests!(ai_tests, ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let root = build_tree(&[Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)]);

        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::largest_values(std::hint::black_box(
                root.clone(),
            )));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::largest_values(std::hint::black_box(
                root.clone(),
            )));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur / iterations;
        let ai_avg = ai_dur / iterations;
        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("  My solution (BFS): {my_avg:?} avg");
        println!("  AI solution (DFS): {ai_avg:?} avg");
        if ai_avg < my_avg {
            println!(
                "  AI is {:.2}x faster",
                my_avg.as_nanos() as f64 / ai_avg.as_nanos() as f64
            );
        } else {
            println!(
                "  My is {:.2}x faster",
                ai_avg.as_nanos() as f64 / my_avg.as_nanos() as f64
            );
        }
    }
}
