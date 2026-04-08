// LeetCode 501: Find Mode in Binary Search Tree [Easy]
// https://leetcode.com/problems/find-mode-in-binary-search-tree/
//
// Given the root of a binary search tree (BST) with duplicates, return all
// the mode(s) (i.e., the most frequently occurred element) in it.
// If the tree has more than one mode, return them in any order.
//
// BST definition:
//   - Left subtree contains only nodes with keys <= node's key
//   - Right subtree contains only nodes with keys >= node's key
//   - Both subtrees must also be BSTs
//
// Examples:
//   [1,null,2,2] -> [2]
//   [0]          -> [0]
//
// Constraints:
//   1 <= number of nodes <= 10^4
//   -10^5 <= Node.val <= 10^5
//
// Follow up: Could you do it without extra space? (implicit recursion stack
// doesn't count)
//
// Run:  cargo run --example lc_0501_find_mode_in_binary_search_tree
// Test: cargo test --example lc_0501_find_mode_in_binary_search_tree
// Bench: cargo test --example lc_0501_find_mode_in_binary_search_tree bench -- --nocapture
//
// First benchmark (100K iterations, 2026-04-08):
//   My solution: avg 567ns
//   AI solution: avg 750ns
//   My is 1.32x faster
//   Reason: in-order traversal exploits BST sorted property with O(1) extra space;
//   HashMap approach does unnecessary hashing overhead.

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

/// My solution — in-order traversal with O(1) extra space (Morris-like tracking).
///
/// Uses the BST property: in-order gives sorted values. Track prev value,
/// current count, and max count to find modes without a HashMap.
///
/// Time:  O(n)
/// Space: O(1) extra (not counting output and recursion stack)
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return all modes (most frequent elements) in the BST.
        ///
        /// # Arguments
        /// * `root` - root of the BST
        ///
        /// # Returns
        /// Vec of mode values (any order)
        pub fn find_mode(root: Node) -> Vec<i32> {
            let mut result = Vec::new();
            let mut prev: Option<i32> = None;
            let mut cnt = 0i32;
            let mut mx = 0i32;

            // In-order traversal: values come out sorted for a BST
            fn dfs(
                node: &Node,
                prev: &mut Option<i32>,
                cnt: &mut i32,
                mx: &mut i32,
                result: &mut Vec<i32>,
            ) {
                if let Some(n) = node {
                    let n = n.borrow();
                    dfs(&n.left, prev, cnt, mx, result);

                    // Count consecutive equal values in sorted order
                    *cnt = if *prev == Some(n.val) { *cnt + 1 } else { 1 };
                    if *cnt > *mx {
                        // New max — reset result list
                        result.clear();
                        result.push(n.val);
                        *mx = *cnt;
                    } else if *cnt == *mx {
                        result.push(n.val);
                    }
                    *prev = Some(n.val);

                    dfs(&n.right, prev, cnt, mx, result);
                }
            }

            dfs(&root, &mut prev, &mut cnt, &mut mx, &mut result);
            result
        }
    }
}

/// AI solution — HashMap-based frequency counting via DFS.
///
/// Traverses the entire tree, counts each value in a HashMap, then collects
/// those with the maximum frequency.
///
/// Time:  O(n)
/// Space: O(n) for the HashMap
mod ai_solution {
    use super::*;
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return all modes using HashMap frequency counting.
        ///
        /// # Arguments
        /// * `root` - root of the BST
        ///
        /// # Returns
        /// Vec of mode values (any order)
        pub fn find_mode(root: Node) -> Vec<i32> {
            let mut cnt: HashMap<i32, i32> = HashMap::new();
            let mut mx = 0;

            fn dfs(node: &Node, cnt: &mut HashMap<i32, i32>, mx: &mut i32) {
                if let Some(n) = node {
                    let n = n.borrow();
                    let entry = cnt.entry(n.val).or_insert(0);
                    *entry += 1;
                    *mx = (*mx).max(*entry);
                    dfs(&n.left, cnt, mx);
                    dfs(&n.right, cnt, mx);
                }
            }

            dfs(&root, &mut cnt, &mut mx);
            cnt.into_iter()
                .filter(|&(_, v)| v == mx)
                .map(|(k, _)| k)
                .collect()
        }
    }
}

fn main() {
    let root = build_tree(&[Some(1), None, Some(2), Some(2)]);
    let my = my_solution::Solution::find_mode(root.clone());
    let ai = ai_solution::Solution::find_mode(root);
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    /// Helper to normalize results for comparison (sort them).
    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort_unstable();
        v
    }

    macro_rules! test_find_mode {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{build_tree, sorted};

                #[test]
                fn test_example1() {
                    let root = build_tree(&[Some(1), None, Some(2), Some(2)]);
                    assert_eq!(sorted(<$solution>::find_mode(root)), vec![2]);
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(0)]);
                    assert_eq!(sorted(<$solution>::find_mode(root)), vec![0]);
                }

                #[test]
                fn test_multiple_modes() {
                    // BST: 1 -> left: 1, right: 2 -> right: 2
                    // Vals in-order: 1, 1, 2, 2 => modes are [1, 2]
                    let root = build_tree(&[Some(1), Some(1), Some(2), None, None, None, Some(2)]);
                    assert_eq!(sorted(<$solution>::find_mode(root)), vec![1, 2]);
                }

                #[test]
                fn test_all_same() {
                    let root = build_tree(&[Some(5), Some(5), Some(5)]);
                    assert_eq!(sorted(<$solution>::find_mode(root)), vec![5]);
                }
            }
        };
    }

    test_find_mode!(my, crate::my_solution::Solution);
    test_find_mode!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals = vec![
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            Some(5),
            Some(7),
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::find_mode(
                std::hint::black_box(root),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::find_mode(
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
