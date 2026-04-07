// LeetCode 112: Path Sum [Easy]
// https://leetcode.com/problems/path-sum/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.041us
//   AI solution avg: 2.222us
//   Ratio: ~tied
//
// Given the root of a binary tree and an integer targetSum, return true if the
// tree has a root-to-leaf path such that adding up all the values along the
// path equals targetSum.
//
// Constraints: [0, 5000] nodes, -1000 <= Node.val <= 1000,
// -1000 <= targetSum <= 1000
//
// Run:  cargo run --example lc_0112_path_sum
// Test: cargo test --example lc_0112_path_sum
// Bench: cargo test --example lc_0112_path_sum bench -- --nocapture

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

/// My solution — recursive DFS subtracting node value from target.
///
/// Time: O(n).
/// Space: O(h) recursion.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Check if there exists a root-to-leaf path summing to target.
        pub fn has_path_sum(root: Node, target_sum: i32) -> bool {
            match root {
                None => false,
                Some(node) => {
                    let node = node.borrow();
                    let remain = target_sum - node.val;
                    // Leaf check
                    if node.left.is_none() && node.right.is_none() {
                        return remain == 0;
                    }
                    Self::has_path_sum(node.left.clone(), remain)
                        || Self::has_path_sum(node.right.clone(), remain)
                }
            }
        }
    }
}

/// AI solution — iterative DFS with explicit stack.
///
/// Time: O(n).
/// Space: O(h) stack.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Check path sum using iterative DFS.
        pub fn has_path_sum(root: Node, target_sum: i32) -> bool {
            let Some(r) = root else { return false };
            let mut stack = vec![(r, target_sum)];

            while let Some((node, remain)) = stack.pop() {
                let node = node.borrow();
                let new_remain = remain - node.val;
                if node.left.is_none() && node.right.is_none() && new_remain == 0 {
                    return true;
                }
                if let Some(ref l) = node.left {
                    stack.push((Rc::clone(l), new_remain));
                }
                if let Some(ref r) = node.right {
                    stack.push((Rc::clone(r), new_remain));
                }
            }
            false
        }
    }
}

fn main() {
    let root = build_tree(&[
        Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4),
        Some(7), Some(2), None, None, None, Some(1),
    ]);
    let my = my_solution::Solution::has_path_sum(root.clone(), 22);
    let ai = ai_solution::Solution::has_path_sum(root, 22);
    println!("has path sum 22: my={my}, ai={ai}");
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
                        Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4),
                        Some(7), Some(2), None, None, None, Some(1),
                    ]);
                    assert!(<$solution>::has_path_sum(root, 22));
                }

                #[test]
                fn test_no_path() {
                    let root = build_tree(&[Some(1), Some(2), Some(3)]);
                    assert!(!<$solution>::has_path_sum(root, 5));
                }

                #[test]
                fn test_empty() {
                    assert!(!<$solution>::has_path_sum(None, 0));
                }

                #[test]
                fn test_single_match() {
                    let root = build_tree(&[Some(1)]);
                    assert!(<$solution>::has_path_sum(root, 1));
                }

                #[test]
                fn test_single_no_match() {
                    let root = build_tree(&[Some(1)]);
                    assert!(!<$solution>::has_path_sum(root, 2));
                }

                #[test]
                fn test_negative_values() {
                    let root = build_tree(&[Some(-2), None, Some(-3)]);
                    assert!(<$solution>::has_path_sum(root, -5));
                }
            }
        };
    }

    test_path_sum!(my, crate::my_solution::Solution);
    test_path_sum!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=31).map(Some).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::has_path_sum(
                std::hint::black_box(root),
                std::hint::black_box(100),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::has_path_sum(
                std::hint::black_box(root),
                std::hint::black_box(100),
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
