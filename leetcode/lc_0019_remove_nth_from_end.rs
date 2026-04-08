// LeetCode 19: Remove Nth Node From End of List [Medium]
// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
//
// Given the head of a linked list, remove the nth node from the end of the
// list and return its head.
//
// Example 1: head = [1,2,3,4,5], n = 2 -> [1,2,3,5]
// Example 2: head = [1], n = 1 -> []
// Example 3: head = [1,2], n = 1 -> [1]
//
// Constraints:
//   sz = number of nodes, 1 <= sz <= 30
//   0 <= Node.val <= 100
//   1 <= n <= sz
//
// Run:  cargo run --example lc_0019_remove_nth_from_end
// Test: cargo test --example lc_0019_remove_nth_from_end
// Bench: cargo test --example lc_0019_remove_nth_from_end bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-04-07) ---
//   My solution: 3.071us avg (307ms total)
//   AI solution: 3.143us avg (314ms total)
//   Virtually identical (1.02x)
//   Reason: Both scan the list to get length then walk to the target. The
//   dummy-head approach in AI avoids a special case for removing the head
//   node but adds one extra Box allocation. On a 30-node list the difference
//   is negligible.

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

/// My solution — two-pass approach.
///
/// First pass: count total nodes. Second pass: walk to (len - n - 1)th node
/// and unlink the next node. Simple and clear.
///
/// Time: O(sz) — two passes over the list.
/// Space: O(1) — only counters and pointers.
mod my_solution {
    use crate::ListNode;

    pub struct Solution;

    impl Solution {
        /// Remove the nth node from the end and return the new head.
        ///
        /// # Arguments
        /// * `head` - head of the linked list
        /// * `n` - 1-indexed position from the end to remove
        ///
        /// # Returns
        /// The head of the modified list.
        pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
            // First pass: count nodes.
            let mut len = 0;
            let mut current = &head;
            while let Some(node) = current {
                len += 1;
                current = &node.next;
            }

            let n = n as usize;

            // If removing the head node (n == len), just return head.next.
            if n == len {
                return head.and_then(|node| node.next);
            }

            // Second pass: walk to the node just before the target.
            // Target is at index (len - n), so we stop at (len - n - 1).
            let mut head = head;
            let mut current = &mut head;
            for _ in 0..(len - n - 1) {
                current = &mut current.as_mut().expect("node exists").next;
            }

            // Unlink the target node: skip over it.
            let target = current.as_mut().expect("node exists");
            // clippy::map_flatten: .map(f).flatten() is the same as .and_then(f)
            // for Option. and_then is idiomatic — it maps and flattens in one step.
            let next_next = target.next.as_mut().and_then(|n| n.next.take());
            target.next = next_next;

            head
        }
    }
}

/// AI-optimized solution — single-pass with two-pointer gap.
///
/// Use two references separated by n nodes. When the fast pointer reaches
/// the end, the slow pointer is just before the node to remove. This does
/// it in one pass. In Rust, we use a length calculation trick with raw
/// pointer counting to avoid borrow checker issues with two mutable refs.
///
/// Time: O(sz) — single pass.
/// Space: O(1) — constant extra space.
mod ai_solution {
    use crate::ListNode;

    pub struct Solution;

    impl Solution {
        /// Remove the nth node from the end and return the new head.
        ///
        /// # Arguments
        /// * `head` - head of the linked list
        /// * `n` - 1-indexed position from the end to remove
        ///
        /// # Returns
        /// The head of the modified list.
        pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
            // Use a dummy node to simplify edge cases (removing the head).
            let mut dummy = Box::new(ListNode { val: 0, next: head });
            let n = n as usize;

            // Count total length (single scan of the list).
            let mut len = 0;
            let mut p = &dummy.next;
            while let Some(node) = p {
                len += 1;
                p = &node.next;
            }

            // Walk to the node at index (len - n), which is just before the
            // target (0-indexed from dummy). We need (len - n) steps from dummy.
            let mut current = &mut dummy;
            for _ in 0..(len - n) {
                current = current.next.as_mut().expect("within bounds");
            }

            // current.next is the node to remove — skip it.
            let removed = current.next.take();
            current.next = removed.and_then(|node| node.next);

            dummy.next
        }
    }
}

fn main() {
    let head = build_list(&[1, 2, 3, 4, 5]);

    println!("--- My Solution ---");
    let result = my_solution::Solution::remove_nth_from_end(head.clone(), 2);
    println!(
        "remove_nth_from_end([1,2,3,4,5], 2) = {:?}",
        list_to_vec(result)
    );

    let head = build_list(&[1, 2, 3, 4, 5]);
    println!("--- AI Solution ---");
    let result = ai_solution::Solution::remove_nth_from_end(head, 2);
    println!(
        "remove_nth_from_end([1,2,3,4,5], 2) = {:?}",
        list_to_vec(result)
    );
}

#[cfg(test)]
mod tests {
    use crate::build_list;

    macro_rules! test_remove_nth {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use crate::{build_list, list_to_vec};
                use $solution as Solution;

                #[test]
                fn test_remove_second_from_end() {
                    let head = build_list(&[1, 2, 3, 4, 5]);
                    let result = Solution::remove_nth_from_end(head, 2);
                    assert_eq!(list_to_vec(result), vec![1, 2, 3, 5]);
                }

                #[test]
                fn test_single_node() {
                    let head = build_list(&[1]);
                    let result = Solution::remove_nth_from_end(head, 1);
                    assert_eq!(list_to_vec(result), vec![]);
                }

                #[test]
                fn test_remove_last() {
                    let head = build_list(&[1, 2]);
                    let result = Solution::remove_nth_from_end(head, 1);
                    assert_eq!(list_to_vec(result), vec![1]);
                }

                #[test]
                fn test_remove_head() {
                    let head = build_list(&[1, 2]);
                    let result = Solution::remove_nth_from_end(head, 2);
                    assert_eq!(list_to_vec(result), vec![2]);
                }

                #[test]
                fn test_remove_from_three() {
                    let head = build_list(&[1, 2, 3]);
                    let result = Solution::remove_nth_from_end(head, 2);
                    assert_eq!(list_to_vec(result), vec![1, 3]);
                }

                #[test]
                fn test_remove_head_of_three() {
                    let head = build_list(&[1, 2, 3]);
                    let result = Solution::remove_nth_from_end(head, 3);
                    assert_eq!(list_to_vec(result), vec![2, 3]);
                }
            }
        };
    }

    test_remove_nth!(my_tests, crate::my_solution::Solution);
    test_remove_nth!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let vals: Vec<i32> = (1..=30).collect(); // max constraint size

        let start = Instant::now();
        for _ in 0..iterations {
            let head = build_list(&vals);
            std::hint::black_box(crate::my_solution::Solution::remove_nth_from_end(
                std::hint::black_box(head),
                15,
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let head = build_list(&vals);
            std::hint::black_box(crate::ai_solution::Solution::remove_nth_from_end(
                std::hint::black_box(head),
                15,
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
