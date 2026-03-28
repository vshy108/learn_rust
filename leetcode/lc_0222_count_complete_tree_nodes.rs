// LeetCode 222: Count Complete Tree Nodes [Easy]
// https://leetcode.com/problems/count-complete-tree-nodes/
//
// Given the root of a complete binary tree, return the number of the
// nodes in the tree.
//
// In a complete binary tree, every level except possibly the last is
// completely filled, and all nodes in the last level are as far left
// as possible. Design an algorithm that runs in less than O(n) time.
//
// Examples:
//   root = [1,2,3,4,5,6] -> 6
//   root = [1]            -> 1
//   root = []             -> 0
//
// Constraints:
//   - The number of nodes in the tree is in [0, 5 * 10^4]
//   - 0 <= Node.val <= 5 * 10^4
//   - The tree is guaranteed to be complete
//
// Run:  cargo run --example lc_0222_count_complete_tree_nodes
// Test: cargo test --example lc_0222_count_complete_tree_nodes
// Bench: cargo test --example lc_0222_count_complete_tree_nodes bench -- --nocapture

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
    /// Create a new TreeNode with the given value.
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Build a binary tree from a level-order vec of Option<i32>.
///
/// `None` entries represent null nodes. Uses a queue (VecDeque) to
/// assign children in breadth-first order, mirroring LeetCode's
/// serialization format.
fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;

    if vals.is_empty() || vals[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    let mut i = 1;
    while i < vals.len() {
        if let Some(current) = queue.pop_front() {
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    current.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    current.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }
    }

    Some(root)
}

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Count the number of nodes in a complete binary tree.
        ///
        /// # Arguments
        ///
        /// * `root` - Root of the complete binary tree
        ///
        /// # Returns
        ///
        /// The total number of nodes
        pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            // return 0 if root.nil?
            // left_height = 0; node = root
            // while node: left_height += 1; node = node.left
            // right_height = 0; node = root
            // while node: right_height += 1; node = node.right
            // if left_height == right_height
            //   return 2**left_height - 1
            // end
            // 1 + count_nodes(root.left) + count_nodes(root.right)
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Count the number of nodes in a complete binary tree.
        ///
        /// # Arguments
        ///
        /// * `root` - Root of the complete binary tree
        ///
        /// # Returns
        ///
        /// The total number of nodes
        pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<Option<i32>>, i32)> = vec![
        (
            vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
            6,
        ),
        (vec![Some(1)], 1),
        (vec![], 0),
    ];

    println!("--- My Solution ---");
    for (vals, expected) in &test_cases {
        let root = build_tree(vals);
        let result = my_solution::Solution::count_nodes(root);
        println!("  tree={vals:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (vals, expected) in &test_cases {
        let root = build_tree(vals);
        let result = ai_solution::Solution::count_nodes(root);
        println!("  tree={vals:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_count_nodes {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;
                use $solution as Solution;

                #[test]
                fn test_full_tree_6_nodes() {
                    let root = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
                    assert_eq!(Solution::count_nodes(root), 6);
                }

                #[test]
                fn test_single_node() {
                    let root = build_tree(&[Some(1)]);
                    assert_eq!(Solution::count_nodes(root), 1);
                }

                #[test]
                fn test_empty_tree() {
                    let root = build_tree(&[]);
                    assert_eq!(Solution::count_nodes(root), 0);
                }

                #[test]
                fn test_perfect_tree_7_nodes() {
                    let root = build_tree(&[
                        Some(1),
                        Some(2),
                        Some(3),
                        Some(4),
                        Some(5),
                        Some(6),
                        Some(7),
                    ]);
                    assert_eq!(Solution::count_nodes(root), 7);
                }

                #[test]
                fn test_two_nodes() {
                    let root = build_tree(&[Some(1), Some(2)]);
                    assert_eq!(Solution::count_nodes(root), 2);
                }

                #[test]
                fn test_three_nodes() {
                    let root = build_tree(&[Some(1), Some(2), Some(3)]);
                    assert_eq!(Solution::count_nodes(root), 3);
                }

                #[test]
                fn test_complete_15_nodes() {
                    let vals: Vec<Option<i32>> = (1..=15).map(Some).collect();
                    let root = build_tree(&vals);
                    assert_eq!(Solution::count_nodes(root), 15);
                }
            }
        };
    }

    test_count_nodes!(my_tests, crate::my_solution::Solution);
    test_count_nodes!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let trees: Vec<Vec<Option<i32>>> = vec![
            vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
            (1..=15).map(Some).collect(),
            (1..=31).map(Some).collect(),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for vals in &trees {
                let root = build_tree(vals);
                std::hint::black_box(crate::my_solution::Solution::count_nodes(
                    std::hint::black_box(root),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for vals in &trees {
                let root = build_tree(vals);
                std::hint::black_box(crate::ai_solution::Solution::count_nodes(
                    std::hint::black_box(root),
                ));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
