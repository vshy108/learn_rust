// LeetCode 117: Populating Next Right Pointers in Each Node II [Medium]
// https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.748us
//   AI solution avg: 1.630us
//   Ratio: ~tied
//
// Same as 116 but the binary tree is NOT necessarily perfect.
// Populate each next pointer to point to its next right node.
//
// Constraints: [0, 6000] nodes, -100 <= Node.val <= 100
//
// Run:  cargo run --example lc_0117_populating_next_right_ii
// Test: cargo test --example lc_0117_populating_next_right_ii
// Bench: cargo test --example lc_0117_populating_next_right_ii bench -- --nocapture

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

/// Result: map val -> next_val (None if last in level).
type NextMap = std::collections::HashMap<i32, Option<i32>>;

/// My solution — BFS level-order traversal, link nodes within each level.
///
/// Time: O(n).
/// Space: O(n) for the queue.
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

/// AI solution — same BFS approach (optimal for arbitrary trees).
/// Uses a level vec instead of deque for slightly different iteration pattern.
///
/// Time: O(n).
/// Space: O(n).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Connect nodes using level-by-level BFS with vec swap.
        pub fn connect(root: &Node) -> NextMap {
            let mut next_map = NextMap::new();
            let Some(r) = root.as_ref() else {
                return next_map;
            };
            let mut level = vec![Rc::clone(r)];

            while !level.is_empty() {
                let mut next_level = Vec::new();
                for (i, node) in level.iter().enumerate() {
                    let node_borrow = node.borrow();
                    let next_val = if i + 1 < level.len() {
                        Some(level[i + 1].borrow().val)
                    } else {
                        None
                    };
                    next_map.insert(node_borrow.val, next_val);
                    if let Some(ref l) = node_borrow.left {
                        next_level.push(Rc::clone(l));
                    }
                    if let Some(ref r) = node_borrow.right {
                        next_level.push(Rc::clone(r));
                    }
                }
                level = next_level;
            }
            next_map
        }
    }
}

fn main() {
    let root = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7)]);
    let my = my_solution::Solution::connect(&root);
    let ai = ai_solution::Solution::connect(&root);
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_connect {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::build_tree;

                #[test]
                fn test_example1() {
                    let root =
                        build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7)]);
                    let next = <$solution>::connect(&root);
                    assert_eq!(next.get(&1).copied().flatten(), None);
                    assert_eq!(next.get(&2).copied().flatten(), Some(3));
                    assert_eq!(next.get(&3).copied().flatten(), None);
                    assert_eq!(next.get(&4).copied().flatten(), Some(5));
                    assert_eq!(next.get(&5).copied().flatten(), Some(7));
                    assert_eq!(next.get(&7).copied().flatten(), None);
                }

                #[test]
                fn test_empty() {
                    let next = <$solution>::connect(&None);
                    assert!(next.is_empty());
                }

                #[test]
                fn test_single() {
                    let root = build_tree(&[Some(1)]);
                    let next = <$solution>::connect(&root);
                    assert_eq!(next.get(&1).copied().flatten(), None);
                }

                #[test]
                fn test_left_only() {
                    let root = build_tree(&[Some(1), Some(2), None, Some(3)]);
                    let next = <$solution>::connect(&root);
                    assert_eq!(next.get(&1).copied().flatten(), None);
                    assert_eq!(next.get(&2).copied().flatten(), None);
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
