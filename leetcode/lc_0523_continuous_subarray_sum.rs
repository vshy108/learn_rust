// LeetCode 523: Continuous Subarray Sum [Medium]
// https://leetcode.com/problems/continuous-subarray-sum/
//
// Given an integer array `nums` and an integer `k`, return `true` if `nums`
// has a good subarray or `false` otherwise.
//
// A good subarray is a subarray where:
//   - its length is at least 2, and
//   - the sum of the elements of the subarray is a multiple of k.
//
// Note that:
//   - A subarray is a contiguous part of the array.
//   - An integer x is a multiple of k if there exists an integer n such that
//     x = n * k. 0 is always a multiple of k.
//
// Examples:
//   nums = [23,2,4,6,7], k = 6  -> true  (subarray [2,4] sums to 6)
//   nums = [23,2,6,4,7], k = 6  -> true  (subarray [23,2,6,4,7] sums to 42)
//   nums = [23,2,6,4,7], k = 13 -> false
//
// Constraints:
//   - 1 <= nums.length <= 10^5
//   - 0 <= nums[i] <= 10^9
//   - 0 <= sum(nums[i]) <= 2^31 - 1
//   - 1 <= k <= 2^31 - 1
//
// Run:  cargo run --example lc_0523_continuous_subarray_sum
// Test: cargo test --example lc_0523_continuous_subarray_sum
// Bench: cargo test --example lc_0523_continuous_subarray_sum bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  760.46ms (avg 7.60us)
// AI solution:  81.72ms (avg 817ns)
// -> AI solution is 9.31x faster
// Reason: Vec<bool> direct-index lookup for small k avoids all hashing;
//   delayed-insertion trick eliminates index tracking overhead.

/// My solution module — prefix sum mod k with HashMap.
///
/// Time:  O(n)
/// Space: O(min(n, k))
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Check if nums has a subarray of size >= 2 whose sum is a multiple of k.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        /// * `k` - The divisor
        ///
        /// # Returns
        ///
        /// `true` if such a subarray exists
        pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
            // Map remainder -> earliest index where that remainder was seen.
            // Seed with 0 -> -1 to handle subarrays starting from index 0.
            let mut mod_map: HashMap<i32, i32> = HashMap::new();
            mod_map.insert(0, -1);
            let mut sum = 0;
            for (i, &num) in nums.iter().enumerate() {
                sum += num;
                let remainder = sum % k;
                if let Some(&prev_index) = mod_map.get(&remainder) {
                    // Subarray length must be at least 2
                    if i as i32 - prev_index >= 2 {
                        return true;
                    }
                } else {
                    mod_map.insert(remainder, i as i32);
                }
            }
            false
        }
    }
}

/// AI solution module — same prefix-mod approach, uses `entry` API.
///
/// Uses `or_insert` to only store the first occurrence of each remainder,
/// avoiding the if-else branch.
///
/// Time:  O(n)
/// Space: O(min(n, k))
/// AI solution module — adaptive Vec<bool> / HashSet with delayed insertion.
///
/// For small k (< 500_000), uses `Vec<bool>` for O(1) direct-index lookup
/// instead of hashing. For large k, falls back to `HashSet`.
/// Inserts the *previous* remainder (`prevsum`) each iteration, which
/// guarantees subarray length >= 2 without tracking indices.
///
/// Time:  O(n)
/// Space: O(min(n, k))
mod ai_solution {
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        /// Check if nums has a subarray of size >= 2 whose sum is a multiple of k.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        /// * `k` - The divisor
        ///
        /// # Returns
        ///
        /// `true` if such a subarray exists
        pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
            if k < 500_000 {
                // Small k: use Vec<bool> for direct index lookup (no hashing)
                let mut sum = 0i32;
                let mut prevsum = 0usize;
                let mut freq = vec![false; k as usize];
                for num in nums {
                    sum += num;
                    if sum >= k {
                        sum %= k;
                    }
                    if freq[sum as usize] {
                        return true;
                    }
                    // Insert previous remainder: ensures subarray length >= 2
                    freq[prevsum] = true;
                    prevsum = sum as usize;
                }
            } else {
                // Large k: use HashSet to avoid huge Vec allocation
                let mut sum = 0i32;
                let mut prevsum = 0i32;
                let mut hmap = HashSet::new();
                for num in nums {
                    sum += num;
                    if sum >= k {
                        sum %= k;
                    }
                    if hmap.contains(&sum) {
                        return true;
                    }
                    // Insert previous remainder: ensures subarray length >= 2
                    hmap.insert(prevsum);
                    prevsum = sum;
                }
            }
            false
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, bool)> = vec![
        (vec![23, 2, 4, 6, 7], 6, true),
        (vec![23, 2, 6, 4, 7], 6, true),
        (vec![23, 2, 6, 4, 7], 13, false),
    ];

    println!("--- My Solution ---");
    for (nums, k, expected) in &test_cases {
        let result = my_solution::Solution::check_subarray_sum(nums.clone(), *k);
        println!("  {nums:?}, k={k} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, k, expected) in &test_cases {
        let result = ai_solution::Solution::check_subarray_sum(nums.clone(), *k);
        println!("  {nums:?}, k={k} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_check_subarray_sum {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
                }

                #[test]
                fn test_example_2() {
                    assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
                }

                #[test]
                fn test_example_3() {
                    assert!(!Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
                }

                #[test]
                fn test_zeros() {
                    // [0,0] sums to 0, which is a multiple of any k
                    assert!(Solution::check_subarray_sum(vec![0, 0], 1));
                }

                #[test]
                fn test_single_element() {
                    // Subarray must be size >= 2
                    assert!(!Solution::check_subarray_sum(vec![6], 6));
                }

                #[test]
                fn test_two_zeros() {
                    // 0 is a multiple of any k
                    assert!(Solution::check_subarray_sum(vec![0, 0], 7));
                }

                #[test]
                fn test_large_k() {
                    assert!(!Solution::check_subarray_sum(vec![1, 2, 3], 100));
                }
            }
        };
    }

    test_check_subarray_sum!(my_tests, crate::my_solution::Solution);
    test_check_subarray_sum!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(Vec<i32>, i32)> = vec![
            (vec![23, 2, 4, 6, 7], 6),
            (vec![23, 2, 6, 4, 7], 6),
            (vec![23, 2, 6, 4, 7], 13),
            (vec![0, 0], 7),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, k) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::check_subarray_sum(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(*k),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, k) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::check_subarray_sum(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(*k),
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
