// LeetCode 108: Convert Sorted Array to Binary Search Tree [Easy]
// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 5.436us
//   AI solution avg: 5.300us
//   Ratio: ~tied
//
// Given an integer array nums where the elements are sorted in ascending order,
// convert it to a height-balanced binary search tree.
//
// Constraints: 1 <= nums.length <= 10^4, -10^4 <= nums[i] <= 10^4,
// nums is sorted in strictly increasing order
//
// Run:  cargo run --example lc_0108_sorted_array_to_bst
// Test: cargo test --example lc_0108_sorted_array_to_bst
// Bench: cargo test --example lc_0108_sorted_array_to_bst bench -- --nocapture

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
        TreeNode { val, left: None, right: None }
    }
}

type Node = Option<Rc<RefCell<TreeNode>>>;

/// Check if tree is a valid BST and return its height (-1 if invalid).
#[cfg(test)]
fn check_bst(node: &Node, min: i64, max: i64) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let n = n.borrow();
            if (n.val as i64) <= min || (n.val as i64) >= max {
                return -1;
            }
            let lh = check_bst(&n.left, min, n.val as i64);
            let rh = check_bst(&n.right, n.val as i64, max);
            if lh < 0 || rh < 0 || (lh - rh).abs() > 1 {
                return -1;
            }
            1 + lh.max(rh)
        }
    }
}

/// Collect inorder traversal to verify BST property.
fn inorder(node: &Node) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(n: &Node, out: &mut Vec<i32>) {
        if let Some(node) = n {
            let node = node.borrow();
            dfs(&node.left, out);
            out.push(node.val);
            dfs(&node.right, out);
        }
    }
    dfs(node, &mut result);
    result
}

/// My solution — recursive mid-point selection with slices.
///
/// Time: O(n).
/// Space: O(log n) recursion.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Convert sorted array to height-balanced BST.
        ///
        /// # Arguments
        /// * `nums` - sorted array of integers
        ///
        /// # Returns
        /// Root of the balanced BST
        pub fn sorted_array_to_bst(nums: Vec<i32>) -> Node {
            Self::build(&nums)
        }

        fn build(nums: &[i32]) -> Node {
            if nums.is_empty() {
                return None;
            }
            let mid = nums.len() / 2;
            let mut node = TreeNode::new(nums[mid]);
            node.left = Self::build(&nums[..mid]);
            node.right = Self::build(&nums[mid + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

/// AI solution — iterative using explicit stack with index ranges.
///
/// Time: O(n).
/// Space: O(n) for stack and nodes.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Convert sorted array to height-balanced BST iteratively.
        ///
        /// # Arguments
        /// * `nums` - sorted array of integers
        ///
        /// # Returns
        /// Root of the balanced BST
        pub fn sorted_array_to_bst(nums: Vec<i32>) -> Node {
            if nums.is_empty() {
                return None;
            }
            let mid = nums.len() / 2;
            let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            // Stack: (parent, is_left_child, start, end)
            let mut stack: Vec<(Rc<RefCell<TreeNode>>, bool, usize, usize)> = Vec::new();
            if mid > 0 {
                stack.push((Rc::clone(&root), true, 0, mid));
            }
            if mid + 1 < nums.len() {
                stack.push((Rc::clone(&root), false, mid + 1, nums.len()));
            }

            while let Some((parent, is_left, start, end)) = stack.pop() {
                if start >= end {
                    continue;
                }
                let m = start + (end - start) / 2;
                let child = Rc::new(RefCell::new(TreeNode::new(nums[m])));
                if is_left {
                    parent.borrow_mut().left = Some(Rc::clone(&child));
                } else {
                    parent.borrow_mut().right = Some(Rc::clone(&child));
                }
                if m > start {
                    stack.push((Rc::clone(&child), true, start, m));
                }
                if m + 1 < end {
                    stack.push((Rc::clone(&child), false, m + 1, end));
                }
            }
            Some(root)
        }
    }
}

fn main() {
    let nums = vec![-10, -3, 0, 5, 9];
    let my = my_solution::Solution::sorted_array_to_bst(nums.clone());
    let ai = ai_solution::Solution::sorted_array_to_bst(nums);
    println!("my inorder: {:?}", inorder(&my));
    println!("ai inorder: {:?}", inorder(&ai));
}

#[cfg(test)]
mod tests {
    use super::{check_bst, inorder};

    macro_rules! test_bst {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{check_bst, inorder};

                #[test]
                fn test_example1() {
                    let nums = vec![-10, -3, 0, 5, 9];
                    let tree = <$solution>::sorted_array_to_bst(nums.clone());
                    assert_eq!(inorder(&tree), nums);
                    assert!(check_bst(&tree, i64::MIN, i64::MAX) > 0);
                }

                #[test]
                fn test_two_elements() {
                    let nums = vec![1, 3];
                    let tree = <$solution>::sorted_array_to_bst(nums.clone());
                    assert_eq!(inorder(&tree), nums);
                    assert!(check_bst(&tree, i64::MIN, i64::MAX) > 0);
                }

                #[test]
                fn test_single() {
                    let nums = vec![0];
                    let tree = <$solution>::sorted_array_to_bst(nums.clone());
                    assert_eq!(inorder(&tree), nums);
                }

                #[test]
                fn test_seven() {
                    let nums = vec![1, 2, 3, 4, 5, 6, 7];
                    let tree = <$solution>::sorted_array_to_bst(nums.clone());
                    assert_eq!(inorder(&tree), nums);
                    assert!(check_bst(&tree, i64::MIN, i64::MAX) > 0);
                }

                #[test]
                fn test_large() {
                    let nums: Vec<i32> = (0..100).collect();
                    let tree = <$solution>::sorted_array_to_bst(nums.clone());
                    assert_eq!(inorder(&tree), nums);
                    assert!(check_bst(&tree, i64::MIN, i64::MAX) > 0);
                }
            }
        };
    }

    test_bst!(my, crate::my_solution::Solution);
    test_bst!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..100).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::sorted_array_to_bst(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::sorted_array_to_bst(
                std::hint::black_box(nums.clone()),
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
