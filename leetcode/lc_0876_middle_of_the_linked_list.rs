// LeetCode 876: Middle of the Linked List [Easy]
// https://leetcode.com/problems/middle-of-the-linked-list/
//
// Given the head of a singly linked list, return the middle node of the
// linked list.
//
// If there are two middle nodes, return the second middle node.
//
// Examples:
//   head = [1,2,3,4,5]   -> [3,4,5]
//   head = [1,2,3,4,5,6] -> [4,5,6]
//
// Constraints:
//   - The number of nodes in the list is in the range [1, 100]
//   - 1 <= Node.val <= 100
//
// Run:  cargo run --example lc_0876_middle_of_the_linked_list
// Test: cargo test --example lc_0876_middle_of_the_linked_list
// Bench: cargo test --example lc_0876_middle_of_the_linked_list bench -- --nocapture

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
        /// Return the middle node of the linked list.
        ///
        /// # Arguments
        ///
        /// * `head` - Head of the singly linked list
        ///
        /// # Returns
        ///
        /// The middle node (second middle if two exist)
        pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            // slow = head; fast = head
            // while fast && fast.next
            //   slow = slow.next
            //   fast = fast.next.next
            // end
            // slow
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
        /// Return the middle node of the linked list.
        ///
        /// # Arguments
        ///
        /// * `head` - Head of the singly linked list
        ///
        /// # Returns
        ///
        /// The middle node (second middle if two exist)
        pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
        (vec![1, 2, 3, 4, 5], vec![3, 4, 5]),
        (vec![1, 2, 3, 4, 5, 6], vec![4, 5, 6]),
    ];

    println!("--- My Solution ---");
    for (input, expected) in &test_cases {
        let head = build_list(input);
        let result = list_to_vec(my_solution::Solution::middle_node(head));
        println!("  {input:?} -> {result:?} (expected {expected:?})");
    }

    println!("--- AI Solution ---");
    for (input, expected) in &test_cases {
        let head = build_list(input);
        let result = list_to_vec(ai_solution::Solution::middle_node(head));
        println!("  {input:?} -> {result:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    use super::{build_list, list_to_vec};

    macro_rules! test_middle_node {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{build_list, list_to_vec};
                use $solution as Solution;

                #[test]
                fn test_odd_length() {
                    let head = build_list(&[1, 2, 3, 4, 5]);
                    assert_eq!(list_to_vec(Solution::middle_node(head)), vec![3, 4, 5]);
                }

                #[test]
                fn test_even_length() {
                    let head = build_list(&[1, 2, 3, 4, 5, 6]);
                    assert_eq!(list_to_vec(Solution::middle_node(head)), vec![4, 5, 6]);
                }

                #[test]
                fn test_single_node() {
                    let head = build_list(&[1]);
                    assert_eq!(list_to_vec(Solution::middle_node(head)), vec![1]);
                }

                #[test]
                fn test_two_nodes() {
                    let head = build_list(&[1, 2]);
                    assert_eq!(list_to_vec(Solution::middle_node(head)), vec![2]);
                }

                #[test]
                fn test_three_nodes() {
                    let head = build_list(&[1, 2, 3]);
                    assert_eq!(list_to_vec(Solution::middle_node(head)), vec![2, 3]);
                }

                #[test]
                fn test_four_nodes() {
                    let head = build_list(&[10, 20, 30, 40]);
                    assert_eq!(list_to_vec(Solution::middle_node(head)), vec![30, 40]);
                }

                #[test]
                fn test_seven_nodes() {
                    let head = build_list(&[1, 2, 3, 4, 5, 6, 7]);
                    assert_eq!(list_to_vec(Solution::middle_node(head)), vec![4, 5, 6, 7]);
                }
            }
        };
    }

    test_middle_node!(my_tests, crate::my_solution::Solution);
    test_middle_node!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5, 6],
            vec![1],
            vec![1, 2],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for vals in &inputs {
                let head = build_list(vals);
                std::hint::black_box(crate::my_solution::Solution::middle_node(
                    std::hint::black_box(head),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for vals in &inputs {
                let head = build_list(vals);
                std::hint::black_box(crate::ai_solution::Solution::middle_node(
                    std::hint::black_box(head),
                ));
            }
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
