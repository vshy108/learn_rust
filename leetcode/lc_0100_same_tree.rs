// LeetCode 100: Same Tree [Easy]
// https://leetcode.com/problems/same-tree/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 10.388us
//   AI solution avg: 11.468us
//   Ratio: ~tied
//
// Given the roots of two binary trees p and q, check if they are the same
// (structurally identical and same node values).
//
// Constraints: [0, 100] nodes, -10^4 <= Node.val <= 10^4
//
// Run:  cargo run --example lc_0100_same_tree
// Test: cargo test --example lc_0100_same_tree
// Bench: cargo test --example lc_0100_same_tree bench -- --nocapture

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

/// My solution — recursive comparison.
///
/// Time: O(n).
/// Space: O(h) where h = tree height.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Check if two binary trees are identical.
        ///
        /// # Arguments
        /// * `p` - first tree
        /// * `q` - second tree
        ///
        /// # Returns
        /// true if trees are structurally identical with same values
        pub fn is_same_tree(p: Node, q: Node) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    let a = a.borrow();
                    let b = b.borrow();
                    a.val == b.val
                        && Self::is_same_tree(a.left.clone(), b.left.clone())
                        && Self::is_same_tree(a.right.clone(), b.right.clone())
                }
                _ => false,
            }
        }
    }
}

/// AI solution — iterative BFS comparison.
///
/// Time: O(n).
/// Space: O(n) for queue.
mod ai_solution {
    use super::*;
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Check if two binary trees are identical.
        ///
        /// # Arguments
        /// * `p` - first tree
        /// * `q` - second tree
        ///
        /// # Returns
        /// true if trees are structurally identical with same values
        pub fn is_same_tree(p: Node, q: Node) -> bool {
            let mut queue: VecDeque<(Node, Node)> = VecDeque::new();
            queue.push_back((p, q));

            while let Some((a, b)) = queue.pop_front() {
                match (a, b) {
                    (None, None) => continue,
                    (Some(na), Some(nb)) => {
                        let na = na.borrow();
                        let nb = nb.borrow();
                        if na.val != nb.val {
                            return false;
                        }
                        queue.push_back((na.left.clone(), nb.left.clone()));
                        queue.push_back((na.right.clone(), nb.right.clone()));
                    }
                    _ => return false,
                }
            }
            true
        }
    }
}

fn main() {
    let p = build_tree(&[Some(1), Some(2), Some(3)]);
    let q = build_tree(&[Some(1), Some(2), Some(3)]);
    let my = my_solution::Solution::is_same_tree(p.clone(), q.clone());
    let ai = ai_solution::Solution::is_same_tree(p, q);
    println!("same trees: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_same_tree {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_same() {
                    let p = build_tree(&[Some(1), Some(2), Some(3)]);
                    let q = build_tree(&[Some(1), Some(2), Some(3)]);
                    assert!(<$solution>::is_same_tree(p, q));
                }

                #[test]
                fn test_different_structure() {
                    let p = build_tree(&[Some(1), Some(2)]);
                    let q = build_tree(&[Some(1), None, Some(2)]);
                    assert!(!<$solution>::is_same_tree(p, q));
                }

                #[test]
                fn test_different_values() {
                    let p = build_tree(&[Some(1), Some(2), Some(1)]);
                    let q = build_tree(&[Some(1), Some(1), Some(2)]);
                    assert!(!<$solution>::is_same_tree(p, q));
                }

                #[test]
                fn test_both_empty() {
                    assert!(<$solution>::is_same_tree(None, None));
                }

                #[test]
                fn test_one_empty() {
                    let p = build_tree(&[Some(1)]);
                    assert!(!<$solution>::is_same_tree(p, None));
                }
            }
        };
    }

    test_same_tree!(my, crate::my_solution::Solution);
    test_same_tree!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=31).map(Some).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let p = build_tree(&vals);
            let q = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::is_same_tree(
                std::hint::black_box(p),
                std::hint::black_box(q),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let p = build_tree(&vals);
            let q = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::is_same_tree(
                std::hint::black_box(p),
                std::hint::black_box(q),
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
