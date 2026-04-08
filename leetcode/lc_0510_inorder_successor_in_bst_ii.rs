// LeetCode 510: Inorder Successor in BST II [Medium] (Premium)
// https://leetcode.com/problems/inorder-successor-in-bst-ii/
//
// Note: Premium problem -- description sourced from doocs/leetcode.
//
// Given a node in a BST, return the in-order successor of that node.
// If there is no successor, return None. The successor is the node with the
// smallest key greater than node.val. Each node has a reference to its parent.
//
// Unlike the standard BST problem, you have direct access to the node but
// NOT to the root. Each node has: val, left, right, parent.
//
// Algorithm:
//   1. If node has a right subtree, successor = leftmost node in right subtree.
//   2. Otherwise, walk up via parent until we find a node that is its parent's
//      left child. That parent is the successor.
//
// Examples:
//   tree = [2,1,3], node = 1 -> 2
//   tree = [5,3,6,2,4,null,null,1], node = 6 -> null
//
// Constraints:
//   1 <= number of nodes <= 10^4
//   -10^5 <= Node.val <= 10^5
//   All values are unique
//
// Note: This problem requires parent pointers (not standard LeetCode TreeNode).
// We define a custom Node struct with parent, left, right, and val fields
// using Rc<RefCell<>> for shared ownership and interior mutability.
//
// Run:  cargo run --example lc_0510_inorder_successor_in_bst_ii
// Test: cargo test --example lc_0510_inorder_successor_in_bst_ii
// Bench: cargo test --example lc_0510_inorder_successor_in_bst_ii bench -- --nocapture
//
// First benchmark (100K iterations, 2026-04-08):
//   My solution: avg 295ns
//   AI solution: avg 292ns
//   ~tied (ratio: 1.01)
//   Reason: both use identical algorithm (right-subtree leftmost or walk-up);
//   minor style differences don't affect performance.

use std::cell::RefCell;
use std::rc::Rc;

/// Custom node with parent pointer for this problem.
#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
            parent: None,
        }
    }
}

type NodeRef = Option<Rc<RefCell<Node>>>;

/// Build a BST from level-order values and return a vec of all node refs
/// (indexed by their position in the level-order array).
fn build_tree_with_parents(vals: &[Option<i32>]) -> Vec<NodeRef> {
    let nodes: Vec<NodeRef> = vals
        .iter()
        .map(|v| v.map(|val| Rc::new(RefCell::new(Node::new(val)))))
        .collect();

    let mut queue = std::collections::VecDeque::new();
    if let Some(Some(root)) = nodes.first() {
        queue.push_back((Rc::clone(root), 0usize));
    }

    while let Some((node, idx)) = queue.pop_front() {
        let left_idx = 2 * idx + 1;
        let right_idx = 2 * idx + 2;

        if left_idx < nodes.len() {
            if let Some(ref left) = nodes[left_idx] {
                node.borrow_mut().left = Some(Rc::clone(left));
                left.borrow_mut().parent = Some(Rc::clone(&node));
                queue.push_back((Rc::clone(left), left_idx));
            }
        }
        if right_idx < nodes.len() {
            if let Some(ref right) = nodes[right_idx] {
                node.borrow_mut().right = Some(Rc::clone(right));
                right.borrow_mut().parent = Some(Rc::clone(&node));
                queue.push_back((Rc::clone(right), right_idx));
            }
        }
    }

    nodes
}

/// My solution — case analysis on right subtree vs parent chain.
///
/// If node has right child, go right then all the way left.
/// Otherwise walk up until we're a left child; that parent is the successor.
/// Translated from Java solution in doocs/leetcode.
///
/// Time:  O(h) where h is tree height
/// Space: O(1)
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the in-order successor of the given node.
        ///
        /// # Arguments
        /// * `node` - reference to a node in the BST
        ///
        /// # Returns
        /// The successor node, or None
        pub fn inorder_successor(node: Rc<RefCell<Node>>) -> NodeRef {
            let node_borrow = node.borrow();

            // Case 1: has right subtree -> find leftmost in right subtree
            if let Some(ref right) = node_borrow.right {
                let mut curr = Rc::clone(right);
                loop {
                    let next = {
                        let b = curr.borrow();
                        b.left.as_ref().map(Rc::clone)
                    };
                    match next {
                        Some(left) => curr = left,
                        None => return Some(curr),
                    }
                }
            }

            // Case 2: no right subtree -> walk up until we're a left child
            let parent = node_borrow.parent.as_ref().map(Rc::clone);
            drop(node_borrow); // Release borrow before comparing

            let mut child = Rc::clone(&node);
            let mut par = parent;

            while let Some(p) = par {
                let is_right_child = {
                    let p_borrow = p.borrow();
                    p_borrow
                        .right
                        .as_ref()
                        .is_some_and(|r| Rc::ptr_eq(r, &child))
                };
                if is_right_child {
                    child = Rc::clone(&p);
                    par = p.borrow().parent.as_ref().map(Rc::clone);
                } else {
                    return Some(p);
                }
            }

            None // No successor (node is the rightmost / largest)
        }
    }
}

/// AI solution — same algorithm expressed slightly differently.
///
/// Uses an iterative approach with Rc::clone to avoid borrow conflicts.
///
/// Time:  O(h)
/// Space: O(1)
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the in-order successor of the given node.
        ///
        /// # Arguments
        /// * `node` - reference to a node in the BST
        ///
        /// # Returns
        /// The successor node, or None
        pub fn inorder_successor(node: Rc<RefCell<Node>>) -> NodeRef {
            // Check if node has right subtree
            let has_right = node.borrow().right.is_some();

            if has_right {
                // Successor is the leftmost node in the right subtree
                let mut curr = node
                    .borrow()
                    .right
                    .as_ref()
                    .map(Rc::clone)
                    .expect("right subtree exists");
                loop {
                    let left = curr.borrow().left.as_ref().map(Rc::clone);
                    match left {
                        Some(l) => curr = l,
                        None => return Some(curr),
                    }
                }
            } else {
                // Walk up: find the first ancestor where we came from the left
                let mut current = Rc::clone(&node);
                loop {
                    let parent = current.borrow().parent.as_ref().map(Rc::clone);
                    match parent {
                        Some(p) => {
                            let came_from_right = p
                                .borrow()
                                .right
                                .as_ref()
                                .is_some_and(|r| Rc::ptr_eq(r, &current));
                            if came_from_right {
                                current = Rc::clone(&p);
                            } else {
                                return Some(p);
                            }
                        }
                        None => return None,
                    }
                }
            }
        }
    }
}

fn main() {
    // Build tree: [2, 1, 3]
    let nodes = build_tree_with_parents(&[Some(2), Some(1), Some(3)]);
    // Node with value 1 is at index 1
    if let Some(ref node1) = nodes[1] {
        let my = my_solution::Solution::inorder_successor(Rc::clone(node1));
        println!(
            "my successor of 1: {:?}",
            my.as_ref().map(|n| n.borrow().val)
        );
    }
    // Rebuild for AI solution (tree is consumed by borrows)
    let nodes = build_tree_with_parents(&[Some(2), Some(1), Some(3)]);
    if let Some(ref node1) = nodes[1] {
        let ai = ai_solution::Solution::inorder_successor(Rc::clone(node1));
        println!(
            "ai successor of 1: {:?}",
            ai.as_ref().map(|n| n.borrow().val)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn successor_val(node: &Rc<RefCell<Node>>) -> Option<i32> {
        my_solution::Solution::inorder_successor(Rc::clone(node)).map(|n| n.borrow().val)
    }

    fn successor_val_ai(node: &Rc<RefCell<Node>>) -> Option<i32> {
        ai_solution::Solution::inorder_successor(Rc::clone(node)).map(|n| n.borrow().val)
    }

    #[test]
    fn test_my_basic() {
        // [2, 1, 3]: successor of 1 is 2
        let nodes = build_tree_with_parents(&[Some(2), Some(1), Some(3)]);
        assert_eq!(successor_val(nodes[1].as_ref().expect("node 1")), Some(2));
    }

    #[test]
    fn test_my_no_successor() {
        // [2, 1, 3]: successor of 3 is None
        let nodes = build_tree_with_parents(&[Some(2), Some(1), Some(3)]);
        assert_eq!(successor_val(nodes[2].as_ref().expect("node 3")), None);
    }

    #[test]
    fn test_ai_basic() {
        let nodes = build_tree_with_parents(&[Some(2), Some(1), Some(3)]);
        assert_eq!(
            successor_val_ai(nodes[1].as_ref().expect("node 1")),
            Some(2)
        );
    }

    #[test]
    fn test_ai_no_successor() {
        let nodes = build_tree_with_parents(&[Some(2), Some(1), Some(3)]);
        assert_eq!(successor_val_ai(nodes[2].as_ref().expect("node 3")), None);
    }

    #[test]
    fn test_my_root_successor() {
        // [2, 1, 3]: successor of 2 is 3
        let nodes = build_tree_with_parents(&[Some(2), Some(1), Some(3)]);
        assert_eq!(successor_val(nodes[0].as_ref().expect("node 2")), Some(3));
    }

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<Option<i32>> = vec![
            Some(5),
            Some(3),
            Some(7),
            Some(2),
            Some(4),
            Some(6),
            Some(8),
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let nodes = build_tree_with_parents(&vals);
            if let Some(ref node) = nodes[3] {
                std::hint::black_box(my_solution::Solution::inorder_successor(Rc::clone(node)));
            }
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let nodes = build_tree_with_parents(&vals);
            if let Some(ref node) = nodes[3] {
                std::hint::black_box(ai_solution::Solution::inorder_successor(Rc::clone(node)));
            }
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
