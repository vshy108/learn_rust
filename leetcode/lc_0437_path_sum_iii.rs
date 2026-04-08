// LeetCode 437: Path Sum III [Medium]
// https://leetcode.com/problems/path-sum-iii/
//
// Given the root of a binary tree and an integer targetSum, return the number
// of paths where the sum of the values along the path equals targetSum.
// The path does not need to start or end at the root or a leaf, but it must
// go downwards (only from parent nodes to child nodes).
//
// Examples:
//   root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8 -> 3
//   root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22 -> 3
//
// Constraints:
//   0 <= number of nodes <= 1000
//   -10^9 <= Node.val <= 10^9
//   -1000 <= targetSum <= 1000
//
// Run:  cargo run --example lc_0437_path_sum_iii
// Test: cargo test --example lc_0437_path_sum_iii
// Bench: cargo test --example lc_0437_path_sum_iii bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 471ns
//   AI solution: avg 109ns
//   Ratio: 4.32x (AI brute-force wins on small trees)

use std::cell::RefCell;
use std::rc::Rc;

/// Shared TreeNode definition used by both solutions.
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
#[cfg(test)]
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

/// My solution — prefix sum with HashMap (from doocs/leetcode Rust).
///
/// Use a running prefix sum from root to current node. At each node, check
/// how many times (prefix_sum - target) appeared previously — that count
/// equals the number of valid paths ending at this node.
///
/// Time:  O(n)
/// Space: O(n)
#[allow(dead_code)]
mod my_solution {
    use super::*;
    use std::collections::HashMap;

    pub struct Solution;
    impl Solution {
        /// Count paths in a binary tree that sum to `target_sum`.
        ///
        /// # Arguments
        /// * `root` - root of the binary tree
        /// * `target_sum` - the target path sum
        ///
        /// # Returns
        /// Number of downward paths whose values sum to `target_sum`.
        pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
            let mut cnt: HashMap<i64, i32> = HashMap::new();
            // Base case: empty prefix sums to 0
            cnt.insert(0, 1);
            Self::dfs(root, 0, target_sum as i64, &mut cnt)
        }

        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            s: i64,
            target: i64,
            cnt: &mut HashMap<i64, i32>,
        ) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let s = s + n.val as i64;
                // Paths ending here with sum = target
                let ans = cnt.get(&(s - target)).copied().unwrap_or(0);
                *cnt.entry(s).or_insert(0) += 1;
                let ans = ans
                    + Self::dfs(n.left.clone(), s, target, cnt)
                    + Self::dfs(n.right.clone(), s, target, cnt);
                // Backtrack: remove current prefix from map
                *cnt.get_mut(&s).unwrap() -= 1;
                ans
            } else {
                0
            }
        }
    }
}

/// AI solution — brute force DFS from every node.
///
/// For each node, start a DFS counting paths beginning at that node
/// which sum to targetSum. Then recurse on left and right children.
///
/// Time:  O(n^2)
/// Space: O(n) — recursion depth
mod ai_solution {
    use super::*;

    pub struct Solution;
    impl Solution {
        /// Count paths in a binary tree that sum to `target_sum`.
        pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
            Self::count_from_all(&root, target_sum as i64)
        }

        /// Visit every node and count paths starting from each.
        fn count_from_all(node: &Option<Rc<RefCell<TreeNode>>>, target: i64) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                Self::count_from_here(node, target, 0)
                    + Self::count_from_all(&n.left, target)
                    + Self::count_from_all(&n.right, target)
            } else {
                0
            }
        }

        /// Count paths starting at `node` going downward with running `sum`.
        fn count_from_here(node: &Option<Rc<RefCell<TreeNode>>>, target: i64, sum: i64) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let new_sum = sum + n.val as i64;
                let mut count = if new_sum == target { 1 } else { 0 };
                count += Self::count_from_here(&n.left, target, new_sum);
                count += Self::count_from_here(&n.right, target, new_sum);
                count
            } else {
                0
            }
        }
    }
}

fn main() {
    // Simple tree: 10 -> left 5, right -3
    let root = Rc::new(RefCell::new(TreeNode::new(10)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(-3))));
    let tree = Some(root);

    println!("my:  {}", my_solution::Solution::path_sum(tree.clone(), 15));
    println!("ai:  {}", ai_solution::Solution::path_sum(tree, 15));
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::*;
        use $module::Solution;

        #[test]
        fn test_example_1() {
            // [10,5,-3,3,2,null,11,3,-2,null,1], target=8, expect 3
            let root = build_tree(&[
                Some(10),
                Some(5),
                Some(-3),
                Some(3),
                Some(2),
                None,
                Some(11),
                Some(3),
                Some(-2),
                None,
                Some(1),
            ]);
            assert_eq!(Solution::path_sum(root, 8), 3);
        }

        #[test]
        fn test_example_2() {
            // [5,4,8,11,null,13,4,7,2,null,null,5,1], target=22, expect 3
            let root = build_tree(&[
                Some(5),
                Some(4),
                Some(8),
                Some(11),
                None,
                Some(13),
                Some(4),
                Some(7),
                Some(2),
                None,
                None,
                Some(5),
                Some(1),
            ]);
            assert_eq!(Solution::path_sum(root, 22), 3);
        }

        #[test]
        fn test_empty_tree() {
            assert_eq!(Solution::path_sum(None, 0), 0);
        }

        #[test]
        fn test_single_node_match() {
            let root = build_tree(&[Some(1)]);
            assert_eq!(Solution::path_sum(root, 1), 1);
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
    F: Fn(Option<Rc<RefCell<TreeNode>>>, i32) -> i32,
{
    let iterations = 500_000;
    let tree = build_tree(&[
        Some(10),
        Some(5),
        Some(-3),
        Some(3),
        Some(2),
        None,
        Some(11),
        Some(3),
        Some(-2),
        None,
        Some(1),
    ]);
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(tree.clone()),
            std::hint::black_box(8),
        ));
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
    bench_solution("My solution", my_solution::Solution::path_sum);
    bench_solution("AI solution", ai_solution::Solution::path_sum);
}
