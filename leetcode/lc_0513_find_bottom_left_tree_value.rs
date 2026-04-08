// LeetCode 513: Find Bottom Left Tree Value [Medium]
// https://leetcode.com/problems/find-bottom-left-tree-value/
//
// Given the root of a binary tree, return the leftmost value in the last row
// of the tree.
//
// Examples:
//   [2,1,3]                         -> 1
//   [1,2,3,4,null,5,6,null,null,7]  -> 7
//
// Constraints:
//   1 <= number of nodes <= 10^4
//   -2^31 <= Node.val <= 2^31 - 1
//
// Run:  cargo run --example lc_0513_find_bottom_left_tree_value
// Test: cargo test --example lc_0513_find_bottom_left_tree_value
// Bench: cargo test --example lc_0513_find_bottom_left_tree_value bench -- --nocapture
//
// First benchmark (100K iterations, 2026-04-08):
//   My solution: avg 2.32us
//   AI solution: avg 2.09us
//   AI is 1.11x faster
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

/// My solution — BFS level-order traversal.
///
/// Process each level left to right. The first element of the last level
/// is the answer. From doocs/leetcode Rust solution.
///
/// Time:  O(n)
/// Space: O(n) for the queue
mod my_solution {
    use super::*;
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Return the leftmost value in the last row of the tree.
        ///
        /// # Arguments
        /// * `root` - root of the binary tree (guaranteed non-empty)
        ///
        /// # Returns
        /// Leftmost value in the deepest level
        pub fn find_bottom_left_value(root: Node) -> i32 {
            let mut queue = VecDeque::new();
            if let Some(r) = root {
                queue.push_back(r);
            }
            let mut ans = 0;

            while !queue.is_empty() {
                // First node of this level is the leftmost
                ans = queue.front().expect("non-empty").borrow().val;
                let size = queue.len();
                for _ in 0..size {
                    if let Some(node) = queue.pop_front() {
                        let node = node.borrow();
                        if let Some(ref left) = node.left {
                            queue.push_back(Rc::clone(left));
                        }
                        if let Some(ref right) = node.right {
                            queue.push_back(Rc::clone(right));
                        }
                    }
                }
            }
            ans
        }
    }
}

/// AI solution — DFS tracking maximum depth.
///
/// Recurse left-first. Track the maximum depth seen and the corresponding
/// value. Since we go left first, the first node at each new max depth
/// is always the leftmost.
///
/// Time:  O(n)
/// Space: O(h) recursion stack
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the leftmost value in the last row using DFS.
        ///
        /// # Arguments
        /// * `root` - root of the binary tree
        ///
        /// # Returns
        /// Leftmost value in the deepest level
        pub fn find_bottom_left_value(root: Node) -> i32 {
            let mut ans = 0;
            let mut max_depth = 0;

            fn dfs(node: &Node, depth: i32, ans: &mut i32, max_depth: &mut i32) {
                if let Some(n) = node {
                    let n = n.borrow();
                    // Visit left first so leftmost is found first at each depth
                    dfs(&n.left, depth + 1, ans, max_depth);
                    dfs(&n.right, depth + 1, ans, max_depth);
                    if depth > *max_depth {
                        *max_depth = depth;
                        *ans = n.val;
                    }
                }
            }

            dfs(&root, 1, &mut ans, &mut max_depth);
            ans
        }
    }
}

fn main() {
    let root = build_tree(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        Some(5),
        Some(6),
        None,
        None,
        Some(7),
    ]);
    let my = my_solution::Solution::find_bottom_left_value(root.clone());
    let ai = ai_solution::Solution::find_bottom_left_value(root);
    println!("my: {my}, ai: {ai}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_bottom_left {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_example1() {
                    let root = build_tree(&[Some(2), Some(1), Some(3)]);
                    assert_eq!(<$solution>::find_bottom_left_value(root), 1);
                }

                #[test]
                fn test_example2() {
                    let root = build_tree(&[
                        Some(1),
                        Some(2),
                        Some(3),
                        Some(4),
                        None,
                        Some(5),
                        Some(6),
                        None,
                        None,
                        Some(7),
                    ]);
                    assert_eq!(<$solution>::find_bottom_left_value(root), 7);
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(42)]);
                    assert_eq!(<$solution>::find_bottom_left_value(root), 42);
                }

                #[test]
                fn test_left_skew() {
                    let root = build_tree(&[Some(1), Some(2), None, Some(3)]);
                    assert_eq!(<$solution>::find_bottom_left_value(root), 3);
                }
            }
        };
    }

    test_bottom_left!(my, crate::my_solution::Solution);
    test_bottom_left!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=31).map(Some).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::find_bottom_left_value(
                std::hint::black_box(root),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::find_bottom_left_value(
                std::hint::black_box(root),
            ));
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
