// LeetCode 142: Linked List Cycle II [Medium]
// https://leetcode.com/problems/linked-list-cycle-ii/
//
// Given the head of a linked list, return the node where the cycle begins.
// If there is no cycle, return null.
//
// NOTE: Standard LeetCode uses raw pointers for cyclic lists. This scaffold
// uses a simplified signature: given a list and a `pos` index indicating
// where the tail connects, return the index of the cycle start (or -1).
// The solution receives the list values and pos, not an actual cyclic list.
//
// Examples:
//   head = [3,2,0,-4], pos = 1 -> 1  (tail connects to node at index 1)
//   head = [1,2],      pos = 0 -> 0  (tail connects to node at index 0)
//   head = [1],         pos = -1 -> -1 (no cycle)
//
// Constraints:
//   - The number of nodes in the list is in [0, 10^4]
//   - -10^5 <= Node.val <= 10^5
//   - pos is -1 or a valid index in the list
//
// Run:  cargo run --example lc_0142_linked_list_cycle_ii
// Test: cargo test --example lc_0142_linked_list_cycle_ii
// Bench: cargo test --example lc_0142_linked_list_cycle_ii bench -- --nocapture

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

/// My solution module.
///
/// Since Rust's ownership model prevents creating actual cyclic linked lists
/// with `Box`, this uses a simplified approach: pass the values and cycle
/// position, and return the index where the cycle starts (or -1).
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Detect where a cycle begins in a linked list.
        ///
        /// # Arguments
        ///
        /// * `values` - Node values in list order
        /// * `pos` - Index where the tail connects (-1 if no cycle)
        ///
        /// # Returns
        ///
        /// Index of the cycle start, or -1 if no cycle
        pub fn detect_cycle(values: &[i32], pos: i32) -> i32 {
            // return nil if head.nil? || head.next.nil?
            // Phase 1: detect cycle
            //   slow = head
            //   fast = head
            //   while fast && fast.next
            //     slow = slow.next
            //     fast = fast.next.next
            //     break if slow == fast
            //   end
            //   return nil if fast.nil? || fast.next.nil?
            // Phase 2: find cycle entry
            //   pointer1 = head
            //   pointer2 = slow
            //   while pointer1 != pointer2
            //     pointer1 = pointer1.next
            //     pointer2 = pointer2.next
            //   end
            //   pointer1
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Detect where a cycle begins in a linked list.
        ///
        /// # Arguments
        ///
        /// * `values` - Node values in list order
        /// * `pos` - Index where the tail connects (-1 if no cycle)
        ///
        /// # Returns
        ///
        /// Index of the cycle start, or -1 if no cycle
        pub fn detect_cycle(values: &[i32], pos: i32) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
        (vec![3, 2, 0, -4], 1, 1),
        (vec![1, 2], 0, 0),
        (vec![1], -1, -1),
    ];

    println!("--- My Solution ---");
    for (values, pos, expected) in &test_cases {
        let result = my_solution::Solution::detect_cycle(values, *pos);
        println!("  {values:?}, pos={pos} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (values, pos, expected) in &test_cases {
        let result = ai_solution::Solution::detect_cycle(values, *pos);
        println!("  {values:?}, pos={pos} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_detect_cycle {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_cycle_at_index_1() {
                    assert_eq!(Solution::detect_cycle(&[3, 2, 0, -4], 1), 1);
                }

                #[test]
                fn test_cycle_at_index_0() {
                    assert_eq!(Solution::detect_cycle(&[1, 2], 0), 0);
                }

                #[test]
                fn test_no_cycle() {
                    assert_eq!(Solution::detect_cycle(&[1], -1), -1);
                }

                #[test]
                fn test_empty_list() {
                    assert_eq!(Solution::detect_cycle(&[], -1), -1);
                }

                #[test]
                fn test_single_node_self_loop() {
                    assert_eq!(Solution::detect_cycle(&[1], 0), 0);
                }

                #[test]
                fn test_cycle_at_last_to_first() {
                    assert_eq!(Solution::detect_cycle(&[1, 2, 3, 4], 0), 0);
                }

                #[test]
                fn test_no_cycle_long_list() {
                    assert_eq!(Solution::detect_cycle(&[1, 2, 3, 4, 5], -1), -1);
                }
            }
        };
    }

    test_detect_cycle!(my_tests, crate::my_solution::Solution);
    test_detect_cycle!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(Vec<i32>, i32)> = vec![
            (vec![3, 2, 0, -4], 1),
            (vec![1, 2], 0),
            (vec![1], -1),
            (vec![1, 2, 3, 4, 5], -1),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for (values, pos) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::detect_cycle(
                    std::hint::black_box(values),
                    std::hint::black_box(*pos),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (values, pos) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::detect_cycle(
                    std::hint::black_box(values),
                    std::hint::black_box(*pos),
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
