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
//
// --- Benchmark (100,000 iterations, 2026-04-03) ---
// My solution: avg 140ns
// AI solution: avg 3.952us
// Ratio: My solution is 28.16x faster
// Reason: Floyd's O(1) space uses no heap allocation; the HashSet approach
//         pays heavy per-element hashing + allocation overhead on small inputs.
//
// --- Rust concepts used ---
// - Closures capturing local variables: `let next = |i| -> Option<usize>`
// - Option chaining: .and_then(|f| next(f)) for two-hop traversal (Ruby's &.)
// - Pattern matching with early return: `match next(slow) { None => return -1 }`
// - .expect() for invariant assertions (phase 2 pointers must be in cycle)
// - HashSet::insert returns false if value already present (dual-purpose check+insert)
// - HashSet::with_capacity(n) to pre-allocate and avoid rehashing
// - Simulating cyclic data structures with indices (Rust ownership prevents Box cycles)

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
/// Floyd's tortoise and hare: two-phase cycle detection using a closure
/// to simulate linked list traversal.
///
/// Time:  O(n) - each pointer traverses at most 2n nodes
/// Space: O(1) - only pointer indices, no extra data structures
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
            let n = values.len();
            if n == 0 { return -1; }

            // Simulate linked list traversal: node i -> i+1, last node -> pos
            // Returns None if no next node (end of list or empty)
            let next = |i: usize| -> Option<usize> {
                if i == n - 1 {
                    // Last node: connects back to pos, or None if no cycle
                    if pos >= 0 { Some(pos as usize) } else { None }
                } else {
                    Some(i + 1)
                }
            };

            // Phase 1: detect cycle using Floyd's tortoise and hare
            // Ruby: slow = head; fast = head
            let mut slow = 0_usize;
            let mut fast = 0_usize;
            loop {
                // Ruby: slow = slow.next
                slow = match next(slow) {
                    Some(s) => s,
                    None => return -1, // no cycle
                };
                // Ruby: fast = fast.next.next
                fast = match next(fast).and_then(|f| next(f)) {
                    Some(f) => f,
                    // Ruby: return nil if fast.nil? || fast.next.nil?
                    None => return -1, // no cycle
                };
                // Ruby: break if slow == fast
                if slow == fast { break; }
            }

            // Phase 2: find cycle entry
            // Ruby: pointer1 = head; pointer2 = slow
            let mut p1 = 0_usize;
            let mut p2 = slow;
            // Ruby: while pointer1 != pointer2
            while p1 != p2 {
                // Both advance one step; guaranteed to meet at cycle start
                p1 = next(p1).expect("p1 should be in cycle path");
                p2 = next(p2).expect("p2 should be in cycle");
            }
            // Ruby: pointer1 (return the node, here we return the index)
            p1 as i32
        }
    }
}

/// AI solution module.
///
/// HashSet approach: traverse the simulated list, recording visited node
/// indices. The first repeated index is the cycle entry point.
///
/// Time:  O(n) - single traversal until revisit
/// Space: O(n) - HashSet storing visited indices
mod ai_solution {
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        /// Detect where a cycle begins in a linked list.
        ///
        /// Traverses the simulated list and tracks visited node indices
        /// in a `HashSet`. The first index seen twice is the cycle start.
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
            let n = values.len();
            if n == 0 { return -1; }

            // HashSet to track which nodes we've already visited
            let mut visited = HashSet::with_capacity(n);
            let mut current = 0_usize;

            loop {
                // If we've seen this index before, it's the cycle entry
                if !visited.insert(current) {
                    return current as i32;
                }
                // Advance: last node wraps to pos, others go to i+1
                if current == n - 1 {
                    if pos >= 0 {
                        current = pos as usize;
                    } else {
                        return -1; // end of list, no cycle
                    }
                } else {
                    current += 1;
                }
            }
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
