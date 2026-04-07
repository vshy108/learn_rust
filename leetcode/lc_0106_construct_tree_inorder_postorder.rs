// LeetCode 106: Construct Binary Tree from Inorder and Postorder Traversal [Medium]
// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.183us
//   AI solution avg: 1.201us
//   Ratio: ~tied
//
// Given two integer arrays inorder and postorder, construct and return the
// binary tree.
//
// Constraints: 1 <= inorder.length <= 3000, all values unique
//
// Run:  cargo run --example lc_0106_construct_tree_inorder_postorder
// Test: cargo test --example lc_0106_construct_tree_inorder_postorder
// Bench: cargo test --example lc_0106_construct_tree_inorder_postorder bench -- --nocapture

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

#[cfg(test)]
fn postorder_vals(root: &Node) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(node: &Node, out: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            dfs(&n.left, out);
            dfs(&n.right, out);
            out.push(n.val);
        }
    }
    dfs(root, &mut result);
    result
}

/// My solution — recursive build with linear search in inorder.
///
/// Time: O(n^2) worst case.
/// Space: O(n) for recursion.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Build tree from inorder and postorder traversals.
        pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Node {
            Self::build(&inorder, &postorder)
        }

        fn build(inorder: &[i32], postorder: &[i32]) -> Node {
            if postorder.is_empty() {
                return None;
            }
            let root_val = *postorder.last().expect("non-empty");
            let mid = inorder.iter().position(|&x| x == root_val)
                .expect("value must exist");
            let mut node = TreeNode::new(root_val);
            node.left = Self::build(&inorder[..mid], &postorder[..mid]);
            node.right = Self::build(
                &inorder[mid + 1..],
                &postorder[mid..postorder.len() - 1],
            );
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

/// AI solution — O(n) using hashmap for inorder index lookup.
///
/// Time: O(n).
/// Space: O(n) for hashmap.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Build tree from inorder and postorder using hashmap lookup.
        pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Node {
            let map: HashMap<i32, usize> = inorder.iter().enumerate()
                .map(|(i, &v)| (v, i))
                .collect();
            let mut post_idx = postorder.len();
            Self::build(&postorder, &mut post_idx, 0, inorder.len(), &map)
        }

        fn build(
            postorder: &[i32],
            post_idx: &mut usize,
            in_start: usize,
            in_end: usize,
            map: &HashMap<i32, usize>,
        ) -> Node {
            if in_start >= in_end {
                return None;
            }
            *post_idx -= 1;
            let root_val = postorder[*post_idx];
            let mid = map[&root_val];
            let mut node = TreeNode::new(root_val);
            // Build right first (postorder: L R Root, so reverse is Root R L)
            node.right = Self::build(postorder, post_idx, mid + 1, in_end, map);
            node.left = Self::build(postorder, post_idx, in_start, mid, map);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

fn main() {
    let ino = vec![9, 3, 15, 20, 7];
    let post = vec![9, 15, 7, 20, 3];
    let my = my_solution::Solution::build_tree(ino.clone(), post.clone());
    let ai = ai_solution::Solution::build_tree(ino, post);
    println!("my inorder: {:?}", inorder_vals(&my));
    println!("ai inorder: {:?}", inorder_vals(&ai));
}

#[cfg(test)]
mod tests {
    use super::{inorder_vals, postorder_vals};

    macro_rules! test_build {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{inorder_vals, postorder_vals};

                #[test]
                fn test_example1() {
                    let ino = vec![9, 3, 15, 20, 7];
                    let post = vec![9, 15, 7, 20, 3];
                    let tree = <$solution>::build_tree(ino.clone(), post.clone());
                    assert_eq!(inorder_vals(&tree), ino);
                    assert_eq!(postorder_vals(&tree), post);
                }

                #[test]
                fn test_single() {
                    let tree = <$solution>::build_tree(vec![-1], vec![-1]);
                    assert_eq!(inorder_vals(&tree), vec![-1]);
                }

                #[test]
                fn test_left_chain() {
                    let ino = vec![3, 2, 1];
                    let post = vec![3, 2, 1];
                    let tree = <$solution>::build_tree(ino.clone(), post.clone());
                    assert_eq!(inorder_vals(&tree), ino);
                    assert_eq!(postorder_vals(&tree), post);
                }

                #[test]
                fn test_right_chain() {
                    let ino = vec![1, 2, 3];
                    let post = vec![3, 2, 1];
                    let tree = <$solution>::build_tree(ino.clone(), post.clone());
                    assert_eq!(inorder_vals(&tree), ino);
                    assert_eq!(postorder_vals(&tree), post);
                }

                #[test]
                fn test_full_tree() {
                    let ino = vec![4, 2, 5, 1, 6, 3, 7];
                    let post = vec![4, 5, 2, 6, 7, 3, 1];
                    let tree = <$solution>::build_tree(ino.clone(), post.clone());
                    assert_eq!(inorder_vals(&tree), ino);
                    assert_eq!(postorder_vals(&tree), post);
                }
            }
        };
    }

    test_build!(my, crate::my_solution::Solution);
    test_build!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let ino = vec![8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15];
        let post = vec![8, 9, 4, 10, 11, 5, 2, 12, 13, 6, 14, 15, 7, 3, 1];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::build_tree(
                std::hint::black_box(ino.clone()),
                std::hint::black_box(post.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::build_tree(
                std::hint::black_box(ino.clone()),
                std::hint::black_box(post.clone()),
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
