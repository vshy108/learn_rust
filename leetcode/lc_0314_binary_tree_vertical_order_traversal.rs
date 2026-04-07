// LeetCode 314. Binary Tree Vertical Order Traversal (Premium)
// https://leetcode.com/problems/binary-tree-vertical-order-traversal/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.424us
//   ai_solution  avg: 0.378us
//   Ratio: ai_solution 1.12x faster

/// Given root of a binary tree, return vertical order traversal of node values
/// (top to bottom, column by column). Same row+column => left to right.
///
/// Example 1:
///   Input: root = [3,9,20,null,null,15,7]
///   Output: [[9],[3,15],[20],[7]]
///
/// Example 2:
///   Input: root = [3,9,8,4,0,1,7]
///   Output: [[4],[9],[3,0,1],[8],[7]]
///
/// Constraints:
///   - Number of nodes in [0, 100]
///   - -100 <= Node.val <= 100

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
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Build a binary tree from a level-order array (None = null node).
fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;
    while i < vals.len() && !queue.is_empty() {
        if let Some(node) = queue.pop_front() {
            // Left child
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
            }
            i += 1;
            // Right child
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
            }
            i += 1;
        }
    }
    Some(root)
}

/// # my_solution: BFS with offset tracking
/// Time: O(n log n) — BFS is O(n), sorting columns by offset
/// Space: O(n) for the queue and hash map
mod my_solution {
    use super::*;
    use std::collections::{BTreeMap, VecDeque};

    pub struct Solution;

    impl Solution {
        /// Vertical order traversal using BFS (level-order).
        ///
        /// BFS naturally processes top-to-bottom, left-to-right within a level.
        /// We track each node's column offset. Using BTreeMap keeps columns sorted.
        ///
        /// # Arguments
        /// * `root` - Root of binary tree
        ///
        /// # Returns
        /// Vertical order traversal as vec of vecs
        pub fn vertical_order(
            root: Option<Rc<RefCell<TreeNode>>>,
        ) -> Vec<Vec<i32>> {
            if root.is_none() {
                return vec![];
            }

            // BTreeMap: column offset -> list of values (BFS order)
            let mut d: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
            let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
            queue.push_back((root.unwrap(), 0));

            while let Some((node, offset)) = queue.pop_front() {
                let node_ref = node.borrow();
                d.entry(offset).or_default().push(node_ref.val);

                if let Some(ref left) = node_ref.left {
                    queue.push_back((Rc::clone(left), offset - 1));
                }
                if let Some(ref right) = node_ref.right {
                    queue.push_back((Rc::clone(right), offset + 1));
                }
            }

            d.into_values().collect()
        }
    }
}

/// # ai_solution: DFS with depth+offset, then sort
/// Time: O(n log n) — DFS O(n), sorting by (offset, depth)
/// Space: O(n) for the map and recursion stack
mod ai_solution {
    use super::*;
    use std::collections::BTreeMap;

    pub struct Solution;

    impl Solution {
        /// Vertical order traversal using DFS.
        ///
        /// DFS collects (depth, value) per column offset. After collecting,
        /// sort each column by depth to maintain top-to-bottom order.
        /// Using stable sort preserves left-to-right order for same depth.
        ///
        /// # Arguments
        /// * `root` - Root of binary tree
        ///
        /// # Returns
        /// Vertical order traversal as vec of vecs
        pub fn vertical_order(
            root: Option<Rc<RefCell<TreeNode>>>,
        ) -> Vec<Vec<i32>> {
            let mut d: BTreeMap<i32, Vec<(i32, i32)>> = BTreeMap::new();
            dfs(&root, 0, 0, &mut d);

            d.into_values()
                .map(|mut entries| {
                    // Sort by depth to ensure top-to-bottom ordering
                    entries.sort_by_key(|&(depth, _)| depth);
                    entries.into_iter().map(|(_, val)| val).collect()
                })
                .collect()
        }
    }

    /// Recursively traverse the tree, recording (depth, val) per column offset.
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
        offset: i32,
        d: &mut BTreeMap<i32, Vec<(i32, i32)>>,
    ) {
        if let Some(n) = node {
            let n_ref = n.borrow();
            d.entry(offset).or_default().push((depth, n_ref.val));
            dfs(&n_ref.left, depth + 1, offset - 1, d);
            dfs(&n_ref.right, depth + 1, offset + 1, d);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    type SolveFn = fn(Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>;

    fn run_tests(solve: SolveFn) {
        // Example 1: [3,9,20,null,null,15,7]
        let root = build_tree(&[
            Some(3), Some(9), Some(20), None, None, Some(15), Some(7),
        ]);
        assert_eq!(
            solve(root),
            vec![vec![9], vec![3, 15], vec![20], vec![7]]
        );

        // Example 2: [3,9,8,4,0,1,7]
        let root = build_tree(&[
            Some(3), Some(9), Some(8), Some(4), Some(0), Some(1), Some(7),
        ]);
        assert_eq!(
            solve(root),
            vec![vec![4], vec![9], vec![3, 0, 1], vec![8], vec![7]]
        );

        // Empty tree
        assert_eq!(solve(None), Vec::<Vec<i32>>::new());

        // Single node
        let root = build_tree(&[Some(1)]);
        assert_eq!(solve(root), vec![vec![1]]);

        // Left-skewed
        let root = build_tree(&[Some(1), Some(2), None, Some(3)]);
        assert_eq!(solve(root), vec![vec![3], vec![2], vec![1]]);

        // Right-skewed
        let root = build_tree(&[Some(1), None, Some(2), None, Some(3)]);
        assert_eq!(solve(root), vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::vertical_order);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::vertical_order);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let root = build_tree(&[
        Some(3), Some(9), Some(20), None, None, Some(15), Some(7),
    ]);
    println!("Input: root = [3,9,20,null,null,15,7]");
    println!(
        "Output (my): {:?}",
        my_solution::Solution::vertical_order(root.clone())
    );
    println!(
        "Output (ai): {:?}",
        ai_solution::Solution::vertical_order(root.clone())
    );

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::vertical_order(
            black_box(root.clone()),
        ));
    }
    let my_dur = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::vertical_order(
            black_box(root.clone()),
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    println!("\nBenchmark ({iterations} iterations, release mode):");
    println!("  my_solution  avg: {my_avg:.3}us");
    println!("  ai_solution  avg: {ai_avg:.3}us");
    if my_avg < ai_avg {
        println!("  my_solution is {:.2}x faster", ai_avg / my_avg);
    } else {
        println!("  ai_solution is {:.2}x faster", my_avg / ai_avg);
    }
}
