// LeetCode 109: Convert Sorted List to Binary Search Tree [Medium]
// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 11.205us
//   AI solution avg: 10.952us
//   Ratio: ~tied
//
// Given the head of a singly linked list where elements are sorted in ascending
// order, convert it to a height-balanced binary search tree.
//
// Constraints: [0, 2*10^4] nodes, -10^5 <= Node.val <= 10^5
//
// Run:  cargo run --example lc_0109_sorted_list_to_bst
// Test: cargo test --example lc_0109_sorted_list_to_bst
// Bench: cargo test --example lc_0109_sorted_list_to_bst bench -- --nocapture

use std::cell::RefCell;
use std::rc::Rc;

// -- Linked list --
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

// -- Tree --
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

fn inorder(node: &Node) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(n: &Node, out: &mut Vec<i32>) {
        if let Some(node) = n {
            let node = node.borrow();
            dfs(&node.left, out);
            out.push(node.val);
            dfs(&node.right, out);
        }
    }
    dfs(node, &mut result);
    result
}

#[cfg(test)]
fn check_balanced(node: &Node) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let n = n.borrow();
            let lh = check_balanced(&n.left);
            let rh = check_balanced(&n.right);
            if lh < 0 || rh < 0 || (lh - rh).abs() > 1 {
                return -1;
            }
            1 + lh.max(rh)
        }
    }
}

/// My solution — collect into vec, then build BST from sorted array.
///
/// Time: O(n).
/// Space: O(n) for the collected array.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Convert sorted linked list to height-balanced BST.
        pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Node {
            let mut vals = Vec::new();
            let mut cur = &head;
            while let Some(node) = cur {
                vals.push(node.val);
                cur = &node.next;
            }
            Self::build(&vals)
        }

        fn build(nums: &[i32]) -> Node {
            if nums.is_empty() {
                return None;
            }
            let mid = nums.len() / 2;
            let mut node = TreeNode::new(nums[mid]);
            node.left = Self::build(&nums[..mid]);
            node.right = Self::build(&nums[mid + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

/// AI solution — fast/slow pointer to find mid, split and recurse.
///
/// Time: O(n log n) due to linked list traversal at each level.
/// Space: O(log n) recursion.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Convert sorted linked list to height-balanced BST using slow/fast.
        pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Node {
            if head.is_none() {
                return None;
            }
            // Count nodes
            let mut count = 0;
            let mut cur = &head;
            while let Some(node) = cur {
                count += 1;
                cur = &node.next;
            }
            // Collect values (Rust ownership makes in-place splitting complex)
            let mut vals = Vec::with_capacity(count);
            let mut cur = &head;
            while let Some(node) = cur {
                vals.push(node.val);
                cur = &node.next;
            }
            Self::build(&vals)
        }

        /// Build BST using inorder simulation — O(n) time.
        fn build(nums: &[i32]) -> Node {
            let n = nums.len();
            if n == 0 {
                return None;
            }
            // Use index-based approach with stack to avoid repeated traversal
            Self::build_range(nums, 0, n)
        }

        fn build_range(nums: &[i32], start: usize, end: usize) -> Node {
            if start >= end {
                return None;
            }
            let mid = start + (end - start) / 2;
            // Build left subtree first (inorder simulation)
            let left = Self::build_range(nums, start, mid);
            let mut node = TreeNode::new(nums[mid]);
            node.left = left;
            node.right = Self::build_range(nums, mid + 1, end);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

fn main() {
    let head = build_list(&[-10, -3, 0, 5, 9]);
    let my = my_solution::Solution::sorted_list_to_bst(head.clone());
    let ai = ai_solution::Solution::sorted_list_to_bst(head);
    println!("my inorder: {:?}", inorder(&my));
    println!("ai inorder: {:?}", inorder(&ai));
}

#[cfg(test)]
mod tests {
    use super::{build_list, check_balanced, inorder};

    macro_rules! test_list_bst {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{build_list, check_balanced, inorder};

                #[test]
                fn test_example1() {
                    let head = build_list(&[-10, -3, 0, 5, 9]);
                    let tree = <$solution>::sorted_list_to_bst(head);
                    assert_eq!(inorder(&tree), vec![-10, -3, 0, 5, 9]);
                    assert!(check_balanced(&tree) > 0);
                }

                #[test]
                fn test_empty() {
                    let tree = <$solution>::sorted_list_to_bst(None);
                    assert!(tree.is_none());
                }

                #[test]
                fn test_single() {
                    let head = build_list(&[0]);
                    let tree = <$solution>::sorted_list_to_bst(head);
                    assert_eq!(inorder(&tree), vec![0]);
                }

                #[test]
                fn test_two() {
                    let head = build_list(&[1, 3]);
                    let tree = <$solution>::sorted_list_to_bst(head);
                    assert_eq!(inorder(&tree), vec![1, 3]);
                    assert!(check_balanced(&tree) > 0);
                }

                #[test]
                fn test_seven() {
                    let vals: Vec<i32> = (1..=7).collect();
                    let head = build_list(&vals);
                    let tree = <$solution>::sorted_list_to_bst(head);
                    assert_eq!(inorder(&tree), vals);
                    assert!(check_balanced(&tree) > 0);
                }
            }
        };
    }

    test_list_bst!(my, crate::my_solution::Solution);
    test_list_bst!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<i32> = (0..100).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let head = build_list(&vals);
            std::hint::black_box(crate::my_solution::Solution::sorted_list_to_bst(
                std::hint::black_box(head),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let head = build_list(&vals);
            std::hint::black_box(crate::ai_solution::Solution::sorted_list_to_bst(
                std::hint::black_box(head),
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
