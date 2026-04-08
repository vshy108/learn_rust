// LeetCode 116: Populating Next Right Pointers in Each Node [Medium]
// https://leetcode.com/problems/populating-next-right-pointers-in-each-node/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.708us
//   AI solution avg: 1.835us
//   Ratio: ~tied
//
// You are given a perfect binary tree where all leaves are on the same level.
// Populate each next pointer to point to its next right node. If there is no
// next right node, the next pointer should be set to NULL.
//
// Constraints: [0, 2^12 - 1] nodes, -1000 <= Node.val <= 1000
//
// Note: We simulate with a standard TreeNode + separate next-map since Rust
// doesn't have Node.next natively. We store next pointers in a HashMap.
//
// Run:  cargo run --example lc_0116_populating_next_right
// Test: cargo test --example lc_0116_populating_next_right
// Bench: cargo test --example lc_0116_populating_next_right bench -- --nocapture

use std::cell::RefCell;
use std::collections::HashMap;
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

/// Represent next pointers as val -> next_val mapping (values are unique in tests).
type NextMap = HashMap<i32, Option<i32>>;

/// My solution — BFS level-order, link nodes within each level.
///
/// Time: O(n).
/// Space: O(n) for queue.
mod my_solution {
    use super::*;
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Connect each node to its next right node using BFS.
        pub fn connect(root: &Node) -> NextMap {
            let mut next_map = NextMap::new();
            let Some(r) = root.as_ref() else {
                return next_map;
            };
            let mut queue = VecDeque::new();
            queue.push_back(Rc::clone(r));

            while !queue.is_empty() {
                let size = queue.len();
                for i in 0..size {
                    let node = queue.pop_front().expect("sized");
                    let node_borrow = node.borrow();
                    let next_val = if i < size - 1 {
                        queue.front().map(|n| n.borrow().val)
                    } else {
                        None
                    };
                    next_map.insert(node_borrow.val, next_val);
                    if let Some(ref l) = node_borrow.left {
                        queue.push_back(Rc::clone(l));
                    }
                    if let Some(ref r) = node_borrow.right {
                        queue.push_back(Rc::clone(r));
                    }
                }
            }
            next_map
        }
    }
}

/// AI solution — O(1) space using already-established next pointers.
/// For perfect binary tree: use parent's next to connect children.
///
/// Time: O(n).
/// Space: O(1) extra (O(n) for NextMap output).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Connect nodes using the "already connected" level above.
        pub fn connect(root: &Node) -> NextMap {
            let mut next_map = NextMap::new();
            let Some(r) = root.as_ref() else {
                return next_map;
            };

            // Process level by level using leftmost node
            let mut leftmost = Some(Rc::clone(r));
            while let Some(ref lm) = leftmost.clone() {
                let mut cur = Some(Rc::clone(lm));
                let mut prev_right: Option<Rc<RefCell<TreeNode>>> = None;

                while let Some(ref node) = cur.clone() {
                    let node_borrow = node.borrow();

                    if let Some(ref left) = node_borrow.left {
                        // Connect left -> right (same parent)
                        if let Some(ref right) = node_borrow.right {
                            next_map.insert(left.borrow().val, Some(right.borrow().val));
                        }
                        // Connect previous right -> current left
                        if let Some(ref pr) = prev_right {
                            next_map.insert(pr.borrow().val, Some(left.borrow().val));
                        }
                    }

                    if let Some(ref right) = node_borrow.right {
                        prev_right = Some(Rc::clone(right));
                    }

                    // Move to next node on this level via next_map
                    let next_val = next_map.get(&node_borrow.val).copied().flatten();
                    cur = next_val.and_then(|v| Self::find_node(root, v));
                }
                // Set the last node on this child level to None
                if let Some(ref pr) = prev_right {
                    next_map.entry(pr.borrow().val).or_insert(None);
                }

                // Move to next level (leftmost child of leftmost)
                leftmost = lm.borrow().left.clone();
            }
            // Root's next is always None
            next_map.insert(r.borrow().val, None);
            next_map
        }

        fn find_node(root: &Node, val: i32) -> Node {
            let Some(n) = root else { return None };
            let n_borrow = n.borrow();
            if n_borrow.val == val {
                return Some(Rc::clone(n));
            }
            Self::find_node(&n_borrow.left, val).or_else(|| Self::find_node(&n_borrow.right, val))
        }
    }
}

fn main() {
    let root = build_tree(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
    ]);
    let my = my_solution::Solution::connect(&root);
    let ai = ai_solution::Solution::connect(&root);
    println!("my next: {my:?}");
    println!("ai next: {ai:?}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_connect {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_perfect_tree() {
                    let root = build_tree(&[
                        Some(1),
                        Some(2),
                        Some(3),
                        Some(4),
                        Some(5),
                        Some(6),
                        Some(7),
                    ]);
                    let next = <$solution>::connect(&root);
                    assert_eq!(next.get(&1).copied().flatten(), None);
                    assert_eq!(next.get(&2).copied().flatten(), Some(3));
                    assert_eq!(next.get(&3).copied().flatten(), None);
                    assert_eq!(next.get(&4).copied().flatten(), Some(5));
                    assert_eq!(next.get(&5).copied().flatten(), Some(6));
                    assert_eq!(next.get(&6).copied().flatten(), Some(7));
                    assert_eq!(next.get(&7).copied().flatten(), None);
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(1)]);
                    let next = <$solution>::connect(&root);
                    assert_eq!(next.get(&1).copied().flatten(), None);
                }

                #[test]
                fn test_empty() {
                    let next = <$solution>::connect(&None);
                    assert!(next.is_empty());
                }

                #[test]
                fn test_two_levels() {
                    let root = build_tree(&[Some(1), Some(2), Some(3)]);
                    let next = <$solution>::connect(&root);
                    assert_eq!(next.get(&1).copied().flatten(), None);
                    assert_eq!(next.get(&2).copied().flatten(), Some(3));
                    assert_eq!(next.get(&3).copied().flatten(), None);
                }
            }
        };
    }

    test_connect!(my, crate::my_solution::Solution);
    test_connect!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = (1..=15).map(Some).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::my_solution::Solution::connect(std::hint::black_box(
                &root,
            )));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&vals);
            std::hint::black_box(crate::ai_solution::Solution::connect(std::hint::black_box(
                &root,
            )));
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
