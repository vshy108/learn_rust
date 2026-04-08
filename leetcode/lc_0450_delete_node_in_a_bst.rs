// LeetCode 450: Delete Node in a BST [Medium]
// https://leetcode.com/problems/delete-node-in-a-bst/
//
// Given a root node of a BST and a key, delete the node with the given key
// and return the (possibly updated) root. Two stages: search then delete.
//
// Examples:
//   root = [5,3,6,2,4,null,7], key = 3 -> [5,4,6,2,null,null,7]
//   root = [5,3,6,2,4,null,7], key = 0 -> [5,3,6,2,4,null,7]
//   root = [], key = 0 -> []
//
// Constraints:
//   0 <= number of nodes <= 10^4
//   -10^5 <= Node.val <= 10^5
//   Each node has a unique value.
//
// Run:  cargo run --example lc_0450_delete_node_in_a_bst
// Test: cargo test --example lc_0450_delete_node_in_a_bst
// Bench: cargo test --example lc_0450_delete_node_in_a_bst bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 376ns
//   AI solution: avg 431ns
//   Ratio: 0.87x (recursive vs iterative)

use std::cell::RefCell;
use std::rc::Rc;

/// Shared TreeNode definition.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Build a BST from a level-order array.
fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
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

/// Collect tree values via inorder traversal (sorted for BST).
fn inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            dfs(&n.left, result);
            result.push(n.val);
            dfs(&n.right, result);
        }
    }
    dfs(root, &mut result);
    result
}

/// My solution — recursive delete with inorder successor
/// (from doocs/leetcode Rust).
///
/// If key < node.val, recurse left. If key > node.val, recurse right.
/// If found: replace with inorder successor (leftmost node of right subtree).
///
/// Time:  O(h) where h is tree height
/// Space: O(h) — recursion
#[allow(dead_code)]
mod my_solution {
    use super::*;

    pub struct Solution;
    impl Solution {
        /// Delete node with given key from BST.
        ///
        /// # Arguments
        /// * `root` - BST root
        /// * `key` - value to delete
        ///
        /// # Returns
        /// Updated BST root.
        pub fn delete_node(
            mut root: Option<Rc<RefCell<TreeNode>>>,
            key: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(rc) = root.as_mut() {
                let mut node = rc.borrow_mut();
                match node.val.cmp(&key) {
                    std::cmp::Ordering::Greater => {
                        node.left = Self::delete_node(node.left.take(), key);
                    }
                    std::cmp::Ordering::Less => {
                        node.right = Self::delete_node(node.right.take(), key);
                    }
                    std::cmp::Ordering::Equal => {
                        match (node.left.is_some(), node.right.is_some()) {
                            (false, false) => return None,
                            (true, false) => return node.left.take(),
                            (false, true) => return node.right.take(),
                            (true, true) => {
                                // Find inorder successor (leftmost of right)
                                let succ_val = Self::find_min(&node.right);
                                node.val = succ_val;
                                node.right = Self::delete_node(node.right.take(), succ_val);
                            }
                        }
                    }
                }
            }
            root
        }

        /// Find the minimum value in a subtree.
        fn find_min(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let n = node.as_ref().unwrap().borrow();
            if n.left.is_none() {
                n.val
            } else {
                Self::find_min(&n.left)
            }
        }
    }
}

/// AI solution — iterative parent-tracking approach.
///
/// Find the node and its parent iteratively, then handle the three
/// deletion cases without recursion.
///
/// Time:  O(h)
/// Space: O(1) — no recursion
mod ai_solution {
    use super::*;

    pub struct Solution;
    impl Solution {
        /// Delete node iteratively.
        pub fn delete_node(
            root: Option<Rc<RefCell<TreeNode>>>,
            key: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            // Wrap in a dummy parent for uniform handling
            let dummy = Rc::new(RefCell::new(TreeNode::new(i32::MAX)));
            dummy.borrow_mut().left = root;

            // Find node and its parent
            let mut parent = Rc::clone(&dummy);
            let mut is_left = true;
            let mut current = dummy.borrow().left.clone();

            while let Some(ref node) = current.clone() {
                let val = node.borrow().val;
                if val == key {
                    break;
                }
                parent = Rc::clone(node);
                if key < val {
                    current = node.borrow().left.clone();
                    is_left = true;
                } else {
                    current = node.borrow().right.clone();
                    is_left = false;
                }
            }

            if current.is_none() {
                return dummy.borrow_mut().left.take();
            }

            let node = current.unwrap();
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();

            // Build replacement subtree
            let replacement = match (left, right) {
                (None, None) => None,
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (Some(l), Some(r)) => {
                    // Attach left subtree to leftmost node of right subtree
                    let mut leftmost = Rc::clone(&r);
                    loop {
                        let next = leftmost.borrow().left.clone();
                        match next {
                            Some(n) => leftmost = n,
                            None => break,
                        }
                    }
                    leftmost.borrow_mut().left = Some(l);
                    Some(r)
                }
            };

            if is_left {
                parent.borrow_mut().left = replacement;
            } else {
                parent.borrow_mut().right = replacement;
            }

            dummy.borrow_mut().left.take()
        }
    }
}

fn main() {
    let tree = build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
    println!(
        "my:  {:?}",
        inorder(&my_solution::Solution::delete_node(tree.clone(), 3))
    );
    println!(
        "ai:  {:?}",
        inorder(&ai_solution::Solution::delete_node(tree, 3))
    );
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::*;
        use $module::Solution;

        #[test]
        fn test_delete_internal_node() {
            let tree = build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
            let result = Solution::delete_node(tree, 3);
            let vals = inorder(&result);
            // Should contain 2,4,5,6,7 (3 removed), still sorted
            assert_eq!(vals, vec![2, 4, 5, 6, 7]);
        }

        #[test]
        fn test_delete_nonexistent() {
            let tree = build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
            let result = Solution::delete_node(tree, 0);
            assert_eq!(inorder(&result), vec![2, 3, 4, 5, 6, 7]);
        }

        #[test]
        fn test_delete_from_empty() {
            let result = Solution::delete_node(None, 0);
            assert_eq!(inorder(&result), Vec::<i32>::new());
        }

        #[test]
        fn test_delete_root() {
            let tree = build_tree(&[Some(5), Some(3), Some(7)]);
            let result = Solution::delete_node(tree, 5);
            let vals = inorder(&result);
            assert_eq!(vals, vec![3, 7]);
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Option<Rc<RefCell<TreeNode>>>, i32) -> Option<Rc<RefCell<TreeNode>>>,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let tree = build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
        std::hint::black_box(f(std::hint::black_box(tree), std::hint::black_box(3)));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::delete_node);
    bench_solution("AI solution", ai_solution::Solution::delete_node);
}
