// LeetCode 104: Maximum Depth of Binary Tree [Easy]
// https://leetcode.com/problems/maximum-depth-of-binary-tree/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.171us
//   AI solution avg: 2.220us
//   Ratio: ~tied
//
// Given the root of a binary tree, return its maximum depth.
// Maximum depth = number of nodes along the longest path from root to leaf.
//
// Constraints: [0, 10^4] nodes, -100 <= Node.val <= 100
//
// Run:  cargo run --example lc_0104_max_depth_binary_tree
// Test: cargo test --example lc_0104_max_depth_binary_tree
// Bench: cargo test --example lc_0104_max_depth_binary_tree bench -- --nocapture

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

/// My solution — recursive DFS.
///
/// Time: O(n).
/// Space: O(h) recursion stack.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the maximum depth of a binary tree.
        ///
        /// # Arguments
        /// * `root` - root of the binary tree
        ///
        /// # Returns
        /// Maximum depth (number of nodes on longest root-to-leaf path)
        pub fn max_depth(root: Node) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let node = node.borrow();
                    1 + Self::max_depth(node.left.clone()).max(Self::max_depth(node.right.clone()))
                }
            }
        }
    }
}

/// AI solution — iterative BFS counting levels.
///
/// Time: O(n).
/// Space: O(n) for queue.
mod ai_solution {
    use super::*;
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Return the maximum depth using level-order traversal.
        ///
        /// # Arguments
        /// * `root` - root of the binary tree
        ///
        /// # Returns
        /// Maximum depth
        pub fn max_depth(root: Node) -> i32 {
            let Some(r) = root else { return 0 };
            let mut queue = VecDeque::new();
            queue.push_back(r);
            let mut depth = 0;

            while !queue.is_empty() {
                depth += 1;
                let size = queue.len();
                for _ in 0..size {
                    if let Some(node) = queue.pop_front() {
                        let node = node.borrow();
                        if let Some(ref l) = node.left {
                            queue.push_back(Rc::clone(l));
                        }
                        if let Some(ref r) = node.right {
                            queue.push_back(Rc::clone(r));
                        }
                    }
                }
            }
            depth
        }
    }
}

fn main() {
    let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let my = my_solution::Solution::max_depth(root.clone());
    let ai = ai_solution::Solution::max_depth(root);
    println!("max depth: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_depth {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_example1() {
                    let root =
                        build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
                    assert_eq!(<$solution>::max_depth(root), 3);
                }

                #[test]
                fn test_example2() {
                    let root = build_tree(&[Some(1), None, Some(2)]);
                    assert_eq!(<$solution>::max_depth(root), 2);
                }

                #[test]
                fn test_empty() {
                    assert_eq!(<$solution>::max_depth(None), 0);
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(1)]);
                    assert_eq!(<$solution>::max_depth(root), 1);
                }

                #[test]
                fn test_left_chain() {
                    let root = build_tree(&[Some(1), Some(2), None, Some(3)]);
                    assert_eq!(<$solution>::max_depth(root), 3);
                }
            }
        };
    }

    test_depth!(my, crate::my_solution::Solution);
    test_depth!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=31).map(Some).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::max_depth(
                std::hint::black_box(root),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::max_depth(
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
