// LeetCode 156: Binary Tree Upside Down [Medium] (Premium)
// https://leetcode.com/problems/binary-tree-upside-down/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.511us
//   AI solution avg: 0.378us
//   Ratio: AI 1.35x faster
//
// Given the root of a binary tree, turn the tree upside down and return
// the new root.
//
// You can turn a binary tree upside down with the following steps:
//   1. The original left child becomes the new root.
//   2. The original root becomes the new right child.
//   3. The original right child becomes the new left child.
//
// The mentioned steps are done level by level. It is guaranteed that every
// right node has a sibling (a left node with the same parent) and has no
// children.
//
// Example 1: root = [1,2,3,4,5] -> [4,5,2,null,null,3,1]
// Example 2: root = [] -> []
// Example 3: root = [1] -> [1]
//
// Constraints:
//   - The number of nodes in the tree will be in the range [0, 10].
//   - 1 <= Node.val <= 10
//   - Every right node has a sibling (left node with same parent).
//   - Every right node has no children.
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0100-0199/0156.Binary%20Tree%20Upside%20Down/README_EN.md
//
// Run:  cargo run --example lc_0156_binary_tree_upside_down
// Test: cargo test --example lc_0156_binary_tree_upside_down
// Bench: cargo test --example lc_0156_binary_tree_upside_down bench -- --nocapture

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
    /// Create a new tree node with the given value.
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Helper to wrap a TreeNode in Rc<RefCell<...>>.
fn wrap(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(node)))
}

/// My solution — recursive approach (translated from Python in doocs/leetcode).
///
/// Recurse down the left spine. Once we reach the leftmost node (new root),
/// rewire on the way back up: the left child's right becomes the current
/// node, the left child's left becomes the current node's right child.
/// Then null out current node's children.
///
/// Time: O(n) — visit each node once.
/// Space: O(h) — recursion depth equals tree height.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Flip the binary tree upside down and return the new root.
        pub fn upside_down_binary_tree(
            root: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            // Base case: empty tree or leaf node (no left child)
            let node = match &root {
                Some(n) if n.borrow().left.is_some() => n.clone(),
                _ => return root,
            };

            let left = node.borrow().left.clone();
            // Recurse down the left spine to find the new root
            let new_root = Self::upside_down_binary_tree(left);

            // Rewire: left_child.right = current, left_child.left = current.right
            let borrowed = node.borrow();
            if let Some(ref left_child) = borrowed.left {
                let mut lc = left_child.borrow_mut();
                lc.right = Some(node.clone());
                lc.left = borrowed.right.clone();
            }
            drop(borrowed);

            // Null out current node's children (it's now a leaf or mid-node)
            node.borrow_mut().left = None;
            node.borrow_mut().right = None;

            new_root
        }
    }
}

/// AI solution — iterative approach using three pointers.
///
/// Walk down the left spine, maintaining prev, prev_right, and current.
/// At each step, rewire: current.left = prev_right, current.right = prev.
/// Advance all three pointers. No recursion needed.
///
/// Time: O(n) — single pass down the left spine.
/// Space: O(1) — only three pointers, no recursion stack.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Flip the binary tree upside down and return the new root.
        pub fn upside_down_binary_tree(
            root: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let mut curr = root;
            let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
            let mut prev_right: Option<Rc<RefCell<TreeNode>>> = None;

            while let Some(node) = curr {
                // Save next (left child) before we overwrite it
                let next = node.borrow().left.clone();
                let curr_right = node.borrow().right.clone();

                // Rewire current node
                node.borrow_mut().left = prev_right;
                node.borrow_mut().right = prev;

                // Advance pointers
                prev = Some(node);
                prev_right = curr_right;
                curr = next;
            }

            prev
        }
    }
}

/// Build a tree from a level-order array (None = null node).
fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = wrap(TreeNode::new(values[0].unwrap()));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone().unwrap());

    let mut i = 1;
    while i < values.len() {
        if let Some(parent) = queue.pop_front() {
            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = wrap(TreeNode::new(val));
                    parent.borrow_mut().left = left.clone();
                    queue.push_back(left.unwrap());
                }
                i += 1;
            }
            if i < values.len() {
                if let Some(val) = values[i] {
                    let right = wrap(TreeNode::new(val));
                    parent.borrow_mut().right = right.clone();
                    queue.push_back(right.unwrap());
                }
                i += 1;
            }
        }
    }

    root
}

/// Serialize a tree to level-order vec (with trailing nulls trimmed).
fn tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    let mut queue = std::collections::VecDeque::new();

    if let Some(r) = root {
        queue.push_back(Some(r.clone()));
    } else {
        return result;
    }

    while let Some(item) = queue.pop_front() {
        if let Some(node) = item {
            let borrowed = node.borrow();
            result.push(Some(borrowed.val));
            queue.push_back(borrowed.left.clone());
            queue.push_back(borrowed.right.clone());
        } else {
            result.push(None);
        }
    }

    // Trim trailing Nones
    while result.last() == Some(&None) {
        result.pop();
    }

    result
}

fn main() {
    let cases: Vec<(Vec<Option<i32>>, Vec<Option<i32>>)> = vec![
        (
            vec![Some(1), Some(2), Some(3), Some(4), Some(5)],
            vec![Some(4), Some(5), Some(2), None, None, Some(3), Some(1)],
        ),
        (vec![], vec![]),
        (vec![Some(1)], vec![Some(1)]),
    ];

    for (input, expected) in &cases {
        let tree1 = build_tree(input);
        let my = my_solution::Solution::upside_down_binary_tree(tree1);
        let my_vec = tree_to_vec(&my);

        let tree2 = build_tree(input);
        let ai = ai_solution::Solution::upside_down_binary_tree(tree2);
        let ai_vec = tree_to_vec(&ai);

        println!("{input:?} -> my={my_vec:?}, ai={ai_vec:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_upside_down {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    let root = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
                    let result = <$solution>::upside_down_binary_tree(root);
                    assert_eq!(
                        tree_to_vec(&result),
                        vec![Some(4), Some(5), Some(2), None, None, Some(3), Some(1)],
                    );
                }

                #[test]
                fn test_empty() {
                    let root = build_tree(&[]);
                    let result = <$solution>::upside_down_binary_tree(root);
                    assert_eq!(tree_to_vec(&result), Vec::<Option<i32>>::new());
                }

                #[test]
                fn test_single_node() {
                    let root = build_tree(&[Some(1)]);
                    let result = <$solution>::upside_down_binary_tree(root);
                    assert_eq!(tree_to_vec(&result), vec![Some(1)]);
                }

                #[test]
                fn test_two_levels() {
                    // [1, 2, 3] -> [2, 3, 1]
                    let root = build_tree(&[Some(1), Some(2), Some(3)]);
                    let result = <$solution>::upside_down_binary_tree(root);
                    assert_eq!(tree_to_vec(&result), vec![Some(2), Some(3), Some(1)]);
                }
            }
        };
    }

    test_upside_down!(my, crate::my_solution::Solution);
    test_upside_down!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let input = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let tree = build_tree(&input);
            std::hint::black_box(crate::my_solution::Solution::upside_down_binary_tree(
                std::hint::black_box(tree),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let tree = build_tree(&input);
            std::hint::black_box(crate::ai_solution::Solution::upside_down_binary_tree(
                std::hint::black_box(tree),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
