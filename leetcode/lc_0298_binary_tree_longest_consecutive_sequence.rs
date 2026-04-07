// LeetCode 298: Binary Tree Longest Consecutive Sequence [Medium] (Premium)
// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 0.476us
//   AI solution avg: 0.390us
//   Ratio: AI 1.22x faster
//
// Given the root of a binary tree, return the length of the longest consecutive
// sequence path (values increase by one, parent to child only — no going up).
//
// Example 1: root = [1,null,3,2,4,null,null,null,5] -> 3 (path: 3->4->5)
// Example 2: root = [2,null,3,2,null,1] -> 2 (path: 2->3)
//
// Constraints:
//   - Number of nodes in [1, 3 * 10^4]
//   - -3*10^4 <= Node.val <= 3*10^4
//
// Run:  cargo run --example lc_0298_binary_tree_longest_consecutive_sequence
// Test: cargo test --example lc_0298_binary_tree_longest_consecutive_sequence
// Bench: cargo test --example lc_0298_binary_tree_longest_consecutive_sequence bench -- --nocapture

use std::cell::RefCell;
use std::collections::VecDeque;
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
        TreeNode { val, left: None, right: None }
    }
}

type Node = Option<Rc<RefCell<TreeNode>>>;

fn build_tree(vals: &[Option<i32>]) -> Node {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    let mut i = 1;
    while i < vals.len() {
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

struct Solution;

/// My solution — DFS returning longest consecutive from this node downward (doocs).
///
/// Recurse into left and right children. If child.val == parent.val + 1,
/// extend the consecutive count, otherwise reset to 1. Track global max.
///
/// Time:  O(n)
/// Space: O(h) recursion depth
mod my_solution {
    use super::*;

    impl Solution {
        pub fn longest_consecutive_my(root: &Node) -> i32 {
            let mut ans = 0;
            Self::dfs_my(root, &mut ans);
            ans
        }

        fn dfs_my(node: &Node, ans: &mut i32) -> i32 {
            let Some(n) = node else { return 0 };
            let n = n.borrow();
            let mut l = Self::dfs_my(&n.left, ans) + 1;
            let mut r = Self::dfs_my(&n.right, ans) + 1;
            // Reset if child doesn't continue the sequence
            if let Some(ref left) = n.left {
                if left.borrow().val - n.val != 1 {
                    l = 1;
                }
            }
            if let Some(ref right) = n.right {
                if right.borrow().val - n.val != 1 {
                    r = 1;
                }
            }
            let t = l.max(r);
            *ans = (*ans).max(t);
            t
        }
    }
}

/// AI solution — DFS passing parent value and current streak down.
///
/// Pass the parent value and current streak length to each child.
/// If current node continues the sequence (val == parent + 1),
/// increment streak; otherwise reset to 1.
///
/// Time:  O(n)
/// Space: O(h)
mod ai_solution {
    use super::*;

    impl Solution {
        pub fn longest_consecutive_ai(root: &Node) -> i32 {
            let mut ans = 0;
            Self::dfs_ai(root, i32::MIN, 0, &mut ans);
            ans
        }

        fn dfs_ai(
            node: &Node,
            parent_val: i32,
            streak: i32,
            ans: &mut i32,
        ) {
            let Some(n) = node else { return };
            let n = n.borrow();
            let new_streak = if n.val == parent_val + 1 {
                streak + 1
            } else {
                1
            };
            *ans = (*ans).max(new_streak);
            Self::dfs_ai(&n.left, n.val, new_streak, ans);
            Self::dfs_ai(&n.right, n.val, new_streak, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_lcs {
        ($name:ident, $vals:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let root = build_tree(&$vals);
                assert_eq!(
                    Solution::longest_consecutive_my(&root),
                    $expected,
                    "my_solution failed"
                );
                assert_eq!(
                    Solution::longest_consecutive_ai(&root),
                    $expected,
                    "ai_solution failed"
                );
            }
        };
    }

    // Example 1: [1,null,3,2,4,null,null,null,5] -> 3
    test_lcs!(
        test_example_1,
        [Some(1), None, Some(3), Some(2), Some(4), None, None, None, Some(5)],
        3
    );

    // Example 2: [2,null,3,2,null,1] -> 2
    test_lcs!(
        test_example_2,
        [Some(2), None, Some(3), Some(2), None, Some(1)],
        2
    );

    // Single node
    test_lcs!(test_single, [Some(1)], 1);

    // All same values
    test_lcs!(
        test_same_values,
        [Some(5), Some(5), Some(5)],
        1
    );

    // Straight line left: 1->2->3->4
    test_lcs!(
        test_left_chain,
        [Some(1), Some(2), None, Some(3), None, Some(4)],
        4
    );

    // Decreasing (not consecutive going down)
    test_lcs!(
        test_decreasing,
        [Some(4), Some(3), None, Some(2)],
        1
    );

    // Consecutive in right subtree only
    test_lcs!(
        test_right_only,
        [Some(10), Some(5), Some(11), None, None, None, Some(12)],
        3
    );

    // Two separate consecutive paths, pick longer
    test_lcs!(
        test_two_paths,
        [Some(1), Some(2), Some(2), Some(3), None, Some(3), None, Some(4)],
        4
    );

    // Negative values
    test_lcs!(
        test_negatives,
        [Some(-3), Some(-2), None, Some(-1)],
        3
    );
}

fn main() {
    println!("LeetCode 298: Binary Tree Longest Consecutive Sequence");
    let root = build_tree(&[
        Some(1), None, Some(3), Some(2), Some(4), None, None, None, Some(5),
    ]);
    println!(
        "my_solution: {}",
        Solution::longest_consecutive_my(&root)
    );
    println!(
        "ai_solution: {}",
        Solution::longest_consecutive_ai(&root)
    );
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        // Build a right-skewed consecutive tree: 1->2->3->...->50
        let vals: Vec<Option<i32>> = (1..=50)
            .flat_map(|v| vec![None, Some(v)])
            .skip(1) // remove leading None
            .collect();
        let root = build_tree(&vals);
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(Solution::longest_consecutive_my(&root));
            std::hint::black_box(Solution::longest_consecutive_ai(&root));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::longest_consecutive_my(&root));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::longest_consecutive_ai(&root));
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
