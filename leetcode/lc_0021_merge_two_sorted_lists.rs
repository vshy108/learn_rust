// LeetCode 21: Merge Two Sorted Lists [Easy]
// https://leetcode.com/problems/merge-two-sorted-lists/
//
// You are given the heads of two sorted linked lists list1 and list2.
// Merge the two lists into one sorted list by splicing together the nodes.
// Return the head of the merged linked list.
//
// Examples:
//   list1 = [1,2,4], list2 = [1,3,4] -> [1,1,2,3,4,4]
//   list1 = [],      list2 = []      -> []
//   list1 = [],      list2 = [0]     -> [0]
//
// Constraints:
//   - The number of nodes in both lists is in [0, 50]
//   - -100 <= Node.val <= 100
//   - Both lists are sorted in non-decreasing order
//
// Run:  cargo run --example lc_0021_merge_two_sorted_lists
// Test: cargo test --example lc_0021_merge_two_sorted_lists
// Bench: cargo test --example lc_0021_merge_two_sorted_lists bench -- --nocapture

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
        /// Merge two sorted linked lists into one sorted list.
        ///
        /// # Arguments
        ///
        /// * `list1` - Head of the first sorted list
        /// * `list2` - Head of the second sorted list
        ///
        /// # Returns
        ///
        /// Head of the merged sorted list
        pub fn merge_two_lists(
            list1: Option<Box<ListNode>>,
            list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Merge two sorted linked lists into one sorted list.
        ///
        /// # Arguments
        ///
        /// * `list1` - Head of the first sorted list
        /// * `list2` - Head of the second sorted list
        ///
        /// # Returns
        ///
        /// Head of the merged sorted list
        pub fn merge_two_lists(
            list1: Option<Box<ListNode>>,
            list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            todo!()
        }
    }
}

fn main() {
    let l1 = build_list(&[1, 2, 4]);
    let l2 = build_list(&[1, 3, 4]);
    let result = my_solution::Solution::merge_two_lists(l1, l2);
    println!("My:  {:?}", list_to_vec(result));

    let l1 = build_list(&[1, 2, 4]);
    let l2 = build_list(&[1, 3, 4]);
    let result = ai_solution::Solution::merge_two_lists(l1, l2);
    println!("AI:  {:?}", list_to_vec(result));
}

#[cfg(test)]
mod tests {
    use super::{build_list, list_to_vec};

    macro_rules! test_merge {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    let l1 = build_list(&[1, 2, 4]);
                    let l2 = build_list(&[1, 3, 4]);
                    assert_eq!(
                        list_to_vec(Solution::merge_two_lists(l1, l2)),
                        vec![1, 1, 2, 3, 4, 4]
                    );
                }

                #[test]
                fn test_both_empty() {
                    assert_eq!(
                        list_to_vec(Solution::merge_two_lists(None, None)),
                        Vec::<i32>::new()
                    );
                }

                #[test]
                fn test_one_empty() {
                    let l2 = build_list(&[0]);
                    assert_eq!(list_to_vec(Solution::merge_two_lists(None, l2)), vec![0]);
                }

                #[test]
                fn test_single_elements() {
                    let l1 = build_list(&[2]);
                    let l2 = build_list(&[1]);
                    assert_eq!(list_to_vec(Solution::merge_two_lists(l1, l2)), vec![1, 2]);
                }

                #[test]
                fn test_negative_values() {
                    let l1 = build_list(&[-3, -1, 4]);
                    let l2 = build_list(&[-2, 0, 5]);
                    assert_eq!(
                        list_to_vec(Solution::merge_two_lists(l1, l2)),
                        vec![-3, -2, -1, 0, 4, 5]
                    );
                }
            }
        };
    }

    test_merge!(my_tests, crate::my_solution::Solution);
    test_merge!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;

        let start = Instant::now();
        for _ in 0..iterations {
            let l1 = build_list(&[1, 2, 4, 6, 8]);
            let l2 = build_list(&[1, 3, 5, 7, 9]);
            std::hint::black_box(crate::my_solution::Solution::merge_two_lists(l1, l2));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let l1 = build_list(&[1, 2, 4, 6, 8]);
            let l2 = build_list(&[1, 3, 5, 7, 9]);
            std::hint::black_box(crate::ai_solution::Solution::merge_two_lists(l1, l2));
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
