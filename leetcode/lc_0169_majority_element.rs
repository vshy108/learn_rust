// LeetCode 169: Majority Element [Easy]
// https://leetcode.com/problems/majority-element/
//
// Given an array nums of size n, return the majority element. The majority
// element is the element that appears more than n/2 times. You may assume
// that the majority element always exists in the array.
//
// Examples:
//   nums = [3,2,3]         -> 3
//   nums = [2,2,1,1,1,2,2] -> 2
//
// Constraints:
//   - n == nums.length
//   - 1 <= n <= 5 * 10^4
//   - -10^9 <= nums[i] <= 10^9
//
// Follow-up: Could you solve the problem in linear time and O(1) space?
//
// Run:  cargo run --example lc_0169_majority_element
// Test: cargo test --example lc_0169_majority_element
// Bench: cargo test --example lc_0169_majority_element bench -- --nocapture
//
// --- Benchmark (100,000 iterations, 2026-04-04) ---
// My solution: avg 479ns
// AI solution: avg 736ns
// Ratio: My solution is 1.54x faster
// Reason: Boyer-Moore is a single branchless-style pass with two register
//         variables (candidate + count). Quickselect does O(n) comparisons
//         but with heavier partitioning swaps and worse cache access patterns
//         on these small inputs.
//
// --- Rust concepts used ---
// - Boyer-Moore Voting: single-pass O(1) space majority-finding algorithm
// - `nums[0]` safe here due to constraint n >= 1 (panics on empty Vec)
// - Partial sort with `select_nth_unstable`: O(n) avg quickselect, no full sort
// - Slice indexing with `nums[n / 2]` after partitioning finds the median
//   (Previously covered: consuming Vec in lc_0134, if-as-expression in lc_0008)

/// My solution module.
///
/// Boyer-Moore Voting Algorithm: maintain a candidate and a count.
/// When count drops to 0, adopt the current element as the new candidate.
/// The majority element survives because it appears > n/2 times.
///
/// Time:  O(n) - single pass through the array
/// Space: O(1) - only two variables (candidate + count)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the majority element (appears more than n/2 times).
        ///
        /// # Arguments
        ///
        /// * `nums` - Array of integers with a guaranteed majority element
        ///
        /// # Returns
        ///
        /// The majority element
        pub fn majority_element(nums: Vec<i32>) -> i32 {
            // Boyer-Moore Voting: candidate tracks the potential majority.
            // count acts as a "net votes" tally. When count hits 0, the
            // current candidate is abandoned and replaced by the next element.
            // The true majority element always survives because it has > n/2 votes.
            let mut count = 0;
            // Safe: constraint guarantees n >= 1, so nums[0] exists.
            // Overwritten immediately on first iteration since count starts at 0.
            let mut candidate = nums[0];

            for num in nums {
                if count == 0 {
                    candidate = num;
                }
                // Conditional expression as value: returns 1 or -1 directly
                count += if num == candidate { 1 } else { -1 };
            }

            candidate
        }
    }
}

/// AI solution module.
///
/// Partial sort (quickselect): since the majority element appears > n/2
/// times, it must occupy the middle index after sorting. Instead of a
/// full O(n log n) sort, `select_nth_unstable` partitions in O(n) average
/// to place the median in position — then we just read it.
///
/// Time:  O(n) average — quickselect partitioning (O(n^2) worst case)
/// Space: O(1) — in-place partitioning, no extra allocation
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the majority element (appears more than n/2 times).
        ///
        /// Uses `select_nth_unstable` (quickselect) to partially sort
        /// the array so the median is in correct position. Since the
        /// majority element appears > n/2 times, the median is always
        /// the majority element.
        ///
        /// # Arguments
        ///
        /// * `nums` - Array of integers with a guaranteed majority element
        ///
        /// # Returns
        ///
        /// The majority element
        pub fn majority_element(mut nums: Vec<i32>) -> i32 {
            let n = nums.len();
            // select_nth_unstable: O(n) avg quickselect that partitions
            // so nums[n/2] holds the value that would be at index n/2
            // in a fully sorted array. No need to sort the rest.
            nums.select_nth_unstable(n / 2);
            // The majority element (> n/2 occurrences) always occupies
            // the median position after any valid partitioning.
            nums[n / 2]
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![(vec![3, 2, 3], 3), (vec![2, 2, 1, 1, 1, 2, 2], 2)];

    println!("--- My Solution ---");
    for (nums, expected) in &test_cases {
        let result = my_solution::Solution::majority_element(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &test_cases {
        let result = ai_solution::Solution::majority_element(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_majority {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_basic_odd() {
                    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
                }

                #[test]
                fn test_basic_even() {
                    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(Solution::majority_element(vec![1]), 1);
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(Solution::majority_element(vec![1, 1]), 1);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(Solution::majority_element(vec![5, 5, 5, 5]), 5);
                }

                #[test]
                fn test_majority_at_start() {
                    assert_eq!(Solution::majority_element(vec![1, 1, 1, 2, 3]), 1);
                }

                #[test]
                fn test_negative_majority() {
                    assert_eq!(Solution::majority_element(vec![-1, -1, 2]), -1);
                }
            }
        };
    }

    test_majority!(my_tests, crate::my_solution::Solution);
    test_majority!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![3, 2, 3],
            vec![2, 2, 1, 1, 1, 2, 2],
            vec![1, 1, 1, 2, 3],
            vec![5, 5, 5, 5],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::my_solution::Solution::majority_element(
                    std::hint::black_box(nums.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::majority_element(
                    std::hint::black_box(nums.clone()),
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
