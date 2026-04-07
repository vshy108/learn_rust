// LeetCode 23: Merge k Sorted Lists [Hard]
// https://leetcode.com/problems/merge-k-sorted-lists/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 179.340µs
//   AI solution avg: 143.639µs
//   Ratio: AI solution is 1.25x faster
//
// You are given an array of k linked-lists, each sorted in ascending order.
// Merge all the linked-lists into one sorted linked-list and return it.
//
// Example 1: lists = [[1,4,5],[1,3,4],[2,6]] -> [1,1,2,3,4,4,5,6]
// Example 2: lists = []                       -> []
// Example 3: lists = [[]]                     -> []
//
// Constraints:
//   k == lists.length
//   0 <= k <= 10^4
//   0 <= lists[i].length <= 500
//   -10^4 <= lists[i][j] <= 10^4
//   lists[i] is sorted in ascending order
//   Sum of lists[i].length <= 10^4
//
// Run:  cargo run --example lc_0023_merge_k_sorted_lists
// Test: cargo test --example lc_0023_merge_k_sorted_lists
// Bench: cargo test --example lc_0023_merge_k_sorted_lists bench -- --nocapture

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
    // Build in reverse so the first element becomes the head.
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

/// My solution — collect all values and sort.
///
/// Flatten every list into a single Vec, sort it, then rebuild one list.
/// Simple but not optimal when lists are already individually sorted.
///
/// Time: O(N log N) where N = total number of nodes across all lists.
/// Space: O(N) for the collected values vector.
mod my_solution {
    use crate::{build_list, ListNode};

    pub struct Solution;

    impl Solution {
        /// Merge k sorted linked lists into one sorted list.
        ///
        /// # Arguments
        /// * `lists` - vector of optional linked-list heads
        ///
        /// # Returns
        /// Head of the merged sorted list, or None if all empty.
        pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
            // Collect all node values into one vector.
            let mut vals: Vec<i32> = Vec::new();
            for mut head in lists {
                while let Some(node) = head {
                    vals.push(node.val);
                    head = node.next;
                }
            }

            // Sort and rebuild a list from sorted values.
            vals.sort_unstable();
            build_list(&vals)
        }
    }
}

/// AI-optimized solution — divide-and-conquer merge.
///
/// Pair-wise merge lists in rounds (like merge sort), halving the number
/// of lists each round. Each merge is a standard two-sorted-list merge.
/// This exploits the pre-sorted order without a global sort.
///
/// Time: O(N log k) where N = total nodes, k = number of lists.
///       There are O(log k) rounds, each touching all N nodes once.
/// Space: O(1) extra (in-place pointer rewiring, ignoring recursion).
mod ai_solution {
    use crate::ListNode;

    pub struct Solution;

    impl Solution {
        /// Merge k sorted linked lists into one sorted list.
        ///
        /// # Arguments
        /// * `lists` - vector of optional linked-list heads
        ///
        /// # Returns
        /// Head of the merged sorted list, or None if all empty.
        pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
            if lists.is_empty() {
                return None;
            }

            // Pair-wise merge until one list remains.
            let mut len = lists.len();
            while len > 1 {
                let mut i = 0;
                let mut j = 0;
                // Merge pairs: lists[0]+lists[1], lists[2]+lists[3], ...
                while j + 1 < len {
                    // Take both lists out of the vector, merge, put back.
                    let left = lists[j].take();
                    let right = lists[j + 1].take();
                    lists[i] = Self::merge_two(left, right);
                    i += 1;
                    j += 2;
                }
                // If odd number of lists, bring the last one forward.
                if j < len {
                    lists[i] = lists[j].take();
                    i += 1;
                }
                len = i;
            }

            lists[0].take()
        }

        /// Merge two sorted lists into one (iterative).
        fn merge_two(
            mut l1: Option<Box<ListNode>>,
            mut l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            // Dummy head avoids special-casing the first node.
            let mut dummy = ListNode::new(0);
            let mut tail = &mut dummy;

            while l1.is_some() && l2.is_some() {
                // Compare heads; move the smaller one to the merged list.
                if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                    let mut node = l1.unwrap();
                    l1 = node.next.take();
                    tail.next = Some(node);
                } else {
                    let mut node = l2.unwrap();
                    l2 = node.next.take();
                    tail.next = Some(node);
                }
                tail = tail.next.as_deref_mut().unwrap();
            }

            // Attach whichever list has remaining nodes.
            tail.next = if l1.is_some() { l1 } else { l2 };

            dummy.next
        }
    }
}

fn main() {
    println!("--- My Solution ---");
    let lists = vec![
        build_list(&[1, 4, 5]),
        build_list(&[1, 3, 4]),
        build_list(&[2, 6]),
    ];
    let my = list_to_vec(my_solution::Solution::merge_k_lists(lists));
    println!("merged: {my:?}");

    println!("--- AI Solution ---");
    let lists = vec![
        build_list(&[1, 4, 5]),
        build_list(&[1, 3, 4]),
        build_list(&[2, 6]),
    ];
    let ai = list_to_vec(ai_solution::Solution::merge_k_lists(lists));
    println!("merged: {ai:?}");
}

#[cfg(test)]
mod tests {
    use crate::build_list;

    macro_rules! test_merge_k_lists {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;
                use crate::{build_list, list_to_vec};

                #[test]
                fn test_example_1() {
                    let lists = vec![
                        build_list(&[1, 4, 5]),
                        build_list(&[1, 3, 4]),
                        build_list(&[2, 6]),
                    ];
                    let result = list_to_vec(Solution::merge_k_lists(lists));
                    assert_eq!(result, vec![1, 1, 2, 3, 4, 4, 5, 6]);
                }

                #[test]
                fn test_empty_input() {
                    let lists: Vec<Option<Box<crate::ListNode>>> = vec![];
                    assert_eq!(list_to_vec(Solution::merge_k_lists(lists)), Vec::<i32>::new());
                }

                #[test]
                fn test_single_empty_list() {
                    let lists = vec![None];
                    assert_eq!(list_to_vec(Solution::merge_k_lists(lists)), Vec::<i32>::new());
                }

                #[test]
                fn test_single_list() {
                    let lists = vec![build_list(&[1, 2, 3])];
                    assert_eq!(list_to_vec(Solution::merge_k_lists(lists)), vec![1, 2, 3]);
                }

                #[test]
                fn test_two_lists() {
                    let lists = vec![build_list(&[1, 3, 5]), build_list(&[2, 4, 6])];
                    assert_eq!(
                        list_to_vec(Solution::merge_k_lists(lists)),
                        vec![1, 2, 3, 4, 5, 6]
                    );
                }

                #[test]
                fn test_lists_with_negatives() {
                    let lists = vec![
                        build_list(&[-5, -1, 3]),
                        build_list(&[-3, 0, 2]),
                    ];
                    assert_eq!(
                        list_to_vec(Solution::merge_k_lists(lists)),
                        vec![-5, -3, -1, 0, 2, 3]
                    );
                }

                #[test]
                fn test_all_same_values() {
                    let lists = vec![
                        build_list(&[1, 1]),
                        build_list(&[1, 1]),
                        build_list(&[1]),
                    ];
                    assert_eq!(
                        list_to_vec(Solution::merge_k_lists(lists)),
                        vec![1, 1, 1, 1, 1]
                    );
                }

                #[test]
                fn test_mixed_empty_and_non_empty() {
                    let lists = vec![
                        None,
                        build_list(&[2, 4]),
                        None,
                        build_list(&[1, 3]),
                        None,
                    ];
                    assert_eq!(
                        list_to_vec(Solution::merge_k_lists(lists)),
                        vec![1, 2, 3, 4]
                    );
                }
            }
        };
    }

    test_merge_k_lists!(my_tests, crate::my_solution::Solution);
    test_merge_k_lists!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;

        // Build k=20 lists, each with 50 elements => 1000 total nodes.
        let make_lists = || -> Vec<Option<Box<crate::ListNode>>> {
            (0..20)
                .map(|i| {
                    let vals: Vec<i32> = (0..50).map(|j| i * 50 + j).collect();
                    build_list(&vals)
                })
                .collect()
        };

        let start = Instant::now();
        for _ in 0..iterations {
            let lists = make_lists();
            std::hint::black_box(crate::my_solution::Solution::merge_k_lists(
                std::hint::black_box(lists),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let lists = make_lists();
            std::hint::black_box(crate::ai_solution::Solution::merge_k_lists(
                std::hint::black_box(lists),
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
