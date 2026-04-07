// LeetCode 250: Count Univalue Subtrees [Medium] (Premium)
// https://leetcode.com/problems/count-univalue-subtrees/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.037us
//   AI solution avg: 0.036us
//   Ratio: AI 1.05x faster
//
// Given the root of a binary tree, return the number of uni-value subtrees.
// A uni-value subtree means all nodes of the subtree have the same value.
//
// Example 1: root = [5,1,5,5,5,null,5] -> 4
// Example 2: root = []                 -> 0
// Example 3: root = [5,5,5,5,5,null,5] -> 6
//
// Constraints:
//   - 0 <= number of nodes <= 1000
//   - -1000 <= Node.val <= 1000
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0250.Count%20Univalue%20Subtrees/README_EN.md
//
// Run:  cargo run --example lc_0250_count_univalue_subtrees
// Test: cargo test --example lc_0250_count_univalue_subtrees
// Bench: cargo test --example lc_0250_count_univalue_subtrees bench -- --nocapture

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
    /// Create a new leaf node.
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

/// Build a binary tree from a level-order array (None = null node).
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
                i += 1;
            }
            // Right child
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

/// My solution — post-order DFS with counter (translated from Python in doocs).
///
/// DFS returns true if the subtree rooted at this node is uni-value.
/// A node is uni-value if both subtrees are uni-value and both children
/// (if present) have the same value as this node.
///
/// Time: O(n)
/// Space: O(h) where h = tree height (recursion stack)
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Count uni-value subtrees.
        pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut ans = 0;
            Self::dfs(&root, &mut ans);
            ans
        }

        /// Return true if subtree at `node` is uni-value.
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> bool {
            let Some(rc) = node else {
                return true;
            };
            let n = rc.borrow();
            // Must evaluate both sides (don't short-circuit) to count all subtrees
            let l = Self::dfs(&n.left, ans);
            let r = Self::dfs(&n.right, ans);
            if !l || !r {
                return false;
            }
            // Check that children values match this node's value
            let a = n.left.as_ref().map_or(n.val, |c| c.borrow().val);
            let b = n.right.as_ref().map_or(n.val, |c| c.borrow().val);
            if a == n.val && b == n.val {
                *ans += 1;
                return true;
            }
            false
        }
    }
}

/// AI solution — post-order DFS returning Option<i32> (None = not uni-value).
///
/// Instead of a boolean + external counter, return Some(val) if the
/// subtree is uni-value with that value, or None if not. Count at
/// each successful node. This encodes the result more richly.
///
/// Time: O(n)
/// Space: O(h)
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Count uni-value subtrees.
        pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut count = 0;
            Self::dfs(&root, &mut count);
            count
        }

        /// Return Some(val) if subtree is uni-value with value `val`, else None.
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> Option<i32> {
            let Some(rc) = node else {
                // Empty subtree is trivially uni-value (wildcard value)
                return Some(i32::MIN);
            };
            let n = rc.borrow();
            let left_val = Self::dfs(&n.left, count);
            let right_val = Self::dfs(&n.right, count);
            // Check both children are uni-value and match this node's value
            // i32::MIN acts as a sentinel for "empty child, matches anything"
            let l_ok = left_val.is_some_and(|v| v == n.val || v == i32::MIN);
            let r_ok = right_val.is_some_and(|v| v == n.val || v == i32::MIN);
            if l_ok && r_ok {
                *count += 1;
                Some(n.val)
            } else {
                None
            }
        }
    }
}

fn main() {
    let tree = build_tree(&[Some(5), Some(1), Some(5), Some(5), Some(5), None, Some(5)]);
    println!(
        "my: {}",
        my_solution::Solution::count_unival_subtrees(tree.clone())
    );
    println!(
        "ai: {}",
        ai_solution::Solution::count_unival_subtrees(tree)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_count_unival {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;
                use super::super::build_tree;

                #[test]
                fn test_example_1() {
                    // [5,1,5,5,5,null,5] -> 4
                    let tree = build_tree(&[
                        Some(5), Some(1), Some(5), Some(5), Some(5), None, Some(5),
                    ]);
                    assert_eq!(Sol::count_unival_subtrees(tree), 4);
                }

                #[test]
                fn test_example_2_empty() {
                    assert_eq!(Sol::count_unival_subtrees(None), 0);
                }

                #[test]
                fn test_example_3_all_same() {
                    // [5,5,5,5,5,null,5] -> 6
                    let tree = build_tree(&[
                        Some(5), Some(5), Some(5), Some(5), Some(5), None, Some(5),
                    ]);
                    assert_eq!(Sol::count_unival_subtrees(tree), 6);
                }

                #[test]
                fn test_single_node() {
                    let tree = build_tree(&[Some(1)]);
                    assert_eq!(Sol::count_unival_subtrees(tree), 1);
                }

                #[test]
                fn test_two_same() {
                    // [1, 1] -> both are uni-value subtrees
                    let tree = build_tree(&[Some(1), Some(1)]);
                    assert_eq!(Sol::count_unival_subtrees(tree), 2);
                }

                #[test]
                fn test_two_different() {
                    // [1, 2] -> only the leaf 2 is uni-value
                    let tree = build_tree(&[Some(1), Some(2)]);
                    assert_eq!(Sol::count_unival_subtrees(tree), 1);
                }

                #[test]
                fn test_left_chain() {
                    // [1, 1, null, 1] -> all 1s, all 3 are uni-value
                    let tree = build_tree(&[Some(1), Some(1), None, Some(1)]);
                    assert_eq!(Sol::count_unival_subtrees(tree), 3);
                }
            }
        };
    }

    test_count_unival!(my, crate::my_solution::Solution);
    test_count_unival!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let tree = build_tree(&[
            Some(5), Some(1), Some(5), Some(5), Some(5), None, Some(5),
        ]);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::count_unival_subtrees(tree.clone()),
            );
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::count_unival_subtrees(tree.clone()),
            );
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
