// LeetCode 98: Validate Binary Search Tree [Medium]
// https://leetcode.com/problems/validate-binary-search-tree/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 5.240us
//   AI solution avg: 4.440us
//   Ratio: AI 1.18x faster
//
// Given the root of a binary tree, determine if it is a valid BST.
// A valid BST: left subtree values < node < right subtree values (strict).
//
// Constraints: [1, 10^4] nodes, -2^31 <= Node.val <= 2^31 - 1
//
// Run:  cargo run --example lc_0098_validate_bst
// Test: cargo test --example lc_0098_validate_bst
// Bench: cargo test --example lc_0098_validate_bst bench -- --nocapture

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

/// My solution — inorder traversal, check sorted order.
///
/// Time: O(n).
/// Space: O(n).
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Validate if tree is a BST.
        ///
        /// # Arguments
        /// * `root` - binary tree root
        ///
        /// # Returns
        /// true if valid BST
        pub fn is_valid_bst(root: Node) -> bool {
            let mut vals = Vec::new();
            Self::inorder(&root, &mut vals);
            // Check strictly increasing
            vals.windows(2).all(|w| w[0] < w[1])
        }

        fn inorder(node: &Node, vals: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                Self::inorder(&n.left, vals);
                vals.push(n.val);
                Self::inorder(&n.right, vals);
            }
        }
    }
}

/// AI solution — recursive range checking (early termination).
///
/// Time: O(n).
/// Space: O(h) where h = tree height.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Validate if tree is a BST.
        ///
        /// # Arguments
        /// * `root` - binary tree root
        ///
        /// # Returns
        /// true if valid BST
        pub fn is_valid_bst(root: Node) -> bool {
            Self::validate(&root, i64::MIN, i64::MAX)
        }

        fn validate(node: &Node, min: i64, max: i64) -> bool {
            match node {
                None => true,
                Some(n) => {
                    let n = n.borrow();
                    let val = n.val as i64;
                    if val <= min || val >= max {
                        return false;
                    }
                    Self::validate(&n.left, min, val) && Self::validate(&n.right, val, max)
                }
            }
        }
    }
}

fn main() {
    let valid = build_tree(&[Some(2), Some(1), Some(3)]);
    let invalid = build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
    println!(
        "valid tree: my={}, ai={}",
        my_solution::Solution::is_valid_bst(valid.clone()),
        ai_solution::Solution::is_valid_bst(valid),
    );
    println!(
        "invalid tree: my={}, ai={}",
        my_solution::Solution::is_valid_bst(invalid.clone()),
        ai_solution::Solution::is_valid_bst(invalid),
    );
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_valid_bst {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_valid() {
                    let root = build_tree(&[Some(2), Some(1), Some(3)]);
                    assert!(<$solution>::is_valid_bst(root));
                }

                #[test]
                fn test_invalid() {
                    let root =
                        build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
                    assert!(!<$solution>::is_valid_bst(root));
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(1)]);
                    assert!(<$solution>::is_valid_bst(root));
                }

                #[test]
                fn test_equal_values() {
                    // BST must be strictly less/greater
                    let root = build_tree(&[Some(1), Some(1)]);
                    assert!(!<$solution>::is_valid_bst(root));
                }

                #[test]
                fn test_empty() {
                    assert!(<$solution>::is_valid_bst(None));
                }
            }
        };
    }

    test_valid_bst!(my, crate::my_solution::Solution);
    test_valid_bst!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=31).map(Some).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::is_valid_bst(
                std::hint::black_box(root),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::is_valid_bst(
                std::hint::black_box(root),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
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
