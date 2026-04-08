// LeetCode 101: Symmetric Tree [Easy]
// https://leetcode.com/problems/symmetric-tree/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.062us
//   AI solution avg: 1.077us
//   Ratio: ~tied
//
// Given the root of a binary tree, check whether it is a mirror of itself
// (i.e., symmetric around its center).
//
// Constraints: [1, 1000] nodes, -100 <= Node.val <= 100
//
// Run:  cargo run --example lc_0101_symmetric_tree
// Test: cargo test --example lc_0101_symmetric_tree
// Bench: cargo test --example lc_0101_symmetric_tree bench -- --nocapture

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

/// My solution — recursive mirror check.
///
/// Time: O(n) visits every node once.
/// Space: O(h) recursion stack where h = tree height.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Check if a binary tree is symmetric.
        ///
        /// # Arguments
        /// * `root` - root of the binary tree
        ///
        /// # Returns
        /// true if the tree is symmetric around its center
        pub fn is_symmetric(root: Node) -> bool {
            match root {
                None => true,
                Some(r) => {
                    let r = r.borrow();
                    Self::is_mirror(&r.left, &r.right)
                }
            }
        }

        /// Recursively compare two subtrees as mirrors.
        fn is_mirror(left: &Node, right: &Node) -> bool {
            match (left, right) {
                (None, None) => true,
                (Some(l), Some(r)) => {
                    let l = l.borrow();
                    let r = r.borrow();
                    l.val == r.val
                        && Self::is_mirror(&l.left, &r.right)
                        && Self::is_mirror(&l.right, &r.left)
                }
                _ => false,
            }
        }
    }
}

/// AI solution — iterative BFS with queue comparing mirror pairs.
///
/// Time: O(n).
/// Space: O(n) for the queue.
mod ai_solution {
    use super::*;
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Check if a binary tree is symmetric using iterative BFS.
        ///
        /// # Arguments
        /// * `root` - root of the binary tree
        ///
        /// # Returns
        /// true if the tree is symmetric around its center
        pub fn is_symmetric(root: Node) -> bool {
            let Some(r) = root else { return true };
            let r = r.borrow();
            let mut queue: VecDeque<(Node, Node)> = VecDeque::new();
            queue.push_back((r.left.clone(), r.right.clone()));

            while let Some((a, b)) = queue.pop_front() {
                match (a, b) {
                    (None, None) => continue,
                    (Some(na), Some(nb)) => {
                        let na = na.borrow();
                        let nb = nb.borrow();
                        if na.val != nb.val {
                            return false;
                        }
                        // Mirror: left-left with right-right, left-right with right-left
                        queue.push_back((na.left.clone(), nb.right.clone()));
                        queue.push_back((na.right.clone(), nb.left.clone()));
                    }
                    _ => return false,
                }
            }
            true
        }
    }
}

fn main() {
    let root = build_tree(&[
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(4),
        Some(4),
        Some(3),
    ]);
    let my = my_solution::Solution::is_symmetric(root.clone());
    let ai = ai_solution::Solution::is_symmetric(root);
    println!("symmetric: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_symmetric {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_symmetric() {
                    let root = build_tree(&[
                        Some(1),
                        Some(2),
                        Some(2),
                        Some(3),
                        Some(4),
                        Some(4),
                        Some(3),
                    ]);
                    assert!(<$solution>::is_symmetric(root));
                }

                #[test]
                fn test_not_symmetric() {
                    let root =
                        build_tree(&[Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]);
                    assert!(!<$solution>::is_symmetric(root));
                }

                #[test]
                fn test_single_node() {
                    let root = build_tree(&[Some(1)]);
                    assert!(<$solution>::is_symmetric(root));
                }

                #[test]
                fn test_empty() {
                    assert!(<$solution>::is_symmetric(None));
                }

                #[test]
                fn test_two_levels() {
                    let root = build_tree(&[Some(1), Some(2), Some(2)]);
                    assert!(<$solution>::is_symmetric(root));
                }
            }
        };
    }

    test_symmetric!(my, crate::my_solution::Solution);
    test_symmetric!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals = vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
            Some(5),
            Some(6),
            Some(6),
            Some(5),
            Some(5),
            Some(6),
            Some(6),
            Some(5),
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::is_symmetric(
                std::hint::black_box(root),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::is_symmetric(
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
