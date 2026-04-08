// LeetCode 724: Find Pivot Index [Easy]
// https://leetcode.com/problems/find-pivot-index/
//
// Given an array of integers `nums`, calculate the pivot index of this array.
//
// The pivot index is the index where the sum of all the numbers strictly to
// the left of the index is equal to the sum of all the numbers strictly to
// the index's right.
//
// If the index is on the left edge of the array, then the left sum is 0
// because there are no elements to the left. This also applies to the right
// edge of the array.
//
// Return the leftmost pivot index. If no such index exists, return -1.
//
// Examples:
//   nums = [1,7,3,6,5,6] -> 3
//     Left sum = 1+7+3 = 11, Right sum = 5+6 = 11
//   nums = [1,2,3]       -> -1
//     No index where left sum == right sum
//   nums = [2,1,-1]      -> 0
//     Left sum = 0, Right sum = 1+(-1) = 0
//
// Constraints:
//   - 1 <= nums.length <= 10^4
//   - -1000 <= nums[i] <= 1000
//
// Run:  cargo run --example lc_0724_find_pivot_index
// Test: cargo test --example lc_0724_find_pivot_index
// Bench: cargo test --example lc_0724_find_pivot_index bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  51.79ms (avg 517ns)
// AI solution:  52.38ms (avg 523ns)
// -> My solution is 1.01x faster
// Reason: essentially identical; the algebraic rewrite has no
//   measurable impact.

/// My solution module — prefix sum approach.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the leftmost pivot index, or -1 if none exists.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Leftmost pivot index, or -1
        pub fn pivot_index(nums: Vec<i32>) -> i32 {
            let total_sum: i32 = nums.iter().sum();
            let mut left_sum = 0;
            for (i, &num) in nums.iter().enumerate() {
                // right_sum = total_sum - left_sum - num
                if left_sum == total_sum - left_sum - num {
                    return i as i32;
                }
                left_sum += num;
            }
            -1
        }
    }
}

/// AI solution module — same prefix sum, uses `position` iterator.
///
/// Uses `iter().position()` with a fold-like tracking of left_sum
/// via a mutable closure capture.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the leftmost pivot index, or -1 if none exists.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Leftmost pivot index, or -1
        pub fn pivot_index(nums: Vec<i32>) -> i32 {
            let total: i32 = nums.iter().sum();
            let mut left = 0;
            for (i, &num) in nums.iter().enumerate() {
                // 2 * left_sum + num == total means left_sum == right_sum
                if 2 * left + num == total {
                    return i as i32;
                }
                left += num;
            }
            -1
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![
        (vec![1, 7, 3, 6, 5, 6], 3),
        (vec![1, 2, 3], -1),
        (vec![2, 1, -1], 0),
    ];

    println!("--- My Solution ---");
    for (nums, expected) in &test_cases {
        let result = my_solution::Solution::pivot_index(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &test_cases {
        let result = ai_solution::Solution::pivot_index(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_pivot_index {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
                }

                #[test]
                fn test_example_3() {
                    assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
                }

                #[test]
                fn test_single_element() {
                    // Left sum = 0, right sum = 0 -> pivot at 0
                    assert_eq!(Solution::pivot_index(vec![1]), 0);
                }

                #[test]
                fn test_pivot_at_end() {
                    // [0, 0, 0] -> pivot at 0 (leftmost)
                    assert_eq!(Solution::pivot_index(vec![0, 0, 0]), 0);
                }

                #[test]
                fn test_negative_numbers() {
                    assert_eq!(Solution::pivot_index(vec![-1, -1, -1, -1, -1, 0]), 2);
                }

                #[test]
                fn test_all_zeros() {
                    assert_eq!(Solution::pivot_index(vec![0, 0, 0, 0]), 0);
                }
            }
        };
    }

    test_pivot_index!(my_tests, crate::my_solution::Solution);
    test_pivot_index!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![1, 7, 3, 6, 5, 6],
            vec![1, 2, 3],
            vec![2, 1, -1],
            vec![0, 0, 0, 0],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::my_solution::Solution::pivot_index(
                    std::hint::black_box(nums.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::pivot_index(
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
