// LeetCode 206: Reverse Linked List [Easy]
// https://leetcode.com/problems/reverse-linked-list/
//
// Given the head of a singly linked list, reverse the list, and return
// the reversed list.
//
// Examples:
//   head = [1,2,3,4,5] -> [5,4,3,2,1]
//   head = [1,2]       -> [2,1]
//   head = []          -> []
//
// Constraints:
//   - The number of nodes in the list is in [0, 5000]
//   - -5000 <= Node.val <= 5000
//
// Follow-up: A linked list can be reversed either iteratively or
// recursively. Could you implement both?
//
// Run:  cargo run --example lc_0206_reverse_linked_list
// Test: cargo test --example lc_0206_reverse_linked_list
// Bench: cargo test --example lc_0206_reverse_linked_list bench -- --nocapture
//
// --- Benchmark (100,000 iterations, 2026-04-05) ---
// My solution: avg 404ns
// AI solution: avg 414ns
// Ratio: My solution is 1.02x faster (effectively equivalent)
// Reason: Both are O(n) single-pass. Iterative avoids function call overhead
//         per node, but on small lists the difference is negligible. Recursive
//         version uses O(n) stack frames vs O(1) for iterative.
/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    /// Create a new ListNode with the given value.
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Helper: build a linked list from a slice.
fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// Helper: collect linked list values into a Vec.
fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Reverse a singly linked list.
        ///
        /// # Arguments
        ///
        /// * `head` - Head of the singly linked list
        ///
        /// # Returns
        ///
        /// Head of the reversed list
        pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev: Option<Box<ListNode>> = None;
            let mut current = head;

            // Ruby: while current  (truthy check on nil/non-nil)
            // Rust: `while current` doesn't work — Option doesn't impl bool.
            //   `while let Some(mut node) = current` destructures the Option,
            //   extracting the inner Box<ListNode> into `node`. When current
            //   is None, the pattern doesn't match and the loop ends.
            //   This also moves ownership out of `current` into `node`,
            //   so `current` is now uninitialized (we reassign it below).
            while let Some(mut node) = current {
                // Ruby: next_node = current.next
                // Rust: can't just read node.next — that would move it out
                //   of `node` while `node` is still alive (partial move error).
                //   .take() swaps node.next with None, returning the old value.
                //   This gives us ownership of the next pointer cleanly.
                current = node.next.take();
                // Ruby: current.next = prev
                node.next = prev;
                // Ruby: prev = current
                prev = Some(node);
                // Ruby: current = next_node  (already done above with .take())
            }

            prev
        }
    }
}

/// AI solution module.
///
/// Recursive approach: reverse the rest of the list first, then append
/// the current node at the end. Each recursive call processes one node,
/// building the reversed list on the way back up the call stack.
///
/// Time:  O(n) - one recursive call per node
/// Space: O(n) - call stack depth equals list length
mod ai_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Reverse a singly linked list recursively.
        ///
        /// Base case: empty list or single node returns as-is.
        /// Recursive case: reverse the tail, then make the next node
        /// point back to the current node.
        ///
        /// # Arguments
        ///
        /// * `head` - Head of the singly linked list
        ///
        /// # Returns
        ///
        /// Head of the reversed list
        pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            // Helper that carries an accumulator (like iterative prev)
            // to avoid O(n) stack depth issues with naive recursion.
            // Tail-recursive style: Rust doesn't guarantee TCO, but this
            // pattern keeps the logic clean and mirrors the iterative approach.
            fn helper(
                current: Option<Box<ListNode>>,
                prev: Option<Box<ListNode>>,
            ) -> Option<Box<ListNode>> {
                match current {
                    None => prev, // base case: end of list, prev is the new head
                    Some(mut node) => {
                        // Same three steps as iterative, but via recursion:
                        let next = node.next.take(); // detach next
                        node.next = prev; // point back
                        helper(next, Some(node)) // recurse with next as current
                    }
                }
            }
            helper(head, None)
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
        (vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]),
        (vec![1, 2], vec![2, 1]),
        (vec![], vec![]),
    ];

    println!("--- My Solution ---");
    for (input, expected) in &test_cases {
        let head = build_list(input);
        let result = list_to_vec(my_solution::Solution::reverse_list(head));
        println!("  {input:?} -> {result:?} (expected {expected:?})");
    }

    println!("--- AI Solution ---");
    for (input, expected) in &test_cases {
        let head = build_list(input);
        let result = list_to_vec(ai_solution::Solution::reverse_list(head));
        println!("  {input:?} -> {result:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    use super::{build_list, list_to_vec};

    macro_rules! test_reverse {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                use super::super::{build_list, list_to_vec};

                #[test]
                fn test_five_elements() {
                    let head = build_list(&[1, 2, 3, 4, 5]);
                    assert_eq!(
                        list_to_vec(Solution::reverse_list(head)),
                        vec![5, 4, 3, 2, 1]
                    );
                }

                #[test]
                fn test_two_elements() {
                    let head = build_list(&[1, 2]);
                    assert_eq!(list_to_vec(Solution::reverse_list(head)), vec![2, 1]);
                }

                #[test]
                fn test_empty_list() {
                    let head = build_list(&[]);
                    assert_eq!(list_to_vec(Solution::reverse_list(head)), Vec::<i32>::new());
                }

                #[test]
                fn test_single_element() {
                    let head = build_list(&[42]);
                    assert_eq!(list_to_vec(Solution::reverse_list(head)), vec![42]);
                }

                #[test]
                fn test_negative_values() {
                    let head = build_list(&[-1, -2, -3]);
                    assert_eq!(list_to_vec(Solution::reverse_list(head)), vec![-3, -2, -1]);
                }

                #[test]
                fn test_duplicate_values() {
                    let head = build_list(&[1, 1, 2, 2]);
                    assert_eq!(list_to_vec(Solution::reverse_list(head)), vec![2, 2, 1, 1]);
                }

                #[test]
                fn test_three_elements() {
                    let head = build_list(&[1, 2, 3]);
                    assert_eq!(list_to_vec(Solution::reverse_list(head)), vec![3, 2, 1]);
                }
            }
        };
    }

    test_reverse!(my_tests, crate::my_solution::Solution);
    test_reverse!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let input = vec![1, 2, 3, 4, 5];

        let start = Instant::now();
        for _ in 0..iterations {
            let head = build_list(&input);
            std::hint::black_box(crate::my_solution::Solution::reverse_list(
                std::hint::black_box(head),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let head = build_list(&input);
            std::hint::black_box(crate::ai_solution::Solution::reverse_list(
                std::hint::black_box(head),
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
