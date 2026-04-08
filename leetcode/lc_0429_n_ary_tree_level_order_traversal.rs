// LeetCode 429: N-ary Tree Level Order Traversal [Medium]
// https://leetcode.com/problems/n-ary-tree-level-order-traversal/
//
// Given an n-ary tree, return the level order traversal of its nodes' values.
//
// Examples:
//   root = [1,null,3,2,4,null,5,6] -> [[1],[3,2,4],[5,6]]
//
// Constraints:
//   Height of tree <= 1000
//   Total nodes in [0, 10^4]
//
// Run:  cargo run --example lc_0429_n_ary_tree_level_order_traversal
// Test: cargo test --example lc_0429_n_ary_tree_level_order_traversal
// Bench: cargo test --example lc_0429_n_ary_tree_level_order_traversal bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 225ns
//   AI solution: avg 194ns
//   Ratio: 1.16x (DFS slightly faster)

use std::collections::VecDeque;

/// N-ary tree node: each node has a value and a list of children.
#[derive(Debug, Clone)]
pub struct NaryNode {
    pub val: i32,
    pub children: Vec<NaryNode>,
}

impl NaryNode {
    pub fn new(val: i32) -> Self {
        NaryNode {
            val,
            children: Vec::new(),
        }
    }
}

/// My solution — BFS with queue (translated from doocs Java).
///
/// Process nodes level by level using a queue. For each level,
/// drain the current queue contents and enqueue their children.
///
/// Time:  O(n)
/// Space: O(n)
#[allow(dead_code)]
mod my_solution {
    use super::*;

    pub struct Solution;
    impl Solution {
        /// Return level order traversal of an N-ary tree.
        ///
        /// # Arguments
        /// * `root` - optional reference to the root node
        ///
        /// # Returns
        /// Vec of Vecs, each inner Vec is one level's values.
        pub fn level_order(root: Option<&NaryNode>) -> Vec<Vec<i32>> {
            let mut ans = Vec::new();
            let root = match root {
                Some(r) => r,
                None => return ans,
            };
            let mut queue = VecDeque::new();
            queue.push_back(root);
            while !queue.is_empty() {
                let mut level = Vec::new();
                for _ in 0..queue.len() {
                    let node = queue.pop_front().unwrap();
                    level.push(node.val);
                    for child in &node.children {
                        queue.push_back(child);
                    }
                }
                ans.push(level);
            }
            ans
        }
    }
}

/// AI solution — DFS with depth tracking (translated from doocs DFS).
///
/// Recursively traverse the tree, tracking the current depth.
/// Append values to the corresponding level in the result vector.
///
/// Time:  O(n)
/// Space: O(n)
mod ai_solution {
    use super::*;

    pub struct Solution;
    impl Solution {
        /// Return level order traversal using DFS.
        pub fn level_order(root: Option<&NaryNode>) -> Vec<Vec<i32>> {
            let mut ans: Vec<Vec<i32>> = Vec::new();
            if let Some(r) = root {
                Self::dfs(r, 0, &mut ans);
            }
            ans
        }

        fn dfs(node: &NaryNode, depth: usize, ans: &mut Vec<Vec<i32>>) {
            // Extend the result if we've reached a new depth
            if ans.len() <= depth {
                ans.push(Vec::new());
            }
            ans[depth].push(node.val);
            for child in &node.children {
                Self::dfs(child, depth + 1, ans);
            }
        }
    }
}

fn main() {
    // Build tree: 1 -> [3, 2, 4], 3 -> [5, 6]
    let mut root = NaryNode::new(1);
    let mut child3 = NaryNode::new(3);
    child3.children.push(NaryNode::new(5));
    child3.children.push(NaryNode::new(6));
    root.children.push(child3);
    root.children.push(NaryNode::new(2));
    root.children.push(NaryNode::new(4));

    println!("my:  {:?}", my_solution::Solution::level_order(Some(&root)));
    println!("ai:  {:?}", ai_solution::Solution::level_order(Some(&root)));
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::*;
        use $module::Solution;

        #[test]
        fn test_example_1() {
            let mut root = NaryNode::new(1);
            let mut child3 = NaryNode::new(3);
            child3.children.push(NaryNode::new(5));
            child3.children.push(NaryNode::new(6));
            root.children.push(child3);
            root.children.push(NaryNode::new(2));
            root.children.push(NaryNode::new(4));
            assert_eq!(
                Solution::level_order(Some(&root)),
                vec![vec![1], vec![3, 2, 4], vec![5, 6]]
            );
        }

        #[test]
        fn test_empty_tree() {
            assert_eq!(Solution::level_order(None), Vec::<Vec<i32>>::new());
        }

        #[test]
        fn test_single_node() {
            let root = NaryNode::new(42);
            assert_eq!(Solution::level_order(Some(&root)), vec![vec![42]]);
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
    F: Fn(Option<&NaryNode>) -> Vec<Vec<i32>>,
{
    let iterations = 500_000;
    let mut root = NaryNode::new(1);
    let mut child3 = NaryNode::new(3);
    child3.children.push(NaryNode::new(5));
    child3.children.push(NaryNode::new(6));
    root.children.push(child3);
    root.children.push(NaryNode::new(2));
    root.children.push(NaryNode::new(4));
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(Some(&root))));
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
    bench_solution("My solution", my_solution::Solution::level_order);
    bench_solution("AI solution", ai_solution::Solution::level_order);
}
