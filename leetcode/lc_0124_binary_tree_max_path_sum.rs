// LeetCode 124: Binary Tree Maximum Path Sum [Hard]
// https://leetcode.com/problems/binary-tree-maximum-path-sum/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.024us
//   AI solution avg: 3.443us
//   Ratio: My 1.70x faster
//
// A path in a binary tree is a sequence of nodes where each pair of adjacent
// nodes has an edge. The path does not need to pass through the root.
// Return the maximum path sum of any non-empty path.
//
// Constraints: [1, 3*10^4] nodes, -1000 <= Node.val <= 1000
//
// Run:  cargo run --example lc_0124_binary_tree_max_path_sum
// Test: cargo test --example lc_0124_binary_tree_max_path_sum
// Bench: cargo test --example lc_0124_binary_tree_max_path_sum bench -- --nocapture

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

/// My solution — recursive DFS tracking global max.
///
/// Time: O(n).
/// Space: O(h) recursion.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the maximum path sum in the binary tree.
        pub fn max_path_sum(root: Node) -> i32 {
            let mut max_sum = i32::MIN;
            Self::dfs(&root, &mut max_sum);
            max_sum
        }

        /// Returns max gain from this node going downward (single branch).
        fn dfs(node: &Node, max_sum: &mut i32) -> i32 {
            let Some(n) = node else { return 0 };
            let n = n.borrow();
            // Only take positive contributions from children
            let left_gain = Self::dfs(&n.left, max_sum).max(0);
            let right_gain = Self::dfs(&n.right, max_sum).max(0);
            // Path through this node: left_gain + val + right_gain
            *max_sum = (*max_sum).max(n.val + left_gain + right_gain);
            // Return single branch max for parent's use
            n.val + left_gain.max(right_gain)
        }
    }
}

/// AI solution — same approach (this is the optimal algorithm).
/// Uses iterative postorder with explicit stack for potentially better cache.
///
/// Time: O(n).
/// Space: O(n) for stack + gain map.
mod ai_solution {
    use super::*;
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return max path sum using iterative postorder traversal.
        pub fn max_path_sum(root: Node) -> i32 {
            let Some(r) = root else { return 0 };
            let mut max_sum = i32::MIN;
            // Map: node_ptr -> max single-branch gain from that node
            let mut gains: HashMap<*const RefCell<TreeNode>, i32> = HashMap::new();
            let mut stack: Vec<(Rc<RefCell<TreeNode>>, bool)> = vec![(r, false)];

            while let Some((node, visited)) = stack.pop() {
                if visited {
                    let nb = node.borrow();
                    let left_gain = nb
                        .left
                        .as_ref()
                        .map(|l| *gains.get(&Rc::as_ptr(l)).unwrap_or(&0))
                        .unwrap_or(0)
                        .max(0);
                    let right_gain = nb
                        .right
                        .as_ref()
                        .map(|r| *gains.get(&Rc::as_ptr(r)).unwrap_or(&0))
                        .unwrap_or(0)
                        .max(0);
                    max_sum = max_sum.max(nb.val + left_gain + right_gain);
                    gains.insert(Rc::as_ptr(&node), nb.val + left_gain.max(right_gain));
                } else {
                    stack.push((Rc::clone(&node), true));
                    let nb = node.borrow();
                    if let Some(ref r) = nb.right {
                        stack.push((Rc::clone(r), false));
                    }
                    if let Some(ref l) = nb.left {
                        stack.push((Rc::clone(l), false));
                    }
                }
            }
            max_sum
        }
    }
}

fn main() {
    let root = build_tree(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let my = my_solution::Solution::max_path_sum(root.clone());
    let ai = ai_solution::Solution::max_path_sum(root);
    println!("max path sum: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_path {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_example1() {
                    let root = build_tree(&[Some(1), Some(2), Some(3)]);
                    assert_eq!(<$solution>::max_path_sum(root), 6);
                }

                #[test]
                fn test_example2() {
                    let root =
                        build_tree(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)]);
                    assert_eq!(<$solution>::max_path_sum(root), 42);
                }

                #[test]
                fn test_single_negative() {
                    let root = build_tree(&[Some(-3)]);
                    assert_eq!(<$solution>::max_path_sum(root), -3);
                }

                #[test]
                fn test_all_negative() {
                    let root = build_tree(&[Some(-1), Some(-2), Some(-3)]);
                    assert_eq!(<$solution>::max_path_sum(root), -1);
                }

                #[test]
                fn test_single_positive() {
                    let root = build_tree(&[Some(5)]);
                    assert_eq!(<$solution>::max_path_sum(root), 5);
                }
            }
        };
    }

    test_path!(my, crate::my_solution::Solution);
    test_path!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=31).map(|i| Some(i * 3 - 50)).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::max_path_sum(
                std::hint::black_box(root),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::max_path_sum(
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
