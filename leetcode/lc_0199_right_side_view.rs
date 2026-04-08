// LeetCode 199: Binary Tree Right Side View [Medium]
// https://leetcode.com/problems/binary-tree-right-side-view/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.091us
//   AI solution avg: 0.194us
//   Ratio: My 2.12x faster
//
// Given the root of a binary tree, return the values of the nodes you
// can see when looking at the tree from the right side (top to bottom).
//
// Constraints: nodes in [0, 100], -100 <= Node.val <= 100
//
// Run:  cargo run --example lc_0199_right_side_view
// Test: cargo test --example lc_0199_right_side_view
// Bench: cargo test --example lc_0199_right_side_view bench -- --nocapture

use std::cell::RefCell;
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

/// Build tree from level-order vec (None = null node).
fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].expect("root"))));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone());
    let mut i = 1;
    while i < vals.len() {
        let parent = queue.pop_front().expect("parent");
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(v)));
                parent.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;
        }
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(v)));
                parent.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }
    }
    Some(root)
}

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the right side view of the tree.
        pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            // DFS: visit right child first, record first node at each depth
            let mut result = Vec::new();
            fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, result: &mut Vec<i32>) {
                if let Some(n) = node {
                    let borrowed = n.borrow();
                    if depth == result.len() {
                        result.push(borrowed.val);
                    }
                    dfs(&borrowed.right, depth + 1, result);
                    dfs(&borrowed.left, depth + 1, result);
                }
            }
            dfs(&root, 0, &mut result);
            result
        }
    }
}

/// AI solution — BFS level-order traversal, take last node per level.
///
/// Time: O(n) — visit every node once.
/// Space: O(w) where w is max tree width (queue size).
mod ai_solution {
    use super::*;
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// BFS: take the last value at each level.
        pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut result = Vec::new();
            let Some(root) = root else { return result };

            let mut queue = VecDeque::new();
            queue.push_back(root);

            while !queue.is_empty() {
                let level_size = queue.len();
                for i in 0..level_size {
                    let node = queue.pop_front().expect("node");
                    let borrowed = node.borrow();
                    // Last node in this level is the rightmost visible
                    if i == level_size - 1 {
                        result.push(borrowed.val);
                    }
                    if let Some(left) = &borrowed.left {
                        queue.push_back(left.clone());
                    }
                    if let Some(right) = &borrowed.right {
                        queue.push_back(right.clone());
                    }
                }
            }
            result
        }
    }
}

fn main() {
    let root = build_tree(&[Some(1), Some(2), Some(3), None, Some(5), None, Some(4)]);
    let my = my_solution::Solution::right_side_view(root.clone());
    let ai = ai_solution::Solution::right_side_view(root);
    println!("right side: my={my:?}, ai={ai:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_rsv {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    let root =
                        build_tree(&[Some(1), Some(2), Some(3), None, Some(5), None, Some(4)]);
                    assert_eq!(<$solution>::right_side_view(root), vec![1, 3, 4],);
                }

                #[test]
                fn test_left_only() {
                    let root = build_tree(&[Some(1), Some(2), None]);
                    assert_eq!(<$solution>::right_side_view(root), vec![1, 2]);
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(1)]);
                    assert_eq!(<$solution>::right_side_view(root), vec![1]);
                }

                #[test]
                fn test_empty() {
                    assert_eq!(<$solution>::right_side_view(None), Vec::<i32>::new());
                }
            }
        };
    }

    test_rsv!(my, crate::my_solution::Solution);
    test_rsv!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let root = build_tree(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            Some(14),
            Some(15),
        ]);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::right_side_view(
                std::hint::black_box(root.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::right_side_view(
                std::hint::black_box(root.clone()),
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
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
