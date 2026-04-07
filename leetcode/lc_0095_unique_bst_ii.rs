// LeetCode 95: Unique Binary Search Trees II [Medium]
// https://leetcode.com/problems/unique-binary-search-trees-ii/
//
// First benchmark: 10_000 iterations, 2025-07-08
//   My solution avg: 42.834us
//   AI solution avg: 42.801us
//   Ratio: ~tied
//
// Given an integer n, return all structurally unique BSTs that store
// values 1 to n.
//
// Constraints: 1 <= n <= 8
//
// Run:  cargo run --example lc_0095_unique_bst_ii
// Test: cargo test --example lc_0095_unique_bst_ii
// Bench: cargo test --example lc_0095_unique_bst_ii bench -- --nocapture

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

/// Collect tree values via inorder traversal for verification.
#[cfg(test)]
fn inorder(root: &Node) -> Vec<i32> {
    let mut result = Vec::new();
    fn helper(node: &Node, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            helper(&n.left, result);
            result.push(n.val);
            helper(&n.right, result);
        }
    }
    helper(root, &mut result);
    result
}

/// My solution — recursive generation (same approach as AI, canonical).
///
/// Time: O(n * C(n)) where C(n) is nth Catalan number.
/// Space: O(n * C(n)).
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Generate all structurally unique BSTs with values 1..=n.
        ///
        /// # Arguments
        /// * `n` - upper range of values
        ///
        /// # Returns
        /// All unique BST roots
        pub fn generate_trees(n: i32) -> Vec<Node> {
            if n == 0 {
                return vec![];
            }
            Self::build(1, n)
        }

        fn build(lo: i32, hi: i32) -> Vec<Node> {
            if lo > hi {
                return vec![None];
            }
            let mut trees = Vec::new();
            for root_val in lo..=hi {
                let left_trees = Self::build(lo, root_val - 1);
                let right_trees = Self::build(root_val + 1, hi);
                for l in &left_trees {
                    for r in &right_trees {
                        let root = Rc::new(RefCell::new(TreeNode {
                            val: root_val,
                            left: l.clone(),
                            right: r.clone(),
                        }));
                        trees.push(Some(root));
                    }
                }
            }
            trees
        }
    }
}

/// AI solution — same recursive approach (canonical, no faster known).
///
/// Time: O(n * C(n)).
/// Space: O(n * C(n)).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Generate all structurally unique BSTs with values 1..=n.
        ///
        /// # Arguments
        /// * `n` - upper range of values
        ///
        /// # Returns
        /// All unique BST roots
        pub fn generate_trees(n: i32) -> Vec<Node> {
            if n == 0 {
                return vec![];
            }
            Self::generate(1, n)
        }

        fn generate(start: i32, end: i32) -> Vec<Node> {
            if start > end {
                return vec![None];
            }
            let mut all = Vec::new();
            for i in start..=end {
                let lefts = Self::generate(start, i - 1);
                let rights = Self::generate(i + 1, end);
                for l in &lefts {
                    for r in &rights {
                        let node = Rc::new(RefCell::new(TreeNode {
                            val: i,
                            left: l.clone(),
                            right: r.clone(),
                        }));
                        all.push(Some(node));
                    }
                }
            }
            all
        }
    }
}

fn main() {
    let my = my_solution::Solution::generate_trees(3);
    let ai = ai_solution::Solution::generate_trees(3);
    println!("my count: {}, ai count: {}", my.len(), ai.len());
}

#[cfg(test)]
mod tests {
    use super::inorder;

    macro_rules! test_generate_trees {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::inorder;

                #[test]
                fn test_n3() {
                    let trees = <$solution>::generate_trees(3);
                    // Catalan(3) = 5
                    assert_eq!(trees.len(), 5);
                    // All trees should have inorder [1, 2, 3] (BST property)
                    for tree in &trees {
                        assert_eq!(inorder(tree), vec![1, 2, 3]);
                    }
                }

                #[test]
                fn test_n1() {
                    let trees = <$solution>::generate_trees(1);
                    assert_eq!(trees.len(), 1);
                }

                #[test]
                fn test_n4() {
                    let trees = <$solution>::generate_trees(4);
                    // Catalan(4) = 14
                    assert_eq!(trees.len(), 14);
                }
            }
        };
    }

    test_generate_trees!(my, crate::my_solution::Solution);
    test_generate_trees!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 10_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::generate_trees(
                std::hint::black_box(5),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::generate_trees(
                std::hint::black_box(5),
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
