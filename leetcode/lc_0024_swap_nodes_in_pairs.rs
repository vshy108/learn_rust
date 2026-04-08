// LeetCode 24: Swap Nodes in Pairs [Medium]
// https://leetcode.com/problems/swap-nodes-in-pairs/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 10.165µs
//   AI solution avg: 10.797µs
//   Ratio: ~tied (my 1.06x faster)
//
// Given a linked list, swap every two adjacent nodes and return its head.
// You must solve the problem without modifying the values in the list's
// nodes (i.e., only nodes themselves may be changed).
//
// Example 1: head = [1,2,3,4] -> [2,1,4,3]
// Example 2: head = []        -> []
// Example 3: head = [1]       -> [1]
// Example 4: head = [1,2,3]   -> [2,1,3]
//
// Constraints:
//   Number of nodes in [0, 100]
//   0 <= Node.val <= 100
//
// Run:  cargo run --example lc_0024_swap_nodes_in_pairs
// Test: cargo test --example lc_0024_swap_nodes_in_pairs
// Bench: cargo test --example lc_0024_swap_nodes_in_pairs bench -- --nocapture

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

/// My solution — recursive swap.
///
/// For each pair, swap the two nodes and recurse on the remainder.
/// Simple and readable, though each recursive call adds a stack frame.
///
/// Time: O(n) — visits every node once.
/// Space: O(n) — recursion depth is n/2.
mod my_solution {
    use crate::ListNode;

    pub struct Solution;

    impl Solution {
        /// Swap every two adjacent nodes in the linked list.
        ///
        /// # Arguments
        /// * `head` - head of the linked list
        ///
        /// # Returns
        /// Head of the list with pairs swapped.
        pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            // Base: 0 or 1 node — nothing to swap.
            if head.is_none() {
                return None;
            }
            let mut first = head.unwrap();

            // Only one node left — return as-is.
            if first.next.is_none() {
                return Some(first);
            }

            // Detach second node and the rest of the list.
            let mut second = first.next.take().unwrap();
            let rest = second.next.take();

            // Recurse on the remainder, then wire up the swap:
            // second -> first -> (recursed rest)
            first.next = Self::swap_pairs(rest);
            second.next = Some(first);
            Some(second)
        }
    }
}

/// AI-optimized solution — iterative with dummy head.
///
/// Use a dummy node before the head. Walk through pairs, rewiring
/// pointers in-place without recursion.
///
/// Time: O(n) — single pass.
/// Space: O(1) — only pointer variables, no recursion.
mod ai_solution {
    use crate::ListNode;

    pub struct Solution;

    impl Solution {
        /// Swap every two adjacent nodes in the linked list.
        ///
        /// # Arguments
        /// * `head` - head of the linked list
        ///
        /// # Returns
        /// Head of the list with pairs swapped.
        pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut dummy = Box::new(ListNode { val: 0, next: head });
            let mut prev = &mut dummy;

            // While there are at least two nodes ahead:
            while prev.next.is_some() && prev.next.as_ref().unwrap().next.is_some() {
                // Detach `first` from prev.
                let mut first = prev.next.take().unwrap();
                // Detach `second` from first.
                let mut second = first.next.take().unwrap();
                // The rest of the list after the pair.
                let rest = second.next.take();

                // Rewire: prev -> second -> first -> rest
                first.next = rest;
                second.next = Some(first);
                prev.next = Some(second);

                // Advance prev past the swapped pair (prev -> second -> first).
                // Move to `first`, which is now at prev.next.next.
                prev = prev
                    .next
                    .as_deref_mut()
                    .unwrap() // second
                    .next
                    .as_mut()
                    .unwrap(); // first
            }

            dummy.next
        }
    }
}

fn main() {
    let cases: &[&[i32]] = &[&[1, 2, 3, 4], &[], &[1], &[1, 2, 3]];
    for vals in cases {
        let my = list_to_vec(my_solution::Solution::swap_pairs(build_list(vals)));
        let ai = list_to_vec(ai_solution::Solution::swap_pairs(build_list(vals)));
        println!("{vals:?} -> my:{my:?}, ai:{ai:?}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_swap_pairs {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use crate::{build_list, list_to_vec};
                use $solution as Solution;

                #[test]
                fn test_four_nodes() {
                    let result = list_to_vec(Solution::swap_pairs(build_list(&[1, 2, 3, 4])));
                    assert_eq!(result, vec![2, 1, 4, 3]);
                }

                #[test]
                fn test_empty() {
                    let result = list_to_vec(Solution::swap_pairs(None));
                    assert_eq!(result, Vec::<i32>::new());
                }

                #[test]
                fn test_single() {
                    let result = list_to_vec(Solution::swap_pairs(build_list(&[1])));
                    assert_eq!(result, vec![1]);
                }

                #[test]
                fn test_three_nodes() {
                    let result = list_to_vec(Solution::swap_pairs(build_list(&[1, 2, 3])));
                    assert_eq!(result, vec![2, 1, 3]);
                }

                #[test]
                fn test_two_nodes() {
                    let result = list_to_vec(Solution::swap_pairs(build_list(&[1, 2])));
                    assert_eq!(result, vec![2, 1]);
                }

                #[test]
                fn test_five_nodes() {
                    let result = list_to_vec(Solution::swap_pairs(build_list(&[1, 2, 3, 4, 5])));
                    assert_eq!(result, vec![2, 1, 4, 3, 5]);
                }

                #[test]
                fn test_same_values() {
                    let result = list_to_vec(Solution::swap_pairs(build_list(&[5, 5, 5, 5])));
                    assert_eq!(result, vec![5, 5, 5, 5]);
                }
            }
        };
    }

    test_swap_pairs!(my_tests, crate::my_solution::Solution);
    test_swap_pairs!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use crate::build_list;
        use std::time::Instant;

        let iterations = 100_000;
        let vals: Vec<i32> = (0..100).collect();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::swap_pairs(
                std::hint::black_box(build_list(&vals)),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::swap_pairs(
                std::hint::black_box(build_list(&vals)),
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
