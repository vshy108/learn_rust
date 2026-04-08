// LeetCode 236: Lowest Common Ancestor of a Binary Tree [Medium]
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.674us
//   AI solution avg: 1.039us
//   Ratio: My 1.54x faster
//
// Given a binary tree, find the lowest common ancestor (LCA) of two
// given nodes in the tree. The LCA is the lowest node that has both p
// and q as descendants (a node can be a descendant of itself).
//
// Unlike LC 235, this is a general binary tree (NOT a BST).
//
// Constraints: [2, 10^5] nodes, -10^9 <= Node.val <= 10^9,
// All values are unique, p != q, p and q exist in the tree.
//
// Run:  cargo run --example lc_0236_lowest_common_ancestor_bt
// Test: cargo test --example lc_0236_lowest_common_ancestor_bt
// Bench: cargo test --example lc_0236_lowest_common_ancestor_bt bench -- --nocapture

use std::cell::RefCell;
use std::rc::Rc;

/// Binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// Create a leaf node.
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

/// My solution — recursive DFS returning Option<val>.
///
/// Post-order traversal: if the current node matches p or q, return it.
/// Recurse into left and right children. If both sides return a value,
/// the current node is the LCA. If only one side returns, propagate it.
///
/// Time: O(n) — visit every node.
/// Space: O(h) — recursion stack.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the value of the LCA of nodes with values `p` and `q`.
        pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> i32 {
            Self::dfs(&root, p, q).expect("LCA must exist since p and q are in the tree")
        }

        /// Return Some(val) if this subtree contains p, q, or the LCA.
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<i32> {
            let n = node.as_ref()?;
            let borrowed = n.borrow();

            let left = Self::dfs(&borrowed.left, p, q);
            let right = Self::dfs(&borrowed.right, p, q);

            if left.is_some() && right.is_some() {
                // Both sides found a target — this node is the LCA
                return Some(borrowed.val);
            }

            if borrowed.val == p || borrowed.val == q {
                // Current node matches one target; if a child also matched,
                // current node is the LCA, otherwise propagate upward
                return Some(borrowed.val);
            }

            // Propagate whichever side found something (or None)
            left.or(right)
        }
    }
}

/// AI solution — iterative with parent pointers.
///
/// Build a parent map via BFS. Then walk from p to root collecting
/// ancestors. Walk from q upward until hitting a node in p's ancestor set.
///
/// Time: O(n) for BFS + O(h) for ancestor walk.
/// Space: O(n) for parent map and ancestor set.
mod ai_solution {
    use super::*;
    use std::collections::{HashMap, HashSet, VecDeque};

    pub struct Solution;

    impl Solution {
        /// Return the value of the LCA of nodes with values `p` and `q`.
        pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> i32 {
            // BFS to build parent map: child_val -> parent_val
            let mut parent: HashMap<i32, i32> = HashMap::new();
            let mut queue = VecDeque::new();

            if let Some(ref r) = root {
                queue.push_back(Rc::clone(r));
            }

            // BFS until we've found both p and q's parents
            while let Some(node) = queue.pop_front() {
                let borrowed = node.borrow();
                if let Some(ref left) = borrowed.left {
                    parent.insert(left.borrow().val, borrowed.val);
                    queue.push_back(Rc::clone(left));
                }
                if let Some(ref right) = borrowed.right {
                    parent.insert(right.borrow().val, borrowed.val);
                    queue.push_back(Rc::clone(right));
                }
            }

            // Collect all ancestors of p (including p itself)
            let mut ancestors: HashSet<i32> = HashSet::new();
            let mut current = p;
            ancestors.insert(current);
            while let Some(&par) = parent.get(&current) {
                ancestors.insert(par);
                current = par;
            }

            // Walk from q upward until we hit an ancestor of p
            let mut current = q;
            while !ancestors.contains(&current) {
                current = *parent.get(&current).expect("q must be in tree");
            }

            current
        }
    }
}

/// Build a tree from a level-order Vec (None = null node).
fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;
    while i < values.len() {
        if let Some(parent) = queue.pop_front() {
            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    parent.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < values.len() {
                if let Some(val) = values[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    parent.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }
    }
    Some(root)
}

fn main() {
    let tree = build_tree(&[
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);

    let my1 = my_solution::Solution::lowest_common_ancestor(tree.clone(), 5, 1);
    let ai1 = ai_solution::Solution::lowest_common_ancestor(tree.clone(), 5, 1);
    println!("LCA(5,1): my={my1}, ai={ai1} (expected 3)");

    let my2 = my_solution::Solution::lowest_common_ancestor(tree.clone(), 5, 4);
    let ai2 = ai_solution::Solution::lowest_common_ancestor(tree, 5, 4);
    println!("LCA(5,4): my={my2}, ai={ai2} (expected 5)");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> Option<Rc<RefCell<TreeNode>>> {
        build_tree(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ])
    }

    macro_rules! test_lca {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::lowest_common_ancestor(sample_tree(), 5, 1), 3,);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::lowest_common_ancestor(sample_tree(), 5, 4), 5,);
                }

                #[test]
                fn test_leaf_nodes() {
                    assert_eq!(<$solution>::lowest_common_ancestor(sample_tree(), 6, 4), 5,);
                }

                #[test]
                fn test_across_root() {
                    assert_eq!(<$solution>::lowest_common_ancestor(sample_tree(), 7, 0), 3,);
                }

                #[test]
                fn test_simple() {
                    let tree = build_tree(&[Some(1), Some(2)]);
                    assert_eq!(<$solution>::lowest_common_ancestor(tree, 1, 2), 1,);
                }
            }
        };
    }

    test_lca!(my, crate::my_solution::Solution);
    test_lca!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let tree_vals = vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let tree = build_tree(&tree_vals);
            std::hint::black_box(crate::my_solution::Solution::lowest_common_ancestor(
                tree, 6, 4,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let tree = build_tree(&tree_vals);
            std::hint::black_box(crate::ai_solution::Solution::lowest_common_ancestor(
                tree, 6, 4,
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
