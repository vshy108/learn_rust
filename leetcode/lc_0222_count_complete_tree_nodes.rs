// LeetCode 222: Count Complete Tree Nodes [Easy]
// https://leetcode.com/problems/count-complete-tree-nodes/
//
// Given the root of a complete binary tree, return the number of the
// nodes in the tree.
//
// In a complete binary tree, every level except possibly the last is
// completely filled, and all nodes in the last level are as far left
// as possible. Design an algorithm that runs in less than O(n) time.
//
// Examples:
//   root = [1,2,3,4,5,6] -> 6
//   root = [1]            -> 1
//   root = []             -> 0
//
// Constraints:
//   - The number of nodes in the tree is in [0, 5 * 10^4]
//   - 0 <= Node.val <= 5 * 10^4
//   - The tree is guaranteed to be complete
//
// --- Benchmark (100_000 iterations, 2026-04-05) ---
//   My solution: 841ms (avg 8.416us)
//   AI solution: 867ms (avg 8.677us)
//   -> My solution is 1.03x faster
//   Reason: Both are O(log^2 n). My solution's recursive spine comparison
//     has simpler per-step work (just follow left/right) vs AI's binary
//     search + bit masking. Nearly identical on small trees.
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
    // #[inline] hints the compiler to copy the function body directly into
    // each call site instead of generating a function call. Useful for tiny
    // functions like constructors to avoid call overhead. The compiler often
    // inlines small functions automatically within the same crate, but
    // #[inline] is needed for cross-crate inlining. Ruby has no equivalent —
    // all method calls go through the method lookup table.
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
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    current.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
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
/// Compare left-spine and right-spine heights. If equal, it's a perfect
/// tree — return 2^h - 1. Otherwise recurse on both subtrees. Each
/// recursive call halves the tree and computes height in O(log n).
///
/// Time:  O(log^2 n) — O(log n) recursion depth, each computing O(log n) height
/// Space: O(log n) — recursion stack depth
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Count the number of nodes in a complete binary tree.
        ///
        /// # Arguments
        ///
        /// * `root` - Root of the complete binary tree
        ///
        /// # Returns
        ///
        /// The total number of nodes
        pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if root.is_none() {
                return 0;
            };

            // Ruby: left_height = 0; node = root
            // Rust: .clone() on Option<Rc<...>> is a cheap ref-count bump,
            // needed because we'll reuse `root` later for the right-height
            // loop and recursive calls
            let mut left_height = 0;
            let mut node = root.clone();

            // Ruby: while node; left_height += 1; node = node.left; end
            // Rust: `while let Some(n) = node` destructures the Option —
            // Rust has no truthy nil, so you must pattern-match
            // .borrow() gives Ref<TreeNode> (RefCell runtime borrow check)
            // .left.clone() clones the Option<Rc<...>> — can't move out of
            // a borrowed RefCell
            while let Some(n) = node {
                left_height += 1;
                node = n.borrow().left.clone();
            }

            // Ruby: right_height = 0; node = root
            let mut right_height = 0;
            let mut node = root.clone();
            while let Some(n) = node {
                right_height += 1;
                node = n.borrow().right.clone();
            }

            // Ruby: return 2**left_height - 1 if left_height == right_height
            // Rust: 1_i32.checked_shl(left_height) avoids panic on overflow
            // (though with constraint <= 5*10^4 nodes, max height ~16, safe)
            if left_height == right_height {
                return (1 << left_height) - 1;
            }

            // Ruby: 1 + count_nodes(root.left) + count_nodes(root.right)
            // Rust: must unwrap the root Rc<RefCell<...>> to access children
            let root_node = root.unwrap();
            let left = root_node.borrow().left.clone();
            let right = root_node.borrow().right.clone();
            1 + Self::count_nodes(left) + Self::count_nodes(right)
        }
    }
}

/// AI solution module.
///
/// Binary search on the last level. Compute tree height h by following
/// the left spine. The last level can have 1..=2^(h-1) nodes. Binary
/// search for the rightmost existing node by traversing root-to-leaf
/// paths using bit masking to decide left/right at each level.
///
/// Time:  O(log^2 n) — O(log n) binary search steps, each traversing O(log n) depth
/// Space: O(1) — iterative, no recursion stack
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Count the number of nodes in a complete binary tree.
        ///
        /// # Arguments
        ///
        /// * `root` - Root of the complete binary tree
        ///
        /// # Returns
        ///
        /// The total number of nodes
        pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let root = match root {
                Some(r) => r,
                None => return 0,
            };

            // Compute height by following the left spine (0-indexed depth)
            // A single node has height 0
            let mut height = 0;
            let mut node = Some(Rc::clone(&root));
            while let Some(n) = node {
                let left = n.borrow().left.clone();
                if left.is_some() {
                    height += 1;
                    node = left;
                } else {
                    node = None;
                }
            }

            // height == 0 means single node (leaf)
            if height == 0 {
                return 1;
            }

            // Binary search for the rightmost node in the last level.
            // Last level indices: 0..2^height (0-indexed, left to right).
            // For each candidate index, traverse from root using bits of
            // the index to decide left (0) or right (1) at each level.
            //
            // Ruby: 2**height  (** operator)
            // Rust: 1 << height — bit shift left is a single CPU instruction,
            // equivalent to 2^height. Rust has no ** operator, so bit shift
            // is the idiomatic way for powers of 2.
            // Gotcha: 1 << 31 on i32 overflows (gives i32::MIN). Safe here
            // since constraint <= 5*10^4 nodes → max height ~16.
            let last_level_capacity = 1 << height;
            let mut lo = 0_i32;
            let mut hi = last_level_capacity - 1;

            while lo < hi {
                // Bias mid toward the right so binary search terminates
                let mid = lo + (hi - lo + 1) / 2;
                if Self::exists(&root, height, mid) {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }

            // Nodes above the last level: 2^height - 1
            // Plus the count of nodes in the last level: lo + 1
            (last_level_capacity - 1) + lo + 1
        }

        /// Check if a node at `idx` (0-indexed) in the last level exists.
        ///
        /// Uses bit masking: starting from the highest bit (level 1),
        /// go left if the bit is 0, right if 1. If we reach the target
        /// depth, the node exists.
        fn exists(root: &Rc<RefCell<TreeNode>>, height: i32, idx: i32) -> bool {
            let mut node = Some(Rc::clone(root));
            // Walk from level 1 down to level `height`
            // The bit at position (height - 1) tells us left/right at level 1,
            // the bit at position 0 tells us left/right at the last level
            let mut bit = 1 << (height - 1);

            while bit > 0 {
                let n = match node {
                    Some(n) => n,
                    None => return false,
                };
                // Bit is 0 → go left; bit is 1 → go right
                node = if idx & bit == 0 {
                    n.borrow().left.clone()
                } else {
                    n.borrow().right.clone()
                };
                bit >>= 1;
            }

            node.is_some()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<Option<i32>>, i32)> = vec![
        (
            vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
            6,
        ),
        (vec![Some(1)], 1),
        (vec![], 0),
    ];

    println!("--- My Solution ---");
    for (vals, expected) in &test_cases {
        let root = build_tree(vals);
        let result = my_solution::Solution::count_nodes(root);
        println!("  tree={vals:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (vals, expected) in &test_cases {
        let root = build_tree(vals);
        let result = ai_solution::Solution::count_nodes(root);
        println!("  tree={vals:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_count_nodes {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;
                use $solution as Solution;

                #[test]
                fn test_full_tree_6_nodes() {
                    let root = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
                    assert_eq!(Solution::count_nodes(root), 6);
                }

                #[test]
                fn test_single_node() {
                    let root = build_tree(&[Some(1)]);
                    assert_eq!(Solution::count_nodes(root), 1);
                }

                #[test]
                fn test_empty_tree() {
                    let root = build_tree(&[]);
                    assert_eq!(Solution::count_nodes(root), 0);
                }

                #[test]
                fn test_perfect_tree_7_nodes() {
                    let root = build_tree(&[
                        Some(1),
                        Some(2),
                        Some(3),
                        Some(4),
                        Some(5),
                        Some(6),
                        Some(7),
                    ]);
                    assert_eq!(Solution::count_nodes(root), 7);
                }

                #[test]
                fn test_two_nodes() {
                    let root = build_tree(&[Some(1), Some(2)]);
                    assert_eq!(Solution::count_nodes(root), 2);
                }

                #[test]
                fn test_three_nodes() {
                    let root = build_tree(&[Some(1), Some(2), Some(3)]);
                    assert_eq!(Solution::count_nodes(root), 3);
                }

                #[test]
                fn test_complete_15_nodes() {
                    let vals: Vec<Option<i32>> = (1..=15).map(Some).collect();
                    let root = build_tree(&vals);
                    assert_eq!(Solution::count_nodes(root), 15);
                }
            }
        };
    }

    test_count_nodes!(my_tests, crate::my_solution::Solution);
    test_count_nodes!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let trees: Vec<Vec<Option<i32>>> = vec![
            vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
            (1..=15).map(Some).collect(),
            (1..=31).map(Some).collect(),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for vals in &trees {
                let root = build_tree(vals);
                std::hint::black_box(crate::my_solution::Solution::count_nodes(
                    std::hint::black_box(root),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for vals in &trees {
                let root = build_tree(vals);
                std::hint::black_box(crate::ai_solution::Solution::count_nodes(
                    std::hint::black_box(root),
                ));
            }
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
