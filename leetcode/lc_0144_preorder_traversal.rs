// LeetCode 144: Binary Tree Preorder Traversal [Easy]
// https://leetcode.com/problems/binary-tree-preorder-traversal/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.293us
//   AI solution avg: 0.393us
//   Ratio: My 1.34x faster
//
// Given the root of a binary tree, return the preorder traversal of its values.
//
// Constraints: [0, 100] nodes, -100 <= Node.val <= 100
//
// Run:  cargo run --example lc_0144_preorder_traversal
// Test: cargo test --example lc_0144_preorder_traversal
// Bench: cargo test --example lc_0144_preorder_traversal bench -- --nocapture

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
    if vals.is_empty() || vals[0].is_none() { return None; }
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

/// My solution — recursive preorder.
///
/// Time: O(n).
/// Space: O(h) recursion stack.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return preorder traversal.
        pub fn preorder_traversal(root: Node) -> Vec<i32> {
            let mut result = Vec::new();
            Self::dfs(&root, &mut result);
            result
        }

        fn dfs(node: &Node, out: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                out.push(n.val);
                Self::dfs(&n.left, out);
                Self::dfs(&n.right, out);
            }
        }
    }
}

/// AI solution — iterative using explicit stack.
///
/// Time: O(n).
/// Space: O(h) for the stack.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return preorder traversal (iterative).
        pub fn preorder_traversal(root: Node) -> Vec<i32> {
            let Some(root) = root else { return vec![] };
            let mut result = Vec::new();
            let mut stack = vec![root];

            while let Some(node) = stack.pop() {
                let n = node.borrow();
                result.push(n.val);
                // Push right first so left is processed first (LIFO)
                if let Some(ref r) = n.right {
                    stack.push(Rc::clone(r));
                }
                if let Some(ref l) = n.left {
                    stack.push(Rc::clone(l));
                }
            }
            result
        }
    }
}

fn main() {
    let root = build_tree(&[Some(1), None, Some(2), Some(3)]);
    let my = my_solution::Solution::preorder_traversal(root.clone());
    let ai = ai_solution::Solution::preorder_traversal(root);
    println!("preorder: my={my:?}, ai={ai:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_preorder {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    let root = build_tree(&[Some(1), None, Some(2), Some(3)]);
                    assert_eq!(
                        <$solution>::preorder_traversal(root),
                        vec![1, 2, 3],
                    );
                }

                #[test]
                fn test_example2() {
                    let root = build_tree(&[
                        Some(1), Some(2), Some(3), Some(4), Some(5),
                        None, Some(8), None, None, Some(6), Some(7),
                        Some(9),
                    ]);
                    assert_eq!(
                        <$solution>::preorder_traversal(root),
                        vec![1, 2, 4, 5, 6, 7, 3, 8, 9],
                    );
                }

                #[test]
                fn test_empty() {
                    assert_eq!(<$solution>::preorder_traversal(None), Vec::<i32>::new());
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(1)]);
                    assert_eq!(<$solution>::preorder_traversal(root), vec![1]);
                }
            }
        };
    }

    test_preorder!(my, crate::my_solution::Solution);
    test_preorder!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=15).map(Some).collect();
        let root = build_tree(&vals);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::preorder_traversal(
                std::hint::black_box(root.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::preorder_traversal(
                std::hint::black_box(root.clone()),
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
            println!("Both solutions are roughly equal");
        }
    }
}
