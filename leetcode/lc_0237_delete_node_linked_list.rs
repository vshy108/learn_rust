// LeetCode 237: Delete Node in a Linked List [Medium]
// https://leetcode.com/problems/delete-node-in-a-linked-list/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.624us
//   AI solution avg: 0.424us
//   Ratio: AI 1.47x faster
//
// There is a singly-linked list. You are given the node to be deleted
// (not the tail). You do NOT have access to the head. Delete the given
// node by copying the next node's value into the given node, then
// bypassing the next node.
//
// Constraints: [2, 1000] nodes, -1000 <= Node.val <= 1000,
// The node to delete is NOT a tail node.
//
// Note: Rust's ownership model makes this unusual. We simulate the
// "given a mutable reference to the node" pattern.
//
// Run:  cargo run --example lc_0237_delete_node_linked_list
// Test: cargo test --example lc_0237_delete_node_linked_list
// Bench: cargo test --example lc_0237_delete_node_linked_list bench -- --nocapture

/// Singly-linked list node.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    /// Create a node with no next pointer.
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

/// Build a linked list from a slice.
fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// Collect values from a linked list into a Vec.
fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;
    while let Some(node) = current {
        result.push(node.val);
        current = &node.next;
    }
    result
}

/// My solution — copy-and-skip.
///
/// Copy the next node's value into the current node, then skip next.
/// This is the classic trick for "delete without head access".
///
/// Time: O(1).
/// Space: O(1).
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Delete the given node (must not be the tail) by copying the
        /// next node's value and bypassing it.
        pub fn delete_node(node: &mut Box<ListNode>) {
            // Copy the next node's value into this node
            let next = node.next.take().expect("node is not the tail");
            node.val = next.val;
            // Skip over the (now-duplicate) next node
            node.next = next.next;
        }
    }
}

/// AI solution — same approach (there's only one way to solve this).
///
/// The copy-and-skip trick is the *only* way to delete a node without
/// head access. Both solutions are identical in logic; the AI version
/// uses a slightly different destructuring style.
///
/// Time: O(1).
/// Space: O(1).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Delete the given node by overwriting it with its successor.
        pub fn delete_node(node: &mut Box<ListNode>) {
            // Destructure the next node
            let next_node = node.next.take().expect("guaranteed not tail");
            // Overwrite current node's fields
            *node = next_node;
        }
    }
}

fn main() {
    // [4,5,1,9], delete node with val 5 => [4,1,9]
    let mut list = build_list(&[4, 5, 1, 9]);
    // Navigate to the node with val 5
    let node = list.as_mut().unwrap().next.as_mut().unwrap();
    my_solution::Solution::delete_node(node);
    println!("After deleting 5: {:?} (expected [4, 1, 9])", to_vec(&list));

    let mut list2 = build_list(&[4, 5, 1, 9]);
    let node2 = list2.as_mut().unwrap().next.as_mut().unwrap();
    ai_solution::Solution::delete_node(node2);
    println!(
        "After deleting 5: {:?} (expected [4, 1, 9])",
        to_vec(&list2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: find a mutable reference to the node with the given value.
    fn find_node(head: &mut Option<Box<ListNode>>, val: i32) -> &mut Box<ListNode> {
        let mut current = head.as_mut().unwrap();
        while current.val != val {
            current = current.next.as_mut().expect("value must exist in list");
        }
        current
    }

    macro_rules! test_delete {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_delete_middle() {
                    let mut list = build_list(&[4, 5, 1, 9]);
                    let node = find_node(&mut list, 5);
                    <$solution>::delete_node(node);
                    assert_eq!(to_vec(&list), vec![4, 1, 9]);
                }

                #[test]
                fn test_delete_first() {
                    let mut list = build_list(&[4, 5, 1, 9]);
                    let node = find_node(&mut list, 4);
                    <$solution>::delete_node(node);
                    assert_eq!(to_vec(&list), vec![5, 1, 9]);
                }

                #[test]
                fn test_delete_second_to_last() {
                    let mut list = build_list(&[4, 5, 1, 9]);
                    let node = find_node(&mut list, 1);
                    <$solution>::delete_node(node);
                    assert_eq!(to_vec(&list), vec![4, 5, 9]);
                }

                #[test]
                fn test_two_nodes() {
                    let mut list = build_list(&[1, 2]);
                    let node = find_node(&mut list, 1);
                    <$solution>::delete_node(node);
                    assert_eq!(to_vec(&list), vec![2]);
                }
            }
        };
    }

    test_delete!(my, crate::my_solution::Solution);
    test_delete!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut list = build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            let node = find_node(&mut list, 5);
            crate::my_solution::Solution::delete_node(node);
            std::hint::black_box(&list);
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut list = build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            let node = find_node(&mut list, 5);
            crate::ai_solution::Solution::delete_node(node);
            std::hint::black_box(&list);
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
