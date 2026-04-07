// LeetCode 107: Binary Tree Level Order Traversal II [Medium]
// https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.612us
//   AI solution avg: 2.584us
//   Ratio: ~tied
//
// Given the root of a binary tree, return the bottom-up level order traversal
// of its nodes' values (i.e., from left to right, level by level from leaf to root).
//
// Constraints: [0, 2000] nodes, -1000 <= Node.val <= 1000
//
// Run:  cargo run --example lc_0107_level_order_bottom
// Test: cargo test --example lc_0107_level_order_bottom
// Bench: cargo test --example lc_0107_level_order_bottom bench -- --nocapture

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

/// My solution — BFS level order then reverse.
///
/// Time: O(n).
/// Space: O(n).
mod my_solution {
    use super::*;
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Return bottom-up level order traversal.
        pub fn level_order_bottom(root: Node) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            let Some(r) = root else { return result };
            let mut queue = VecDeque::new();
            queue.push_back(r);
            while !queue.is_empty() {
                let size = queue.len();
                let mut level = Vec::with_capacity(size);
                for _ in 0..size {
                    if let Some(node) = queue.pop_front() {
                        let node = node.borrow();
                        level.push(node.val);
                        if let Some(ref l) = node.left {
                            queue.push_back(Rc::clone(l));
                        }
                        if let Some(ref r) = node.right {
                            queue.push_back(Rc::clone(r));
                        }
                    }
                }
                result.push(level);
            }
            result.reverse();
            result
        }
    }
}

/// AI solution — DFS with depth tracking, insert at correct position.
///
/// Time: O(n).
/// Space: O(h) recursion + O(n) result.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return bottom-up level order traversal using DFS.
        pub fn level_order_bottom(root: Node) -> Vec<Vec<i32>> {
            let mut levels = Vec::new();
            Self::dfs(&root, 0, &mut levels);
            levels.reverse();
            levels
        }

        fn dfs(node: &Node, depth: usize, levels: &mut Vec<Vec<i32>>) {
            if let Some(n) = node {
                let n = n.borrow();
                if depth == levels.len() {
                    levels.push(Vec::new());
                }
                levels[depth].push(n.val);
                Self::dfs(&n.left, depth + 1, levels);
                Self::dfs(&n.right, depth + 1, levels);
            }
        }
    }
}

fn main() {
    let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let my = my_solution::Solution::level_order_bottom(root.clone());
    let ai = ai_solution::Solution::level_order_bottom(root);
    println!("bottom-up: my={my:?}, ai={ai:?}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_level_order {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_example1() {
                    let root = build_tree(&[
                        Some(3), Some(9), Some(20), None, None, Some(15), Some(7),
                    ]);
                    assert_eq!(
                        <$solution>::level_order_bottom(root),
                        vec![vec![15, 7], vec![9, 20], vec![3]],
                    );
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(1)]);
                    assert_eq!(<$solution>::level_order_bottom(root), vec![vec![1]]);
                }

                #[test]
                fn test_empty() {
                    assert_eq!(
                        <$solution>::level_order_bottom(None),
                        Vec::<Vec<i32>>::new(),
                    );
                }

                #[test]
                fn test_balanced() {
                    let root = build_tree(&[
                        Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7),
                    ]);
                    assert_eq!(
                        <$solution>::level_order_bottom(root),
                        vec![vec![4, 5, 6, 7], vec![2, 3], vec![1]],
                    );
                }
            }
        };
    }

    test_level_order!(my, crate::my_solution::Solution);
    test_level_order!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=31).map(Some).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::level_order_bottom(
                std::hint::black_box(root),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::level_order_bottom(
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
