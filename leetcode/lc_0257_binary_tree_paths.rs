// LeetCode 257: Binary Tree Paths [Easy]
// https://leetcode.com/problems/binary-tree-paths/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 1.282us
//   AI solution avg: 1.621us
//   Ratio: My 1.26x faster
//
// Given the root of a binary tree, return all root-to-leaf paths in any order.
// A leaf is a node with no children.
//
// Example 1: root = [1,2,3,null,5] -> ["1->2->5","1->3"]
// Example 2: root = [1]            -> ["1"]
//
// Constraints:
//   - Number of nodes in [1, 100]
//   - -100 <= Node.val <= 100
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0257.Binary%20Tree%20Paths/README_EN.md
//
// Run:  cargo run --example lc_0257_binary_tree_paths
// Test: cargo test --example lc_0257_binary_tree_paths
// Bench: cargo test --example lc_0257_binary_tree_paths bench -- --nocapture

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
        TreeNode {
            val,
            left: None,
            right: None,
        }
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

/// My solution — DFS with backtracking (from doocs Python).
///
/// Collect node values in a Vec<String>, join with "->" at leaves,
/// pop on backtrack. Classic preorder traversal with path recording.
///
/// Time: O(n^2) — each leaf path can be up to O(n) to join
/// Space: O(n) — recursion depth + path storage
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return all root-to-leaf paths as strings.
        pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
            let mut ans: Vec<String> = Vec::new();
            let mut t: Vec<String> = Vec::new();
            Self::dfs(&root, &mut t, &mut ans);
            ans
        }

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, t: &mut Vec<String>, ans: &mut Vec<String>) {
            let Some(n) = node else { return };
            let n = n.borrow();
            t.push(n.val.to_string());
            if n.left.is_none() && n.right.is_none() {
                // Leaf: join the path and record
                ans.push(t.join("->"));
            } else {
                Self::dfs(&n.left, t, ans);
                Self::dfs(&n.right, t, ans);
            }
            t.pop();
        }
    }
}

/// AI solution — DFS passing owned String path (no backtracking).
///
/// Instead of a mutable Vec, pass the path string by value. At each
/// node, append "->val" (or just "val" for root). At leaves, push
/// the completed string. No explicit backtracking needed since String
/// is consumed per branch via cloning.
///
/// Time: O(n^2)
/// Space: O(n^2) — each recursive call owns a path copy
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return all root-to-leaf paths as strings.
        pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
            let mut ans: Vec<String> = Vec::new();
            if let Some(r) = root {
                Self::dfs(&r, String::new(), &mut ans);
            }
            ans
        }

        fn dfs(node: &Rc<RefCell<TreeNode>>, path: String, ans: &mut Vec<String>) {
            let n = node.borrow();
            // Build current path segment
            let current = if path.is_empty() {
                n.val.to_string()
            } else {
                format!("{}->{}", path, n.val)
            };
            if n.left.is_none() && n.right.is_none() {
                ans.push(current);
                return;
            }
            if let Some(ref left) = n.left {
                Self::dfs(left, current.clone(), ans);
            }
            if let Some(ref right) = n.right {
                Self::dfs(right, current, ans);
            }
        }
    }
}

fn main() {
    let tree = build_tree(&[Some(1), Some(2), Some(3), None, Some(5)]);
    println!(
        "my: {:?}",
        my_solution::Solution::binary_tree_paths(tree.clone())
    );
    println!("ai: {:?}", ai_solution::Solution::binary_tree_paths(tree));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    macro_rules! test_binary_tree_paths {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    let tree = build_tree(&[Some(1), Some(2), Some(3), None, Some(5)]);
                    let result: HashSet<String> =
                        Sol::binary_tree_paths(tree).into_iter().collect();
                    let expected: HashSet<String> =
                        ["1->2->5", "1->3"].iter().map(|s| s.to_string()).collect();
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_single_node() {
                    let tree = build_tree(&[Some(1)]);
                    assert_eq!(Sol::binary_tree_paths(tree), vec!["1"]);
                }

                #[test]
                fn test_left_skewed() {
                    let tree = build_tree(&[Some(1), Some(2), None, Some(3)]);
                    assert_eq!(Sol::binary_tree_paths(tree), vec!["1->2->3"]);
                }

                #[test]
                fn test_right_skewed() {
                    let tree = build_tree(&[Some(1), None, Some(2), None, Some(3)]);
                    assert_eq!(Sol::binary_tree_paths(tree), vec!["1->2->3"]);
                }

                #[test]
                fn test_full_tree() {
                    // Complete binary tree: 1, 2, 3, 4, 5, 6, 7
                    let tree = build_tree(&[
                        Some(1),
                        Some(2),
                        Some(3),
                        Some(4),
                        Some(5),
                        Some(6),
                        Some(7),
                    ]);
                    let result: HashSet<String> =
                        Sol::binary_tree_paths(tree).into_iter().collect();
                    let expected: HashSet<String> = ["1->2->4", "1->2->5", "1->3->6", "1->3->7"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect();
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_negative_values() {
                    let tree = build_tree(&[Some(-10), Some(5), Some(-3)]);
                    let result: HashSet<String> =
                        Sol::binary_tree_paths(tree).into_iter().collect();
                    let expected: HashSet<String> = ["-10->5", "-10->-3"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect();
                    assert_eq!(result, expected);
                }
            }
        };
    }

    test_binary_tree_paths!(my, crate::my_solution::Solution);
    test_binary_tree_paths!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a balanced tree of 15 nodes
        let vals: Vec<Option<i32>> = (1..=15).map(Some).collect();
        let tree = build_tree(&vals);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::binary_tree_paths(
                tree.clone(),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::binary_tree_paths(
                tree.clone(),
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
