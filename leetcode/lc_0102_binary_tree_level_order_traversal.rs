// LeetCode 102: Binary Tree Level Order Traversal [Medium]
// https://leetcode.com/problems/binary-tree-level-order-traversal/
//
// Given the root of a binary tree, return the level order traversal of
// its nodes' values (i.e., from left to right, level by level).
//
// Examples:
//   root = [3,9,20,null,null,15,7] -> [[3],[9,20],[15,7]]
//   root = [1]                     -> [[1]]
//   root = []                      -> []
//
// Constraints:
//   - The number of nodes in the tree is in [0, 2000]
//   - -1000 <= Node.val <= 1000
//
// Benchmark (100k iterations):
//   My solution:  131.3ms (avg 1.313us) -- VecDeque BFS
//   AI solution:  153.6ms (avg 1.536us) -- Vec-swap BFS
//   -> My solution is 1.17x faster
//
// Rust Concepts:
//   - Rc<RefCell<TreeNode>> for shared mutable tree nodes
//   - VecDeque for O(1) pop_front (Ruby Array#shift equivalent)
//   - .borrow() to access RefCell contents (shared reference)
//   - if let Some(x) = ... for Option pattern matching (Ruby nil check)
//   - .clone() on Rc increments reference count (cheap pointer copy)
//   - Vec-swap BFS: process whole level as Vec, swap current/next
//   - Vec::with_capacity() to pre-allocate known sizes
//
// Run:  cargo run --example lc_0102_binary_tree_level_order_traversal
// Test: cargo test --example lc_0102_binary_tree_level_order_traversal
// Bench: cargo test --example lc_0102_binary_tree_level_order_traversal bench -- --nocapture

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
            // Assign left child.
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    current.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            // Assign right child.
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
/// Time:  O(n) -- visits each node exactly once
/// Space: O(n) -- VecDeque stores up to n nodes; result holds all n values
mod my_solution {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    use super::TreeNode;

    pub struct Solution;

    impl Solution {
        /// Return the level order traversal of a binary tree's node values.
        ///
        /// # Arguments
        ///
        /// * `root` - The root of the binary tree
        ///
        /// # Returns
        ///
        /// A Vec of Vecs, each inner Vec containing values at that level
        pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            // Ruby: return [] if root.nil?
            // Rust: Option wraps nullable values — None = nil, Some(_) = present.
            // Early return with empty vec if root is None.
            let root = match root {
                Some(node) => node,
                None => return vec![],
            };
            // vec![] creates an empty Vec — type is inferred from the annotation.
            // Ruby's [] is untyped; Rust needs to know T at compile time.
            let mut result: Vec<Vec<i32>> = vec![];
            // Ruby: queue = [root]
            // Rust: VecDeque gives O(1) pop_front; Vec's remove(0) is O(n).
            let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            queue.push_back(root);
            while !queue.is_empty() {
                let level_size = queue.len();
                let mut current_level: Vec<i32> = vec![];
                for _ in 0..level_size {
                    // Ruby: node = queue.shift
                    // Rust: pop_front returns Option — .expect() is safe since
                    // we only loop level_size times (guaranteed non-empty).
                    let node = queue.pop_front().expect("queue non-empty");
                    // Ruby: node.val / node.left / node.right accessed directly.
                    // Rust: Rc<RefCell<T>> requires .borrow() to get a shared ref.
                    let node_ref = node.borrow();
                    // Ruby: current_level << node.val
                    current_level.push(node_ref.val);
                    // Ruby: queue << node.left if node.left
                    // Rust: if let Some(x) = ... matches the Some variant (non-nil).
                    // .clone() increments the Rc reference count (cheap pointer copy).
                    if let Some(left) = node_ref.left.clone() {
                        queue.push_back(left);
                    }
                    // Ruby: queue << node.right if node.right
                    if let Some(right) = node_ref.right.clone() {
                        queue.push_back(right);
                    }
                }
                result.push(current_level);
            }

            result
        }
    }
}

/// AI solution module.
///
/// Uses a Vec-swap BFS instead of VecDeque. Since we process entire
/// levels at once, we drain the current-level Vec and collect children
/// into a next-level Vec, then swap. This avoids VecDeque's ring-buffer
/// overhead and is more cache-friendly for level-by-level traversal.
///
/// Time:  O(n) — visits each node exactly once
/// Space: O(w) — w = max width of the tree (widest level stored in Vec)
mod ai_solution {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::TreeNode;

    pub struct Solution;

    impl Solution {
        /// Return the level order traversal of a binary tree's node values.
        ///
        /// # Arguments
        ///
        /// * `root` - The root of the binary tree
        ///
        /// # Returns
        ///
        /// A Vec of Vecs, each inner Vec containing values at that level
        pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            let root = match root {
                Some(node) => node,
                None => return vec![],
            };

            let mut result = Vec::new();
            // Current level nodes — start with just the root.
            let mut current = vec![root];

            while !current.is_empty() {
                let mut vals = Vec::with_capacity(current.len());
                // Pre-allocate next level: binary tree can have at most 2x children.
                let mut next = Vec::with_capacity(current.len() * 2);

                for node in &current {
                    let node_ref = node.borrow();
                    vals.push(node_ref.val);
                    if let Some(ref left) = node_ref.left {
                        next.push(Rc::clone(left));
                    }
                    if let Some(ref right) = node_ref.right {
                        next.push(Rc::clone(right));
                    }
                }

                result.push(vals);
                current = next;
            }

            result
        }
    }
}

#[allow(clippy::type_complexity)]
fn main() {
    let test_cases: Vec<(Vec<Option<i32>>, Vec<Vec<i32>>)> = vec![
        (
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            vec![vec![3], vec![9, 20], vec![15, 7]],
        ),
        (vec![Some(1)], vec![vec![1]]),
        (vec![], vec![]),
    ];

    println!("--- My Solution ---");
    for (vals, expected) in &test_cases {
        let root = build_tree(vals);
        let result = my_solution::Solution::level_order(root);
        println!("  {vals:?} -> {result:?} (expected {expected:?})");
    }

    println!("--- AI Solution ---");
    for (vals, expected) in &test_cases {
        let root = build_tree(vals);
        let result = ai_solution::Solution::level_order(root);
        println!("  {vals:?} -> {result:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    macro_rules! test_level_order {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                use super::super::build_tree;

                #[test]
                fn test_basic_tree() {
                    let root =
                        build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
                    assert_eq!(
                        Solution::level_order(root),
                        vec![vec![3], vec![9, 20], vec![15, 7]]
                    );
                }

                #[test]
                fn test_single_node() {
                    let root = build_tree(&[Some(1)]);
                    assert_eq!(Solution::level_order(root), vec![vec![1]]);
                }

                #[test]
                fn test_empty_tree() {
                    let root = build_tree(&[]);
                    let expected: Vec<Vec<i32>> = vec![];
                    assert_eq!(Solution::level_order(root), expected);
                }

                #[test]
                fn test_left_skewed() {
                    let root = build_tree(&[Some(1), Some(2), None, Some(3)]);
                    assert_eq!(Solution::level_order(root), vec![vec![1], vec![2], vec![3]]);
                }

                #[test]
                fn test_right_skewed() {
                    let root = build_tree(&[Some(1), None, Some(2), None, Some(3)]);
                    assert_eq!(Solution::level_order(root), vec![vec![1], vec![2], vec![3]]);
                }

                #[test]
                fn test_full_binary_tree() {
                    let root = build_tree(&[
                        Some(1),
                        Some(2),
                        Some(3),
                        Some(4),
                        Some(5),
                        Some(6),
                        Some(7),
                    ]);
                    assert_eq!(
                        Solution::level_order(root),
                        vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]]
                    );
                }

                #[test]
                fn test_negative_values() {
                    let root = build_tree(&[Some(-10), Some(9), Some(20)]);
                    assert_eq!(Solution::level_order(root), vec![vec![-10], vec![9, 20]]);
                }
            }
        };
    }

    test_level_order!(my_tests, crate::my_solution::Solution);
    test_level_order!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let tree_vals = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];

        let start = Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&tree_vals);
            std::hint::black_box(crate::my_solution::Solution::level_order(
                std::hint::black_box(root),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let root = build_tree(&tree_vals);
            std::hint::black_box(crate::ai_solution::Solution::level_order(
                std::hint::black_box(root),
            ));
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
