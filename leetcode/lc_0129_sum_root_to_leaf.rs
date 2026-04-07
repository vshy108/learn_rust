// LeetCode 129: Sum Root to Leaf Numbers [Medium]
// https://leetcode.com/problems/sum-root-to-leaf-numbers/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.034us
//   AI solution avg: 0.153us
//   Ratio: My 4.50x faster
//
// You are given the root of a binary tree containing digits from 0 to 9 only.
// Each root-to-leaf path could represent a number.
// Return the total sum of all root-to-leaf numbers.
//
// Constraints: [1, 1000] nodes, 0 <= Node.val <= 9, depth <= 10
//
// Run:  cargo run --example lc_0129_sum_root_to_leaf
// Test: cargo test --example lc_0129_sum_root_to_leaf
// Bench: cargo test --example lc_0129_sum_root_to_leaf bench -- --nocapture

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

/// My solution — DFS accumulating the current number, add to sum at leaves.
///
/// Time: O(n).
/// Space: O(h) recursion stack where h = tree height.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return sum of all root-to-leaf numbers.
        pub fn sum_numbers(root: Node) -> i32 {
            fn dfs(node: &Node, current: i32) -> i32 {
                match node {
                    None => 0,
                    Some(n) => {
                        let n = n.borrow();
                        let num = current * 10 + n.val;
                        // Leaf node — return the accumulated number
                        if n.left.is_none() && n.right.is_none() {
                            return num;
                        }
                        dfs(&n.left, num) + dfs(&n.right, num)
                    }
                }
            }
            dfs(&root, 0)
        }
    }
}

/// AI solution — iterative DFS using a stack of (node, current_number).
///
/// Time: O(n).
/// Space: O(h) for the stack.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return sum of all root-to-leaf numbers (iterative).
        pub fn sum_numbers(root: Node) -> i32 {
            let Some(r) = root else { return 0 };
            let mut stack = vec![(r, 0i32)];
            let mut total = 0;

            while let Some((node, parent_num)) = stack.pop() {
                let node = node.borrow();
                let num = parent_num * 10 + node.val;
                if node.left.is_none() && node.right.is_none() {
                    total += num;
                }
                if let Some(ref l) = node.left {
                    stack.push((Rc::clone(l), num));
                }
                if let Some(ref r) = node.right {
                    stack.push((Rc::clone(r), num));
                }
            }
            total
        }
    }
}

fn main() {
    // [1, 2, 3] -> 12 + 13 = 25
    let root = build_tree(&[Some(1), Some(2), Some(3)]);
    let my = my_solution::Solution::sum_numbers(root.clone());
    let ai = ai_solution::Solution::sum_numbers(root);
    println!("sum root-to-leaf: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_sum_numbers {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    let root = build_tree(&[Some(1), Some(2), Some(3)]);
                    assert_eq!(<$solution>::sum_numbers(root), 25);
                }

                #[test]
                fn test_example2() {
                    let root = build_tree(&[Some(4), Some(9), Some(0), Some(5), Some(1)]);
                    assert_eq!(<$solution>::sum_numbers(root), 1026);
                }

                #[test]
                fn test_single_node() {
                    let root = build_tree(&[Some(5)]);
                    assert_eq!(<$solution>::sum_numbers(root), 5);
                }

                #[test]
                fn test_left_only() {
                    let root = build_tree(&[Some(1), Some(2), None]);
                    assert_eq!(<$solution>::sum_numbers(root), 12);
                }
            }
        };
    }

    test_sum_numbers!(my, crate::my_solution::Solution);
    test_sum_numbers!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let root = build_tree(&[
            Some(4), Some(9), Some(0), Some(5), Some(1),
            Some(3), Some(7), Some(8), Some(2), Some(6),
        ]);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::sum_numbers(
                std::hint::black_box(root.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::sum_numbers(
                std::hint::black_box(root.clone()),
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
            println!("Both solutions are roughly equal");
        }
    }
}
