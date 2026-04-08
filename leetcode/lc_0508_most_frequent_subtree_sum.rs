// LeetCode 508: Most Frequent Subtree Sum [Medium]
// https://leetcode.com/problems/most-frequent-subtree-sum/
//
// Given the root of a binary tree, return the most frequent subtree sum.
// If there is a tie, return all values with the highest frequency (any order).
//
// The subtree sum of a node = sum of all node values in the subtree rooted at
// that node (including itself).
//
// Examples:
//   [5,2,-3]  -> [2,-3,4]   (sums: left=2, right=-3, root=5+2-3=4, all freq 1)
//   [5,2,-5]  -> [2]        (sums: left=2, right=-5, root=5+2-5=2, freq of 2 is 2)
//
// Constraints:
//   1 <= number of nodes <= 10^4
//   -10^5 <= Node.val <= 10^5
//
// Run:  cargo run --example lc_0508_most_frequent_subtree_sum
// Test: cargo test --example lc_0508_most_frequent_subtree_sum
// Bench: cargo test --example lc_0508_most_frequent_subtree_sum bench -- --nocapture
//
// First benchmark (100K iterations, 2026-04-08):
//   My solution: avg 750ns
//   AI solution: avg 883ns
//   My is 1.18x faster
//   Reason: single-pass DFS tracks max frequency inline, avoiding a second
//   iteration over the HashMap and an intermediate Vec allocation.

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

/// My solution — DFS with HashMap, track max frequency during traversal.
///
/// Adapted from doocs/leetcode Rust solution. Post-order DFS computes each
/// subtree sum and updates a frequency HashMap. Track max frequency inline.
///
/// Time:  O(n)
/// Space: O(n)
mod my_solution {
    use super::*;
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return the most frequent subtree sum(s).
        ///
        /// # Arguments
        /// * `root` - root of the binary tree
        ///
        /// # Returns
        /// Vec of subtree sums with highest frequency
        pub fn find_frequent_tree_sum(root: Node) -> Vec<i32> {
            fn dfs(node: &Node, cnt: &mut HashMap<i32, i32>, mx: &mut i32) -> i32 {
                if let Some(n) = node {
                    let n = n.borrow();
                    let l = dfs(&n.left, cnt, mx);
                    let r = dfs(&n.right, cnt, mx);
                    let s = l + r + n.val;
                    let entry = cnt.entry(s).or_insert(0);
                    *entry += 1;
                    *mx = (*mx).max(*entry);
                    s
                } else {
                    0
                }
            }

            let mut cnt = HashMap::new();
            let mut mx = 0;
            dfs(&root, &mut cnt, &mut mx);

            cnt.into_iter()
                .filter(|&(_, v)| v == mx)
                .map(|(k, _)| k)
                .collect()
        }
    }
}

/// AI solution — DFS collecting all sums into a Vec, then count frequencies.
///
/// First pass: collect all subtree sums into a Vec.
/// Second pass: count frequencies and return the most common.
///
/// Time:  O(n)
/// Space: O(n)
mod ai_solution {
    use super::*;
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return the most frequent subtree sum(s) using two-pass approach.
        ///
        /// # Arguments
        /// * `root` - root of the binary tree
        ///
        /// # Returns
        /// Vec of subtree sums with highest frequency
        pub fn find_frequent_tree_sum(root: Node) -> Vec<i32> {
            fn subtree_sum(node: &Node, sums: &mut Vec<i32>) -> i32 {
                if let Some(n) = node {
                    let n = n.borrow();
                    let s = n.val + subtree_sum(&n.left, sums) + subtree_sum(&n.right, sums);
                    sums.push(s);
                    s
                } else {
                    0
                }
            }

            let mut sums = Vec::new();
            subtree_sum(&root, &mut sums);

            // Count frequencies
            let mut cnt: HashMap<i32, i32> = HashMap::new();
            for &s in &sums {
                *cnt.entry(s).or_insert(0) += 1;
            }

            let mx = cnt.values().cloned().max().unwrap_or(0);
            cnt.into_iter()
                .filter(|&(_, v)| v == mx)
                .map(|(k, _)| k)
                .collect()
        }
    }
}

fn main() {
    let root = build_tree(&[Some(5), Some(2), Some(-3)]);
    let my = my_solution::Solution::find_frequent_tree_sum(root.clone());
    let ai = ai_solution::Solution::find_frequent_tree_sum(root);
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort_unstable();
        v
    }

    macro_rules! test_freq_sum {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{build_tree, sorted};

                #[test]
                fn test_example1() {
                    let root = build_tree(&[Some(5), Some(2), Some(-3)]);
                    assert_eq!(
                        sorted(<$solution>::find_frequent_tree_sum(root)),
                        vec![-3, 2, 4]
                    );
                }

                #[test]
                fn test_example2() {
                    let root = build_tree(&[Some(5), Some(2), Some(-5)]);
                    assert_eq!(sorted(<$solution>::find_frequent_tree_sum(root)), vec![2]);
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(5)]);
                    assert_eq!(sorted(<$solution>::find_frequent_tree_sum(root)), vec![5]);
                }
            }
        };
    }

    test_freq_sum!(my, crate::my_solution::Solution);
    test_freq_sum!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals = vec![
            Some(5),
            Some(2),
            Some(-3),
            Some(1),
            Some(3),
            Some(-4),
            Some(7),
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::find_frequent_tree_sum(
                std::hint::black_box(root),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::find_frequent_tree_sum(
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
