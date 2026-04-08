// LeetCode 589: N-ary Tree Preorder Traversal [Easy]
// https://leetcode.com/problems/n-ary-tree-preorder-traversal/
// Note: No Rust option available on LeetCode for this problem.
//
// Given the root of an n-ary tree, return the preorder traversal of its
// nodes' values.
//
// N-ary tree input serialization is represented in their level order
// traversal. Each group of children is separated by the null value.
//
// Examples:
//   [1,null,3,2,4,null,5,6] -> [1,3,5,6,2,4]
//   [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,
//    null,13,null,null,14] -> [1,2,3,6,7,11,14,4,8,12,5,9,13,10]
//
// Constraints:
//   - The number of nodes in the tree is in the range [0, 10^4]
//   - 0 <= Node.val <= 10^4
//   - The height of the tree is less than or equal to 1000
//
// Run:  cargo run --example lc_0589_n_ary_tree_preorder_traversal
// Test: cargo test --example lc_0589_n_ary_tree_preorder_traversal
// Bench: cargo test --example lc_0589_n_ary_tree_preorder_traversal bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  83.27ms (avg 832ns)
// AI solution:  116.88ms (avg 1.16us)
// -> My solution is 1.40x faster
// Reason: recursive DFS borrows the tree without allocating new heap
//   nodes; the iterative version moves each Box<Node> onto the stack
//   and then extracts children, adding allocation pressure.

/// N-ary tree node definition (shared by both solutions).
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Option<Box<Node>>>,
}

impl Node {
    /// Create a new Node with the given value and no children.
    pub fn new(val: i32) -> Self {
        Node {
            val,
            children: Vec::new(),
        }
    }

    /// Create a new Node with the given value and children.
    pub fn with_children(val: i32, children: Vec<Option<Box<Node>>>) -> Self {
        Node { val, children }
    }
}

/// My solution module — recursive DFS.
///
/// Time:  O(n)
/// Space: O(h) recursion depth, where h = tree height
mod my_solution {
    use crate::Node;

    pub struct Solution;

    impl Solution {
        /// Return the preorder traversal of the n-ary tree.
        ///
        /// # Arguments
        ///
        /// * `root` - Root of the n-ary tree
        ///
        /// # Returns
        ///
        /// Preorder traversal of node values
        pub fn preorder(root: Option<Box<Node>>) -> Vec<i32> {
            let mut result = Vec::new();
            Self::dfs(&root, &mut result);
            result
        }

        fn dfs(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
            if let Some(n) = node {
                result.push(n.val);
                for child in &n.children {
                    Self::dfs(child, result);
                }
            }
        }
    }
}

/// AI solution module — iterative with explicit stack.
///
/// Pushes children in reverse order so the leftmost child is processed
/// first (LIFO stack order).
///
/// Time:  O(n)
/// Space: O(n) for the stack in worst case (wide tree)
mod ai_solution {
    use crate::Node;

    pub struct Solution;

    impl Solution {
        /// Return the preorder traversal of the n-ary tree.
        ///
        /// # Arguments
        ///
        /// * `root` - Root of the n-ary tree
        ///
        /// # Returns
        ///
        /// Preorder traversal of node values
        pub fn preorder(root: Option<Box<Node>>) -> Vec<i32> {
            let mut result = Vec::new();
            let mut stack: Vec<Box<Node>> = Vec::new();
            if let Some(node) = root {
                stack.push(node);
            }
            while let Some(node) = stack.pop() {
                result.push(node.val);
                // Push children in reverse so leftmost is popped first
                // clippy::manual_flatten: iterating + if-let-Some is what
                // .flatten() does — it skips None and unwraps Some values.
                for c in node.children.into_iter().rev().flatten() {
                    stack.push(c);
                }
            }
            result
        }
    }
}

/// Helper to build example tree: [1,null,3,2,4,null,5,6]
///       1
///     / | \
///    3  2  4
///   / \
///  5   6
fn build_example_tree_1() -> Option<Box<Node>> {
    let node5 = Node::new(5);
    let node6 = Node::new(6);
    let node3 = Node::with_children(3, vec![Some(Box::new(node5)), Some(Box::new(node6))]);
    let node2 = Node::new(2);
    let node4 = Node::new(4);
    let root = Node::with_children(
        1,
        vec![
            Some(Box::new(node3)),
            Some(Box::new(node2)),
            Some(Box::new(node4)),
        ],
    );
    Some(Box::new(root))
}

fn main() {
    println!("--- My Solution ---");
    let result = my_solution::Solution::preorder(build_example_tree_1());
    println!("  [1,null,3,2,4,null,5,6] -> {result:?} (expected [1, 3, 5, 6, 2, 4])");

    let result = my_solution::Solution::preorder(None);
    println!("  [] -> {result:?} (expected [])");

    println!("--- AI Solution ---");
    let result = ai_solution::Solution::preorder(build_example_tree_1());
    println!("  [1,null,3,2,4,null,5,6] -> {result:?} (expected [1, 3, 5, 6, 2, 4])");

    let result = ai_solution::Solution::preorder(None);
    println!("  [] -> {result:?} (expected [])");
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_preorder {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::super::*;
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::preorder(build_example_tree_1()),
                        vec![1, 3, 5, 6, 2, 4]
                    );
                }

                #[test]
                fn test_empty_tree() {
                    assert_eq!(Solution::preorder(None), Vec::<i32>::new());
                }

                #[test]
                fn test_single_node() {
                    let root = Some(Box::new(Node::new(42)));
                    assert_eq!(Solution::preorder(root), vec![42]);
                }

                #[test]
                fn test_left_skewed() {
                    // 1 -> 2 -> 3
                    let node3 = Node::new(3);
                    let node2 = Node::with_children(2, vec![Some(Box::new(node3))]);
                    let root = Node::with_children(1, vec![Some(Box::new(node2))]);
                    assert_eq!(Solution::preorder(Some(Box::new(root))), vec![1, 2, 3]);
                }

                #[test]
                fn test_wide_tree() {
                    // Root with 4 leaf children
                    let root = Node::with_children(
                        1,
                        vec![
                            Some(Box::new(Node::new(2))),
                            Some(Box::new(Node::new(3))),
                            Some(Box::new(Node::new(4))),
                            Some(Box::new(Node::new(5))),
                        ],
                    );
                    assert_eq!(
                        Solution::preorder(Some(Box::new(root))),
                        vec![1, 2, 3, 4, 5]
                    );
                }

                #[test]
                fn test_val_zero() {
                    let root = Some(Box::new(Node::new(0)));
                    assert_eq!(Solution::preorder(root), vec![0]);
                }

                #[test]
                fn test_two_levels() {
                    //     10
                    //    / \
                    //   20  30
                    //   |
                    //   40
                    let node40 = Node::new(40);
                    let node20 = Node::with_children(20, vec![Some(Box::new(node40))]);
                    let node30 = Node::new(30);
                    let root = Node::with_children(
                        10,
                        vec![Some(Box::new(node20)), Some(Box::new(node30))],
                    );
                    assert_eq!(
                        Solution::preorder(Some(Box::new(root))),
                        vec![10, 20, 40, 30]
                    );
                }
            }
        };
    }

    test_preorder!(my_tests, crate::my_solution::Solution);
    test_preorder!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;

        let start = Instant::now();
        for _ in 0..iterations {
            let tree = build_example_tree_1();
            std::hint::black_box(crate::my_solution::Solution::preorder(
                std::hint::black_box(tree),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let tree = build_example_tree_1();
            std::hint::black_box(crate::ai_solution::Solution::preorder(
                std::hint::black_box(tree),
            ));
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
