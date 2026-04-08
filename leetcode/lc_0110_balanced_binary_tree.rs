// LeetCode 110: Balanced Binary Tree [Easy]
// https://leetcode.com/problems/balanced-binary-tree/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.588us
//   AI solution avg: 1.996us
//   Ratio: AI 1.30x faster
//
// Given a binary tree, determine if it is height-balanced.
// A height-balanced tree has left and right subtree heights differing by at most 1.
//
// Constraints: [0, 5000] nodes, -10^4 <= Node.val <= 10^4
//
// Run:  cargo run --example lc_0110_balanced_binary_tree
// Test: cargo test --example lc_0110_balanced_binary_tree
// Bench: cargo test --example lc_0110_balanced_binary_tree bench -- --nocapture

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

/// My solution — top-down recursive, computes height at each node.
///
/// Time: O(n log n) worst case due to repeated height computation.
/// Space: O(h) recursion.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Check if a binary tree is height-balanced.
        pub fn is_balanced(root: Node) -> bool {
            match root {
                None => true,
                Some(node) => {
                    let node = node.borrow();
                    let diff = (Self::height(&node.left) - Self::height(&node.right)).abs();
                    diff <= 1
                        && Self::is_balanced(node.left.clone())
                        && Self::is_balanced(node.right.clone())
                }
            }
        }

        fn height(node: &Node) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    1 + Self::height(&n.left).max(Self::height(&n.right))
                }
            }
        }
    }
}

/// AI solution — bottom-up O(n): return -1 if unbalanced, else height.
///
/// Time: O(n) single pass.
/// Space: O(h) recursion.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Check if a binary tree is height-balanced using bottom-up approach.
        pub fn is_balanced(root: Node) -> bool {
            Self::check(&root) >= 0
        }

        /// Returns height if balanced, -1 if not.
        fn check(node: &Node) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let lh = Self::check(&n.left);
                    if lh < 0 {
                        return -1;
                    }
                    let rh = Self::check(&n.right);
                    if rh < 0 {
                        return -1;
                    }
                    if (lh - rh).abs() > 1 {
                        return -1;
                    }
                    1 + lh.max(rh)
                }
            }
        }
    }
}

fn main() {
    let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let my = my_solution::Solution::is_balanced(root.clone());
    let ai = ai_solution::Solution::is_balanced(root);
    println!("balanced: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_balanced {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_balanced() {
                    let root =
                        build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
                    assert!(<$solution>::is_balanced(root));
                }

                #[test]
                fn test_unbalanced() {
                    let root = build_tree(&[
                        Some(1),
                        Some(2),
                        Some(2),
                        Some(3),
                        Some(3),
                        None,
                        None,
                        Some(4),
                        Some(4),
                    ]);
                    assert!(!<$solution>::is_balanced(root));
                }

                #[test]
                fn test_empty() {
                    assert!(<$solution>::is_balanced(None));
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(1)]);
                    assert!(<$solution>::is_balanced(root));
                }

                #[test]
                fn test_left_heavy() {
                    let root = build_tree(&[Some(1), Some(2), None, Some(3)]);
                    assert!(!<$solution>::is_balanced(root));
                }
            }
        };
    }

    test_balanced!(my, crate::my_solution::Solution);
    test_balanced!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=31).map(Some).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::is_balanced(
                std::hint::black_box(root),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::is_balanced(
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
