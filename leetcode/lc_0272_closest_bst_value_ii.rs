// LeetCode 272: Closest Binary Search Tree Value II [Hard] (Premium)
// https://leetcode.com/problems/closest-binary-search-tree-value-ii/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.092us
//   AI solution avg: 0.112us
//   Ratio: My 1.21x faster
//
// Given the root of a BST, a target value, and an integer k, return the k
// values in the BST that are closest to the target. You may return the answer
// in any order. There is a unique set of k closest values.
//
// Example 1: root = [4,2,5,1,3], target = 3.714286, k = 2 -> [4,3]
// Example 2: root = [1], target = 0.000000, k = 1 -> [1]
//
// Constraints:
//   - Number of nodes = n, 1 <= k <= n <= 10^4
//   - 0 <= Node.val <= 10^9
//   - -10^9 <= target <= 10^9
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0272.Closest%20Binary%20Search%20Tree%20Value%20II/README_EN.md
//
// Run:  cargo run --example lc_0272_closest_bst_value_ii
// Test: cargo test --example lc_0272_closest_bst_value_ii
// Bench: cargo test --example lc_0272_closest_bst_value_ii bench -- --nocapture

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Definition for a binary tree node.
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

/// Build a binary tree from a level-order Vec<Option<i32>>.
fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    let mut i = 1;
    while i < vals.len() {
        if let Some(node) = queue.pop_front() {
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }
    }
    Some(root)
}

/// My solution — Inorder traversal with sliding window (from doocs).
///
/// Inorder traversal yields sorted values. Maintain a VecDeque of size k.
/// While queue has fewer than k elements, push directly. Otherwise, if the
/// current value is closer than the front (the farthest in the window),
/// pop front and push back. If not closer, early-return since all
/// subsequent inorder values are even farther from target.
///
/// Time:  O(n) — visits each node at most once
/// Space: O(n) for recursion stack + O(k) for the deque
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        pub fn closest_k_values(
            root: Option<Rc<RefCell<TreeNode>>>,
            target: f64,
            k: i32,
        ) -> Vec<i32> {
            let k = k as usize;
            let mut q = VecDeque::with_capacity(k + 1);
            // `found` flag signals early termination from inorder DFS
            Self::dfs(&root, target, k, &mut q);
            q.into_iter().collect()
        }

        /// Inorder DFS: returns false to signal early termination.
        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            target: f64,
            k: usize,
            q: &mut VecDeque<i32>,
        ) -> bool {
            let Some(n) = node else { return true };
            let n_ref = n.borrow();

            // Traverse left subtree
            if !Self::dfs(&n_ref.left, target, k, q) {
                return false;
            }

            let val = n_ref.val;
            if q.len() < k {
                q.push_back(val);
            } else {
                // Since inorder is sorted, once current is farther than
                // the front of the window, all remaining will be too
                let cur_diff = (val as f64 - target).abs();
                let front_diff = (q[0] as f64 - target).abs();
                if cur_diff >= front_diff {
                    return false; // early exit
                }
                q.pop_front();
                q.push_back(val);
            }

            // Traverse right subtree
            Self::dfs(&n_ref.right, target, k, q)
        }
    }
}

/// AI solution — Max-heap of size k (BinaryHeap).
///
/// Traverse the entire tree. For each node, push (distance, value) into
/// a max-heap keyed by distance (and by value for tie-breaking). If the
/// heap exceeds k elements, pop the farthest. After traversal, collect
/// the heap contents.
///
/// Time:  O(n log k)
/// Space: O(n) for recursion + O(k) for the heap
mod ai_solution {
    use super::*;
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    /// Wrapper for heap ordering: max by distance, then max by value (so
    /// we evict the farthest / largest-valued node first).
    #[derive(PartialEq)]
    struct HeapEntry {
        dist: f64,
        val: i32,
    }

    impl Eq for HeapEntry {}

    impl PartialOrd for HeapEntry {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for HeapEntry {
        fn cmp(&self, other: &Self) -> Ordering {
            // Primary: max distance. Secondary: max val (evict larger on tie).
            self.dist
                .partial_cmp(&other.dist)
                .unwrap_or(Ordering::Equal)
                .then_with(|| self.val.cmp(&other.val))
        }
    }

    pub struct Solution;

    impl Solution {
        pub fn closest_k_values(
            root: Option<Rc<RefCell<TreeNode>>>,
            target: f64,
            k: i32,
        ) -> Vec<i32> {
            let k = k as usize;
            let mut heap = BinaryHeap::with_capacity(k + 1);
            Self::dfs(&root, target, k, &mut heap);
            heap.into_iter().map(|e| e.val).collect()
        }

        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            target: f64,
            k: usize,
            heap: &mut BinaryHeap<HeapEntry>,
        ) {
            let Some(n) = node else { return };
            let n_ref = n.borrow();
            let val = n_ref.val;
            let dist = (val as f64 - target).abs();

            heap.push(HeapEntry { dist, val });
            if heap.len() > k {
                heap.pop(); // remove farthest
            }

            Self::dfs(&n_ref.left, target, k, heap);
            Self::dfs(&n_ref.right, target, k, heap);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Helper: run both solutions and compare (order-independent).
    macro_rules! test_closest_k {
        ($name:ident, $vals:expr, $target:expr, $k:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let root = build_tree(&$vals);
                let expected: HashSet<i32> =
                    $expected.iter().copied().collect();

                let my_res: HashSet<i32> =
                    my_solution::Solution::closest_k_values(
                        root.clone(),
                        $target,
                        $k,
                    )
                    .into_iter()
                    .collect();
                assert_eq!(my_res, expected, "my_solution failed");

                let ai_res: HashSet<i32> =
                    ai_solution::Solution::closest_k_values(root, $target, $k)
                        .into_iter()
                        .collect();
                assert_eq!(ai_res, expected, "ai_solution failed");
            }
        };
    }

    // Example 1
    test_closest_k!(
        test_example_1,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        3.714286,
        2,
        [4, 3]
    );

    // Example 2
    test_closest_k!(test_example_2, [Some(1)], 0.0, 1, [1]);

    // k equals tree size — return all nodes
    test_closest_k!(
        test_k_equals_n,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        3.0,
        5,
        [1, 2, 3, 4, 5]
    );

    // k = 1, single closest
    test_closest_k!(
        test_k_one,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        2.9,
        1,
        [3]
    );

    // Target exactly on a node
    test_closest_k!(
        test_exact_match,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        3.0,
        2,
        [3, 2]
    );

    // Target below all nodes
    test_closest_k!(
        test_target_below_all,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        -10.0,
        2,
        [1, 2]
    );

    // Target above all nodes
    test_closest_k!(
        test_target_above_all,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        100.0,
        2,
        [4, 5]
    );

    // Balanced tree, k=3
    test_closest_k!(
        test_balanced,
        [Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14)],
        7.0,
        3,
        [6, 8, 4]
    );

    // Left-skewed tree
    test_closest_k!(
        test_left_skewed,
        [Some(5), Some(3), None, Some(1)],
        2.0,
        2,
        [1, 3]
    );

    // Right-skewed tree
    test_closest_k!(
        test_right_skewed,
        [Some(1), None, Some(3), None, None, None, Some(5)],
        4.0,
        2,
        [3, 5]
    );

    // Two-node tree, k=1
    test_closest_k!(
        test_two_nodes_k1,
        [Some(1), None, Some(2)],
        1.6,
        1,
        [2]
    );

    // Large k with balanced tree
    test_closest_k!(
        test_large_k,
        [Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14)],
        9.0,
        5,
        [6, 8, 10, 12, 4]
    );
}

fn main() {
    println!("LeetCode 272: Closest Binary Search Tree Value II");
    let root = build_tree(&[Some(4), Some(2), Some(5), Some(1), Some(3)]);
    let target = 3.714286;
    let k = 2;
    println!(
        "my_solution:  closest_k_values([4,2,5,1,3], {}, {}) = {:?}",
        target,
        k,
        my_solution::Solution::closest_k_values(root.clone(), target, k)
    );
    println!(
        "ai_solution:  closest_k_values([4,2,5,1,3], {}, {}) = {:?}",
        target,
        k,
        ai_solution::Solution::closest_k_values(root, target, k)
    );
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let root = build_tree(&[
            Some(8),
            Some(4),
            Some(12),
            Some(2),
            Some(6),
            Some(10),
            Some(14),
        ]);
        let target = 7.0;
        let k = 3;
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(my_solution::Solution::closest_k_values(
                root.clone(),
                target,
                k,
            ));
            std::hint::black_box(ai_solution::Solution::closest_k_values(
                root.clone(),
                target,
                k,
            ));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::closest_k_values(
                root.clone(),
                target,
                k,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::closest_k_values(
                root.clone(),
                target,
                k,
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
