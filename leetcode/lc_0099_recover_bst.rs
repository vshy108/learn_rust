// LeetCode 99: Recover Binary Search Tree [Medium]
// https://leetcode.com/problems/recover-binary-search-tree/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 3.823us
//   AI solution avg: 3.258us
//   Ratio: AI 1.17x faster
//
// Two nodes of a BST are swapped by mistake. Recover the tree without
// changing its structure.
//
// Constraints: [2, 1000] nodes, -2^31 <= Node.val <= 2^31 - 1
//
// Run:  cargo run --example lc_0099_recover_bst
// Test: cargo test --example lc_0099_recover_bst
// Bench: cargo test --example lc_0099_recover_bst bench -- --nocapture

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

fn inorder_vals(root: &Node) -> Vec<i32> {
    let mut result = Vec::new();
    fn helper(node: &Node, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            helper(&n.left, result);
            result.push(n.val);
            helper(&n.right, result);
        }
    }
    helper(root, &mut result);
    result
}

/// My solution — collect inorder, find swapped pair, fix in tree.
///
/// Time: O(n).
/// Space: O(n).
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Recover BST by swapping back the two wrong nodes.
        ///
        /// # Arguments
        /// * `root` - BST with two swapped nodes
        pub fn recover_tree(root: &mut Node) {
            // Collect inorder values
            let vals = inorder_vals(root);
            let mut sorted = vals.clone();
            sorted.sort();

            // Find the two positions that differ
            let mut swaps = Vec::new();
            for i in 0..vals.len() {
                if vals[i] != sorted[i] {
                    swaps.push(sorted[i]);
                }
            }

            if swaps.len() == 2 {
                // Fix in tree: find nodes with swaps[0] and swaps[1] (actually the wrong vals)
                let wrong_a = swaps[1]; // larger value in wrong position
                let wrong_b = swaps[0]; // smaller value in wrong position
                let mut found = 0;
                Self::fix(root, wrong_a, wrong_b, &mut found);
            }
        }

        fn fix(node: &mut Node, a: i32, b: i32, found: &mut i32) {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                if n.val == a && *found & 1 == 0 {
                    n.val = b;
                    *found |= 1;
                } else if n.val == b && *found & 2 == 0 {
                    n.val = a;
                    *found |= 2;
                }
                if *found < 3 {
                    Self::fix(&mut n.left, a, b, found);
                }
                if *found < 3 {
                    Self::fix(&mut n.right, a, b, found);
                }
            }
        }
    }
}

/// AI solution — inorder traversal finding two violations.
///
/// Track prev node during inorder. First violation: prev > curr (first=prev).
/// Second violation: prev > curr again (second=curr). Swap their values.
///
/// Time: O(n).
/// Space: O(h) for recursion.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Recover BST by swapping back the two wrong nodes.
        ///
        /// # Arguments
        /// * `root` - BST with two swapped nodes
        pub fn recover_tree(root: &mut Node) {
            let mut first: Option<Rc<RefCell<TreeNode>>> = None;
            let mut second: Option<Rc<RefCell<TreeNode>>> = None;
            let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

            Self::inorder(root, &mut prev, &mut first, &mut second);

            // Swap values of first and second
            if let (Some(f), Some(s)) = (first, second) {
                let tmp = f.borrow().val;
                f.borrow_mut().val = s.borrow().val;
                s.borrow_mut().val = tmp;
            }
        }

        fn inorder(
            node: &Node,
            prev: &mut Option<Rc<RefCell<TreeNode>>>,
            first: &mut Option<Rc<RefCell<TreeNode>>>,
            second: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(n) = node {
                Self::inorder(&n.borrow().left.clone(), prev, first, second);

                if let Some(p) = prev {
                    if p.borrow().val > n.borrow().val {
                        if first.is_none() {
                            *first = Some(Rc::clone(p));
                        }
                        *second = Some(Rc::clone(n));
                    }
                }
                *prev = Some(Rc::clone(n));

                Self::inorder(&n.borrow().right.clone(), prev, first, second);
            }
        }
    }
}

fn main() {
    let mut root = build_tree(&[Some(1), Some(3), None, None, Some(2)]);
    println!("before: {:?}", inorder_vals(&root));
    my_solution::Solution::recover_tree(&mut root);
    println!("my after: {:?}", inorder_vals(&root));

    let mut root2 = build_tree(&[Some(3), Some(1), Some(4), None, None, Some(2)]);
    println!("before: {:?}", inorder_vals(&root2));
    ai_solution::Solution::recover_tree(&mut root2);
    println!("ai after: {:?}", inorder_vals(&root2));
}

#[cfg(test)]
mod tests {
    use super::{build_tree, inorder_vals};

    macro_rules! test_recover {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{build_tree, inorder_vals};

                #[test]
                fn test_example1() {
                    // Tree: [1, 3, null, null, 2] => 3 and 1 swapped
                    let mut root = build_tree(&[Some(1), Some(3), None, None, Some(2)]);
                    <$solution>::recover_tree(&mut root);
                    let vals = inorder_vals(&root);
                    assert_eq!(vals, vec![1, 2, 3]);
                }

                #[test]
                fn test_example2() {
                    // Tree: [3, 1, 4, null, null, 2] => 3 and 2 swapped
                    let mut root = build_tree(&[
                        Some(3), Some(1), Some(4), None, None, Some(2),
                    ]);
                    <$solution>::recover_tree(&mut root);
                    let vals = inorder_vals(&root);
                    assert_eq!(vals, vec![1, 2, 3, 4]);
                }
            }
        };
    }

    test_recover!(my, crate::my_solution::Solution);
    test_recover!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a valid BST [1..15] then swap two values
        let vals: Vec<Option<i32>> = vec![
            Some(8), Some(4), Some(12),
            Some(2), Some(6), Some(10), Some(14),
            Some(1), Some(3), Some(5), Some(7),
            Some(9), Some(11), Some(13), Some(15),
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut root = build_tree(&vals);
            // Swap nodes 3 and 13
            fn swap_vals(
                node: &super::Node,
                a: i32, b: i32,
            ) {
                if let Some(n) = node {
                    let mut n = n.borrow_mut();
                    if n.val == a { n.val = b; }
                    else if n.val == b { n.val = a; }
                    swap_vals(&n.left, a, b);
                    swap_vals(&n.right, a, b);
                }
            }
            swap_vals(&root, 3, 13);
            std::hint::black_box(
                crate::my_solution::Solution::recover_tree(&mut root),
            );
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut root = build_tree(&vals);
            fn swap_vals2(
                node: &super::Node,
                a: i32, b: i32,
            ) {
                if let Some(n) = node {
                    let mut n = n.borrow_mut();
                    if n.val == a { n.val = b; }
                    else if n.val == b { n.val = a; }
                    swap_vals2(&n.left, a, b);
                    swap_vals2(&n.right, a, b);
                }
            }
            swap_vals2(&root, 3, 13);
            std::hint::black_box(
                crate::ai_solution::Solution::recover_tree(&mut root),
            );
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
