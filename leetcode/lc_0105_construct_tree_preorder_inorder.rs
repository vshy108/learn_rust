// LeetCode 105: Construct Binary Tree from Preorder and Inorder Traversal [Medium]
// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.171us
//   AI solution avg: 1.191us
//   Ratio: ~tied
//
// Given two integer arrays preorder and inorder where preorder is the preorder
// traversal and inorder is the inorder traversal of the same tree, construct
// and return the binary tree.
//
// Constraints: 1 <= preorder.length <= 3000, preorder.length == inorder.length,
// -3000 <= values <= 3000, all values unique
//
// Run:  cargo run --example lc_0105_construct_tree_preorder_inorder
// Test: cargo test --example lc_0105_construct_tree_preorder_inorder
// Bench: cargo test --example lc_0105_construct_tree_preorder_inorder bench -- --nocapture

use std::cell::RefCell;
use std::collections::HashMap;
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

/// Helper: collect tree values in preorder for verification.
fn preorder_vals(root: &Node) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(node: &Node, out: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            out.push(n.val);
            dfs(&n.left, out);
            dfs(&n.right, out);
        }
    }
    dfs(root, &mut result);
    result
}

/// Helper: collect tree values in inorder for verification.
#[cfg(test)]
fn inorder_vals(root: &Node) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(node: &Node, out: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            dfs(&n.left, out);
            out.push(n.val);
            dfs(&n.right, out);
        }
    }
    dfs(root, &mut result);
    result
}

/// My solution — recursive build with linear search for root in inorder.
///
/// Time: O(n^2) worst case due to linear search in inorder.
/// Space: O(n) for recursion.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Build a binary tree from preorder and inorder traversals.
        ///
        /// # Arguments
        /// * `preorder` - preorder traversal values
        /// * `inorder` - inorder traversal values
        ///
        /// # Returns
        /// Root of the constructed binary tree
        pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Node {
            Self::build(&preorder, &inorder)
        }

        fn build(preorder: &[i32], inorder: &[i32]) -> Node {
            if preorder.is_empty() {
                return None;
            }
            let root_val = preorder[0];
            // Find root position in inorder — splits into left/right subtrees
            let mid = inorder.iter().position(|&x| x == root_val)
                .expect("value must exist in inorder");
            let mut node = TreeNode::new(root_val);
            node.left = Self::build(&preorder[1..=mid], &inorder[..mid]);
            node.right = Self::build(&preorder[mid + 1..], &inorder[mid + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

/// AI solution — O(n) using hashmap for inorder index lookup + index tracking.
///
/// Time: O(n).
/// Space: O(n) for hashmap.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Build a binary tree from preorder and inorder traversals.
        ///
        /// # Arguments
        /// * `preorder` - preorder traversal values
        /// * `inorder` - inorder traversal values
        ///
        /// # Returns
        /// Root of the constructed binary tree
        pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Node {
            let map: HashMap<i32, usize> = inorder.iter().enumerate()
                .map(|(i, &v)| (v, i))
                .collect();
            let mut pre_idx = 0;
            Self::build(&preorder, &mut pre_idx, 0, inorder.len(), &map)
        }

        fn build(
            preorder: &[i32],
            pre_idx: &mut usize,
            in_start: usize,
            in_end: usize,
            map: &HashMap<i32, usize>,
        ) -> Node {
            if in_start >= in_end {
                return None;
            }
            let root_val = preorder[*pre_idx];
            *pre_idx += 1;
            let mid = map[&root_val];
            let mut node = TreeNode::new(root_val);
            // Left subtree spans [in_start, mid)
            node.left = Self::build(preorder, pre_idx, in_start, mid, map);
            // Right subtree spans [mid+1, in_end)
            node.right = Self::build(preorder, pre_idx, mid + 1, in_end, map);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

fn main() {
    let pre = vec![3, 9, 20, 15, 7];
    let ino = vec![9, 3, 15, 20, 7];
    let my = my_solution::Solution::build_tree(pre.clone(), ino.clone());
    let ai = ai_solution::Solution::build_tree(pre, ino);
    println!("my preorder: {:?}", preorder_vals(&my));
    println!("ai preorder: {:?}", preorder_vals(&ai));
}

#[cfg(test)]
mod tests {
    use super::{inorder_vals, preorder_vals};

    macro_rules! test_build {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{inorder_vals, preorder_vals};

                #[test]
                fn test_example1() {
                    let pre = vec![3, 9, 20, 15, 7];
                    let ino = vec![9, 3, 15, 20, 7];
                    let tree = <$solution>::build_tree(pre.clone(), ino.clone());
                    assert_eq!(preorder_vals(&tree), pre);
                    assert_eq!(inorder_vals(&tree), ino);
                }

                #[test]
                fn test_single() {
                    let tree = <$solution>::build_tree(vec![-1], vec![-1]);
                    assert_eq!(preorder_vals(&tree), vec![-1]);
                }

                #[test]
                fn test_left_chain() {
                    let pre = vec![1, 2, 3];
                    let ino = vec![3, 2, 1];
                    let tree = <$solution>::build_tree(pre.clone(), ino.clone());
                    assert_eq!(preorder_vals(&tree), pre);
                    assert_eq!(inorder_vals(&tree), ino);
                }

                #[test]
                fn test_right_chain() {
                    let pre = vec![1, 2, 3];
                    let ino = vec![1, 2, 3];
                    let tree = <$solution>::build_tree(pre.clone(), ino.clone());
                    assert_eq!(preorder_vals(&tree), pre);
                    assert_eq!(inorder_vals(&tree), ino);
                }

                #[test]
                fn test_full_tree() {
                    let pre = vec![1, 2, 4, 5, 3, 6, 7];
                    let ino = vec![4, 2, 5, 1, 6, 3, 7];
                    let tree = <$solution>::build_tree(pre.clone(), ino.clone());
                    assert_eq!(preorder_vals(&tree), pre);
                    assert_eq!(inorder_vals(&tree), ino);
                }
            }
        };
    }

    test_build!(my, crate::my_solution::Solution);
    test_build!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Complete binary tree: preorder and inorder of a 15-node perfect tree
        let pre = vec![1, 2, 4, 8, 9, 5, 10, 11, 3, 6, 12, 13, 7, 14, 15];
        let ino = vec![8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::build_tree(
                std::hint::black_box(pre.clone()),
                std::hint::black_box(ino.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::build_tree(
                std::hint::black_box(pre.clone()),
                std::hint::black_box(ino.clone()),
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
