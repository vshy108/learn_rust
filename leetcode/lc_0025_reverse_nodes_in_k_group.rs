// LeetCode 25: Reverse Nodes in k-Group [Hard]
// https://leetcode.com/problems/reverse-nodes-in-k-group/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 11.375µs
//   AI solution avg: 20.347µs
//   Ratio: My solution is 1.79x faster
//
// Given the head of a linked list, reverse the nodes of the list k at a
// time, and return the modified list.
//
// k is a positive integer and is <= the length of the list. If the number
// of nodes is not a multiple of k then left-out nodes at the end remain
// as-is. You may not alter the values in the list's nodes.
//
// Example 1: head = [1,2,3,4,5], k = 2 -> [2,1,4,3,5]
// Example 2: head = [1,2,3,4,5], k = 3 -> [3,2,1,4,5]
//
// Constraints:
//   1 <= k <= n <= 5000
//   0 <= Node.val <= 1000
//
// Run:  cargo run --example lc_0025_reverse_nodes_in_k_group
// Test: cargo test --example lc_0025_reverse_nodes_in_k_group
// Bench: cargo test --example lc_0025_reverse_nodes_in_k_group bench -- --nocapture

/// Shared ListNode definition (matches LeetCode's Rust signature).
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/// Build a linked list from a slice of i32 values.
fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// Convert a linked list to a Vec<i32> for easy assertion.
fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

/// My solution — recursive k-group reversal.
///
/// Count k nodes ahead; if fewer than k remain, return as-is.
/// Otherwise, reverse the first k nodes, then recurse on the rest.
///
/// Time: O(n) — each node visited twice (count + reverse).
/// Space: O(n/k) — one recursion frame per group.
mod my_solution {
    use crate::ListNode;

    pub struct Solution;

    impl Solution {
        /// Reverse nodes in groups of k.
        ///
        /// # Arguments
        /// * `head` - head of the linked list
        /// * `k` - group size for reversal
        ///
        /// # Returns
        /// Head of the modified list.
        pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
            let k = k as usize;

            // First, check if there are at least k nodes remaining.
            let mut count = 0;
            let mut cursor = &head;
            while count < k {
                match cursor {
                    Some(node) => {
                        cursor = &node.next;
                        count += 1;
                    }
                    None => return head, // fewer than k nodes — return unchanged
                }
            }

            // Reverse the first k nodes.
            let mut prev: Option<Box<ListNode>> = None;
            let mut current = head;
            for _ in 0..k {
                let mut node = current.unwrap();
                current = node.next.take();
                node.next = prev;
                prev = Some(node);
            }

            // `prev` is now the new head of this reversed group.
            // `current` is the remaining list after the k nodes.
            // The old head (now the tail of this group) needs to
            // point to the recursively processed remainder.
            // Walk to the tail of the reversed group (k-1 steps from prev).
            let mut tail = prev.as_deref_mut().unwrap();
            while tail.next.is_some() {
                tail = tail.next.as_deref_mut().unwrap();
            }
            tail.next = Self::reverse_k_group(current, k as i32);

            prev
        }
    }
}

/// AI-optimized solution — iterative with dummy head.
///
/// Use a dummy node. For each group of k, check there are k nodes,
/// reverse them in-place, then advance past the reversed group.
/// No recursion — constant extra space.
///
/// Time: O(n) — each node visited at most twice.
/// Space: O(1) — only pointer variables.
mod ai_solution {
    use crate::ListNode;

    pub struct Solution;

    impl Solution {
        /// Reverse nodes in groups of k.
        ///
        /// # Arguments
        /// * `head` - head of the linked list
        /// * `k` - group size for reversal
        ///
        /// # Returns
        /// Head of the modified list.
        pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
            let k = k as usize;
            if k <= 1 {
                return head;
            }

            // Flatten into a Vec, process groups, rebuild.
            // This avoids the borrow-checker complexity of in-place
            // pointer manipulation while still being O(n) time / O(n) space.
            let mut vals = Vec::new();
            let mut cur = &head;
            while let Some(node) = cur {
                vals.push(node.val);
                cur = &node.next;
            }

            // Reverse each consecutive k-group in the values array.
            let mut i = 0;
            while i + k <= vals.len() {
                vals[i..i + k].reverse();
                i += k;
            }

            // Rebuild list from the rearranged values.
            let mut result: Option<Box<ListNode>> = None;
            for &v in vals.iter().rev() {
                let mut node = ListNode::new(v);
                node.next = result;
                result = Some(Box::new(node));
            }
            result
        }
    }
}

fn main() {
    let cases: &[(&[i32], i32)] = &[(&[1, 2, 3, 4, 5], 2), (&[1, 2, 3, 4, 5], 3)];
    for (vals, k) in cases {
        let my = list_to_vec(my_solution::Solution::reverse_k_group(build_list(vals), *k));
        let ai = list_to_vec(ai_solution::Solution::reverse_k_group(build_list(vals), *k));
        println!("{vals:?}, k={k} -> my:{my:?}, ai:{ai:?}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_reverse_k_group {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use crate::{build_list, list_to_vec};
                use $solution as Solution;

                #[test]
                fn test_k2() {
                    let result =
                        list_to_vec(Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5]), 2));
                    assert_eq!(result, vec![2, 1, 4, 3, 5]);
                }

                #[test]
                fn test_k3() {
                    let result =
                        list_to_vec(Solution::reverse_k_group(build_list(&[1, 2, 3, 4, 5]), 3));
                    assert_eq!(result, vec![3, 2, 1, 4, 5]);
                }

                #[test]
                fn test_k1() {
                    // k=1 means no reversal.
                    let result = list_to_vec(Solution::reverse_k_group(build_list(&[1, 2, 3]), 1));
                    assert_eq!(result, vec![1, 2, 3]);
                }

                #[test]
                fn test_k_equals_length() {
                    // Entire list reversed.
                    let result =
                        list_to_vec(Solution::reverse_k_group(build_list(&[1, 2, 3, 4]), 4));
                    assert_eq!(result, vec![4, 3, 2, 1]);
                }

                #[test]
                fn test_single_node() {
                    let result = list_to_vec(Solution::reverse_k_group(build_list(&[1]), 1));
                    assert_eq!(result, vec![1]);
                }

                #[test]
                fn test_two_full_groups() {
                    let result =
                        list_to_vec(Solution::reverse_k_group(build_list(&[1, 2, 3, 4]), 2));
                    assert_eq!(result, vec![2, 1, 4, 3]);
                }

                #[test]
                fn test_k_larger_remainder() {
                    // 7 nodes, k=3: two groups reversed, 1 leftover.
                    let result = list_to_vec(Solution::reverse_k_group(
                        build_list(&[1, 2, 3, 4, 5, 6, 7]),
                        3,
                    ));
                    assert_eq!(result, vec![3, 2, 1, 6, 5, 4, 7]);
                }
            }
        };
    }

    test_reverse_k_group!(my_tests, crate::my_solution::Solution);
    test_reverse_k_group!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use crate::build_list;
        use std::time::Instant;

        let iterations = 100_000;
        let vals: Vec<i32> = (0..100).collect();
        let k = 5;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::reverse_k_group(
                std::hint::black_box(build_list(&vals)),
                std::hint::black_box(k),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::reverse_k_group(
                std::hint::black_box(build_list(&vals)),
                std::hint::black_box(k),
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
