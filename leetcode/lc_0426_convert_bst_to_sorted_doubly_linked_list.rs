// LeetCode 426: Convert Binary Search Tree to Sorted Doubly Linked List [Medium]
// https://leetcode.com/problems/convert-binary-search-tree-to-sorted-doubly-linked-list/
//
// Convert a BST to a sorted circular doubly-linked list in-place.
// Left pointer = predecessor, right pointer = successor.
// Return pointer to the smallest element.
//
// Since Rust's ownership model makes true in-place pointer manipulation
// with Rc<RefCell> impractical, we adapt: collect inorder values, then
// build a new circular doubly-linked list using a Node struct with
// left (prev) and right (next) pointers.
//
// Examples:
//   root = [4,2,5,1,3] -> circular list [1,2,3,4,5]
//   root = [2,1,3]     -> circular list [1,2,3]
//   root = []           -> None
//
// Constraints:
//   0 <= number of nodes <= 2000
//   -1000 <= Node.val <= 1000
//
// Run:  cargo run --example lc_0426_convert_bst_to_sorted_doubly_linked_list
// Test: cargo test --example lc_0426_convert_bst_to_sorted_doubly_linked_list
// Bench: cargo test --example lc_0426_convert_bst_to_sorted_doubly_linked_list bench -- --nocapture
//
// First benchmark (200K iterations, 2026-04-08):
//   My solution: avg 3.908us
//   AI solution: avg 3.961us
//   Ratio: 1.01x (similar, both inorder-based)

use std::cell::RefCell;
use std::rc::Rc;

/// Binary search tree node.
#[derive(Debug, Clone, PartialEq)]
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

/// Doubly-linked list node. `left` = prev, `right` = next.
#[derive(Debug, Clone, PartialEq)]
pub struct DllNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<DllNode>>>,
    pub right: Option<Rc<RefCell<DllNode>>>,
}

impl DllNode {
    pub fn new(val: i32) -> Self {
        DllNode {
            val,
            left: None,
            right: None,
        }
    }
}

type TreeLink = Option<Rc<RefCell<TreeNode>>>;
type DllLink = Option<Rc<RefCell<DllNode>>>;

/// Time: O(n), Space: O(n)
/// Recursive inorder traversal to collect sorted values, then build
/// circular doubly-linked list by linking nodes sequentially and
/// connecting head/tail.
#[allow(dead_code)]
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Convert BST to sorted circular doubly-linked list.
        ///
        /// # Arguments
        /// * `root` - Root of the BST
        ///
        /// # Returns
        /// Pointer to the smallest element of the circular list, or None.
        pub fn tree_to_doubly_list(root: TreeLink) -> DllLink {
            let mut vals = Vec::new();
            Self::inorder(&root, &mut vals);
            if vals.is_empty() {
                return None;
            }
            Self::build_circular_dll(&vals)
        }

        /// Recursive inorder traversal collecting values in sorted order.
        fn inorder(node: &TreeLink, vals: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                Self::inorder(&n.left, vals);
                vals.push(n.val);
                Self::inorder(&n.right, vals);
            }
        }

        /// Build a circular doubly-linked list from sorted values.
        ///
        /// Creates DllNode for each value, chains left/right pointers,
        /// and connects head <-> tail to form the circle.
        fn build_circular_dll(vals: &[i32]) -> DllLink {
            let nodes: Vec<Rc<RefCell<DllNode>>> = vals
                .iter()
                .map(|&v| Rc::new(RefCell::new(DllNode::new(v))))
                .collect();

            // Link consecutive nodes
            for i in 0..nodes.len() - 1 {
                nodes[i].borrow_mut().right = Some(Rc::clone(&nodes[i + 1]));
                nodes[i + 1].borrow_mut().left = Some(Rc::clone(&nodes[i]));
            }

            // Close the circle: tail.right = head, head.left = tail
            let head = &nodes[0];
            let tail = &nodes[nodes.len() - 1];
            tail.borrow_mut().right = Some(Rc::clone(head));
            head.borrow_mut().left = Some(Rc::clone(tail));

            Some(Rc::clone(head))
        }
    }
}

/// Time: O(n), Space: O(h) for the stack + O(n) for the DLL nodes
/// Iterative inorder traversal using an explicit stack. Directly link
/// DllNodes as we visit BST nodes, then close the circle at the end.
#[allow(dead_code)]
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Convert BST to sorted circular doubly-linked list.
        ///
        /// # Arguments
        /// * `root` - Root of the BST
        ///
        /// # Returns
        /// Pointer to the smallest element of the circular list, or None.
        pub fn tree_to_doubly_list(root: TreeLink) -> DllLink {
            if root.is_none() {
                return None;
            }

            let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
            let mut current = root;
            let mut head: DllLink = None;
            let mut prev: DllLink = None;

            // Iterative inorder: push left children, then process, then go right
            loop {
                // Drill down to leftmost node
                while let Some(node) = current {
                    stack.push(Rc::clone(&node));
                    let left = node.borrow().left.clone();
                    current = left;
                }

                if let Some(node) = stack.pop() {
                    let val = node.borrow().val;
                    let dll_node = Rc::new(RefCell::new(DllNode::new(val)));

                    if let Some(ref p) = prev {
                        // Link previous node -> current
                        p.borrow_mut().right = Some(Rc::clone(&dll_node));
                        dll_node.borrow_mut().left = Some(Rc::clone(p));
                    } else {
                        // First node visited is the head (smallest)
                        head = Some(Rc::clone(&dll_node));
                    }
                    prev = Some(dll_node);

                    // Move to right subtree
                    current = node.borrow().right.clone();
                } else {
                    break;
                }
            }

            // Close the circle: tail.right = head, head.left = tail
            if let (Some(h), Some(t)) = (&head, &prev) {
                t.borrow_mut().right = Some(Rc::clone(h));
                h.borrow_mut().left = Some(Rc::clone(t));
            }

            head
        }
    }
}

/// Build a BST from a level-order array (None = null node).
fn build_bst(vals: &[Option<i32>]) -> TreeLink {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vals
        .iter()
        .map(|v| v.map(|val| Rc::new(RefCell::new(TreeNode::new(val)))))
        .collect();

    // Link parent -> children via level-order indices
    let mut child_idx = 1;
    for i in 0..nodes.len() {
        if let Some(ref node) = nodes[i] {
            if child_idx < nodes.len() {
                node.borrow_mut().left = nodes[child_idx].clone();
                child_idx += 1;
            }
            if child_idx < nodes.len() {
                node.borrow_mut().right = nodes[child_idx].clone();
                child_idx += 1;
            }
        }
    }
    nodes[0].clone()
}

/// Extract values from circular DLL by walking right from head.
/// Returns empty vec if head is None.
fn dll_to_vec(head: &DllLink) -> Vec<i32> {
    let mut result = Vec::new();
    if let Some(h) = head {
        let mut cur = Rc::clone(h);
        loop {
            result.push(cur.borrow().val);
            let next = cur.borrow().right.clone();
            match next {
                Some(ref n) if !Rc::ptr_eq(n, h) => cur = Rc::clone(n),
                _ => break,
            }
        }
    }
    result
}

/// Verify the circular DLL: walk forward and backward, check circularity.
#[allow(dead_code)]
fn verify_circular_dll(head: &DllLink, expected: &[i32]) -> bool {
    let forward = dll_to_vec(head);
    if forward != expected {
        return false;
    }
    if expected.is_empty() {
        return true;
    }
    // Verify backward traversal via left pointers
    let h = head.as_ref().unwrap();
    let tail = h.borrow().left.clone().unwrap();
    let mut backward = Vec::new();
    let mut cur = Rc::clone(&tail);
    loop {
        backward.push(cur.borrow().val);
        let prev = cur.borrow().left.clone();
        match prev {
            Some(ref p) if !Rc::ptr_eq(p, &tail) => cur = Rc::clone(p),
            _ => break,
        }
    }
    backward.reverse();
    backward == expected
}

fn main() {
    println!("LeetCode 426: Convert BST to Sorted Doubly Linked List");
    let root = build_bst(&[Some(4), Some(2), Some(5), Some(1), Some(3)]);
    let r1 = my_solution::Solution::tree_to_doubly_list(root.clone());
    let r2 = ai_solution::Solution::tree_to_doubly_list(root);
    println!("  my:  {:?}", dll_to_vec(&r1));
    println!("  ai:  {:?}", dll_to_vec(&r2));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_both {
        ($name:ident, $tree:expr, $expected:expr) => {
            mod $name {
                use super::*;
                #[test]
                fn my() {
                    let root = build_bst(&$tree);
                    let result = my_solution::Solution::tree_to_doubly_list(root);
                    let expected: &[i32] = &$expected;
                    assert!(
                        verify_circular_dll(&result, expected),
                        "my_solution: got {:?}, expected {:?}",
                        dll_to_vec(&result),
                        expected
                    );
                }
                #[test]
                fn ai() {
                    let root = build_bst(&$tree);
                    let result = ai_solution::Solution::tree_to_doubly_list(root);
                    let expected: &[i32] = &$expected;
                    assert!(
                        verify_circular_dll(&result, expected),
                        "ai_solution: got {:?}, expected {:?}",
                        dll_to_vec(&result),
                        expected
                    );
                }
            }
        };
    }

    test_both!(
        example1,
        [Some(4), Some(2), Some(5), Some(1), Some(3)],
        [1, 2, 3, 4, 5]
    );
    test_both!(example2, [Some(2), Some(1), Some(3)], [1, 2, 3]);
    test_both!(empty_tree, [], []);
    test_both!(single_node, [Some(1)], [1]);
    test_both!(two_nodes_left, [Some(2), Some(1)], [1, 2]);
    test_both!(two_nodes_right, [Some(1), None, Some(2)], [1, 2]);
    test_both!(left_skew, [Some(3), Some(2), None, Some(1)], [1, 2, 3]);
    test_both!(
        right_skew,
        [Some(1), None, Some(2), None, Some(3)],
        [1, 2, 3]
    );
    test_both!(
        seven_nodes,
        [
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            Some(5),
            Some(7)
        ],
        [1, 2, 3, 4, 5, 6, 7]
    );
    test_both!(
        negative_vals,
        [
            Some(0),
            Some(-2),
            Some(2),
            Some(-3),
            Some(-1),
            Some(1),
            Some(3)
        ],
        [-3, -2, -1, 0, 1, 2, 3]
    );
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(TreeLink) -> DllLink,
{
    let iterations = 200_000;
    // Build a balanced BST with 127 nodes (depth 7)
    fn build_balanced(vals: &[i32]) -> TreeLink {
        if vals.is_empty() {
            return None;
        }
        let mid = vals.len() / 2;
        let node = Rc::new(RefCell::new(TreeNode::new(vals[mid])));
        node.borrow_mut().left = build_balanced(&vals[..mid]);
        node.borrow_mut().right = build_balanced(&vals[mid + 1..]);
        Some(node)
    }
    let vals: Vec<i32> = (1..=127).collect();
    let root = build_balanced(&vals);

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(root.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::tree_to_doubly_list);
    bench_solution("AI solution", ai_solution::Solution::tree_to_doubly_list);
}
