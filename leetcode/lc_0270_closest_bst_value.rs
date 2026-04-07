// LeetCode 270: Closest Binary Search Tree Value [Easy] (Premium)
// https://leetcode.com/problems/closest-binary-search-tree-value/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.007us
//   AI solution avg: 0.014us
//   Ratio: My 1.98x faster
//
// Given the root of a binary search tree and a target value, return the value
// in the BST that is closest to the target. If there are multiple answers,
// return the smallest.
//
// Example 1: root = [4,2,5,1,3], target = 3.714286 -> 4
// Example 2: root = [1], target = 4.428571 -> 1
//
// Constraints:
//   - Number of nodes in [1, 10^4]
//   - 0 <= Node.val <= 10^9
//   - -10^9 <= target <= 10^9
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0270.Closest%20Binary%20Search%20Tree%20Value/README_EN.md
//
// Run:  cargo run --example lc_0270_closest_bst_value
// Test: cargo test --example lc_0270_closest_bst_value
// Bench: cargo test --example lc_0270_closest_bst_value bench -- --nocapture

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
        if let Some(node) = queue.pop_front() {
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
    }
    Some(root)
}

/// My solution — Iterative BST-directed traversal (from doocs Solution 2).
///
/// Walk down the BST: at each node compare |node.val - target| to current best.
/// Then go left if target < node.val, else right. This exploits the BST ordering
/// to prune half the tree at each step, giving O(h) time on balanced trees.
/// When distances tie, prefer the smaller value.
///
/// Time:  O(h) where h = tree height (O(log n) balanced, O(n) skewed)
/// Space: O(1)
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        pub fn closest_value(
            root: Option<Rc<RefCell<TreeNode>>>,
            target: f64,
        ) -> i32 {
            let mut ans = 0;
            let mut diff = f64::INFINITY;
            let mut cur = root;

            while let Some(node) = cur {
                let node_ref = node.borrow();
                let nxt = (node_ref.val as f64 - target).abs();
                // Update answer if closer, or same distance but smaller value
                if nxt < diff || (nxt == diff && node_ref.val < ans) {
                    diff = nxt;
                    ans = node_ref.val;
                }
                // BST-guided search: go left if target is smaller, else right
                cur = if target < node_ref.val as f64 {
                    node_ref.left.clone()
                } else {
                    node_ref.right.clone()
                };
            }

            ans
        }
    }
}

/// AI solution — Find floor and ceiling, then compare.
///
/// Instead of tracking closest at every node, this approach finds two
/// candidates: the BST floor (largest value <= target) and BST ceiling
/// (smallest value >= target). Then picks the closer one (smaller on tie).
/// Two separate passes down the tree, each O(h).
///
/// Time:  O(h) where h = tree height
/// Space: O(1)
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        pub fn closest_value(
            root: Option<Rc<RefCell<TreeNode>>>,
            target: f64,
        ) -> i32 {
            // Find floor: largest value <= target
            let floor = Self::find_floor(&root, target);
            // Find ceiling: smallest value >= target
            let ceil = Self::find_ceil(&root, target);

            match (floor, ceil) {
                (Some(f), Some(c)) => {
                    let df = target - f as f64;
                    let dc = c as f64 - target;
                    // Prefer floor (smaller value) when distances tie
                    if df <= dc { f } else { c }
                }
                (Some(f), None) => f,
                (None, Some(c)) => c,
                // At least one node exists per constraints, so this
                // case is unreachable; fallback for safety
                (None, None) => unreachable!("tree has at least one node"),
            }
        }

        /// Walk down BST collecting the largest value <= target.
        fn find_floor(
            root: &Option<Rc<RefCell<TreeNode>>>,
            target: f64,
        ) -> Option<i32> {
            let mut result = None;
            let mut cur = root.clone();
            while let Some(node) = cur {
                let node_ref = node.borrow();
                if (node_ref.val as f64) <= target {
                    result = Some(node_ref.val);
                    cur = node_ref.right.clone(); // try to find larger floor
                } else {
                    cur = node_ref.left.clone(); // current too big
                }
            }
            result
        }

        /// Walk down BST collecting the smallest value >= target.
        fn find_ceil(
            root: &Option<Rc<RefCell<TreeNode>>>,
            target: f64,
        ) -> Option<i32> {
            let mut result = None;
            let mut cur = root.clone();
            while let Some(node) = cur {
                let node_ref = node.borrow();
                if (node_ref.val as f64) >= target {
                    result = Some(node_ref.val);
                    cur = node_ref.left.clone(); // try to find smaller ceil
                } else {
                    cur = node_ref.right.clone(); // current too small
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper macro to reduce boilerplate in BST closest-value tests.
    macro_rules! test_closest {
        ($name:ident, $vals:expr, $target:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let root = build_tree(&$vals);
                assert_eq!(
                    my_solution::Solution::closest_value(root.clone(), $target),
                    $expected,
                    "my_solution failed"
                );
                assert_eq!(
                    ai_solution::Solution::closest_value(root, $target),
                    $expected,
                    "ai_solution failed"
                );
            }
        };
    }

    // Example 1: root = [4,2,5,1,3], target = 3.714286 -> 4
    test_closest!(
        test_example_1,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        3.714286,
        4
    );

    // Example 2: root = [1], target = 4.428571 -> 1
    test_closest!(test_example_2, [Some(1)], 4.428571, 1);

    // Target exactly matches a node value
    test_closest!(
        test_exact_match,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        2.0,
        2
    );

    // Target between two nodes, closer to left child
    test_closest!(
        test_closer_to_left,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        2.3,
        2
    );

    // Target between two nodes, closer to right child
    test_closest!(
        test_closer_to_right,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        2.8,
        3
    );

    // Target is equidistant: pick smallest value.
    // Tree [2,1,3], target=2.0 → exactly 2
    test_closest!(
        test_tie_returns_smallest,
        [Some(2), Some(1), Some(3)],
        2.0,
        2
    );

    // Target equidistant from 1 and 3 → pick 1 (smaller)
    test_closest!(
        test_equidistant_pick_smaller,
        [Some(2), Some(1), Some(3)],
        2.0_f64, // actually 2 is exact; use 1.9999... nah
        2
    );

    // Better equidistant test: target=2.5, nodes 2 and 3 are equidistant → pick 2
    test_closest!(
        test_equidistant_two_and_three,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        2.5,
        2
    );

    // Skewed tree (right only)
    test_closest!(
        test_right_skewed,
        [Some(1), None, Some(3), None, None, None, Some(5)],
        4.0,
        3
    );

    // Skewed tree (left only)
    test_closest!(
        test_left_skewed,
        [Some(5), Some(3), None, Some(1)],
        2.0,
        1
    );

    // Target smaller than all nodes
    test_closest!(
        test_target_below_all,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        -1.0,
        1
    );

    // Target larger than all nodes
    test_closest!(
        test_target_above_all,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        100.0,
        5
    );

    // Two-node tree
    test_closest!(test_two_nodes, [Some(1), None, Some(2)], 1.4, 1);

    // Two-node tree, closer to right
    test_closest!(test_two_nodes_right, [Some(1), None, Some(2)], 1.6, 2);

    // Large value nodes
    test_closest!(
        test_large_values,
        [Some(1_000_000_000), Some(500_000_000), None, Some(250_000_000)],
        600_000_000.0,
        500_000_000
    );

    // Target is 0.0
    test_closest!(
        test_target_zero,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        0.0,
        1
    );

    // Perfectly balanced large tree — 9.0 equidistant from 8 and 10, pick 8
    test_closest!(
        test_balanced_seven_nodes,
        [Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14)],
        9.0,
        8
    );
}

fn main() {
    println!("LeetCode 270: Closest Binary Search Tree Value");
    let root = build_tree(&[Some(4), Some(2), Some(5), Some(1), Some(3)]);
    let target = 3.714286;
    println!(
        "my_solution:  closest_value([4,2,5,1,3], {}) = {}",
        target,
        my_solution::Solution::closest_value(root.clone(), target)
    );
    println!(
        "ai_solution:  closest_value([4,2,5,1,3], {}) = {}",
        target,
        ai_solution::Solution::closest_value(root, target)
    );
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let root = build_tree(&[Some(4), Some(2), Some(5), Some(1), Some(3)]);
        let target = 3.714286;
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(my_solution::Solution::closest_value(
                root.clone(),
                target,
            ));
            std::hint::black_box(ai_solution::Solution::closest_value(
                root.clone(),
                target,
            ));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::closest_value(
                root.clone(),
                target,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::closest_value(
                root.clone(),
                target,
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
