// LeetCode 113: Path Sum II [Medium]
// https://leetcode.com/problems/path-sum-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.281us
//   AI solution avg: 1.971us
//   Ratio: My 1.54x faster
//
// Given the root of a binary tree and an integer targetSum, return all
// root-to-leaf paths where the sum of the node values equals targetSum.
//
// Constraints: [0, 5000] nodes, -1000 <= Node.val <= 1000,
// -1000 <= targetSum <= 1000
//
// Run:  cargo run --example lc_0113_path_sum_ii
// Test: cargo test --example lc_0113_path_sum_ii
// Bench: cargo test --example lc_0113_path_sum_ii bench -- --nocapture

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

/// My solution — recursive DFS backtracking to collect all paths.
///
/// Time: O(n^2) worst case (O(n) nodes, each path up to O(n) length).
/// Space: O(n) for path + recursion.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return all root-to-leaf paths with given sum.
        pub fn path_sum(root: Node, target_sum: i32) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            let mut path = Vec::new();
            Self::dfs(&root, target_sum, &mut path, &mut result);
            result
        }

        fn dfs(node: &Node, remain: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if let Some(n) = node {
                let n = n.borrow();
                path.push(n.val);
                let remain = remain - n.val;
                if n.left.is_none() && n.right.is_none() && remain == 0 {
                    result.push(path.clone());
                } else {
                    Self::dfs(&n.left, remain, path, result);
                    Self::dfs(&n.right, remain, path, result);
                }
                path.pop(); // Backtrack
            }
        }
    }
}

/// AI solution — same DFS approach (optimal for this problem).
/// Uses iterative DFS with explicit stack and path reconstruction.
///
/// Time: O(n^2) worst case.
/// Space: O(n).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return all root-to-leaf paths with given sum using iterative DFS.
        pub fn path_sum(root: Node, target_sum: i32) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            let Some(r) = root else { return result };
            // Stack: (node, remaining_sum, path_so_far)
            let mut stack = vec![(r, target_sum, Vec::new())];

            while let Some((node, remain, mut path)) = stack.pop() {
                let node = node.borrow();
                path.push(node.val);
                let new_remain = remain - node.val;

                if node.left.is_none() && node.right.is_none() && new_remain == 0 {
                    result.push(path);
                    continue;
                }
                if let Some(ref r) = node.right {
                    stack.push((Rc::clone(r), new_remain, path.clone()));
                }
                if let Some(ref l) = node.left {
                    stack.push((Rc::clone(l), new_remain, path));
                }
            }
            result
        }
    }
}

fn main() {
    let root = build_tree(&[
        Some(5),
        Some(4),
        Some(8),
        Some(11),
        None,
        Some(13),
        Some(4),
        Some(7),
        Some(2),
        None,
        None,
        Some(5),
        Some(1),
    ]);
    let my = my_solution::Solution::path_sum(root.clone(), 22);
    let ai = ai_solution::Solution::path_sum(root, 22);
    println!("paths: my={my:?}, ai={ai:?}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_path_sum {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_example1() {
                    let root = build_tree(&[
                        Some(5),
                        Some(4),
                        Some(8),
                        Some(11),
                        None,
                        Some(13),
                        Some(4),
                        Some(7),
                        Some(2),
                        None,
                        None,
                        Some(5),
                        Some(1),
                    ]);
                    let mut result = <$solution>::path_sum(root, 22);
                    result.sort();
                    assert_eq!(result, vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]],);
                }

                #[test]
                fn test_no_path() {
                    let root = build_tree(&[Some(1), Some(2), Some(3)]);
                    assert_eq!(<$solution>::path_sum(root, 5), Vec::<Vec<i32>>::new(),);
                }

                #[test]
                fn test_empty() {
                    assert_eq!(<$solution>::path_sum(None, 0), Vec::<Vec<i32>>::new(),);
                }

                #[test]
                fn test_single_match() {
                    let root = build_tree(&[Some(1)]);
                    assert_eq!(<$solution>::path_sum(root, 1), vec![vec![1]]);
                }

                #[test]
                fn test_negative() {
                    let root = build_tree(&[Some(-2), None, Some(-3)]);
                    assert_eq!(<$solution>::path_sum(root, -5), vec![vec![-2, -3]]);
                }
            }
        };
    }

    test_path_sum!(my, crate::my_solution::Solution);
    test_path_sum!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=15).map(Some).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::path_sum(
                std::hint::black_box(root),
                std::hint::black_box(15),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::path_sum(
                std::hint::black_box(root),
                std::hint::black_box(15),
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
