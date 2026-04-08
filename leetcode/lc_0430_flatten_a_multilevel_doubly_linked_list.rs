// LeetCode 430: Flatten a Multilevel Doubly Linked List [Medium]
// https://leetcode.com/problems/flatten-a-multilevel-doubly-linked-list/
//
// Given a doubly linked list where nodes have next, prev, and child pointers,
// flatten it so all nodes appear in a single-level doubly linked list.
// Child list nodes appear after curr and before curr.next.
//
// Examples:
//   [1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
//   -> [1,2,3,7,8,11,12,9,10,4,5,6]
//
// Constraints:
//   Number of nodes <= 1000
//   1 <= Node.val <= 10^5
//
// Run:  cargo run --example lc_0430_flatten_a_multilevel_doubly_linked_list
// Test: cargo test --example lc_0430_flatten_a_multilevel_doubly_linked_list
// Bench: cargo test --example lc_0430_flatten_a_multilevel_doubly_linked_list bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 282ns
//   AI solution: avg 594ns
//   Ratio: 0.47x (recursive faster than stack)
//
// Note: Since Rust's ownership model makes true doubly-linked lists complex,
// we represent the problem using a simplified structure and convert to/from
// vectors for testing. The algorithmic logic is preserved.

/// Simplified node for the multilevel list problem.
/// We store val and child nodes in a tree-like structure,
/// then flatten into a Vec<i32> representing the final order.
#[derive(Debug, Clone)]
pub struct MlNode {
    pub val: i32,
    pub next: Option<Box<MlNode>>,
    pub child: Option<Box<MlNode>>,
}

impl MlNode {
    pub fn new(val: i32) -> Self {
        MlNode {
            val,
            next: None,
            child: None,
        }
    }
}

/// Convert a flattened list (no children) to a Vec.
fn to_vec(mut node: Option<Box<MlNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(n) = node {
        result.push(n.val);
        node = n.next;
    }
    result
}

/// My solution — recursive flatten (translated from doocs Python/Java).
///
/// Recursively process the list: when a node has a child, flatten the
/// child list, connect it in place, then continue with the rest.
///
/// Time:  O(n)
/// Space: O(d) — depth of child nesting
#[allow(dead_code)]
mod my_solution {
    use super::*;

    pub struct Solution;
    impl Solution {
        /// Flatten a multilevel doubly linked list.
        ///
        /// # Arguments
        /// * `head` - head of the multilevel list
        ///
        /// # Returns
        /// Head of the flattened single-level list.
        pub fn flatten(head: Option<Box<MlNode>>) -> Option<Box<MlNode>> {
            if head.is_none() {
                return None;
            }
            let mut dummy = Box::new(MlNode::new(0));
            Self::flatten_dfs(&mut dummy, head);
            dummy.next
        }

        /// DFS flatten: returns the tail of the flattened portion.
        fn flatten_dfs<'a>(
            pre: &'a mut Box<MlNode>,
            cur: Option<Box<MlNode>>,
        ) -> &'a mut Box<MlNode> {
            if let Some(mut node) = cur {
                let next = node.next.take();
                let child = node.child.take();
                // Link current node after pre
                pre.next = Some(node);
                let tail = pre.next.as_mut().unwrap();
                // Recursively flatten child first, then next
                let tail = Self::flatten_dfs(tail, child);
                Self::flatten_dfs(tail, next)
            } else {
                pre
            }
        }
    }
}

/// AI solution — iterative with stack.
///
/// Use a stack to process nodes. When encountering a child,
/// push next onto the stack and process child first.
///
/// Time:  O(n)
/// Space: O(d) — stack depth
mod ai_solution {
    use super::*;

    pub struct Solution;
    impl Solution {
        /// Flatten using an iterative stack approach.
        pub fn flatten(head: Option<Box<MlNode>>) -> Option<Box<MlNode>> {
            let mut result = Vec::new();
            let mut stack: Vec<Box<MlNode>> = Vec::new();
            if let Some(node) = head {
                stack.push(node);
            }
            while let Some(mut node) = stack.pop() {
                result.push(node.val);
                // Push next first (processed later), then child (processed next)
                if let Some(next) = node.next.take() {
                    stack.push(next);
                }
                if let Some(child) = node.child.take() {
                    stack.push(child);
                }
            }
            // Build the result list from collected values
            let mut head = None;
            for &val in result.iter().rev() {
                let mut node = MlNode::new(val);
                node.next = head;
                head = Some(Box::new(node));
            }
            head
        }
    }
}

fn main() {
    // Build: 1 -> 2 -> 3, and 2 has child 4 -> 5
    let n5 = MlNode::new(5);
    let n4 = MlNode {
        val: 4,
        next: Some(Box::new(n5)),
        child: None,
    };
    let n3 = MlNode::new(3);
    let n2 = MlNode {
        val: 2,
        next: Some(Box::new(n3)),
        child: Some(Box::new(n4)),
    };
    let n1 = MlNode {
        val: 1,
        next: Some(Box::new(n2)),
        child: None,
    };

    let head = Some(Box::new(n1));
    println!(
        "my:  {:?}",
        to_vec(my_solution::Solution::flatten(head.clone()))
    );
    println!("ai:  {:?}", to_vec(ai_solution::Solution::flatten(head)));
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::*;
        use $module::Solution;

        #[test]
        fn test_basic_with_child() {
            // 1 -> 2(child: 4->5) -> 3
            // Expected: [1, 2, 4, 5, 3]
            let n3 = MlNode::new(3);
            let n5 = MlNode::new(5);
            let n4 = MlNode {
                val: 4,
                next: Some(Box::new(n5)),
                child: None,
            };
            let n2 = MlNode {
                val: 2,
                next: Some(Box::new(n3)),
                child: Some(Box::new(n4)),
            };
            let n1 = MlNode {
                val: 1,
                next: Some(Box::new(n2)),
                child: None,
            };
            let result = to_vec(Solution::flatten(Some(Box::new(n1))));
            assert_eq!(result, vec![1, 2, 4, 5, 3]);
        }

        #[test]
        fn test_empty() {
            assert_eq!(to_vec(Solution::flatten(None)), Vec::<i32>::new());
        }

        #[test]
        fn test_no_children() {
            // 1 -> 2 -> 3
            let n3 = MlNode::new(3);
            let n2 = MlNode {
                val: 2,
                next: Some(Box::new(n3)),
                child: None,
            };
            let n1 = MlNode {
                val: 1,
                next: Some(Box::new(n2)),
                child: None,
            };
            let result = to_vec(Solution::flatten(Some(Box::new(n1))));
            assert_eq!(result, vec![1, 2, 3]);
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
    F: Fn(Option<Box<MlNode>>) -> Option<Box<MlNode>>,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let n3 = MlNode::new(3);
        let n5 = MlNode::new(5);
        let n4 = MlNode {
            val: 4,
            next: Some(Box::new(n5)),
            child: None,
        };
        let n2 = MlNode {
            val: 2,
            next: Some(Box::new(n3)),
            child: Some(Box::new(n4)),
        };
        let n1 = MlNode {
            val: 1,
            next: Some(Box::new(n2)),
            child: None,
        };
        std::hint::black_box(f(std::hint::black_box(Some(Box::new(n1)))));
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
    bench_solution("My solution", my_solution::Solution::flatten);
    bench_solution("AI solution", ai_solution::Solution::flatten);
}
